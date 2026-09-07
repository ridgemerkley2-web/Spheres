// Run: node --test tools/ui/check_surface_grain.cjs
//
// There is no GPU in node, so this file does not pretend to compile anything.
// It checks two separate things.
//
// First, the shape of the artefact: that the module exports what a host can
// splice, that the GLSL is balanced, that the documented entry point exists with
// the documented signature, that nothing banned is in it, and that the chunk
// declares exactly one uniform and nothing else at global scope beyond constants
// and functions.
//
// Second, and this is the part worth having: a port of the hash and the noise to
// JavaScript, run over a few hundred thousand samples with Math.fround at every
// step so it rounds the way a 32-bit shader rounds. If the hash is biased, or
// clips, or the fbm sits off zero, the shader looks wrong and nothing else in
// this repository would tell us. Every constant the port uses is read back out
// of the shipped GLSL rather than copied, so the two cannot drift apart quietly.
//
// Third, and this was added after a mutation run rather than designed in: the
// arithmetic the port mirrors is PINNED AS TEXT against the shipped GLSL. The
// statistical tests all run the port, so reverting the shader to an earlier form
// left them green while the shader changed underneath - three separate times.
// The pin is crude and will complain about a reformat. That is the right trade.
//
// EVIDENCE THAT THESE CHECKS CAN FAIL. Ten deliberate reversions were run
// against this file and nine were caught: SG_HASH_A back to 43, corner keys back
// to base+offset, clamp back to max, amplitudes doubled, a duplicated rotation,
// SG_SEED zeroed, the early return removed, a sgPlane swizzle transposed, the
// per-face drift tripled, and the fade pushed past Nyquist. The one that first
// escaped - the swizzle - is why sgPlane is in the pin list.
//
// WHAT THIS DELIBERATELY CANNOT DO.
//   - It cannot compile or link the shader. An argument in the wrong order, a
//     mat2 column-major mistake, a type that will not convert: all invisible
//     here. The first real compile is a browser mounting a card.
//   - It cannot prove the JavaScript port and the GLSL are the same program. It
//     pins the expressions and reads every constant back out of the shipped
//     source, which is as far as text goes; two expressions that are equal as
//     text can still be different programs after a compiler contracts one of
//     them into a fused multiply-add.
//   - It cannot measure ALU cost, register pressure or whether the driver takes
//     the early-out branch. The op counts in the report are hand counts.
//   - It cannot say whether the result looks like painted steel. That is a
//     screenshot and a person.
//   - It cannot see gl_FragCoord.w, so the port takes it as an argument. Whether
//     the host's projection actually makes it one over view depth is a property
//     of the host's matrix, not of this file.
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');

const MODULE_PATH = path.join(__dirname, '../../spheres-web/ui/surface-grain.js');
const api = require(MODULE_PATH);
const source = fs.readFileSync(MODULE_PATH, 'utf8');

// --- the GLSL, with comments removed, which is what the hygiene tests read ----
const glsl = api.glsl;
const bare = glsl.replace(/\/\*[\s\S]*?\*\//g, ' ').replace(/\/\/[^\n]*/g, ' ');

// --- constants, read back out of the shipped GLSL ---------------------------
function scalar(name) {
  const m = new RegExp(`const\\s+float\\s+${name}\\s*=\\s*(-?[\\d.]+)\\s*;`).exec(bare);
  assert(m, `${name} is declared as a const float in the GLSL`);
  return Number(m[1]);
}
function vector(name, size) {
  const m = new RegExp(`const\\s+vec${size}\\s+${name}\\s*=\\s*vec${size}\\(([^)]*)\\)\\s*;`).exec(bare);
  assert(m, `${name} is declared as a const vec${size} in the GLSL`);
  const parts = m[1].split(',').map(v => Number(v.trim()));
  assert.equal(parts.length, size, `${name} has ${size} components`);
  assert(parts.every(Number.isFinite), `${name} components are numbers`);
  return parts;
}
const K = {
  TEXEL: scalar('SG_TEXEL'), GAIN: scalar('SG_GAIN'),
  CUT_LO: scalar('SG_CUT_LO'), CUT_HI: scalar('SG_CUT_HI'),
  FACE: scalar('SG_FACE'), TEMP: scalar('SG_TEMP'), FLOOR: scalar('SG_FLOOR'),
  BUCKET: scalar('SG_BUCKET'), DITHER: scalar('SG_DITHER'),
  SEED: scalar('SG_SEED'), GRAZE: scalar('SG_GRAZE'),
  HASH_A: scalar('SG_HASH_A'), HASH_B: scalar('SG_HASH_B'),
  FREQ: vector('SG_FREQ', 4), AMP: vector('SG_AMP', 4), KEY: vector('SG_KEY', 3),
};
// The rotations are inline mat2 literals in the octave block; take them in order
// so the port turns the same coordinates as the shader.
const ROT = [...bare.matchAll(/mat2\(([^)]*)\)/g)].map(m => m[1].split(',').map(v => Number(v.trim())));
const OFFSET = [...bare.matchAll(/\*\s*SG_FREQ\.[yzw]\s*\+\s*([\d.]+)\)/g)].map(m => Number(m[1]));

// --- the port ---------------------------------------------------------------
// Math.fround after every operation, because the shader is doing this in 32-bit
// floats and a hash built on fract() is exactly the kind of arithmetic where the
// difference shows. GLSL fract(x) is x - floor(x), so it is positive for negative
// inputs, which Math.floor gives us for free.
const f = Math.fround;
const fract = x => f(x - Math.floor(x));
const clamp = (x, a, b) => Math.max(a, Math.min(b, x));
const dot3 = (a, b) => f(f(a[0] * b[0]) + f(f(a[1] * b[1]) + f(a[2] * b[2])));

function sgHash(k) {
  const h = f(fract(k) + K.SEED);
  return fract(f(h * f(f(h * K.HASH_A) + K.HASH_B)));
}
// cornerKey is the whole point of the rewrite this file forced: the key for a
// given lattice corner is built from that corner's own integer coordinates, by
// one expression, so the two cells that share it agree to the bit. Reaching the
// other three corners as base + SG_KEY instead differs by an ulp, and an ulp
// inside fract() is a full-scale jump.
const cornerKey = (ix, iy) => f(f(ix * K.KEY[0]) + f(iy * K.KEY[1]));
function sgNoise(px, py) {
  const ix = Math.floor(px), iy = Math.floor(py);
  let tx = f(px - ix), ty = f(py - iy);
  tx = f(f(tx * tx) * f(3.0 - f(2.0 * tx)));
  ty = f(f(ty * ty) * f(3.0 - f(2.0 * ty)));
  const h0 = sgHash(cornerKey(ix, iy));
  const h1 = sgHash(cornerKey(f(ix + 1), iy));
  const h2 = sgHash(cornerKey(ix, f(iy + 1)));
  const h3 = sgHash(cornerKey(f(ix + 1), f(iy + 1)));
  const a = f(h0 + f(tx * f(h1 - h0)));
  const b = f(h2 + f(tx * f(h3 - h2)));
  return f(f(f(a + f(ty * f(b - a))) * 2.0) - 1.0);
}
function sgPlane(N, P) {
  const a = N.map(Math.abs);
  if (a[1] >= a[0] && a[1] >= a[2]) return [f(P[0] + f(P[1] * 0.31)), f(P[2] + f(P[1] * 0.53))];
  if (a[0] >= a[2]) return [f(P[2] + f(P[0] * 0.53)), f(P[1] + f(P[0] * 0.31))];
  return [f(P[0] + f(P[2] * 0.31)), f(P[1] + f(P[2] * 0.53))];
}
function turn(m, q) {
  return [f(f(m[0] * q[0]) + f(m[2] * q[1])), f(f(m[1] * q[0]) + f(m[3] * q[1]))];
}
// fragW stands in for gl_FragCoord.w, which is one over the view depth for any
// ordinary perspective matrix. texelUniform stands in for uSurfaceTexel; zero is
// what an unset uniform reads back as.
function weights(N, V, fragW, texelUniform = 0) {
  const texel = texelUniform > 0 ? texelUniform : K.TEXEL;
  const graze = Math.max(Math.abs(dot3(N, V)), K.GRAZE);
  const mpp = f(f(texel / Math.max(fragW, 1e-4)) * f(1 / Math.sqrt(graze)));
  const span = f(K.CUT_HI - K.CUT_LO);
  return {mpp, w: K.FREQ.map(F => clamp(f(f(K.CUT_HI - f(mpp * F)) / span), 0, 1))};
}
function surface(albedo, N, P, V, fragW, texelUniform = 0) {
  const {w} = weights(N, V, fragW, texelUniform);
  if (w[0] <= 0.002) return albedo.slice();
  const q = sgPlane(N, P);
  let m = 0, fine = 0;
  m = f(m + f(f(K.AMP[0] * w[0]) * sgNoise(f(q[0] * K.FREQ[0]), f(q[1] * K.FREQ[0]))));
  for (let o = 1; o < 4; o++) {
    if (w[o] <= 0.002) continue;
    const r = turn(ROT[o - 1], q);
    fine = sgNoise(f(f(r[0] * K.FREQ[o]) + OFFSET[o - 1]), f(f(r[1] * K.FREQ[o]) + OFFSET[o - 1]));
    m = f(m + f(f(K.AMP[o] * w[o]) * fine));
  }
  const shift = f(0.5 + f(K.DITHER * fine));
  const b = N.map(n => Math.floor(f(f(n * K.BUCKET) + shift)));
  const fh = sgHash(dot3(b, K.KEY));
  const v = f(f(m + f(f(f(fh * 2.0) - 1.0) * f(K.FACE * w[0]))) * K.GAIN);
  const t = f(f(f(f(fract(f(f(fh * 7.13) + 0.37)) * 2.0) - 1.0) * f(K.TEMP * w[0])) * K.GAIN);
  const c = albedo.map(a => f(f(a * f(1.0 + v)) + f(v * K.FLOOR)));
  const cl = x => Math.min(1, Math.max(0, x));
  return [cl(f(c[0] * f(1.0 + t))), cl(c[1]), cl(f(c[2] * f(1.0 - t)))];
}

