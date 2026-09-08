# Gameplay, political systems and art integration — 7 September 2026

This release combines the local equipment and supply work committed at
`cdfc6c5` with both recent Claude branch tips:

- `feat/hoi4-map-and-tech`: `fc0f0c299149bd4859fcc4653f4e56ae28dda5ba`
- `feat/art-p0`: `c2e49c6b0492f8a0fe45c988a644e8487c0c206c`

Both remote tips were fetched again and remained unchanged before publication.
The integration is published on `codex/resume-spheres` and retains both branch
histories. Claude's source branches are not rewritten.

## Compatibility decisions

- Daily construction keeps financial funding, no starting material bill and no
  fixed twelve-project queue cap. The older monthly simulation retains its
  legacy capacity rules. API fields distinguish those modes explicitly.
- Equipment state version 8, physical ammunition, maintenance, fleet targets,
  supply purchasing and the two tactical aircraft remain intact. Detailed ground
  meshes, surface treatment and levels of detail are integrated alongside them.
- All twelve equipment GLBs are regenerated from the shared mesh code. The asset
  manifest includes eleven equipment platforms, thirteen sites and five blocks.
- Map construction sprites explicitly request map detail in the actual site
  provider and use separate cache keys from close card previews.
- Government and covert actions use the game's review dialog, with campaign,
  target and pending-command guards. Navigation and session reset include the
  government panel.
- Selected cities show a representative town block, labeled as such. The globe
  scatter overlay withdrawn by Claude's final commit remains withdrawn.
- Company development, company-owned stock and government purchases remain
  planned work in [the procurement plan](../COMPANIES_AND_PROCUREMENT_PLAN.md).

## Verification

| Check | Result |
| --- | --- |
| Release simulation and CLI tests | 903 passed, 64 existing ignored |
| Release web tests | 240 passed, 3 existing ignored |
| Node UI and art checks | 930 passed |
| Separate mesh validation runner | 32 checks passed |
| Equipment model generation | All twelve files current |
| Component coverage | 186 distinct pairs; zero weak or absent pairs |
| Release web build | Passed |

The bloc inertness test contained six stale golden hashes from before the
approved E-3 productivity reference repair. Before updating those exact pins,
an isolated build of `cdfc6c5` and this integration were run for 240 months with
seeds 0–5 and 1990. All seven complete pretty-serialized worlds matched
byte-for-byte. Only the expected hashes and provenance comments changed; the
strict equality assertions and political on/off checks remain.

An isolated browser session loaded the existing synthetic France aviation save,
rendered the detailed tank and aircraft designers, reviewed and cancelled a
government action, and displayed Paris's representative city preview. A new
warehouse showed its provincial and national effects, queued with zero spending,
and displayed its construction site. With a $1m daily cap, one day spent $250k,
left $59.75m of the $60m contract, and reduced the 240-day estimate to 239 days.
No browser warnings or errors were recorded. This QA save was not promoted to
the user's running campaign.

## Existing art performance limits

[P0_BUDGETS.md](art/P0_BUDGETS.md) records 79 graded configurations: 47 pass,
two are below the detail floor, and 30 exceed geometry budgets. The same 30
overages already exist in Claude's incoming report; this integration introduces
none and does not raise the ceilings. Consequently, `bench_art.cjs --check`
still exits nonzero for those overages, with zero stale-report errors. This is
an outstanding art optimization task, not a passing performance gate. The
performance sweep covers ground configurations; twelve-file GLB inventory and
component validation also include aircraft. Browser frame-rate acceptance is
not asserted by these geometry measurements.
