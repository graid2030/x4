"""
Lightweight CAT/DAT reader for X4 resources.

Responsibilities:
- Iterate .cat index files and read XML payloads from paired .dat.
- Filter for relevant entries (XML files only).

No external dependencies; uses standard library only.
"""

from __future__ import annotations

import os
from dataclasses import dataclass
from typing import Iterable, Iterator, List, Optional


@dataclass
class CatEntry:
    path: str
    size: int
    offset: int


def _parse_cat_index(cat_path: str) -> List[CatEntry]:
    entries: List[CatEntry] = []
    offset = 0
    with open(cat_path, "r", encoding="utf-8", errors="replace") as fh:
        for line in fh:
            # Lines look like: "libraries/xyz.xml 1234 0 0"
            parts = line.rstrip("\n").rsplit(" ", 3)
            if len(parts) < 4:
                continue
            rel_path, size_s, _, _ = parts
            try:
                size = int(size_s)
            except ValueError:
                continue
            entries.append(CatEntry(path=rel_path, size=size, offset=offset))
            offset += size
    return entries


def _read_dat_slice(dat_path: str, offset: int, size: int) -> bytes:
    with open(dat_path, "rb") as fh:
        fh.seek(offset)
        return fh.read(size)


def iter_xml_from_cat(x4dir: str, cat_filename: str) -> Iterator[tuple[str, str]]:
    """
    Yield (relative_path, xml_text) for XML entries found in the given CAT.
    Silently skips non-XML entries.
    """
    cat_path = os.path.join(x4dir, cat_filename)
    dat_path = cat_path[:-3] + "dat"
    if not (os.path.isfile(cat_path) and os.path.isfile(dat_path)):
        return iter(())
    entries = _parse_cat_index(cat_path)
    for e in entries:
        if not e.path.lower().endswith(".xml"):
            continue
        data = _read_dat_slice(dat_path, e.offset, e.size)
        try:
            text = data.decode("utf-8", errors="replace")
        except Exception:
            continue
        yield e.path, text


def discover_cat_files(x4dir: str) -> List[str]:
    """
    Return a list of .cat filenames (basename only) from root and extensions.
    """
    cats: List[str] = []
    # Base game .cat files
    for name in os.listdir(x4dir):
        if name.lower().endswith(".cat"):
            cats.append(name)
    # Extensions/*/*.cat
    ext_dir = os.path.join(x4dir, "extensions")
    if os.path.isdir(ext_dir):
        for vendor in os.listdir(ext_dir):
            vpath = os.path.join(ext_dir, vendor)
            if not os.path.isdir(vpath):
                continue
            for name in os.listdir(vpath):
                if name.lower().endswith(".cat"):
                    cats.append(os.path.join("extensions", vendor, name))
    # Deduplicate while preserving order
    seen = set()
    unique: List[str] = []
    for c in cats:
        if c not in seen:
            unique.append(c)
            seen.add(c)
    return unique

