"""
World position accumulation for stations within a sector (simplified).

Responsibilities:
- Traverse save component graph under a sector, summing offset/position.
- Yield stations with computed world positions (ignores rotation math for now).
"""

from __future__ import annotations

import xml.etree.ElementTree as ET
from dataclasses import dataclass
from typing import Iterator, Tuple


@dataclass
class StationWorld:
    sector_code: str | None
    station_code: str | None
    station_macro: str | None
    x: float
    y: float
    z: float


def _pos_tuple(elem: ET.Element | None) -> Tuple[float, float, float]:
    if elem is None:
        return 0.0, 0.0, 0.0
    return (
        float(elem.get("x", "0") or 0.0),
        float(elem.get("y", "0") or 0.0),
        float(elem.get("z", "0") or 0.0),
    )


def iter_stations_with_world_pos(save_xml: str) -> Iterator[StationWorld]:
    root = ET.fromstring(save_xml)
    for sector in root.findall(".//component[@class='sector']"):
        sector_code = sector.get("code")

        def walk(comp: ET.Element, acc: Tuple[float, float, float]):
            cx, cy, cz = _pos_tuple(comp.find("./offset/position"))
            acc2 = (acc[0] + cx, acc[1] + cy, acc[2] + cz)
            if comp.get("class") == "station":
                yield StationWorld(
                    sector_code=sector_code,
                    station_code=comp.get("code"),
                    station_macro=comp.get("macro"),
                    x=acc2[0],
                    y=acc2[1],
                    z=acc2[2],
                )
            # traverse nested connection -> component chains
            for child in comp.findall("./connections/connection/component"):
                yield from walk(child, acc2)

        # start walking from the sector node with zero acc; walk adds sector offset
        yield from walk(sector, (0.0, 0.0, 0.0))
