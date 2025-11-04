Sector Knowledge

Scope
- Single source for sector ownership and naming.
- Keep this file concise; cross-cutting terms live in `BasicInfo.md`.

Data Flow
- save sectors → owner/knownto/contested from attributes
- sector.macro → resource dataset `properties/identification@name` → localization token → display name

Core Fields (from save)
- code: in‑game sector code (e.g., LRG-080)
- owner: owning faction id (e.g., argon, teladi, player)
- knownto: discovery status (e.g., player)
- contested: "1" if contested, else empty/absent
- macro: sector macro id (e.g., cluster_18_sector001_macro)

Pointers
- Save structure path for sectors: see `BasicInfo.md`: Save Sectors.
- Localization/name resolution: see `BasicInfo.md`: Sector Name.

The Reach (PDK-873)
- Owner: Argon Federation (player outpost present).
- Stations parsed from `quicksave.xml.gz` (timestamp aligned with current research):
  code | name | owner | x | y | z
  --- | --- | --- | --- | --- | ---
  SDU-179 | ARG Advanced Electronics Factory I | argon | -8174.36 | 4329.99 | -38616.41
  QSD-954 | Solar Start | player | 45628.42 | 0.00 | -1290.15
  PVW-854 | ARG Defence Platform | argon | -26925.06 | -4512.18 | 39570.09
  OBN-199 | ARG Hull Parts Factory II | argon | 5616.04 | 4765.55 | -25181.20
  IWQ-735 | ARG Quantum Tube Fabrication I | argon | 19665.88 | 88.13 | 34671.01
  OTJ-238 | ARG Plasma Conductor Factory I | argon | -26605.30 | 3318.09 | -26546.30
  RYR-051 | ARG Refined Metals Production I | argon | -12302.06 | -2260.29 | -25177.36
  OOY-783 | ARG Advanced Composite Factory I | argon | 440009.62 | -6194.47 | 25818.71
  LVQ-109 | ARG Missile Component Factory I | argon | 406571.80 | -2346.83 | 22269.67
  MRA-977 | ARG Food Ration Production I | argon | 153245.42 | -12350.87 | -65109.09
  HLU-251 | ARG Shield Component Plant I | argon | 138403.41 | 5334.64 | -132775.77
  TQT-803 | ARG Advanced Composite Factory II | argon | 149441.37 | 7647.62 | -149665.15
  NHN-853 | ARG Claytronic Production Complex I | argon | 111107.14 | 1586.65 | -149320.03
  KYC-722 | ARG Scripted Station KYC-722 | argon | 166981.17 | -453.28 | -163068.28
  UCK-345 | ARG Medical Supply Production I | argon | 260983.67 | -8799.34 | -141795.25
  IQQ-550 | ARG Hull Parts Factory I | argon | 130398.04 | -36558.70 | -200442.44
