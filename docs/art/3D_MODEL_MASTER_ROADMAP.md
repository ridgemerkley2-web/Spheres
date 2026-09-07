# SPHERES — complete 3D art production roadmap

Prepared for Ridge and Claude, 6 September 2026. This is a production brief for actual mesh assets and their game presentation. It is a proposed whole-game art scope, not a claim that the future gameplay systems below already exist. “Physical” means spatial, selectable 3D game geometry; this brief does not require printable solids, engineering CAD or manufacturing detail.

## 1. The outcome

Make Spheres feel like a living modern world: mountainous terrain with readable settlements, construction that visibly progresses, an economy represented by identifiable facilities and transport, and a military designer in which every external specification changes an actual part. A player should recognize the object's purpose at map scale, inspect it at close range, and understand its connection to a real game record.

Deliver a coherent reusable asset library, not hundreds of unrelated showcase renders. Prioritize silhouette, believable proportions, modularity, and readable controls. Beautiful renderings accompany the meshes but never substitute for editable geometry.

The complete handoff consists of this roadmap, `3D_ASSET_BACKLOG.csv`, and `CLAUDE_3D_ART_HANDOFF.md`. The CSV is the work register: one row is a base asset, component kit or scene assembly, explicitly distinguished by its deliverable type. Variants, LODs and animations are deliverables within those rows, not additional unique models in the headline count.

The initial register contains **282 work packages**: 85 base assets, 42 modular platforms, 35 component kits, 33 facility assemblies, 24 assembly kits, 15 building kits, 11 scatter kits, eight regional style kits, two retrofit kits and 27 scene assemblies. Some refine existing assets and some deliberately share geometry. This is not a promise of 282 entirely new unique meshes. Phase totals are P0: 4, P1: 24, P2: 71, P3: 46, P4: 29, P5: 22, P6: 60 and P7: 26; P8 is the library-wide release audit.

## 2. Current baseline: preserve and improve

| Existing work | Source | Treatment |
|---|---|---|
| 46 static catalogue meshes | `spheres-web/ui/arsenal-models.js`, `arsenal3d.js`, `arsenal3d.css` | Keep exact `arsenal.rs::DECK` IDs and names. Improve shape/detail incrementally; preserve shared rendering and glyph fallback. |
| Claude catalogue source | Commit `092569227023ff4278a5d699018af46bd39c7c94`, remote branch `feat/arsenal-models-codex` | Already imported into this working tree. Do not blindly cherry-pick it again or overwrite concurrent work. |
| Nine configurable ground platforms | `equipment-mesh.js`, `equipment-model.js`, `equipment-export.js` | Preserve part selection, camera, component IDs, quotes, and GLB export. Upgrade existing art rather than creating a competing designer. |
| Ten ground example GLBs | `spheres-web/ui/equipment-models/` | Nine platforms plus a mobile tank configuration; these are reference exports, not ten independent platform families. |
| Ground component rules | `spheres-sim/src/equipment.rs`, `equipment_specs.rs`, `equipment_ground.rs` | Authoritative compatibility and slot ownership. Art must not invent stats or change research. |
| 13 construction project kinds | `spheres-sim/src/production.rs::PROJECT_KINDS` | Each needs an intelligible physical representation. Automation and efficiency are retrofit kits, not fictional new standalone buildings. |
| Terrain and settlements | `tools/terrain/`, map renderer and existing world data | Preserve measured geography, projection and names. Surface detail is a visual layer, not a second world map. |
| Existing painted page art | `spheres-web/ui/page-art/`, `component-art/` | Remains useful as lightweight fallback and backdrop. New 3D scenes should complement it. |

Current configurable families are tanks (main battle, heavy, light, destroyer), IFV, APC, reconnaissance, self-propelled artillery and mobile air defense. Aircraft and ships in the catalogue are static previews; their component designers remain future engineering work. The latest ground build passed its release build, but production browser deployment was pending when this handoff was requested.

## 3. Art direction

**Style:** a detailed, grounded strategy-game miniature world. Use convincing silhouettes, bevelled edges, layered surfaces and restrained wear. Keep the overall shape clean enough to read on a small card. Avoid both toy-like featureless boxes and photorealistic micro-detail that disappears in the actual game.

**Lighting:** neutral daylight for asset review; restrained warm/cool contrast for page scenes. Separate lighting from albedo. Olive, sand and winter military finishes are cosmetic options. Civilian colors should make industries legible without making nations look like theme parks.

