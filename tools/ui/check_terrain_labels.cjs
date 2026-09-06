// Run: node --test tools/ui/check_terrain_labels.cjs
// Executes the real feature-label module and real shipped geography in isolated VMs.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const uiPath = path.resolve(__dirname, '../../spheres-web/ui');
const modulePath = path.join(uiPath, 'terrain-labels.js');
const context = vm.createContext({window: {}});
vm.runInContext(fs.readFileSync(modulePath, 'utf8'), context, {filename: modulePath, timeout: 2000});
const api = context.window.TerrainLabels;
const plain = value => JSON.parse(JSON.stringify(value));
const clone = value => JSON.parse(JSON.stringify(value));
function frozen(value) {
  if (!value || typeof value !== 'object' || Object.isFrozen(value)) return value;
  for (const child of Object.values(value)) frozen(child);
  return Object.freeze(value);
}
function synthetic(name, points, terrain = 'mountain') {
  const terrainById = {}, districtById = {};
  points.forEach(([x, y], index) => {
    const id = 'd-' + String(index).padStart(3, '0');
    terrainById[id] = {f: name, t: terrain}; districtById[id] = {id, cx: x, cy: y};
  });
  return {terrainById, districtById};
}
function feature(name, x, y, extra = {}) {
  const anchor = {id: name + '-a', x, y};
  return {name, terrain: 'mountain', count: 12, anchor, anchors: [anchor], minZoom: 1.6, ...extra};
}
function canvasFixture(options = {}) {
  const calls = [], stack = [];
  const initial = {font: '9px serif', globalAlpha: .65, fillStyle: '#123', strokeStyle: '#456', lineWidth: 1,
    textAlign: 'start', textBaseline: 'alphabetic', shadowColor: 'transparent', shadowBlur: 0};
  const ctx = {...initial,
    save() {stack.push(Object.fromEntries(Object.keys(initial).map(key => [key, this[key]]))); calls.push({op: 'save'});},
    restore() {assert(stack.length, 'canvas restore must match save'); Object.assign(this, stack.pop()); calls.push({op: 'restore'});},
    measureText(text) {
      const size = Number(this.font.match(/([\d.]+)px/)?.[1] || 10);
      const width = [...String(text)].reduce((total, char) => total + (char === 'W' ? 1.1 : char === 'i' ? .22 : .58) * size, 0);
      calls.push({op: 'measureText', text, font: this.font, width});
      return {width, actualBoundingBoxAscent: size * .8, actualBoundingBoxDescent: size * .2};
    },
    strokeText(text, x, y) {calls.push({op: 'strokeText', text, x, y, font: this.font, alpha: this.globalAlpha});},
    fillText(text, x, y) {calls.push({op: 'fillText', text, x, y, font: this.font, alpha: this.globalAlpha});},
    setLineDash(value) {calls.push({op: 'setLineDash', value});},
  };
  const view = {zoom: 4, ratio: 1, width: 1600, height: 1000, ...options.view};
  const projectCalls = [], facingCalls = [];
  const globe = {labelBoxes: options.labelBoxes ? options.labelBoxes.map(box => [...box]) : [],
    projectWorld(x, y, incomingView) {projectCalls.push([x, y]); assert.equal(incomingView, view); return options.project ? options.project(x, y) : [x, y];},
    facingWorld(x, y) {facingCalls.push([x, y]); return options.facing ? options.facing(x, y) : 1;},
  };
  return {ctx, view, globe, calls, projectCalls, facingCalls,
    fills: () => calls.filter(call => call.op === 'fillText'),
    assertRestored() {for (const [key, value] of Object.entries(initial)) assert.equal(ctx[key], value, key + ' is restored'); assert.equal(stack.length, 0);},
  };
}

