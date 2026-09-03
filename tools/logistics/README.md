# Physical logistics network

`build_network.py` bakes the route graph consumed by
`spheres-sim/src/logistics.rs`. Run it from the repository root with the
workspace Python runtime:

```powershell
python tools/logistics/build_network.py
```

The output is `spheres-sim/data/logistics_network.json`. It is deterministic:
running the generator twice against unchanged inputs must produce the same
bytes.

## What the graph means

- District identity, land adjacency, centroids and coastal contact come from
  the game's committed Natural Earth-derived map artifacts.
- Coastal gateways are modeled freight handoff points. They are **not** claims
  that a particular historical port, terminal or railhead existed there.
- Sea nodes are a coarse ocean sampling used to draw and cost strategic paths.
  This graph is not suitable for navigation.
- Named chokepoints are schematic connectors and edge labels. The identity and
  strategic role of the listed passages are documented by the U.S. Energy
  Information Administration's [World Oil Transit Chokepoints](https://www.eia.gov/international/analysis/special-topics/World_Oil_Transit_Chokepoints)
  atlas. Their graph coordinates are display placement, not survey coordinates.
- No historical port count, route throughput, road length, rail length or 1990
  service schedule is inferred. Capacity is an openly modeled gameplay value
  in the simulation.
- The six roster nations without drawable district geometry remain unmapped.
  The game reports that refusal instead of inventing a route.

## Inputs

- `spheres-web/ui/districts.js` — projected district shapes and centroids.
- `spheres-web/ui/coast.png` — the globe's signed coastline field.
- `spheres-sim/data/districts.json` — stable identity and land adjacency.

The output metadata records the SHA-256 of every input. The generator uses the
same Robinson constants and inverse as `mapgen.rs` / `globe3d.js`, so route
coordinates remain registered to the globe.

Requires Pillow, which is part of the bundled Codex workspace Python runtime.

## Browser smoke check

`check_browser.cjs` uses Playwright against a **disposable localhost server**.
It starts a France campaign, advances through normal game controls, tests
routing-policy buttons, cargo expansion and focus, a narrow viewport, and the
infrastructure shortcut. It never saves or overwrites `save.json`.

```powershell
node tools/logistics/check_browser.cjs http://127.0.0.1:7777 --allow-new-game path/to/screenshots
```

Install/provide Playwright via `NODE_PATH`. If using a system browser instead of
Playwright's downloaded Chromium, set `PLAYWRIGHT_CHANNEL` to `msedge` or
`chrome`. The `--allow-new-game` argument is required because this resets the
in-memory campaign on that test server. Do not point it at a game in progress.
