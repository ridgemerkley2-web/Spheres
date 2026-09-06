// Run: node --test tools/ui/check_map_detail.cjs
// Exercise the shipped rendering functions with recording canvas/GL surfaces.
// These checks protect layer semantics and caching; browser checks cover appearance.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const page = fs.readFileSync(path.join(__dirname, '../../spheres-web/ui/index.html'), 'utf8');

function pageFunction(name) {
  const hit = new RegExp(`^(?:async\\s+)?function ${name}\\(`, 'm').exec(page);
  assert(hit, `Missing actual page function ${name}`);
  const lineEnd = page.indexOf('\n', hit.index);
  const first = page.slice(hit.index, lineEnd);
  return /\}\s*$/.test(first) ? first : page.slice(hit.index, page.indexOf('\n}', hit.index) + 2);
}
const run = (context, code) => vm.runInContext(code, context, { timeout: 2000 });
const plain = value => JSON.parse(JSON.stringify(value));

function canvasFixture() {
  const calls = [];
  const ctx = {
    globalAlpha: 1, lineWidth: 1, strokeStyle: '', fillStyle: '',
    setTransform() {}, clearRect() {}, save() {}, restore() {},
    stroke(shape) { calls.push({ kind: 'stroke', shape, alpha: this.globalAlpha,
      width: this.lineWidth, color: this.strokeStyle }); },
    fill(shape, rule) { calls.push({ kind: 'fill', shape, rule, alpha: this.globalAlpha,
      color: this.fillStyle }); },
  };
  const context = vm.createContext({ ctx, calls, performance: { now: () => 1 },
    p2d: value => value,
    clamp: (value, lo, hi) => Math.min(hi, Math.max(lo, value)),
    paintStockTint: () => calls.push({ kind: 'stock' }),
    paintFronts: () => calls.push({ kind: 'fronts' }),
    polUpload: () => calls.push({ kind: 'upload' }),
    polColor: id => `color-${id}`,
  });
  context.window = context;
  run(context, fs.readFileSync(path.join(__dirname, '../../spheres-web/ui/water-detail.js'), 'utf8'));
  run(context, `
    let ui = { mapMode: 'terrain', cam: { k: 4 },
      mapDetails: { borders: true, provinces: true, cities: true, labels: true } };
    let selected = 'BBB';
    const ZB1 = 2.2, ZB2 = 4, OPQ = .7;
    let CAMBAND = -1;
    const S = { player: 'AAA', nations: [{ id: 'AAA' }, { id: 'BBB' }],
      districts: { 'A-transferred': 'BBB', 'A-still-owned': 'AAA' } };
    const WORLD = { countries: { AA: 'country-A', BB: 'country-B', CC: 'country-neutral' },
      graticule: ['grid'] };
    const TERRITORY = { AAA: ['AA'], BBB: ['BB'] };
    const DINDEX = { 'A-transferred': { iso: 'AA', path: 'transferred-province' },
      'A-still-owned': { iso: 'AA', path: 'unchanged-province' } };
    const DISTRICTS = { byCountry: { AA: [DINDEX['A-transferred'], DINDEX['A-still-owned']],
      BB: [{ path: 'province-B' }] } };
    const RIVERS = { rivers: [{ n: 'Fixture River', d: 'river' }], lakes: ['lake'] };
    const MAP_MODES = Object.fromEntries(['terrain', 'political', 'relations'].map(mode =>
      [mode, { opaque: mode === 'political', color: nation => 'color-' + nation.id }]));
    const POL = { ctx, s: 1, canvas: { width: 200, height: 100 }, dirty: true, fop: .1 };
    function camDials() { return { fop: .1, gratop: .2 }; }
    function me() { return S.nations[0]; }
    ${pageFunction('zramp')}
    ${pageFunction('mapDetailKey')}
    ${pageFunction('mapDetailStyle')}
    ${pageFunction('camBandCheck')}
    ${pageFunction('paintPolitical')}
  `);
  return { context, calls, paint() { calls.length = 0; run(context, 'paintPolitical()'); return calls; } };
}