**Era:** begin with a believable 1990 baseline. Add late-1990s/2000s and contemporary visual variants only when an era or researched specification supports them. Future designs are clearly conceptual. Modern towers, huge solar fields and stealth silhouettes must not blanket every starting nation.

**Regional variety:** use climate, building materials and urban morphology, not a single stereotype per country. Start with eight reusable kits: temperate masonry, North American grid/suburb, East Asian dense city, Mediterranean, arid courtyard, tropical mixed-density, continental apartment estate, and high-latitude settlement. Mix kits for diverse cities. National identity comes first from actual city names, geography and sourced landmarks.

**Truthfulness:** distinguish representative scenery from measured buildings and game facilities. A model of a formation is a visual representative, not a claim that the formation contains one vehicle. Real equipment designations retain recognizable external features; dimension claims need a source. Generic/custom designs remain original game art. No downloaded assets without recorded usable rights and provenance.

## 4. Shared production contract

### Geometry and delivery formats

- Master source: editable `.blend` with a reproducible Blender Python generator where practical, or the existing deterministic JavaScript mesh generator. State which is authoritative. No opaque mesh-only delivery if components must remain editable.
- Portable deliverable: self-contained glTF 2.0 `.glb`, embedded materials, mesh names, part metadata, pivots and LOD assets. OBJ is an optional interchange export, not the metadata authority.
- Runtime today: preserve the existing local procedural/vertex-color pipelines. There is no general imported textured-GLB runtime loader in this game. A new GLB cannot be integrated merely by copying it into a folder.
- First implement a small local offline conversion path from the agreed GLB subset into the existing renderer's indexed triangles/material groups and semantic ranges, or extend the renderer deliberately. The first milestone must prove this path before bulk Blender production. No CDN or network dependency; no mandatory new player build step.
- PBR textures may be authored in masters, but treat textured runtime materials as a separate milestone. Supply a vertex-color approximation compatible with the current rendering path. Do not silently lose UVs, normals or part identities in conversion.
- Native convention is metres, right-handed, +Y up, +Z forward, +X right. Blender export must convert its axes explicitly. Apply transforms and verify with an axis/one-metre fixture.
- Ground asset root is centered horizontally at ground contact, Y=0. Building roots are centered on their footprint at grade. Ships use a documented waterline datum; aircraft and spacecraft roots use a documented assembly datum with separate ground/gear contact metadata.
- No hidden negative scale, accidental duplicate faces, degenerate triangles, NaNs, z-fighting panels or inverted normals. Separate watertightness is optional for decorative game meshes; visible silhouette and shading must be sound.

### Naming and metadata

Proposed asset identity: `category.family.variant.v1`; existing game IDs stay in a separate mapping field and never get renamed to match art. Example: `ground.ifv.baseline.v1` maps to `ground_ifv`.

Every manifest entry includes: asset ID, display name, kind, version, source file, generator command, runtime output, GLB path, game ID or `visual_only`, dimensions and basis (`measured`/`approximate`/`fictional`), axis convention, pivot datum, bounds, LOD triangle counts, material/texture bytes, slots, sockets, animation names, collision/selection proxy, era tags, source/license notes, preview images, validation result and owning milestone.

Each selectable part carries `part_id`, `slot`, `label`, and the relevant `component_id`. Preserve existing part-range contracts through adapters. Never use mesh array order as a permanent identity. A selected radar or wheel must highlight that surface and open the correct specification; keyboard selection must work too.

### Assembly rules

Use named sockets with explicit parent, transform, forward/up axes and supported module family. Keep a component compatibility matrix sourced from the simulation for implemented platforms. An attractive but incompatible combination must not appear selectable. For future families, label the matrix `proposed` until engineering implements it.

Components are reusable but silhouettes must stay distinct. A light tank is not a uniformly shrunken MBT; an IFV needs troop volume and access; a scout needs observation equipment; a howitzer needs a different mounting and loading space. Maintain variant-specific clearance envelopes as visual bounds, not engineering calculations.

Internal slots may be represented by a removable inspection panel, a stylized removable pack or a labelled internal bay. Do not pretend invisible software needs a giant external box. Ammunition specifications need readable stowage/inspection representation, not detailed weapon internals.

### Proposed performance targets

