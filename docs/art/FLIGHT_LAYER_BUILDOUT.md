# Flight layer implementation record

## First milestone — 8 September 2026

The approved direction is arcade strategy with detailed aircraft. This milestone
adds the command-room interaction prototype and upgrades the two aircraft already
supported by the equipment designer. It does not implement a new combat resolver,
campaign squadrons, airbases, readiness spending, or new fighter/support platforms.
The roadmap remains approved; those phases are outstanding.

Open `/tools/arsenal/flight-command.html` on the existing static workshop server.
Its fictional fleet has six visible mission choices, compatibility explanations,
area selection, cautious/balanced/intense commitment, review/cancel, an unreachable
area guard, a preview assignment and bounded report history. Four primary screens
keep specialist controls out of the initial flow. No campaign API is called.

The aircraft bench offers eight real visual component slots, the two existing
platforms, semantic assembly selection, orbit/zoom and GLB export. Unsupported
heavy components are omitted for the light aircraft. Models load only when the
Aircraft screen is opened. All displayed fleet counts are labeled demonstration
data; they must be replaced by authoritative server projections during integration.

## Mesh contract

| Base aircraft | Inspection triangles | Catalogue | Map | Inspection GLB bytes |
|---|---:|---:|---:|---:|
| Light attack | 197,632 | 10,752 | 1,272 | 21,348,060 |
| Tactical strike | 228,640 | 14,904 | 1,696 | 24,698,664 |

Inspection uses denser sampling of the authored curved fuselage, wing roots,
intake ducts, canopy and wing profiles. No duplicated or degenerate triangles
are added to meet a count. Recessed heat-shield rings and nozzle actuator links
add physical engine detail. Semantic parts and configuration metadata survive LOD
changes and export. Aircraft keep original concept proportions, not a claim of
historical or engineering fidelity. Existing local material lighting is reused.

The 100k floor applies to each assembled inspection aircraft, not individual
parts or map icons. New aircraft families must meet the same contract. This pass
does not claim that every arsenal model or ground vehicle has been upgraded.

## Verification

- Native web suite: 274 passed, 3 existing ignored.
- Equipment mesh suite: 32 checks passed, including winding, normalized normals,
  deterministic geometry, eight slots and every current aviation component.
- Additional aircraft LOD checks cover base/loaded configurations, semantic part
  continuity, buffer integrity, memory budgets and invalid LOD requests.
- Full UI run: 1,302 of 1,305 checks passed initially. Three existing aircraft
  snapshot/old-budget checks correctly detected this intentional mesh rebuild.
  Aircraft-only snapshots and the explicit art budgets were updated to the new
  requirement; all 26 affected-suite/new-LOD checks then passed. Ground pins,
  winding checks and performance budgets were preserved. GLB byte-for-byte
  geometry/metadata checks passed after regenerating both aircraft exports.
- Live in-app browser: both inspection meshes render; tactical map LOD displays
  1,620 triangles; extended light-aircraft fuel changes the model to 201,848.
  Unreachable review is disabled, cancel preserves standby, reachable support
  assignment updates the demonstration fleet.
- Responsive DOM check at 390px: document width 375px, canvas width 341px;
  no horizontal page overflow on the Aircraft screen.
- Standalone Playwright runner is supplied but could not launch local Chromium
  (`spawn UNKNOWN`); do not report that runner as passing. Browser verification
  uses the existing in-app browser instead.

No native server was launched or existing campaign modified in this milestone.
Campaign deployment requires a rebuilt web executable because its assets are
embedded; the static workshop reads these updated files directly.

## Tactical aircraft rebuild — 9 September 2026

The tactical strike family now uses an original shape study informed by the
Hornet and F-16 previews recorded in `AIRCRAFT_REFERENCE_REVIEW.md`. No external
mesh or texture was imported. The light attack family retains its previous
geometry, colors and all three LODs byte for byte.

The body has broader wing-root shoulders, a flatter upper profile, narrower
wingtips, thinner tailplanes, and separate inboard flaps/outboard ailerons with
open hinge gaps. Lower, rounded rectangular intakes expose recessed ducts and
fans; a dark end plate closes the peripheral corners. Extended fuel tanks mount
under the outer wings, with pylons clear of the other external stores.

The canopy now covers an opening in the fuselage, two modeled seats with
headrests and harnesses, instrument panels and control sticks. Digital avionics
add illuminated displays alongside the existing targeting pod. The clear shell
uses a separate glass range; frames and interior remain opaque. Paint is a
restrained gray with attached skin seams, access caps and dorsal fittings.

