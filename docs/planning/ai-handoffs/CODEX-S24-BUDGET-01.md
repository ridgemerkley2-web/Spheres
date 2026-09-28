# CODEX-S24-BUDGET-01 — explain debt-rate components

Owner: Codex. State: **complete** (28 September 2026). Parent: S24, bounded repair.

Claude's successor fixtures exposed a budget display that counted the real-rate
floor as sovereign risk. Repair `52e1c2ab` separates raw real rate, floor adjustment
and actual sovereign spread, using the existing simulator calculation. Both budget
explanations show those components. Economic formulas and charged amounts are unchanged.

[Evidence](../../campaign-certification/S24/repairs/budget-rate-explanation/README.md)
at `15b35901` retains the failing original regression, 25 passing UI tests, two
passing native tests, a successful release build and actual Brazil/France browser
journeys at 1440/390 pixels. No S24 or CP1 qualification follows from this repair.