These are initial budgets to validate on the user's machine, not measured performance promises.

| Asset/use | Detail target | Texture/material target |
|---|---|---|
| Ground vehicle close inspection LOD0 | 20–45k triangles assembled | ≤4 materials; vertex colors first; optional shared 1–2K atlas later |
| Aircraft inspection LOD0 | 25–60k | ≤4 materials; shared family atlas |
| Large ship inspection LOD0 | 40–90k | ≤6 materials; repeated fittings instanced |
| LOD1 catalogue preview | 4–12k | Retain silhouette, markings and named selectable major parts |
| LOD2 map vehicle | 300–1,500 | One material where practical |
| Building close view / map | 2–12k / 100–800 | Shared regional/industrial atlas or palette |
| Tree/prop near / far | 100–800 / billboard or 20–100 | Shared atlas; avoid alpha overdraw |
| Scene assembly | Target ≤150k visible triangles initially | Shared assets, baked backdrop detail; no all-page continuous rendering |

Measure frame time, upload stalls, GPU memory and download bytes in a fixed benchmark. Target smooth 60fps in the focused single-model viewer and at least 30fps in a representative city/map scene on agreed hardware. If missed, reduce distant content before lowering text/control responsiveness. Establish final simultaneous instance limits through the first benchmark, not a promise of unlimited cities.

One shared renderer for catalogue cards; render only visible cards and stop when hidden. Map LODs use screen size with hysteresis, spatial culling, instancing and bounded caches. No WebGL context per asset/card. Retain low-performance settings and a static image/glyph fallback. Animation honors reduced motion and never advances the simulation.

## 5. Full asset scope

The accompanying CSV enumerates the work. The families below explain what “done” means, including assemblies not sensible as one giant mesh.

### A. Ground forces — immediate priority

Upgrade all nine implemented platforms first. Each ships a clear baseline and working compatible component swaps. Tanks have 12 independent slots: mobility, transmission, tracks, suspension, turret, armament, ammunition, protection, active protection, sensors, fire control and communications. Preserve the actual code keys, including `fire_control` and `active_protection`.

Specialists have 13 slots: APC/recon substitute `wheels` for tracks; IFV/APC add `troop_compartment`; reconnaissance adds `recon_package`; artillery adds `artillery_loader`; mobile air defense adds `radar`. Build a manifest entry for every actual selectable component from the current registry. Do not assume every shared component is valid on every hull.

Reusable component packs cover powertrain/exhausts, transmissions/access panels, tracks, wheels, suspension, turret/casemate mounts, guns, stowage, armor modules, active protection, optics, fire-control fittings, radios, troop bays, scout installations, artillery loading and radar. Visual differences must correspond to selections; painting the same barrel a different color is insufficient for different weapon families.

Follow-up visual families: logistics truck, fuel truck, recovery vehicle, combat engineer, bridge layer, command vehicle, field ambulance, missile carrier, radar carrier, multiple-launch rocket vehicle, towed artillery and unmanned ground scout. These need separate gameplay work before becoming purchasable designs. Art availability alone grants no combat, fuel or support capability.

### B. Air forces

Preserve and upgrade every existing named/generic air catalogue item. Then make reusable visual archetypes for light fighter, twin-engine multirole, stealth fighter, interceptor, close-air-support aircraft, strategic bomber, flying-wing bomber, transport, tanker, airborne early warning, electronic warfare, maritime patrol, trainer, scout drone, armed drone, stealth drone, attack helicopter, utility helicopter and heavy-lift helicopter.

Future designer module proposal: fuselage/airframe, wing, tail, engine count and nacelles, intake, landing gear, nose sensor, avionics bay, cockpit/canopy, countermeasure fittings, external pylons, compatible stores and fuel tanks. Rotorcraft use separate rotor/hub, tail rotor or alternative configuration, transmission housing and landing-system families. No arbitrary mixing of incompatible wings, rotor systems or engines.

Basic optional animation: gear and control surface display, rotor rotation and bay-door inspection. Keep stores as visual game representations. Aircraft aerodynamics, hardpoint performance and component pricing are future sim contracts, not values authored by the artist.

### C. Naval forces

Upgrade existing patrol/escort/task-group and submarine catalogue models. Add patrol boat, missile boat, corvette, frigate, destroyer, cruiser, carrier, helicopter/amphibious ship, landing craft, conventional submarine, nuclear attack submarine, strategic submarine, fleet replenishment ship and mine-countermeasure vessel as distinct archetypes.

