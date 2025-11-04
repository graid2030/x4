Save Structure

Scope
- Capture only the parts of the save-game XML already documented elsewhere in this project.
- Paths are relative to the root `<savegame>` element.

Universe Layout
- Base path: `/savegame/universe/component`
- Every in-universe object (sector, station, ship, npc, etc.) is expressed as a `<component>` node with a `class` attribute that identifies its type.

Sectors
- Path: `/savegame/universe//component[@class='sector']`
- Attributes: `id`, `code` (display code like `LRG-080`), `macro`, `owner`, `knownto`, `contested`.
- Knownto drives discovery: `knownto="player"` ⇒ visible when spoilers are hidden; empty or missing ⇒ still undiscovered.
- See also: `SectorInfo.md` for ownership logic, `BasicInfo.md` for discovery details.

Stations
- Path: `/savegame/universe//component[@class='station']`
- Attributes: `id`, `code`, `macro`, `owner`, optional `name`.
- Position: aggregated from nested `offset/position` values along the connection chain (see `services/x4/positions.py`).
- Construction: `./construction/sequence/entry` nodes enumerate modules; the highest `index` matches the module count.
- Production metadata: `<source>` child carries `entry` (ware/faction key) and optional `class` (`script` for story/special stations); `nameindex` increments per factory and maps to the UI suffix.
- Crew slots: `./control/post` where `@component` points to an NPC id (roles like manager, engineer, shiptrader).
- Workforce: `./workforces/workforce` carries per-race counts (`@race`, `@amount`).
- Trade offers: `<trade>` elements under the station component expose marketplace data with attributes `ware`, `price` (centi-credits), `amount`, `buyer`/`seller` (presence determines buy vs sell offer). Used by arbitrage scripts to pair stations.

Player Profile
- Path: `/savegame/player`
- Attributes: `name` (current player name), `money` (credits as integer string), `location` (token referencing map position).
- Downstream usage: player name/credits shown in Property Owned, logbook headers, and asset overviews.

Ships
- Path: `/savegame/universe//component[starts-with(@class,'ship')]`
- Attributes: `id`, `code`, `macro`, `class`.
- Size from `@class` suffix (`ship_s`, `ship_m`, etc.); parsed via `services/x4/ships.py:parse_size_from_class`.
- Pilot post: `./control/post[@id in {'aipilot','pilot','playerpilot'}]` provides a `@component`/`@ref` pointing to the pilot NPC.
- Fleet chain: `./connections/connection[@name='commander']` links to commander ship via `@ref` (or embedded `<component>` id).
- Outputs consumed by `services/x4/ships.py:iter_ships_info`.

NPCs
- Path: `/savegame/universe//component[@class='npc' and @owner='player']`
- Attributes: `id`, `name`, optional `code`; `@owner` remains `player` after hire.
- Skills:
  - Variant 1: multiple `./traits/skill` nodes with `@type` and integer `@value` (0–10 scale).
  - Variant 2: a single `./traits/skills` element with attributes `skill=value`.
  - Stars in UI equal `value / 2`.
- Roles inferred by matching the NPC id to station/ship `control/post@component` references (see `services/x4/npcs.py`).

Resource Areas
- Path: `/savegame/universe//component[@class='sector']/resourceareas/resourcearea`
- Attributes per resourcearea: `macro`, `ware`, `max`, `time`.
- Recharge per ware = `max / time`; totals per sector come from summing all entries that share the same sector macro.
- Aggregation flow described in `ResourceInfo.md`.

Economy Log
- Base node: `/savegame/economylog`
- Trade entries: `/savegame/economylog/entries[@type='trade']/log` (see `EconomyInfo.md`).
- Removed object lookup: `/savegame/economylog/removed/object` retains `{id,name,code,owner}` for ships/stations referenced after removal.
