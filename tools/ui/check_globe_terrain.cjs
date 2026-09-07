// Actual camera, projection, terrain ray picking and pointer handlers. The
// synthetic height surfaces are fixtures, never additions to source geography.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const source = fs.readFileSync(path.join(__dirname, '../../spheres-web/ui/globe3d.js'), 'utf8');
const sandbox = vm.createContext({window: {}, document: {documentElement: {classList: {add() {}, remove() {}}}}});
vm.runInContext(source, sandbox, {filename: 'globe3d.js', timeout: 3000});
const Globe = sandbox.window.Globe3D, DEG = Math.PI / 180;
function fixture(options = {}) {
  const ratio = options.ratio || 1, cw = options.width || 1200, ch = options.height || 800;
  const rect = {left: 40.3, top: 80.9, width: cw + .35, height: ch + .2};
  const size = {width: cw * ratio, height: ch * ratio, cw, ch, ratio, aspect: cw / ch};
  const longitude = options.lon ?? 12, latitude = options.lat ?? 46;
  const height = options.sampleHeight || (() => options.elevation ?? 0);
  const globe = Object.create(Globe.prototype);
  Object.assign(globe, {yaw: -longitude * DEG, pitch: latitude * DEG, zoom: options.zoom || 128,
    canvas: {clientWidth: cw, clientHeight: ch, getBoundingClientRect: () => rect, addEventListener() {}, setPointerCapture() {}},
    lastSize: size, pointers: new Map(), schedule() {},
    options: {terrainEnabled: () => options.terrain !== false, terrainSurface: () => ({sampleHeight: height}), tilted: () => options.tilted !== false},
  });
  return {globe, size, rect, longitude, latitude, view: () => globe.view(size),
    project(lon, lat) {return globe.projectGeo(lon, lat, globe.view(size), 1);},
    pick(point) {return globe.geoAt(rect.left + point[0] / ratio, rect.top + point[1] / ratio);},
  };
}
function groundDistance(a, b) {
  const dLon = ((a[0] - b[0] + 540) % 360 - 180) * Math.cos((a[1] + b[1]) / 2 * DEG);
  return Math.hypot(dLon, a[1] - b[1]) * DEG * 6371000;
}
function analyticPick(f, point, radius = 1) {
  const view = f.view(), basis = view.rayBasis, origin = view.camera;
  const local = [(point[0] / f.size.width * 2 - 1) * view.halfTan * view.aspect,
    (1 - point[1] / f.size.height * 2) * view.halfTan, -1];
  const direction = [0,1,2].map(i => basis[i] * local[0] + basis[i+3] * local[1] + basis[i+6] * local[2]);
  const length = Math.hypot(...direction); direction.forEach((value, i) => direction[i] = value / length);
  const b = direction.reduce((sum, value, i) => sum + value * origin[i], 0);
  const c = origin.reduce((sum, value) => sum + value * value, 0) - radius * radius;
  const distance = -b - Math.sqrt(b*b-c), hit = origin.map((value, i) => value + direction[i] * distance);
  const inverse = view.invBasis;
  const world = [0,1,2].map(i => inverse[i]*hit[0] + inverse[i+3]*hit[1] + inverse[i+6]*hit[2]);
  return [Math.atan2(world[0], world[2]) / DEG, Math.asin(world[1] / Math.hypot(...world)) / DEG];
}

test('terrain tilt and camera height derive from one live view and return to overhead when disabled', () => {
  for (const zoom of [1, 8, 12, 20, 80, 192]) {
    const f = fixture({zoom, elevation: 3000}), view = f.view();
    assert.equal(view.zoom, zoom); assert.equal(view.distance, f.globe.distance());
    assert(view.tilt >= 0 && view.tilt <= 55 * DEG);
    if (zoom <= 8) assert.equal(view.tilt, 0);
    if (zoom >= 20) assert.equal(view.tilt, 55 * DEG);
    const altitude = view.distance - 1, focus = zoom >= 12 ? 3000 * 3 / 6371000 : 0;
    assert(Math.abs(view.camera[2] - (1 + focus + Math.cos(view.tilt) * altitude)) < 1e-12);
  }
  for (const options of [{terrain: false}, {tilted: false}]) {
    const f = fixture(options); assert.equal(f.view().tilt, 0); assert.equal(Math.abs(f.view().camera[1]), 0);
  }
});

