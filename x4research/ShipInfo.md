Ship Info

Scope
- Canonical rules for identifying ships, deriving size, pilot assignment, and fleet hierarchy from save-game XML.
- Keep cross-cutting fallbacks (e.g., name resolution cascades) in BasicInfo.md.

Identification
- Save path: `/savegame/universe//component[starts-with(@class,'ship')]`
- Key attributes: `@id`, `@code`, `@macro`, `@class`
- Default ship names come from the macro definition:
  - Resource location: `assets/units/ships/**/<ship_macro>.xml` inside CAT/DAT archives.
  - Each `<macro>` includes `properties/identification@name` pointing to a localization token (e.g., `{20101,10702}`).
  - Resolve tokens via the language t-files (e.g., `t/0001-L044.xml`) using `services.x4.localization.collect_pages`/`resolve_text`.
  - `data/x4-cat-miner.py` already builds a reusable mapping (`x4-component-names.json`) for all macros; ad-hoc lookups can reuse that data instead of re-reading CATs.

Size Derivation
- Input: `@class` string, forms like `ship_s`, `ship_m`, `ship_l`, `ship_xl`.
- Rule: suffix → size token: `s→S`, `m→M`, `l→L`, `xl→XL`.
- Service: `services/x4/ships.py:parse_size_from_class`.

Pilot Assignment
- Save path: `./control/post@ref` on the ship component.
- Output: component id of the assigned pilot (NPC).
- Service: emitted as `pilot_id` in `services/x4/ships.py:iter_ships_info`.

Fleet Hierarchy
- Commander link path: `./connections/connection[@name='commander']`.
- Commander id: prefer `@ref`; fallback to embedded `./component/@id|@code`.
- Subordinates: all ships whose commander id equals this ship's id.
- Service: `commander_id` and `subordinates` in `iter_ships_info`.

Equipment & Software
- `connections/connection/component` holds installed modules:
  - class `weapon` → hardpoint weapon macro (primary/secondary).
  - class `shieldgenerator` → shield slot contents.
  - class `engine` → engine slots (expect duplicates when the hull exposes multiple engines).
- The ship component itself stores the thruster via `@thruster=<thruster_macro>`.
- `software@wares` lists installed ship software as space-separated ware ids.
- Deployables (probes, satellites, mines) appear as `<item macro=* amount=*>` under the ship's storage subtree.
- Cargo capacity and fill:
  - Attached storage macro (class `storage`) exposes the maximum via `properties/cargo@max` (with `tags` for container/solid/liquid).
  - Actual cargo lives under the storage component as `<ware ware=* amount=* volume=*>`; multiply `amount*volume` to obtain filled cubic meters.

Naming Fallbacks (reference)
- Prefer: custom name → model name → macro id.
- Keep fallback definition centralized in BasicInfo.md.

Sample Save Observation
- Example derived from parsed save data validated by `tools/selfcheck_ships_fleets.py`.
- Component `XNU-614`: `macro=ship_arg_m_transporter_01_a_macro`, size `M`, commander `None`.
- Pilot component resolved via `control/post` mapping; name sourced from NPC component traits.
