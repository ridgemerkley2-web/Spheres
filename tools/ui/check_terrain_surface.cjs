// Run: node --test tools/ui/check_terrain_surface.cjs
// Exercises the shipped CPU terrain code. GPU shader compilation and the
// complete visual result are additionally checked in the running preview.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const ui = path.resolve(__dirname, '../../spheres-web/ui');
const source = fs.readFileSync(path.join(ui, 'terrain-surface.js'), 'utf8');
const globeSource = fs.readFileSync(path.join(ui, 'globe3d.js'), 'utf8');
function fixture(extra = {}) {
  const warnings = [], c = vm.createContext({window: {}, Float32Array, Uint32Array, AbortController,
    console: {warn(...values) {warnings.push(values.join(' '));}}, ...extra});
  vm.runInContext(source, c, {filename: 'terrain-surface.js'});
  vm.runInContext(globeSource, c, {filename: 'globe3d.js'});
  return {api: c.window.TerrainSurface, Globe: c.window.Globe3D, warnings, context: c};
}
const api = fixture().api;
const plain = value => JSON.parse(JSON.stringify(value));
const tick = () => new Promise(resolve => setImmediate(resolve));
async function settled(cache) {
  for (let i = 0; i < 100; i++) { await tick(); if (!cache.pending.size && !cache.queue.length) return; }
  throw new Error('tile queue did not settle');
}
function tile(x, y) {return {x, y, key: x + ':' + y};}
function planeTile(x, y, fn) {
  const values = new Float32Array(api.tileSize ** 2);
  for (let row = 0; row < api.tileSize; row++) for (let col = 0; col < api.tileSize; col++)
    values[row * api.tileSize + col] = fn(-180 + x * 10 + (col - .5) / 60, 90 - y * 10 - (row - .5) / 60);
  return values;
}
function camera(f, lon, lat, zoom, tilted = true) {
  const globe = Object.create(f.Globe.prototype);
  globe.yaw = -lon * Math.PI / 180; globe.pitch = lat * Math.PI / 180; globe.zoom = zoom;
  globe.options = {terrainEnabled: () => true, tilted: () => tilted};
  return globe.view({width: 1280, height: 720, aspect: 1280 / 720, ratio: 1});
}
function glFixture() {
  const constants = ['VERTEX_ARRAY_BINDING', 'ARRAY_BUFFER_BINDING', 'CURRENT_PROGRAM', 'ARRAY_BUFFER',
    'ELEMENT_ARRAY_BUFFER', 'DYNAMIC_DRAW', 'FLOAT', 'TRIANGLES', 'UNSIGNED_INT'];
  const gl = Object.fromEntries(constants.map(key => [key, key]));
  const original = {vao: {old: 'vao'}, array: {old: 'array'}, program: {old: 'program'}};
  const state = {...original}, calls = [], live = new Set();
  const make = kind => {const resource = {kind}; live.add(resource); return resource;};
  Object.assign(gl, {
    getParameter(key) {return state[{VERTEX_ARRAY_BINDING: 'vao', ARRAY_BUFFER_BINDING: 'array', CURRENT_PROGRAM: 'program'}[key]];},
    createVertexArray() {return make('vao');}, createBuffer() {return make('buffer');},
    deleteVertexArray(value) {live.delete(value);}, deleteBuffer(value) {live.delete(value);},
    bindVertexArray(value) {state.vao = value;}, bindBuffer(target, value) {if (target === gl.ARRAY_BUFFER) state.array = value;},
    bufferData(target, values, usage) {calls.push({op: 'upload', target, values, usage});},
    enableVertexAttribArray(location) {calls.push({op: 'enableAttribute', location});},
    vertexAttribPointer(...args) {calls.push({op: 'attribute', args});},
    useProgram(value) {state.program = value;}, getUniformLocation(program, name) {return {program, name};},
    uniformMatrix3fv(location, transpose, value) {calls.push({op: 'uniform', location, value: [...value], transpose});},
    uniform3fv(location, value) {calls.push({op: 'uniform', location, value: [...value]});},
    uniform2fv(location, value) {calls.push({op: 'uniform', location, value: [...value]});},
    drawElements(mode, count, type, offset) {calls.push({op: 'draw', mode, count, type, offset, program: state.program, vao: state.vao});},
  });
  return {gl, calls, live, restored() {assert.deepEqual(state, original);}};
}

