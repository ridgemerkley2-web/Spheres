// Run: node --test tools/ui/check_city_labels.cjs
// The actual Globe3D city pass with a recording canvas; no duplicated renderer.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const source = fs.readFileSync(path.join(__dirname, '../../spheres-web/ui/globe3d.js'), 'utf8');

function city(name, x, overrides = {}) {
  return { name, lon: x, lat: 100, pop: 1000000, rank: 1, capital: false, ...overrides };
}
function fixture(cities, options = {}) {
  const calls = [], projected = [], counts = [];
  const canvas = {
    font: 'stale font', fillStyle: '', strokeStyle: '', lineWidth: 0,
    beginPath() {},
    arc(...args) { calls.push({ kind: 'marker', args, radius: args[2] }); },
    fill() {}, stroke() {},
    measureText(text) {
      const width = options.widths?.[text] ?? 80;
      calls.push({ kind: 'measure', text, width, font: this.font });
      return { width };
    },
    strokeText(text, x, y) { calls.push({ kind: 'outline', text, x, y }); },
    fillText(text, x, y) { calls.push({ kind: 'label', text, x, y, font: this.font }); },
  };
  const context = vm.createContext({ window: {} });
  vm.runInContext(source, context, { timeout: 2000 });
  const globe = Object.create(context.window.Globe3D.prototype);
  Object.assign(globe, {
    zoom: options.zoom ?? 4, overlayContext: canvas,
    labelBoxes: (options.boxes || []).map(box => box.slice()),
    options: { cities, onCitiesChange: count => counts.push(count), ...options.flags },
    projectGeo(lon, lat, view, lift) {
      projected.push({ lon, lat, view, lift });
      return options.hidden?.includes(lon) ? null : [lon, lat];
    },
  });
  const view = { ratio: options.ratio ?? 1 };
  return { globe, calls, projected, counts, view, draw() { globe.drawCities(view); } };
}
const ofKind = (fixture, kind) => fixture.calls.filter(call => call.kind === kind);

test('city collision boxes use the measured active font at either device pixel ratio', () => {
  for (const ratio of [1, 2]) {
    for (const capital of [false, true]) {
      const f = fixture([city('WWW', 100, { capital })], { ratio, widths: { WWW: 123.25 * ratio } });
      f.draw();
      const measure = ofKind(f, 'measure')[0];
      const fontSize = (10.5 + 2 * 1.1) * ratio;
      assert.equal(measure.font, `${capital ? 600 : 400} ${fontSize}px Inter, system-ui, sans-serif`);
      assert.equal(f.globe.labelBoxes[0][2], 123.25 * ratio);
      assert.equal(f.globe.labelBoxes[0][0], 100 + 6 * ratio + 123.25 * ratio / 2);
      assert.equal(ofKind(f, 'label')[0].font, measure.font);
      assert.equal(ofKind(f, 'marker')[0].radius, (capital ? 3.2 : 2.2) * ratio);
    }
  }
});

test('a measured name blocked by an existing map label keeps its city marker', () => {
  const existing = [300, 100, 20, 14];
  const f = fixture([city('WWW', 100)], { widths: { WWW: 300 }, boxes: [existing] });
  f.draw();
  assert.equal(ofKind(f, 'label').length, 0, 'A wide measured name overlaps the existing label');
  assert.equal(ofKind(f, 'outline').length, 0);
  assert.equal(ofKind(f, 'marker').length, 1);
  assert.deepEqual(f.globe.labelBoxes, [existing], 'A suppressed name must not reserve extra space');
  assert.equal(f.globe.citiesShown, 1);
  assert.deepEqual(f.counts, [1], 'The city readout counts visible places');
});

test('source rank still decides colliding city names while both markers remain', () => {
  const input = [city('Lower rank', 180, { rank: 3 }), city('Higher rank', 100, { rank: 1 })];
  const before = JSON.stringify(input);
  const f = fixture(input, { widths: { 'Higher rank': 140, 'Lower rank': 140 } });
  f.draw();
  assert.deepEqual(ofKind(f, 'label').map(call => call.text), ['Higher rank']);
  assert.equal(ofKind(f, 'marker').length, 2);
  assert.equal(f.globe.citiesShown, 2);
  assert.equal(f.globe.labelBoxes.length, 1);
  assert.equal(JSON.stringify(input), before, 'Drawing must not reorder the source dataset');
});

test('hiding labels keeps markers without measuring text or reserving label space', () => {
  const f = fixture([city('Capital', 100, { capital: true }), city('Town', 500)],
    { flags: { showLabels: false } });
  f.draw();
  assert.equal(ofKind(f, 'marker').length, 2);
  assert.equal(ofKind(f, 'measure').length, 0);
  assert.equal(ofKind(f, 'label').length, 0);
  assert.equal(f.globe.labelBoxes.length, 0);
  assert.equal(f.globe.citiesShown, 2);
  f.calls.length = 0;
  f.globe.options.showLabels = true;
  f.draw();
  assert.equal(ofKind(f, 'label').length, 2, 'Names return when the option is enabled');
});

test('rank eligibility and globe projection still determine which markers are drawn', () => {
  const f = fixture([
    city('Eligible', 100, { rank: 5 }),
    city('Too small', 200, { rank: 6 }),
    city('Capital exception', 300, { rank: 10, capital: true, pop: 1000 }),
    city('Far side', 400, { rank: 1 }),
  ], { hidden: [400], flags: { showLabels: false } });
  f.draw();
  assert.deepEqual(ofKind(f, 'marker').map(call => call.args[0]), [100, 300]);
  assert.deepEqual(f.projected.map(call => call.lon), [400, 100, 300]);
  assert(f.projected.every(call => call.lat === 100 && call.view === f.view && call.lift === 1.002));
  assert.equal(f.globe.citiesShown, 2);
});

test('disabling cities hides both markers and names and resets the count', () => {
  const f = fixture([city('Visible', 100)]);
  f.draw();
  f.calls.length = 0;
  f.projected.length = 0;
  f.globe.options.showCities = false;
  f.draw();
  assert.equal(f.calls.length, 0);
  assert.equal(f.projected.length, 0);
  assert.equal(f.globe.citiesShown, 0);
  assert.deepEqual(f.counts, [1, 0]);
});
