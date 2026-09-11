// Decode the actual GLB bytes: alignment, geometry, bounds, and portable metadata.
const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const file = path.resolve(__dirname, '../../spheres-web/ui/equipment-export.js');
const {glb} = require(file);

function triangle() {
  return {
    positions: new Float32Array([-1, 0, 0, 1, 0, 0, 0, 2, 0]),
    normals: new Float32Array([0, 0, 2, 0, 0, 2, 0, 0, 2]),
    colors: new Float32Array([0.2, 0.4, 0.1, 0.2, 0.4, 0.1, 0.2, 0.4, 0.1]),
    bounds: {min: [-1, 0, 0], max: [1, 2, 0]},
    parts: [{name: 'Hull', first: 0, count: 3}],
    triangleCount: 1,
    description: 'A small original test model.'
  };
}

function decode(bytes) {
  assert.ok(bytes instanceof Uint8Array);
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  assert.equal(view.getUint32(0, true), 0x46546c67);
  assert.equal(view.getUint32(4, true), 2);
  assert.equal(view.getUint32(8, true), bytes.byteLength);
  const jsonLength = view.getUint32(12, true);
  assert.equal(jsonLength % 4, 0);
  assert.equal(view.getUint32(16, true), 0x4e4f534a);
  const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonLength)));
  const binaryLength = view.getUint32(20 + jsonLength, true);
  assert.equal(binaryLength % 4, 0);
  assert.equal(view.getUint32(24 + jsonLength, true), 0x004e4942);
  assert.equal(28 + jsonLength + binaryLength, bytes.byteLength);
  assert.equal(json.asset.version, '2.0');
  assert.equal(json.buffers.length, 1);
  assert.equal(json.buffers[0].uri, undefined);
  assert.equal(json.buffers[0].byteLength, binaryLength);
  const binary = new DataView(bytes.buffer, bytes.byteOffset + 28 + jsonLength, binaryLength);
  return {json, binary, values(index) {
    const accessor = json.accessors[index];
    const bufferView = json.bufferViews[accessor.bufferView];
    assert.equal(accessor.componentType, 5126);
    assert.equal(accessor.type, 'VEC3');
    assert.equal(bufferView.byteOffset % 4, 0);
    assert.equal(bufferView.target, 34962);
    assert.equal(bufferView.byteLength, accessor.count * 3 * 4);
    assert.ok(bufferView.byteOffset + bufferView.byteLength <= binary.byteLength);
    return Array.from({length: accessor.count * 3}, (_, i) => binary.getFloat32(bufferView.byteOffset + i * 4, true));
  }};
}

test('GLB contains aligned embedded geometry, matte material, and preserved colors', () => {
  const mesh = triangle(), model = decode(glb(mesh, 'Sentinel 90'));
  assert.equal(model.json.nodes[0].name, 'Sentinel 90');
  assert.equal(model.json.scenes[model.json.scene].nodes[0], 0);
  assert.deepEqual(model.json.meshes[0].primitives[0], {attributes: {POSITION: 0, NORMAL: 1, COLOR_0: 2}, material: 0, mode: 4});
  assert.deepEqual(model.values(0), Array.from(mesh.positions));
  assert.deepEqual(model.values(2), Array.from(mesh.colors));
  assert.deepEqual(model.json.accessors[0].min, mesh.bounds.min);
  assert.deepEqual(model.json.accessors[0].max, mesh.bounds.max);
  assert.equal(model.json.materials[0].pbrMetallicRoughness.metallicFactor, 0);
  assert.deepEqual(model.json.materials[0].pbrMetallicRoughness.baseColorFactor, [1, 1, 1, 1]);
  assert.deepEqual(model.json.meshes[0].extras.parts, mesh.parts);
  assert.equal(model.json.meshes[0].extras.triangleCount, 1);
  assert.equal(model.json.meshes[0].extras.description, mesh.description);
});

test('normalizes normals without changing the source mesh', () => {
  const mesh = triangle(), before = new Float32Array(mesh.normals);
  assert.deepEqual(decode(glb(mesh)).values(1), [0, 0, 1, 0, 0, 1, 0, 0, 1]);
  assert.deepEqual(mesh.normals, before);
});

