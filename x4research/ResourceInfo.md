Resource Knowledge

Resource Areas
- Save path: `/savegame/universe//component[@class='sector']/resourceareas/resourcearea`.
- Each `resourcearea` provides `macro` (local resource region id), `ware` (resource type such as `ore`, `silicon`, `ice`, `methane`), and the parameters `max` and `time`.
- Recharge rate per ware derives from `max / time`; summing recharge across all resourceareas within a sector gives the sector-wide availability metric.
- The `macro` values tie back to sector macros, allowing aggregation by sector or cluster without touching UI overlays.

Aggregation Flow
- Iterate sectors, extract all descendant resourceareas.
- Coalesce entries by `macro` and `ware`, summing recharge to get totals per resource per sector.
- Persist computed tables (e.g., in cache) for tooling to reuse without rescanning the save.