test('shipped geography builds only sourced named features at actual district members', () => {
  const data = vm.createContext({window: {}});
  for (const name of ['terrain.js', 'districts.js']) vm.runInContext(fs.readFileSync(path.join(uiPath, name), 'utf8'), data, {timeout: 5000});
  const terrainById = data.window.TERRAIN.byId;
  const districtById = Object.fromEntries(Object.values(data.window.DISTRICTS.byCountry).flat().map(district => [district.id, district]));
  const before = JSON.stringify([terrainById, districtById]);
  const groups = api.build(terrainById, districtById, {worldWidth: 2400});
  assert(groups.length > 20, 'the shipped named geography produces a useful set of labels');
  assert.equal(new Set(groups.map(group => group.name)).size, groups.length, 'each feature has one group');
  for (const group of groups) {
    const members = Object.entries(terrainById).filter(([id, terrain]) => terrain.f === group.name && Number.isFinite(districtById[id]?.cx) && Number.isFinite(districtById[id]?.cy));
    assert(members.length > 0, group.name + ' comes from the terrain artifact');
    assert.equal(group.count, members.length);
    assert(members.some(([, terrain]) => terrain.t === group.terrain), 'feature classification is sourced');
    for (const anchor of [group.anchor, ...group.anchors]) {
      assert(members.some(([id]) => id === anchor.id), 'anchor belongs to its feature');
      assert.equal(anchor.x, districtById[anchor.id].cx); assert.equal(anchor.y, districtById[anchor.id].cy);
    }
    assert(group.anchors.length <= 6 && group.anchors.length >= 1);
    assert.equal(new Set(group.anchors.map(anchor => anchor.id)).size, group.anchors.length);
  }
  assert.equal(JSON.stringify([terrainById, districtById]), before, 'source geography is not modified');
});

test('medoids stay on actual ground, including disconnected and world-seam features', () => {
  const seam = synthetic('Seam Range', [[3, 100], [5, 100], [2399, 100]]);
  const seamGroup = api.build(seam.terrainById, seam.districtById, {worldWidth: 2400})[0];
  assert.equal(seamGroup.anchor.id, 'd-000'); assert.equal(seamGroup.anchor.x, 3);
  const disconnected = synthetic('Scattered Range', [[100, 100], [110, 110], [120, 120], [1750, 200], [1790, 190], [1810, 210], [2200, 400], [2230, 420], [2250, 390]]);
  const group = api.build(disconnected.terrainById, disconnected.districtById, {worldWidth: 2400})[0];
  assert(Object.hasOwn(disconnected.districtById, group.anchor.id));
  assert.equal(group.anchor.x, disconnected.districtById[group.anchor.id].cx);
  assert.equal(group.anchor.y, disconnected.districtById[group.anchor.id].cy);
  assert(group.anchors.length > 1 && group.anchors.length <= 6, 'disconnected extents get bounded alternate anchors');
  assert(group.anchors.some(anchor => anchor.x < 200)); assert(group.anchors.some(anchor => anchor.x > 1700));
});

test('build is deterministic for reversed input order, with frozen output and immutable sources', () => {
  const a = synthetic('Zulu Ridge', [[100, 100], [200, 100], [300, 100], [400, 100]]);
  a.terrainById['d-000'].t = 'lowland'; a.terrainById['d-003'].t = 'lowland';
  a.terrainById.extra = {f: 'Alpha Plain', t: 'lowland'}; a.districtById.extra = {id: 'extra', cx: 900, cy: 200};
  frozen(a.terrainById); frozen(a.districtById);
  const before = JSON.stringify(a);
  const first = api.build(a.terrainById, a.districtById, {worldWidth: 2400});
  const reversedTerrain = Object.fromEntries(Object.entries(a.terrainById).reverse());
  const reversedDistricts = Object.fromEntries(Object.entries(a.districtById).reverse());
  const second = api.build(reversedTerrain, reversedDistricts, {worldWidth: 2400});
  assert.deepEqual(plain(first), plain(second)); assert.equal(JSON.stringify(a), before);
  assert(Object.isFrozen(first));
  for (const group of first) {assert(Object.isFrozen(group)); assert(Object.isFrozen(group.anchor)); assert(Object.isFrozen(group.anchors)); for (const anchor of group.anchors) assert(Object.isFrozen(anchor));}
});