function glFixture() {
  const calls = [];
  const gl = new Proxy({}, { get: (_, name) => {
    if (String(name).toUpperCase() === name) return name;
    return (...args) => calls.push([name, ...args]);
  } });
  const context = vm.createContext({ gl, calls, performance: { now: () => 1 }, $: () => null,
    document: { getElementById: () => null },
    paintPolitical() {}, paintSelection() {}, glPerf() {} });
  run(context, `
    const GL = { inProof: false, ok: true, ready: true, maxLod: 7, shimmer: false, time: 0 };
    const GLR = { gl, u: new Proxy({}, { get: (_, name) => name }), vao: {}, progMap: {} };
    const GLCV = { isConnected: true, width: 1200, height: 700, style: {} };
    const GLBAKE = { W: 2400, SDF_MAX: 1 }, GLPROJ = { H_EXT: 1018.2 };
    const WORLD = { w: 2400, h: 1018.2 };
    const POL = { dirty: false, canvas: { width: 4096, height: 1738 }, region: [0,0,WORLD.w,WORLD.h] };
    const SEL = { dirty: false, canvas: { width: 4096, height: 1738 }, region: [0,0,WORLD.w,WORLD.h] };
    let ui = { mapMode: 'terrain' };
    function camDials() { return { ground: 1, sat: 1, shade: 1, sea: 1, haze: 0, ao: 1, glint: 0 }; }
    const view = { invBasis: [1,0,0,0,1,0,0,0,1], distance: 2, halfTan: .5,
      pxPerWorld: 1, lk: 0, zoom: 1, yaw: 0, pitch: 0, aspect: 1200/700,
      camera: [0,0,2], rayBasis: [1,0,0,0,1,0,0,0,1] };
    ${pageFunction('glDrawGlobe')}
  `);
  return { context, calls, draw(mode) {
    calls.length = 0;
    run(context, `ui.mapMode = ${JSON.stringify(mode)}; glDrawGlobe(view);`);
    return calls;
  } };
}

test('political painting keeps transferred ownership and data overlays independent of scenery', () => {
  const fixture = canvasFixture();
  const before = run(fixture.context, 'JSON.stringify(S)');
  const calls = fixture.paint();
  const transferred = calls.filter(call => call.kind === 'fill' && call.shape === 'transferred-province');
  assert.equal(transferred.length, 1);
  assert.equal(transferred[0].color, 'color-BBB');
  assert.equal(transferred[0].alpha, .92);
  assert(!calls.some(call => call.kind === 'fill' && call.shape === 'unchanged-province'));
  assert(calls.some(call => call.kind === 'stock'));
  assert(calls.some(call => call.kind === 'fronts'));
  assert.equal(run(fixture.context, 'JSON.stringify(S)'), before);
  assert.equal(run(fixture.context, 'POL.dirty'), false);
  assert.equal(calls.at(-1).kind, 'upload');
});

test('the optional grid draws lightly in Terrain and never changes geographic fills', () => {
  const fixture = canvasFixture();
  const without = plain(fixture.paint());
  assert(!without.some(call => call.shape === 'grid'));
  run(fixture.context, 'ui.mapDetails.grid = true;');
  const terrain = plain(fixture.paint());
  const grid = terrain.find(call => call.kind === 'stroke' && call.shape === 'grid');
  assert.equal(grid.alpha, .12); assert.equal(grid.width, .55);
  assert.deepEqual(terrain.filter(call => call.kind === 'fill'), without.filter(call => call.kind === 'fill'));
  run(fixture.context, "ui.mapMode = 'political';");
  const political = fixture.paint().find(call => call.kind === 'stroke' && call.shape === 'grid');
  assert.equal(political.alpha, .2); assert.equal(political.width, 1.1);
  run(fixture.context, 'ui.mapDetails.grid = false;');
  assert(!fixture.paint().some(call => call.shape === 'grid'));
});

test('detail switches suppress decorative boundaries while preserving country ownership, water, and fronts', () => {
  const fixture = canvasFixture();
  const baseline = plain(fixture.paint());
  const countryFills = calls => calls.filter(call => call.kind === 'fill' && call.shape.startsWith('country-'));
  const provinces = new Set(['transferred-province', 'unchanged-province', 'province-B']);
  assert(baseline.some(call => call.kind === 'stroke' && provinces.has(call.shape)));
  run(fixture.context, 'ui.mapDetails.borders = false; ui.mapDetails.provinces = false;');
  const reduced = fixture.paint();
  assert(!reduced.some(call => call.kind === 'stroke' && provinces.has(call.shape)));
  assert(!reduced.some(call => call.kind === 'stroke' && ['country-B', 'country-neutral'].includes(call.shape)));
  assert(reduced.some(call => call.kind === 'stroke' && call.shape === 'country-A'),
    'The player locator must remain visible');
  assert.deepEqual(countryFills(reduced), countryFills(baseline));
  for (const kind of ['stock', 'fronts']) assert(reduced.some(call => call.kind === kind));
  assert(reduced.some(call => call.kind === 'fill' && call.shape === 'transferred-province' && call.color === 'color-BBB'));
  assert(reduced.some(call => call.kind === 'stroke' && call.shape === 'river'));
  assert(reduced.some(call => call.kind === 'fill' && call.shape === 'lake'));
});

