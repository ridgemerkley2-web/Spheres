# SPHERES

A grand strategy game beginning in January 1990. Govern a country through
budgets, construction, research, companies, diplomacy and military operations.
The Rust simulation runs locally with a browser interface and saved campaigns.

**Development home: `codex/campaign-certification`.**
This is the single integration branch for the game. Start here when building,
reviewing or continuing work; older branches are historical snapshots.

## Where development stands

As of **30 September 2026**, **S01–S22 of the 30 core milestones are complete**.
The economy, government, company procurement, ground/air operations, AI,
tutorial, navigation and performance milestones are integrated.
The first certified 1990–2035 campaign release is still in development.

The next formal milestone is **S23: historical character and cartoon coverage**,
which needs the eight country casts finished first. Engineering continues
alongside it on the full campaign test matrix and the failing political-balance
check. Human playtesting and final release qualification follow.

**[Read the current roadmap and next actions →](ROADMAP.md)**

| Start here | Purpose |
|---|---|
| [Roadmap](ROADMAP.md) | Current priorities, blockers, owners and the path to release |
| [Contributing](CONTRIBUTING.md) | How to start work, verify it and integrate it |
| [Work assignments](docs/AI_WORKSTREAMS.md) | Exact tasks, existing claims and handoff records |
| [Documentation](docs/README.md) | Design, system references, evidence and archives |

## Play from source

Install Rust, then run from this checkout:

```sh
cargo run --locked --release -p spheres-web
```

Open <http://127.0.0.1:7777> if the browser does not open automatically.
On Windows, `Play SPHERES.cmd` runs the same local game. Save before closing;
the server's working directory owns the campaign files.

A prepared portable package can run without Rust, but a certified public release
has not been published. See the [player guide](docs/PLAYING.md) for campaign,
map and recovery controls.

## Repository

- `spheres-sim/` — deterministic simulation and sourced world data.
- `spheres-web/` — local server, browser UI and game assets.
- `spheres-cli/` — headless runners and calibration tools.
- `tools/` — verification, research, assets and packaging.
- `docs/` — current contracts, planning, evidence and historical records.

The [latest verification runs](https://github.com/ridgemerkley2-web/Spheres/actions/workflows/verify.yml?query=branch%3Acodex%2Fcampaign-certification)
report engineering health. Component or package tests do not complete
the remaining campaign, content or human-playtest requirements.
