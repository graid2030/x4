NPC Knowledge

Player-Owned NPC Components
- Save path: `/savegame/universe//component[@class='npc' and @owner='player']`.
- Attributes exposed on the NPC component include `id`, `name`, optional `code`, and employment metadata (e.g., current `owner` always `player` after hire).

Skills & Traits
- Skill ratings can appear either as individual `traits/skill` nodes (with `type`/`value`) or as a single aggregated `traits/skills` element whose attributes map `skill→value`.
- Values are integers on a 0–10 scale; the in-game 0–5 star rating is simply `value / 2` (e.g., `8 -> 4★`, `5 -> 2.5★`).
- The set of skills per NPC can be pivoted into columns by aggregating the per-type values for a given `component@id`.

Role Resolution
- Employment roles derive from joins between NPC ids and station/ship posts.
- Ships expose `control/post` elements referencing NPC component ids for roles such as `aipilot` and `engineer`.
- Stations expose analogous posts (manager, engineer, shiptrader) under `control/post`. Matching these references to NPC ids establishes each NPC’s current assignment.
- Common `control/post@id` tokens: `aipilot`, `engineer`, `servicecrew`, `manager`, `shiptrader`. These map directly to the role strings surfaced in tooling (see `services/x4/npcs.py`).
