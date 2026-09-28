# S19 — supplier input recovery controls

27 September 2026. Runtime **`c8a59bfd48cb49b7596332ab76f42aa8b2f03712`**.
The recovery UI is verified. **S19 procurement, flight and CP1 remain open.**

## Problem and change

The recorded France campaign reached paid tank certification and tooling, then
stalled with no advanced components. The manufacturer card described the shortage
without showing its quantities or linking to recovery. The backend supported
advanced-component trade, but the Exchange buy selector offered only intermediates
and capital goods; navigation also discarded the component target.

The company card now reads the simulation's exact scheduled work packet and shows
needed, available and missing inputs in a compact section. Only the matching
equipment product receives that reading; existing grandfathered work retains its
original terms. Missing components expose navigation to Industry, the Advanced
Industry construction review and component trade. Navigation clears unrelated old
quotes and carries the native shortfall into the quantity control. It places no order.
The Exchange now names and quotes advanced components correctly.

These are presentation/navigation changes, not new production rules. Company
money, warehouse inputs, paid work and government holdings retain their owners.

## Recorded-campaign check

The [browser driver](../../../../tools/ui/ci-supplier-input-recovery.cjs) starts a
disposable native server and loads the unchanged, hashed 1 July 1993 France save
through Saved campaigns. It uses visible controls for every action and GETs for
native observations. It verifies actual served UI bytes against the compiled checkout.

- The next tank requires **0.4441 advanced components**; warehouse stock is zero.
- Industry shows no completed Advanced Industry site.
- The construction shortcut selects Advanced Industry and a real province. Native
  review says it can start: **$240m**, **660 days**, with operating requirements
  separately disclosed. No construction order was placed in this navigation check.
- The import shortcut preserves the exact native component shortage. Economic
  Competition is enabled through its existing explicit control in this disposable
  copy; this is the only simulation command besides loading the campaign.
- The actual quote response has **no offers**. No goods or equipment are granted,
  bought or delivered; the day and zero component stock remain unchanged.
- Desktop and 390px screenshots were inspected. The compact input rows remain
  readable without horizontal overflow. Earlier exploratory driver attempts and
  the superseded bulky layout remain local; they are not final pass evidence.

## Validation

- **355 UI tests passed** across equipment, Exchange, industry, cash flow, company
  network and guidance checks.
- **17 native company tests passed**; one existing long supplier-export test remains
  ignored. No new full-workspace pass is claimed.
- Locked release build passed with three existing `tiny_http` warnings.
- Recorded-campaign recovery browser route passed on that exact runtime.
- Existing complete first-hour browser route passed, including ordinary actions,
  stale-response handling, save/load/Continue, new-campaign isolation and narrow UI.

[Manifest, raw results, logs, source hashes and screenshots](input-recovery-evidence/manifest.json).
The first-hour result's historical hardcoded `driver_committed: false` is retained
unaltered; the manifest independently verifies both captured drivers against the
runtime commit, normalizing only CRLF to LF for Git comparison.

## Next campaign work

Resume the original checkpoint in another disposable campaign. Build and fund the
reviewed plant through ordinary controls, renew annual budgets and inspect its real
workers, power, raw inputs and operating spending. Alternatively, obtain an actual
arrived component purchase if a supplier develops surplus. Then obtain company
stock, buy it, verify settled payment/delivery and save/resume. Squadron readiness
and a supported flown result remain separate requirements. A working shortcut or
an affordable construction preview does not satisfy those milestones.
