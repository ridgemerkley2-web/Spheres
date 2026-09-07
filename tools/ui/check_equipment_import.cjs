// The return leg of the art pipeline. equipment-export.js proves geometry can
// leave the designer; this proves the same bytes come back as the object the
// viewer already draws, and that everything this runtime cannot draw is refused
// by name instead of half-imported.
//
// Every model bar is asked of the twelve committed .glb files, so this fails if the
// reader, the writer, the generator or the committed bytes drift apart. The
// design each file is rebuilt from is the one the file itself records in
// extras.specification: that is the round trip being proved, and it does not go
// red merely because build_equipment_models.cjs later chooses different
// components for a variant. Whether the committed asset is the CURRENT
// generator's output is that script's own --check, not this file's.
const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const source = path.resolve(__dirname, '../../spheres-web/ui/equipment-import.js');
const {fromGlb} = require(source);
const {build} = require('../../spheres-web/ui/equipment-mesh.js');
const {glb} = require('../../spheres-web/ui/equipment-export.js');
const viewer = require('../../spheres-web/ui/equipment-model.js');
const folder = path.resolve(__dirname, '../../spheres-web/ui/equipment-models');

// File to the platform it must carry, so a swapped or misnamed asset is caught
// even though the components come from the file. Nine ground types, two aircraft, plus the one
// mobility preset, matching the committed model README.
const MODELS = [
  ['spheres-tank-balanced.glb', 'tank_standard'], ['spheres-tank-mobile.glb', 'tank_standard'],
  ['spheres-tank-heavy.glb', 'tank_heavy'], ['spheres-tank-light.glb', 'tank_light'],
  ['spheres-tank-destroyer.glb', 'tank_destroyer'], ['spheres-ground-ifv.glb', 'ground_ifv'],
  ['spheres-ground-apc.glb', 'ground_apc'], ['spheres-ground-recon.glb', 'ground_recon'],
  ['spheres-ground-artillery.glb', 'ground_artillery'], ['spheres-ground-air-defense.glb', 'ground_air_defense'],
  ['spheres-air-light-attack.glb', 'air_light_attack'], ['spheres-air-tactical-strike.glb', 'air_tactical_strike']
];
const cache = new Map();
const model = file => {
  if (!cache.has(file)) cache.set(file, fromGlb(fs.readFileSync(path.join(folder, file))));
  return cache.get(file);
};

// Node's deep comparison walks a float typed array element by element as object
// keys and does not return in useful time at 400k vertices, so compare here and
// report the first difference by index rather than dump the whole array.
function identical(field, got, want) {
  assert.equal(got.length, want.length, `${field}: ${got.length} values against ${want.length}`);
  for (let i = 0; i < want.length; i++)
    if (!Object.is(got[i], want[i])) assert.fail(`${field}[${i}] is ${got[i]}, expected ${want[i]}`);
}
// Normals are the one attribute the exporter recomputes, dividing by a float32
// magnitude that is itself within an ulp of one, so they return within rounding
// rather than bit for bit. Measured in ulps rather than as an absolute distance,
// because an absolute bar is meaningless on a component near zero and slack on
// one near one: 1e-6 admits 8 ulp at 1.0 and 67 ulp at 0.125.
//
// The bar is 4 ulp. Two float32 roundings is what the mechanism can produce
// (n/m, where m is a float32 magnitude within an ulp of one), and measured
// 2026-09-06 across all ten committed assets every difference is exactly 1 ulp
// and never 2: 306 differing components in 2,749,320, worst 5.96e-8 absolute
// against the generator and 1.86e-9 on re-export, with the freshly generated
// air-defense design bit-identical. 4 ulp is that mechanism plus headroom; it
// still reds at ~8000 ulp, the size of the drift that matters — normals arriving
// unnormalised. Checked red at 5 ulp of injected drift before being committed.
const f32 = new Float32Array(1), i32 = new Int32Array(f32.buffer);
// Sign-magnitude float32 bits laid out as a monotonic integer line, so adjacent
// representable values are one apart. Positive and negative zero land on the
// same ordinal: they shade and export identically, so they are not a difference.
const ordinal = value => { f32[0] = value; const raw = i32[0]; return raw < 0 ? -2147483648 - raw : raw; };
function close(field, got, want, tolerance = 4) {
  assert.equal(got.length, want.length, `${field}: ${got.length} values against ${want.length}`);
  let worst = 0, where = -1;
  for (let i = 0; i < want.length; i++) {
    const difference = Math.abs(ordinal(got[i]) - ordinal(want[i]));
    if (difference > worst) { worst = difference; where = i; }
  }
  assert.ok(worst <= tolerance, `${field}[${where}] drifted by ${worst} ulp (${Math.abs(got[where] - want[where]).toExponential(3)}): ${got[where]} against ${want[where]}`);
}
// The exporter measures accessor min/max over the float32 vertex data while the
// generator keeps float64, and the light chassis is scaled once more after that
// rounding: at most two float32 roundings, so 2^-23 of relative error. The bar
// is 2^-22; the worst measured across the ten assets, 2026-09-06, is 4.29e-8
// relative, on spheres-ground-recon.glb.
const rounded = (got, want) => Math.abs(got - want) <= Math.abs(want) * 2 ** -22;