test('missing names or finite district centroids cannot invent a terrain feature', () => {
  const terrain = {valid: {f: 'Real Mountain', t: 'mountain'}, missing: {t: 'mountain'}, empty: {f: ''}, whitespace: {f: '  '},
    numeric: {f: 7}, nullname: {f: null}, noDistrict: {f: 'Missing Region'}, nan: {f: 'Invalid X'}, infinite: {f: 'Invalid Y'}, string: {f: 'String Coordinate'}};
  const districts = Object.fromEntries(Object.keys(terrain).filter(key => key !== 'noDistrict').map(id => [id, {id, cx: 100, cy: 100}]));
  districts.nan.cx = NaN; districts.infinite.cy = Infinity; districts.string.cx = '100';
  assert.deepEqual(plain(api.build(terrain, districts, {worldWidth: 2400})).map(group => group.name), ['Real Mountain']);
  for (const [t, d] of [[null, null], [undefined, undefined], [{}, {}], [terrain, null], [null, districts]]) assert.equal(api.build(t, d).length, 0);
});

test('feature extent determines the major, regional and local reveal thresholds', () => {
  for (const [count, expected] of [[1, 3.2], [3, 3.2], [4, 2.4], [11, 2.4], [12, 1.6], [20, 1.6]]) {
    const data = synthetic('Range ' + count, Array.from({length: count}, (_, index) => [100 + index * 10, 100]));
    assert.equal(api.build(data.terrainById, data.districtById)[0].minZoom, expected);
  }
});

test('disabled, missing and world-scale terrain labels produce no canvas or collision changes', () => {
  for (const setup of [{settings: {enabled: false}}, {groups: []}, {groups: null}, {view: {zoom: 1.6}}, {view: {zoom: 1}}]) {
    const f = canvasFixture({view: setup.view, labelBoxes: [[10, 10, 20, 20]]}); const before = clone(f.globe.labelBoxes);
    const groups = Object.hasOwn(setup, 'groups') ? setup.groups : [feature('Hidden Range', 400, 300)];
    assert.equal(api.draw(f.ctx, f.view, f.globe, groups, setup.settings || {}), 0);
    assert.deepEqual(f.globe.labelBoxes, before); assert.equal(f.calls.length, 0); f.assertRestored();
  }
});

test('draw rejects hidden hemisphere, missing projections and labels outside the viewport', () => {
  const f = canvasFixture({project: (x, y) => x === 20 ? null : [x, y], facing: x => x === 30 ? .24 : 1});
  const groups = [feature('Missing Projection', 20, 300), feature('Past Horizon', 30, 300), feature('Offscreen', -1000, 300),
    feature('Clipped At Right', 1595, 300), feature('Clipped At Bottom', 600, 995), feature('Visible Range', 600, 300)];
  assert.equal(api.draw(f.ctx, f.view, f.globe, groups, {}), 1);
  assert.deepEqual(f.fills().map(call => call.text), ['Visible Range']); assert.equal(f.globe.labelBoxes.length, 1); f.assertRestored();
});

test('a visible regional center takes priority over a fringe member closer to the camera', () => {
  const f = canvasFixture();
  const group = feature('Plateau', 400, 300);
  group.anchors.push({id: 'fringe', x: f.view.width / 2, y: f.view.height / 2});
  assert.equal(api.draw(f.ctx, f.view, f.globe, [group], {}), 1);
  assert.equal(f.fills()[0].x, 400);
});

