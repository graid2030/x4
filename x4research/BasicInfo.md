Basic Info

Structure References
- Save XML paths documented in `SaveStructure.md`.
- External resource lookups documented in `GameDataStructure.md`.
- Economy/trade log details documented in `EconomyInfo.md`.

Save Sectors
- Definition: components with class='sector' in the save-game XML.
- Path: /savegame/universe//component[@class='sector']
- Relevant attributes: id, code, macro, owner, knownto, contested.

Sector Name
- Input: sector.macro from the save.
- Source: game resources datasets with properties/identification@name.
- Resolution: resolve token via language t-files (localization pages).
- Output: display name (e.g., Argon Prime).

Conventions
- Codes like LRG-080 are display codes, stored on sector components as @code in the save.
- Owner values are faction ids (e.g., argon, teladi, player); map to display names via localization if needed.

Faction Colors
- Palette entries track canonical faction colors used across visualizations.
- Owner -> hex examples: teladi -> #a2b927 (yellow-green), argon -> #0450f2.
- Keep the palette definitions centralized in `services/x4/palette.py`.

Discovery & Spoilers
- Discovery state is stored per component via the `knownto` attribute.
- Items known to the player set `knownto="player"`; undiscovered entries omit the flag.
- The "Hide undiscovered" toggle filters sectors, stations, and ships to only those with `knownto` containing the player token.
- See `services/x4/discovery.py` for reusable filtering helpers.

Player Profile
- Current saves store the player name and liquid credits on the root `<player>` element (`@name`, `@money`).
- `@money` is an integer string in credits (e.g., `319082` Cr in `quicksave.xml.gz`).
- Player metadata feeds the asset overview/tool headers and the logbook sidebar.

Ships
- See ShipInfo.md for ship identification, size derivation, pilot assignment, and commander/subordinate links.
