# S21 — campaign goals, history and continuation

Status: **complete** on `4ac849a4f5a636a0e573e5cfdad03e05c7133566`. [Exact qualification and retained evidence](manifest.json).
**G4 and CP1 remain open.** Claude owns S19; S20 awaits its integration.

The new **Campaign** dock button opens an illustrated overview, goals and history.
It shows real national changes, fixed goal targets, qualifying days, current
blockers and retained achievements. Military agendas remain accessible through
Global Command. Goal completion creates no economic or military rewards.

History pages the complete saved dispatch archive by economic, government,
military and diplomatic category. Former-country events stay reachable after
succession. Metric changes are observations; the page does not invent causal
explanations. Older simulation-only saves cannot recover history they never held.

A lost government now has a clear result and an explicit observer option. A
simulation-authored USSR or Yugoslavia dissolution offers only living successors.
The successor keeps its existing economy, government, military and obligations;
unfinished old-country goals close in that country's record. Pending receipts are
preserved, stale-country orders are refused, and unsent former-country drafts are
cleared. No arbitrary country switching or free assets are introduced.

Every day through **31 December 2035** settles. On 1 January 2036, play pauses for
review; the player may save the result or explicitly continue in open-ended
sandbox. That choice survives reload and does not award victory or certification.

The browser scenario exposed an immediate post-dissolution save defect: newborn
republics lacked complete government/party-leadership records until the next tick.
Integrated campaigns now initialize both at birth through the existing government
and roster rules. The first-day save reloads without changing any world field.
No historical character coverage is claimed by this initialization repair.

## Qualification

- Windows and Linux: **404 native web tests**, 21 ignored fixture/stress entry points.
- Windows and Linux: **886 simulation library tests**, exact counts and any ignored tests in the manifest.
- Windows and Linux: **1,603 UI tests**, one existing skip. UI/unit sources are byte-identical to their `5b92fbb` qualification; the manifest records the exact three-file later delta.
- Chrome: 1440px and 390px review, goals/history navigation, Escape/focus return,
  failed-read recovery, real continuation controls, save/load after 2035 and USSR → Russia.
  Two continuation orders, zero browser errors. The world and archive are compared
  before/after inspection; successor changes use an exact expected-world oracle.
- Eight protected original files and both protected worktree heads remain unchanged.

The late-date and dissolution starting conditions are **authored test scenarios**.
They exercise ordinary daily simulation and visible controls, and do not stand in
for S25's full 1990–2035 campaign or S26's human playtests. Early failed runs are
retained, with large diagnostic logs compressed losslessly.

## Review and reproduce

The separate review runtime is [localhost:7861](http://127.0.0.1:7861/). Choose
Continue, then Campaign. It uses a copy of the existing France/Italy checkpoint
on 16 October 1992; original review servers and saves remain available.

```text
cargo test --locked --release -p spheres-web
cargo test --locked --release -p spheres-sim --lib
node tools/ui/run-unit.cjs
```

For browser reproduction, export to a **new absolute** `SPHERES_S21_FIXTURE_DIR`:
`cargo test --locked --release -p spheres-web s21_export_review_fixtures -- --ignored`.
Build the same clean revision, then set `SPHERES_BINARY`, `SPHERES_EXPECTED_REVISION`
and a new absolute `SPHERES_S21_BROWSER_OUTPUT` and run
`node tools/ui/ci-campaign-journey.cjs`. The scenarios and commands are checked in;
fixture hashes, build identity and browser records are retained in the manifest.

Shared-shell edits are limited to the Campaign entry/refresh and player-context
transport wiring. No tutorial/advisor module was changed; Claude should bring its
S19 patch forward onto this integrated base before S20 navigation work.
