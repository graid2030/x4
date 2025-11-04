"""
NPC parsing helpers for player-employed crew in X4 save games.

Responsibilities:
- Extract player-owned NPC components together with core attributes.
- Collect skill ratings from `traits/skill` entries.
- Resolve current assignments by joining `control/post` references on ships/stations.
"""

from __future__ import annotations

import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from typing import Dict, List, Optional


ROLE_HINTS = {
    "aipilot": "pilot (ship)",
    "pilot": "pilot",
    "engineer": "engineer",
    "servicecrew": "service crew",
    "manager": "manager (station)",
    "shiptrader": "shiptrader (station)",
}


@dataclass
class NpcAssignment:
    """Represents one control/post link that references an NPC."""

    parent_id: Optional[str]
    parent_class: Optional[str]
    post_id: Optional[str]

    def role_hint(self) -> Optional[str]:
        if not self.post_id:
            return None
        return ROLE_HINTS.get(self.post_id.lower())


@dataclass
class PlayerNpc:
    """Structured view of a player-employed NPC."""

    id: Optional[str]
    name: Optional[str]
    code: Optional[str]
    owner: Optional[str]
    skills: Dict[str, int] = field(default_factory=dict)
    assignments: List[NpcAssignment] = field(default_factory=list)

    @property
    def primary_role(self) -> Optional[str]:
        """Derive a readable role label from the first matching assignment."""

        for assignment in self.assignments:
            hint = assignment.role_hint()
            if hint:
                return hint
        return None


def _collect_player_npcs(root: ET.Element) -> Dict[str, PlayerNpc]:
    """Return mapping of player NPC id -> PlayerNpc without assignments."""

    npcs: Dict[str, PlayerNpc] = {}
    for comp in root.findall(".//component[@class='npc']"):
        if comp.get("owner") != "player":
            continue
        npc_id = comp.get("id")
        if not npc_id:
            continue
        skills: Dict[str, int] = {}
        for skill in comp.findall("./traits/skill"):
            stype = skill.get("type")
            if not stype:
                continue
            try:
                value = int(skill.get("value", "0"))
            except ValueError:
                value = 0
            skills[stype] = value
        skills_block = comp.find("./traits/skills")
        if skills_block is not None:
            for stype, raw in skills_block.attrib.items():
                try:
                    value = int(raw)
                except ValueError:
                    continue
                # Preserve explicit `traits/skill` values if already filled.
                skills.setdefault(stype, value)
        npcs[npc_id] = PlayerNpc(
            id=npc_id,
            name=comp.get("name"),
            code=comp.get("code"),
            owner=comp.get("owner"),
            skills=skills,
        )
    return npcs


def _collect_assignments(root: ET.Element) -> Dict[str, List[NpcAssignment]]:
    """Scan all components for control/post references to NPC ids."""

    assignments: Dict[str, List[NpcAssignment]] = {}
    for comp in root.findall(".//component"):
        posts = comp.findall("./control/post")
        if not posts:
            continue
        parent_id = comp.get("id") or comp.get("code")
        for post in posts:
            npc_id = post.get("component") or post.get("ref")
            if not npc_id:
                continue
            assignments.setdefault(npc_id, []).append(
                NpcAssignment(
                    parent_id=parent_id,
                    parent_class=comp.get("class"),
                    post_id=post.get("id"),
                )
            )
    return assignments


def parse_player_npcs(save_xml: str) -> List[PlayerNpc]:
    """Parse all player-employed NPCs together with skills and assignments."""

    root = ET.fromstring(save_xml)
    npcs = _collect_player_npcs(root)
    if not npcs:
        return []

    assignments = _collect_assignments(root)
    for npc_id, posts in assignments.items():
        npc = npcs.get(npc_id)
        if npc:
            npc.assignments.extend(posts)
    return list(npcs.values())
