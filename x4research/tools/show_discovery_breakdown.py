"""
Show discovery vs spoiler filtering counts for sectors, stations, and ships.

Usage mirrors the project style:
    PYTHONPATH=. python3 tools/show_discovery_breakdown.py --save <path> [--hide]
"""

from __future__ import annotations

import argparse
import sys
import xml.etree.ElementTree as ET
from typing import Iterable

from services.x4.discovery import filter_by_spoilers, is_known_to_player
from services.x4.save_parse import load_save_xml


def _iter_components(root: ET.Element, *, exact_class: str | None = None, prefix: str | None = None) -> Iterable[ET.Element]:
    """Yield components filtered by class or class prefix."""
    for comp in root.findall(".//component"):
        cls = comp.get("class") or ""
        if exact_class and cls == exact_class:
            yield comp
        elif prefix and cls.startswith(prefix):
            yield comp


def _count_known(components: Iterable[ET.Element]) -> tuple[int, int]:
    total = 0
    known = 0
    for comp in components:
        total += 1
        if is_known_to_player(comp.get("knownto")):
            known += 1
    return known, total


def build_summary(root: ET.Element) -> list[tuple[str, int, int]]:
    sectors = list(_iter_components(root, exact_class="sector"))
    stations = list(_iter_components(root, exact_class="station"))
    ships = list(_iter_components(root, prefix="ship"))

    summary = []
    for label, comps in (("sectors", sectors), ("stations", stations), ("ships", ships)):
        known, total = _count_known(comps)
        summary.append((label, known, total))
    return summary


def parse_args(argv: list[str]) -> argparse.Namespace:
    ap = argparse.ArgumentParser(description="Show discovery counts for sectors/stations/ships")
    ap.add_argument("--save", required=True, help="Path to an X4 save (xml or .gz)")
    ap.add_argument(
        "--hide",
        action="store_true",
        help="Apply spoilers filter (only show items known to the player)",
    )
    return ap.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    xml_text = load_save_xml(args.save)
    root = ET.fromstring(xml_text)

    rows = []
    for label, known, total in build_summary(root):
        visible = known if args.hide else total
        rows.append(
            {
                "category": label,
                "known": known,
                "total": total,
                "visible_when_hide": visible,
            }
        )

    print("category\tknown\ttotal\tvisible")
    for row in rows:
        visible = row["visible_when_hide"] if args.hide else row["total"]
        print(f"{row['category']}\t{row['known']}\t{row['total']}\t{visible}")

    if args.hide:
        for row in rows:
            comps = list(
                _iter_components(
                    root,
                    exact_class="sector" if row["category"] == "sectors" else None,
                    prefix="ship" if row["category"] == "ships" else None,
                )
            )
            if row["category"] == "stations":
                comps = list(_iter_components(root, exact_class="station"))
            filtered = filter_by_spoilers(comps, True, get_knownto=lambda c: c.get("knownto"))
            assert len(filtered) == row["visible_when_hide"], f"Mismatch for {row['category']}"
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