function wholeParts(mesh) {
  let end = 0;
  for (const part of mesh.parts) {
    assert.equal(part.first, end, `${part.name} does not begin where the previous part ended`);
    assert.ok(part.count > 0 && part.count % 3 === 0, `${part.name} covers ${part.count} vertices`);
    assert.ok(part.label, `${part.name} carries no label for the part selector`);
    assert.ok(part.slot in mesh.specification.components, `${part.name} names slot ${part.slot}, which its specification does not carry`);
    end += part.count;
  }
  assert.equal(end, mesh.positions.length / 3, 'the parts must cover every vertex, or a triangle is unpickable');
}

for (const [file, platform] of MODELS) {
  test(`${file} imports as a complete, self-consistent runtime mesh`, () => {
    const mesh = model(file);
    assert.ok(mesh.positions.length > 9000, 'a vehicle should import with modelled detail, not a placeholder');
    assert.equal(mesh.positions.length % 9, 0);
    assert.equal(mesh.normals.length, mesh.positions.length);
    assert.equal(mesh.colors.length, mesh.positions.length);
    assert.equal(mesh.triangleCount, mesh.positions.length / 9);
    for (const field of ['positions', 'normals', 'colors'])
      assert.equal(mesh[field].constructor.name, 'Float32Array', `${field} must upload without a conversion`);
    for (let i = 0; i < mesh.normals.length; i += 3)
      assert.ok(Math.abs(Math.hypot(mesh.normals[i], mesh.normals[i + 1], mesh.normals[i + 2]) - 1) < 1e-5);

    assert.equal(mesh.specification.platform, platform);
    const slots = platform.startsWith('air_') ? 8 : platform.startsWith('ground_') ? 13 : 12;
    assert.equal(Object.keys(mesh.specification.components).length, slots, 'every specification slot should survive the round trip');
    assert.ok(mesh.description.length > 20, 'the model should import with its own description');
    assert.ok(mesh.parts.length > 5);
    wholeParts(mesh);

    // The bounds are rebuilt from the accessor min/max rather than measured, so
    // check they are the true extents of the geometry and not merely around it.
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < mesh.positions.length; i++) {
      min[i % 3] = Math.min(min[i % 3], mesh.positions[i]);
      max[i % 3] = Math.max(max[i % 3], mesh.positions[i]);
    }
    assert.deepEqual(mesh.bounds, {min, max});
    assert.equal(mesh.bounds.min[1], 0, 'Y=0 is the ground contact, and the import must not move it');
    for (let axis = 0; axis < 3; axis++) assert.ok(mesh.bounds.max[axis] > mesh.bounds.min[axis]);
  });

  test(`${file} rebuilds bitwise from the specification it carries`, () => {
    const mesh = model(file), generated = build(mesh.specification);
    // The pipeline closes here: the design recorded in the file, run back through
    // the generator, is the geometry in the file. Positions and colors return bit
    // for bit, because the exporter writes the float32 the generator rounded and
    // nothing between the two rewrites it.
    identical('positions', mesh.positions, generated.positions);
    identical('colors', mesh.colors, generated.colors);
    close('normals', mesh.normals, generated.normals);
    assert.equal(mesh.triangleCount, generated.triangleCount);
    assert.deepEqual(mesh.parts, generated.parts);
    assert.deepEqual(mesh.specification, generated.specification);
    assert.equal(mesh.description, generated.description);
    for (const key of ['min', 'max']) for (let axis = 0; axis < 3; axis++)
      assert.ok(rounded(mesh.bounds[key][axis], generated.bounds[key][axis]),
        `bounds.${key}[${axis}] imported as ${mesh.bounds[key][axis]} against ${generated.bounds[key][axis]}`);
  });

  test(`${file} survives a further export and import unchanged`, () => {
    // Writer after reader is the identity on real committed bytes, with no
    // generator involved: whatever the reader dropped would show up here.
    const mesh = model(file), again = fromGlb(glb(mesh, 'Re-exported vehicle'));
    identical('positions', again.positions, mesh.positions);
    identical('colors', again.colors, mesh.colors);
    close('normals', again.normals, mesh.normals);
    assert.deepEqual(again.parts, mesh.parts);
    assert.deepEqual(again.bounds, mesh.bounds);
    assert.deepEqual(again.specification, mesh.specification);
    assert.equal(again.description, mesh.description);
    assert.equal(again.triangleCount, mesh.triangleCount);
  });
}

