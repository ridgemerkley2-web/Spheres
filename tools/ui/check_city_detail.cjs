// Run: node --test tools/ui/check_city_detail.cjs
// Exercise actual sourced city records, rendering, collision and selection.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const ui = path.resolve(__dirname, '../../spheres-web/ui');
const sandbox = vm.createContext({window: {}});
for (const name of ['city-detail.js', 'cities.js']) vm.runInContext(fs.readFileSync(path.join(ui, name), 'utf8'), sandbox, {filename: name, timeout: 3000});
const api = sandbox.window.CityDetail, shipped = sandbox.window.CITIES;
const plain = value => JSON.parse(JSON.stringify(value));
function city(name, x = 300, y = 300, extra = {}) {return {name, lon: x / 10, lat: y / 10, pop: 1250000, rank: 1, capital: false, ...extra};}
function fixture(options = {}) {
  const calls = [], stack = [], projects = [];
  const initial = {font: '8px serif', textAlign: 'start', textBaseline: 'alphabetic', lineJoin: 'miter', lineWidth: 1,
    fillStyle: '#123', strokeStyle: '#321', globalAlpha: .3};
  const ctx = {...initial,
    save() {stack.push(Object.fromEntries(Object.keys(initial).map(key => [key, this[key]])));},
    restore() {assert(stack.length); Object.assign(this, stack.pop());},
    beginPath() {calls.push({op: 'beginPath'});}, closePath() {calls.push({op: 'closePath'});},
    arc(x, y, radius) {calls.push({op: 'arc', x, y, radius});}, moveTo(x, y) {calls.push({op: 'moveTo', x, y});},
    lineTo(x, y) {calls.push({op: 'lineTo', x, y});}, fill() {calls.push({op: 'fill', color: this.fillStyle});},
    stroke() {calls.push({op: 'stroke', color: this.strokeStyle});},
    fillRect(x, y, width, height) {calls.push({op: 'fillRect', x, y, width, height, color: this.fillStyle});},
    measureText(text) {
      const size = Number(this.font.match(/([\d.]+)px/)[1]);
      const width = options.widths?.[text] ?? text.length * size * .5;
      calls.push({op: 'measureText', text, font: this.font, width});
      return {width, actualBoundingBoxLeft: options.overhang || width / 2,
        actualBoundingBoxRight: options.overhang || width / 2, actualBoundingBoxAscent: size * .7, actualBoundingBoxDescent: size * .3};
    },
    strokeText(text, x, y) {calls.push({op: 'strokeText', text, x, y});},
    fillText(text, x, y) {calls.push({op: 'fillText', text, x, y, font: this.font});},
  };
  const view = {zoom: 24, ratio: 1, width: 1200, height: 800, ...options.view};
  const globe = {labelBoxes: options.boxes || [], facingGeo: options.facing || (() => 1),
    projectGeo(lon, lat, incoming, lift) {projects.push({lon, lat, view: incoming, lift}); return options.project ? options.project(lon, lat) : [lon * 10 * view.ratio, lat * 10 * view.ratio];},
  };
  return {ctx, view, globe, calls, projects,
    draw(cities, settings = {}) {return api.draw(ctx, view, globe, cities, settings);},
    fills() {return calls.filter(call => call.op === 'fillText');},
    restored() {assert.equal(stack.length, 0); for (const key in initial) assert.equal(ctx[key], initial[key], key + ' restored');},
  };
}

test('all 1,249 shipped settlements retain their exact source location and metadata', () => {
  const before = JSON.stringify(shipped), cities = api.build(shipped);
  assert.equal(cities.length, 1249); assert(Object.isFrozen(cities));
  const originals = new Map(shipped.map(city => [api.id(city), city]));
  for (const city of cities) {
    const original = originals.get(city.id); assert(original); assert(Object.isFrozen(city));
    for (const field of ['name', 'lon', 'lat', 'pop', 'rank', 'capital']) assert.equal(city[field], original[field]);
  }
  assert.equal(api.build(shipped), cities); assert.equal(api.build(cities), cities);
  assert.equal(JSON.stringify(shipped), before);
});

