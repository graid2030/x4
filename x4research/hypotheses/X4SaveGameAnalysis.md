X4SaveGameAnalysis.R - Derived Research Hypotheses

Scope
- Source: C:\Users\graid\Downloads\X4SaveGameAnalysisRelease\X4SaveGameAnalysis\X4SaveGameAnalysis.R
- Purpose: enumerate concrete, testable hypotheses suggested by the script's logic to drive our modular tools and docs.
- Notes: save-path details belong in SaveStructure.md; resource references in GameDataStructure.md.

Status Key
- [PROPOSED] not yet tested
- [VERIFIED] validated and promoted to knowledge
- [PARTIAL] partly validated; gaps noted
- [REJECTED] tested, not supported; keep reason
- [SUPERSEDED] replaced by a newer rule

Maintenance
- After verification, move facts into canonical knowledge files and mark the hypothesis [VERIFIED] with a pointer (e.g., SectorInfo.md). Optionally prune verified entries later. Keep [REJECTED] items with a one-line reason for future reference.

[VERIFIED] Sectors & Ownership
- Sector ownership and discovery resolve from sector components.
  Data flow: save sectors + attrs owner/knownto -> table
  Ask: Select any sector on the map and report owner and whether it is discovered for you.
- Contested sectors indicated by sector attribute contested==1.
  Data flow: save sectors + contested flag -> filter
  Ask: Name one sector that shows a contested/war icon in the UI (if any).
- Sector name resolves from sector macro via resources table.
  Data flow: sector.macro + sectors.csv/localization -> name
  Ask: Provide the displayed sector name from the in-game UI.
- Sector race (theme) derives from external sector dataset.
  Data flow: sector.macro + X4_sectors.csv -> race
  Ask: If the UI shows a sector faction theme, confirm which one you see.

[PARTIAL] Resources & Mining
- Resource recharge per ware computed as max/time in resourceareas.
  Data flow: sector.component + resourceareas nodes -> sum(max/time)
  Status: Hatikvah's Choice III overlay shows a dense red field (strong ore) while other colors are sparse.
  Ask: Open the sector resource overlay; list resources visible (Ore/Silicon/Gas) and relative abundance bars you see.
- Sector macro groups resourceareas for aggregation.
  Data flow: area.macro + group by macro -> totals
  Status: Hatikvah's Choice III (same cluster as I) carries the rich ore patch; Hatikvah's Choice I shows low density, confirming per-sector grouping.
  Ask: Compare two sectors in the same cluster; do overlays suggest similar resource areas? Briefly describe.
- Mining yield potential correlates with recharge sums per ware.
  Data flow: recharge table + per-ware totals -> yield proxy
  Status: Current overlays suggest ore is the primary mining target in Hatikvah's Choice III.
  Ask: Based on overlays, which ware appears most promising in one chosen sector?
- Resource cache validity tied to GUID and DLC set.
  Data flow: game.guid/DLC -> cache key -> refresh
  Ask: After starting a new game or adding DLC, did old resource summaries look stale until refreshed? Yes/No.

[PARTIAL] Stations & Construction
- Station module count equals max construction sequence entry index.
  Data flow: station + construction/sequence/entry@index -> max
  Status: UI screenshot lacks construction sequence; need Logical Overview -> Construction once scanner unlocked.
  Ask: Report the module count for one of your stations from Logical Overview -> Construction.
- Station hull approximated by modules*250000.
  Data flow: modules -> multiply -> hull estimate
  Status: Hull observed at 210,000 MJ but module count unknown, so formula still untested.
  Ask: Share the hull value shown for that station (if available).
- Station mass approximated as hull/300.
  Data flow: hull -> divide -> mass estimate
  Status: No mass value visible in provided UI; still pending confirmation.
  Ask: If mass is visible anywhere in UI, report it; otherwise skip.
- Manager, engineer, shiptrader link via workforce/roles.
  Data flow: station workforce + role ids -> join NPCs
  Status: Manager slot shown with 11 subordinates but engineer/shiptrader unreadable (`???`); need comms or scan to reveal roles.
  Ask: List which roles (manager/engineer/shiptrader) are present on that station.

[VERIFIED] Ships & Fleets
- Ship size derives from class suffix (s/m/l/xl).
  Data flow: component@class -> parse size -> factor
  Ask: For one ship, report its size class (S/M/L/XL).
- Ship model maps from ship macro to model in shipdata.
  Data flow: ship.macro + shipdata.csv -> model
  Ask: Report the ship model name shown in the UI.
- Fleet hierarchy via commander/subordinate connections.
  Data flow: connected@commander/subordinates -> leader-follower
  Ask: Does the ship have a commander or subordinates? State relation.
- Pilot assignment from control/post component mapping.
  Data flow: ship control/post -> pilot component id
  Ask: Provide the pilot name/role as shown.
  Note: Parsing validated via tools/selfcheck_ships_fleets.py.
  Verified example: Mercury Vanguard (code XNU-614) — size M (Transporter); pilot "Несса Колина"; commander: none; subordinates: 0.

[VERIFIED] NPCs & Skills (see NPCInfo.md)
- Player-employed NPCs are components class='npc' owner='player'.
  Data flow: save components -> filter -> df.npcs
  Verified via services/x4/npcs.parse_player_npcs fixture test.
  Ask: Select a hired NPC and confirm they are marked as employed by you.