test('RG16 decode preserves signed metres, byte rollovers and dimension checks', () => {
  const pixels = new Uint8ClampedArray(api.tileSize ** 2 * 4), codes = [0, 1, 255, 256, 32768, 65535];
  codes.forEach((code, i) => {pixels[i * 4] = code >> 8; pixels[i * 4 + 1] = code & 255; pixels[i * 4 + 2] = 217; pixels[i * 4 + 3] = 255;});
  const values = api.decodePixels(pixels);
  codes.forEach((code, i) => assert(Math.abs(values[i] - (-1500 + code * 10500 / 65535)) < .0005));
  assert(values[3] > values[2]); assert.equal(values[0], -1500); assert.equal(values[5], 9000);
  assert.throws(() => api.decodePixels(pixels, 600, 600), /dimensions/);
  assert.throws(() => api.decodePixels(pixels.subarray(4)), /dimensions/);
});

test('tile coordinates use north-up cell centres and wrap the dateline', () => {
  assert.deepEqual(plain(api.tileAt(-180, 90)), {x: 0, y: 0, key: '0:0', px: .5, py: .5});
  assert.deepEqual(plain(api.tileAt(180, 90)), plain(api.tileAt(-180, 90)));
  assert.deepEqual(plain(api.tileAt(-175, 85)), {x: 0, y: 0, key: '0:0', px: 300.5, py: 300.5});
  assert.equal(api.tileAt(0, -90).y, 17); assert.equal(api.tileAt(0, -90).py, 600.5);
  assert.equal(api.tilePath(8, 3), '/terrain-tiles/x08_y03.png');
});

test('shared gutters interpolate a continuous surface across longitude and latitude seams', () => {
  const fn = (lon, lat) => 2000 + lon * 7 - lat * 13;
  const cache = new api.TileCache();
  for (const x of [17, 18]) for (const y of [8, 9]) cache.tiles.set(x + ':' + y, planeTile(x, y, fn));
  for (const lon of [-.01, 0, .01]) for (const lat of [-.01, 0, .01]) {
    assert(Math.abs(cache.sample(lon, lat) - fn(lon, lat)) < .001, lon + ', ' + lat);
    const expected = [fn(lon, lat), fn(lon + 1 / 60, lat), fn(lon - 1 / 60, lat), fn(lon, lat - 1 / 60), fn(lon, lat + 1 / 60)];
    cache.sampleSurface(lon, lat).forEach((value, i) => assert(Math.abs(value - expected[i]) < .001, 'slope neighbour ' + i));
  }
  assert.equal(cache.sample(90, 0), null, 'missing data is explicit rather than synthetic elevation');
  cache.dispose();
});

test('mesh vertices physically rise for peaks and fall into valleys', () => {
  const mesh = api.buildMesh({west: -.1, east: .1, south: -.1, north: .1},
    (lon, lat) => Math.abs(lon) < .035 && Math.abs(lat) < .035 ? 7000 : 500, {maxSegments: 16, exaggeration: 3});
  let low = Infinity, high = 0;
  for (let i = 0; i < mesh.vertices.length; i += 6) {
    const r = Math.hypot(mesh.vertices[i], mesh.vertices[i + 1], mesh.vertices[i + 2]);
    low = Math.min(low, r); high = Math.max(high, r);
  }
  assert(Math.abs((high - low) * api.earthMetres - 6500 * 3) < 1, 'height is in geometry, not only fragment colour');
  assert.equal(mesh.minHeight, 500); assert.equal(mesh.maxHeight, 7000);
});

test('native slope retains east/south signs and latitude-dependent ground distance', () => {
  const mesh = api.buildMesh({west: -.1, east: .1, south: 59.9, north: 60.1},
    (lon, lat) => 2500 + lon * 100 - (lat - 60) * 200, {maxSegments: 16});
  const row = Math.floor(mesh.rows / 2), col = Math.floor(mesh.columns / 2), offset = (row * (mesh.columns + 1) + col) * 6;
  const metresPerDegree = Math.PI / 180 * api.earthMetres;
  assert(Math.abs(mesh.vertices[offset + 4] - 100 / (metresPerDegree * .5)) < 1e-9);
  assert(Math.abs(mesh.vertices[offset + 5] - 200 / metresPerDegree) < 1e-9);
});