test('tilted projection and picking round-trip sourced coordinates at close zoom and multiple pixel ratios', () => {
  for (const ratio of [1, 1.5, 2]) {
    for (const zoom of [20, 80, 128, 192]) {
      for (const [lon, lat] of [[12, 46], [-105, 40], [179.85, -20]]) {
        const f = fixture({ratio, zoom, lon, lat, elevation: 1800});
        for (const [dx, dy] of [[0, 0], [.04, 0], [-.04, .03], [0, -.04]]) {
          const point = f.project(lon + dx, lat + dy); assert(point, 'near-focus terrain is projected');
          const picked = f.pick(point); assert(picked, 'projected terrain remains pickable');
          // Icons are intentionally lifted 0.00001 Earth radii (63.71 m).
          // At 55 degrees the corresponding flat-ground offset is ~91 m.
          const distance = groundDistance(picked, [lon + dx, lat + dy]);
          assert(distance < 100, `${zoom}x, DPR ${ratio}, ${lon}/${lat}: ${distance.toFixed(3)} m`);
          const projectedAgain = f.project(picked[0], picked[1]); assert(projectedAgain);
          const error = Math.hypot(projectedAgain[0] - point[0], projectedAgain[1] - point[1]) / ratio;
          assert(error < 1, `${zoom}x reprojection offset ${error.toFixed(3)} CSS px`);
        }
      }
    }
  }
});

test('smooth synthetic valleys and peaks retain terrain-aware picking at city zoom', () => {
  const sampleHeight = (lon, lat) => 1600 + 350 * Math.sin((lon - 12) * 12) + 200 * Math.cos((lat - 46) * 10);
  for (const zoom of [80, 128, 192]) {
    const f = fixture({zoom, sampleHeight});
    for (const [lon, lat] of [[12, 46], [11.95, 45.97], [12.04, 46.04]]) {
      const point = f.project(lon, lat); assert(point);
      const picked = f.pick(point); assert(picked);
      const distance = groundDistance(picked, [lon, lat]);
      assert(distance < 110, `${zoom}x sloped-ground offset ${distance.toFixed(3)} m`);
    }
  }
});

test('raised terrain above the base-globe horizon stays pickable', () => {
  const f = fixture({zoom: 20, lon: 0, lat: 0, elevation: 8000});
  const point = f.project(0, 11.8); assert(point);
  assert(point[0] > 0 && point[0] < f.size.width && point[1] > 0 && point[1] < f.size.height);
  const picked = f.pick(point);
  assert(picked, 'a negative unit-sphere discriminant must still test the elevated terrain shell');
  // Near the horizon a 63.71 m icon lift spans hundreds of ground metres;
  // verify the actual elevated sphere intersection instead of limiting that
  // perspective effect with an arbitrary geographic-distance allowance.
  const exact = analyticPick(f, point, 1 + 8000 * 3 / 6371000);
  assert(groundDistance(picked, exact) < 2, 'bounded terrain marching agrees with the analytic raised sphere');
  const projectedAgain = f.project(...picked);
  assert(Math.hypot(projectedAgain[0] - point[0], projectedAgain[1] - point[1]) < .1,
    'the lifted marker remains within a tenth of a screen pixel at the grazing angle');
});

test('overhead picking retains camera precision after leaving Terrain despite the small marker lift', () => {
  const f = fixture({terrain: false, zoom: 192});
  for (const [lon, lat] of [[12, 46], [12.1, 46.1], [11.9, 45.9]]) {
    const point = f.project(lon, lat); assert(point); const picked = f.pick(point); assert(picked);
    assert(groundDistance(picked, analyticPick(f, point)) < .25, 'picking matches the analytic surface under the marker');
    const projectedAgain = f.project(...picked);
    assert(Math.hypot(projectedAgain[0]-point[0], projectedAgain[1]-point[1]) < 1,
      'the normalized close-zoom marker lift stays below a screen pixel');
  }
});