test('a freshly generated mesh exports and imports without loss', () => {
  // The same proof as above, but against whatever the generator produces today
  // rather than against bytes committed earlier, so a generator change cannot
  // leave the reader unproven. One tank and one specialist, plus an upgraded
  // configuration that no committed asset covers.
  const specs = [build(model('spheres-tank-heavy.glb').specification),
    build(model('spheres-ground-artillery.glb').specification),
    build({platform: 'ground_air_defense', components: {armament: 'ground_aa_missiles', ammunition: 'ground_ammo_missiles', radar: 'ground_radar_tracking', fire_control: 'fcs_digital'}})];
  for (const generated of specs) {
    const returned = fromGlb(glb(generated, 'Fresh design'));
    identical('positions', returned.positions, generated.positions);
    identical('colors', returned.colors, generated.colors);
    close('normals', returned.normals, generated.normals);
    assert.deepEqual(returned.parts, generated.parts);
    assert.deepEqual(returned.specification, generated.specification);
    assert.equal(returned.description, generated.description);
    assert.equal(returned.triangleCount, generated.triangleCount);
    wholeParts(returned);
    for (const key of ['min', 'max']) for (let axis = 0; axis < 3; axis++)
      assert.ok(rounded(returned.bounds[key][axis], generated.bounds[key][axis]));
  }
});

test('an imported model picks the same component as the mesh it came from', () => {
  for (const file of ['spheres-tank-heavy.glb', 'spheres-ground-artillery.glb']) {
    const mesh = model(file), again = fromGlb(glb(mesh, 'Re-exported vehicle')), aspect = 1.6;
    // One camera for both, so any difference in a hit is the geometry's fault
    // rather than a rounding in the bounds the camera was framed from.
    const camera = viewer.frame(mesh.bounds, aspect, ...viewer.views.hero, 1);
    let hits = 0, slots = 0;
    for (const x of [-0.3, -0.1, 0, 0.1, 0.3]) for (const y of [-0.2, 0, 0.2]) {
      const ray = viewer.rayAt(camera, x, y, aspect);
      const a = viewer.raycast(mesh, ray.origin, ray.direction), b = viewer.raycast(again, ray.origin, ray.direction);
      assert.equal(Boolean(a), Boolean(b));
      if (!a) continue;
      hits++;
      assert.equal(a.vertex, b.vertex);
      assert.equal(a.distance, b.distance);
      assert.equal(a.part?.name, b.part?.name);
      assert.equal(a.part?.slot, b.part?.slot);
      if (a.part?.slot) slots++;
    }
    assert.ok(hits >= 10, `${file}: only ${hits} of 15 rays reached the model`);
    assert.ok(slots >= 10, `${file}: only ${slots} hits landed on a specification slot`);
    // And the imported bounds frame a usable camera on their own.
    const own = viewer.frame(again.bounds, aspect, ...viewer.views.hero, 1);
    assert.ok([...own.matrix, ...own.eye].every(Number.isFinite));
    own.eye.forEach((value, i) => assert.ok(Math.abs(value - camera.eye[i]) < 1e-5));
  }
});

