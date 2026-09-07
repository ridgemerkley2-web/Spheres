# The world map is 661 m/px, and that settles where scatter can live

Measured 2026-09-07 in the running game, not argued from the source.

## What was built, and what happened to it

`spheres-web/ui/world-scatter.js` is a finished, tested terrain-following
placement pass: 609 lines, 17 checks in `tools/ui/check_world_scatter.cjs`
including a ten-entry sabotage ledger where each defect must fail on its own
named bar. It plans stable-under-pan instance positions from injected height,
land and vegetation samplers. None of that is in question.

It was wired onto the globe as a ground-dressing overlay and the wiring was
**removed the same session**, because the premise was wrong by three orders of
magnitude.

## The measurement

Driven to the Olympic Peninsula in a live campaign, terrain surface ready:

| zoom | ground scale at screen centre | a 20 m tree is |
| --- | --- | --- |
| 64 | 1,134 m/px (vertical), ~526 m/px (horizontal) | 0.02 px |
| 192 — `ZOOM_MAX`, the most the game allows | 661 m/px | **0.03 px** |

The visible footprint at maximum zoom is still 8.5 deg x 1.7 deg, roughly 640 km
across. At zoom 64 it is 32 deg x 7.7 deg — Vancouver to Calgary.

The overlay drew each instance as an 8-26 px sprite. At the true scale every one
of them was about **500x too large**: not ground dressing but a forest of trees
the size of counties. A layer whose every mark misstates its own size by that
factor is a lie about the world, and the roadmap's rule against inventing
geography applies to inventing SCALE just as much as to inventing roads.

An earlier version of this file refused any window wider than six degrees,
believing that "zoom past 32 means a city view". That belief was never measured.
It is false: the globe keeps a distant horizon at every zoom, so the visible
ground stays continental no matter how far in the camera goes.

## Where the kit does work, measured the same way

`scatter-mesh.js` is not at fault — it is built for scenes, and the scene scales
in this game are right for it:

| surface | footprint | scale on a 200 px card | a 15 m oak |
| --- | --- | --- | --- |
| town block | 148 x 104 m | 0.74 m/px | 20 px |
| construction site | 55-77 x 42-54 m | 0.36 m/px | 42 px |

Both read. So the kit has a home; the globe is not it.

Of those two, the town block **already plants itself** — 4.1% of its vertices
are canopy-coloured in the residential and mixed districts, 5.4% commercial,
6.6% civic, 6.4% industrial (measured by colour, not by part name, so a part
merely called "park" cannot pass). Adding scatter there would duplicate work
town-mesh already does.

**All thirteen construction kinds have 0.00% canopy.** They sit on bare ground.
That is the one place in this game where the scatter kit is both correctly
scaled and genuinely missing, and it is the identified next step.

## What is true of world-scatter.js right now

It has no consumer. It is finished, tested and unrouted, and it is deliberately
NOT served: shipping 33 KB of placement plus 102 KB of kit to every page load
for a layer nothing draws is a cost with no return. Site perimeter planting is
a local layout problem — where the fence is — not a geographic one, so it is not
obvious that a lat/lon placement pass is the tool for it either.

Restoring the globe wiring means reverting this commit, and the numbers above
say not to.
