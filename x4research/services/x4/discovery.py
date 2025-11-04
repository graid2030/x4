"""
Discovery filtering helpers derived from `knownto` attributes.

Responsibilities:
- Interpret the `knownto` attribute the game writes on sectors, stations,
  and ships to flag whether the player has discovered the object.
- Provide small utilities to apply the "hide undiscovered" behaviour that
  the original R script toggled via `spoilers.hide`.
"""

from __future__ import annotations

from typing import Callable, Iterable, List, TypeVar

T = TypeVar("T")


def is_known_to_player(knownto: str | None) -> bool:
    """Return True if the component is marked as known to the player."""
    if not knownto:
        return False
    # Values observed are either "player" or empty. Split defensively to
    # handle future multi-token variants (e.g., "player ally").
    tokens = [token.strip().lower() for token in knownto.split() if token.strip()]
    return "player" in tokens


def filter_by_spoilers(
    items: Iterable[T],
    hide_undiscovered: bool,
    *,
    get_knownto: Callable[[T], str | None],
) -> List[T]:
    """Filter items by discovery status when hide_undiscovered is True.

    Args:
        items: iterable of dataclass/dict rows.
        hide_undiscovered: mirrors the UI toggle; if False, items pass through.
        get_knownto: accessor returning the raw knownto string (or None).
    """
    if not hide_undiscovered:
        return list(items)
    return [item for item in items if is_known_to_player(get_knownto(item))]
