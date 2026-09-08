# Spheres configurable ground and aviation model assets

These twelve glTF 2.0 binary files contain the actual triangle geometry used by the Spheres equipment designer. They cover nine ground vehicle types, two tactical aviation platforms and a mobile tank preset. They are original procedural game art created for this project, with no downloaded models, textures, logos, or third-party art dependencies. The separate arsenal catalogue model deck is documented with its own source and provenance.

- **spheres-tank-balanced.glb**: the standard platform and baseline components.
- **spheres-tank-mobile.glb**: the standard platform with the mobility-focused powerpack.
- **spheres-tank-heavy.glb**: the heavy platform, protection, and armament.
- **spheres-tank-light.glb**: a compact light chassis, turret and 90 mm gun.
- **spheres-tank-destroyer.glb**: a fixed casemate with a 120 mm gun.
- **spheres-ground-ifv.glb**: tracked troop hull and compact autocannon turret.
- **spheres-ground-apc.glb**: six-wheel troop carrier and protected machine-gun station.
- **spheres-ground-recon.glb**: compact wheeled scout with an observation package.
- **spheres-ground-artillery.glb**: enclosed howitzer turret, long barrel and loading access.
- **spheres-ground-air-defense.glb**: tracked twin-cannon vehicle with a raised search array.
- **spheres-air-light-attack.glb**: compact straight-wing aircraft, single turbine, two external stores and tricycle landing gear.
- **spheres-air-tactical-strike.glb**: longer swept-wing aircraft, twin turbofans, paired stabilizers and four external stores.

Tank models carry twelve independent game specifications in their glTF mesh extras; specialist models carry thirteen, including their mission installation. Every semantic part includes a readable label, specification slot and contiguous vertex range for exact triangle picking in the designer. Internal ammunition choices are metadata and game ratings; exterior lockers identify the ammunition bay.

Aircraft carry eight independent specifications: engine, wing, radar, avionics, countermeasures, hardpoints, payload and fuel. External engine nacelles, wing planform, radomes, cockpit fittings and targeting pods, dispensers, mounting stations, bombs and fuel tanks change with the selected components. The parked models include landing gear, cockpit glazing and framing, intake fans and open exhaust nozzles. Fixed airframe and landing-gear geometry shares the wing structure selection; it is not an additional simulation upgrade. All eight slots are selectable in the live preview and retained in GLB metadata. This release depicts light attack and tactical strike aircraft; it does not add fighter or interception roles.

These are fictional game representations, not historical vehicle reproductions, engineering models, or manufacturing plans. Visual changes communicate selected components; the simulation owns all performance and cost values. Component choices can change six to eight wheels, track shoes, powertrain fittings, weapons, armor, optics, loading equipment, troop compartments, reconnaissance masts and radar. Vertex colors provide original olive and muted aircraft finishes without external textures. The in-game cosmetic finish is preserved in downloads; temporary selection highlights are never exported.

Open the files in a glTF-compatible 3D viewer or import them into a 3D modeling application. Models have embedded geometry and materials and require no companion files. The in-game designer can export the current configuration using the same exporter.

Regenerate from the repository root:

    node tools/ui/build_equipment_models.cjs

Verify committed assets match the source generator:

    node tools/ui/build_equipment_models.cjs --check

Source geometry: `spheres-web/ui/equipment-mesh.js`. Exporter: `spheres-web/ui/equipment-export.js`.
