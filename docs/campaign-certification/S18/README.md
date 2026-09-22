# S18 — live flight pages and CP1 aircraft inspection

Status: implementation in qualification. S18 is not complete until the retained
final evidence passes. The roadmap remains in progress.

Air command now separates Command, Aircraft, Bases and Reports over the existing
native flight read model and reviewed command channel. The Aircraft inspection
selects an exact frozen owned revision; no model choice changes a local draft,
buys equipment or alters the campaign. Cockpit/engine/intake controls are available
in the campaign viewer. The former standalone flight demonstration is an art-only
workshop, with no invented fleet or preview mission assignments.

Light attack and fighter now share the detailed seat, harness, instrument, control,
fan, liner and nozzle techniques of the tactical aircraft, with single-seat
canopies and their own airframe proportions. Their fuselages have real cockpit
openings; intakes and exhausts clear the surrounding geometry. All three have
transparent glass, eight selectable component slots, lower-detail meshes and
portable GLB exports. A fighter GLB is added to the deterministic asset build.

| Default family | Inspection triangles | Catalogue | Map |
|---|---:|---:|---:|
| Light attack | 199,326 | 11,474 | 1,252 |
| Fighter | 200,446 | 11,594 | 1,296 |
| Tactical strike | 228,640 | 14,904 | 1,696 |

Meaningful geometry tests probe visible seats beneath canopy glass, actual inlet
recess depth, surface normals, component ownership, camera occlusion and export
preservation. Browser checks exercise exterior/cockpit/engine/intake views, every
slot, lower LODs and actual GLB downloads. Prior ground-vehicle geometry pins remain
unchanged; aircraft pins explicitly move to the reviewed S18 geometry.

Planned qualification: full Windows/Linux web and Node checks, exact asset
regeneration, the S17 native staff fixture with the new four-page browser route,
and exact saved-world/history comparisons before and after read-only inspection,
ordinary reviewed commands, day advances, save/load and Continue. No simulation
rules change in S18; S17 simulation/integration evidence remains separately pinned.

Original artwork is procedural game concept art, not a historic aircraft replica.
The 100k+ target applies to inspection; map/detail performance qualification remains
S22. Full historical coverage, long campaigns, independent human playtests and CP1
remain later gates. The [AI workboard](../../AI_WORKSTREAMS.md) assigns S19 guidance
and bounded character research to Claude without granting those markers completion.
