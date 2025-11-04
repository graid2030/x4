Game Data Structure

Scope
- Covers only the external game assets referenced by our current knowledge base (sector naming and related lookups).
- Paths are relative to the X4 installation directory (`X4_GAME_PATH`).

CAT/DAT Archives
- XML resources live inside paired `.cat` (index) and `.dat` (blob) archives.
- Base set: files like `01.cat`, `02.cat` in the install root.
- DLC/extension content: `extensions/<dlc_name>/<pack>.cat`.
- Reader: `services/x4/io_catdat.py` iterates entries by decoding the `.cat` index and slicing the `.dat` payload.

Localization Pages (t-files)
- Location: `t/<language_id>*.xml` within the archives (e.g., `t/0001-L044.xml`).
- Root attribute `id` equals the language id (e.g., `44` for English).
- Structure: `<page id="..."><t id="...">Text or {page,entry} tokens</t></page>`.
- Resolver: `services/x4/localization.collect_pages` builds a nested dict `page -> entry -> text`, and `resolve_text` recursively expands `{page,entry}` references.

Sector Datasets
- Location: XML under `libraries/` and `maps/` archives entries.
- Pattern: `<dataset macro="cluster_27_sector001_macro"><properties><identification name="{page,entry}"/></properties></dataset>`.
- Resolver: `services/x4/sector_resources.extract_sector_names` traverses datasets, lowers the macro id, and resolves the `identification@name` token via localization pages to produce the display name.
- Result consumed by tooling such as `tools/check_sector_name_resolution.py`.

Galaxy Objects (gates, accelerators, highways)
- Location: primarily in `assets/maps/xu_ep2_universe_macro.xml` and nested macros (`cluster_*/sector_*_macro`) with references to structural assets under `assets/structures/**`.
- Structure: `<component class="gate"|"accelerationgate"|"highway" ...>` nodes chained via `connections/connection/component`, same as stations.
- Positioning: world-space coordinates derived from cumulative `offset/position` and `offset/rotation` down the connection tree (our `services/x4/positions.py` logic applies unchanged).
- Metadata: optional `properties/identification@name` tokens localize gate/highway names; connection attributes link start/end sectors.

Faction Palette (Reference)
- Canonical colors stored in code (`services/x4/palette.py`); values sourced from the mapping visuals in X4 but not yet tied to an external resource file.
- Used by `services/x4/mapping.py` when constructing sector plot rows.
