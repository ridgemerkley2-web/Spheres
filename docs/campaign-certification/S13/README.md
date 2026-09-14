# S13 — Geographic airbases, access and range

Status: **complete** on runtime `4c4129abeeec4e9e2987f121e875f5efc54b4c7f`, qualified with S12–S15. See the [combined acceptance manifest](../S15/manifest.json).

Air command builds geographic airbases using the existing national financial construction envelope. Foundation capacity remains zero until the complete contract has been funded and finished. Existing construction competes for the same daily pool. The game charges no construction materials or second treasury account.

| Improvement | Finished effect | Modeled price and work |
|---|---|---|
| Foundation | 12 aircraft spaces | $24m; 12 fully funded days |
| Capacity, levels 2–5 | 12 extra spaces each level | $18m × target level; 9 × target level days |
| Support, level 1 | Post-flight service falls from 2 to 1 funded day | $12m; 6 fully funded days |
| Protection, levels 1–5 | 8% less modeled sortie loss per level, up to 40% | $15m × target level; 8 × target level days |

Transfers take visible time and reserve destination space, including other countries' inbound squadrons. Access is checked at review, throughout funded construction, on arrival and at launch. A country can finance a base in a consenting host's territory using existing exact-host basing permission; this transfers no land or host funds. Lost access or physical control pauses eligibility. An interrupted transfer can be redirected without creating aircraft.

The map shows the base, occupancy, combat radius and reviewed target or transit route. Light-attack radius is 700 km and tactical-strike radius is 1,400 km, multiplied by 1.35 for installed extended endurance. Return flight is included. Native distance arithmetic is deterministic across platforms. These are explicit game assumptions, not a historical airfield census or performance catalogue.

See the [combined flight qualification](../S15/README.md) for acceptance evidence. G3 remains open.
