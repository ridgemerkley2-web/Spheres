# S19 — real company stock, paid purchase and delivery

27 September 2026. Runtime **`c8a59bfd48cb49b7596332ab76f42aa8b2f03712`**.
The later procurement route now passes. **S19 remains open for squadron readiness,
a supported flown mission and their retained outcomes. No CP1 claim.**

## What the campaign actually did

The unchanged [1 July 1993 France checkpoint](PROCUREMENT_SUPPLY.md) already held
a certified tank design, paid development and company tooling. It had no advanced
components and no finished tank stock. All subsequent mutations used visible
controls in disposable native servers: reviewed construction orders, daily time
steps, annual Cabinet budget renewals, equipment review and named save/load.
GET requests and browser state reads observed results; no inventory, money,
production work or receipts were injected.

1. Ordered the reviewed **$240m / 660-day Advanced Industry plant** in
   Auvergne-Rhône-Alpes. It completed and began producing components; its first
   observed completion was 25 April 1995. Sampling bounds that observation rather
   than establishing an exact completion date.
2. The first run still failed its 900-day stock bound. The new plant shared the
   province's **0.457839 power/day** local grid with the manufacturer. Its output
   was approximately **0.114460 components/day**, while the supplier's actual
   work advanced only by a negligible floating-point residual. The label
   “Building company stock” alone was insufficient evidence of useful production.
3. Resumed the untouched **22 October 1995** named checkpoint in another isolated
   server. Reviewed and funded a **$120m / 420-day Power Grid** in the same province.
   It raised the local ceiling to **5.457839 power/day**. Once complete, the plant
   produced approximately **0.2 components/day**, and the company finished stock.
4. Observed **one available tank on 1 February 1997**. Reviewed and bought exactly
   one `France-design-1` from France Defence Industries at approximately **$5.405m**.
   Native receipt **285** records purchase/payment on **1 February** and delivery
   on **8 February 1997**, with the result observed on **9 February**.
5. Guidance shows all five procurement milestones done. Manufacturer and
   certification are current native tallies without invented dates; purchase,
   payment and delivery display the receipt dates. Construction payment,
   completion and operating-output milestones remain done.
6. A separate browser proof loads the actual delivered checkpoint, compares the
   accepted advisor model against an independent native guidance reading, checks
   rendered milestones/dates, saves and loads a named campaign, then reloads the
   page and uses Continue. The same receipt and milestones survive all three
   readings. Desktop and 390px screenshots were inspected; no horizontal panel
   overflow was found.

No production rules or runtime code changed in this checkpoint. The new work is
reusable browser qualification, authentic saved campaigns and the workboard update.

## Evidence and reproduction

[Manifest and raw evidence](procurement-delivery-evidence/manifest.json) retain
executed driver copies, input hashes, binary/served-asset hashes, all browser
actions and POST payloads, native observations, logs and screenshots. Large raw
JSON files are losslessly gzip-compressed; the manifest records both hashes.
The two saved campaigns retain their original bytes when decompressed.

The chain is the original 1993 save → recorded 1995 component/grid checkpoint →
recorded 1997 delivered checkpoint → independent save/load/Continue proof.
Each input hash is checked against its preceding saved output. All four runs used
the same binary. Their captured helpers differ where the test driver evolved;
the final tool files are not retroactively presented as the older executed drivers.

The failed stock-bound run is retained as `component-shortfall`, not relabeled a
pass. `superseded-continue-timing` retains the first proof attempt, which passed
named load but opened guidance while Continue was still adopting the campaign.
The final proof waits for the visible app and the existing session/command busy
flags before opening guidance. It passes without a runtime change. An additional
read-only site diagnostic from the first run is explicitly identified separately.

Use a locked release binary built from the stated runtime or a checkout with
identical runtime files. Set `SPHERES_EXPECTED_REVISION` to the full revision and,
for this Windows setup, `SPHERES_BROWSER_CHANNEL=msedge`. The driver starts and
cleans up only its own server/browser; existing previews and source saves remain.

- `SPHERES_SUPPLIER_PRODUCTION=1` runs the plant route from the original checkpoint.
  Its stock bound fails without the needed grid, as recorded; this is diagnostic.
- `SPHERES_SUPPLIER_GRID_RECOVERY=1` with the recorded component/grid checkpoint
  builds the grid and completes actual stock purchase/delivery and load.
- `SPHERES_SUPPLIER_VERIFY_DELIVERY=1` with the delivered checkpoint verifies
  native guidance, rendered dates, named save/load, Continue and narrow layout.

For either custom checkpoint, set `SPHERES_SUPPLIER_CHECKPOINT` to its JSON or gzip
path and `SPHERES_SUPPLIER_CHECKPOINT_SHA256` to the **uncompressed** SHA-256 in the
manifest. Select only one mode. `SPHERES_SUPPLY_OUTPUT` selects a fresh evidence
directory. Run `node tools/ui/ci-supplier-input-recovery.cjs`. Playwright must be
available through the normal development dependencies or the recorded runtime's
`NODE_PATH`. No recorded result is used to bypass the native simulation.

Validation for this checkpoint: **81 guidance tests passed**, browser grid and
delivery route passed, independent delivery/save/load/Continue/narrow proof passed,
and **8 planning tests plus workboard validation passed**. No new full-workspace
or native test-suite pass is claimed; runtime code is unchanged.

## Next

Resume the delivered campaign for the flight route: obtain actual aircraft,
form and support a squadron, prove readiness, and fly a supported mission under
an eligible conflict. Retain this procurement result and the construction result
through those actions and save/resume. Existing first-hour new-campaign isolation
evidence is retained; this procurement proof does not independently test fresh
campaign isolation. S20 still follows S19 closure.
