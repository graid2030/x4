#!/usr/bin/env python3

"""
HTML карта сектора со станциями и торговлей.

- Читает сейв X4 (.xml/.xml.gz) и вспом. JSON (x4-offsets.json, x4-names.json, x4-component-names.json, x4-localization.json).
- Строит HTML страницу с встраиваемым SVG-планом сектора, списком станций и их офферов (buy/sell),
  а также таблицей арбитражных возможностей внутри сектора.

Пример:
  python x4-sector-map-html.py <save.xml.gz> --sector "Hatikvah" --out hatikvah.html --top 40
"""

from lxml import etree
import gzip
import sys
import argparse
import json
import re
from typing import Dict, List, Tuple


def create_optimized_parser():
    return etree.XMLParser(
        remove_blank_text=True, remove_comments=True, remove_pis=True,
        huge_tree=True, collect_ids=False, resolve_entities=False
    )


def resolveName(identity, names):
    text = identity
    if not isinstance(identity, str):
        return str(identity)
    for segment in re.findall(r"({[^}]+})", identity):
        try:
            pageEntry = segment.split(",")
            page = pageEntry[0][1:]
            entry = pageEntry[1][:-1]
            if page in names and entry in names[page]:
                segmentText = resolveName(names[page][entry], names)
                text = text.replace(segment, segmentText)
            else:
                text = text.replace(segment, segment[1:-1])
        except (IndexError, KeyError):
            text = text.replace(segment, segment[1:-1])
    return text


def getStationName(station, component_names, localization):
    macro = (station.get('macro') or '').lower()
    if macro in component_names:
        base_name = component_names[macro]
        nameindex = station.get('nameindex', '0')
        return f"{base_name} #{nameindex}" if nameindex and nameindex != '0' else base_name

    name_attribute = station.get('name')
    if name_attribute and name_attribute.startswith('{'):
        resolved_name = resolveName(name_attribute, localization)
        if resolved_name != name_attribute:
            return resolved_name

    id_component = station.find(".//component[@class='identification']")
    if id_component is not None:
        id_name_attribute = id_component.get('name')
        if id_name_attribute and id_name_attribute.startswith('{'):
            resolved_name = resolveName(id_name_attribute, localization)
            if resolved_name != id_name_attribute:
                return resolved_name

    return station.get('code') or station.get('macro') or 'Станция'


def getPosition(obj, sector_zone_offsets, position=None):
    if position is None:
        position = {'x': 0.0, 'y': 0.0, 'z': 0.0, 'pitch': 0.0, 'roll': 0.0, 'yaw': 0.0}

    macro = obj.get('macro')
    if macro and macro in sector_zone_offsets:
        for key, value in sector_zone_offsets[macro].items():
            position[key] += value

    objpos = obj.find('./offset/position')
    if objpos is not None:
        position['x'] += float(objpos.get('x', 0.0))
        position['y'] += float(objpos.get('y', 0.0))
        position['z'] += float(objpos.get('z', 0.0))

    objrot = obj.find('./offset/rotation')
    if objrot is not None:
        position['pitch'] += float(objrot.get('pitch', 0.0))
        position['roll'] += float(objrot.get('roll', 0.0))
        position['yaw'] += float(objrot.get('yaw', 0.0))

    if obj.get('class') == 'galaxy':
        for key in position:
            try:
                position[key] = float(int(position[key]))
            except Exception:
                position[key] = float(position[key])
        return position

    parent = obj.getparent()
    if parent is not None:
        return getPosition(parent, sector_zone_offsets, position)
    return position


def getStationWares(station):
    wares_data = []
    trade_elements = station.findall(".//trade[@ware]")
    for trade_elem in trade_elements:
        ware_id = trade_elem.get('ware')
        price = trade_elem.get('price')
        amount = trade_elem.get('amount')
        buyer = trade_elem.get('buyer')
        seller = trade_elem.get('seller')
        if not (ware_id and price):
            continue
        try:
            price_val = int(price) / 100.0
            amount_val = int(amount) if amount else 0
            # buyer -> станция ПОКУПАЕТ (buy), seller -> станция ПРОДАЁТ (sell)
            if buyer is not None:
                trade_type = 'buy'
            elif seller is not None:
                trade_type = 'sell'
            else:
                trade_type = 'sell' if amount_val > 0 else 'buy'
            if price_val > 0:
                wares_data.append({'ware': ware_id, 'price': price_val, 'amount': amount_val, 'type': trade_type})
        except (ValueError, TypeError):
            continue
    return wares_data