test('derives accessor bounds from exported positions instead of stale display bounds', () => {
  const mesh = triangle();
  mesh.bounds = {min: [100, 100, 100], max: [101, 101, 101]};
  const {json} = decode(glb(mesh));
  assert.deepEqual(json.accessors[0].min, [-1, 0, 0]);
  assert.deepEqual(json.accessors[0].max, [1, 2, 0]);
});

test('preserves Unicode names and deterministic export bytes', () => {
  const mesh = triangle(), name = 'Sentinelle — 重型';
  assert.equal(decode(glb(mesh, name)).json.nodes[0].name, name);
  assert.deepEqual(glb(mesh, name), glb(mesh, name));
});

for (const field of ['positions', 'normals', 'colors']) {
  test(`rejects non-finite ${field} rather than writing an invalid model`, () => {
    for (const bad of [NaN, Infinity, -Infinity]) {
      const mesh = triangle(); mesh[field][0] = bad;
      assert.throws(() => glb(mesh), /finite/);
    }
  });
}

test('rejects incomplete geometry, mismatched counts, and non-float attributes', () => {
  const mesh = triangle();
  assert.throws(() => glb(null), /required/);
  assert.throws(() => glb({...mesh, positions: new Float32Array(0)}), /triangles/);
  assert.throws(() => glb({...mesh, positions: new Float32Array(6)}), /triangles/);
  assert.throws(() => glb({...mesh, normals: new Float32Array(6)}), /matching/);
  assert.throws(() => glb({...mesh, colors: new Uint32Array(9)}), /Float32/);
  assert.throws(() => glb({...mesh, colors: Array.from(mesh.colors)}), /Float32/);
  assert.throws(() => glb({...mesh, triangleCount: 2}), /count/);
});

test('rejects impossible colors, zero normals, and invalid semantic ranges', () => {
  const mesh = triangle();
  mesh.colors[0] = 1.01; assert.throws(() => glb(mesh), /colors/);
  mesh.colors[0] = -0.01; assert.throws(() => glb(mesh), /colors/);
  mesh.colors[0] = 0; mesh.normals.fill(0); assert.throws(() => glb(mesh), /nonzero/);
  for (const part of [{name: 'Hull', first: 1, count: 2}, {name: 'Hull', first: 0, count: 6}, {name: 'Hull', first: -3, count: 3}]) {
    assert.throws(() => glb({...triangle(), parts: [part]}), /part ranges/);
  }
});

test('browser UMD API exports the same binary as the Node API', () => {
  const context = vm.createContext({TextEncoder, Uint8Array, DataView, ArrayBuffer, Float32Array});
  vm.runInContext(fs.readFileSync(file, 'utf8'), context);
  assert.equal(typeof context.EquipmentExport.glb, 'function');
  assert.deepEqual(context.EquipmentExport.glb(triangle(), 'Browser tank'), glb(triangle(), 'Browser tank'));
});

test('generated models for every vehicle type contain complete portable meshes and all twelve specifications', () => {
  const folder = path.resolve(__dirname, '../../spheres-web/ui/equipment-models');
  const results = ['balanced', 'mobile', 'heavy', 'light', 'destroyer'].map(id => {
    const bytes = fs.readFileSync(path.join(folder, `spheres-tank-${id}.glb`));
    // Inspection detail has its own 12 MB budget; map meshes retain their smaller caps.
    assert.ok(bytes.byteLength < 12000000, `${id} exceeds the inspection asset budget`);
    const model = decode(bytes);
    const positions = model.values(0), normals = model.values(1), colors = model.values(2);
    assert.ok(positions.length > 1000, 'The vehicle should contain modeled details, not a placeholder triangle.');
    assert.equal(positions.length, normals.length);
    assert.equal(positions.length, colors.length);
    assert.equal(positions.length % 9, 0);
    assert.ok(positions.every(Number.isFinite));
    assert.ok(colors.every(value => Number.isFinite(value) && value >= 0 && value <= 1));
    for (let index = 0; index < normals.length; index += 3)
      assert.ok(Math.abs(Math.hypot(...normals.slice(index, index + 3)) - 1) < 0.00001);
    assert.ok(model.json.meshes[0].extras.parts.length > 5);
    assert.equal(Object.keys(model.json.meshes[0].extras.specification.components).length,12);
    assert.ok(model.json.meshes[0].extras.specification.components.ammunition);
    return positions;
  });
  assert.notDeepEqual(results[0], results[1]);
  assert.notDeepEqual(results[0], results[2]);
});