test('one feature can use an unobstructed alternate anchor without drawing twice', () => {
  const f = canvasFixture({labelBoxes: [[400, 300, 250, 50]]});
  const group = feature('Long Range', 400, 300); group.anchors.push({id: 'other', x: 1000, y: 300}, {id: 'third', x: 1200, y: 700});
  const before = JSON.stringify(group);
  assert.equal(api.draw(f.ctx, f.view, f.globe, [group], {}), 1); assert.equal(f.fills().length, 1);
  assert.notEqual(f.fills()[0].x, 400); assert.equal(f.globe.labelBoxes.length, 2); assert.equal(JSON.stringify(group), before); f.assertRestored();
  assert.deepEqual(f.globe.labelBoxes[0], [400, 300, 250, 50], 'earlier nation/province boxes stay intact');
});

test('terrain labels share measured collision boxes with existing labels and each other', () => {
  const f = canvasFixture(); const groups = [feature('WWWWWWWW', 500, 300), feature('Second Range', 500, 300), feature('Remote Range', 1200, 600)];
  const count = api.draw(f.ctx, f.view, f.globe, groups, {}); assert.equal(count, 2); assert.equal(f.fills().length, 2);
  assert.equal(f.globe.labelBoxes.length, count);
  for (const box of f.globe.labelBoxes) {assert.equal(box.length, 4); assert(box.every(Number.isFinite)); assert(box[2] > 0); assert(box[3] > 0);}
  const wide = f.calls.find(call => call.op === 'measureText' && call.text === 'WWWWWWWW'); assert(wide);
  const wideBox = f.globe.labelBoxes.find(box => Math.abs(box[0] - 500) < 100); assert(wideBox);
  assert(wideBox[2] >= wide.width, 'the reserved rectangle covers actual text width'); f.assertRestored();
});

test('label budgets cap sparse features at twelve by default and respect a smaller explicit cap', () => {
  const groups = Array.from({length: 20}, (_, index) => feature('Feature ' + index, 250 + index % 5 * 700, 200 + Math.floor(index / 5) * 300));
  for (const [settings, expected] of [[{}, 12], [{maxLabels: 3}, 3], [{maxLabels: 0}, 0]]) {
    const f = canvasFixture({view: {width: 4000, height: 1600}});
    assert.equal(api.draw(f.ctx, f.view, f.globe, groups, settings), expected); assert.equal(f.fills().length, expected); assert.equal(f.globe.labelBoxes.length, expected); f.assertRestored();
  }
});

test('labels use italic text and pixel-ratio-scaled measured fonts while restoring canvas state', () => {
  const rendered = [];
  for (const ratio of [1, 2]) {
    const f = canvasFixture({view: {ratio, width: 1600 * ratio, height: 1000 * ratio}});
    assert.equal(api.draw(f.ctx, f.view, f.globe, [feature('Measured Range', 600 * ratio, 300 * ratio)], {}), 1);
    const fill = f.fills()[0], measure = f.calls.find(call => call.op === 'measureText');
    assert.match(fill.font, /italic/); assert.equal(fill.font, measure.font);
    rendered.push({size: Number(fill.font.match(/([\d.]+)px/)[1]), width: measure.width, box: f.globe.labelBoxes[0]});
    f.assertRestored();
  }
  assert.equal(rendered[1].size, rendered[0].size * 2); assert.equal(rendered[1].width, rendered[0].width * 2);
  assert.equal(rendered[1].box[2], rendered[0].box[2] * 2); assert.equal(rendered[1].box[3], rendered[0].box[3] * 2);
});

test('regional names fade in above their threshold and reach stable opacity after six tenths of zoom', () => {
  const group = feature('Regional Range', 500, 300, {count: 5, minZoom: 2.4}); const alphas = [];
  for (const zoom of [2.4, 2.7, 3, 3.6]) {
    const f = canvasFixture({view: {zoom}}); const count = api.draw(f.ctx, f.view, f.globe, [group], {});
    if (zoom === 2.4) {assert.equal(count, 0); assert.equal(f.fills().length, 0);}
    else {assert.equal(count, 1); alphas.push(f.fills()[0].alpha);}
    f.assertRestored();
  }
  assert(alphas[0] > 0 && alphas[0] < alphas[1]); assert.equal(alphas[1], alphas[2]);
});

