"""Utility to display player NPC details for a given ship code."""

from __future__ import annotations

import argparse
import json
from typing import Any, Dict

from services.x4.npcs import parse_player_npcs
from services.x4.save_parse import load_save_xml
from services.x4.ships import iter_ships_info


def build_ship_index(save_xml: str) -> Dict[str, Dict[str, Any]]:
    """Index ships by code/id with pilot references."""

    ships: Dict[str, Dict[str, Any]] = {}
    for info in iter_ships_info(save_xml):
        key = info.code or info.id
        if not key:
            continue
        ships[key] = {
            "id": info.id,
            "code": info.code,
            "macro": info.macro,
            "class": info.class_str,
            "size": info.size,
            "pilot_id": info.pilot_id,
            "commander_id": info.commander_id,
            "subordinates": info.subordinates,
        }
    return ships


def find_npc_by_ship_code(save_path: str, ship_code: str) -> Dict[str, Any] | None:
    """Return ship and pilot NPC data for the requested ship code."""

    xml_text = load_save_xml(save_path)
    ships = build_ship_index(xml_text)
    ship = ships.get(ship_code)
    if not ship:
        return None

    npcs = {npc.id: npc for npc in parse_player_npcs(xml_text)}
    pilot_id = ship.get("pilot_id")
    npc = npcs.get(pilot_id)
    if not npc:
        return {"ship": ship, "npc": None}

    return {
        "ship": ship,
        "npc": {
            "id": npc.id,
            "name": npc.name,
            "code": npc.code,
            "owner": npc.owner,
            "skills": npc.skills,
            "primary_role": npc.primary_role,
            "assignments": [
                {
                    "parent_id": a.parent_id,
                    "parent_class": a.parent_class,
                    "post_id": a.post_id,
                }
                for a in npc.assignments
            ],
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("save", help="Path to the X4 save (xml or gz).")
    parser.add_argument("ship_code", help="Ship code or id (e.g., TNE-375).")
    args = parser.parse_args()

    details = find_npc_by_ship_code(args.save, args.ship_code)
    if not details:
        print(f"No ship found for code {args.ship_code}")
        return
    print(json.dumps(details, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