test('specialist GLB assets preserve complete geometry, all thirteen slots and pickable semantic parts',()=>{
  const folder=path.resolve(__dirname,'../../spheres-web/ui/equipment-models');
  const expected=[['ifv','ground_ifv','troop_compartment'],['apc','ground_apc','troop_compartment'],['recon','ground_recon','recon_package'],['artillery','ground_artillery','artillery_loader'],['air-defense','ground_air_defense','radar']];
  for(const [id,platform,mission] of expected){
    const bytes=fs.readFileSync(path.join(folder,`spheres-ground-${id}.glb`)),model=decode(bytes),extras=model.json.meshes[0].extras;
    assert(bytes.byteLength>500000&&bytes.byteLength<5000000);
    assert.equal(extras.specification.platform,platform);assert.equal(Object.keys(extras.specification.components).length,13);assert(extras.specification.components[mission]);
    const positions=model.values(0),normals=model.values(1),colors=model.values(2);
    assert.equal(positions.length,extras.triangleCount*9);assert(positions.every(Number.isFinite));assert.equal(positions.length,normals.length);assert.equal(positions.length,colors.length);
    assert(colors.every(v=>v>=0&&v<=1));for(let i=0;i<normals.length;i+=3)assert(Math.abs(Math.hypot(normals[i],normals[i+1],normals[i+2])-1)<1e-5);
    let end=0;for(const part of extras.parts){assert.equal(part.first,end);assert.equal(part.count%3,0);assert(part.count>0);assert(part.slot in extras.specification.components);assert(part.label);end+=part.count;}
    assert.equal(end,positions.length/3);assert(extras.parts.some(p=>p.slot===mission));
  }
});

test('an upgraded specialist exports its current mission components and semantic slot labels',()=>{
  const {build}=require('../../spheres-web/ui/equipment-mesh.js');
  const mesh=build({platform:'ground_air_defense',components:{armament:'ground_aa_missiles',ammunition:'ground_ammo_missiles',radar:'ground_radar_tracking',fire_control:'fcs_digital'}});
  const exported=decode(glb(mesh,'Coastal defense revision')).json.meshes[0].extras;
  assert.deepEqual(exported.specification,mesh.specification);assert.deepEqual(exported.parts,mesh.parts);
  assert.equal(exported.specification.components.armament,'ground_aa_missiles');assert.equal(exported.specification.components.radar,'ground_radar_tracking');
});

test('both aircraft GLBs preserve generated geometry, eight specifications and semantic picking ranges',()=>{
  const {build}=require('../../spheres-web/ui/equipment-mesh.js');
  for(const [id,platform] of [['light-attack','air_light_attack'],['tactical-strike','air_tactical_strike']]){
    const mesh=build({platform}),bytes=fs.readFileSync(path.resolve(__dirname,`../../spheres-web/ui/equipment-models/spheres-air-${id}.glb`)),model=decode(bytes),extras=model.json.meshes[0].extras;
    // 100k+ inspection aircraft use 108 bytes per triangle plus GLB metadata.
    assert(bytes.byteLength>10800000&&bytes.byteLength<28000000);assert.deepEqual(extras.specification,mesh.specification);assert.deepEqual(extras.parts,mesh.parts);
    assert.equal(Object.keys(extras.specification.components).length,8);
    assert.deepEqual(model.values(0),Array.from(mesh.positions));assert.deepEqual(model.values(2),Array.from(mesh.colors));
    const normals=model.values(1);assert(normals.every((n,i)=>Math.abs(n-mesh.normals[i])<1e-7),'export normalization preserves surface orientation');
  }
  const upgraded=build({platform:'air_tactical_strike',components:{air_avionics:'air_avionics_digital',air_payload:'air_payload_guided',air_fuel:'air_fuel_extended'}}),out=decode(glb(upgraded,'Configured strike aircraft'));
  assert.deepEqual(out.json.meshes[0].extras.specification,upgraded.specification);assert.deepEqual(out.json.meshes[0].extras.parts,upgraded.parts);
  assert(out.json.meshes[0].extras.parts.some(p=>p.slot==='air_payload'&&p.label.includes('precision')));
});
