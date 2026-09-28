# S20 complete — map, province and room navigation

Closed 28 September 2026 UTC. Qualified runtime:
`534c8bf190a0e39f3b1e6c0b025201b37e8279ff`.
[Exact manifest, checks and seven retained attempts](manifest.json).

The combined journey now runs from Find → city → province → construction review,
and from an actually completed Arms Plant → Industry → company procurement →
Air Command and its real mission reports. It uses the unchanged, recorded France
campaign from [S19's ordinary flight qualification](../S19/integration/CLOSEOUT.md),
dated 13 May 1999. No equipment, funds, facility or outcome is injected.

| Acceptance | Verified result |
|---|---|
| Current city/province ownership, construction, output and problems | Current owner/province labels and exact native population/economic records, activity counts and completed-facility records compared in the browser. Unmapped cities cannot open an invented province. |
| Keyboard and touch, focus, return, loading and error recovery at 390px | 1440px keyboard and 390px touch routes pass. A delayed genuine reading retains focus, an aborted reading exposes Retry, and late province/session responses cannot replace newer context. Replacing the campaign session clears the selected city. |
| Honest geography and clear presentation | City massing remains explicitly representative; no street-accuracy claim. Existing illustrated room framing is retained. Desktop/narrow province, construction, company and Air Command views were visually inspected; vertical scrolling works without horizontal overflow. |

## Repairs delivered

- Province and city entry, refreshed-control focus and close/return behavior are
  consistent. A delayed Find-dialog close no longer steals destination focus.
- Construction remembers its province/control/campaign identity so a recreated
  “Build here” button receives focus on return. Missing or changed contexts safely
  use the Construction dock instead.
- Failed province reads have a guarded Retry, busy state and dated readings.
  Stale responses and selected cities do not leak into a replacement campaign session.
- A completed Arms Plant's “Manage equipment” action opens company procurement.
- Queued company work says “Waiting for plant” while its stock target is unmet.
  Dated flight reports use readable province and ammunition names while preserving
  their original saved settlements.

## Qualification

**1,717 UI tests passed**, with no failures or skips. **420 native web tests passed**,
with no failures and **21 existing ignored tests**. Release compilation passed.
The final native browser run passed with **13 screenshots, 15 native observations,
30 focus observations and zero page errors**.

Both visibly created before/after saves contain 63,066,286 bytes. The complete
world, history, log and journey compare equal after removing only the separately
validated storage timestamp. The route sends no `/api/command` or `/api/advance`.
Its POST audit permits saves/load plus exactly three verified read-only preview
endpoints: construction, ministry budget and equipment design. Those preview
handlers borrow the world immutably; quotes do not enact orders.

The first six attempts remain failed records. They exposed two real focus defects
and four harness issues: detached-node observation, nested preview cancellation,
negative-zero loss during JSON cloning, and omitted read-only preview endpoints.
Corrections retain strict native comparisons and the complete save comparison;
no failed result was relabeled. The initial aggregate's two outdated cabinet
fixtures are likewise retained alongside the corrected passing suite.

To reproduce, compile the pinned runtime and run
`tools/ui/ci-supplier-input-recovery.cjs` with `SPHERES_NAVIGATION_CLOSEOUT=1`,
`SPHERES_EXPECTED_REVISION` set to the full revision above,
`SPHERES_BROWSER_CHANNEL=msedge`, and `SPHERES_SUPPLIER_CHECKPOINT` set to the
archived S19 qualified save. Its uncompressed SHA-256 is
`1af599ec1568a5b2148354de25cb800780d8ce69d352664f0a944242fac5ca8c`; provide it as
`SPHERES_SUPPLIER_CHECKPOINT_SHA256`. Choose a disposable `SPHERES_SUPPLY_OUTPUT`.
Captured drivers identify the exact behavior of each historical attempt.

S20 is complete and earns the remaining requirement for [G4](../G4/README.md).
S22 performance qualification is next. Historical coverage, worldwide cartoons,
long campaigns, independent human testing and CP1 retain their own requirements.
This is Windows qualification of the supported recorded-campaign journey, not a
new all-platform, performance or full-period certificate.
