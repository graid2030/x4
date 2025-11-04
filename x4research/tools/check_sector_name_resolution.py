#!/usr/bin/env python3
"""
Check Hypothesis 1: Sector Name Resolution from Game Resources.

Inputs:
- --save <path>       : X4 save file (.xml or .gz)
- --x4dir <path>      : X4 install directory with .cat/.dat files
- --langid <id>       : language id, default 44 (English)
- --sector-code <code>: optional, filter to a single sector code (e.g., LRG-080)
- --expected <text>   : optional, compare against expected string "Name(CODE)"

Output:
- Prints table: code, macro, resolved_name, combined "Name(CODE)"
- If --expected provided, prints PASS/FAIL summary and exits with code 0/1 accordingly.
"""

from __future__ import annotations

import argparse
import sys
from typing import Dict, List, Tuple

from services.x4.io_catdat import discover_cat_files, iter_xml_from_cat
from services.x4.localization import collect_pages
from services.x4.sector_resources import extract_sector_names
from services.x4.save_parse import iter_sectors


def build_sector_name_map(x4dir: str, langid: str) -> Dict[str, str]:
    # Two-stage: collect localization pages then map sectors.
    cat_files = discover_cat_files(x4dir)
    # Prefer earlier core files to limit work but include all for completeness
    xml_stream_all = []
    for cat in cat_files:
        for rel_path, text in iter_xml_from_cat(x4dir, cat):
            xml_stream_all.append((rel_path, text))

    pages = collect_pages(xml_stream_all, langid)
    return extract_sector_names(xml_stream_all, pages)


def main(argv: List[str]) -> int:
    ap = argparse.ArgumentParser(description="Verify sector name resolution from game resources")
    ap.add_argument("--save", required=True, help="Path to X4 save (.xml or .gz)")
    ap.add_argument("--x4dir", required=True, help="Path to X4 install with .cat/.dat")
    ap.add_argument("--langid", default="44", help="Language id (default 44)")
    ap.add_argument("--sector-code", dest="sector_code", help="Filter by sector code, e.g. LRG-080")
    ap.add_argument("--expected", help='Expected string to compare (e.g., "Argon Prime(LRG-080)")')
    args = ap.parse_args(argv)

    # Parse sectors from save
    sectors = list(iter_sectors(args.save))
    if args.sector_code:
        sectors = [s for s in sectors if (s.code or "").upper() == args.sector_code.upper()]
        if not sectors:
            print(f"No sector with code {args.sector_code} found in save.")
            return 2

    # Build sector name map from resources
    name_map = build_sector_name_map(args.x4dir, args.langid)

    # Emit results and perform optional verification
    rows: List[Tuple[str, str, str, str]] = []
    combined_values: List[str] = []
    for s in sectors:
        macro = (s.macro or "").lower()
        code = s.code or ""
        name = name_map.get(macro, "<unresolved>")
        combined = f"{name}({code})" if name != "<unresolved>" and code else name
        rows.append((code, macro, name, combined))
        combined_values.append(combined)

    # Print compact table
    print("code\tmacro\tname\tcombined")
    for r in rows:
        print("\t".join(r))

    if args.expected is not None:
        outcome = "PASS" if args.expected in combined_values else "FAIL"
        print(f"Expected: {args.expected} -> {outcome}")
        return 0 if outcome == "PASS" else 1

    return 0


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main(sys.argv[1:]))