test('city IDs distinguish duplicate names and normalized ordering is deterministic', () => {
  const source = [city('Springfield', 300), city('Springfield', 400), city('Capital', 500, 300, {rank: 3, capital: true})];
  assert.notEqual(api.id(source[0]), api.id(source[1]));
  assert.deepEqual(plain(api.build(source)), plain(api.build([...source].reverse())));
  assert.equal(api.build([...source, source[0]]).length, 3);
});

test('missing names or invalid coordinates never create invented map points', () => {
  assert.deepEqual(plain(api.build([city(''), city('NaN', 300, 300, {lon: NaN}), city('Outside', 300, 300, {lat: 100}),
    city('Outside longitude', 300, 300, {lon: 181})])), []);
  assert.deepEqual(plain(api.build(null)), []);
});

test('rank bands progressively reveal settlements and selected cities remain findable', () => {
  const source = [city('Major', 100, 300, {rank: 0}), city('Regional', 300, 300, {rank: 4}), city('Town', 500, 300, {rank: 10})];
  for (const [zoom, count] of [[1, 1], [3, 2], [24, 3]]) {
    const f = fixture({view: {zoom}}); assert.equal(f.draw(source, {labels: false}).shown, count); f.restored();
  }
  const f = fixture({view: {zoom: 1}});
  assert.equal(f.draw(source, {labels: false, selectedId: api.id(source[2])}).shown, 2);
});

test('capital stars and close settlement symbols have bounded screen-space size', () => {
  for (const ratio of [1, 2]) {
    for (const zoom of [1, 24, 128, 512]) {
      const f = fixture({view: {zoom, ratio, width: 1200 * ratio, height: 800 * ratio}});
      const frame = f.draw([city('Capital', 200, 300, {capital: true}), city('City', 600)], {labels: false});
      if (zoom < 32) {
        assert.equal(f.calls.filter(call => call.op === 'moveTo').length, 1);
        assert.equal(f.calls.filter(call => call.op === 'lineTo').length, 9);
        assert.equal(f.calls.filter(call => call.op === 'fillRect').length, zoom >= 14 ? 3 : 0);
        assert(frame.hits.every(hit => hit.radius > 0 && hit.radius <= 8 * ratio));
      } else {
        assert.equal(f.calls.filter(call => call.op === 'fill' && call.color === '#e1dfcb').length, 10,
          'both capital and ordinary settlement get five roof faces');
        assert.equal(f.calls.filter(call => call.op === 'fill' && call.color === '#ffe1a1').length, 1,
          'the capital retains its distinct star badge');
        assert(frame.hits.every(hit => hit.symbolBox[2] >= 24*ratio && hit.symbolBox[2] <= 36*ratio && hit.symbolBox[3] <= 36*ratio));
      }
      f.restored();
    }
  }
});

test('only focused or selected cities display source-population captions', () => {
  const source = [city('Selected', 200), city('Focused', 600), city('Other', 1000)];
  const f = fixture(); const frame = f.draw(source, {selectedId: api.id(source[0]), focusedId: api.id(source[1])});
  assert.equal(f.fills().filter(call => call.text === 'Source pop. 1.3m').length, 2);
  assert.equal(frame.hits.filter(hit => hit.caption).length, 2);
  assert(f.fills().some(call => call.text === 'Other')); f.restored();
});

test('an absent source population is disclosed rather than fabricated from size or rank', () => {
  const source = city('Unknown', 500, 300, {pop: 0}); const f = fixture();
  f.draw([source], {selectedId: api.id(source)});
  assert(f.fills().some(call => call.text === 'Source population unavailable'));
  assert(!f.fills().some(call => /Source pop\. 0/.test(call.text))); f.restored();
});