// --- sampling helpers -------------------------------------------------------
// A fixed linear congruential generator, so a failure is reproducible and a pass
// means something about this exact sample rather than about last night's.
function stream(seed) {
  let s = seed >>> 0;
  return () => ((s = (s * 1664525 + 1013904223) >>> 0) / 4294967296);
}
function moments(values) {
  const n = values.length;
  let mean = 0;
  for (const v of values) mean += v;
  mean /= n;
  let sq = 0, lo = Infinity, hi = -Infinity;
  for (const v of values) {sq += (v - mean) * (v - mean); lo = Math.min(lo, v); hi = Math.max(hi, v);}
  return {mean, sd: Math.sqrt(sq / n), lo, hi, n};
}
// Canonical framings. Every asset in the deck is fitted to its card, so the view
// distance is set by the asset's own size: half its extent over tan(half the 26
// degree lens), with the fitting margin arsenal3d.js applies.
const FRAMING = [
  {label: 'rifle, 1 m', size: 1, live: 4},
  {label: 'jeep, 4 m', size: 4, live: 3},
  {label: 'tank, 8 m', size: 8, live: 3},
  {label: 'aircraft, 20 m', size: 20, live: 2},
  {label: 'frigate, 133 m', size: 133, live: 1},
];
const depthFor = size => size * 0.5 / Math.tan(13 * Math.PI / 180) * 1.10;

test('the module is the agreed shape and freezes what it hands out', () => {
  assert.deepEqual(Object.keys(api).sort(), ['glsl', 'id', 'notes']);
  assert.equal(api.id, 'surface-grain');
  assert(Object.isFrozen(api), 'the export is frozen');
  assert.throws(() => {api.id = 'other';}, 'a frozen export refuses assignment in strict mode');
  for (const key of ['id', 'glsl', 'notes']) assert.equal(typeof api[key], 'string', `${key} is a string`);
  assert(api.glsl.length > 500 && api.notes.length > 500, 'neither the source nor the notes is a stub');
  // The other half of the UMD: a page with no module object gets a global.
  const sandbox = vm.createContext({});
  vm.runInContext(source, sandbox, {filename: 'surface-grain.js'});
  assert.equal(sandbox.Surfacegrain.id, api.id, 'the browser branch publishes the same object');
  assert.equal(sandbox.Surfacegrain.glsl, api.glsl);
});

