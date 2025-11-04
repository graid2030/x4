#!/usr/bin/env python3
"""
List sectors with ownership and discovery flags from an X4 save.

- If --save is omitted, auto-detect the newest save under
  "~/Documents/Egosoft/X4/<profile>/save".
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path
from typing import List

from services.x4.save_parse import iter_sectors_info


def _find_latest_save() -> str:
    docs = Path.home() / "Documents" / "Egosoft" / "X4"
    if not docs.exists():
        raise FileNotFoundError("Egosoft/X4 Documents folder not found")
    candidates: list[Path] = []
    for profile in docs.iterdir():
        if not profile.is_dir():
            continue
        save_dir = profile / "save"
        if not save_dir.exists():
            continue
        for p in save_dir.iterdir():
            if re.search(r"\.xml(\.gz)?$", p.name, re.IGNORECASE):
                candidates.append(p)
    if not candidates:
        raise FileNotFoundError("No save files found in Egosoft/X4/*/save")
    candidates.sort(key=lambda p: p.stat().st_mtime, reverse=True)
    return str(candidates[0])


def main(argv: List[str]) -> int:
    ap = argparse.ArgumentParser(description="List X4 sectors with owner/knownto/contested")
    ap.add_argument("--save", help="Optional path to save (.xml or .gz)")
    args = ap.parse_args(argv)

    save = args.save or _find_latest_save()
    rows = list(iter_sectors_info(save))

    print("code\towner\tknownto\tcontested\tmacro")
    for s in rows:
        code = s.code or ""
        owner = s.owner or ""
        knownto = s.knownto or ""
        contested = s.contested or ""
        macro = (s.macro or "").lower()
        print("\t".join([code, owner, knownto, contested, macro]))
    return 0


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main(sys.argv[1:]))

