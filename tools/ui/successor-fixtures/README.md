# S24 successor-country fixture harness

**Preparation for S24 only.** Every successor activation made here is a labelled
**fixture**: it does not prove organic succession, historical timing or the S25
long-campaign route, and it is not S24 qualification. Codex owns S24 and its
exact-build startup and recovery qualification.

| File | Purpose |
|---|---|
| `inventory.cjs` | Builds the successor identity inventory from current data and simulation source; `--write` / `--check`. |
| `run.cjs` | Runs the fixture layers against an already-built `spheres-web` binary in an isolated directory. |
| `lib.cjs` | Pure helpers: Rust table readers, byte-exact save staging, labels and result validation. |
| `../check_successor_fixtures.cjs` | `node --test` suite (also picked up by `node tools/ui/run-unit.cjs`). |

Evidence and the committed inventory live in
[`docs/campaign-certification/S24/preparation/successor-fixtures/`](../../../docs/campaign-certification/S24/preparation/successor-fixtures/README.md).

## What is checked

Each of the 23 expected successor identities gets exactly one result per layer,
with status `passed`, `failed`, `blocked` or `unrun` (never omitted).

| Layer | Identities | What happens |
|---|---|---|
| `selection_guard` (recipe **G0**) | all 23 | `/api/new` refuses the successor as a 1 January 1990 start with the served text, the running campaign is untouched, the successor is absent from `/api/roster`, and `/api/sources` reports `start_1990: false`. No successor state is created. |
| `activation_save_load` (**N1** or **H1**) | the 21 with a native activation path | Load a dissolved-parent fixture through `/api/load`; confirm the parent is dead, the served *Continue as X* offer exists and turns are paused; send the **served** `continue_campaign` command (and its lost-response replay); inspect selection identity, map ownership, government, budget, guidance and retained capabilities; save, reload through the ordinary path, inspect again, prove equal readings and a byte-equal archive projection; then advance seven days as the successor with duplicate-turn replays. |
| `browser_ui` (**B1** on N1/H1) | selected (default Russia, Kazakhstan, Serbia, Croatia) | The same fixture through ordinary controls: Campaigns → Load, Campaign → *Continue as X* → confirm; header identity, Government room (hero, officeholder, electoral/regime structure, UI data equal to the native reading), Economy → *Your yearly budget* (ten ministries, nation kicker), Advisors (native `/api/guidance` reading for the successor); named Save and Load, then the same rooms at 390 px; exactly one command and no advance issued; no page errors. |

Namibia and East Timor have **no native activation path** (no dissolution seats them,
no `districts::SUCCESSOR_PARENTS` entry, no continuation family). Their activation and
browser cases are recorded `unrun` / `no_native_hook` and cite the
[integration proposal](../../../docs/campaign-certification/S24/preparation/successor-fixtures/integration-proposal.md).
The expectation of 23 is not reduced.

## Fixture recipes

**G0 — selection guard.** Read-only against the boot campaign; label kind
`selection_guard_no_activation` (`fixture: false`, `activation: false`).

**N1 — existing native S21 exporter (USSR family, reused unchanged).**
`campaign_journey::tests::s21_export_review_fixtures` authors `succession.json`:
`Game::new_fresh(1990, USSR)` with `fresh_play_rules` and a Prosperity aim, the union's
stability 0 and separatism 1 set in memory, and one real daily advance in which
`politics::tick` runs `dissolve_ussr`. The same archive serves all 15 Soviet
successors, because the player chooses which one to continue as. Label kind
`native_s21_authored_dissolution`.

**H1 — harness-staged parent collapse (both families; the only recipe for Yugoslavia).**
1. `/api/new {seed: 1990, nation: <parent>}` on the exact binary, then `/api/save`.
2. In that disposable save only, replace the parent's `stability` and `separatism`
   raw number tokens with `0.0` and `1.0`. The save is never re-serialised (after the
   first tick the world carries a u64 RNG state a JavaScript number cannot hold); the
   two tokens are spliced and every other byte is proven identical (`verifyOnlyPatched`).
3. Load it through the ordinary `/api/load`, advance one real day, and require the
   parent's dissolution flag, its death and the full served continuation family.

Label kind `harness_staged_parent_collapse`. `--cross-check` additionally builds an
H1 USSR fixture with the served `choose_campaign_aim prosperity` command and compares
its simulation world, key by key as raw bytes, with the N1 fixture.

Both recipes stage the same two fields the native tests stage
(`campaign_journey::tests::dissolved`, `s02_succession`, `a_dissolved_state_is_not_served_as_a_live_belligerent`);
nothing else about the world is authored, and every later step is ordinary play.

## Isolation and safety

