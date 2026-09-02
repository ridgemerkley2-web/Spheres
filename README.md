# SPHERES v0.5 — Playable Slice

Grand strategy simulation, January 1990 start. Deterministic core (SplitMix64,
single command queue, seeded), 137 nations at the start and up to 158 once
federations come apart, a 328-technology tree, monthly ticks.

## Run it
    cargo run --release -p spheres-web                  # browser UI — map, charts, diplomacy
    cargo run --release -p spheres-cli -- play          # interactive, default seed
    cargo run --release -p spheres-cli -- play 42       # different history
    cargo run --release -p spheres-cli -- run 30 1990   # headless 30-year report
    cargo run --release -p spheres-cli -- resume save.json
    cargo test                                          # 103 calibration/invariant tests

`spheres-web` opens http://127.0.0.1:7777 in your browser: a strategic map of the
world sized by GDP and coloured by your relations, policy sliders, GDP/oil history
charts, a league table, and a dispatch feed. Click a nation to open it and act on it.
Time advances in months; a war or a collapse interrupts a long advance so you can
respond.

In the CLI, type `help`. Core loop: read briefing -> set policy (rate/tax/military/
invest), act diplomatically (improve/sanction/war) -> `next`, `year`, or `6` to
advance -> world reacts.

## Manufacturing

The browser's **Production → Manufacture** board routes the existing defense
procurement budget through completed province arms plants, the resource market,
and long-lead Arsenal orders. See [MANUFACTURING.md](MANUFACTURING.md) for the
economic contract, every current connection, and the staged integration plan.

## What emerges (unscripted, seed-dependent)
- Iraq invades Kuwait in the early 90s; a US/UK coalition repels it; Iraq never tries again
- The USSR dissolves ~1991-95 from stagnation + separatism; Russia inherits the arsenal
- Japan's bubble pops into a lost decade
- China compounds into a roughly 11x miracle over thirty years
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

Statecraft is merged: mutual defence pacts with an upkeep both signatories pay,
patronage as a standing transfer, trade dependency that accumulates and then
becomes leverage, and covert action that is deniable until it is not.

`feat/financial-system` (currencies, FX regimes, contagion) is still on its own
branch, blocked on data rather than on work: `WorldState.finance` covers only the
original 16 nations. See ROADMAP.