// A small original fixture, and the split/repack pair the refusal cases mutate.
function fixture() {
  return {
    positions: new Float32Array([-1, 0, 0, 1, 0, 0, 0, 2, 0, -1, 0, 1, 1, 0, 1, 0, 2, 1]),
    normals: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1]),
    colors: new Float32Array([0.2, 0.4, 0.1, 0.2, 0.4, 0.1, 0.2, 0.4, 0.1, 0.3, 0.5, 0.2, 0.3, 0.5, 0.2, 0.3, 0.5, 0.2]),
    parts: [{name: 'Hull', first: 0, count: 3, slot: 'protection', label: 'hull'},
      {name: 'Deck', first: 3, count: 3, slot: 'protection', label: 'deck'}],
    triangleCount: 2,
    description: 'A small original test model.',
    specification: {platform: 'tank_standard', components: {protection: 'protection_standard'}}
  };
}
function split(bytes) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const jsonLength = view.getUint32(12, true);
  const document = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonLength)));
  const binaryLength = view.getUint32(20 + jsonLength, true);
  return {document, binary: bytes.slice(28 + jsonLength, 28 + jsonLength + binaryLength)};
}
// Container-level damage is expressed through these options, so a case can bend
// the wrapper without hand-assembling a second GLB writer.
function pack({document, binary}, {magic = 0x46546c67, version = 2, total = null, jsonPad = 0, bin = true} = {}) {
  const json = new TextEncoder().encode(JSON.stringify(document));
  const jsonLength = ((json.byteLength + 3) & ~3) + jsonPad, binaryLength = (binary.byteLength + 3) & ~3;
  const out = new Uint8Array(20 + jsonLength + (bin ? 8 + binaryLength : 0));
  const view = new DataView(out.buffer);
  view.setUint32(0, magic, true); view.setUint32(4, version, true);
  view.setUint32(8, total === null ? out.byteLength : total, true);
  view.setUint32(12, jsonLength, true); view.setUint32(16, 0x4e4f534a, true);
  out.fill(0x20, 20, 20 + jsonLength); out.set(json, 20);
  if (bin) {
    view.setUint32(20 + jsonLength, binaryLength, true);
    view.setUint32(24 + jsonLength, 0x004e4942, true);
    out.set(binary, 28 + jsonLength);
  }
  return out;
}
const original = glb(fixture(), 'Fixture vehicle');
// Three equal, non-interleaved runs: positions, then normals, then colors.
const floats = model => ({
  view: new DataView(model.binary.buffer, model.binary.byteOffset, model.binary.byteLength),
  stride: model.binary.byteLength / 3
});

test('the fixture and its repack import cleanly, so a refusal below is the mutation talking', () => {
  const direct = fromGlb(original), repacked = fromGlb(pack(split(original)));
  assert.deepEqual(repacked, direct);
  assert.deepEqual(direct.positions, fixture().positions);
  assert.deepEqual(direct.colors, fixture().colors);
  assert.deepEqual(direct.parts, fixture().parts);
  assert.deepEqual(direct.specification, fixture().specification);
  assert.deepEqual(direct.bounds, {min: [-1, 0, 0], max: [1, 2, 1]});
  assert.equal(direct.triangleCount, 2);
  assert.equal(direct.description, 'A small original test model.');
});

