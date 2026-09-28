# CLAUDE-S24-SUCCESSORS-01 — successor-country fixture harness

Owner: Claude; reviewer/integrator: Codex. State: **ready_for_review** (28 September 2026). Parent: S24, **preparation only**; Codex owns S24 closure.
Branch: `claude/s24-successors-01`. Base: `e41aa18d`; claim `328db128`; merged with integration `75626a13` at `26d5d63a`, the
source of the committed run. Touched paths: only the owned paths below. S24 is not advanced beyond preparation; see the Result.
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md).

## Build

Inventory the 23 expected successor identities from current data, explicitly flagging
any count or mapping discrepancy rather than altering the expectation. Create a
standalone harness and documented fixture recipes for activation, save/load and
country-specific UI checks. Reuse existing fixture/test entry points where possible;
if an unavailable native hook blocks a case, deliver its focused integration proposal
and an explicit unrun result instead of editing command dispatch or forging a pass.

Run at least three representative cases where supported, including USSR→Russia,
with selection identity, government, budget and guidance inspection after reload.
Use disposable test campaigns in an isolated directory and retain exact build/source
identity and original fixture inputs. All seeded/forced activations must be labeled
as fixtures; they do not prove organic succession or the S25 long campaign route.

## Owned paths

New `tools/ui/successor-fixtures/`, `tools/ui/check_successor_fixtures.cjs`,
`docs/campaign-certification/S24/preparation/successor-fixtures/`; this handoff.
Read current native tests, country data and browser drivers. No production code,
existing driver, user save, shared schema, workflow or default CI edits.

## Acceptance

Deliver the 23-row inventory, runnable isolated harness, tests for missing/duplicate
identities and mislabeled results, and raw results/screenshots for executed cases.
Report passed, failed, blocked and unrun independently. Document required setup and
cleanup without touching user campaigns. Full S24 still needs all canonical
dependencies and exact-build startup/recovery qualification by Codex.

## Result

Submitted `ready_for_review` on 28 September 2026. Commits: `0b64a3fa` (harness, inventory, proposal), `e18d3b5d`
(merge), `ea0f3249` (screenshot timing), `26d5d63a` (merge of integration `75626a13`), then the evidence and record
commit at the branch head. Touched paths (nothing else):

- `tools/ui/successor-fixtures/` (new): `inventory.cjs`, `run.cjs`, `lib.cjs`, `README.md` (recipes, setup, cleanup);
- `tools/ui/check_successor_fixtures.cjs` (new, 13 `node --test` tests; also found by `tools/ui/run-unit.cjs`);
- `docs/campaign-certification/S24/preparation/successor-fixtures/` (new): `inventory.json`, `integration-proposal.md`,
  `README.md`, `evidence/` (`result.json`, 12 screenshots, logs, exporter provenance; no saves or binaries);
- this record.

No production code, command dispatch, existing driver, shared schema, save, workflow, CI, `campaign-pathway.json` or
task queue changed.

**Inventory (23 rows).** Sources agree on 23: the S24 pathway statement, census `successor_nations`, C01 rows with
`start_1990: false` and the roster. 21 have a native activation path: 15 Soviet (`dissolve_ussr`: Russia, Ukraine,
Belarus, Moldova, Georgia, Armenia, Azerbaijan, Kazakhstan, Uzbekistan, Turkmenistan, Kyrgyzstan, Tajikistan,
Lithuania, Latvia, Estonia) and 6 Yugoslav (`dissolve_yugoslavia`: Serbia, Croatia, Slovenia, Bosnia, Macedonia,
Montenegro), each with a `SUCCESSOR_PARENTS` entry, a continuation family and districts listed under its 1990 parent.
Russia is the only certified case. Discrepancies, flagged and not absorbed: **D-activation** — Namibia and East Timor
have no dissolution, parent entry or continuation family; **D-map** — their `NA-*`/`TL-*` districts (13 each) have no
1990 owner. Observations: `sources_json` claims all 23 are seated (O1); census `existing_lifecycle_records` is also 23
but is not a successor count (O2); both gaps are successors by date, not breakups (O3). The expectation stays 23.
Proposal P1 asks for a ruling; nothing was forged.

