# S01 — frozen integration and verification baseline

Session date: 10 September 2026 (America/Los_Angeles). **Status: complete.** User approval: “Okay lets begin development with s1”. S00/G0 are
complete; S02–S30 remain planned. S01 establishes evidence and integration
contracts; it does not earn G1 or certify a campaign.

The integration will preserve **financial construction**, **company-owned
equipment development and stock**, and **historical/cartoon party leadership**.
Master contains useful population, industry, fiscal, warfare and city work, but
also conflicting economic and save behavior. A clean textual merge alone would
not establish compatible gameplay.

## Frozen inputs

`git fetch origin --prune` succeeded during this session. These are the fetched
revisions, not remembered commit counts. Repository:
`https://github.com/ridgemerkley2-web/Spheres.git`.

| Source | Exact revision |
| --- | --- |
| Active / `origin/codex/resume-spheres` | `5f7f355502f17bd6bd8f0383a2d14f0024fa7884` |
| `origin/master` | `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9` |
| `origin/claude/admiring-euclid-a867c9` | `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9` |
| `origin/codex/war-overhaul` | `eab54c35dbcb1f2bb486113e7361f5c57fb2f308` |
| `origin/codex/population-rebuild` | `825f7964ab3cceba9594e353634bb53f0ea86030` |
| `origin/feat/art-p0` | `3b755687710bca6dbf8963bbe4f9acd878423f63` |
| `origin/codex/industry-rebuild` | `f2709d120673bf24c02460fb68babe4be5a5ba80` |
| `origin/codex/company-system` | `0e6b8c6af0f5340f080b2216574dd6df9fa57345` |
| `origin/feat/hoi4-map-and-tech` | `fc0f0c299149bd4859fcc4653f4e56ae28dda5ba` |
| `origin/feat/arsenal-models-codex` | `092569227023ff4278a5d699018af46bd39c7c94` |
| `origin/feat/ideology-roads` | `cbcc3648bc4f0d5e1f93f552975af73cc44e391f` |

Active has **11 commits not in master**; master has **26 not in active**. Git
reports two merge bases: `ebddf7a47fc5d46fb0b01fd88dde9135e0384ce3` and
`fa2b4a025094a904dfa1401aea0f1f4a0d4f2c85`. Do not assume a single linear fork.

`git merge-tree --write-tree --messages <active> <master>` returned exit 1,
with nine conflict paths. This writes an advisory tree object; it does not merge
a branch or change a working copy. The advisory tree
`c4c253e6bb05c40b018e3464984a5261255cc9e1` contains unresolved conflict content
and must never be checked out as a completed integration.

| Conflict path | Integration owner |
| --- | --- |
| `COMPANIES.md` | S03: supplier property plus contractor assignments |
| `ROADMAP.md` | S05: reconcile delivered scope after integration |
| `docs/art/P0_MANIFEST.md` | S04/S22: preserve mesh/asset inventory and measurements |
| `spheres-sim/src/companies.rs` | S03: incompatible company models |
| `spheres-sim/src/lib.rs` | S02–S05: daily owners, commands and save dialects |
| `spheres-web/src/equipment_supply_view.rs` | S03: actual supplier lifecycle |
| `spheres-web/src/main.rs` | S02–S05: commands, profiles, enrollment and embedded assets |
| `spheres-web/ui/arsenal-models.js` | S04/S18: preserve detailed military meshes and asset IDs |
| `spheres-web/ui/index.html` | S04/S05: reconcile all room/module bindings |

## Isolated working locations

Paths below are relative to this task's `work/` directory. The machine-readable
manifest records resolved absolute paths.

| Location | Purpose |
| --- | --- |
| `Spheres` | Original active branch; only pathway, baseline documentation and test tooling change in S01 |
| `campaign-certification/integration` | New `codex/campaign-certification` branch starting at the exact active pin; clean input for active tests and subsequent integration |
| `campaign-certification/master-baseline` | Detached worktree at the exact master pin; baseline comparison only |
| `campaign-certification/evidence/S01` | Raw command results, hashes and measurement outputs |
| `campaign-certification/fixtures/lifetime-legacy` | Copies of three historical synthetic benchmark archives; input files are hashed before/after |
| `campaign-certification/disposable-campaigns` | Reserved for explicit migration fixtures in S02–S05 |
| `Spheres/company-sim-target` | Reused active Cargo build cache; the isolated worktree is recompiled before execution |
| `campaign-certification/master-target` | Separate master Cargo build cache |

Native tests use their own synthetic worlds and temporary test folders. S01
does not start a game/server or send commands to a running campaign. Eight
existing campaign/archive files in `work/economy-preview` are monitored by hash;
their contents are never checked into this evidence pack. Those files are a
known local save inventory, not proof of the identity of a campaign held only
in a running browser's memory.

The integration branch receives only the S01 documentation/tooling commit
by fast-forward after verification. Master game code is deliberately left for
the scoped S02–S05 integration work.

## Compatibility contracts