test('Terrain boundaries reveal smoothly and become finer through a regional dive', () => {
  const fixture = canvasFixture();
  const styles = [1, 1.35, 1.8, 2.2, 4, 8, 12, 48].map(zoom =>
    plain(run(fixture.context, `mapDetailStyle('terrain', ${zoom})`)));
  assert.equal(styles[0].provinceOpacity, 0);
  assert(styles[2].provinceOpacity > 0, 'Province detail must begin before the old abrupt 2.2x cutoff');
  assert(styles[4].provinceOpacity > styles[2].provinceOpacity);
  for (let i = 1; i < styles.length; i++) {
    for (const key of ['nationWidth', 'provinceWidth', 'riverWidth']) {
      assert(styles[i][key] > 0 && styles[i][key] <= styles[i - 1][key], key);
    }
    assert(styles[i].provinceOpacity >= styles[i - 1].provinceOpacity);
    assert(styles[i].provinceOpacity <= 1);
    assert(styles[i].riverOpacity > 0 && styles[i].riverOpacity <= 1);
  }
  for (const mode of ['political', 'fronts', 'resources', 'relations', 'stability', 'growth', 'economy']) {
    const world = plain(run(fixture.context, `mapDetailStyle('${mode}', 1)`));
    const regional = plain(run(fixture.context, `mapDetailStyle('${mode}', 4)`));
    assert.deepEqual(world, { nationWidth: .5, provinceOpacity: 0, provinceWidth: .5, riverOpacity: .25, riverWidth: .7 });
    assert.deepEqual(regional, { nationWidth: .5, provinceOpacity: .5, provinceWidth: .5, riverOpacity: .55, riverWidth: .7 });
  }
});

test('regional Terrain zoom refreshes cached borders between the old bands and reuses stationary frames', () => {
  const fixture = canvasFixture();
  run(fixture.context, 'ui.cam.k = 2.4; camBandCheck(); paintPolitical(); camBandCheck();');
  assert.equal(run(fixture.context, 'POL.dirty'), false);
  const originalKey = run(fixture.context, 'POL.detailKey');
  run(fixture.context, 'ui.cam.k = 3.7; camBandCheck();');
  assert.equal(run(fixture.context, 'CAMBAND'), 1, 'The old province/label zoom band did not change');
  assert.equal(run(fixture.context, 'POL.dirty'), true, 'Continuous border detail still needs a new paint');
  run(fixture.context, 'paintPolitical(); camBandCheck();');
  assert.notEqual(run(fixture.context, 'POL.detailKey'), originalKey);
  assert.equal(run(fixture.context, 'POL.dirty'), false);
  run(fixture.context, 'ui.cam.cx = 140; ui.cam.cy = 40; camBandCheck();');
  assert.equal(run(fixture.context, 'POL.dirty'), false, 'A rotation must reuse the geographic texture');
  run(fixture.context, "ui.mapMode = 'political'; ui.cam.k = 2.4; camBandCheck(); paintPolitical(); ui.cam.k = 3.7; camBandCheck();");
  assert.equal(run(fixture.context, 'POL.dirty'), false, 'Terrain detail must not introduce extra repaint work in other modes');
});

test('the political texture composites at full strength in every shading mode', () => {
  const fixture = glFixture();
  for (const mode of ['terrain', 'political', 'fronts', 'resources', 'relations', 'stability', 'growth', 'economy']) {
    const calls = fixture.draw(mode);
    assert.deepEqual(calls.filter(call => call[0] === 'uniform1f' && call[1] === 'polOp'), [['uniform1f', 'polOp', 1]]);
    assert.equal(calls.filter(call => call[0] === 'drawArrays').length, 1);
  }
});

test('Terrain sharpening resets when changing to any other map shading', () => {
  const fixture = glFixture();
  const locationList = /u:\s*U\(progMap,\s*(\[[\s\S]*?\])\)/.exec(pageFunction('glInit'));
  assert(locationList, 'The actual map program must register its uniforms');
  assert(vm.runInNewContext(locationList[1]).includes('terrainView'),
    'The draw must use a uniform location belonging to the actual map program');
  assert.match(page, /uniform\s+float\s+uTerrainView\s*;/);
  for (const mode of ['terrain', 'political', 'terrain', 'fronts', 'resources', 'relations', 'stability', 'growth', 'economy']) {
    const calls = fixture.draw(mode);
    assert.deepEqual(calls.filter(call => call[0] === 'uniform1f' && call[1] === 'terrainView'),
      [['uniform1f', 'terrainView', mode === 'terrain' ? 1 : 0]], mode);
  }
});
