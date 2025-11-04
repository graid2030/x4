"""
Ship and fleet parsing utilities from X4 save XML (simplified).

Responsibilities:
- Identify ships and derive size from class suffix (s/m/l/xl).
- Extract pilot assignment from control/post reference.
- Capture commander/subordinate hierarchy via connections.

Notes:
- Implemented to be robust against minimal fixtures; real saves may vary.
- Keep focused and under 200–250 lines per project standards.
"""

from __future__ import annotations

import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from typing import Dict, List, Optional


SIZE_ORDER = ("s", "m", "l", "xl")
PILOT_POST_IDS = {"aipilot", "pilot", "playerpilot"}


def parse_size_from_class(class_str: Optional[str]) -> Optional[str]:
    """Derive size token (S/M/L/XL) from a class string.

    Accepts forms like 'ship_s', 'ship_m', 'ship_l', 'ship_xl'.
    Returns 'S','M','L','XL' or None if not detected.
    """
    if not class_str:
        return None
    parts = class_str.lower().split("_")
    if len(parts) >= 2 and parts[-1] in SIZE_ORDER and parts[0] == "ship":
        token = parts[-1]
        return token.upper() if token != "xl" else "XL"
    return None


@dataclass
class ShipInfo:
    id: Optional[str]
    code: Optional[str]
    macro: Optional[str]
    class_str: Optional[str]
    size: Optional[str]
    pilot_id: Optional[str]
    commander_id: Optional[str]
    subordinates: List[str] = field(default_factory=list)


def iter_ships_info(save_xml: str) -> List[ShipInfo]:
    """Parse ships with size, pilot and commander references.

    Returns list; subordinates lists are populated by reversing commander links.
    """
    root = ET.fromstring(save_xml)
    # Collect ship elements first
    ship_elems: Dict[str, ET.Element] = {}
    for comp in root.findall(".//component"):
        cls = comp.get("class") or ""
        if not cls.lower().startswith("ship"):
            continue
        sid = comp.get("id") or comp.get("code") or ""
        if sid:
            ship_elems[sid] = comp

    # First pass: basic fields and commander links
    ships: Dict[str, ShipInfo] = {}
    commander_of: Dict[str, Optional[str]] = {}
    for sid, comp in ship_elems.items():
        size = parse_size_from_class(comp.get("class"))
        pilot_id = None
        for post in comp.findall("./control/post"):
            post_id = (post.get("id") or "").lower()
            if post_id not in PILOT_POST_IDS:
                continue
            pilot_id = post.get("component") or post.get("ref")
            if pilot_id:
                break
        commander_id = None
        conn = comp.find("./connections/connection[@name='commander']")
        if conn is not None:
            # Prefer explicit ref attr
            commander_id = conn.get("ref")
            # Or embedded component id
            if not commander_id:
                ccomp = conn.find("./component")
                if ccomp is not None:
                    commander_id = ccomp.get("id") or ccomp.get("code")
        ships[sid] = ShipInfo(
            id=comp.get("id"),
            code=comp.get("code"),
            macro=comp.get("macro"),
            class_str=comp.get("class"),
            size=size,
            pilot_id=pilot_id,
            commander_id=commander_id,
        )
        commander_of[sid] = commander_id

    # Second pass: populate subordinates
    for sid, commander_id in commander_of.items():
        if commander_id and commander_id in ships:
            ships[commander_id].subordinates.append(sid)

    return list(ships.values())
