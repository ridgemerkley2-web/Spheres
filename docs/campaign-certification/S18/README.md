# S18 — live flight pages and CP1 aircraft inspection

Status: **complete** on `902b820c0bea0ed9bcd32a6085fc63dad6d92715`. [Exact qualification and retained evidence](manifest.json). G4 and CP1 remain open.

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

Qualification passed on Windows and Linux: **398 native web tests** (20 ignored) and **1596 Node tests** (1 skipped) per platform. Asset regeneration and the native fixture export also passed.

Runtime/native/browser/art evidence is pinned to `902b820c0bea0ed9bcd32a6085fc63dad6d92715`. The final Node runs use
`1b7ac3eb6e872f065efbcb1c5e80f12180be898b`; its only change corrects the existing inlet-depth test's
sampling coordinate to the new duct position. The manifest verifies that exact
one-line test-only difference. Failed initial runs remain in the evidence archive.

The ordinary browser route checked all four campaign pages at 1440px and 390px,
downloaded actual owned specifications, retained the local draft, then reviewed two
staff commands and advanced six ordinary days. **Ten exact native-world and ten
history-envelope comparisons** passed through inspection, commands, Save, Load and
Continue, with zero browser errors. No world fields were ignored; only the separately
validated save timestamp can differ in an envelope. Authored starting forces are
disclosed; this is not an unassisted long-campaign or human-usability claim.

Thirteen final art captures plus the live-page/campaign captures were inspected.
The art browser exercised all three families, eight selectable slots, cockpit,
engine and inlet cameras, LOD switching and real GLB downloads. Original saves and
the two protected worktree heads are unchanged. No simulation code or dependency
changes were made; S17 retains its separately pinned simulation/integration evidence.
Follow-up polish for S20: normalize signed-zero display values and replace raw
province/ammunition identifiers in inherited mission report prose with friendly names.
The restore toast is transient and is visible in the immediately captured page shots.

[Open the separately copied S18 review campaign](http://127.0.0.1:7860). The older S17
review remains available. S19 is assigned to Claude in the shared workboard; its
handoff is now dependency-ready. Codex's S20 shared-navigation work waits for that handoff.

Original artwork is procedural game concept art, not a historic aircraft replica.
The 100k+ target applies to inspection; map/detail performance qualification remains
S22. Full historical coverage, long campaigns, independent human playtests and CP1
remain later gates. The [AI workboard](../../AI_WORKSTREAMS.md) assigns S19 guidance
and bounded character research to Claude without granting those markers completion.