Reusable hull sections, bridges, funnels, mast/sensor families, visible propulsion fittings, deck mounts, closed launch modules, defensive mounts, hangars, helicopter decks, aircraft deck props and boat davits form the naval library. Hull class determines legal deck layout and sockets. Ship module placement is authored and checked for clipping; free-form arbitrary deck building is not required in the first implementation.

Use waterline, bow direction, deck height, mast pivot and wake anchors consistently. A task-group icon must be labelled an assembly/representative, not given a fictional single-ship dimension. Underwater view is a separate display mode; no exposed classified/internal engineering detail is needed.

### D. Infantry, support, missiles and space

Keep all existing deck IDs covered, including refits and abstract network capabilities. Infantry receives a reusable low-detail human rig with standing, walking, carrying and idle poses; equipment variants use helmets, packs, radios and generic external gear. Formation views are small representative dioramas, not literal personnel counts. Injury/gore is unnecessary.

Abstract fits such as tactical networks and autonomous recognition use a plausible equipment rack, field terminal or command diorama with an explanatory label. Do not present an algorithm as a newly manufactured vehicle.

Missile/munition previews need external silhouette, fins, canisters and carrier/launch-context props appropriate to the game. No operational mechanism simulation. Space assets include observation/communications/navigation satellites, radar satellite, small-satellite bus, ground antenna, launch vehicle and launch-pad assembly. Solar panels, antennas and masts have separate pivots; orbit markers remain distinct from object scale. Additional satellites and launch systems are visual/future entries until mapped to real game records.

### E. Economy, construction and resources

Give every current project kind a physical composition:

| Game key | Physical representation |
|---|---|
| `infrastructure` | Road/bridge/utility corridor kit; match the project context |
| `civilian_industry` | Modular factory and service yard |
| `power_grid` | Substation, pylons and feeder equipment |
| `research_center` | Lab campus with service buildings |
| `arms_plant` | Assembly hall, test/service yard and secure loading area |
| `machinery_works` | Machine-tool halls and equipment loading |
| `generation` | Generic generation facility, with sourced/known technology-specific variants later |
| `processing_plant` | Industrial processing hall, tanks/pipes and material handling |
| `freight_terminal` | Rail/road/container transfer yard |
| `warehouse` | Storage halls and loading docks |
| `automation` | Robot/machine-cell retrofit installed in an existing facility |
| `efficiency` | Heat-recovery/insulation/control retrofit installed in an existing facility |
| `starter_industry` | Smaller workshop/light-industry compound |

All standalone facilities need site, foundation, structural frame, enclosed/commissioning and completed states. Use a small shared stage kit plus asset-specific structure, rather than five wholly unrelated copies. Thresholds bind to actual completed work from the server. Paused/blocked sites stop activity and show an honest overlay; elapsed wall-clock time never completes a building. A completed level upgrade adds a wing or increases an appropriate compound footprint within clear limits. Five building levels need not mean five identical skyscrapers.

Resource scenery covers open-pit mine, underground mine entrance, quarry, oil field, offshore platform, gas processing, refinery, ore processing, steel works, cement plant, farms, grain storage, greenhouse, timber yard, fishing harbor, water works and recycling. Only facilities supported by actual records get facility pins. Representative regional industry is labelled scenery and does not fabricate extraction capacity.

### F. Civilian transport and logistics

Create cars, buses, delivery vans, articulated freight trucks, tankers, construction vehicles, locomotives, passenger coaches, container/grain/tank rail wagons, inland barge, container ship, bulk carrier, oil tanker, ferry and cargo aircraft. Reuse military and civilian bases where sensible while preserving silhouette.

Networks require road straights/bends/junctions, divided highways, rail straights/curves/switches, bridges, tunnel portals, stations, terminals, airport runway/taxiway/hangar/gates, quays and port cranes. Use spline-driven segment kits with compatible endpoints rather than individually modelling every road.

Shipment animations follow existing route/deadline data when available. Decorative traffic is a bounded visual loop explicitly separate from cargo accounting. Never imply one displayed truck equals the precise shipment volume unless the mapping is implemented and disclosed.

### G. Cities, provinces and terrain

