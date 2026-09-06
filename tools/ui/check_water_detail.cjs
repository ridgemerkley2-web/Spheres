// Run: node --test tools/ui/check_water_detail.cjs
// Executes the actual renderer against recording contexts and the shipped paths.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const ui = path.resolve(__dirname, '../../spheres-web/ui');
const sandbox = vm.createContext({window: {}});
for (const name of ['water-detail.js', 'rivers.js']) vm.runInContext(fs.readFileSync(path.join(ui, name), 'utf8'), sandbox, {filename: name, timeout: 3000});
const api = sandbox.window.WaterDetail, shipped = sandbox.window.RIVERS.rivers;
const plain = value => JSON.parse(JSON.stringify(value));
const fixtureRivers = [{n: 'Long River', d: 'M100 200L200 200 300 200'},
  {n: 'Middle River', d: 'M600 400L680 400'}, {n: 'Short River', d: 'M950 600L975 600'}];
const style = {riverOpacity: .82, riverWidth: .5};
function canvas(options = {}) {
  const calls = [], stack = [], projected = [];
  const initial = {globalAlpha: .61, strokeStyle: '#321', fillStyle: '#123', lineWidth: 1.7,
    lineCap: 'butt', lineJoin: 'miter', font: '9px serif', textAlign: 'start', textBaseline: 'alphabetic'};
  const ctx = {...initial,
    save() {stack.push(Object.fromEntries(Object.keys(initial).map(key => [key, this[key]])));},
    restore() {assert(stack.length); Object.assign(this, stack.pop());},
    stroke(path) {calls.push({op: 'stroke', path, width: this.lineWidth, alpha: this.globalAlpha, color: this.strokeStyle, cap: this.lineCap});},
    measureText(name) {
      const size = Number(this.font.match(/([\d.]+)px/)[1]);
      const width = String(name).length * size * .55;
      return {width, actualBoundingBoxLeft: options.overhang || width / 2,
        actualBoundingBoxRight: options.overhang || width / 2, actualBoundingBoxAscent: size * .7, actualBoundingBoxDescent: size * .3};
    },
    strokeText(name, x, y) {calls.push({op: 'strokeText', name, x, y});},
    fillText(name, x, y) {calls.push({op: 'fillText', name, x, y, alpha: this.globalAlpha, color: this.fillStyle, font: this.font});},
  };
  const view = {zoom: 6, ratio: 1, width: 1600, height: 1000, ...options.view};
  const globe = {labelBoxes: options.boxes || [],
    facingWorld(x, y) {return options.facing ? options.facing(x, y) : 1;},
    projectWorld(x, y, incoming) {assert.equal(incoming, view); projected.push([x, y]); return options.project ? options.project(x, y) : [x, y];},
  };
  return {ctx, calls, view, globe, projected,
    restored() {assert.equal(stack.length, 0); for (const key in initial) assert.equal(ctx[key], initial[key], key + ' restored');},
    fills() {return calls.filter(call => call.op === 'fillText');},
  };
}
function render(rivers, zoom, options = {}) {
  const f = canvas();
  const seen = [];
  const count = api.paint(f.ctx, rivers, {...style, ...options.style},
    {terrainView: options.terrainView !== false, zoom, path: d => {seen.push(d); return {source: d};}});
  f.restored();
  return {...f, count, seen};
}
function label(name, x, y, extra = {}) {return {name, minZoom: 2, anchors: [{x, y}], ...extra};}

test('all shipped courses are preserved verbatim at regional zoom; no source mutation', () => {
  const before = JSON.stringify(shipped);
  const result = render(shipped, 5);
  assert.equal(result.count, shipped.length);
  assert.deepEqual(result.seen, Array.from(shipped, river => river.d));
  for (const call of result.calls) assert(shipped.some(river => river.d === call.path.source));
  assert.equal(JSON.stringify(shipped), before);
});