**Harness.** `run.cjs` starts its own `spheres-web` with `cwd` = a new isolated directory (the server's only save root),
a free port (never 7777) and `--no-open`, refuses to run unless `/api/build.save_directory` is that directory, the
binary's served revision is `--expected-revision` and the harness/runtime tree is committed. Recipes: **G0** selection
guard; **N1** the existing S21 exporter's USSR dissolution, reused unchanged; **H1** a byte-exact two-field staging
(parent `stability 0.0`, `separatism 1.0`) of a fresh `/api/new` save followed by one real day; **B1** the same through
ordinary browser controls. Every activation carries a fixture label and the disclaimer; the validator rejects organic
claims, wrong kinds, forged no-hook passes, missing or duplicate results, bad summaries and changed screenshots.

**Commands and results** (28 September 2026; `CARGO_TARGET_DIR=D:/spheres-target-s24`, worktree `D:/Spheres-s24-successors`):

- `cargo build --locked --release -p spheres-web` at `26d5d63a`: exit 0; frozen binary SHA-256
  `dd4eeb76844a40685f9fccea8193a18108f8ef5c81dac345f7b27deb9cf10e07` (358,615,277 bytes, served `26d5d63ae97b`).
- `SPHERES_S21_FIXTURE_DIR=<new dir> cargo test --locked --release -p spheres-web campaign_journey::tests::s21_export_review_fixtures -- --ignored --exact --nocapture`,
  twice: 1 passed each; `succession.json` projection `810292a4…` both times and equal to the `e18d3b5d` run.
- `NODE_PATH=C:/Users/ridge/spheres-war-overhaul/tools/ui/node_modules SPHERES_BROWSER_CHANNEL=chrome node tools/ui/successor-fixtures/run.cjs --binary D:/spheres-scratch/s24/bin/spheres-web-26d5d63ae97b.exe --expected-revision 26d5d63ae97b0efc2cf5ddbb8a8260ed6e80d0c8 --out D:/spheres-scratch/s24/runs/final-26d5 --native-s21 D:/spheres-scratch/s24/runs/native-s21-26d5-1 --native-provenance D:/spheres-scratch/s24/native-s21-export-26d5.json --cross-check`:
  exit 0, no validation errors, 61 disposable archives hashed and deleted, server stopped.

| Layer | Passed | Failed | Blocked | Unrun |
|---|---:|---:|---:|---:|
| Selection guard (G0), all 23 | 23 | 0 | 0 | 0 |
| Activation + save/reload (N1 for 15 Soviet, H1 for 6 Yugoslav) | 21 | 0 | 0 | 2 (EastTimor, Namibia: `no_native_hook`) |
| Browser UI (B1): Russia N1, Kazakhstan N1, Serbia H1, Croatia H1 | 4 | 0 | 0 | 19 (17 `not_selected`, 2 `no_native_hook`) |

USSR → Russia (N1): continued through the served `continue_campaign` on 2 January 1990 (plus its lost-response replay);
selection identity and transition, 86/86 districts, government (proportional 5% threshold, LDPR-led, the Presidency),
ten-ministry budget on Russia's books, guidance bound to Russia and retained capabilities read the same after an
ordinary save/load with a byte-equal archive projection; seven days as Russia with duplicate-turn replays. The browser
case read the same through the Government, budget and Advisors rooms after activation and after a named save/load at
390 px. The H1 USSR fixture with the served aim command equals the N1 world byte for byte (informational cross-check).

- `node --test tools/ui/check_successor_fixtures.cjs`: 13 pass (a deliberate harness edit turns the provenance test red).
- `node tools/ui/successor-fixtures/inventory.cjs --check …/inventory.json`: PASS, 23 identities, 2 discrepancies.
- `python tools/planning/workboard.py --check`: PASS. `git diff --check` on the owned paths: clean.

**Screenshots.** `docs/campaign-certification/S24/preparation/successor-fixtures/evidence/screenshots/`:
`{russia,kazakhstan,serbia,croatia}-after_activation-government.jpg`, `…-after_activation-budget.jpg`,
`…-after_reload-guidance.jpg`.

**Finding F1 (proposal P3, informational).** Every activated successor's budget card calls the real-rate floor a
"sovereign spread" (Serbia: real −34.90%, +32.90pp at 28% of GDP): `policy_json` serves an unfloored real rate while
the simulation charges `REAL_RATE_FLOOR`. The effective rate is correct; only the breakdown and label disagree.

**Setup and cleanup.** See the harness README. The run used only `D:/spheres-scratch/s24`; no user campaign or save
location was read or written. The cargo target and frozen binaries were deleted after the run; the exporter's
`succession.json` input is kept by hash only (regenerate it with the command above).

**Limitations.** Every activation is a labelled fixture on 2 January 1990: no organic succession, historical timing or
S25 route is shown, and seven days is not a long-run check. Namibia and East Timor stay unrun until P1 is ruled on; the
Yugoslav family uses H1 because no native Yugoslav exporter exists (P2). Only four identities are browser-driven; map
ownership is read from served data, not the globe; advice quality is not judged. The harness runs on Windows paths
with an existing Playwright install and system Chrome. Full S24 still needs its canonical dependencies and Codex's
exact-build startup and recovery qualification.
