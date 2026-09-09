# Military inspection mesh detail

The procedural source remains `spheres-web/ui/equipment-mesh.js`. These are original configurable visual concepts, not replicas or engineering specifications for actual vehicles. No simulation rating, equipment identity, upgrade prerequisite or production value is changed by this art.

## What changed

Tracked inspection vehicles now use physically separated paired road tyres with an open central channel for the guide horns. The previous full-width rubber surface crossed that channel. The separated tyres retain the original outer wheel locations; pressed inboard annuli and a central bearing shaft connect the assembly. Tank horns now taper from a broad seated foot through a shoulder to a narrow tip. The tracked IFV, artillery and air-defense families gain those horns and connected pin-and-plate track end connectors. The connectors meet the existing shoes along the belt; they are not isolated decorative cylinders.

The shared inspection fittings draw hinges with leaves and alternating knuckles, small mounted latches, narrow panel joints and shallow weld beads. They sit in local surface frames on the hull, roof, storage lids, engine access plates and troop doors. Suspension pivots gain bearing housings and fasteners; the howitzer recoil assembly gains its cylinder housing, brackets and short connected line. Cooling-bank crossbars join their existing louvres and frames. Tank thermal shrouds now have a longitudinal split and clasp seats rather than only circumferential bands.

Hydropneumatic tank suspensions also have a capped pressure reservoir, mount collars and a short connected hydraulic line at each strut. The additional paired-wheel detail had concealed the old thin-piston distinction: the unchanged component coverage measurement initially reported four weak tank variants. The reservoir assembly restores the geometric difference without new parts or changed vehicle extent. Measured outline changes are 0.258 m² (standard), 0.304 m² (heavy), 0.180 m² (light) and 0.308 m² (destroyer), all above the existing 0.06 m² criterion. The component coverage thresholds are unchanged.

Both tactical aircraft gain curved, span-varying wing sections with thinner tips and sharp trailing edges. Their fuselage, radome and canopy use shared normals derived from their elliptical sections and taper, improving the continuous surface shading without subdivision. Added geometry includes seated canopy seals and hinge hardware, waist service doors, flap actuators and hinge knuckles, landing-gear torque links, brake fittings and hoses, and overlapping exhaust petals mounted on the existing nozzle lip. The aircraft remain parked original concepts, and selected engines, wings, payloads and sensors retain their existing visual distinctions.

The additional small hardware is inspection-only on ground vehicles. Existing part names, labels, slots, order, meter units and ground contact are preserved. No new picking IDs are created. `tank_light` retains its existing uniform 0.8 scale. Ground LOD1/LOD2 positions, normals and colors are byte-for-byte identical to the pre-pass buffers for all nine platforms. Aircraft keep their existing inspection-only behavior; this pass does not invent an aircraft LOD pipeline.

## Measured geometry

These are default `build({platform})` readings, before any exporter indexing or compression. Fully selected tank designs can differ from the legacy default components.

| Platform | Previous inspection triangles | Detailed inspection triangles | Unchanged LOD1 / LOD2 |
| --- | ---: | ---: | ---: |
| Standard tank | 47,288 | 69,380 | 9,425 / 1,164 |
| Heavy tank | 50,816 | 74,628 | 9,010 / 1,200 |
| Light tank | 47,288 | 69,380 | 9,425 / 1,164 |
| Tank destroyer | 47,288 | 69,380 | 9,425 / 1,164 |
| IFV | 20,746 | 43,334 | 5,828 / 1,120 |
| APC | 20,710 | 27,134 | 4,675 / 942 |
| Reconnaissance vehicle | 18,418 | 24,102 | 4,984 / 932 |
| Artillery | 20,822 | 45,042 | 6,050 / 1,092 |
| Air defense | 18,950 | 40,438 | 5,590 / 1,040 |
| Light attack aircraft | 11,524 | 17,576 | Inspection geometry retained |
| Tactical strike aircraft | 14,724 | 21,328 | Inspection geometry retained |

A sweep of the 283 specialist platform/option pairs parsed by the existing mesh contract found 45,826 triangles at most (artillery with hydropneumatic suspension). The existing fully loaded specialist fixtures reached 46,106 triangles for artillery; other loaded readings were IFV 44,238, APC 32,870, reconnaissance 29,898 and air defense 40,618. This is the tested option sweep and loaded fixtures, not a claim to exhaust every possible combination.

The revised inspection art budget is separate from correctness and coarse performance. A 48,000-triangle specialist inspection band gives approximately 4.1% headroom above the measured loaded maximum. Shipped default specialist GLBs retain a 5 MB file budget, and tank GLBs a 12 MB file budget. In the current non-indexed export format, each triangle uses 108 attribute bytes; default artillery therefore needs approximately 4.87 MB including a 32 KB JSON allowance. Custom selected designs may exceed the default reference asset size. Exported binaries and their manifest are regenerated and verified separately from this source change.

## Verification and remaining review

`node --test tools/ui/check_equipment_mesh_detail.cjs` checks actual tyre clearance, horn taper, airfoil cross-section volume, curved aircraft normals, finite nondegenerate triangles and normal winding for all eleven families, immutable deterministic specifications, part continuity and meter scale. It also pins all 18 pre-pass ground coarse buffer hashes, protecting map/card rendering cost.

All ten detail tests passed. As a negative control, the five geometry-specific assertions were executed against the pre-detail mesh source and all five rejected it: tank wheel clearance, specialist wheel clearance, horn taper, airfoil volume and curved aircraft surface normals. A separate hydropneumatic coverage regression was verified failing before the reservoir correction and passing afterward against the actual unchanged coverage measurement. The nineteen existing viewer tests also passed, including actual triangle picking and camera fitting. All eighteen tactical-strike component options plus the light-attack baseline, and all four corrected hydropneumatic tank configurations, passed the existing full normal/smoothing contract without changing its thresholds.

`node tools/ui/check_equipment_mesh.cjs` remains the broader component, smoothing, winding and option-space contract; `node --test tools/ui/check_equipment_model.cjs` checks the viewer integration. Shipping export/import checks and visual review are separate required integration steps. Triangle counts establish neither visual quality nor historical authenticity by themselves, and the mesh source makes no claim that added physical detail is a new gameplay capability.
