# Construction payment retention

Runtime: `24d05428`.

Completed construction used to remove its active funding row. Guidance then
lost the evidence for **Construction work paid**, even though completion and
output still appeared. The simulation now retains each nation's latest actual
project-payment receipt separately from the active queue.

The receipt is written only after successful positive spending. It stores the
project, site, kind, date and paid amount; it does not spend any additional money.
Zero-spend days cannot erase it. Completion and cancellation keep it, and a
province transfer cannot award the former owner's payment to the new owner.
Storage is bounded to one project-payment receipt per nation.

The native guidance reader exposes only the selected nation's receipt. The
advisor combines it with active project/mine payments, applying the same date
and amount validation. The extended browser route now requires the paid,
completed and producing milestones together before checking save/resume.

## Save compatibility and limits

The new industry field defaults to an empty map and is omitted when empty.
Older saves load without fabricated receipts. Payments already discarded before
this update cannot be recovered from a completion headline. New receipts are
captured prospectively; no migration rewrites historical spending or ownership.
This change retains project payments, not a complete transaction ledger or a
new historical mine-payment archive.

## Regression coverage

- Actual funding through completion, with exact existing cash/inventory checks.
- Positive receipts survive later zero-budget days and cancellation.
- Industry serialization preserves the receipt; older field-less data stays empty.
- Province ownership changes do not transfer the payer's achievement.
- Guidance accepts a retained dated payment, rejects future/invalid records and
  handles older responses without the new optional field.
- The browser driver requires all three construction milestones to persist in
  its existing save/load, Continue, narrow layout and fresh-campaign checks.

## Verified results — 27 September 2026

The fresh France browser run passed on exact runtime
`24d054289929d3afbe50497ee64547bb725923bf`. After 169 additional ordinary daily
advances, all three construction milestones were **Done**, dated 3 July 1990.
The paid-work milestone remained intact after named save/load and reload/Continue.
The new-campaign reset and 390-pixel layout checks also passed. Desktop and narrow
screenshots were visually inspected; original campaigns and preview servers were
not used by this isolated run.

- 1,888 native workspace tests passed, with 89 existing ignored tests unchanged.
- The resource timing test passed separately: 0.0589 ms/month against its unchanged
  0.15 hard limit. The 0.05 advisory target remains unmet.
- 1,703 UI tests passed. The focused run includes 23 industry tests and 25 guidance
  outcome tests; these overlap the broader suites and are not extra unique tests.
- The locked release build passed, with only existing vendored warnings.

[Manifest and source identities](payment-retention-evidence/manifest.json),
[browser result](payment-retention-evidence/result.json), and
[desktop screenshot](payment-retention-evidence/route-later-construction-desktop.png)
retain exact commands, raw logs and artifact hashes. This closes the construction
payment-retention follow-up, not S19/G4/CP1. Actual company purchase/payment/delivery,
squadron readiness and flown-result qualification remain next.
