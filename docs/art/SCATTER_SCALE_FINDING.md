# The world map is 55-417 m/px, and that settles where scatter can live

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

Measured in a live campaign with the terrain surface loaded, over San Francisco
Bay (-122.25, 37.85) on an 1878x889 map, at `ZOOM_MAX`:

| where in the view | east-west | north-south | a 20 m tree is |
| --- | --- | --- | --- |
| at the centre, where the camera is pointed | **55 m/px** | 112 m/px | **0.36 px at best** |
| three quarters of the way to the horizon | 102 m/px | **417 m/px** | 0.05 px |

The two axes differ because the camera is pitched: north-south is foreshortened
and gets worse the further up the screen you read.

**CORRECTED 2026-09-07.** An earlier version of this file said "661 m/px" and
quoted it as *the* scale of the map. It was a real measurement but a horizon-ward
one, taken through the resting camera tilt at a point far from the centre of the
screen, and presenting it as the general figure overstated the coarseness by
about twelve times against the best case. The scale is not one number: the camera
is pitched, so it runs from about 55 m/px where you are looking to over 400 m/px
near the horizon, and it also depends on the screen size.

The CONCLUSION is unchanged and is why the correction did not reopen the
decision: at the most generous reading on the largest screen, a 20 m tree is
about a third of a pixel. Scatter still cannot live on this map.
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

## What actually limits zooming further (2026-09-07)

Ridge asked to raise the map's resolution and zoom much further. Measured rather
than assumed, and the answer is not the zoom cap:

| source | resolution | what it feeds |
| --- | --- | --- |
| ETOPO terrain tiles | **1,855 m**/sample | the raised 3D mesh at zoom >= 12 |
| `height-detail.png` 4800x2036 | 8,349 m/px | shading and the land ramp |
| `relief/terrain/coast/lake.png` 2400x1018 | 16,698 m/px | the globe surface |
| `cover.png` 1200x509 | 33,396 m/px | vegetation tint |

**THE 55 m/px IN THIS DOCUMENT IS ITSELF WRONG, and it is wrong in the
dangerous direction.** Corrected 2026-09-07 after three independent
re-measurements agreed against it. 55 m/px is `mx / uPxPerWorld`, and
`pixelsPerWorld()` divides pixels per radian of ARC by canvas units per radian
of LONGITUDE — two different radians — so it is 1/cos(latitude) too large and
the metres per pixel it implies are that much too small. The true figures at
screen centre, at latitude 41.9 on an 1878x889 pane:

| zoom | cross-track (E-W) | along-track (N-S) | the shader's own gate variable |
| --- | --- | --- | --- |
| 192 | 70.2 m/px | 122.7 m/px | 114.6 m/px |
| 512 | 26.3 m/px | 45.9 m/px | 43.0 m/px |
| 1500 | 9.0 m/px | 15.7 m/px | 14.7 m/px |

Three numbers, not one, and the difference between them matters. The gate
variable is what the ground shader actually tests, `max(fwidth(world.x),
fwidth(world.y)) * mx`, and it is 1.633x the cross-track figure because the
55-degree pitch stretches the along-track footprint. A document written to
correct a scale error understated the scale by 27% for a year, in the direction
of "the map is finer than it is", which is exactly the direction that lets a
feature be specified too small to see. The live page now measures it correctly:
`groundMetresPerPixel()` in index.html reads 26.3 at zoom 512, and
`drawCityLayer`'s per-city version carried the same cos(latitude) defect until
the same pass fixed it.

`ZOOM_MAX` was 192 when this was written, which puts the camera 81 km up. The
best data under it is 1,855 m per sample, so at 70 m/px the map was ALREADY
magnifying its finest source about 27 times. Raising the cap was tried: at zoom
768 (35 m/px, 76 km across) the surface is a featureless olive field, and the
terrain mesh has about eight elevation samples across the whole screen.

**The cap has since moved twice, and neither move contradicts the table above —
both were paid for by generating what the data cannot supply.** 192 -> 512 when
the procedural cover layer gave the close range parcels and woodland gated on
metres per pixel, and 512 -> 1500 when the relief stopped being a constant. The
SHAPE of the land is still 1,855 m per sample at every one of those zooms; what
improves is the cover and, since 2026-09-07, the city masses drawn on top of it.
The legend says which parts are generated, and that disclosure is what makes the
range honest rather than the resolution.

AND THE SHADER STOPS ADDING DETAIL AT ZOOM 32. Every detail ramp in the fragment
shader is a smoothstep on `uLk = log2(zoom)`, and the last of them, `tMicro`,
completes at `uLk` 4.4 — zoom 21. Measured in the running game: `tDetail` and
`tMicro` are both 1.000 at zoom 32, 64, 192, 768 and 4096, and the exaggeration
term `Z` has been at its floor since zoom 16. So the whole range from 32 to the
current cap of 192 — six-fold — is pure magnification, which is why the ground
looks softer the closer you get.

TWO REPAIRS WERE TRIED AND BOTH REVERTED, because neither could be shown to
work. Enabling the micro-detail term on the raised-mesh path (it is gated
`uMesh == 0`, undocumented) changed nothing visible; nor did extending the
octave ladder so it keeps climbing past `uLk` 4.4. The likely reason is that the
micro term is a MODULATION of existing slope — `sigma = 0.72 * slope * ...` — so
on ground that is genuinely gentle it has nothing to multiply. Neither change
shipped: a graphics change that cannot be demonstrated is not an improvement.

So zooming much further needs one of two things, and both are decisions rather
than work:

1. **Higher-resolution elevation.** ETOPO 2022 also ships at 15 arc-seconds
   (~460 m, 4x finer) and SRTM at 3 arc-seconds (~90 m, 20x). The pipeline in
   `tools/terrain/` already reads a staged `etopo_60s.nc`, which is NOT committed
   — the generators are one-shot and their inputs are staging only. So this is a
   download and a re-bake, not new code.
2. **Procedural land cover.** What makes real aerial imagery look sharp at 35 m/px
   is not relief, it is fields, woodland, roads and settlement. That would be
   invented surface, on the same footing as this repo's other declared models,
   and it is the option that would also finally give the scatter, road and prop
   kits somewhere to live.
