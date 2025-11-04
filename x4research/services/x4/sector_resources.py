"""
Sector resource extraction from X4 CAT/DAT XMLs.

Responsibilities:
- Scan XML docs for <dataset macro=...><properties><identification name=...>
- Resolve name tokens via provided localization pages map.
"""

from __future__ import annotations

import xml.etree.ElementTree as ET
from typing import Dict, Iterable, Tuple

from .localization import LangPages, resolve_text


def extract_sector_names(xml_docs: Iterable[Tuple[str, str]], pages: LangPages) -> Dict[str, str]:
    """
    Return mapping of sector macro (lowercased) -> display name.
    """
    mapping: Dict[str, str] = {}
    for path, xml_text in xml_docs:
        # Focus on files that plausibly carry sector datasets to keep it efficient
        low = path.lower()
        if not (low.endswith(".xml") and ("/libraries/" in low or "/maps/" in low or "\\libraries\\" in low)):
            # Still parse in case; cheap guard to skip obvious non-lib docs
            pass
        try:
            root = ET.fromstring(xml_text)
        except ET.ParseError:
            continue
        for dataset in root.findall(".//dataset"):
            macro = dataset.get("macro")
            if not macro:
                continue
            ident = dataset.find(".//properties/identification")
            if ident is None:
                continue
            token = ident.get("name")
            if not token:
                continue
            name = resolve_text(token, pages)
            if name:
                mapping[macro.lower()] = name
    return mapping