// Every refusal this reader owes the pipeline, each asserted to name its cause.
const REFUSALS = [
  ['an indexed primitive', m => { m.document.meshes[0].primitives[0].indices = 3; }, /indexed/],
  ['a second primitive', m => { m.document.meshes[0].primitives.push({...m.document.meshes[0].primitives[0]}); }, /2 primitives/],
  ['a second material', m => { m.document.materials.push({name: 'Second'}); }, /2 materials/],
  ['a second mesh', m => { m.document.meshes.push({...m.document.meshes[0]}); }, /2 meshes/],
  ['an interleaved buffer view', m => { m.document.bufferViews[0].byteStride = 36; }, /byteStride of 36/],
  ['a non-VEC3 accessor', m => { m.document.accessors[2].type = 'VEC4'; }, /not VEC3/],
  ['a non-FLOAT accessor', m => { m.document.accessors[1].componentType = 5123; }, /not FLOAT/],
  ['a normalized accessor', m => { m.document.accessors[2].normalized = true; }, /normalized/],
  ['a sparse accessor', m => { m.document.accessors[0].sparse = {count: 1}; }, /sparse/],
  ['a node matrix', m => { m.document.nodes[0].matrix = [1,0,0,0, 0,1,0,0, 0,0,1,0, 0,3.5,0,1]; }, /non-identity matrix/],
  ['a node translation', m => { m.document.nodes[0].translation = [0, 1.5, 0]; }, /non-identity translation/],
  ['a node rotation', m => { m.document.nodes[0].rotation = [0, 0.7071, 0, 0.7071]; }, /non-identity rotation/],
  ['a node scale', m => { m.document.nodes[0].scale = [1, 1, 2]; }, /non-identity scale/],
  ['a node hierarchy', m => { m.document.nodes[0].children = [1]; m.document.nodes.push({name: 'Turret pivot'}); }, /2 nodes/],
  ['a parented node', m => { m.document.nodes[0].children = [0]; }, /child node/],
  ['a scene that draws something else', m => { m.document.scenes[0].nodes = []; }, /default scene/],
  ['texture coordinates', m => { m.document.meshes[0].primitives[0].attributes.TEXCOORD_0 = 0; }, /TEXCOORD_0/],
  ['tangents', m => { m.document.meshes[0].primitives[0].attributes.TANGENT = 0; }, /TANGENT/],
  ['skinning attributes', m => { const a = m.document.meshes[0].primitives[0].attributes; a.JOINTS_0 = 0; a.WEIGHTS_0 = 0; }, /JOINTS_0/],
  ['a missing attribute', m => { delete m.document.meshes[0].primitives[0].attributes.COLOR_0; }, /no COLOR_0 attribute/],
  ['an image', m => { m.document.images = [{uri: 'camouflage.png'}]; }, /images list/],
  ['a texture', m => { m.document.textures = [{source: 0}]; }, /textures list/],
  ['a sampler', m => { m.document.samplers = [{magFilter: 9729}]; }, /samplers list/],
  ['an animation', m => { m.document.animations = [{channels: [], samplers: []}]; }, /animations list/],
  ['a skin', m => { m.document.skins = [{joints: [0]}]; }, /skins list/],
  ['a required extension', m => { m.document.extensionsRequired = ['KHR_draco_mesh_compression']; }, /KHR_draco_mesh_compression/],
  ['a used extension', m => { m.document.extensionsUsed = ['KHR_materials_clearcoat']; }, /KHR_materials_clearcoat/],
  ['a root extension block', m => { m.document.extensions = {KHR_lights_punctual: {lights: []}}; }, /extension block/],
  ['a primitive extension block', m => { m.document.meshes[0].primitives[0].extensions = {KHR_draco_mesh_compression: {}}; }, /KHR_draco_mesh_compression/],
  ['a part range that is not whole triangles', m => { m.document.meshes[0].extras.parts[1].first = 4; }, /whole number of triangles/],
  ['a part count that is not whole triangles', m => { m.document.meshes[0].extras.parts[0].count = 2; }, /whole number of triangles/],
  ['a part running past the vertices', m => { m.document.meshes[0].extras.parts[1].count = 9; }, /past the uploaded buffer/],
  ['a negative part range', m => { m.document.meshes[0].extras.parts[0].first = -3; }, /not a vertex range/],
  ['an unnamed part', m => { delete m.document.meshes[0].extras.parts[0].name; }, /has no name/],
  ['a triangle strip', m => { m.document.meshes[0].primitives[0].mode = 5; }, /a triangle strip/],
  ['mismatched attribute counts', m => { m.document.accessors[1].count = 3; }, /must describe the same vertices/],
  ['an external buffer', m => { m.document.buffers[0].uri = 'geometry.bin'; }, /external resource/],
  ['a buffer view past the chunk', m => { m.document.bufferViews[2].byteOffset = 8192; }, /past the end of the/],
  ['missing POSITION bounds', m => { delete m.document.accessors[0].min; }, /no usable min/],
  ['bounds that exclude the geometry', m => { m.document.accessors[0].min = [0, 0, 0]; }, /do not contain the geometry/],
  ['a stale triangle count', m => { m.document.meshes[0].extras.triangleCount = 7; }, /triangles but the geometry holds/],
  ['a non-finite position', m => { floats(m).view.setFloat32(4, NaN, true); }, /POSITION holds NaN/],
  ['a colour outside the unit range', m => { const {view, stride} = floats(m); view.setFloat32(stride * 2 + 4, 1.5, true); }, /outside 0\.\.1/],
  // Found by driving the reader against the writer's own bars rather than by
  // reading it: a zero normal imported, failed the |n|=1 check above, and then
  // threw a bare TypeError out of EquipmentExport.glb when the model was saved.
  ['a normal with no direction', m => { const {view, stride} = floats(m); for (let a = 0; a < 3; a++) view.setFloat32(stride + a * 4, 0, true); }, /NORMAL is zero at vertex 0/],
  ['a glTF 1.0 container', m => { m.options.version = 1; }, /container version 1/],
  ['a missing BIN chunk', m => { m.options.bin = false; }, /no BIN chunk/],
  ['an unpadded chunk', m => { m.options.jsonPad = 2; }, /4-byte padding/],
  ['a wrong declared length', m => { m.options.total = 40; }, /header declares 40/],
  ['a file that is not a GLB', m => { m.options.magic = 0x004d4c47; }, /magic/]
];
for (const [what, change, expected] of REFUSALS) {
  test(`refuses ${what} rather than importing part of it`, () => {
    const damaged = {...split(original), options: {}};
    change(damaged);
    assert.throws(() => fromGlb(pack(damaged, damaged.options)), error => {
      assert.ok(error instanceof Error);
      assert.match(error.message, /^Cannot import this GLB: /);
      assert.match(error.message, expected);
      return true;
    });
  });
}