Make towns visible before authoring dozens of prestige military models. Start with houses, row houses, low/mid/high-rise apartments, office blocks, storefronts, warehouses, civic buildings, school, hospital, university, stadium, park and utility buildings. Assemble district tiles and regional variants from common footprints. Separate residential, commercial, industrial and civic clusters.

Use existing named city anchors. City population may guide representative density; an inferred layout must be labelled representative, not a street-accurate reconstruction. Source actual roads/buildings only in an explicitly researched later geography pass. Do not place a duplicate city asset at every province centroid.

Terrain detailing needs biome trees, scrub, grasses, rocks, cliffs, snow-edge features, shoreline dressing, river banks and agricultural field patches. Peaks and valleys remain driven by the existing elevation surface; scattered rock meshes must not replace or displace that data. Terrain-following placement excludes water, excessive slope, important labels and transport corridors. Bridges cross actual gaps; buildings use local grade pads rather than floating or sinking.

At world zoom show relief and clean city markers; at regional zoom show settlement clusters and major facilities; at city zoom show block/road silhouettes and selected buildings. Unit map representations use a documented readability scale, while inspection models remain metre-based. Never force one-to-one physical scaling onto a globe where it makes vehicles invisible.

Named landmarks are a later optional pass. Begin with a small curated set chosen by gameplay visibility, with reference/provenance for each. Do not attempt one handcrafted city for every nation before the reusable system works.

### H. Page scenes and environmental storytelling

Build reusable scene assemblies for the main menu, nation selection, economy, treasury/budget, construction, industry, resources, research campus, six component branches, equipment library, ground designer, aircraft hangar, naval dockyard, production, service/refit, logistics, diplomacy, intelligence, domestic government, world/history and help.

Use the same model assets placed into hangars, workshops, offices, yards and landscapes. Main menu: a view linking city, port and industry. Nation selection: globe/table with changeable nation context. Economy: industrial/city panorama. Construction: a legible active site. Research: clean lab/workshop. Service: repair bay with the selected model. This creates thematic distinction without maintaining a separate expensive art pipeline for each page.

Every scene has a low-cost still fallback, calm text-safe area, predictable subject placement and desktop/narrow crops. Keep tables, modal surfaces and action buttons above the art with reliable contrast. Interactive model views use neutral lighting even when the surrounding page is cinematic. Background animation must be pausable and never steal clicks, focus or zoom gestures from the controls.

Cross-cutting finishing work belongs inside the relevant package: optional owner markings and editable number plates, three military finishes, facility signs, non-destructive selection/damage overlays, shared dust/exhaust/wake effects, ground contact shadows and inspection-room lighting. Damage and smoke only reflect supported game state or an explicitly decorative scene; do not invent building destruction records. Country markings must follow current ownership and era data when available, not be permanently painted into a generic mesh. Numbers, budgets, diplomacy graphs, research connectors, map borders and other abstract information remain crisp UI elements; turning every statistic into a 3D prop would make the game harder to use.

## 6. Build sequence and exit gates

| Phase | Deliverable | Exit gate |
|---|---|---|
| P0 — contract and vertical slice | Inventory, manifest, viewer/export adapter, one polished MBT, one wheeled APC, one factory and one representative town block | All four render inside the actual game pipeline with LOD, metadata, correct scale and fallback; preview screenshots and measured performance recorded. |
| P1 — complete ground designer | Nine platforms, all implemented component visuals, selectable parts, three finishes, export fidelity | All legal starting designs work; representative incompatible combinations remain refused by the existing sim; selection/camera survive edits; saved designs reload unchanged. |
| P2 — visible civilian world | All 13 project representations, construction stages, core city/block/road/tree kits, initial logistics props | A real saved campaign shows named cities and correctly staged projects at appropriate zoom; no invented capacity and no label/input obstruction. |
| P3 — entire current arsenal | Improve all 46 static deck entries using the accepted language | Exact bidirectional deck coverage; no ID/name drift; all catalogue, line and holdings contexts work in a shared renderer. |
| P4 — aircraft and rotorcraft library | All proposed air archetypes and module sockets; hangar scene | Art combinations proven in isolated assembly viewer; only already-supported game items integrated as purchasable. Air designer gameplay is a separate engineering milestone. |
| P5 — naval library | Surface/submarine archetypes, modular fittings and dockyard | Waterline/scale/LOD validated; static supported deck items integrated; future naval designer rules remain explicit. |
| P6 — full civilian and support coverage | Resource facilities, remaining transport, support vehicles, infantry, space and regional architecture | Every backlog family has a reviewed base asset or documented deliberate reuse; era variants are tagged; map density benchmark passes. |
| P7 — whole-game scene polish | All page scene assemblies, motion and fallback exports | All page sizes readable, reduced motion/fallback verified, no unbounded GPU work or new navigation friction. |
| P8 — release audit | Coverage report, deduplication, budgets, provenance and packaging | No missing IDs/files, no broken exports, fixed benchmark and save compatibility checks pass; handoff catalog is complete. |