test('actual horizontal drag preserves screen travel while its angular rate shrinks with close zoom', () => {
  const rates = [];
  for (const zoom of [20, 80, 128, 192]) {
    const f = fixture({zoom, elevation: 1200}); f.globe.bind();
    const before = f.project(f.longitude, f.latitude); assert(before);
    const event = (x, y) => ({pointerId: 1, button: 0, clientX: x, clientY: y, preventDefault() {}});
    const x = f.rect.left + f.size.cw / 2, y = f.rect.top + f.size.ch / 2;
    f.globe.onPointerDown(event(x, y)); f.globe.onPointerMove(event(x + 8, y));
    const after = f.project(f.longitude, f.latitude); assert(after);
    const travel = Math.hypot(after[0] - before[0], after[1] - before[1]);
    assert(travel > 7.7 && travel < 8.3, `${zoom}x eight-pixel drag moved the ground ${travel.toFixed(3)} px`);
    rates.push(Math.abs(f.globe.yaw + f.longitude * DEG));
    assert.equal(f.globe.drag.moved, true); f.globe.onPointerUp(event(x + 8, y));
  }
  for (let i = 1; i < rates.length; i++) assert(rates[i] < rates[i - 1]);
});

test('close zoom stays bounded and wheel updates keep the camera outside a high terrain focus', () => {
  // The ceiling moved 192 -> 1024 when the procedural cover layer gave the
  // close range something to resolve. This test used to assert the wheel
  // reaches ZOOM_MAX over 8,800 m of terrain, which was true only because 192
  // was far enough out that the clearance guard never bit. At 1024 it does bite
  // -- Everest at 3x exaggeration is 26.4 km of relief and the camera would be
  // INSIDE it -- so the guard now stops the wheel at about 860.
  //
  // That is the guard working, so the test asserts the two properties that
  // matter rather than the one number that happened to satisfy both: over high
  // ground clearance WINS over the ceiling, and over flat ground the ceiling is
  // actually reachable. Pinning only the first would let the cap rot to
  // anything; pinning only the second would let the camera fly into a mountain.
  // THE CEILING IS DERIVED, SO THIS RE-DERIVES IT. The camera sits 2.45/zoom
  // Earth radii up and the terrain mesh is drawn at 3x, so the camera is
  // outside the highest ground only while 2.45/ZOOM_MAX > 8848*3/6371000.
  // 1024 was tried and fails this by 11 km. Pinning the literal instead would
  // let someone raise the exaggeration and fly the camera into a mountain with
  // every test still green.
  const EXAGGERATION = 3, EVEREST = 8848, EARTH = 6371000;
  const clearance = 2.45 / Globe.ZOOM_MAX;
  assert(clearance > EVEREST * EXAGGERATION / EARTH,
    `ZOOM_MAX ${Globe.ZOOM_MAX} puts the camera ${((EVEREST * EXAGGERATION / EARTH - clearance) * EARTH / 1000).toFixed(1)} km inside exaggerated Everest`);

  const high = fixture({zoom: 192, elevation: 8800}); high.globe.bind();
  for (let i = 0; i < 6; i++) high.globe.onWheel({deltaY: -1000, preventDefault() {}});
  // There is NO terrain-dependent clamp in the zoom path, and an earlier draft
  // of this test asserted one because a single wheel notch happened to fall
  // short of the old ceiling. The ceiling alone is what keeps the camera out,
  // which is exactly why it has to be derived above rather than chosen.
  assert(high.globe.zoom <= Globe.ZOOM_MAX,
    `zoom ${high.globe.zoom} passed the ceiling`);
  const camera = high.view().camera;
  assert(Math.hypot(...camera) > 1 + 8800 * 3 / 6371000,
    'the camera is inside the exaggerated terrain');

  // One wheel event multiplies the zoom by exp(1000 * 0.0015) = 4.48, so from
  // 192 a single notch lands at 860 and never reaches the ceiling. Roll it
  // until it stops moving, which is what a player does.
  const flat = fixture({zoom: 192, elevation: 0}); flat.globe.bind();
  for (let i = 0; i < 6; i++) flat.globe.onWheel({deltaY: -1000, preventDefault() {}});
  assert.equal(flat.globe.zoom, Globe.ZOOM_MAX,
    'over flat ground the wheel must still reach the ceiling, or the ceiling is fiction');
});
