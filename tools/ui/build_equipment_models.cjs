// Deterministically generate the same original meshes used by the equipment room.
// node tools/ui/build_equipment_models.cjs [--check]
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const {build} = require('../../spheres-web/ui/equipment-mesh.js');
const {glb} = require('../../spheres-web/ui/equipment-export.js');
const folder = path.resolve(__dirname, '../../spheres-web/ui/equipment-models');
const baseline = {
  platform: 'tank_standard',
  components: {mobility:'engine_diesel_900',transmission:'transmission_manual',tracks:'tracks_standard',suspension:'suspension_torsion',turret:'turret_standard',armament:'gun_105',ammunition:'ammo_mixed',protection:'protection_standard',active_protection:'aps_none',sensors:'optics_day',fire_control:'fcs_basic',communications:'comms_radio'}
};
const variants = [
  {id: 'balanced', name: 'Spheres balanced tank', spec: baseline},
  {id: 'mobile', name: 'Spheres mobile tank', spec: {...baseline, components: {...baseline.components, mobility: 'engine_diesel_1200'}}},
  {id: 'heavy', name: 'Spheres heavy tank', spec: {...baseline, platform: 'tank_heavy', components: {...baseline.components, mobility:'engine_diesel_1200',protection: 'protection_heavy',turret:'turret_heavy',armament:'gun_120'}}},
  {id: 'light',name:'Spheres light tank',spec:{...baseline,platform:'tank_light',components:{...baseline.components,mobility:'engine_diesel_600',turret:'turret_compact',armament:'gun_90'}}},
  {id: 'destroyer',name:'Spheres tank destroyer',spec:{...baseline,platform:'tank_destroyer',components:{...baseline.components,turret:'turret_casemate',armament:'gun_120',ammunition:'ammo_penetrator'}}}
];
for(const [id,platform,name] of [
  ['ifv','ground_ifv','Spheres infantry fighting vehicle'],['apc','ground_apc','Spheres armored personnel carrier'],
  ['recon','ground_recon','Spheres reconnaissance vehicle'],['artillery','ground_artillery','Spheres self-propelled artillery'],
  ['air-defense','ground_air_defense','Spheres mobile air defense']
])variants.push({id,name,file:`spheres-ground-${id}.glb`,spec:{platform}});
for(const [id,platform,name] of [['light-attack','air_light_attack','Spheres light attack aircraft'],['tactical-strike','air_tactical_strike','Spheres tactical strike aircraft']])variants.push({id,name,file:`spheres-air-${id}.glb`,spec:{platform}});
const readme = `# Spheres configurable ground and aviation model assets

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

Aircraft carry eight independent specifications: engine, wing, radar, avionics, countermeasures, hardpoints, payload and fuel. External engine nacelles, wing planform, radomes, cockpit fittings and targeting pods, dispensers, mounting stations, bombs and fuel tanks change with the selected components. The parked models include landing gear, cockpit glazing and framing, intake fans and open exhaust nozzles. The tactical rebuild adds a real cockpit opening, two seats and instruments, transparent glass, separate trailing controls, blended wing shoulders and underwing fuel tanks. Its GLB stores opaque/glass primitive slices over unchanged global buffers; mesh extras retain surface ranges and their opacity alongside semantic parts. Both inspection aircraft exceed 100,000 triangles, with cheaper catalogue/map meshes available in the generator. Fixed airframe and landing-gear geometry shares the wing structure selection; it is not an additional simulation upgrade. All eight slots are selectable in the live preview and retained in GLB metadata. This release depicts light attack and tactical strike aircraft; it does not add fighter or interception roles.

These are fictional game representations, not historical vehicle reproductions, engineering models, or manufacturing plans. Visual changes communicate selected components; the simulation owns all performance and cost values. Component choices can change six to eight wheels, track shoes, powertrain fittings, weapons, armor, optics, loading equipment, troop compartments, reconnaissance masts and radar. Vertex colors provide original olive and muted aircraft finishes without external textures. The in-game cosmetic finish is preserved in downloads; temporary selection highlights are never exported.

The September inspection pass adds paired road wheels with guide clearance, tapered track guide horns, attached seams, hinges, latches, grilles and welds. Aircraft gain curved wing sections, canopy seals, landing-gear fittings and overlapping exhaust petals. The later armored-vehicle rebuild deliberately refines the tank and specialist silhouettes at all three detail levels. Inspection assets retain explicit budgets of 12 MB per tank and 5 MB per specialist.

The following form-refinement pass replaces overhanging cylindrical nose armor with folded plates, reshapes turret cheeks and roof transitions, blends aircraft wing roots into the body and exposes their intake throats. Aircraft and arsenal previews use locally bundled CC0 paint normal/roughness maps; their source and license are recorded in the military-textures README. These shared material maps are renderer resources and are not embedded in the exported GLBs.

The tank-focused rebuild corrects the hull proportions, integrates the turret cheeks and bustle, recesses the mantlet, and refines the running gear at all three detail levels. Tank paint now uses explicit material classes: steel, rubber, lenses and cables keep their own finishes. These five canonical tank files retain the generator's base vertex palette, so they rebuild exactly from their stored specifications. Downloads from the live designer include the selected finish and approximate contact shading. Fragment-level camouflage, fine wear and lighting cannot be reproduced exactly by sparse vertex colors; downloaded custom finishes sample them at vertices. The separately licensed Strv 103 reference asset lives in tank-assets/strv103 and is not one of these configurable designs.

The tactical interior pass adds closed swept intake fan blades, recessed exhaust liners and turbine faces, converging nozzle petals and actuator links. The two-seat cockpit includes shaped seat pads, harness webbing and buckles, side consoles, throttles, control grips, rudder pedals and distinct analog gauges/digital display graphics. The default tactical inspection build contains 228,640 triangles; catalogue detail uses 14,904 and the unchanged map mesh uses 1,696. The flight workshop's Cockpit, Engines and Intakes controls inspect actual mesh regions without changing the export.

The specialist rebuild gives IFV, APC, reconnaissance, artillery and air-defense vehicles distinct long/compact troop hulls, integrated weapon mounts and mission equipment. Wheeled tires have open bead rings around visible hubs; tracked road wheels and end gears have separate clearances. These five platforms now use the same authored material classes, camouflage, wear and contact shading as the approved tanks in the live designer. Their canonical GLBs also keep generator colors for exact specification rebuilds. See docs/art/ARMORED_VEHICLE_REBUILD.md for the art and validation record.

Open the files in a glTF-compatible 3D viewer or import them into a 3D modeling application. Models have embedded geometry and materials and require no companion files. The in-game designer can export the current configuration using the same exporter. The game's local material lighting and self-shadows are renderer effects; they are not baked textures in these GLBs. A third-party viewer applies its own lighting to the embedded vertex colors and base material.

Regenerate from the repository root:

    node tools/ui/build_equipment_models.cjs

Verify committed assets match the source generator:

    node tools/ui/build_equipment_models.cjs --check

Source geometry: \`spheres-web/ui/equipment-mesh.js\`. Exporter: \`spheres-web/ui/equipment-export.js\`.
`;
function output(name, bytes) {
  const target = path.join(folder, name);
  const expected = Buffer.from(bytes);
  if (process.argv.includes('--check')) {
    const actual = fs.readFileSync(target);
    // This Windows repository allows CRLF Markdown checkouts; model bytes stay exact.
    if (name.endsWith('.md')) assert.equal(actual.toString('utf8').replace(/\r\n/g, '\n'), expected.toString('utf8'));
    else assert.deepEqual(actual, expected, `${name} is out of date; regenerate the model assets.`);
  } else {
    fs.mkdirSync(folder, {recursive: true});
    if (!fs.existsSync(target) || !fs.readFileSync(target).equals(expected)) fs.writeFileSync(target, expected);
  }
}
for (const variant of variants) {
  const mesh = build(variant.spec);
  const bytes = glb(mesh, variant.name);
  output(variant.file||`spheres-tank-${variant.id}.glb`, bytes);
  process.stdout.write(`${variant.id}: ${bytes.byteLength.toLocaleString('en-US')} bytes\n`);
}
output('README.md', readme);
