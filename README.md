# SPHERES v0.5 — Playable Slice

Grand strategy simulation, January 1990 start. Deterministic core (SplitMix64,
single command queue, seeded), 16 nations, monthly ticks.

## Run it
    cargo run --release -p spheres-cli -- play          # interactive, default seed
    cargo run --release -p spheres-cli -- play 42       # different history
    cargo run --release -p spheres-cli -- run 30 1990   # headless 30-year report
    cargo run --release -p spheres-cli -- resume save.json
    cargo test                                          # 10 calibration/invariant tests

Type `help` in-game. Core loop: read briefing -> set policy (rate/tax/military/
invest), act diplomatically (improve/sanction/war) -> `next`, `year`, or `6` to
advance -> world reacts.

## What emerges (unscripted, seed-dependent)
- Iraq invades Kuwait in the early 90s; a US/UK coalition repels it; Iraq never tries again
- The USSR dissolves ~1991-95 from stagnation + separatism; Russia inherits the arsenal
- Japan's bubble pops into a lost decade
- China compounds ~7-9%/yr into a 9x miracle
- India & Pakistan test in 1998; nuclear deterrence forbids their wars thereafter
- The coalition embargo on Iraq outlives the war by about a decade, hollowing its
  economy while the shortfall it caused keeps oil dear for everyone else

## Notes
This is the v0.5 rebuild of the sim core (compact re-implementation after the
v0.4 container was lost): same architecture, monthly rather than hourly ticks,
smaller roster. Not yet reimplemented from v0.4: Yugoslavia + successors,
democratic election detail, tech eras.
