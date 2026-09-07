# Section 4's triangle ceilings are stale — a decision for Ridge, not for me

`node tools/ui/bench_art.cjs --check` exits 1 with **30 configurations over
budget**. I have not touched the ceilings, and I am not going to: the harness
explicitly forbids widening a budget to empty its own table, and §4 is your
design document. This file is the evidence, laid out so the call takes one read.

## Why they are over

Every one of the 30 is a deliberate fidelity increase, made on your instruction
*"I sent the 3D models to be way more detailed with realistic mesh"*. The
ceilings predate that request. Nothing drifted; the target moved and the
document did not.

## What is over, and by how much

| group | rows | ceiling | measured | over by |
| --- | --- | --- | --- | --- |
| ground platforms | 8 | 45,000 | 47,288 – 51,304 | 5% – 14% |
| construction sites (near, complete, L5) | 13 | 12,000 | 30,164 – 39,302 | 151% – 227% |
| town blocks (close) | 4 | 150,000 | 151,008 – 214,044 | 0.7% – 43% |
| town kit buildings (close, max size) | 5 | 12,000 | 12,028 – 24,252 | 0.2% – 102% |

The sites are the striking row and the reason is documented: that 12,000 was set
when twelve of the thirteen kinds were placeholder massing on a shared stage
kit. They have since had their art pass — the placeholder count went 12/13 to
0/13 — so the ceiling is measuring something that no longer exists.

## What it actually costs, measured

- Everything resident at close detail: **1,602,304 triangles, 165.03 MiB**. That
  is a worst case the game never reaches, and since the model cache is now
  bounded at 1.2M triangles (~124 MiB) it *cannot* reach it.
- Generator source, which is what ships down the wire: **576.1 KiB, 160.3 KiB
  gzipped**. Triangle count costs nothing here — the meshes are recipes baked by
  the client, not payloads.
- Build time, the cost a player actually feels, is per-first-paint:
  `TownMesh.block residential` 119.6 ms cold / 53.5 ms warm;
  `SiteMesh.build arms_plant complete L5` 33.4 ms cold / 15.0 ms warm;
  `EquipmentMesh.build tank_heavy` 33.3 ms cold / 21.9 ms warm.
  The far LODs are free by comparison — a map-LOD site is 0.09 ms.
- Frame cost is not the problem and never was: the designer draws 94,576
  triangles a frame, which is nothing for a GPU.

**The town block is the one number I would not wave through.** 214,044 triangles
is 23 MiB for a single city-card vignette, and it is the heaviest thing the game
builds by a factor of five. Its 119.6 ms cold build is also the slowest
first-paint in the library. If any ceiling deserves to hold rather than move,
it is that one.

## I tried the obvious fix on the town block. It does not work, and here is why

Before putting this to you I attempted the trim myself, because if it were cheap
the decision would only be about the other 26 rows.

Buildings are 81% of a close block (174,088 of 214,044 for residential; the rest
is streetscape and ground). And they are drawn absurdly small: the block is 148 m
across, so on the ~300 px card the city view gives it, a 9 m house is about 18
pixels wide and arrives carrying 6,299 triangles — roughly **350 triangles per
pixel it can occupy**. Stamping lots one detail tier down took the worst block
from 214,044 to 76,278, under the ceiling, with the map LOD and the standalone
`building()` card both untouched.

Then `check_town_mesh.cjs` refused it, correctly. The block ships three assembled
levels and asserts they stay separated — `close > mid*2 && mid > map*4`. The
block ladder is BUILT ON the building ladder, and there are only three building
tiers, so stepping close down to `mid` collapses the middle:

| lot tiers used | worst close block | separation |
| --- | --- | --- |
| close / mid / map (today) | 214,044 | holds |
| mid / mid / map | 76,278 | fails — 71,164 / 57,784 / 3,422 |
| mid / map / map | 76,278 | fails — 71,164 / 7,102 / 3,422 |