test('course-length tiers progressively reveal secondary rivers without losing majors', () => {
  const world = render(fixtureRivers, 1), middle = render(fixtureRivers, 2), close = render(fixtureRivers, 4);
  assert.equal(world.count, 1); assert.equal(middle.count, 2); assert.equal(close.count, 3);
  assert.equal(world.seen[0], fixtureRivers[0].d);
  for (const path of world.seen) assert(middle.seen.includes(path));
  for (const path of middle.seen) assert(close.seen.includes(path));
  assert(close.calls.filter(call => call.color === '#173d4b').every(call => call.alpha < .2), 'casing stays subtle');
  for (const source of close.seen) {
    const lines = close.calls.filter(call => call.path.source === source);
    assert.equal(lines.length, 2); assert(lines[0].width > lines[1].width); assert(lines[0].alpha < lines[1].alpha);
  }
});

test('every shipped course is visible by zoom 4, with a quieter actual world view', () => {
  const world = render(shipped, 1), close = render(shipped, 4);
  assert(world.count > 10 && world.count < shipped.length / 2);
  assert.equal(close.count, shipped.length);
});

test('long named reaches share a tier but unnamed tiny courses do not combine', () => {
  const rivers = [{n: 'Connected', d: 'M0 0L80 0'}, {n: 'Connected', d: 'M100 0L180 0'},
    ...Array.from({length: 20}, (_, i) => ({n: null, d: `M${i} 500L${i + 10} 500`}))];
  assert.equal(render(rivers, 1).count, 2);
});

test('separate M commands never manufacture length across disconnected reaches', () => {
  const rivers = [{n: 'Separated', d: 'M100 100L105 100M2200 100L2205 100'}];
  assert.equal(render(rivers, 1).count, 0);
  assert.equal(api.buildLabels(rivers)[0].length, 10);
});

test('political modes preserve the old opacity, color, width and single strokes', () => {
  const result = render(fixtureRivers, 1, {terrainView: false, style: {riverOpacity: .25, riverWidth: .7, color: '#4c8ee0'}});
  assert.equal(result.count, 3); assert.equal(result.calls.length, 3);
  for (const call of result.calls) {
    assert.equal(call.width, .7); assert.equal(call.alpha, .25); assert.equal(call.color, '#4c8ee0'); assert.equal(call.cap, 'butt');
  }
});

test('terrain widths and alpha remain bounded through maximum camera zoom', () => {
  for (const zoom of [1, 2, 4, 12, 48, 1000]) {
    const result = render(fixtureRivers, zoom, {style: {riverOpacity: 10, riverWidth: 100}});
    for (const call of result.calls) {
      assert(call.width > 0 && call.width <= 1.15); assert(call.alpha > 0 && call.alpha <= 1);
    }
  }
});

test('painter restores the canvas when a path resolver throws', () => {
  const f = canvas();
  assert.throws(() => api.paint(f.ctx, fixtureRivers, style, {terrainView: true, zoom: 6, path() {throw new Error('bad path');}}), /bad path/);
  f.restored();
});

test('disabled or invalid paint inputs are a no-op', () => {
  const f = canvas();
  for (const incoming of [{riverOpacity: 0, riverWidth: .5}, {riverOpacity: NaN, riverWidth: .5}, {riverOpacity: .5, riverWidth: -1}]) {
    assert.equal(api.paint(f.ctx, fixtureRivers, incoming, {path: d => d}), 0);
  }
  assert.equal(api.paint(f.ctx, fixtureRivers, style), 0);
  assert.equal(f.calls.length, 0); f.restored();
});