test('shared label collisions suppress names while settlement symbols stay pickable', () => {
  const f = fixture({boxes: [[600, 400, 1200, 800]]});
  const frame = f.draw([city('Hidden name', 400)]);
  assert.equal(frame.shown, 1); assert.equal(frame.hits[0].labelBox, null); assert.equal(f.fills().length, 0);
  assert.equal(api.pick(frame, 400, 300).name, 'Hidden name'); assert.equal(f.globe.labelBoxes.length, 1); f.restored();
});

test('names try alternate sides to avoid another settlement marker', () => {
  const f = fixture({widths: {Alpha: 90}});
  const frame = f.draw([city('Alpha', 300, 300, {rank: 0}), city('Beta', 350, 300, {rank: 2})]);
  assert.equal(frame.shown, 2);
  const alpha = f.fills().find(call => call.text === 'Alpha'); assert(alpha); assert(alpha.x < 300, 'right-side marker sends the name to the left'); f.restored();
});

test('projector coordinates remain authoritative for terrain placement', () => {
  const f = fixture(); const source = city('Peak city', 300); let seen;
  const frame = f.draw([source], {project(city, view, globe) {seen = {city, view, globe}; return [600, 420];}});
  assert.equal(seen.city.lon, source.lon); assert.equal(seen.city.lat, source.lat);
  assert.equal(seen.view, f.view); assert.equal(seen.globe, f.globe);
  assert.deepEqual(plain(frame.hits[0].point), [600, 420]); assert.equal(f.projects.length, 0); f.restored();
});

test('hidden, offscreen and invalid projected locations have no draw or hit target', () => {
  for (const options of [{facing: () => -1}, {project: () => null}, {project: () => [NaN, 200]}, {project: () => [2, 300]}]) {
    const f = fixture(options); assert.equal(f.draw([city('Hidden')]).shown, 0); assert.equal(f.calls.length, 0); f.restored();
  }
});

test('labels stay wholly within the backing viewport at either pixel ratio', () => {
  for (const ratio of [1, 2]) {
    const f = fixture({view: {ratio, width: 600 * ratio, height: 400 * ratio}});
    const frame = f.draw([city('Right edge settlement', 560, 200)]);
    assert.equal(frame.shown, 1); const box = frame.hits[0].labelBox; assert(box);
    assert(box[0] + box[2] / 2 <= f.view.width); assert(box[0] - box[2] / 2 >= 0);
    assert(box[1] + box[3] / 2 <= f.view.height); assert(box[1] - box[3] / 2 >= 0); f.restored();
  }
});

test('hiding labels preserves city targets; hiding cities returns a fresh empty hit frame', () => {
  const f = fixture(); const source = [city('City')];
  const marked = f.draw(source, {labels: false}); assert.equal(marked.shown, 1);
  assert.equal(f.fills().length, 0); assert(!f.calls.some(call => call.op === 'measureText'));
  f.calls.length = 0; const hidden = f.draw(source, {enabled: false});
  assert.deepEqual(plain(hidden), {shown: 0, hits: []}); assert.equal(api.pick(hidden, 300, 300), null);
  assert.equal(f.calls.length, 0); f.restored();
});

test('symbol and label clicks select the original source facts exactly once', () => {
  const f = fixture(); const frame = f.draw([city('City', 500)]), hit = frame.hits[0], selected = [];
  assert.equal(api.pick(frame, ...hit.point), hit.city); assert(hit.labelBox);
  const result = api.activate(frame, hit.labelBox[0], hit.labelBox[1], city => selected.push(city));
  assert.equal(result, hit.city); assert.deepEqual(selected, [hit.city]);
  assert.equal(result.pop, 1250000); assert.equal(result.rank, 1);
  assert.equal(api.activate(frame, 1199, 799, city => selected.push(city)), null); assert.equal(selected.length, 1);
});

