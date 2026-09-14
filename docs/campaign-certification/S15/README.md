# S15 — Tactical missions and campaign results

Status: **complete**, together with S12–S14, on runtime `4c4129abeeec4e9e2987f121e875f5efc54b4c7f`. Execution stops after S15; S16 remains planned.

Air command connects owned aircraft, saved squadrons, financially funded geographic bases, routine upkeep and compatible stores to **Support army** and **Strike target**. Both use the existing campaign contact and casualty resolver. Aircraft alone do not capture territory. New aircraft, transfers, service and refit reservations retain their separate ownership and readiness records.

The disclosed authored France journey ran from **18 Feb 1992** to **28 Mar 1992** through **39 days**, **18 ordinary reviewed commands** and **11 named native checkpoints**. It paid for and received 2 supplier aircraft, assigned purchased aircraft, completed home and consenting-host bases, completed Support and Protection improvements, displayed foreign transit, bought compatible stores under the support cap, cancelled one mission and flew both tactical mission kinds before funded recovery. The browser observed **0 whole-aircraft losses**; this is the recorded result, not a claim that a positive loss occurred.

| Qualification | Passed | Ignored / skipped |
|---|---:|---:|
| Windows web | 393 | 18 |
| Windows sim | 861 | 26 |
| Windows integration | 83 | 2 |
| Windows node | 1587 | 1 |
| Linux web | 393 | 18 |
| Linux sim | 861 | 26 |
| Linux integration | 83 | 2 |
| Linux node | 1587 | 1 |

The fixture export passed separately. Existing ignored native tests and the skipped Node case remain listed in the [manifest](manifest.json); they are not counted as passes. The browser completed **15 full-world comparisons** and **4 historical-envelope comparisons**. No world fields are ignored; only the separately validated save timestamp may differ between historical envelopes. These comparisons cover named native checkpoints and final Save/Load/Continue, not every intermediate day.

Direct native tests separately cover positive whole-aircraft loss, finite stores shared by simultaneous missions, prepared-order save/resume and frozen-launch replay prevention. The shared transport tests cover lost-response retries and the four Air command kinds through protected session/client/sequence receipts. The authored browser journey does not simulate a lost network response.

The final browser driver is `1653638fc050b4b4796875ffdeb7fbff9e7b6d7e`. Its only change from the runtime candidate is `tools/ui/ci-flight-operations.cjs`; recorded diffs for simulation, web runtime and Cargo files are empty. Windows/Linux native and Node qualification remains bound to the unchanged runtime candidate. The final driver is qualified by the recorded final browser journey.

Desktop **1440px** and mobile **390px** captures were inspected: Codex inspected these final browser captures directly. The funded base controls, exact squadron assignments, dated mission outcomes, launch geography and compatible-store review are readable at the inspected widths. Map focus has settled; the scrollable map card exposes its return controls. No 320px check or independent human playtest is claimed.

The [evidence inventory](evidence/inventory.json) retains 422 complete selected input files, with verified byte reconstruction. Large files may be gzip-compressed and split; concatenate numbered pieces before decompression. Full saves, canonical worlds, proof records and selected original failed attempts are retained without filtering native fields. Executables are identified by source and SHA-256 rather than repackaged. Prior session evidence is preserved.

[Open the separate review campaign](http://127.0.0.1:7857). Its copied final save and native canonical audit match the browser's Continue result. Preservation evidence covers the existing eight named files and two worktree HEADs.

The manifest lists every authored precondition, including funded starting fixtures, certified starting revisions, the France–Italy conflict and UK host consent. This is not historical opening equipment, an unassisted campaign or complete campaign certification. Whole-aircraft loss and upkeep rates, base capacity and range are explicit game assumptions. No 320px visual result, independent human playtest, new performance qualification or remote push is claimed. Existing historical/content, long-campaign and release requirements remain open. **G3 and CP1 remain unearned. Next: S16 — fighters and Defend skies**, requiring a new instruction.