test('ocean bathymetry stays at sea level without inverted seafloor geometry', () => {
  const mesh = api.buildMesh({west: -.1, east: .1, south: -.1, north: .1}, () => -1500, {maxSegments: 16});
  for (let i = 0; i < mesh.vertices.length; i += 6) {
    assert(Math.abs(Math.hypot(...mesh.vertices.slice(i, i + 3)) - 1) < 1e-7);
    assert.equal(mesh.vertices[i + 3], -1500); assert.equal(mesh.vertices[i + 4], 0); assert.equal(mesh.vertices[i + 5], 0);
  }
});

test('mesh triangles face outward and share their edges', () => {
  const mesh = api.buildMesh({west: -.2, east: .2, south: -.2, north: .2}, () => 1000, {maxSegments: 16});
  for (let i = 0; i < mesh.indices.length; i += 3) {
    const p = [...mesh.indices.slice(i, i + 3)].map(v => [...mesh.vertices.slice(v * 6, v * 6 + 3)]);
    const a = p[1].map((v, j) => v - p[0][j]), b = p[2].map((v, j) => v - p[0][j]);
    const n = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    assert(n.reduce((sum, v, j) => sum + v * p[0][j], 0) > 0);
  }
  assert.equal(mesh.indices.length, mesh.rows * mesh.columns * 6);
  assert.equal(Math.max(...mesh.indices), mesh.vertices.length / 6 - 1);
});

test('native spacing is retained nearby and geometry allocation is capped for broad views', () => {
  const local = api.buildMesh({west: 0, east: 2, south: 0, north: 1}, () => 0);
  assert.equal(local.columns, 120); assert.equal(local.rows, 60);
  const wide = api.buildMesh({west: -30, east: 30, south: -20, north: 20}, () => 0);
  assert.equal(wide.columns, 256); assert.equal(wide.rows, 256);
  assert.equal(wide.vertices.length / 6, 257 ** 2); assert.equal(wide.indices.length / 3, 256 ** 2 * 2);
  assert(wide.vertices.byteLength + wide.indices.byteLength < 3.2 * 1024 ** 2);
});

test('missing elevation or gradient coverage declines the complete mesh', () => {
  assert.equal(api.buildMesh({west: 0, east: 1, south: 0, north: 1}, (lon, lat) => lon > 1 ? null : 100), null);
  assert.equal(api.buildMesh({west: 0, east: 1, south: 0, north: 1}, () => NaN), null);
});

test('tilted camera footprint includes the skyline where screen corners see sky', () => {
  const f = fixture(), view = camera(f, 85, 28, 24), bounds = f.api.footprint(view);
  assert(bounds.north > 40, 'horizon bisection retains distant visible ground');
  assert(bounds.south < 28); assert(bounds.west < 85 && bounds.east > 85);
  const close = f.api.footprint(camera(f, 85, 28, 192));
  assert(close.east - close.west < 3, 'close zoom reaches native regional detail');
});

test('dateline footprints stay regional and request both neighbouring tile columns', () => {
  const f = fixture(), view = camera(f, 179.8, 0, 192, false);
  const box = f.api.paddedBounds(f.api.footprint(view)), tiles = f.api.tilesForBounds(box);
  assert(box.east - box.west < 5); assert(box.east > 180 && box.west < 180);
  assert(tiles.some(t => t.x === 35)); assert(tiles.some(t => t.x === 0));
  assert.equal(new Set(tiles.map(t => t.key)).size, tiles.length);
  assert(tiles.length <= 4);
});

