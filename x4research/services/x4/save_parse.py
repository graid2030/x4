"""
Save-game parsing helpers focused on sectors.

Responsibilities:
- Open X4 save (.gz or raw XML) and parse sector components.
- Provide lightweight iterators for sector name/ownership attributes.
"""

from __future__ import annotations

import gzip
import io
import os
import xml.etree.ElementTree as ET
from dataclasses import dataclass
from typing import Iterator


@dataclass
class SectorRef:
    code: str | None
    macro: str | None


@dataclass
class SectorInfo:
    id: str | None
    code: str | None
    macro: str | None
    owner: str | None
    knownto: str | None
    contested: str | None


def _open_text(path: str) -> str:
    with open(path, "rb") as fh:
        head = fh.read(2)
        fh.seek(0)
        data = fh.read()
    if path.lower().endswith(".gz") or head == b"\x1f\x8b":
        return gzip.decompress(data).decode("utf-8", errors="replace")
    return data.decode("utf-8", errors="replace")


def iter_sectors(save_path: str) -> Iterator[SectorRef]:
    """
    Yield SectorRef for each component with class='sector'.
    """
    xml_text = _open_text(save_path)
    root = ET.fromstring(xml_text)
    for comp in root.findall(".//component"):
        if comp.get("class") != "sector":
            continue
        yield SectorRef(code=comp.get("code"), macro=comp.get("macro"))


def iter_sectors_info(save_path: str) -> Iterator[SectorInfo]:
    """
    Yield SectorInfo with ownership and discovery attributes for each sector.
    Fields are raw strings from the save attributes for minimal coupling.
    """
    xml_text = _open_text(save_path)
    root = ET.fromstring(xml_text)
    for comp in root.findall(".//component"):
        if comp.get("class") != "sector":
            continue
        yield SectorInfo(
            id=comp.get("id"),
            code=comp.get("code"),
            macro=comp.get("macro"),
            owner=comp.get("owner"),
            knownto=comp.get("knownto"),
            contested=comp.get("contested"),
        )


def load_save_xml(save_path: str) -> str:
    """Return the raw XML text from an X4 save path (.gz supported)."""
    return _open_text(save_path)