def choose_sector(sectors_info: List[Dict], arg_value: str = None) -> Dict:
    # sectors_info sorted by name
    def by_term(term: str):
        t = term.lower()
        matches = [s for s in sectors_info if t in (s['name'] or '').lower() or t == (s['code'] or '').lower()]
        return matches

    if arg_value:
        # Try number -> by index
        try:
            idx = int(arg_value)
            if 1 <= idx <= len(sectors_info):
                return sectors_info[idx - 1]
        except ValueError:
            pass
        # Try exact code or substring of name
        matches = by_term(arg_value)
        if matches:
            return matches[0]

    # Fallback: show list and ask once
    print("Доступные секторы:")
    for i, s in enumerate(sectors_info, start=1):
        print(f"{i:3d}) {s['name']} ({s['code']})")
    user_inp = input("\nВведите номер или часть имени сектора: ").strip()
    if not user_inp:
        return sectors_info[0]
    try:
        idx = int(user_inp)
        if 1 <= idx <= len(sectors_info):
            return sectors_info[idx - 1]
    except ValueError:
        pass
    matches = by_term(user_inp)
    return matches[0] if matches else sectors_info[0]


# Faction colors for ships (global constant)
FACTION_COLORS = {
    'argon': '#4a9eff',      # Синий
    'paranid': '#ff8c42',    # Оранжевый
    'teladi': '#4caf50',     # Зеленый
    'split': '#e74c3c',      # Красный
    'xenon': '#95a5a6',      # Серый
    'khaak': '#9b59b6',      # Фиолетовый
    'player': '#ffd700',     # Золотой
    'antigone': '#00bcd4',   # Голубой
    'holyorder': '#ffeb3b',  # Желтый
    'ministry': '#ff5722',   # Темно-красный
    'ownerless': '#bdc3c7',  # Светло-серый
    'unknown': '#7f8c8d',    # Темно-серый
}


