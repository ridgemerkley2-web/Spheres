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
  for (const zoom of [1, 8, 12, 20, 80, 192, 700, Globe.ZOOM_MAX]) {
    const f = fixture({zoom, elevation: 3000}), view = f.view();
    assert.equal(view.zoom, zoom); assert.equal(view.distance, f.globe.distance());
    assert(view.tilt >= 0 && view.tilt <= 55 * DEG);
    if (zoom <= 8) assert.equal(view.tilt, 0);
    if (zoom >= 20) assert.equal(view.tilt, 55 * DEG);
    // The focus height is the terrain at the relief the globe draws at THIS
    // zoom, which is 3x at map zooms and comes down to 1x at the ceiling.
    const altitude = view.distance - 1, focus = zoom >= 12 ? 3000 * Globe.exaggerationFor(zoom) / 6371000 : 0;
    assert.equal(view.terrainExaggeration, Globe.exaggerationFor(zoom));
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
  const exact = analyticPick(f, point, 1 + 8000 * Globe.exaggerationFor(f.globe.zoom) / 6371000);
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

test('at the ceiling the relief is 1x and picking still lands on the flattened ground', () => {
  // The ramp changes the surface the picker marches against, so the
  // round-trip is checked where the ramp has done all of its work. The
  // offsets are a few hundred metres because at the ceiling the frame is only
  // a few kilometres across.
  const sampleHeight = (lon, lat) => 1600 + 350 * Math.sin((lon - 12) * 12) + 200 * Math.cos((lat - 46) * 10);
  const f = fixture({zoom: Globe.ZOOM_MAX, sampleHeight});
  assert.equal(f.view().terrainExaggeration, 1, 'the ceiling is where the relief reaches life size');
  for (const [lon, lat] of [[12, 46], [12.003, 45.998], [11.996, 46.004]]) {
    const point = f.project(lon, lat); assert(point, 'near-focus ground is in frame at the ceiling');
    const picked = f.pick(point); assert(picked, 'the flattened ground is still pickable');
    const distance = groundDistance(picked, [lon, lat]);
    assert(distance < 110, `ceiling sloped-ground offset ${distance.toFixed(3)} m`);
  }
});

test('the relief ramp and the zoom ceiling are one derivation, and the camera clears the rock at every zoom', () => {
  // THE CEILING IS DERIVED, SO THIS RE-DERIVES IT -- against the exaggeration
  // the globe actually draws at each zoom, not a literal 3. The camera sits
  // 2.45/zoom Earth radii up and the terrain is drawn E(zoom) times taller
  // than life, so it is outside the highest ground only while
  //
  //     2.45 / zoom > 8848 * E(zoom) / 6371000.
  //
  // A fixed 3x fails that past 588 (1024 was tried and is 11 km inside
  // Everest). With E = KNEE / zoom the zoom cancels out of the inequality,
  // which is the only reason the ceiling could move to 1500 without a higher
  // camera. Pinning the knee, the ceiling or the quantum instead would let
  // someone move one of them and fly the camera into a mountain with every
  // test still green; so the ACTUAL function is swept against the ACTUAL
  // inequality, densely enough to hit every quantised step.
  const EVEREST = 8848, EARTH = 6371000;
  const E = Globe.exaggerationFor;
  assert.equal(typeof E, 'function', 'the globe must export the relief it draws');
  const zooms = [];
  for (let z = Globe.ZOOM_MIN; z < Globe.ZOOM_MAX; z *= 1.01) zooms.push(z);
  for (let z = 400; z <= Globe.ZOOM_MAX; z += 1) zooms.push(z);
  zooms.push(Globe.ZOOM_MAX);
  zooms.sort((a, b) => a - b);
  let previous = Infinity, tightest = Infinity;
  for (const zoom of zooms) {
    const e = E(zoom);
    assert(e >= 1 && e <= 3, `relief ${e} at zoom ${zoom} is outside 1x..3x`);
    assert(e <= previous, `relief rises from ${previous} to ${e} at zoom ${zoom}: the ramp must only come down as the camera does`);
    previous = e;
    const clearance = 2.45 / zoom, rock = EVEREST * e / EARTH;
    assert(clearance > rock,
      `zoom ${zoom.toFixed(1)} at ${e}x puts the camera ${Math.round((rock - clearance) * EARTH)} m inside exaggerated Everest`);
    tightest = Math.min(tightest, clearance / rock - 1);
  }
  // The ends of the ramp are what make the range worth having: 3x at world
  // zoom or the map view flattens, 1x at the ceiling or the ceiling could
  // still go higher, and the old 512 ceiling still sees the full 3x so nothing
  // a player saw before the ramp has changed.
  assert.equal(E(Globe.ZOOM_MIN), 3);
  assert.equal(E(Globe.ZOOM_MAX), 1);
  assert.equal(E(512), 3, 'the relief at the old ceiling is no longer the full 3x');
  assert(tightest > 0.05, `the tightest clearance on the ramp is only ${(tightest * 100).toFixed(1)}% -- float error in the mesh could eat that`);

  // And the live camera agrees with the derivation: rolled to the stop over
  // 8,800 m of ground, it stays outside the rock at the relief it is drawing.
  const high = fixture({zoom: 192, elevation: 8800}); high.globe.bind();
  for (let i = 0; i < 6; i++) high.globe.onWheel({deltaY: -1000, preventDefault() {}});
  assert(high.globe.zoom <= Globe.ZOOM_MAX, `zoom ${high.globe.zoom} passed the ceiling`);
  const camera = high.view().camera;
  assert(Math.hypot(...camera) > 1 + 8800 * E(high.globe.zoom) / EARTH, 'the camera is inside the exaggerated terrain');

  // One wheel event multiplies the zoom by exp(1000 * 0.0015) = 4.48, so from
  // 192 the notches land at 860 and then the stop. Roll it until it stops
  // moving, which is what a player does; over flat ground the wheel must
  // still reach the ceiling, or the ceiling is fiction.
  const flat = fixture({zoom: 192, elevation: 0}); flat.globe.bind();
  for (let i = 0; i < 6; i++) flat.globe.onWheel({deltaY: -1000, preventDefault() {}});
  assert.equal(flat.globe.zoom, Globe.ZOOM_MAX);
});