test('overlapping hit padding chooses the closest symbol and exact symbols beat nearby labels', () => {
  const a = {name: 'A'}, b = {name: 'B'};
  const frame = {hits: [{city: a, point: [100, 100], radius: 6, markerBox: [100, 100, 44, 44], labelBox: [118, 100, 30, 20]},
    {city: b, point: [120, 100], radius: 6, markerBox: [120, 100, 44, 44], labelBox: null}]};
  assert.equal(api.pick(frame, 120, 100), b); assert.equal(api.pick(frame, 100, 120), a);
  assert.equal(api.pick(frame, NaN, 100), null); assert.equal(api.pick(null, 100, 100), null);
});

test('Find matches accents, exact names and duplicate names while returning source coordinates', () => {
  const source = [city('San José', 100), city('San Jose', 200), city('San Jose del Cabo', 300), city('York', 500), city('New York', 600, 300, {rank: 0})];
  assert.deepEqual(Array.from(api.search(source, 'sAN JOSE'), value => value.lon).sort((a, b) => a - b), [10, 20, 30]);
  assert.equal(api.search(source, 'york')[0].name, 'York', 'exact name wins over a higher-ranked substring match');
  assert.equal(api.search(source, 'san jose', 1).length, 1); assert.equal(api.search(source, '', 10).length, 0);
  assert.equal(api.search(source, 'san jose', 0).length, 0); assert.equal(api.search(source, 'not present').length, 0);
});

test('label count remains sparse on a narrow viewport without dropping source points', () => {
  const f = fixture({view: {width: 390, height: 300}});
  const source = [city('A', 80, 100), city('B', 250, 100), city('C', 80, 230), city('D', 250, 230)];
  const frame = f.draw(source, {maxLabels: 1}); assert.equal(frame.shown, 4); assert.equal(f.fills().length, 1); f.restored();
});

test('canvas state is restored even when label measurement fails', () => {
  const f = fixture(); f.ctx.measureText = () => {throw new Error('measure failed');};
  assert.throws(() => f.draw([city('City')]), /measure failed/); f.restored();
});

test('close isometric skylines stay grounded and their entire artwork is clickable at every pixel ratio', () => {
  for (const ratio of [1, 2]) {
    for (const capital of [false, true]) {
      for (const zoom of [32, 128, 192]) {
        const f = fixture({view: {zoom, ratio, width: 1200*ratio, height: 800*ratio}});
        const source = city('Settlement', 500, 300, {capital});
        const hit = f.draw([source], {labels: false}).hits[0];
        assert.deepEqual(plain(hit.point), [500*ratio, 300*ratio], 'the source ground anchor does not move with symbol size');
        const [x,y,width,height] = hit.symbolBox;
        const contained = (px,py) => Math.abs(px-x) <= width/2 && Math.abs(py-y) <= height/2;
        for (const call of f.calls) {
          if (call.op === 'moveTo' || call.op === 'lineTo') assert(contained(call.x,call.y), 'each roof and facade vertex fits its hit box');
          if (call.op === 'arc') {
            assert(contained(call.x-call.radius,call.y-call.radius));
            assert(contained(call.x+call.radius,call.y+call.radius));
          }
        }
        const frame = {hits:[hit]};
        assert.equal(api.pick(frame, x, y-height*.45), hit.city, 'the tower roof is selectable');
        assert.equal(api.pick(frame, ...hit.point), hit.city, 'the original ground point stays selectable');
        assert(hit.markerBox[2] >= width && hit.markerBox[3] >= height); f.restored();
      }
    }
  }
});

test('close labels and population captions reserve the whole skyline height', () => {
  const f = fixture({view: {zoom: 128}}), source = city('Capital', 500, 300, {capital:true});
  const frame = f.draw([source], {selectedId:api.id(source)}), hit = frame.hits[0];
  assert(hit.labelBox); const symbol = hit.symbolBox, label = hit.labelBox;
  assert(Math.abs(symbol[0]-label[0]) >= (symbol[2]+label[2])/2 || Math.abs(symbol[1]-label[1]) >= (symbol[3]+label[3])/2);
  assert(f.fills().some(call => call.text === 'Source pop. 1.3m')); f.restored();
});
