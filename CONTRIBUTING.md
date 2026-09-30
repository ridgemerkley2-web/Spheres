# Contributing to SPHERES

## One development home

`codex/campaign-certification` is the game's integration and default branch.
Read [ROADMAP.md](ROADMAP.md), then [the workboard](docs/AI_WORKSTREAMS.md).
Create a short-lived task branch from its latest remote tip in an isolated
checkout. Integrate reviewed work back into that same branch. Never force-push
it or overwrite another checkout's uncommitted work.

The `dashboard` branch only hosts the existing status website. Research branches
with unfinished claims are temporary inputs, not competing game versions.
Retired branches are preserved under `archive/2026-09-30/*` tags with exact
commit IDs in the [branch inventory](docs/archive/branches-2026-09-30.json).
Do not restart or push an old branch without checking its disposition first.

## Choose and finish a task

```sh
python tools/planning/workboard.py --tasks --owner Codex
python tools/planning/workboard.py --tasks --owner Claude
python tools/planning/workboard.py --check
```

Check the [live branch exceptions](docs/AI_WORKSTREAMS.md#existing-work-to-preserve)
for newly claimed work awaiting integration into the task register. Claim one
bounded deliverable with owner, base, touched files and next checkpoint. Keep
progress in its handoff and update the task queue after review.
Canonical session completion needs its own acceptance evidence.

The [game vision](docs/design/BIBLE.md), [design specification](docs/design/SPEC.md)
and [engineering rules](docs/development/ENGINEERING.md) govern implementation.
`spheres-sim` owns game state and rules. UI code presents authoritative results.
Preserve determinism, real financial/property ownership, save compatibility,
original research evidence and failed checkpoints.

## Verification

Use a separate `CARGO_TARGET_DIR` for each worktree. For game changes, run the
relevant focused checks and required full verification before claiming completion:

```sh
cargo test --locked --release --workspace --no-fail-fast -- --skip tests::the_resource_pass_stays_under_budget
cargo test --locked --release -p spheres-sim --lib tests::the_resource_pass_stays_under_budget -- --exact --nocapture --test-threads=1
node tools/ui/run-unit.cjs
cargo build --locked --release -p spheres-web
```

Run the absolute resource timing test alone, without another CPU-heavy workload.
It retains the 0.15 ms/month limit. Browser/recovery tests must use disposable
campaigns; never aim them at the user's active server or saves. Native/UI/browser
checks, historical sourcing, long campaigns, performance and human playtests each
prove different things. Report the exact build, commands, outcomes and limits.

Documentation-only changes need link/path and workboard validation. Changes to
packaging or planning tools also need their affected tool tests. Do not change
gameplay or rerun expensive simulations merely to reorganize documentation.

## Keep the repository clear

- README introduces the game and points to the roadmap.
- ROADMAP explains current priorities; the existing JSON registers own status.
- `docs/reference/` and `docs/design/` hold system contracts and design intent.
- `docs/campaign-certification/` holds evidence; keep original artifacts and
  source pins intact. Historical source paths can be opened at their pinned commit.
- `docs/archive/` holds superseded plans, journals and branch recovery records.
- Close task branches after acceptance and integration; preserve unique work
  before retiring a branch. Keep source/data/assets in normal Git history.