P0 is a hard dependency for bulk modelling. After P0, do P1 and P2 before spending weeks on future aircraft/naval designs. P3 reuses previous Claude work. Regional architecture and scene polish should be built from proven kits. Work in small reviewable batches (roughly 3–6 base assets or one complete component family), then integrate and validate. Avoid promising a calendar date until P0 establishes throughput.

## 7. Definition of done for each asset

1. Editable source, generator/version instructions, portable GLB, runtime-compatible output, manifest entry and provenance exist.
2. Front, side, rear, top and three-quarter screenshots demonstrate silhouette, scale and pivots under the shared neutral rig. Include a screenshot in its actual intended game context.
3. Bounds, axes, normal direction, triangle/material/byte budgets and LOD transitions pass automated checks. Missing textures produce no black/pink surfaces; shared materials are not duplicated per instance.
4. Supported modules assemble without visible clipping or detached fittings in baseline and boundary combinations. Semantic picking maps to the same component before and after export/import.
5. Animation previews show correct pivots. No animation is required for a static object unless its backlog brief asks for one. Reduced-motion and hidden-screen behavior are tested.
6. Deterministic generation gives the same normalized geometry/manifest. Remove nondeterministic timestamps from canonical outputs or compare normalized data rather than arbitrary file bytes.
7. UI and simulation values remain owned by their existing code. Art exports cannot change national stats, advance time, rewrite IDs or alter saved revisions.
8. Runtime view is checked at desktop and 390px width, with WebGL unavailable and after context loss/restoration. Selection also works by keyboard; labels and tooltips remain usable.

Use existing `tools/ui/check_arsenal_models.cjs`, `check_equipment_mesh.cjs`, `check_equipment_model.cjs`, `check_equipment_export.cjs` and `build_equipment_models.cjs --check` as regression anchors. Extend them for new geometry contracts. Follow repository test requirements when integrating; an art-gallery screenshot alone is not evidence of working in-game integration.

## 8. Integration boundaries and handoff discipline

Before editing, inspect the actual current branch and dirty tree. Read `BIBLE.md`, `SPEC.md`, `CLAUDE.md`, `EQUIPMENT_DESIGNER.md` and both model READMEs. This workspace contains substantial uncommitted concurrent work. Create `codex/` branches only when appropriate to the working convention; do not reset, clean or replace the workspace. Integrate additive asset batches with a precise file list and commit reference.

Keep masters under a proposed `art-source/` tree and production records under `docs/art/`. Preserve current runtime paths until the adapter is proven. Large editable masters may require Git LFS or an external versioned asset bundle; establish that before committing large binaries. Runtime assets must remain available from a normal release build without fetching private tools.

Every batch handoff reports: assets completed, existing assets reused, files changed, source commit, exact regeneration commands, screenshots, measured budgets, automated/browser checks, unresolved limitations and next batch. Mark rows as proposed/in progress/reviewed/integrated, with integration status distinct from model completion.

Adding an aircraft hull does not implement aircraft design; adding a port does not create trade capacity; adding an ambulance does not create a medical system. Model these visuals now where useful, but connect capability only through a separate reviewed simulation change. Keep the current save intact while showing the new art.

## 9. First assignment to Claude

Start with P0: inspect the existing catalogue and configurable mesh contracts, publish the normalized manifest, and produce a visually improved MBT, wheeled APC, arms plant with staged construction, and representative temperate town block. Prove the source-to-game conversion with actual clickable modules, map-scale LODs and one real in-game screenshot for each. Return the small batch for review before expanding to all nine ground platforms and all construction types.

The first review should answer four questions: Do the models look substantially better? Do selected components visibly match the specifications? Does a settlement/factory read well at useful map zoom? Can this pipeline produce the rest of the game without manual rework for every mesh?
