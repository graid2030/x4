"""
Zone/Sector offsets extraction from X4 resource XMLs.

Responsibilities:
- Locate connections with ref='zones' and read offset/position and offset/rotation.
- Map found offsets by child <macro ref="..."> (lowercased key).
"""

from __future__ import annotations

import xml.etree.ElementTree as ET
from typing import Dict, Iterable, Tuple


def _float(attr: str | None) -> float:
    try:
        return float(attr) if attr is not None else 0.0
    except Exception:
        return 0.0


def extract_zone_offsets(xml_docs: Iterable[Tuple[str, str]]) -> Dict[str, dict]:
    """
    Return mapping: macro_ref(lowercased) -> {x,y,z,pitch,roll,yaw}
    """
    out: Dict[str, dict] = {}
    for path, xml_text in xml_docs:
        try:
            root = ET.fromstring(xml_text)
        except ET.ParseError:
            continue
        for conn in root.findall(".//connection[@ref='zones']"):
            pos = conn.find("./offset/position")
            rot = conn.find("./offset/rotation")
            has_any = pos is not None or rot is not None
            if not has_any:
                continue
            macro = conn.find("./macro")
            if macro is None:
                continue
            ref = macro.get("ref")
            if not ref:
                continue
            key = ref.lower()
            out[key] = {
                "x": _float(pos.get("x") if pos is not None else None),
                "y": _float(pos.get("y") if pos is not None else None),
                "z": _float(pos.get("z") if pos is not None else None),
                "pitch": _float(rot.get("pitch") if rot is not None else None),
                "roll": _float(rot.get("roll") if rot is not None else None),
                "yaw": _float(rot.get("yaw") if rot is not None else None),
            }
    return out