* The web server's only save root is its **working directory**:
  `storage::write/read/list` are called with `Path::new(".")` in
  `spheres-web/src/main.rs` (`save.json`, `saves/<slot>.json`, `saves/auto-N.json`).
  The harness starts its **own** server with `cwd = <out>/server`, refuses to run
  unless `/api/build.save_directory` equals that directory, and uses harness-chosen
  slot names only. It never lists, reads or writes any other save location
  (for example `C:/Users/ridge/saves` or another server's directory).
* `--out` must be a new absolute directory **outside the repository**.
* The port is a free local port and never 7777 (another session's server); the server
  gets `--no-open`, and is stopped at the end (`cleanup.server_stopped`).
* Browser contexts are fresh and headless; nothing persists between cases.
* Every disposable archive is hashed and then deleted (`--keep-saves` keeps them).

## Setup

```sh
# 1. Build once into your own target directory, then freeze a copy of the binary:
#    a later `cargo test` re-runs build.rs and relinks target/release/spheres-web.exe.
CARGO_TARGET_DIR=<own-target> cargo build --locked --release -p spheres-web
cp <own-target>/release/spheres-web.exe <frozen>/spheres-web.exe

# 2. Native N1 fixture (USSR). A sparse checkout also needs tools/campaign for this
#    test build. SPHERES_S21_FIXTURE_DIR must be a NEW absolute directory.
SPHERES_S21_FIXTURE_DIR=<new-abs-dir> CARGO_TARGET_DIR=<own-target> \
  cargo test --locked --release -p spheres-web campaign_journey::tests::s21_export_review_fixtures -- --ignored --exact

# 3. Browser layer: Playwright 1.58.2 without downloading anything, system Chrome.
export NODE_PATH=<an existing tools/ui/node_modules containing playwright 1.58.2>
export SPHERES_BROWSER_CHANNEL=chrome
```

The harness must be committed (it refuses a dirty harness or runtime tree unless
`--allow-dirty`, which marks the result `development_run: true`), and the runtime
source at `HEAD` must equal `--expected-revision`, the commit the binary was built from.

## Run

```sh
node tools/ui/successor-fixtures/run.cjs --binary <frozen>/spheres-web.exe \
  --expected-revision <40-hex build commit> --out <new absolute dir outside the repo> \
  --native-s21 <dir with succession.json> --native-provenance <exporter record.json> --cross-check
```

| Option | Default | Meaning |
|---|---|---|
| `--api all\|none\|A,B` | `all` | Identities for the API layer (identities without a native path stay `unrun`). |
| `--browser Id:Recipe,...\|none` | `Russia:N1,Kazakhstan:N1,Serbia:H1,Croatia:H1` | Browser cases; `N1` needs `--native-s21` and a Soviet successor. |
| `--native-s21 <dir>` | none | Without it, Soviet successors use H1. |
| `--native-provenance <json>` | none | Exporter command, revision and hashes, embedded in the result. |
| `--cross-check` | off | H1-versus-N1 world comparison (informational). |
| `--keep-saves` / `--allow-dirty` | off | Keep archives / development run. |

Outputs in `--out`: `result.json` (format `spheres-s24-successor-fixture-results/v1`),
`screenshots/*.jpg` (viewport-clipped, modest), `progress.jsonl` (HTTP timings),
`server.log`, and an empty `server/saves` after cleanup. The run validates its own
result with `lib.validateResults` and exits non-zero on any validation error.

A **failed** case is a product check that did not hold (the failing check and observed
values are recorded); **blocked** means the harness could not reach a stage (environment
or setup error, with the stage and error); **unrun** is `no_native_hook` or `not_selected`.

## Inventory

```sh
node tools/ui/successor-fixtures/inventory.cjs                       # summary
node tools/ui/successor-fixtures/inventory.cjs --write <inventory.json>
node tools/ui/successor-fixtures/inventory.cjs --check <inventory.json>   # full semantic drift check
```

Sources: `docs/planning/campaign-pathway.json` (S24 statement), C01 `countries.json`
and `census.json`, `spheres-sim/src/nations.rs` (roster), `districts.rs`
(`SUCCESSOR_PARENTS`), `politics.rs` (dissolutions and triggers),
`spheres-web/src/campaign_journey.rs` (continuation families),
`spheres-sim/data/districts.json` and the successor start refusal in `main.rs`.
Discrepancies are recorded, never absorbed into the expectation.
`node --test tools/ui/check_successor_fixtures.cjs` compares identities and
activation paths against live sources and validates the committed results.

## Cleanup

Automatic: the server is stopped and every disposable archive deleted. Afterwards
delete the `--out` directory, any `SPHERES_S21_FIXTURE_DIR` directories and your
cargo target directory. Nothing else is created.

## Limits

Fixtures dissolve the parent on 2 January 1990 by authored staging; organic timing,
long-run behaviour and the S25 route are out of scope. Only the four default
identities are browser-driven; the other 17 activatable successors are covered at the
served-API level. The map is checked through served district ownership, not the
WebGL globe. Guidance checks bind the reading to the successor; they do not judge
advice quality. Money semantics are not asserted (see the evidence README's findings).
