# S22 preparation — art accounting and equipment shadows

**Bounded preparation complete**, qualified at `2c959cd7c4f61d09a77d5859b1cbee24aa509b10`.
**S22 remains planned; S20, full performance qualification, G4 and CP1 remain open.**
Claude still owns S19. No navigation or guidance files changed in this packet.
[Exact-source evidence and hashes](manifest.json).

The old audit stopped at the new CPU `materialClasses` array. The repaired tool
validates that layout, measures actual attribute lengths, counts CPU backing
buffers separately and sums payloads without assuming every attribute is uploaded.
Unexpected layouts still fail. The inventory now includes the fighter and all
three detail levels for nine ground vehicles and three CP1 aircraft. Source-weight
records use canonical LF text and reproduce on Windows and Linux.

Chrome exposed a real renderer defect: `packed` is reserved in the shader language.
The self-shadow receiver failed compilation, and the renderer silently used its
projected fallback. Renaming that local variable restores actual shadow-map
rendering. Geometry, game statistics, model budgets and campaign saves are unchanged.

| Standard baseline inspection | Before repair | After repair |
| --- | ---: | ---: |
| Tank triangles submitted during one orbit redraw | 137,746 | 68,874 |
| Fighter triangles submitted during one orbit redraw | 400,894 | 213,248 |
| Tank resident buffer payload | 10,744,248 bytes | 10,744,248 bytes |
| Fighter resident buffer payload | 21,648,384 bytes | 21,648,384 bytes |

The first shadow pass draws separately; later orbit redraws reuse it. These are
draw-submission and buffer-payload observations, **not FPS or total GPU memory**.
Shadow textures/renderbuffers are outside the buffer totals. Before observations
are development diagnostics, retained separately from the clean-commit final run.

## Checked

- 39 focused accounting, renderer and ownership regressions pass on both Windows
  and Linux. Generated manifest (33 assets) and measurement records reproduce on both.
- Chrome 153 / RTX 5070: all 12 platforms, LOD0 → LOD1 → LOD2 → LOD0, orbit,
  paint/selection, disposal and a fresh visit. Tank/fighter context loss and
  restoration rebuild the real shadow pass. A 390px fighter capture is retained.
- Final browser assertions require actual offscreen shadow submissions and no
  shader compiler diagnostics, so a working fallback cannot pass this check.
- The release executable builds; its served renderer bytes match the tested
  source. The actual equipment room renders self-shadows with no page errors,
  military orders or day advances in the native smoke check.
- Eight original save files and both protected worktrees pass preservation checks.

The pinned release is available locally at <http://127.0.0.1:7862/> with a separate
copy of the France campaign on 16 October 1992. Prior review runtimes remain intact.
The native smoke check opens the existing equipment entry function; it is not a
manual first-hour usability test.

## Open work

The original roadmap budget gate is **still failing**: 42 of 112 graded
configurations exceed its original ceilings. This includes later high-detail art;
limits were not widened and models were not reduced to hide the failures.
`bench_art.cjs --check-records` checks freshness only. `bench_art.cjs --check`
still exits 1, correctly identifying the overruns. All are listed in
[P0_BUDGETS.md](../../../art/P0_BUDGETS.md) and P0_MEASUREMENTS.json.

After S19 integration and S20, S22 must measure the agreed campaign workloads,
FPS/input response, cold loading, tick throughput, memory, city/inspection caches
and low-detail profile at early/mid/late dates. This repair supplies trustworthy
tools and fixes a reproduced defect; it does not replace that qualification.
