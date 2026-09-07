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
