# SPHERES v0.5 — Playable Slice

Grand strategy simulation, January 1990 start. Deterministic core (SplitMix64,
single command queue, seeded), 24 nations at the start and up to 30 once
federations come apart, a 253-technology tree, monthly ticks.

## Run it
    cargo run --release -p spheres-web                  # browser UI — map, charts, diplomacy
    cargo run --release -p spheres-cli -- play          # interactive, default seed
    cargo run --release -p spheres-cli -- play 42       # different history
    cargo run --release -p spheres-cli -- run 30 1990   # headless 30-year report
    cargo run --release -p spheres-cli -- resume save.json
    cargo test                                          # 46 calibration/invariant tests

`spheres-web` opens http://127.0.0.1:7777 in your browser: a strategic map of the
world sized by GDP and coloured by your relations, policy sliders, GDP/oil history
charts, a league table, and a dispatch feed. Click a nation to open it and act on it.
Time advances in months; a war or a collapse interrupts a long advance so you can
respond.

In the CLI, type `help`. Core loop: read briefing -> set policy (rate/tax/military/
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
- Yugoslavia comes apart in the nineties and the wars follow from separatism, not
  from a script; Slovenia gets out early, Bosnia does not
- Ukraine is born out of the Soviet dissolution and gives its warheads back
- The United States takes and holds the technological frontier, and the nations
  behind it converge by copying — which is cheaper than inventing, and gets
  harder the closer they get

## Notes
This is the v0.5 rebuild of the sim core (compact re-implementation after the
v0.4 container was lost): same architecture, monthly rather than hourly ticks.
Reimplemented since: Yugoslavia + successors, the expanded roster, and the
technology tree. Still not back from v0.4: democratic election detail, and
technological eras as live rotating paradigms — the tree carries `Era` only as a
calibration bracket, not as a paradigm the world passes through.

Two systems are written but not merged, each blocked on a decision rather than on
work: `feat/statecraft` (pacts, patronage, covert action, trade) and
`feat/financial-system` (currencies, FX regimes, contagion). Both are green on
their own branch and both collide with the expanded roster — see ROADMAP.