def build_svg(stations: List[Dict], sector_name: str, ships: List[Dict] = None, player_pos: Dict = None, size: str = "1400x1000") -> Tuple[str, Dict[str, float]]:
    try:
        w_str, h_str = size.lower().split('x', 1)
        W = max(320, int(w_str)); H = max(240, int(h_str))
    except Exception:
        W, H = 1400, 1000
    margin = 48

    # Include player position and ships in bounds calculation
    all_x = [s['x'] for s in stations]
    all_z = [s['z'] for s in stations]
    if ships:
        all_x.extend([sh['x'] for sh in ships])
        all_z.extend([sh['z'] for sh in ships])
    if player_pos:
        all_x.append(player_pos['x'])
        all_z.append(player_pos['z'])

    minx = min(all_x, default=0.0)
    maxx = max(all_x, default=1.0)
    minz = min(all_z, default=0.0)
    maxz = max(all_z, default=1.0)
    if maxx - minx < 1e-6:
        maxx = minx + 1.0
    if maxz - minz < 1e-6:
        maxz = minz + 1.0
    sx = (W - 2 * margin) / (maxx - minx)
    sz = (H - 2 * margin) / (maxz - minz)
    k = min(sx, sz)

    def map_point(x, z) -> Tuple[float, float]:
        px = margin + (x - minx) * k
        py = H - (margin + (z - minz) * k)  # invert Z to screen Y
        return px, py

    # Grid
    grid = []
    step_world = max((maxx - minx), (maxz - minz)) / 10.0
    step_world = max(step_world, 1.0)
    for gi in range(0, 11):
        gx = minx + gi * step_world
        gz = minz + gi * step_world
        x1, y1 = map_point(minx + gi * step_world, minz)
        x2, y2 = map_point(minx + gi * step_world, maxz)
        x3, y3 = map_point(minx, minz + gi * step_world)
        x4, y4 = map_point(maxx, minz + gi * step_world)
        grid.append((x1, y1, x2, y2))
        grid.append((x3, y3, x4, y4))

    # Build SVG
    lines = []
    ap = lines.append
    ap(f"<svg xmlns='http://www.w3.org/2000/svg' width='{W}' height='{H}' viewBox='0 0 {W} {H}'>")
    ap("  <defs>")
    ap("    <style><![CDATA[")
    ap("      .grid{stroke:#e9eef3;stroke-width:1}")
    ap("      .point{fill:#1f77b4;stroke:#fff;stroke-width:1}")
    ap("      .player{fill:#ff4444;stroke:#fff;stroke-width:2}")
    ap("      .label{font:10px/1.2 -apple-system,BlinkMacSystemFont,Segoe UI,Roboto,Arial,sans-serif;fill:#1c2a38}")
    ap("      .player-label{font:bold 11px/1.2 -apple-system,BlinkMacSystemFont,Segoe UI,Roboto,Arial,sans-serif;fill:#ff4444}")
    ap("      .ship-label{font:8px/1.2 -apple-system,BlinkMacSystemFont,Segoe UI,Roboto,Arial,sans-serif;fill:#1c2a38}")
    ap("      .title{font:600 16px -apple-system,BlinkMacSystemFont,Segoe UI,Roboto,Arial,sans-serif;fill:#1c2a38}")
    for faction, color in FACTION_COLORS.items():
        ap(f"      .ship-{faction}{{fill:{color};stroke:#fff;stroke-width:0.8}}")
    ap("    ]]></style>")
    ap("  </defs>")
    ap(f"  <text class='title' x='{W/2:.1f}' y='22' text-anchor='middle'>{sector_name}</text>")
    # grid lines
    for (x1,y1,x2,y2) in grid:
        ap(f"  <line class='grid' x1='{x1:.1f}' y1='{y1:.1f}' x2='{x2:.1f}' y2='{y2:.1f}' />")

    # stations
    for s in stations:
        x, y = map_point(s['x'], s['z'])
        title = f"{s['name']}".replace('&', '&amp;')
        ap(f"  <g>")
        ap(f"    <circle class='point' cx='{x:.1f}' cy='{y:.1f}' r='4'>")
        ap(f"      <title>{title}</title>")
        ap(f"    </circle>")
        ap(f"    <text class='label' x='{x+6:.1f}' y='{y-6:.1f}'>" + title + "</text>")
        ap(f"  </g>")

    # ships (triangles)
    if ships:
        for sh in ships:
            x, y = map_point(sh['x'], sh['z'])
            owner = sh['owner'].lower()
            ship_class = f"ship-{owner}" if owner in FACTION_COLORS else "ship-unknown"
            title = f"{sh['name']} ({owner})".replace('&', '&amp;')
            # Triangle pointing up (▲)
            points = f"{x:.1f},{y-3:.1f} {x-2.5:.1f},{y+2:.1f} {x+2.5:.1f},{y+2:.1f}"
            ap(f"  <g>")
            ap(f"    <polygon class='{ship_class}' points='{points}'>")
            ap(f"      <title>{title}</title>")
            ap(f"    </polygon>")
            ap(f"  </g>")

    # player position
    if player_pos:
        x, y = map_point(player_pos['x'], player_pos['z'])
        ap(f"  <g>")
        ap(f"    <circle class='player' cx='{x:.1f}' cy='{y:.1f}' r='6'>")
        ap(f"      <title>Игрок</title>")
        ap(f"    </circle>")
        ap(f"    <text class='player-label' x='{x+8:.1f}' y='{y-8:.1f}'>Игрок</text>")
        ap(f"  </g>")

    ap("</svg>")
    svg = "\n".join(lines)
    return svg, {"minx": minx, "maxx": maxx, "minz": minz, "maxz": maxz, "W": W, "H": H, "k": k}