The viewer draws opaque parts before glass, preserves selection highlighting,
and excludes the glazing from opaque shadow maps. Exported glTF uses separate
opaque/glass primitives with alpha materials while preserving the original
global vertex buffers and semantic part ranges. This is vertex-colored art with
procedural surface shading, not a completed UV unwrap or hand-painted texture
set. Distinct liveries and further aircraft families remain future art work.

The workshop no longer applies an automatic zoom on first mount, allowing the
whole aircraft to fit in Top view. User-selected zoom still persists between
views. All eight configurable slots and three detail levels remain available.

The GLB reader now accepts the explicitly marked Spheres aircraft dialect,
validating material opacity, each primitive's slice of the global arrays, bounds
and glass ownership before restoring the model. Generic unsupported glTF
features continue to be rejected. Importer checks passed 128/128, including
inspection/catalogue/map round trips and malformed surface metadata.

Browser review covered perspective, side and top framing, the default tactical
and light inspection builds, tactical map detail, efficient engines, digital
avionics, extended fuel, assembly selection and the download control. The default
map build displayed 1,696 triangles; digital avionics plus extended fuel displayed
217,020 at inspection detail. No browser console errors occurred. Automatic
first-load zoom was removed and its script version updated to avoid stale cached
preview behavior.

Validation for this rebuild:

- Native web tests: 274 passed, 3 existing ignored.
- Full UI regression: 1,345/1,358 passed on the final broad run. The 13 failures
  shared a test-fixture extraction boundary: a newly inserted camera test was
  included when the ground workshop suites extracted their common fixture. The
  camera test was moved outside that boundary; all 44 checks in the affected
  viewer/armored-workshop/tank-workshop suites then passed. No runtime behavior
  changed for this test repair.
- Geometry regressions: 28 passed, including actual-triangle intake, cockpit and
  control-gap probes and exact ground/light-aircraft snapshot preservation.
- Importer: 128 passed, including transparent GLB round trips and malformed
  dialect rejection. Surface/render-state tests passed in the broad run.
- Deterministic model generation with `--check` passed for all twelve GLBs.

Logs are kept outside the repository in the parent work directory under
`aircraft-rebuild-*`. The standalone Chromium runner remains unavailable as
documented above; the browser observations use the existing in-app browser.

## Engine and cockpit detail — 9 September 2026

The follow-up pass spends additional geometry on visible internals in the
tactical aircraft. The intake fans now have closed swept blades with radial
curvature, tapered spinners and containment collars. Removing the hidden rear
engine-casing cap exposes a deeper exhaust liner, recessed turbine face and
central cone. Separate converging nozzle petals, actuator links and liner ribs
replace the previous shallow cylindrical outlet. These remain fictional game
visuals; no real engine performance or internal engineering is represented.

Both crew stations gain contoured seat pads, upholstered headrests, flat harness
webbing, buckles, side consoles, throttle controls, shaped control grips and
rudder pedals. Analog avionics have round gauges with ticks and needles; digital
avionics have graphical horizon, radar and status displays. Canopy seals and
fastenings complete the interior without altering the clear shell or headroom.

The default tactical model now has 228,640 inspection triangles and 14,904 at
catalogue detail. A loaded twin-engine/stability-wing configuration has 237,140
and 16,828 respectively. Its existing map mesh is unchanged; the entire light
aircraft family also retains its previous geometry. All eight component slots
and the original semantic assembly names remain intact.

The flight workshop adds dedicated Cockpit, Engines and Intakes camera controls.
They frame actual relevant mesh regions; orbit and zoom use that region as the
center. Perspective/Side/Top and Reset return to the whole aircraft. Close-up
mode does not change the generated geometry or exported model.

Validation for the interior pass:

- Full UI suite: 1,372 passed, zero failures. This includes the earlier viewer
  fixture repair, seven new physical interior regressions and seven close-up
  controller regressions.
- The physical probes verify open exhaust depth, closed fan blades at both
  close detail levels, tapered nozzle petals, throttle visibility beneath glass,
  unchanged light/map geometry and the existing triangle budgets. They caught
  thin fan vanes being culled at catalogue detail; preserving those vanes with
  fewer radial sections fixed visibility within the budget.
- All twelve generated GLBs pass the deterministic `--check`; component
  coverage and the art manifest were regenerated.
- Existing in-app browser: visually checked analog and digital cockpits, the
  outboard intake and both exhausts. Cockpit framing looks toward the instrument
  faces; intake framing clears the fuselage. Restored the selected tactical
  loadout and left its digital cockpit open. No browser errors occurred.

Current verification logs are in the parent work directory under
`aircraft-interior-*`. No native campaign integration changes were needed for
this geometry and inspection-control pass.
