# Arsenal close-up mesh detail

Measured 8 September 2026. This pass changes physical geometry in all 46 existing catalogue recipes. Model IDs, class metadata, published spans, vertex-colour palette and exact near bounding boxes remain unchanged. Every far mesh is byte-identical to the pre-pass position, normal and colour buffers.

## Visible changes

- Armour and mechanised infantry: pins join adjacent track shoes; the tank mantlet receives retaining screws; the engine cover has perimeter/support rails. The AFV's six coincident vent blades are now spread across a horizontal bank. Existing sprockets, road wheels, skirt hinges and track pads are retained.
- Aircraft: exhaust openings now face aft within their original axial interval. The old detailed nozzle mouth pointed into the fuselage. Recessed flameholders and petal actuator rods are actual geometry. Conventional intakes have canted compressor blades; low-observable S-ducts and the scramjet do not gain exposed compressor discs. E-3 nacelle end caps no longer hide the four compressor faces. F-117 and B-2 glass gains structural frames; the Sentinel gains elevon hinge fittings; both Predator variants gain wheel sidewalls, oleo torque links and axle forks. The AESA rack gains coolant coupling collars.
- Ships: railing stays, watertight door dogs/handles, raised radar feed rows and recessed mounting fasteners improve deck and superstructure detail. Submarine and directed-energy assembly joints gain recessed retaining grooves.
- Missiles, ground electronics and spacecraft: existing fastener runs receive washer rims, hard hexagonal heads and inward-facing socket recesses; existing cylindrical joints receive real grooves and darker recessed gasket surfaces. Patriot canister bands gain clamp catches and screws. HGV control fairings gain clevis pins, while its thermal tiles remain plain.

These are procedural visual interpretations, not certified engineering drawings or component-accurate maintenance models. Existing silhouettes and equipment identity remain the priority. No textures, materials, renderer, gameplay, production logic or catalogue IDs are changed in this file.

## Geometry and memory budget

Near detail grew from **272,491 to 338,727 triangles** (+24.3%). The largest card is mechanised infantry at **15,760 triangles**. The approved per-card ceiling is **16,000**, up from 12,000 for visible close-up machinery; map meshes retain their existing 300–1,500 range. No blanket tessellation multiplier is applied.

All near position/normal/colour arrays total **34.89 MiB**; all far arrays add **4.07 MiB**. Building both complete decks therefore retains **38.96 MiB** of CPU attribute buffers. GPU uploads, transient smoothing allocations, renderer framebuffers and other assets are additional. The catalogue cache is still lazy and keyed by model and LOD.

| Model ID | Before near | Current near | Unchanged far |
| --- | ---: | ---: | ---: |
| `inf_light` | 8,118 | 8,530 | 1,224 |
| `inf_mech` | 11,108 | 15,760 | 632 |
| `arm_gen2` | 8,499 | 11,519 | 1,039 |
| `arm_gen3` | 9,407 | 12,427 | 1,081 |
| `trophy` | 10,147 | 13,167 | 1,153 |
| `air_gen2` | 4,796 | 5,580 | 544 |
| `air_gen3` | 5,300 | 6,868 | 820 |
| `air_gen4` | 6,124 | 7,692 | 840 |
| `f15e` | 6,632 | 8,200 | 1,004 |
| `f117` | 4,504 | 4,596 | 952 |
| `e3` | 6,040 | 9,288 | 1,004 |
| `b2` | 4,094 | 4,142 | 982 |
| `predator` | 4,036 | 4,252 | 470 |
| `mq1b` | 4,756 | 4,972 | 1,070 |
| `f22` | 4,528 | 5,308 | 1,220 |
| `ea18g` | 5,836 | 7,404 | 840 |
| `rq170` | 4,030 | 4,350 | 782 |
| `f35a` | 4,452 | 4,842 | 1,186 |
| `cca` | 4,284 | 4,674 | 750 |
| `sixthgen` | 4,036 | 4,816 | 1,056 |
| `aesa` | 5,148 | 5,820 | 708 |
| `nav_patrol` | 6,712 | 7,780 | 766 |
| `nav_escort` | 9,948 | 10,488 | 1,146 |
| `nav_blue` | 11,162 | 11,486 | 1,266 |
| `la_ssn` | 4,658 | 5,490 | 572 |
| `aip_ssk` | 4,346 | 5,178 | 572 |
| `laws` | 4,372 | 9,540 | 616 |
| `msl_sam` | 7,824 | 10,384 | 680 |
| `msl_brm` | 7,030 | 7,798 | 1,003 |
| `msl_deterrent` | 4,546 | 6,466 | 980 |
| `paveway` | 4,216 | 6,136 | 1,134 |
| `tomahawk` | 4,484 | 6,084 | 1,231 |
| `patriot` | 4,292 | 6,052 | 836 |
| `jdam` | 4,848 | 6,928 | 968 |
| `gbi` | 4,106 | 5,194 | 865 |
| `x51` | 4,448 | 5,168 | 759 |
| `hgv` | 4,102 | 4,518 | 386 |
| `owa` | 4,126 | 5,342 | 560 |
| `raven` | 4,680 | 5,464 | 460 |
| `switchblade` | 4,162 | 5,506 | 469 |
| `cuas` | 6,938 | 8,178 | 1,226 |
| `link16` | 4,132 | 6,544 | 364 |
| `c4isr` | 5,880 | 8,280 | 504 |
| `atr` | 4,674 | 6,402 | 558 |
| `spc_recon` | 11,032 | 12,432 | 1,354 |
| `kh11` | 9,898 | 11,682 | 876 |

## Validation

Run:

```text
node tools/ui/check_arsenal_models.cjs
node tools/ui/check_arsenal_model_detail.cjs
```

The existing suite checks every vertex, normal, face, OBJ export, cache/determinism and both LOD budgets. The new suite pins the original IDs/spans/bounds and SHA-256 hashes of every far buffer set; checks all 46 receive geometry within the new budget; verifies that socket holes reach a recessed floor, pin caps keep hard normals, aft exhausts remain open, compressor authoring stays explicit, grooved joints retain their envelope, and the AFV vent bank has a real span. The private helper probes exist only in the test VM and add no runtime API.

Visual review should include aft aircraft angles and the underside of UAV gear, as well as the normal three-quarter card view. Small fasteners are intended for inspection zoom and are deliberately absent from map geometry. This document reports mesh contracts and measured buffer costs; it does not claim a measured browser frame rate.