test('the GLSL is balanced and every bracket closes', () => {
  const pairs = {'{': '}', '(': ')', '[': ']'};
  const stack = [];
  for (const ch of bare) {
    if (pairs[ch]) stack.push(pairs[ch]);
    else if (ch === '}' || ch === ')' || ch === ']') assert.equal(stack.pop(), ch, 'brackets nest');
  }
  assert.equal(stack.length, 0, 'every bracket is closed');
  assert((bare.match(/\{/g) || []).length >= 5, 'the chunk really does contain function bodies to balance');
  assert(!bare.includes('`'), 'no backtick survived the template literal that carries this');
  assert(!bare.includes('\\'), 'and no escape did either, which would mean the string was mangled');
});

test('the documented entry point exists with the documented signature', () => {
  const entry = /vec3\s+surface\s*\(\s*vec3\s+albedo\s*,\s*vec3\s+N\s*,\s*vec3\s+P\s*,\s*vec3\s+V\s*\)\s*\{/;
  assert(entry.test(bare), 'vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V) is defined verbatim');
  assert.equal((bare.match(/vec3\s+surface\s*\(/g) || []).length, 1, 'it is defined once');
  // Everything it calls has to be defined above it, because GLSL has no forward
  // declarations here and the host splices this in as one block.
  for (const helper of ['sgHash', 'sgHash4', 'sgNoise', 'sgPlane']) {
    const defined = bare.indexOf(`${helper}(`);
    const called = bare.indexOf(`${helper}(`, defined + 1);
    assert(defined >= 0, `${helper} is defined`);
    assert(called > defined, `${helper} is defined before it is called`);
  }
});

test('the GLSL contains nothing the brief bans', () => {
  for (const token of ['texture', 'sampler', '#version', 'fwidth', 'dFdx', 'dFdy', 'discard', 'main(']) {
    assert(!bare.includes(token), `banned token absent: ${token}`);
  }
  assert(!/\btime\b/i.test(bare), 'no time uniform, and nothing named time');
  assert(!/#extension/.test(bare), 'no extensions');
  assert(!/\b(in|out|varying|attribute)\s+(lowp\s+|mediump\s+|highp\s+)?(float|vec[234]|mat[234]|int)\s+\w+\s*;/.test(bare),
    'no stage inputs or outputs are declared; this is a chunk, not a shader');
  assert(!/\bprecision\s+(lowp|mediump|highp)\b/.test(bare), 'no precision statement, which is the host shader\'s to make');
  assert(!/\bgl_Frag(Color|Data)\b/.test(bare), 'it writes to nothing');
  // Animation would break determinism and outrun the reduced-motion setting, and
  // there is no clock here to read anyway.
  assert(!/\bsin\s*\(|\bcos\s*\(|\btan\s*\(/.test(bare), 'no trigonometry, so no driver-dependent hash');
});

test('global scope holds one documented uniform, constants and functions, and nothing else', () => {
  const uniforms = [...bare.matchAll(/\buniform\s+\w+\s+(\w+)\s*;/g)].map(m => m[1]);
  assert.deepEqual(uniforms, ['uSurfaceTexel'], 'exactly the one uniform the notes document');
  assert(api.notes.includes('uSurfaceTexel'), 'and the notes do document it');
  // Walk the top level and check each statement is a uniform, a const, or a
  // function definition. Anything else would be a global the host inherits.
  const forms = [];
  let depth = 0, start = 0;
  for (let i = 0; i < bare.length; i++) {
    const ch = bare[i];
    if (ch === '{') depth++;
    else if (ch === '}') {
      depth--;
      if (depth === 0) {forms.push(bare.slice(start, i + 1)); start = i + 1;}
    } else if (ch === ';' && depth === 0) {forms.push(bare.slice(start, i + 1)); start = i + 1;}
  }
  assert.equal(depth, 0);
  const seen = forms.map(s => s.trim()).filter(Boolean);
  assert(seen.length >= 8, 'the chunk has the expected number of top-level forms');
  for (const form of seen) {
    const ok = /^uniform\s/.test(form) || /^const\s/.test(form)
      || /^(float|vec[234]|mat[234])\s+\w+\s*\([^)]*\)\s*\{/.test(form.replace(/\s+/g, ' '));
    assert(ok, `top-level form is a uniform, a const or a function definition: ${form.slice(0, 60)}`);
  }
});

test('the constants the port uses are the constants the shader ships', () => {
  // Not a tautology: these were read out of api.glsl with regexes above, so if a
  // constant is renamed, retyped or deleted this file stops finding it, and if
  // one is changed every statistical test below is measuring the new value.
  assert(K.TEXEL > 0 && K.TEXEL < 0.02, 'the fallback texel is a plausible metres/pixel/metre');
  assert(K.CUT_HI > K.CUT_LO && K.CUT_HI <= 0.5, 'the fade ends at or below the Nyquist limit of 0.5 cycles/pixel');
  assert(K.FREQ.every((v, i) => i === 0 || v > K.FREQ[i - 1]), 'octave frequencies ascend');
  // A LADDER, not four numbers. Each octave is the same multiple of the last, so
  // the fbm has a flat spectrum across its band and the fade can retire them one
  // at a time as the model shrinks. A mutation run raised the finest octave to 64
  // c/m and nothing else in this file noticed, because the fade simply switched
  // it off - which is safe, and is also the effect quietly disappearing.
  const ratios = [1, 2, 3].map(i => K.FREQ[i] / K.FREQ[i - 1]);
  const spread = Math.max(...ratios) / Math.min(...ratios);
  assert(spread < 1.05, `the octaves are a geometric ladder; ratios ${ratios.map(r => r.toFixed(2)).join(', ')}`);
  assert(ratios[0] > 2.5 && ratios[0] < 5,
    `and the step is an octave-and-a-bit, not a jump; ${ratios[0].toFixed(2)}`);
  assert(K.AMP.every(a => a > 0 && a < 0.06), 'no octave is loud on its own');
  assert(K.AMP.reduce((a, b) => a + b, 0) < 0.12, 'the octaves together cannot move albedo more than a tenth');
  assert.equal(ROT.length, 3, 'three rotated octaves');
  for (const m of ROT) {
    assert.equal(m.length, 4);
    assert(Math.abs(m[0] * m[0] + m[1] * m[1] - 1) < 2e-3, 'each mat2 is a rotation, not a scale');
    assert(Math.abs(m[0] * m[2] + m[1] * m[3]) < 2e-3, 'and is orthogonal');
  }
  assert.equal(OFFSET.length, 3, 'each rotated octave carries its own lattice offset');
  assert(new Set(OFFSET).size === 3, 'the offsets differ');
});

test('the hash is uniform, deterministic and never leaves 0..1', () => {
  const draw = stream(20260907);
  const values = [];
  for (let i = 0; i < 200000; i++) values.push(sgHash(f(draw() * 400 - 200)));
  const m = moments(values);
  assert(values.every(Number.isFinite), 'no NaN and no infinity');
  assert(m.lo >= 0 && m.hi < 1, `output stays in [0,1): got [${m.lo}, ${m.hi}]`);
  assert(Math.abs(m.mean - 0.5) < 0.005, `mean is 0.5, got ${m.mean.toFixed(5)}`);
  assert(Math.abs(m.sd - Math.sqrt(1 / 12)) < 0.005, `variance is a uniform's 1/12, got sd ${m.sd.toFixed(5)}`);
  // Chi-square over 32 bins. The expectation is 31 and a fair hash lands near it;
  // the two-step avalanche this replaced scored 114 on the same sample, which is
  // the size of failure this is here to catch.
  const bins = new Array(32).fill(0);
  for (const v of values) bins[Math.min(31, Math.floor(v * 32))]++;
  const expect = values.length / 32;
  const chi = bins.reduce((s, b) => s + (b - expect) ** 2 / expect, 0);
  assert(chi < 80, `chi-square over 32 bins is ${chi.toFixed(1)}, well past the 31 expected`);
  assert.equal(sgHash(0.37), sgHash(0.37), 'deterministic');
  assert(sgHash(0) > 0, 'the seed keeps a zero key off the zero fixed point');
  // Neighbouring lattice keys must not resemble each other, or the noise reads as
  // a gradient rather than a field.
  const a = [], b = [];
  for (let i = -300; i < 300; i++) {
    a.push(sgHash(f(i * K.KEY[0])) - 0.5);
    b.push(sgHash(f(f(i * K.KEY[0]) + K.KEY[1])) - 0.5);
  }
  const r = a.reduce((s, v, i) => s + v * b[i], 0) / a.length / (1 / 12);
  assert(Math.abs(r) < 0.08, `adjacent cells are uncorrelated, got r=${r.toFixed(4)}`);
});

test('the noise is centred, band-limited and free of NaN over the whole model range', () => {
  const draw = stream(4242);
  const values = [];
  for (let i = 0; i < 200000; i++) values.push(sgNoise(f((draw() - 0.5) * 800), f((draw() - 0.5) * 800)));
  const m = moments(values);
  assert(values.every(Number.isFinite), 'no NaN');
  assert(m.lo > -1.0001 && m.hi < 1.0001, `stays inside -1..1, got [${m.lo.toFixed(4)}, ${m.hi.toFixed(4)}]`);
  assert(Math.abs(m.mean) < 0.01, `mean is zero, got ${m.mean.toFixed(5)}`);
  // Bilinear interpolation of independent uniform corners with a cubic weight.
  // 0.43 is what that is; anything much lower means the interpolation is
  // averaging the field away, anything higher means it is not interpolating.
  assert(m.sd > 0.40 && m.sd < 0.46, `standard deviation is about 0.43, got ${m.sd.toFixed(4)}`);
  // Continuity. A hash leaking into the interpolation shows up as a large jump
  // between two samples a thousandth of a cell apart.
  let worst = 0;
  for (let i = 0; i < 20000; i++) {
    const x = f((draw() - 0.5) * 200), y = f((draw() - 0.5) * 200);
    worst = Math.max(worst, Math.abs(sgNoise(x, y) - sgNoise(f(x + 0.001), f(y + 0.001))));
  }
  assert(worst < 0.02, `the field is continuous; worst step over 0.001 cells was ${worst.toFixed(4)}`);
  // Far from the origin the lattice key gets big and fract() starts throwing away
  // the low bits. This is the check that the small SG_KEY multipliers are doing
  // their job: a 133 m hull is 2500 cells across at the finest octave.
  for (const centre of [0, 50, 400, 2600]) {
    const far = [];
    for (let i = 0; i < 30000; i++) far.push(sgNoise(f(centre + draw() * 20), f(centre + draw() * 20)));
    const fm = moments(far);
    assert(fm.sd > 0.38, `the field still has its amplitude near ${centre}, sd ${fm.sd.toFixed(4)}`);
    assert(Math.abs(fm.mean) < 0.06, `and is not biased near ${centre}, mean ${fm.mean.toFixed(4)}`);
  }
  assert.equal(sgNoise(1.234, 5.678), sgNoise(1.234, 5.678), 'deterministic');
});

test('the four octaves are independent fields, so their amplitudes do not stack', () => {
  // Each octave after the first is turned by its own rotation and pushed by its
  // own offset. Paste the same matrix twice, or the same offset, and two octaves
  // become one field at double amplitude: the RMS goes up, the extremes go up
  // further, and the surface starts to look like camouflage. Correlation is the
  // only thing that catches that, and it catches it here rather than in a
  // screenshot.
  //
  // Sampled over 500 m rather than over one asset, because the coarsest octave
  // has a 2.5 m cycle: across a single vehicle it only ever visits a couple of
  // dozen lattice cells, and a correlation read off that few is mostly the
  // sample talking. Over 500 m it visits two hundred thousand.
  const draw = stream(606);
  const columns = [[], [], [], []];
  for (let i = 0; i < 200000; i++) {
    const q = [f((draw() - 0.5) * 500), f((draw() - 0.5) * 500)];
    columns[0].push(sgNoise(f(q[0] * K.FREQ[0]), f(q[1] * K.FREQ[0])));
    for (let o = 1; o < 4; o++) {
      const r = turn(ROT[o - 1], q);
      columns[o].push(sgNoise(f(f(r[0] * K.FREQ[o]) + OFFSET[o - 1]), f(f(r[1] * K.FREQ[o]) + OFFSET[o - 1])));
    }
  }
  const stats = columns.map(moments);
  stats.forEach((s, o) => assert(s.sd > 0.40 && s.sd < 0.46, `octave ${o} carries its amplitude, sd ${s.sd.toFixed(4)}`));
  for (let a = 0; a < 4; a++) {
    for (let b = a + 1; b < 4; b++) {
      let cov = 0;
      for (let i = 0; i < columns[a].length; i++) cov += (columns[a][i] - stats[a].mean) * (columns[b][i] - stats[b].mean);
      const r = cov / columns[a].length / (stats[a].sd * stats[b].sd);
      assert(Math.abs(r) < 0.02, `octaves ${a} and ${b} are uncorrelated, got r=${r.toFixed(4)}`);
    }
  }
});

test('every live octave is under the Nyquist limit at every framing in the deck', () => {
  const N = [0, 1, 0], V = [0, 0.4, 0.92];
  for (const framing of FRAMING) {
    const {mpp, w} = weights(N, V, 1 / depthFor(framing.size));
    const live = w.filter(x => x > 0.002).length;
    assert.equal(live, framing.live, `${framing.label}: ${live} live octaves, expected ${framing.live}`);
    K.FREQ.forEach((F, i) => {
      if (w[i] <= 0.002) return;
      const cyclesPerPixel = mpp * F;
      assert(cyclesPerPixel <= K.CUT_HI + 1e-6,
        `${framing.label}: live octave ${F} c/m runs at ${cyclesPerPixel.toFixed(3)} cycles/pixel`);
    });
    assert(w[0] > 0.002, `${framing.label}: the coarsest octave always survives, so nothing goes flat`);
  }
  // And the case the fade exists for. A vehicle drawn 40 px tall keeps at most
  // the coarsest octave, which at 2.5 m a cycle is 17 px and cannot fizz.
  const mapTexel = 2 * Math.tan(13 * Math.PI / 180) / 40;
  const map = weights(N, V, 1 / depthFor(6), mapTexel);
  assert(map.w.filter(x => x > 0.002).length <= 2, 'at map size the two finest octaves are gone');
  assert(map.w[1] < 0.35, 'and what is left of the second is a trace');
  K.FREQ.forEach((F, i) => {
    if (map.w[i] > 0.002) assert(map.mpp * F <= K.CUT_HI + 1e-6, 'nothing left at map size is near Nyquist');
  });
});

test('at map range the treatment is a bit-exact no-op', () => {
  // Not "nearly albedo". The early return hands back the same object's values, so
  // a distant model renders exactly as it does today and the fade can never leave
  // a seam between an instance that grains and one that does not.
  const albedo = [0.203, 0.241, 0.132];
  const draw = stream(7);
  for (let i = 0; i < 2000; i++) {
    let z = draw() * 2 - 1, a = draw() * Math.PI * 2, r = Math.sqrt(1 - z * z);
    const N = [r * Math.cos(a), z, r * Math.sin(a)];
    const P = [(draw() - 0.5) * 12, draw() * 4, (draw() - 0.5) * 12];
    const out = surface(albedo, N, P, [0, 0.4, 0.92], 1 / 4000);
    assert.deepEqual(out, albedo, 'identical, not approximately identical');
  }
});

test('the modulation stays inside the envelope it was designed for at every framing', () => {
  // The design target is roughly two per cent RMS of albedo, which is gentle
  // mottling, and a hard ceiling near ten per cent that the fbm reaches about as
  // often as four independent octaves all peak together. Past that it reads as
  // speckle, which is the failure this file exists to catch before a screenshot
  // does.
  const albedo = [0.20, 0.24, 0.13];
  for (const framing of FRAMING) {
    const draw = stream(99 + framing.size);
    const rel = [];
    for (let i = 0; i < 40000; i++) {
      let z = draw() * 2 - 1, a = draw() * Math.PI * 2, r = Math.sqrt(1 - z * z);
      const N = [r * Math.cos(a), z, r * Math.sin(a)];
      const half = framing.size * 0.5;
      const P = [(draw() - 0.5) * framing.size, draw() * half, (draw() - 0.5) * framing.size];
      const out = surface(albedo, N, P, [0, 0.4, 0.92], 1 / depthFor(framing.size));
      assert(out.every(Number.isFinite), 'no NaN reaches the framebuffer');
      assert(out.every(v => v >= 0), 'and nothing goes negative');
      rel.push((out[1] - albedo[1]) / albedo[1]);
    }
    const m = moments(rel);
    assert(m.sd > 0.004 && m.sd < 0.030,
      `${framing.label}: RMS modulation is ${(m.sd * 100).toFixed(2)}%, wanted roughly 1 to 2.5%`);
    assert(Math.abs(m.lo) < 0.13 && m.hi < 0.13,
      `${framing.label}: extremes are ${(m.lo * 100).toFixed(1)}% to ${(m.hi * 100).toFixed(1)}%`);
    assert(Math.abs(m.mean) < 0.02,
      `${framing.label}: the asset is not systematically brightened, mean ${(m.mean * 100).toFixed(2)}%`);
  }
});

test('the treatment reads the same on any base hue, so the finish repaint survives it', () => {
  // Sand and winter rewrite vertex colours before upload, only where the colour
  // is olive-ish. Whatever arrives here, the modulation has to be the same
  // percentage of it, or a repainted vehicle grains differently from an olive one
  // and the effect looks like a property of the paint.
  const olive = [0.20, 0.24, 0.13];
  const sand = [0.55, 0.46, 0.29];
  const winter = [0.63, 0.68, 0.65];
  const rubber = [0.035, 0.038, 0.036];
  const draw = stream(31337);
  let worst = 0;
  for (let i = 0; i < 20000; i++) {
    let z = draw() * 2 - 1, a = draw() * Math.PI * 2, r = Math.sqrt(1 - z * z);
    const N = [r * Math.cos(a), z, r * Math.sin(a)];
    const P = [(draw() - 0.5) * 6, draw() * 2.4, (draw() - 0.5) * 8];
    const args = [N, P, [0, 0.4, 0.92], 1 / depthFor(6)];
    // Recover the modulation from the untouched green channel: the shader
    // computes albedo*(1+v) + v*FLOOR, so v is a function of position and normal
    // and of nothing else.
    const recover = alb => (surface(alb, ...args)[1] - alb[1]) / (alb[1] + K.FLOOR);
    const v = recover(olive);
    for (const other of [sand, winter, rubber]) worst = Math.max(worst, Math.abs(recover(other) - v));
  }
  assert(worst < 1e-6, `the modulation is independent of the albedo it lands on; worst gap ${worst}`);
});

test('the colour temperature tilt opposes red against blue and leaves green alone', () => {
  const draw = stream(5150);
  const grey = [0.4, 0.4, 0.4];
  let tilted = 0, worst = 0;
  for (let i = 0; i < 20000; i++) {
    let z = draw() * 2 - 1, a = draw() * Math.PI * 2, r = Math.sqrt(1 - z * z);
    const N = [r * Math.cos(a), z, r * Math.sin(a)];
    const P = [(draw() - 0.5) * 6, draw() * 2.4, (draw() - 0.5) * 8];
    const out = surface(grey, N, P, [0, 0.4, 0.92], 1 / depthFor(6));
    // Red above the green line implies blue below it, by construction.
    const rd = out[0] - out[1], bd = out[2] - out[1];
    if (Math.abs(rd) > 1e-7) {tilted++; assert(rd * bd <= 0, 'red and blue move in opposite directions');}
    worst = Math.max(worst, Math.abs(rd) / grey[0]);
  }
  assert(tilted > 19000, 'nearly every sample carries some tilt');
  assert(worst < 0.02, `the tilt stays under two per cent of albedo, worst ${(worst * 100).toFixed(2)}%`);
});

test('extreme and degenerate inputs produce finite, non-negative colour', () => {
  const cases = [];
  for (const albedo of [[0, 0, 0], [1, 1, 1], [0.001, 0, 1], [0.5, 0.5, 0.5]]) {
    for (const N of [[0, 1, 0], [0, 0, 0], [-1, 0, 0], [0.577, 0.577, 0.577]]) {
      for (const P of [[0, 0, 0], [0, 0.0001, 0], [-70, 0, 70], [1e5, 1e5, 1e5]]) {
        for (const fragW of [1e-9, 1 / 0.05, 1 / 12, 1 / 4000, 1e9]) {
          cases.push([albedo, N, P, [0, 0, 1], fragW]);
        }
      }
    }
  }
  for (const [albedo, N, P, V, fragW] of cases) {
    const out = surface(albedo, N, P, V, fragW);
    assert(out.every(Number.isFinite), `finite for N=${N} P=${P} w=${fragW}`);
    assert(out.every(v => v >= 0), 'never negative');
    assert(out.every((v, i) => v <= albedo[i] * 1.13 + 0.005), 'never more than a tenth above the albedo it was given');
  }
  // A zero-length normal must not divide by zero or pick a plane at random each
  // frame; the projection compares abs components and the first branch wins.
  const flat = surface([0.3, 0.3, 0.3], [0, 0, 0], [1, 1, 1], [0, 0, 1], 1 / 12);
  assert(flat.every(Number.isFinite));
});

test('nothing about the result depends on call order or on a clock', () => {
  const albedo = [0.22, 0.26, 0.15];
  const N = [0.3, 0.9, 0.31], P = [1.1, 0.8, -2.2], V = [0, 0.4, 0.92];
  const first = surface(albedo, N, P, V, 1 / 12);
  for (let i = 0; i < 50; i++) surface([0.5, 0.1, 0.9], [1, 0, 0], [i, i, i], V, 1 / 3);
  assert.deepEqual(surface(albedo, N, P, V, 1 / 12), first, 'same inputs, same output, always');
});

test('a GPU that rounds inversesqrt differently cannot change the picture', () => {
  // inversesqrt is the one operation in the chunk a driver is allowed to
  // approximate; everything else is multiply, add, floor and fract. A couple of
  // ULP there must not move a fade weight enough to see, or the deck would look
  // different on different machines and determinism would be a claim rather than
  // a property.
  const albedo = [0.20, 0.24, 0.13];
  const draw = stream(8080);
  let worst = 0;
  for (let i = 0; i < 5000; i++) {
    let z = draw() * 2 - 1, a = draw() * Math.PI * 2, r = Math.sqrt(1 - z * z);
    const N = [r * Math.cos(a), z, r * Math.sin(a)];
    const P = [(draw() - 0.5) * 6, draw() * 2.4, (draw() - 0.5) * 8];
    const V = [0, 0.4, 0.92];
    const base = surface(albedo, N, P, V, 1 / depthFor(6));
    // Perturbing the view vector by a relative 1e-5 moves the footprint estimate
    // by more than any plausible inversesqrt error does.
    const nudged = surface(albedo, N, P, [V[0], V[1] * (1 + 1e-5), V[2]], 1 / depthFor(6));
    worst = Math.max(worst, ...base.map((v, k) => Math.abs(v - nudged[k])));
  }
  assert(worst < 1e-4, `a 1e-5 wobble in the footprint moves colour by ${worst.toExponential(2)}, under a 255th of a step`);
});

// ---------------------------------------------------------------------------
// The tests below were added by the hardening pass. Each of them found a real
// defect in the first draft of the chunk, and each carries the rejected
// behaviour inline so it can be SHOWN to go red - a check that cannot fail
// against the thing it exists to catch is not a check.
// ---------------------------------------------------------------------------

test('the field does not crack at a cell boundary, and the rejected key scheme does', () => {
  // A lattice corner belongs to two cells. Reach it as base + SG_KEY from one
  // and as (i+1)*SG_KEY from the other and the two float expressions differ by
  // an ulp - which is nothing, until it lands inside fract(), where it flips the
  // corner from 0.999 to 0.001 and drops a full-amplitude step down the cell
  // line. The shipped sgNoise builds every corner key from that corner's own
  // integer coordinates, so the two cells agree to the bit.
  const rejected = (px, py) => {          // the first draft, kept to prove teeth
    const ix = Math.floor(px), iy = Math.floor(py);
    let tx = f(px - ix), ty = f(py - iy);
    tx = f(f(tx * tx) * f(3.0 - f(2.0 * tx)));
    ty = f(f(ty * ty) * f(3.0 - f(2.0 * ty)));
    const base = f(f(ix * K.KEY[0]) + f(iy * K.KEY[1]));
    const h0 = sgHash(base), h1 = sgHash(f(base + K.KEY[0]));
    const h2 = sgHash(f(base + K.KEY[1])), h3 = sgHash(f(base + f(K.KEY[0] + K.KEY[1])));
    const a = f(h0 + f(tx * f(h1 - h0))), b = f(h2 + f(tx * f(h3 - h2)));
    return f(f(f(a + f(ty * f(b - a))) * 2.0) - 1.0);
  };
  const crackRate = (noise, lo, hi) => {
    let cracked = 0, total = 0, worst = 0;
    for (let ix = lo; ix < hi; ix++) {
      // The step either side of the line has to be RELATIVE. A fixed 1e-4 is
      // below one ulp at cell 2400, so both samples land in the same cell and
      // the test measures nothing - which is exactly how this first passed
      // against the scheme it exists to reject.
      const eps = Math.max(1e-3, Math.abs(ix + 1) * 4e-6);
      for (let iy = lo; iy < hi; iy++) {
        const y = f(iy + 0.5);
        const left = f(ix + 1 - eps), right = f(ix + 1 + eps);
        assert(Math.floor(left) === ix && Math.floor(right) === ix + 1,
          'the two samples straddle the cell line rather than landing in one cell');
        const jump = Math.abs(noise(left, y) - noise(right, y));
        if (jump > 0.02) cracked++;
        worst = Math.max(worst, jump);
        total++;
      }
    }
    return {rate: cracked / total, worst, total};
  };
  // Cell magnitudes the level of detail can actually ask for: a live octave
  // needs FREQ*extent under 0.44 of the viewport height, so a 1080 px view of
  // any asset in this deck stays inside about 500 cells. 2600 is far past that
  // and is checked anyway, because the fade is a runtime decision and this is
  // not.
  for (const [lo, hi] of [[0, 200], [400, 600], [2400, 2600]]) {
    const good = crackRate(sgNoise, lo, hi);
    assert.equal(good.rate, 0, `no corner cracks between cells ${lo} and ${hi}`);
    assert(good.worst < 0.01,
      `the field is continuous across every cell line in ${lo}..${hi}; worst step ${good.worst}`);
  }
  // And the rejected scheme fails it, which is what makes the assertion above
  // mean something. Near the origin its rate is small; when it does crack the
  // step is the full range of the field.
  const bad = crackRate(rejected, 2400, 2600);
  assert(bad.rate > 0.001, `the rejected base+offset keys crack; measured ${(bad.rate * 100).toFixed(3)}%`);
  assert(bad.worst > 0.5, `and when they crack it is full scale; worst step ${bad.worst.toFixed(3)}`);
});

test('the noise does not repeat at any lattice offset, and the rejected hash did', () => {
  // sgNoise projects a 2D lattice onto a 1D key, i.x*Kx + i.y*Ky. Integer
  // offsets (a,b) exist whose key difference lands within a thousandth of a
  // whole number, and fract() cannot tell those two corners apart unless the
  // hash changes fast on that scale. If it cannot, the whole field repeats at a
  // fixed diagonal offset and the surface starts to read as camouflage.
  //
  // The dangerous offsets are DERIVED from the shipped SG_KEY rather than
  // written down, so changing the key re-derives the danger list instead of
  // quietly invalidating this test.
  const danger = [];
  for (let a = -24; a <= 24; a++) {
    for (let b = -24; b <= 24; b++) {
      if (!a && !b) continue;
      const d = a * K.KEY[0] + b * K.KEY[1];
      danger.push({a, b, gap: Math.abs(d - Math.round(d))});
    }
  }
  danger.sort((x, y) => x.gap - y.gap);
  const offsets = danger.slice(0, 24).map(o => [o.a, o.b]);
  for (let a = -6; a <= 6; a++) for (let b = -6; b <= 6; b++) if (Math.abs(a) + Math.abs(b) > 2) offsets.push([a, b]);
  assert(danger[0].gap < 0.002, 'the projection really does have a near-collision to defend against');

  const corr = (noise, list) => {
    const draw = stream(606), n = 12000, px = [], py = [], b0 = [];
    for (let i = 0; i < n; i++) {
      const x = f((draw() - 0.5) * 400), y = f((draw() - 0.5) * 400);
      px.push(x); py.push(y); b0.push(noise(x, y));
    }
    const s0 = moments(b0);
    let worst = 0, at = null;
    for (const [a, b] of list) {
      const b1 = [];
      for (let i = 0; i < n; i++) b1.push(noise(f(px[i] + a), f(py[i] + b)));
      const s1 = moments(b1);
      let cov = 0;
      for (let i = 0; i < n; i++) cov += (b0[i] - s0.mean) * (b1[i] - s1.mean);
      const r = cov / n / (s0.sd * s1.sd);
      if (Math.abs(r) > Math.abs(worst)) { worst = r; at = [a, b]; }
    }
    return {worst, at};
  };
  const live = corr(sgNoise, offsets);
  assert(Math.abs(live.worst) < 0.08,
    `no lattice offset repeats the field; worst r=${live.worst.toFixed(4)} at (${live.at})`);

  // The interpolation kernel spans one cell, so cells one apart SHOULD share a
  // corner and correlate. An exact integer-mixing hash scores 0.177 here; a
  // field that scores zero is not interpolating and one that scores high is not
  // decorrelating, so this is bounded on both sides.
  const near = corr(sgNoise, [[1, 0], [0, 1]]);
  assert(Math.abs(near.worst) > 0.10 && Math.abs(near.worst) < 0.30,
    `adjacent cells share a corner and correlate about 0.18; got ${near.worst.toFixed(3)}`);

  // Teeth: the sensitivity this shipped with, 43 rather than SG_HASH_A, repeats
  // the field at r = 0.74. If SG_HASH_A is ever lowered back toward it, the
  // assertion above goes red instead of the artefact going quietly wrong.
  const dull = k => { const h = f(fract(k) + K.SEED); return fract(f(h * f(f(h * 43.0) + 27.3))); };
  const dullNoise = (px, py) => {
    const ix = Math.floor(px), iy = Math.floor(py);
    let tx = f(px - ix), ty = f(py - iy);
    tx = f(f(tx * tx) * f(3.0 - f(2.0 * tx)));
    ty = f(f(ty * ty) * f(3.0 - f(2.0 * ty)));
    const ck = (a, b) => f(f(a * K.KEY[0]) + f(b * K.KEY[1]));
    const h0 = dull(ck(ix, iy)), h1 = dull(ck(f(ix + 1), iy));
    const h2 = dull(ck(ix, f(iy + 1))), h3 = dull(ck(f(ix + 1), f(iy + 1)));
    const a = f(h0 + f(tx * f(h1 - h0))), b = f(h2 + f(tx * f(h3 - h2)));
    return f(f(f(a + f(ty * f(b - a))) * 2.0) - 1.0);
  };
  const rejectedCorr = corr(dullNoise, offsets);
  assert(Math.abs(rejectedCorr.worst) > 0.4,
    `the rejected hash repeats and this test sees it; got ${rejectedCorr.worst.toFixed(4)} at (${rejectedCorr.at})`);
  assert(K.HASH_A > 400, 'SG_HASH_A is high enough to separate a near-collision');
});

// --- the real deck ----------------------------------------------------------
// Everything below is read off ACTUAL vertices of ACTUAL meshes, framed the way
// arsenal3d.js frames them. A sphere of random normals cannot answer the
// question these ask, which is how far a fragment of a real model moves from
// the colour the palette gave it.
const MeshLib = require('../../spheres-web/ui/equipment-mesh.js');
const ModelLib = require('../../spheres-web/ui/equipment-model.js');
const ARSENAL_FOV = 26, ARSENAL_PITCH = 20, ARSENAL_YAW = 34;
// arsenal3d.js fits the SILHOUETTE, walking every vertex, then adds 3%.
function fitCamera(positions, centre, tanV, pitchDeg, yawDeg) {
  const rp = pitchDeg * Math.PI / 180, ry = yawDeg * Math.PI / 180;
  const fwd = [Math.sin(ry) * Math.cos(rp), Math.sin(rp), Math.cos(ry) * Math.cos(rp)];
  const c = [-fwd[1], fwd[0], 0], l = Math.hypot(...c) || 1, rt = c.map(v => v / l);
  const up = [fwd[1] * rt[2] - fwd[2] * rt[1], fwd[2] * rt[0] - fwd[0] * rt[2], fwd[0] * rt[1] - fwd[1] * rt[0]];
  let worst = 0;
  for (let i = 0; i < positions.length; i += 3) {
    const x = positions[i] - centre[0], y = positions[i + 1] - centre[1], z = positions[i + 2] - centre[2];
    const need = (x * fwd[0] + y * fwd[1] + z * fwd[2])
      + Math.max(Math.abs(x * rt[0] + y * rt[1] + z * rt[2]), Math.abs(x * up[0] + y * up[1] + z * up[2])) / tanV;
    if (need > worst) worst = need;
  }
  const d = worst * 1.03;
  return {d, eye: fwd.map((v, i) => v * d + centre[i]), fwd};
}
// Walk a mesh's real vertices and hand back what the fragment stage would see.
function* fragments(mesh, colors, cam) {
  const p = mesh.positions, nn = mesh.normals;
  for (let i = 0; i < p.length; i += 3) {
    const P = [p[i], p[i + 1], p[i + 2]];
    let N = [nn[i], nn[i + 1], nn[i + 2]];
    const albedo = [colors[i], colors[i + 1], colors[i + 2]];
    const w = [cam.eye[0] - P[0], cam.eye[1] - P[1], cam.eye[2] - P[2]];
    const len = Math.hypot(...w) || 1, V = w.map(v => v / len);
    // arsenal3d.js flips the normal toward the viewer before it calls surface().
    if (N[0] * V[0] + N[1] * V[1] + N[2] * V[2] < 0) N = N.map(v => -v);
    const depth = w[0] * cam.fwd[0] + w[1] * cam.fwd[1] + w[2] * cam.fwd[2];
    if (depth <= 0.01) continue;
    yield {P, N, V, albedo, fragW: 1 / depth};
  }
}
const PLATFORMS = ['tank_main', 'tank_light', 'ground_ifv', 'ground_apc', 'ground_artillery'];
const percentile = (sorted, p) => sorted[Math.min(sorted.length - 1, Math.floor(p / 100 * sorted.length))];

test('on the real meshes, at the real framing, the excursion is small and never clips', () => {
  const tanV = Math.tan(ARSENAL_FOV * Math.PI / 360);
  const texel = 2 * tanV / 256;
  let checked = 0;
  for (const platform of PLATFORMS) {
    const mesh = MeshLib.build({platform});
    assert(mesh && mesh.positions.length > 3000, `${platform} builds a mesh to test against`);
    const b = mesh.bounds;
    const centre = [0, 1, 2].map(k => (b.min[k] + b.max[k]) / 2);
    const cam = fitCamera(mesh.positions, centre, tanV, ARSENAL_PITCH, ARSENAL_YAW);
    for (const finish of ['olive', 'sand', 'winter']) {
      const colors = ModelLib.finishColors(mesh.colors, finish);
      const mods = [], absMods = [];
      let lo = Infinity, hi = -Infinity, n = 0;
      for (const g of fragments(mesh, colors, cam)) {
        const out = surface(g.albedo, g.N, g.P, g.V, g.fragW, texel);
        assert(out.every(Number.isFinite), `${platform}/${finish}: a fragment came back NaN`);
        for (const ch of out) { lo = Math.min(lo, ch); hi = Math.max(hi, ch); }
        // The modulation, recovered from the untilted green channel:
        // out.g = albedo.g * (1 + v) + v * SG_FLOOR.
        const v = (out[1] - g.albedo[1]) / (g.albedo[1] + K.FLOOR);
        mods.push(v); absMods.push(Math.abs(v));
        n++;
      }
      assert(n > 3000, `${platform}/${finish}: ${n} fragments sampled`);
      // NOTHING may leave the unit interval. The winter repaint lifts a bright
      // olive panel to a luminance of 0.945, and a positive excursion on top of
      // that measured 1.017 before the chunk clamped its own output.
      assert(lo >= 0 && hi <= 1,
        `${platform}/${finish}: output stayed in 0..1, got [${lo.toFixed(4)}, ${hi.toFixed(4)}]`);
      const m = moments(mods);
      absMods.sort((a, c) => a - c);
      const rms = Math.sqrt(mods.reduce((a, v) => a + v * v, 0) / n);
      assert(rms > 0.012 && rms < 0.030,
        `${platform}/${finish}: modulation is ${(rms * 100).toFixed(2)}% RMS, designed for about 2%`);
      assert(percentile(absMods, 50) < 0.025,
        `${platform}/${finish}: half the fragments move under 2.5%, p50 ${(percentile(absMods, 50) * 100).toFixed(2)}%`);
      assert(percentile(absMods, 95) < 0.055,
        `${platform}/${finish}: p95 excursion ${(percentile(absMods, 95) * 100).toFixed(2)}%`);
      assert(percentile(absMods, 99.9) < 0.080,
        `${platform}/${finish}: p99.9 excursion ${(percentile(absMods, 99.9) * 100).toFixed(2)}%`);
      // The ceiling that decides whether this reads as surface or as speckle.
      assert(absMods[absMods.length - 1] < 0.100,
        `${platform}/${finish}: worst excursion ${(absMods[absMods.length - 1] * 100).toFixed(2)}%`);
      // A whole vehicle uniformly lifted or dropped would be a repaint, not a
      // grain. The coarsest octave has a 2.5 m cycle, so a 5 m hull visits only
      // a few of its cells and this band is 1.5% rather than 0.2%.
      assert(Math.abs(m.mean) < 0.015,
        `${platform}/${finish}: the model is not systematically shaded, mean ${(m.mean * 100).toFixed(2)}%`);
      checked++;
    }
  }
  assert.equal(checked, PLATFORMS.length * 3, 'every platform was checked on every finish');
});

test('the finish repaint changes the colour and not the grain, on real vertices', () => {
  // Sand and winter rewrite VERTEX colours before upload, and only where the
  // colour is olive-ish, so one model carries repainted and untouched materials
  // side by side. The modulation has to be the same function of position and
  // normal on both, or the effect reads as a property of the paint.
  const tanV = Math.tan(ARSENAL_FOV * Math.PI / 360), texel = 2 * tanV / 256;
  const mesh = MeshLib.build({platform: 'tank_main'});
  const b = mesh.bounds;
  const centre = [0, 1, 2].map(k => (b.min[k] + b.max[k]) / 2);
  const cam = fitCamera(mesh.positions, centre, tanV, ARSENAL_PITCH, ARSENAL_YAW);
  const finishes = ['olive', 'sand', 'winter'].map(fn => ModelLib.finishColors(mesh.colors, fn));
  // The repaint has to be doing something, or this test proves nothing.
  let repainted = 0;
  for (let i = 0; i < mesh.colors.length; i += 3) if (Math.abs(finishes[2][i] - mesh.colors[i]) > 0.01) repainted++;
  assert(repainted > 1000, `the winter repaint touched ${repainted} vertices, so there is something to test`);

  // The modulation is read off a NEUTRAL probe at the same geometry - mid grey
  // cannot clip, so v comes back clean - and then every finish is asserted to be
  // that same v applied to its own colour. Recovering v from the painted
  // fragment instead breaks the moment the clamp bites, which on winter it does.
  const probes = [];
  for (const g of fragments(mesh, mesh.colors, cam)) {
    const out = surface([0.5, 0.5, 0.5], g.N, g.P, g.V, g.fragW, texel);
    probes.push((out[1] - 0.5) / (0.5 + K.FLOOR));
  }
  let worst = 0, clamped = 0, count = 0;
  for (const colors of finishes) {
    let i = 0;
    for (const g of fragments(mesh, colors, cam)) {
      const v = probes[i++];
      const out = surface(g.albedo, g.N, g.P, g.V, g.fragW, texel);
      const want = Math.min(1, Math.max(0, f(f(g.albedo[1] * f(1.0 + v)) + f(v * K.FLOOR))));
      if (want === 1 || want === 0) clamped++;
      worst = Math.max(worst, Math.abs(out[1] - want));
      count++;
    }
  }
  assert(worst < 1e-6, `the grain is the same function of geometry under every finish; worst gap ${worst}`);
  // And the ceiling is reached by a handful of fragments, not by a surface.
  assert(clamped / count < 0.001,
    `the clamp bites on ${clamped} of ${count} fragments, under a tenth of a per cent`);
});

test('the shader still computes what the port computes', () => {
  // THE BLUNT INSTRUMENT, and it earned its place. A mutation run reverted the
  // GLSL to three earlier forms - base+offset corner keys, a max() instead of a
  // clamp(), a duplicated rotation - and every statistical test in this file
  // stayed green, because they all run the PORT and the port is transcribed by
  // hand. Statistics cannot see a change the port did not receive.
  //
  // So the expressions the port mirrors are pinned here as text. It is crude and
  // it will complain about a harmless reformat; that is the correct trade
  // against a shader that silently stops being the program these numbers
  // describe. Each entry names the port line it stands for.
  const tight = bare.replace(/\s+/g, ' ');
  const must = [
    // sgHash: the port's f(h * f(f(h * K.HASH_A) + K.HASH_B))
    ['float h = fract(k) + SG_SEED;', 'sgHash seeds the key'],
    ['return fract(h * (h * SG_HASH_A + SG_HASH_B));', 'sgHash is one polynomial round'],
    // sgNoise: per-corner keys, the fix for cracked corners. The port builds
    // cornerKey(ix, iy) = ix*KEY.x + iy*KEY.y independently for all four.
    ['vec4 cx = i.x + vec4(0.0, 1.0, 0.0, 1.0);', 'corner x coordinates'],
    ['vec4 cy = i.y + vec4(0.0, 0.0, 1.0, 1.0);', 'corner y coordinates'],
    ['vec4 h = sgHash4(cx * SG_KEY.x + cy * SG_KEY.y);', 'each corner key from its own coordinates'],
    ['t = t * t * (3.0 - 2.0 * t);', 'the smoothstep weight the port squares by hand'],
    // sgPlane: the three projections, swizzle for swizzle. A mutation run turned
    // P.zy into P.yz and every statistic in this file stayed green - which is
    // the swizzle typo the header used to concede it could not see.
    ['if (a.y >= a.x && a.y >= a.z) return P.xz + P.y * vec2(0.31, 0.53);', 'the y-facing projection'],
    ['if (a.x >= a.z) return P.zy + P.x * vec2(0.53, 0.31);', 'the x-facing projection'],
    ['return P.xy + P.z * vec2(0.31, 0.53);', 'the z-facing projection'],
    // the octave block, each rotation applied to q and scaled by its own frequency
    ['m += SG_AMP.x * w.x * sgNoise(q * SG_FREQ.x);', 'the coarsest octave, unrotated'],
    ['fine = sgNoise(mat2(0.8018, 0.5976, -0.5976, 0.8018) * q * SG_FREQ.y + 5.7);', 'octave 2'],
    ['fine = sgNoise(mat2(0.6549, -0.7558, 0.7558, 0.6549) * q * SG_FREQ.z + 11.3);', 'octave 3'],
    ['fine = sgNoise(mat2(0.9111, 0.4122, -0.4122, 0.9111) * q * SG_FREQ.w + 23.9);', 'octave 4'],
    // the face drift, which is the only place the normal is quantised
    ['vec3 b = floor(N * SG_BUCKET + (0.5 + SG_DITHER * fine));', 'the face bucket'],
    ['float fh = sgHash(dot(b, SG_KEY));', 'the face hash'],
    // surface: the footprint, the fade and the early out
    ['float mpp = texel / max(gl_FragCoord.w, 1e-4) * inversesqrt(max(abs(dot(N, V)), SG_GRAZE));',
      'metres per pixel'],
    ['vec4 w = clamp((SG_CUT_HI - mpp * SG_FREQ) / (SG_CUT_HI - SG_CUT_LO), 0.0, 1.0);', 'the octave weights'],
    ['if (w.x <= 0.002) return albedo;', 'the bit-exact map-range no-op'],
    // the ends, both of them
    ['vec3 c = albedo * (1.0 + v) + v * SG_FLOOR;', 'multiplicative with an additive sliver'],
    ['return clamp(c * vec3(1.0 + t, 1.0, 1.0 - t), 0.0, 1.0);', 'clamped at BOTH ends'],
  ];
  for (const [expr, why] of must) {
    assert(tight.includes(expr.replace(/\s+/g, ' ')), `the GLSL still does this: ${why} -- ${expr}`);
  }
  // And the rejected forms must be gone, not merely shadowed.
  assert(!/float\s+base\s*=/.test(bare),
    'no scalar corner base survives; that was the cracked-corner scheme');
  assert(!/return\s+max\s*\(\s*c\s*\*/.test(bare),
    'the return is clamped at both ends, not floored at zero');
  // Three distinct rotations. Two octaves sharing one are still uncorrelated
  // fields because their frequencies differ - the octave-independence test
  // measures that and stays green - so this is a guard on the intent, which is
  // that a value lattice is axis-aligned and these models are built on the axes.
  const seen = ROT.map(m => m.join(','));
  assert.equal(new Set(seen).size, 3, 'the three octave rotations are distinct');
});

test('the clamp is load-bearing, not decorative', () => {
  // A clamp nobody ever reaches is dead code and should be deleted. This proves
  // the ceiling is actually hit on the real deck: the winter repaint lifts a
  // bright olive panel to a luminance of 0.945, and a positive excursion on top
  // of that leaves the unit interval.
  const tanV = Math.tan(ARSENAL_FOV * Math.PI / 360), texel = 2 * tanV / 256;
  let over = 0, worstUnclamped = 0, total = 0;
  for (const platform of ['tank_main', 'ground_apc', 'ground_artillery']) {
    const mesh = MeshLib.build({platform});
    const b = mesh.bounds;
    const centre = [0, 1, 2].map(k => (b.min[k] + b.max[k]) / 2);
    const cam = fitCamera(mesh.positions, centre, tanV, ARSENAL_PITCH, ARSENAL_YAW);
    const colors = ModelLib.finishColors(mesh.colors, 'winter');
    for (const g of fragments(mesh, colors, cam)) {
      const out = surface(g.albedo, g.N, g.P, g.V, g.fragW, texel);
      // Recover v from a neutral probe, which cannot clip, then rebuild what the
      // chunk WOULD have returned with no ceiling.
      const probe = surface([0.5, 0.5, 0.5], g.N, g.P, g.V, g.fragW, texel);
      const v = (probe[1] - 0.5) / (0.5 + K.FLOOR);
      const raw = f(f(g.albedo[1] * f(1.0 + v)) + f(v * K.FLOOR));
      if (raw > 1) { over++; worstUnclamped = Math.max(worstUnclamped, raw); }
      assert(out[1] <= 1 && out[1] >= 0, 'the clamped output is inside 0..1');
      total++;
    }
  }
  assert(over > 0,
    'without the ceiling at least one fragment would leave the unit interval, so the clamp is real');
  assert(worstUnclamped > 1.0 && worstUnclamped < 1.10,
    `the overshoot is a sliver, not a design fault; worst unclamped ${worstUnclamped.toFixed(4)}`);
  assert(over / total < 0.001,
    `${over} of ${total} fragments would overshoot, under a tenth of a per cent`);
});

test('the level of detail is honest at the sizes this renderer actually draws', () => {
  // arsenal3d.js draws catalogue cards at the canvas size times the device pixel
  // ratio, and bakes map sprites through sprite(id, sizePx) at anything from 8
  // to 256 px. The chunk cannot see either number; it takes uSurfaceTexel, and
  // the fallback is calibrated for 256. This records what each viewport does
  // BOTH ways, so the cost of leaving the uniform unset is a measured number in
  // this file rather than a paragraph in the notes.
  const tanV = Math.tan(ARSENAL_FOV * Math.PI / 360);
  const mesh = MeshLib.build({platform: 'tank_main', lod: 1});
  const b = mesh.bounds;
  const centre = [0, 1, 2].map(k => (b.min[k] + b.max[k]) / 2);
  const cam = fitCamera(mesh.positions, centre, tanV, ARSENAL_PITCH, ARSENAL_YAW);
  const N = [0, 1, 0], V = [0, 0.4, 0.92];
  for (const px of [40, 64, 128, 256, 512, 1080]) {
    const texel = 2 * tanV / px;
    const set = weights(N, V, 1 / cam.d, texel);
    // With the uniform set, every live octave resolves: more than two pixels a
    // cycle, which is the Nyquist limit the fade was built around.
    K.FREQ.forEach((F, i) => {
      if (set.w[i] <= 0.002) return;
      const pxPerCycle = 1 / (F * set.mpp);
      assert(pxPerCycle > 2.0,
        `${px}px with the uniform set: octave ${F} c/m runs at ${pxPerCycle.toFixed(2)} px/cycle`);
    });
    assert(set.w[0] > 0.002, `${px}px: the coarsest octave survives, so no model goes flat`);
    // The ladder has to REACH. Four octaves that are all switched off at every
    // size this renderer draws would be an effect that exists only on paper.
    const liveHere = set.w.filter(x => x > 0.002).length;
    if (px >= 1024) assert.equal(liveHere, 4, `${px}px: all four octaves are earning their cost`);
    if (px === 256) assert(liveHere >= 3, `${px}px: a catalogue card carries at least three octaves`);
    // Unset, the chunk believes it is drawing into 256 px. Above that it gives
    // away detail it could have had; below it, it keeps detail it cannot
    // resolve, and these are the numbers that say how much.
    const unset = weights(N, V, 1 / cam.d, 0);
    const liveSet = set.w.filter(x => x > 0.002).length;
    const liveUnset = unset.w.filter(x => x > 0.002).length;
    if (px === 256) {
      // SG_TEXEL is 0.0018 where the true figure is 0.0018036, so this is
      // calibration and not identity.
      assert.equal(liveUnset, liveSet, 'at 256 px the fallback keeps the right octaves');
      unset.w.forEach((w, i) => assert(Math.abs(w - set.w[i]) < 0.01,
        `at 256 px octave ${K.FREQ[i]} weighs ${w.toFixed(4)} against ${set.w[i].toFixed(4)}`));
    } else if (px < 256) {
      // It over-details rather than going flat, and the worst surviving octave
      // is still resolved by at least half a pixel, so the error is a static
      // pattern in a baked sprite and never a shimmer.
      assert(liveUnset >= liveSet, `${px}px unset keeps at least as many octaves as it should`);
      const topUnset = K.FREQ.filter((F, i) => unset.w[i] > 0.002).pop();
      const realPxPerCycle = 1 / (topUnset * set.mpp);
      assert(realPxPerCycle > 0.5,
        `${px}px unset: the finest surviving octave spans ${realPxPerCycle.toFixed(2)} real pixels`);
    } else {
      assert(liveUnset <= liveSet, `${px}px unset keeps no more octaves than it should`);
    }
  }
  // Whatever the viewport, past the coarsest octave the function is the identity
  // and a distant model is byte for byte the one that shipped.
  const gone = weights(N, V, 1 / 4000, 0);
  assert(gone.w.every(x => x <= 0.002), 'at map range every octave is off');
});

// The port is exported so a scratch harness can drive it over real geometry
// without copying it, which would defeat the point. node --test ignores this.
module.exports = {sgHash, sgNoise, sgPlane, turn, weights, surface, K, ROT, OFFSET, glsl, bare, f, fract};