test('major, regional and local features reveal in that order as the camera approaches', () => {
  const groups = [feature('Major Range', 350, 300), feature('Regional Range', 800, 300, {count: 4, minZoom: 2.4}), feature('Local Ridge', 1250, 300, {count: 1, minZoom: 3.2})];
  for (const [zoom, expected] of [[1.6, 0], [1.61, 1], [2.4, 1], [2.41, 2], [3.2, 2], [3.21, 3]]) {
    const f = canvasFixture({view: {zoom}}); assert.equal(api.draw(f.ctx, f.view, f.globe, groups, {}), expected);
    assert.deepEqual(f.fills().map(call => call.text), groups.slice(0, expected).map(group => group.name)); f.assertRestored();
  }
});

test('drawing the same view selects the same member anchor and reserves the same boxes', () => {
  const long = feature('Long Range', 200, 300); long.anchors.push({id: 'central', x: 800, y: 500}, {id: 'east', x: 1400, y: 300});
  const groups = frozen([long, feature('Other Range', 300, 750)]);
  const passes = [];
  for (let attempt = 0; attempt < 2; attempt++) {
    const f = canvasFixture(); assert.equal(api.draw(f.ctx, f.view, f.globe, groups, {}), 2);
    assert.equal(f.fills()[0].x, 200, 'the visible primary member supplies a stable regional label');
    passes.push({fills: f.fills(), boxes: plain(f.globe.labelBoxes)}); f.assertRestored();
  }
  assert.deepEqual(passes[0], passes[1]);
});

test('duplicate feature entries still draw at most one name', () => {
  const f = canvasFixture(); const first = feature('Single Range', 400, 300), duplicate = feature('Single Range', 1200, 700);
  assert.equal(api.draw(f.ctx, f.view, f.globe, [first, duplicate], {}), 1);
  assert.equal(f.fills().length, 1); assert.equal(f.globe.labelBoxes.length, 1); f.assertRestored();
});

test('screen density uses CSS area consistently across pixel ratios and caps large budgets', () => {
  for (const ratio of [1, 2]) {
    const groups = Array.from({length: 30}, (_, index) => feature('F' + index, (80 + index % 5 * 160) * ratio, (50 + Math.floor(index / 5) * 100) * ratio));
    const f = canvasFixture({view: {ratio, width: 800 * ratio, height: 600 * ratio}});
    assert.equal(api.draw(f.ctx, f.view, f.globe, groups, {maxLabels: 24}), 10);
    assert.equal(f.fills().length, 10); f.assertRestored();
  }
  const large = canvasFixture({view: {width: 6000, height: 6000}});
  const groups = Array.from({length: 30}, (_, index) => feature('F' + index, 300 + index % 6 * 800, 300 + Math.floor(index / 6) * 800));
  assert.equal(api.draw(large.ctx, large.view, large.globe, groups, {maxLabels: 999}), 24); large.assertRestored();
});

test('missing canvas, globe or view dimensions are harmless before drawing starts', () => {
  for (const missing of ['ctx', 'globe', 'view', 'width', 'height']) {
    const f = canvasFixture(); const args = [f.ctx, f.view, f.globe, [feature('Safe Range', 500, 300)], {}];
    if (missing === 'ctx') args[0] = null;
    else if (missing === 'globe') args[2] = null;
    else if (missing === 'view') args[1] = null;
    else delete f.view[missing];
    assert.equal(api.draw(...args), 0); assert.equal(f.calls.length, 0); assert.equal(f.globe.labelBoxes.length, 0); f.assertRestored();
  }
});
