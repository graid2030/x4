"""
Minimal mapping utilities for building sector plot data.

Focus: Combine cluster positions with sector macros and owners to yield
plot-ready rows including color via the shared palette.
"""

from typing import Dict, Iterable, List

from .palette import colour_for_owner


def build_sector_plot_data(
    clusters: Iterable[Dict],
    sector_index: Iterable[Dict],
    save_sectors: Iterable[Dict],
) -> List[Dict]:
    """Produce plot rows by joining clusters -> sectors.csv -> save sectors.

    Params:
    - clusters: rows with keys: macro, x, y
    - sector_index: rows with keys: cluster, macro  (sector macro)
    - save_sectors: rows with keys: macro, owner, contested, name, knownto

    Returns: list of dicts with x, y, owner, contested, name, knownto, colour
    """
    sector_by_macro: Dict[str, Dict] = {s["macro"]: s for s in save_sectors}
    out: List[Dict] = []
    for cl in clusters:
        cl_macro = cl.get("macro")
        for si in sector_index:
            if si.get("cluster") != cl_macro:
                continue
            sec = sector_by_macro.get(si.get("macro"))
            if not sec:
                continue
            owner = sec.get("owner")
            out.append(
                {
                    "x": cl.get("x"),
                    "y": cl.get("y"),
                    "owner": owner,
                    "contested": sec.get("contested", 0),
                    "name": sec.get("name"),
                    "knownto": sec.get("knownto"),
                    "colour": colour_for_owner(owner),
                }
            )
    return out

