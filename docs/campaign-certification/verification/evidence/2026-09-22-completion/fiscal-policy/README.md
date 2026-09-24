# Fiscal direction and budget ownership

This bounded review found fiscal accounting and policy-direction defects in
checkpoint `90fce947`. These findings do not establish a cause of A1's remaining
coup-concentration failure, and the repair changes no political acceptance bar.

`SPEC.md:55` defines fiscal consolidation as raising taxes and trimming spending.
The existing `the_fiscal_ai_arrests_a_debt_spiral` test describes half-percent
military and investment cuts. The Army floor's comment also explicitly disallows
implicit spending increases. No separate mandate to raise every low appropriation
to 1% military or 2% investment was found.

The old high-debt floor nevertheless raises opening military appropriations in
Guyana (0.9% to 1%), Jamaica and Sao Tome (0.8% to 1%), and Samoa (zero to 1%).
It raises Belgium's public investment from 1.4% to 2%. The same branch can lower
an already legal 60% tax rate to its 55% soft limit. Conversely, low-debt recovery
can cut investment that already exceeds its historical reference, and its tax
decrement can overshoot the 30% stopping point. These are directional clamp
errors; the ordinary in-range rates and stopping points are retained.

The legacy fiscal loop also ignores an explicit annual plan when its government
is no longer the current player and no department programme is enrolled. It
changes aggregate spending while retaining incompatible ministry rows. The new
guard treats an explicit plan and open fiscal stocks as owners independently,
including the older plan-only save shape. Player, programme and fiscal-recovery
ownership remains covered by an unchanged-behaviour control.

The paid Fiscal Consolidation card has the same below-floor spending inversion.
Its bounded repair retains its existing 30% investment and 20% military cuts,
with floors limiting cuts rather than inventing spending. Actual annual and
programme plans receive the proportional changes through ordinary dispatch;
department shares, prior authority, prepaid funds, receipts and current cash are
preserved. The outer card still charges 34 political capital once. Stock-only
aggregate budgets keep their existing owner. Quote validation prevents a refused
programme command from partially applying taxes, stability loss or a charge.
When tax and both spending lines have no room to change, the legacy card is
unavailable. This prevents repeated 34-point charges and stability losses for
financially empty votes. Each tax-only, investment-only and defence-only case
remains available, including plans with unchanged department allocations.

Month ordering was checked: economy settles before government, then politics;
modern economic/fiscal AI runs later. There is no missing junta-specific access
guard in the legacy fiscal loop or Army funding request. Paid tax and budget
commands exist, but inventing a junta priority or free reallocation would be a
new policy rather than an accounting repair.

`original21-witness.rs` exercises the public entry points against the exact
retained iteration21 library without rebuilding Cargo: **1 passed, 6 failed**.
The failures reproduce actual spending increases, reversed restoration and plan
inconsistency; the owner-control test passes. `review.json` records the library,
source and test hashes. The log and wrapper are archived byte-for-byte. These
are intentional before-fix failure witnesses, not a passing certification run.
`original21-card-witness.rs` separately reproduces the first three card defects:
**0 passed, 3 failed** against the same retained library. The later stock-only
and no-op/single-lever controls are part of the six applied card tests, not that
earlier three-test witness. Their native result is still pending here.

Seven policy tests and six card tests were applied after independent source
review. `application.json` pins the applied files and the exact external test
fragments. Source edits were then frozen for the parent's combined native run;
the reviewer ran no Cargo build. `git diff --check` passed for both changed files.

The corrected native tests and default-output attribution are pending at this
receipt's creation. The earlier 1,866-pass restored21 workspace run describes
the baseline only. A1 remains outstanding; no final campaign certification or
hosted CI result is claimed here.

## Measured fiscal epoch

The iteration23 sim-library run passed 1,000 tests and failed only four exact
timeline fingerprints (25 ignored, one isolated performance test filtered). The
unaltered failure log is archived here. A separate 240-month probe measured all
seven seeds; every startup hash is unchanged. `default-pin-migration.json` records
the ten active end-hash replacements, the old and new outputs, and the bounded
policy attribution. The earlier original21 witnesses remain unchanged.

The seven policy and six card regression tests passed in that run. The repaired
closed-book spending/tax directions and refusal of financially empty cards can
change later economics, card availability and random draws. They are intentional
policy corrections, not serialization-only updates. No political calibration bar
or on/off comparison was relaxed. Post-pin recompilation and broader final
verification remain pending; this receipt does not claim A1 passes.