test('cache bounds concurrent requests, replaces stale queues and disposes pending loads', async () => {
  const promises = new Map(), started = [];
  const cache = new api.TileCache({concurrency: 2, loadTile(x, y, signal) {
    started.push([x, y, signal]); return new Promise(resolve => promises.set(x + ':' + y, resolve));
  }});
  cache.require([tile(0, 0), tile(1, 0), tile(2, 0), tile(3, 0)]); await tick();
  assert.equal(started.length, 2); assert.equal(cache.pending.size, 2);
  cache.require([tile(4, 0)]);
  promises.get('0:0')(new Float32Array(api.tileSize ** 2)); await tick();
  assert.equal(started.length, 3); assert.equal(started[2][0], 4, 'old queued tiles were replaced');
  cache.dispose(); assert(started[1][2].aborted); assert(started[2][2].aborted);
  promises.get('1:0')(new Float32Array(api.tileSize ** 2)); promises.get('4:0')(new Float32Array(api.tileSize ** 2));
  await tick(); assert.equal(cache.tiles.size, 0);
});

test('cache evicts older regions and never exceeds its decoded memory budget', async () => {
  const cache = new api.TileCache({cacheLimit: 4, loadTile: async () => new Float32Array(api.tileSize ** 2)});
  cache.require([tile(0, 0), tile(1, 0), tile(2, 0), tile(3, 0)]); await settled(cache);
  assert.equal(cache.tiles.size, 4);
  cache.require([tile(4, 0), tile(5, 0), tile(6, 0), tile(7, 0)]); await settled(cache);
  assert.equal(cache.tiles.size, 4); assert([...cache.tiles.keys()].every(key => Number(key.split(':')[0]) >= 4));
  cache.dispose();
});

test('failed tiles retain ordinary globe fallback without repeated request storms', async () => {
  const f = fixture(); let requests = 0;
  const cache = new f.api.TileCache({loadTile: async () => {requests++; throw new Error('HTTP404');}});
  assert.equal(cache.require([tile(0, 0)]), false); await settled(cache);
  assert.equal(cache.require([tile(0, 0)]), false); await settled(cache);
  assert.equal(requests, 1); assert.equal(f.warnings.length, 1); assert.equal(cache.sample(-175, 85), null);
  cache.dispose();
});

test('mesh becomes drawable only after coverage and restores caller GL bindings', async () => {
  const f = fixture(), gpu = glFixture(); let requests = 0, changes = 0;
  const surface = f.api.create(gpu.gl, {loadTile: async () => {requests++; return new Float32Array(api.tileSize ** 2).fill(1000);}, onChange() {changes++;}});
  const view = camera(f, 85, 28, 192), program = {mesh: true};
  assert.equal(surface.update({...view, zoom: 4}), false); assert.equal(requests, 0);
  assert.equal(surface.update(view), false); assert.equal(surface.draw(program, view), false);
  for (let i = 0; i < 15 && !surface.update(view); i++) await tick();
  assert(surface.ready); assert(changes > 0); assert(surface.stats.vertices > 1000);
  assert.equal(surface.draw(program, view), true); gpu.restored();
  const draw = gpu.calls.find(call => call.op === 'draw');
  assert.equal(draw.program, program); assert.equal(draw.type, gpu.gl.UNSIGNED_INT);
  assert.deepEqual(gpu.calls.find(call => call.op === 'uniform' && call.location.name === 'uCamera').value, [...view.camera]);
  const uploads = gpu.calls.filter(call => call.op === 'upload').length;
  assert(surface.update(view)); assert.equal(gpu.calls.filter(call => call.op === 'upload').length, uploads, 'stationary view does not rebuild');
  assert.equal(surface.sampleHeight(85, 28), 1000);
  surface.dispose(); assert.equal(gpu.live.size, 0); assert.equal(surface.draw(program, view), false);
});

test('constant manifest tiles decode locally without fetching nonexistent PNGs', async () => {
  const fetched = [], entries = {};
  for (let y = 0; y < 18; y++) for (let x = 0; x < 36; x++) {
    const key = 'x' + String(x).padStart(2, '0') + '_y' + String(y).padStart(2, '0');
    entries[key] = {x, y, file: null, constant_code: 10000};
  }
  const f = fixture({fetch: async url => {fetched.push(url); return {ok: true, json: async () => ({
    grid: {columns: 36, rows: 18, tile_degrees: 10, interior_pixels: 600, gutter_pixels: 1, png_pixels: 602, samples_per_degree: 60}, tiles: entries})};}});
  const cache = new f.api.TileCache(); cache.require([tile(18, 9), tile(19, 9)]); await settled(cache);
  assert.equal(cache.tiles.size, 2); assert.deepEqual(fetched, ['/terrain-tiles/manifest.json']);
  assert(Math.abs(cache.sample(5, -5) - (-1500 + 10000 * 10500 / 65535)) < .0001);
  cache.dispose();
});

