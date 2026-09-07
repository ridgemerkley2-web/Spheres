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

Aircraft carry eight independent specifications: engine, wing, radar, avionics, countermeasures, hardpoints, payload and fuel. External engine nacelles, wing planform, radomes, cockpit fittings and targeting pods, dispensers, mounting stations, bombs and fuel tanks change with the selected components. The parked models include landing gear, cockpit glazing and framing, intake fans and open exhaust nozzles. Fixed airframe and landing-gear geometry shares the wing structure selection; it is not an additional simulation upgrade. All eight slots are selectable in the live preview and retained in GLB metadata. This release depicts light attack and tactical strike aircraft; it does not add fighter or interception roles.

These are fictional game representations, not historical vehicle reproductions, engineering models, or manufacturing plans. Visual changes communicate selected components; the simulation owns all performance and cost values. Component choices can change six to eight wheels, track shoes, powertrain fittings, weapons, armor, optics, loading equipment, troop compartments, reconnaissance masts and radar. Vertex colors provide original olive and muted aircraft finishes without external textures. The in-game cosmetic finish is preserved in downloads; temporary selection highlights are never exported.

Open the files in a glTF-compatible 3D viewer or import them into a 3D modeling application. Models have embedded geometry and materials and require no companion files. The in-game designer can export the current configuration using the same exporter.

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
  const bytes = glb(build(variant.spec), variant.name);
  output(variant.file||`spheres-tank-${variant.id}.glb`, bytes);
  process.stdout.write(`${variant.id}: ${bytes.byteLength.toLocaleString('en-US')} bytes\n`);
}
output('README.md', readme);
