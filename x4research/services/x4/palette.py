"""
Faction palette and owner-to-color mapping for X4.

Sourced to match the palette used in the original X4SaveGameAnalysis.R.
This module centralizes color mapping so tools can stay consistent.
"""

from typing import Dict


FACTION_LEVELS = [
    "PLA", "ARG", "ANT", "TEL", "MIN", "HOP",
    "PAR", "ALI", "HAT", "SCA", "ZYA", "FRF",
    "FAF", "XEN", "KHK", "PIO", "BUC", "RIP",
    "TER", "VIG", "YAK", "NIL",
]

# Display names aligned by position with FACTION_LEVELS (kept for reference)
FACTION_NAMES = [
    "Player", "Argon Federation", "Antigone Republic", "Teladi Company", "Ministry of Finance",
    "Holy Order of the Pontifex", "Godrealm of the Paranid", "Alliance of the Word",
    "Hatikvah Free League", "Scale Plate Pact", "Zyarth Patriarchy", "Free Families",
    "Fallen Families", "Xenon", "Kha'ak", "Segaris Pioneers", "Duke's Buccaneers",
    "Riptide Rakers", "Terran Protectorate", "Vigour Syndicate", "Yaki", "Ownerless",
]

# Hex colors aligned by position with FACTION_LEVELS
FACTION_COLOURS = [
    "#33f23a", "#0450f2", "#4c91d3", "#a2b927", "#8eb48c", "#f26ca5",
    "#7a03f2", "#aa37c2", "#1beaf0", "#7e7732", "#fc691a", "#f19600",
    "#ff4848", "#c10200", "lightpink", "#39ad9b", "#5500f4", "#5683a3",
    "#bdd2fb", "#988397", "#fe8ffa", "#808080",
]

# Owner ids as appear in saves/resources, aligned to FACTION_LEVELS positions
SECTOR_OWNERS = [
    "player", "argon", "antigone", "teladi", "ministry", "holyorder",
    "paranid", "alliance", "hatikvah", "scaleplate", "split", "freesplit",
    "fallensplit", "xenon", "khaak", "pioneers", "buccaneers", "scavenger",
    "terran", "loanshark", "yaki", "ownerless",
]


def owner_to_colour_map() -> Dict[str, str]:
    """Return mapping of sector owner id (e.g., 'teladi') to hex color.

    The mapping indexes into FACTION_COLOURS using the shared position across
    SECTOR_OWNERS and FACTION_LEVELS.
    """
    return {owner: FACTION_COLOURS[i] for i, owner in enumerate(SECTOR_OWNERS)}


def colour_for_owner(owner: str) -> str:
    """Get hex color for a given sector owner id. Falls back to gray if unknown."""
    return owner_to_colour_map().get(owner, "#808080")


def is_yellowish(hex_color: str) -> bool:
    """Heuristic: classify a hex color as yellowish via HSV hue range.

    Accept ~50-80 degrees as yellow range; return False if parsing fails.
    """
    try:
        hc = hex_color.lstrip('#')
        if len(hc) == 6:
            r, g, b = int(hc[0:2], 16), int(hc[2:4], 16), int(hc[4:6], 16)
        else:
            # Named colors like 'lightpink' -> not yellow
            return False
    except Exception:
        return False

    mx, mn = max(r, g, b), min(r, g, b)
    if mx == mn:
        return False
    if mx == r:
        h = (60 * (g - b) / (mx - mn)) % 360
    elif mx == g:
        h = 60 * (2 + (b - r) / (mx - mn))
    else:
        h = 60 * (4 + (r - g) / (mx - mn))
    return 50 <= h <= 80

