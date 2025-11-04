"""
Localization helpers for X4 text tokens and t-files.

Responsibilities:
- Parse t-file XML pages for a chosen language id.
- Resolve text with embedded tokens like "{12,34}" recursively.
"""

from __future__ import annotations

import re
import xml.etree.ElementTree as ET
from typing import Dict, Iterable, Tuple


LangPages = Dict[str, Dict[str, str]]  # page_id -> entry_id -> text


def collect_pages(xml_docs: Iterable[Tuple[str, str]], lang_id: str) -> LangPages:
    """
    Build localization page map from provided (path, xml_text) documents.
    Only includes docs whose root has attribute id == lang_id.
    """
    pages: LangPages = {}
    for path, xml_text in xml_docs:
        try:
            root = ET.fromstring(xml_text)
        except ET.ParseError:
            continue
        if root.get("id") != lang_id:
            continue
        for page in root.findall(".//page"):
            pid = page.get("id")
            if not pid:
                continue
            bucket = pages.setdefault(pid, {})
            for t in page.findall("./t"):
                tid = t.get("id")
                if not tid:
                    continue
                bucket[tid] = t.text or ""
    return pages


_TOKEN_RE = re.compile(r"\{([^}]+)\}")


def resolve_text(text: str, pages: LangPages) -> str:
    """
    Replace occurrences of "{page,entry}" with localized strings recursively.
    Also strips trailing code tags in parentheses: "Name (ABC-123)" -> "Name ".
    """
    if not text:
        return text

    def repl(match: re.Match) -> str:
        spec = match.group(1)
        # Allow whitespace: "{ 12 , 34 }"
        parts = [p.strip() for p in spec.split(",", 1)]
        if len(parts) != 2:
            return match.group(0)
        page, entry = parts
        try:
            raw = pages[page][entry]
        except KeyError:
            return match.group(0)
        return resolve_text(raw, pages)

    out = _TOKEN_RE.sub(repl, text)
    # Drop any code suffix in parentheses, keeping the readable name only
    out = re.sub(r"(.*)\([^()]*\)$", r"\1", out).strip()
    return out