test('mesh shader projects actual displaced coordinates through the shared camera', () => {
  assert.match(api.vertexSource, /layout\(location=0\) in vec3 aSurface/);
  assert.match(api.vertexSource, /vSurface = aSurface/);
  assert.match(api.vertexSource, /transpose\(uInvRot\) \* aSurface/);
  assert.match(api.vertexSource, /transpose\(uRayBasis\) \* \(local - uCamera\)/);
  assert.match(api.vertexSource, /gl_Position = vec4\(eye.xy \/ uHalf, A \* eye.z \+ B, -eye.z\)/);
});

function lakeFixture() {
  const f = fixture();
  vm.runInContext(fs.readFileSync(path.join(ui, 'rivers.js'), 'utf8'), f.context);
  f.sampler = f.api.makeLakeSampler(f.context.window.RIVERS, f.Globe);
  return f;
}

test('water planes match six transcribed HydroLAKES records and exact existing shorelines', () => {
  const f = lakeFixture(), provenance = JSON.parse(fs.readFileSync(path.join(ui, 'lake-surfaces.json'), 'utf8'));
  assert.equal(f.sampler.count, 6); assert.equal(f.warnings.length, 0);
  assert.deepEqual(plain(f.api.bedLakes), provenance.lakes.map(lake => [lake.lake_index, lake.surface_metres, lake.path_fnv1a]));
  for (const lake of provenance.lakes) {
    assert.equal(lake.surface_metres, Number(lake.hydrolakes_elevation_field));
    const hash = require('node:crypto').createHash('sha256').update(f.context.window.RIVERS.lakes[lake.lake_index]).digest('hex');
    assert.equal(hash, lake.path_sha256);
  }
  for (const [lon, lat, expected] of [[108, 53.5, 449], [-87.5, 47.6, 179], [-87, 44, 175], [-82, 45, 175], [-81, 42, 172], [-77.8, 43.6, 73]])
    assert.equal(f.sampler.at(lon, lat), expected, lon + ', ' + lat);
  assert.equal(f.sampler.at(85, 28), null, 'land is not assigned a lake level');
});

test('Baikal surface replaces its bed in geometry, slope and picking height together', () => {
  const f = lakeFixture(), cache = new f.api.TileCache({waterSampler: f.sampler});
  cache.tiles.set('28:3', new Float32Array(api.tileSize ** 2).fill(-600));
  assert.equal(cache.sample(108, 53.5), 449);
  assert.deepEqual(plain(cache.sampleSurface(108, 53.5)), [449, 449, 449, 449, 449]);
  assert.equal(cache.sample(108, 52), -600, 'uncovered native terrain is not modified');
  const mesh = f.api.buildMesh({west: 107.99, east: 108.01, south: 53.49, north: 53.51},
    (lon, lat) => cache.sample(lon, lat), {sampleSurface: (lon, lat) => cache.sampleSurface(lon, lat)});
  assert(mesh); assert.equal(mesh.minHeight, 449); assert.equal(mesh.maxHeight, 449);
  for (let i = 0; i < mesh.vertices.length; i += 6) {
    assert(Math.abs((Math.hypot(...mesh.vertices.slice(i, i + 3)) - 1) * api.earthMetres - 449 * 3) < .5);
    assert.equal(mesh.vertices[i + 4], 0); assert.equal(mesh.vertices[i + 5], 0);
  }
  cache.dispose();
});

test('a changed lake path cannot silently apply a water height to another feature', () => {
  const f = lakeFixture(), altered = {lakes: [...f.context.window.RIVERS.lakes]};
  altered.lakes[1] = altered.lakes[4];
  const sampler = f.api.makeLakeSampler(altered, f.Globe);
  assert.equal(sampler.count, 5); assert.equal(sampler.at(108, 53.5), null);
  assert.equal(f.warnings.length, 1); assert.match(f.warnings[0], /shoreline changed/);
});