So the trim is available, but only at the price of a fourth building tier or of
making close buildings cheaper everywhere — including on their own card, where
they really are the subject and really do fill the frame. Both are art decisions
with a wider blast radius than a budget line, so I reverted to 214,044 and left
it with you. The measurement is the useful part: the waste is real and large,
and the fix is a tier of detail this kit does not yet have.

## Section 4 does not grade 43% of the art, and that reframes the question

Added after a verification pass on 2026-09-07. `bench_art.cjs` grades 79
configurations and reports 30 over. It does not grade the road, scatter or prop
kits at all — 78 pieces with no row anywhere. I set out to give them bands and
an adversarial review killed the proposal, correctly, for reasons worth keeping:

- `P0_BUDGETS.md` is GENERATED and says so; its ceilings live in the `BUDGETS`
  table in `bench_art.cjs`, where every entry quotes a section 4 row and cell
  **verbatim** and `verifyBudgets()` exits 1 if that cell ever moves. Five of the
  seven bands I proposed had no cell to quote.
- The scatter bands are already pinned by `assert.deepEqual` in
  `check_scatter_mesh.cjs`, added 2026-09-06 after a pass measured that widening
  the dressing band to [1, 900000] left the whole suite green. My numbers
  contradicted those pins without acknowledging them.
- The rate I derived them from — 250 triangles per metre of footprint radius —
  was a one-point fit. `radius` is a per-FAMILY constant: all eight trees carry
  3.2 while their triangle counts run 206 to 699, so it explains none of the
  variance in the family it was calibrated from.

So no bands were added. What the pass DID establish is more useful than bands:

| | cards | verdict |
| --- | --- | --- |
| graded by a section 4 row | 123 | **25 over a ceiling**, 10 under a floor |
| no section 4 row exists | 94 | the deck (46), roads (13), props (35) |

**Section 4 has six rows; the library now has seven kits.** Nearly half the
shipped art is ungraded, and forcing the existing rows onto it produces category
errors in both directions — its Tree/prop row grades a 218 m container ship as a
tree, and fails a 54-triangle grass tuft for being cheaper than a tree FLOOR.
The ten "under floor" readings above are exactly that, and they are not a cost
problem.

That makes the real question larger than the 30 rows: **does section 4's table
still describe this library?** It was written for vehicles, buildings and a
generic tree/prop. It has no row for a road network, a scenery vehicle or a
vessel, and one row spanning an oak and a grass tuft cannot bind either. A
fourth option therefore exists:

4. **Re-cut section 4 itself** — keep the vehicle and building rows, split
   Tree/prop into plant and scenery-object rows, and add rows for the road and
   vessel kits. Then re-derive the ceilings once, against a table that fits.
   More work than options 1-3 and it is the only one that ends with every
   shipped asset graded by something.

The review bench now reports all of this honestly. It previously could not: four
of its nine bands were LOOSER than the section 4 rows they claimed to grade
against (vehicles 150,000 against 45,000; sites and stages 40,000 against
12,000; town 400,000 against 150,000), so over-budget art displayed as passing,
and every town card printed a triangle count for a block it was not drawing.
`tools/ui/check_art_gallery.cjs` now holds it to section 4 by re-reading the
roadmap.
## The options

1. **Revise the ceilings to match the art.** Sites to 40,000, platforms to
   52,000, town kit to 25,000. Honest about what was built, and the measurements
   above say nothing breaks. Leaves the town-block ceiling where it is, which
   would keep 4 rows red until the blocks are trimmed.
2. **Trim the art back to the ceilings.** This undoes the fidelity work you
   asked for, most of it on the sites. I would not recommend it.
3. **Split the difference** — revise sites and platforms, hold the line on town
   blocks and trim those four, since that is where the real cost sits.

I would take option 3. But it is a design call about what this game is willing
to spend, and that is yours; tell me which and I will make the change in one
pass, including re-running the bench and the manifest.

## What I did not do

I did not change a single ceiling, and I did not quietly exclude any
configuration from the check to make it green. The 30 rows are all still there
and `--check` still exits 1.
