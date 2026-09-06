# SPHERES 0.6

A deterministic grand strategy sandbox beginning in January 1990, with 137
starting nations, successor states, eight research domains and 328 discoveries.
Govern through budgets, production, trade, diplomacy and military commitments.
The browser advances by **calendar day**, with pause, five speeds and one-day steps.
The command-line historical runner retains its separate monthly calibration mode.

## Play

For the ready-built Windows release, extract the whole ZIP to a writable folder
and double-click **Play SPHERES.cmd**. No Rust installation is needed. Keep the
server window open while playing. The game and its artwork run locally.

For a source checkout, install Rust, then double-click the repository's
`Play SPHERES.cmd`, or run:

```sh
cargo run --locked --release -p spheres-web
```

Open http://127.0.0.1:7777 if the browser does not open. A custom port is available
with `--port 7823`; `--no-open` suppresses automatic browser launch. **About** shows
the precise version, branch, source revision, build date and absolute save folder.
Saves use the server's working directory. The packaged launcher selects its own
folder so moving a shortcut does not move your saves.

The clock is HOI4's: it runs by itself until something stops it. Space stops it
from any screen — the cabinet, the tech screen and the resource board included —
and starts it again from the map or either of those two boards; 1-5 pick a speed
(one simulated day every 1000, 500, 250, 100 or 0 ms of real time — the last is
as fast as the server answers), + and - walk that ladder, N steps a single day.
Opening the shortcut card or Global Command stops the clock outright. A war or a
collapse pauses it and says why, and you press Space again when you have decided
what to do.

Choose a country, use **Advisor** to fund a budget and follow a development
project, and use **Find** to open a province without hunting on the globe.
**Research list** explains availability, prerequisites, payoff and estimated time.
**Decisions** contains diplomatic requests, standing policies, monetary choices
and optional peaceful campaign aims. Domination remains an available aim.

The map opens in **Terrain**. Use its corner controls to switch to **Political**
or **Fronts**, zoom, rotate, return to **World**, or center **Home**. **Details**
toggles detailed terrain, borders, provinces, cities, labels, physical names and an optional
coordinate grid; **More layers** holds the economic and resource views.
Terrain reveals a separate 4800×2036 NOAA elevation layer as you zoom in,
along with physical-region names and a hierarchy of named rivers. Graphics
hardware that cannot load this layer retains the base relief. Provincial
boundaries appear gradually as you zoom in.
Drag to rotate, scroll or pinch to zoom, and select a province to inspect it.

## Campaigns and recovery

The main menu offers **Continue campaign**, **New campaign** and **Saved campaigns**.
Continue resumes the world already running in the local server. Saved campaigns
contains Load, available backups and named saves. Save explicitly before closing.

New campaign opens the searchable 1990 nation roster. Compare opening population
and output, choose a nation, then press **Govern**. **World settings** contains the
optional seed. Historical avatars are drawn from each nation's history and do not
identify its serving leader in 1990.
Named slots and the default `save.json` retain the world, dispatch archive and
multiresolution history. Atomic writes keep a previous backup; rotating autosaves
provide additional recovery points. The save screen lists slots and backups.
Older raw-world saves remain readable, but cannot recreate history they never
stored. New 1990 industry profiles are granted only when starting a new campaign.

A lost action response leaves a visible pending receipt. Retry that receipt or
review the authoritative state. Reload preserves pending command identity;
repeating the same receipt cannot charge the action twice. Starting or loading a
different campaign invalidates old session receipts.

## Development and verification

```sh
cargo test --locked --release --workspace --no-fail-fast
node tools/ui/run-unit.cjs
cargo build --locked --release -p spheres-web
```

The Node unit runner needs no browser or dependencies. CI also installs the
pinned browser tooling in `tools/ui` and runs `ci-browser.cjs` against its own
disposable server. Never point recovery tests at a campaign you want to keep.
Windows and Linux CI cover the locked Rust build, simulation/accounting tests,
UI tests, lost-response recovery, saves and narrow-screen rendering. The
workflow must run on GitHub before a remote CI result can be claimed.

Daily calibration is separate from the legacy monthly report:

```sh
cargo run --locked --release -p spheres-sim --example daily_calibration -- --years 30 --seeds 1990,7,42 --output results.csv
cargo run --locked --release -p spheres-cli -- run 30 1990
```

The first command exercises named scenarios using browser daily rules and
writes per-year observations and summary variance. Three seeds are an exploratory
baseline, not proof of balanced difficulty. The nightly/manual calibration
workflow keeps this slower scan separate from pull-request verification.

## Current contracts

Start with [CURRENT_ARCHITECTURE.md](CURRENT_ARCHITECTURE.md) and
[DECISIONS.md](DECISIONS.md). Detailed rules live in
[PLAYER_DECISIONS.md](PLAYER_DECISIONS.md),
[MILITARY_OPERATIONS.md](MILITARY_OPERATIONS.md),
[CAMPAIGN_AIMS.md](CAMPAIGN_AIMS.md),
[SECTOR_PROFILES.md](SECTOR_PROFILES.md),
[MANUFACTURING.md](MANUFACTURING.md) and
[PROVINCE_ECONOMY.md](PROVINCE_ECONOMY.md).
[PLAYTEST.md](PLAYTEST.md) gives a short, repeatable usability protocol.

Historical design rulings remain in BIBLE.md, SPEC.md and the domain documents.
They should be read with dated amendments; earlier roadmap statements are not a
reliable description of the current browser. Missing elections detail, household
microeconomics and individual military platforms are not implied by this release.