- Skills collected from traits/skill[@value] and pivoted.
  Data flow: npc/traits/skill -> wide table -> scores
  Verified: skills map to integer values (0–5) captured per NPC component.
  Ask: Report visible stars/skills for that NPC.
- NPC roles inferred from employment (pilot/engineer/manager).
  Data flow: joins with ship/station roles -> role label
  Verified: control/post component references resolve to role hints (e.g., `aipilot`).
  Ask: State their current role (pilot/engineer/manager).

[VERIFIED] Economy & Trades (see EconomyInfo.md)
- economylog trade entries capture money, amounts, parties.
  Data flow: economylog/entries[@type='trade']/log -> df.tradelog
  Ask: Provide one trade entry from the logbook with credits, amounts, and counterpart.
- Positive money are sales; negatives are purchases.
  Data flow: sign of money -> buy/sell split
  Ask: Give one sale (positive) and one purchase (negative) example.
- 'Ship construction' entries isolate ship sales volume/revenue.
  Data flow: commodity filter -> aggregate -> plots
  Ask: Share one 'Ship construction' log entry (buyer, ship type, amount, credits).
- Removed objects section preserves names/codes for log joins.
  Data flow: economylog/removed/object -> enrich trades
  Ask: If available, include any related 'removed object' info for that trade.

[PROPOSED] Transfers & Accounts
- Station manager surplus transfers appear in upkeep logs.
  Data flow: log category=upkeep + text -> parse money
  Ask: Provide one manager surplus transfer line from your logbook.
- v4/v5 text variations both parsable for surplus.
  Data flow: dual regex -> normalized fields -> table
  Ask: Note your game version and the exact text of that upkeep line.

[PROPOSED] Policing & Piracy
- Pirate Harassment events parsed to ship, sector, pirate, response.
  Data flow: log title match -> regex fields -> table
  Ask: Share one Pirate Harassment entry (ship, sector, pirate, response if present).
- Police Interdiction events parsed to ship, sector, faction, response.
  Data flow: log title match -> regex fields -> table
  Ask: Share one Police Interdiction entry similarly.

[VERIFIED] Discovery & Spoilers
- Undiscovered sectors/ships/stations controlled by knownto filtering.
  Data flow: knownto attr + spoilers flag -> visibility
  Ask: With 'hide undiscovered' enabled, confirm that an undiscovered sector is hidden on your map.

[VERIFIED] Mapping & Visualization
- Sector map built from cluster/sector CSV geometries and owners.
  Data flow: X4_clusters.csv + X4_sectors.csv + owners -> plot
  Ask: Confirm you see sectors plotted and colored by owner on the map.
- Faction colors applied consistently across charts.
  Data flow: faction id + palette -> visuals
  Verified: Teladi color = #a2b927 (yellowish)
  Ask: Which color represents Teladi in your map legend? Name a Teladi-owned sector with that color.

[PROPOSED] Contested/Warfare Indicators
- Contested table filters discovered sectors only when spoilers.hide.
  Data flow: contested=1 + knownto filter -> table
  Ask: With spoilers hidden, does the contested list exclude undiscovered sectors? Confirm via UI.
- Owner name resolves via faction names, with custom player name.
  Data flow: owner code + factions.names -> display
  Ask: For a sector you own, does the UI show your custom player/faction name?

[PROPOSED] Money & Aggregations
- Earnings per seller/ware computed over time windows.
  Data flow: filter by time -> groupby -> sums/rates
  Ask: From recent logbook entries, which seller/ware earned the most in the last hour?
- Moving average applied to hourly earnings for smoothing.
  Data flow: time series -> MA(avg_smoothing) -> trend
  Ask: If your tool shows a smoothed earnings curve, does it align with hourly data trend?

[PROPOSED] Shipbuilding Insights
- Buyer faction ship construction revenue aggregated and cumulated.
  Data flow: sales filter -> groupby buyer -> area plot
  Ask: Provide one 'Ship construction' sale (buyer faction, credits) from the logbook.
- Ship types and amounts rolled up under buyer for sunburst.
  Data flow: buyer + ship name -> sum money/amount
  Ask: Include ship type and amount for that sale.

[PARTIAL] Player Asset Overview
- Sunburst of player assets: sector -> wing/leader -> followers/ships.
  Data flow: ships + wings + sector -> hierarchy
  Status: Current snapshot shows only unassigned ships (no wings/fleets); need save evidence with a wing/leader hierarchy to validate.
  Ask: From Property Owned, name one wing/leader and a few followers with their sectors.
- Player profile metadata comes straight from `/savegame/player` (`@name`, `@money`).
  Data flow: player node -> asset overview header (name/credits)
  Status: Current save reports `Grisha Egorov`, 319,082 Cr.
  Ask: Confirm your Property Owned header shows the same player name and credit total.

[PROPOSED] Data Hygiene & Safety
- Log numeric fields coerced with NA guards for robustness.
  Data flow: text -> numeric with suppressed warnings
  Ask: Share any log entry where a number was missing or malformed (if seen).
- Name fallbacks cascade: custom -> model -> macro.
  Data flow: name is NA -> fallback chain
  Ask: Provide one case where a custom name was missing and the UI fell back to model or macro (if applicable).

Follow-ups (for our tooling)
- Verify resourceareas extraction matches game DAT/CAT definitions.
- Confirm commander/subordinate connection semantics across DLCs.
- Align economylog schema across versions (v4/v5 text).
- Validate module-to-hull and hull-to-mass factors with resources.
- Map sector macro to localization for names in our pipeline.