def main():
    parser = argparse.ArgumentParser(description="HTML карта сектора с товарами и арбитражем")
    parser.add_argument("savefile", help="Путь к файлу сохранения (.xml или .xml.gz)")
    parser.add_argument("--sector", help="Номер, код или часть имени сектора (если не указан - показывается сектор игрока)")
    parser.add_argument("--out", help="Файл HTML для записи (по умолчанию sector-map-<code>.html)")
    parser.add_argument("--size", default="1400x1000", help="Размер SVG, например 1400x1000")
    parser.add_argument("--top", type=int, default=30, help="Сколько арбитражных строк показать (по умолчанию 30)")
    parser.add_argument("--min-profit", type=float, default=0.0, help="Минимальная прибыль/шт для показа")
    args = parser.parse_args()

    # Load save
    if args.savefile.endswith('.gz'):
        with gzip.open(args.savefile, 'rb') as f:
            rawxml = f.read()
    else:
        with open(args.savefile, 'rb') as f:
            rawxml = f.read()
    if not rawxml:
        print("ОШИБКА: не удалось прочитать сейв")
        sys.exit(1)

    # Load helpers
    try:
        with open("x4-offsets.json", "r") as jsonfile:
            sector_zone_offsets = json.load(jsonfile)
        with open("x4-names.json", "r") as jsonfile:
            sector_macros = json.load(jsonfile)
        with open("x4-component-names.json", "r", encoding='utf-8') as jsonfile:
            component_names = json.load(jsonfile)
        with open("x4-localization.json", "r", encoding='utf-8') as jsonfile:
            localization = json.load(jsonfile)
    except FileNotFoundError as e:
        print(f"ОШИБКА: не найден файл {e.filename}. Поместите JSON рядом со скриптом.")
        sys.exit(1)

    root = etree.fromstring(rawxml, parser=create_optimized_parser())

    # Get player info from save header
    player_elem = root.find("./info/player")
    player_location_raw = player_elem.get('location') if player_elem is not None else None
    player_component_id = None
    if player_location_raw:
        # Parse location like "{20004,1130011}" to get component ID
        try:
            loc_parts = player_location_raw.strip('{}').split(',')
            if len(loc_parts) == 2:
                player_component_id = loc_parts[1].strip()
                print(f"ID компонента игрока: {player_component_id}")
        except Exception as e:
            print(f"Ошибка парсинга location: {e}")

    # Collect sectors and find player's sector
    sectors_nodes = root.findall(".//universe/component/connections/connection/component/connections/connection/component[@class='sector']")
    sectors_info = []
    seen = set()
    player_sector_code = None

    for sector in sectors_nodes:
        code = sector.get('code')
        if not code or code in seen:
            continue
        seen.add(code)
        macro = sector.get('macro')
        name = sector_macros.get(macro, '') or code or macro
        sectors_info.append({'node': sector, 'code': code, 'macro': macro, 'name': name})

        # Check if player is in this sector
        if player_component_id and not player_sector_code:
            all_components = sector.findall("./connections/connection/component/connections/connection/component")
            for comp in all_components:
                if comp.get('id') == player_component_id:
                    player_sector_code = code
                    print(f"Игрок находится в секторе: {name} ({code})")
                    break

    sectors_info.sort(key=lambda s: s['name'].lower())
    if not sectors_info:
        print("Секторы не найдены в сейве")
        sys.exit(1)

    # If no sector specified and player found, use player's sector
    sector_arg = args.sector
    if not sector_arg and player_sector_code:
        sector_arg = player_sector_code
        print(f"Автоматически выбран сектор игрока: {player_sector_code}")

    chosen = choose_sector(sectors_info, sector_arg)
    chosen_code = chosen['code']
    chosen_name = chosen['name']

    # Find player location in chosen sector
    player_pos = None
    player_ship = None

    if player_component_id:
        # Find all components in the chosen sector
        all_resources = chosen['node'].findall("./connections/connection/component/connections/connection/component")
        for res in all_resources:
            # Check if this component's ID matches player location
            comp_id = res.get('id')
            if comp_id == player_component_id:
                player_ship = res
                pos = getPosition(res, sector_zone_offsets)
                player_pos = {
                    'x': float(pos.get('x', 0.0)),
                    'y': float(pos.get('y', 0.0)),
                    'z': float(pos.get('z', 0.0)),
                }
                ship_name = res.get('name', res.get('code', 'Unknown'))
                print(f"Игрок найден в секторе {chosen_code}: {ship_name}, позиция {player_pos}")
                break

        if not player_pos:
            if player_sector_code:
                print(f"Игрок не в секторе {chosen_code}, а в секторе {player_sector_code}")
            else:
                print(f"Игрок не найден в секторе {chosen_code}")

    # Collect stations and ships in chosen sector
    stations: List[Dict] = []
    ships: List[Dict] = []
    resources = chosen['node'].findall("./connections/connection/component/connections/connection/component")
    for res in resources:
        if res.getparent() is None:
            continue

        connection = res.getparent().get('connection')

        # Collect stations
        if connection == 'stations':
            if res.get('state') == 'wreck':
                continue
            pos = getPosition(res, sector_zone_offsets)
            name = getStationName(res, component_names, localization)
            code = res.get('code') or ''
            offers = getStationWares(res)
            sell = [w for w in offers if w['type'] == 'sell']
            buy = [w for w in offers if w['type'] == 'buy']
            stations.append({
                'name': name,
                'code': code,
                'x': float(pos.get('x', 0.0)),
                'y': float(pos.get('y', 0.0)),
                'z': float(pos.get('z', 0.0)),
                'sell': sell,
                'buy': buy,
            })

        # Collect ships
        elif connection == 'ships':
            if res.get('state') == 'wreck':
                continue
            pos = getPosition(res, sector_zone_offsets)
            name = res.get('name') or res.get('code') or 'Unknown'
            code = res.get('code') or ''
            owner = res.get('owner') or 'unknown'
            ship_class = res.get('class') or 'ship'
            macro = res.get('macro') or ''
            ships.append({
                'name': name,
                'code': code,
                'x': float(pos.get('x', 0.0)),
                'y': float(pos.get('y', 0.0)),
                'z': float(pos.get('z', 0.0)),
                'owner': owner,
                'class': ship_class,
                'macro': macro,
            })

    # Arbitrage within sector
    sell_by_ware: Dict[str, List[Dict]] = {}
    buy_by_ware: Dict[str, List[Dict]] = {}
    for s in stations:
        for w in s['sell']:
            if w['amount'] > 0:
                sell_by_ware.setdefault(w['ware'], []).append({**w, 'station': s['name'], 'code': s['code']})
        for w in s['buy']:
            if w['amount'] > 0:
                buy_by_ware.setdefault(w['ware'], []).append({**w, 'station': s['name'], 'code': s['code']})

    results = []
    for ware, sells in sell_by_ware.items():
        buys = buy_by_ware.get(ware)
        if not buys:
            continue
        for s in sells:
            for b in buys:
                if s['code'] == b['code']:
                    continue
                unit_profit = b['price'] - s['price']
                if unit_profit <= 0 or unit_profit < args.min_profit:
                    continue
                qty = min(s['amount'], b['amount']) if (s['amount'] and b['amount']) else 0
                if qty <= 0:
                    continue
                total_profit = unit_profit * qty
                percent_profit = (unit_profit / s['price'] * 100.0) if s['price'] > 0 else 0.0
                results.append({
                    'ware': ware,
                    'buy_price': s['price'], 'buy_station': s['station'], 'buy_code': s['code'],
                    'sell_price': b['price'], 'sell_station': b['station'], 'sell_code': b['code'],
                    'qty': qty, 'unit_profit': unit_profit, 'total_profit': total_profit, 'percent_profit': percent_profit,
                })

    results.sort(key=lambda r: (r['percent_profit'], r['unit_profit'], r['total_profit']), reverse=True)

    # Build SVG
    svg_markup, _ = build_svg(stations, f"{chosen_name} ({chosen_code})", ships=ships, player_pos=player_pos, size=args.size)

    # Build HTML
    css = """
    :root{--bg:#0b1320;--panel:#0f1c2e;--ink:#e8eef5;--muted:#9fb3c8;--accent:#4dabf7;--up:#2fb344;--down:#fa5252}
    *{box-sizing:border-box}
    body{margin:0;background:var(--bg);color:var(--ink);font:14px/1.45 -apple-system,BlinkMacSystemFont,Segoe UI,Roboto,Arial,sans-serif}
    header{padding:16px 20px;border-bottom:1px solid #1c2a38}
    header h1{margin:0;font-size:18px;font-weight:600}
    header .meta{color:var(--muted);font-size:12px;margin-top:2px}
    main{display:grid;grid-template-columns: 1fr 420px;gap:16px;padding:16px}
    .card{background:var(--panel);border:1px solid #1c2a38;border-radius:10px;overflow:hidden}
    .card h2{margin:0;padding:12px 14px;border-bottom:1px solid #1c2a38;font-size:14px;font-weight:600;color:var(--ink)}
    .map{padding:12px}
    .map svg{width:100%;height:auto;border-radius:8px;background:#fff}
    .side{display:flex;flex-direction:column;gap:16px}
    table{width:100%;border-collapse:collapse}
    th,td{padding:6px 8px;border-bottom:1px solid #1c2a38;font-variant-numeric:tabular-nums}
    th{color:var(--muted);text-align:left;font-weight:500}
    .num{text-align:right}
    .pill{display:inline-block;padding:1px 6px;border-radius:999px;background:#1c2a38;color:var(--muted);font-size:11px}
    .buy{color:var(--down)} .sell{color:var(--up)}
    details{border-top:1px dashed #1c2a38}
    details>summary{cursor:pointer;padding:10px 12px;color:var(--muted)}
    .offers{padding:8px 12px 12px}
    .offers .cols{display:grid;grid-template-columns:1fr 1fr;gap:16px}
    .offers h3{margin:6px 0;font-size:12px;color:var(--muted)}
    .footer{padding:10px 14px;color:var(--muted);font-size:12px;border-top:1px solid #1c2a38}
    """

    # ships table
    ships_sorted = sorted(ships, key=lambda x: (x['owner'], x['name']))
    ships_rows = []
    for sh in ships_sorted:
        owner = sh['owner']
        color = FACTION_COLORS.get(owner.lower(), '#7f8c8d')
        ship_class = sh.get('class', 'ship')
        # Simplify ship class display
        if ship_class.startswith('ship_'):
            ship_class = ship_class.replace('ship_', '').upper()
        ships_rows.append(
            f"<tr>"
            f"<td><svg width='10' height='10' style='vertical-align:middle'><polygon points='5,1 1,9 9,9' style='fill:{color};stroke:#fff;stroke-width:1'/></svg></td>"
            f"<td>{sh['name']}</td>"
            f"<td>{owner.capitalize()}</td>"
            f"<td>{ship_class}</td>"
            f"<td class='pill'>{sh['code']}</td>"
            f"</tr>"
        )
    if not ships_rows:
        ships_rows.append("<tr><td colspan='5' style='text-align:center;color:#9fb3c8'>Кораблей не найдено</td></tr>")
    ships_html = f"""
    <table>
      <thead>
        <tr>
          <th></th>
          <th>Название</th>
          <th>Владелец</th>
          <th>Класс</th>
          <th>Код</th>
        </tr>
      </thead>
      <tbody>
        {chr(10).join(ships_rows)}
      </tbody>
    </table>
    """

    # stations detail blocks
    station_blocks = []
    for s in stations:
        sells = sorted(s['sell'], key=lambda x: x['ware'])
        buys = sorted(s['buy'], key=lambda x: x['ware'])
        sell_rows = "\n".join(
            f"<tr><td>{w['ware']}</td><td class='num'>{int(w['price'])}</td><td class='num'>{w['amount']:,}</td></tr>"
            for w in sells[:50]
        ) or "<tr><td colspan='3' class='muted'>—</td></tr>"
        buy_rows = "\n".join(
            f"<tr><td>{w['ware']}</td><td class='num'>{int(w['price'])}</td><td class='num'>{w['amount']:,}</td></tr>"
            for w in buys[:50]
        ) or "<tr><td colspan='3' class='muted'>—</td></tr>"
        block = f"""
        <details>
          <summary>{s['name']} <span class='pill'>{s['code']}</span></summary>
          <div class='offers'>
            <div class='cols'>
              <div>
                <h3 class='sell'>Продаёт</h3>
                <table><thead><tr><th>Товар</th><th class='num'>Цена</th><th class='num'>Кол-во</th></tr></thead>
                <tbody>{sell_rows}</tbody></table>
              </div>
              <div>
                <h3 class='buy'>Покупает</h3>
                <table><thead><tr><th>Товар</th><th class='num'>Цена</th><th class='num'>Кол-во</th></tr></thead>
                <tbody>{buy_rows}</tbody></table>
              </div>
            </div>
          </div>
        </details>
        """
        station_blocks.append(block)
    stations_html = "\n".join(station_blocks)

    # arbitrage table
    arb_rows = []
    for r in results[:max(1, args.top)]:
        arb_rows.append(
            f"<tr>"
            f"<td>{r['ware']}</td>"
            f"<td>{r['buy_station']} <span class='pill'>{r['buy_code']}</span></td>"
            f"<td class='num'>{int(r['buy_price'])}</td>"
            f"<td>{r['sell_station']} <span class='pill'>{r['sell_code']}</span></td>"
            f"<td class='num'>{int(r['sell_price'])}</td>"
            f"<td class='num'>{int(r['unit_profit'])} <span class='sell'>+{r['percent_profit']:.1f}%</span></td>"
            f"<td class='num'>{r['qty']:,}</td>"
            f"<td class='num'>{int(r['total_profit']):,}</td>"
            f"</tr>"
        )
    if not arb_rows:
        arb_rows.append("<tr><td colspan='8' style='text-align:center;color:#9fb3c8'>Выгодных пар не найдено</td></tr>")
    arb_table = "\n".join(arb_rows)

    player_status = " • Игрок в секторе" if player_pos else ""

    # Count ships by faction and generate legend
    ships_by_faction = {}
    for sh in ships:
        owner = sh['owner'].lower()
        ships_by_faction[owner] = ships_by_faction.get(owner, 0) + 1

    legend_html = ""
    for faction, count in sorted(ships_by_faction.items(), key=lambda x: -x[1]):
        color = FACTION_COLORS.get(faction, '#7f8c8d')
        faction_name = faction.capitalize()
        legend_html += f"<span style='display:inline-block;margin-right:12px'><svg width='12' height='12' style='vertical-align:middle'><polygon points='6,2 2,10 10,10' style='fill:{color};stroke:#fff;stroke-width:1'/></svg> {faction_name}: {count}</span>"

    html = f"""
<!doctype html>
<html lang="ru">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Карта сектора — {chosen_name} ({chosen_code})</title>
  <style>{css}</style>
  <meta name="color-scheme" content="dark light" />
  <meta name="generator" content="x4-sector-map-html.py" />
</head>
<body>
  <header>
    <h1>Карта сектора — {chosen_name} ({chosen_code})</h1>
    <div class="meta">Станций: {len(stations)} • Кораблей: {len(ships)} • Арбитражных строк: {len(results)}{player_status}</div>
  </header>
  <main>
    <section class="card map">
      {{svg}}
      <div class="footer">
        <div style="margin-bottom:8px">Подсказка: наведите на точку — всплывёт название. Красная точка — игрок. ● — станции, ▲ — корабли.</div>
        <div style="font-size:11px;color:var(--muted)">Легенда фракций: {legend_html}</div>
      </div>
    </section>

    <aside class="side">
      <section class="card">
        <h2>Арбитраж в секторе</h2>
        <div style="overflow:auto; max-height: 50vh;">
          <table>
            <thead>
              <tr>
                <th>Товар</th>
                <th>Где купить</th>
                <th class="num">Цена</th>
                <th>Где продать</th>
                <th class="num">Цена</th>
                <th class="num">Приб/шт</th>
                <th class="num">Кол-во</th>
                <th class="num">Итого</th>
              </tr>
            </thead>
            <tbody>
              {{arb}}
            </tbody>
          </table>
        </div>
      </section>

      <section class="card">
        <h2>Станции и офферы</h2>
        <div style="overflow:auto; max-height: 45vh;">
          {{stations}}
        </div>
      </section>

      <section class="card">
        <h2>Корабли в секторе</h2>
        <div style="overflow:auto; max-height: 45vh;">
          {{ships}}
        </div>
      </section>
    </aside>
  </main>
</body>
</html>
    """.replace("{svg}", svg_markup).replace("{arb}", arb_table).replace("{stations}", stations_html).replace("{ships}", ships_html)

    out_path = args.out or f"sector-map-{chosen_code}.html"
    with open(out_path, 'w', encoding='utf-8') as f:
        f.write(html)
    player_msg = ", игрок в секторе" if player_pos else ""
    print(f"HTML сохранён: {out_path} (станций: {len(stations)}, кораблей: {len(ships)}, арбитраж: {len(results)}{player_msg})")


if __name__ == "__main__":
    main()
