Economy & Trades Knowledge

Scope
- Canonical reference for economic logs already captured in the knowledge base.
- Focuses on the economy log collected under `/savegame/economylog`; future sections should extend this file.

Economy Log (Trades)
- Save path: `/savegame/economylog/entries[@type='trade']/log`.
- Attributes:
  - `time`: game-time stamp (seconds).
  - `ware`: ware id (`siliconwafers`, `medicalsupplies`, ...).
  - `buyer` / `seller`: component ids (ships or stations). If the object was removed, resolve via `/savegame/economylog/removed/object`.
  - `price`: per-unit price in centi-credits (UI value ÷ 100).
  - `v`: quantity traded (units).
  - `b`, `bmax`: buyer cargo fill and capacity after the trade.
  - `s`, `smax`: seller cargo fill and capacity (present when the seller keeps stock).
- Totals:
  - Purchase total (UI debit) = `(price / 100) * v`. Recorded as a negative number in the Transaction Log (e.g., Hermes Vanguard IKM-447 bought `394 × Silicon Wafers` for `255.15 Cr` each → `−108,208 Cr` including trade fees).
  - Sale proceeds surface as “Profit from Trade Orders” grouped records with positive values (e.g., `+12,180 Cr`). The detailed line items sit in the matching `Trade Order` entries for the ship.
- Removed objects:
  - `/savegame/economylog/removed/object` stores `{id, name, code, owner}` for ships/stations no longer present, allowing historical joins (see `data/X4SaveGameAnalysis.R` trade log merge).
- Player observation:
  - Transaction Log UI shows per-trade debit lines (“Trade Order” entries) and periodic aggregated profits. Buying silicon wafers produced `−108,208 Cr`; subsequent profit entries reflect resale batches.

Notes
- Money sign convention aligns with the Transaction Log: negative = purchase/debit; positive = sale/profit.
- Prices and totals include trade fees; the centi-credit representation in saves converts cleanly by dividing by 100.