| Contract | Detailed evidence | Required outcome |
| --- | --- | --- |
| Population, workforce, projects, GDP and fiscal ownership | [ECONOMY_INTEGRATION.md](ECONOMY_INTEGRATION.md) | One owner for each ledger; preserve project IDs, paid entitlements and financial-only construction |
| Corporate stock, cash, development, deliveries, ammunition and refits | [COMPANY_AND_SAVE_CONTRACT.md](COMPANY_AND_SAVE_CONTRACT.md) | Supplier property survives; contractor assignments occupy a separate typed namespace |
| Native/browser saves, session commands, rule defaults, warfare, map and government | [RUNTIME_AND_VALIDATION.md](RUNTIME_AND_VALIDATION.md) | No silent opt-ins or dropped identities; retain retries, stale-session rejection and live asset bindings |
| Hardware, dated workloads and performance limits | [PERFORMANCE_BASELINE.md](PERFORMANCE_BASELINE.md) | Measured early/mid/late baseline with honest scope and fixed engineering budgets |

These documents inventory 16 project kinds, the original seven-site ledger and
new three-site ledger, advanced components, population and fiscal state,
company/save versions, commands and test generators. The preserved common
nation roster contains 137 starting and 23 successor identities; S04/S24 must
reconfirm this when integrating geography and succession.

## Baseline executions

All commands run at their named pinned worktree. Test times taken while other
functional tests compile/run are not performance measurements. Per-check hashes,
exit codes and evidence paths are recorded in [manifest.json](manifest.json).
The evidence directory disables Git text normalization so its SHA256 values
remain valid after checkout on Windows or Linux. Raw test output keeps its
original final blank lines; it is not reformatted as authored documentation.

| Check | Active | Master |
| --- | --- | --- |
| Locked release workspace Rust tests | 1,271 passed; 0 failed; 71 ignored | 1,339 passed; 0 failed; 70 ignored |
| `node tools/ui/run-unit.cjs` | 1,371 passed; 0 failed; 1 optional fixture skipped | 1,093 passed; 0 failed; 0 skipped |
| `node tools/ui/bench_art.cjs --check` | Fails on unsupported `materialClasses`; full budget count unavailable | Fails: stale budget report and 33 over-budget rows |
| Isolated lifetime profile | Six cases completed; within frozen headless budgets | Six cases completed; stress latency exceeds frozen budgets |
| Real-browser campaign/package checks | Not run in S01 | Not run in S01 |

The skipped UI case is `archived real API reading identifies renewal without
disabled takeover alarms`; its optional external evidence is absent from a
clean worktree. The prior active checkout passed all 1,372 with that fixture.
This session records the skip rather than silently treating it as a pass.

The headless runner also passed a syntax check and a negative overwrite check:
an existing evidence folder is refused before launching a child process, and
the original profile hash stays unchanged.

The updated pathway was checked at 736px and 360px in light/dark themes; selecting
S02 shows its planned status and prerequisite. No preview console errors were
reported. This is roadmap verification, not a game-browser qualification.

The source inventory lists all 71 active ignored Rust attributes. Default test
success does not run those cases. Only explicitly invoked ignored tests gain
fresh evidence. Local Node is v24.12.0; CI specifies Node 22. Windows local tests
do not establish a Linux or GitHub CI pass.

## Known issues and assigned sessions

| ID | Status / consequence | Owner |
| --- | --- | --- |
| S01-R-ECON-01 through 07 | Construction material/capacity regressions, automatic rule changes, lost leases, GDP mismatch, save break, demographic ownership and settlement order | S02; save checks in S05 |
| S01-R-COMP-01 through 07 | Supplier property/command/refit/ammunition/settlement incompatibility and deleted tests | S03; save checks in S05 |
| S01-F-ART-A | Active measurement code assumes an obsolete vertex layout; no valid upload budget measurement | S22 |
| S01-F-ART-M | Master's report is stale and 33 measured rows exceed its old ceilings | S22; preserve evidence during S04 |
| S01-F-PERF-M | Master stress latency reaches 7.7 s p95 and a 15.5 s maximum in the late fixture | S02/S04 diagnosis; S22 qualification |
| S01-F-SAVE-UI | Native active loader accepts a standalone party envelope; browser storage dispatch rejects it, although nested campaign envelopes work | S05 |
| S01-C-UI-FIXTURE | One optional advisor snapshot test skips in an isolated checkout | S21/S24: make its intended fixture portable or explicitly retain optional status |
| S01-C-BROWSER | No fresh live campaign, packaged build, FPS, UI latency or browser/GPU memory evidence in this session | S05, S22, S24, S28 |

An inventoried existing failure does not prevent completing a baseline session;
it prevents claiming a green release. These entries cannot be waived by changing
expected counts, hiding a failed branch or dropping a required country.

## S01 closure

All three session acceptance items are complete: fetched source pins and the
advisory merge are recorded; ownership/version/project risks have concrete
contracts and assigned sessions; isolated branch tests and actual dated
performance measurements are stored with hardware, binary and input hashes.
The eight monitored campaign files and three copied benchmark inputs are
unchanged. Game runtime files remain at the active input revision.

The output revision is the commit containing this report and `manifest.json`;
use `git log -1 --format=%H -- docs/campaign-certification/S01/manifest.json`.
This avoids a self-referential commit hash inside its own contents. The session
close message records the resolved output SHA.

## Next session: S02

Begin in `codex/campaign-certification`, using the frozen master input. Integrate
schema/identity additions with inert defaults, then financial construction and
operating receipts, then population/workforce, then GDP and fiscal observation.
Use the 12 preservation fixtures in the economy inventory. S03 owns supplier
integration and S04 owns operational warfare/map work; coordinate their shared
`lib.rs`, browser entry and save changes rather than merging them blindly.

The S02 exit is a connected daily economy with once-only people, jobs, output,
cash and debt accounting. S05 qualifies combined saves before G1 can close.
