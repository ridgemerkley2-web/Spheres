// Run: node --test tools/ui/check_polar_cap.cjs
// Evaluate the actual shader's north-cap expressions with GLSL scalar/vector
// arithmetic. Browser checks still verify compilation and rendered appearance.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const ui = path.join(__dirname, '../../spheres-web/ui');
const page = fs.readFileSync(path.join(ui, 'index.html'), 'utf8');
const shader = /const GLSL_MAP = `([\s\S]*?)`;/.exec(page)?.[1];
assert(shader, 'The actual map shader must be available');
const top = Number(/const float LAT_TOP = ([\d.]+);/.exec(shader)?.[1]);
const bottom = Number(/const float LAT_BOT = (-?[\d.]+);/.exec(shader)?.[1]);
const mask = /float capN = ([^;]+);/.exec(shader)?.[1];
const composition = /col = (mix\(col, [^,]+, capN\));/.exec(shader)?.[1];
assert(mask && composition, 'The actual north-cap mask and composition must be available');

function smoothstep(lo, hi, value) {
  assert(hi > lo, 'A north-cap transition needs ascending edges');
  const t = Math.max(0, Math.min(1, (value - lo) / (hi - lo)));
  return t * t * (3 - 2 * t);
}
function palette(name) {
  const match = new RegExp(`const vec3\\s+${name}\\s*=\\s*vec3\\(([^)]+)\\);`).exec(shader);
  assert(match, `The shipped ${name} palette must exist`);
  return match[1].split(',').map(Number);
}
const context = vm.createContext({ LAT_TOP: top, LAT_BOT: bottom, smoothstep,
  OCEAN_0: palette('OCEAN_0'),
  mix: (a, b, amount) => a.map((value, i) => value * (1 - amount) + b[i] * amount),
});
const maskProgram = new vm.Script(mask);
const compositionProgram = new vm.Script(composition);
function evaluate(latitude, color = [.72, .13, .31]) {
  context.glat = latitude;
  context.col = color;
  context.capN = maskProgram.runInContext(context, { timeout: 1000 });
  return { mask: context.capN, color: compositionProgram.runInContext(context, { timeout: 1000 }) };
}

test('north fallback leaves every sampled latitude and its composed highlights untouched', () => {
  assert.equal(top, 83, 'The original projection clip remains fixed');
  assert.equal(bottom, -58);
  const highlight = [.72, .13, .31];
  for (const latitude of [-90, -58, 0, 80, 82.4, 82.9, 82.9999, top]) {
    const result = evaluate(latitude, highlight);
    assert.equal(result.mask, 0, `No fallback wash at ${latitude} degrees`);
    assert.deepEqual(result.color, highlight);
  }
});

test('north fallback blends continuously only beyond the last sampled parallel', () => {
  let previous = 0;
  for (let step = 0; step <= 120; step++) {
    const result = evaluate(top + step / 100);
    assert(Number.isFinite(result.mask));
    assert(result.mask >= previous && result.mask <= 1);
    previous = result.mask;
  }
  assert.equal(evaluate(top).mask, 0);
  assert(Math.abs(evaluate(top + .6).mask - .5) < 1e-12);
  assert.equal(evaluate(top + 1.2).mask, 1);
});

test('the unsampled north cap uses the existing ocean palette without invented ice geometry', () => {
  const ocean = palette('OCEAN_0');
  for (const latitude of [84.2, 86.4, 90]) {
    for (const color of [[1, 1, 1], [0, 0, 0], [.72, .13, .31]]) {
      const result = evaluate(latitude, color);
      assert.equal(result.mask, 1);
      assert.deepEqual(result.color, ocean);
    }
  }
});

test('the cap presentation preserves the real globe picking boundary', () => {
  const browser = vm.createContext({ window: {} });
  vm.runInContext(fs.readFileSync(path.join(ui, 'globe3d.js'), 'utf8'), browser);
  const globe = Object.create(browser.window.Globe3D.prototype);
  globe.geoAt = () => [0, top];
  const edge = globe.worldAt(100, 100);
  assert(edge && edge.every(Number.isFinite), 'The last sampled latitude stays pickable');
  globe.geoAt = () => [0, top + .0001];
  assert.equal(globe.worldAt(100, 100), null, 'Unsampled polar ground remains outside map picking');
});
