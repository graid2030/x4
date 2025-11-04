Station Knowledge

Component Structure
- Stations appear under `/savegame/universe//component[@class='station']`.
- Common attributes: `id`, `macro`, `owner`, `code`, `name`. Positional data is provided through nested `offset/position` and `offset/rotation` elements so world coordinates can be reconstructed.
- Connected subcomponents (docks, modules, turrets) sit under `connections/connection/component` chains, matching the station's build layout.

Construction Sequence
- Build state is captured in `construction/sequence/entry` nodes.
- Each `entry` includes an `index` attribute that increments per module in the build plan; the set of entries therefore enumerates every module that has been queued or completed.
- The maximum `index` (or simply the number of `entry` elements) mirrors the total module count for the station, covering both finished and pending modules.

Personnel Attachments
- Crew assignments use `control/post` children. Attributes include `id` (role slot identifier) and `component` (references the NPC component occupying the slot).
- Workforce population is maintained under `workforces/workforce` with attributes like `race` and `amount`, allowing per-species workforce totals to be read directly from the save.

Naming & Sources
- Factory labels derive from the `<source>` element on station components.
  - `source@entry` follows `<faction>_<ware>` (e.g., `arg_hullparts`), which maps to the ware’s localized name via game resources.
  - `source@nameindex` (or station attribute `nameindex`) increments per factory instance; the UI renders Roman numerals (`1 -> I`, `2 -> II`, ...).
- Special/mission structures expose `source@class="script"` with no ware entry; keep fallback naming (e.g., `ARG Scripted Station KYC-722`).
- Player custom names persist on the station element itself (`@name="Solar Start"`), overriding any factory-derived label.