test('shipped river names are sourced and all bounded alternate anchors are actual exported vertices', () => {
  const labels = api.buildLabels(shipped), names = new Set();
  assert(labels.length > 100); assert(Object.isFrozen(labels));
  for (const label of labels) {
    assert(!names.has(label.name)); names.add(label.name);
    const courses = shipped.filter(river => typeof river.n === 'string' && river.n.trim() === label.name);
    assert(courses.length, label.name + ' is sourced');
    const points = new Set();
    for (const course of courses) {
      const numbers = course.d.replace(/[ML]/g, ' ').trim().split(/[ ,]+/).map(Number);
      for (let i = 0; i < numbers.length; i += 2) points.add(`${numbers[i]},${numbers[i + 1]}`);
    }
    assert(Object.isFrozen(label)); assert(Object.isFrozen(label.anchors));
    assert(label.anchors.length >= 1 && label.anchors.length <= 8);
    for (const anchor of label.anchors) {assert(points.has(`${anchor.x},${anchor.y}`), label.name + ' anchor is on its path'); assert(Object.isFrozen(anchor));}
  }
  assert.equal(api.buildLabels(shipped), labels, 'repeated frames reuse the weakly cached source preprocessing');
});

test('unsupported curves and missing names never produce invented labels', () => {
  assert.deepEqual(plain(api.buildLabels([{n: 'Curve', d: 'M0 0C10 10 20 0 30 0'},
    {n: '', d: 'M100 100L400 100'}, {n: null, d: 'M100 100L400 100'}, {n: 'Point', d: 'M2 3'}, {n: 'Broken', d: 'M2 3L10'}])), []);
  assert.deepEqual(plain(api.buildLabels(null)), []);
});

test('labels are deterministic for unchanged source data and preserve source names', () => {
  const first = api.buildLabels(fixtureRivers), second = api.buildLabels(plain(fixtureRivers));
  assert.deepEqual(plain(first), plain(second));
  assert.deepEqual(Array.from(first, value => value.name), fixtureRivers.map(river => river.n));
});

test('river names appear only at regional zoom and respect enable and density limits', () => {
  const labels = api.buildLabels(fixtureRivers);
  for (const options of [{view: {zoom: 2}}, {disabled: true}]) {
    const f = canvas(options);
    assert.equal(api.drawLabels(f.ctx, f.view, f.globe, labels, {enabled: !options.disabled}), 0); assert.equal(f.calls.length, 0); f.restored();
  }
  const f = canvas(); assert.equal(api.drawLabels(f.ctx, f.view, f.globe, labels, {maxLabels: 1}), 1); f.restored();
  assert.equal(f.fills().length, 1); assert.match(f.fills()[0].font, /^italic /); assert.equal(f.fills()[0].color, '#a8d3db');
});

test('facing and invalid projections reject labels before they reserve a collision box', () => {
  for (const options of [{facing: () => -.2}, {facing: () => .29}, {project: () => null}, {project: () => [NaN, 50]}]) {
    const f = canvas(options);
    assert.equal(api.drawLabels(f.ctx, f.view, f.globe, [label('River', 400, 400)]), 0);
    assert.equal(f.globe.labelBoxes.length, 0); f.restored();
  }
});

test('label collision uses shared boxes and tries an alternate actual river vertex', () => {
  const f = canvas({boxes: [[800, 491, 100, 40]]});
  const labels = [label('River', 800, 500, {anchors: [{x: 800, y: 500}, {x: 200, y: 500}]})];
  assert.equal(api.drawLabels(f.ctx, f.view, f.globe, labels), 1);
  assert.equal(f.fills()[0].x, 200); assert.equal(f.globe.labelBoxes.length, 2); f.restored();
});

test('measured italic overhang and viewport edges reject clipped names at multiple pixel ratios', () => {
  for (const ratio of [1, 2]) {
    const f = canvas({view: {ratio, width: 600 * ratio, height: 400 * ratio}, overhang: 100 * ratio});
    assert.equal(api.drawLabels(f.ctx, f.view, f.globe, [label('Nile', 50 * ratio, 200 * ratio)]), 0);
    assert.equal(f.globe.labelBoxes.length, 0); f.restored();
  }
});

test('small screens keep a sparse name layer and global collision boxes prevent repeated names', () => {
  const f = canvas({view: {width: 390, height: 300}});
  const labels = [label('River A', 110, 100), label('River B', 260, 200), label('River A', 260, 250)];
  assert.equal(api.drawLabels(f.ctx, f.view, f.globe, labels), 1);
  assert.equal(f.globe.labelBoxes.length, 1); f.restored();
});