test('refuses a truncated file and anything that is not GLB bytes', () => {
  assert.throws(() => fromGlb(original.subarray(0, 16)), /shorter than a 12-byte GLB header/);
  for (const bad of [null, undefined, 'a-model.glb', 42, {byteLength: 100}])
    assert.throws(() => fromGlb(bad), TypeError);
});

test('reads an ArrayBuffer and a view that does not start on a 4-byte boundary', () => {
  const expected = fromGlb(original);
  assert.deepEqual(fromGlb(original.buffer.slice(original.byteOffset, original.byteOffset + original.byteLength)), expected);
  // A GLB sliced out of a larger download, or out of a pooled Node buffer, can
  // start at any byte; a Float32Array view over the chunk would throw on that.
  const shifted = new Uint8Array(original.byteLength + 3).subarray(3);
  shifted.set(original);
  assert.equal(shifted.byteOffset % 4, 3);
  assert.deepEqual(fromGlb(shifted), expected);
});

test('browser UMD API reads the same model as the Node API', () => {
  const context = vm.createContext({TextDecoder, Uint8Array, Float32Array, DataView, ArrayBuffer});
  vm.runInContext(fs.readFileSync(source, 'utf8'), context);
  assert.equal(typeof context.EquipmentImport.fromGlb, 'function');
  const browser = context.EquipmentImport.fromGlb(original), node = fromGlb(original);
  // Compared through plain values: the context's arrays and objects carry its
  // own prototypes, which a strict deep comparison rejects on sight.
  for (const field of ['positions', 'normals', 'colors'])
    assert.deepEqual(Array.from(browser[field]), Array.from(node[field]));
  for (const field of ['parts', 'bounds', 'specification'])
    assert.equal(JSON.stringify(browser[field]), JSON.stringify(node[field]));
  assert.equal(browser.triangleCount, node.triangleCount);
  assert.equal(browser.description, node.description);
  assert.throws(() => context.EquipmentImport.fromGlb(new Uint8Array(64)), /Cannot import this GLB/);
});
