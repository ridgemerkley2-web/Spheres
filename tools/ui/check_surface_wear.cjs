// Checks for spheres-web/ui/surface-wear.js — the weathering-and-accumulation
// fragment-shader chunk.
//
// WHAT THIS FILE CANNOT DO, SAID FIRST SO NOBODY READS A GREEN RUN AS MORE THAN
// IT IS. There is no GPU in node and no attempt is made to pretend otherwise:
//
//   - It does NOT compile the GLSL. Nothing here would catch a missing
//     semicolon inside a line, a type mismatch, a call to a function that does
//     not exist in GLSL ES 3.00, or a name that collides with something the
//     host shader already declared. It checks the lexical facts a text can
//     honestly be checked for — balance, banned tokens, the entry point's exact
//     signature, namespacing — and stops there. Compiling it is a browser job.
//   - It does NOT link the chunk against a host shader, so it cannot prove the
//     splice is well formed at the seam.
//   - It does NOT rasterise anything, so it says nothing about whether the
//     result LOOKS right, whether the streaks read as streaks, or whether any
//     of it is the correct art. Those are eyes-on-a-screen questions. (The
//     ALIASING question, which used to be on this list, is now answered: a
//     software rasteriser was written against arsenal3d's real camera and the
//     result is in the aliasing test near the bottom.)
//   - The port below runs in JavaScript doubles. The shader runs in fp32. The
//     statistics agree to far more digits than the tolerances here use, but a
//     driver whose sin() differs in the seventh digit will produce a picture
//     that differs in the seventh digit, and no test in node can see that. The
//     shader is built out of SMOOTH functions precisely so that this does not
//     matter; a hash-based noise would make the same statement false.
//
// WHAT IT DOES DO. The port is the real test. Every constant and every line of
// swWeather() is transcribed from the shader source, and the field and the
// finished treatment are then exercised — over hundreds of thousands of random
// samples AND over every vertex of a real 47,288-triangle tank in all three
// finishes — to prove the properties the art rests on: that the noise is
// deterministic, zero-mean, bounded, unclipped and CONTINUOUS; that the
// treatment stays in range without its clamp ever firing; that no layer pins
// flat against a bound; that a fragment never moves further from its own colour
// than the finish selector moves it; that dirt goes DOWN and dust goes UP; that
// detail collapses with distance; and that none of it depends on the base being
// olive, which is what the sand and winter repaints require.
//
// TWO OF THESE TESTS EXIST BECAUSE THE FIRST DRAFT FAILED THEM, and both
// failures were invisible to the draft's own checks:
//
//   1. THE FOLD WAS A SEAM. swWrap folded coordinates by 2*pi on the reasoning
//      that sin is 2*pi periodic. swField is not a bare sine — it phase-warps
//      at 1.7 times the carrier — so folding by 2*pi shifted the warp by 1.7
//      periods and put a hard value cliff at every fold plane: jumps of up to
//      0.73 out of a +/-1 range, on planes 23 cm apart in the fine octave. The
//      old test for this measured sin(x) against sin(swWrap(x)), a bare sine
//      the shader never evaluates, and passed. It is now measured THROUGH
//      swField, and a band-limit test walks each octave at 0.1 mm and fails on
//      any step a continuous field could not make.
//   2. THE HARD CLAMP DELETED THE ART IT WAS PROTECTING. clamp(dust, 0, 1) has
//      zero slope above 1, so on 26% of the tank's dark up-facing area — the
//      tops of tracks and road wheels — the mottle was being clamped away and
//      the surface went flat, which is the exact uniform-panel failure this
//      chunk exists to remove. Nothing in the old file looked at a real mesh.
//
// THE PORT IS ONLY EVIDENCE WHILE IT IS STILL THE SHADER, so two tests tie them
// together: the numeric literals are compared as an ordered SEQUENCE, and the
// shader body is pinned by hash. This was measured rather than hoped for.
// Fourteen deliberate defects were injected into the shader — an inverted
// gravity gradient, a fade pinned open, a dropped bracket, a smuggled sampler,
// a clock, an fwidth, a renamed entry point, an un-namespaced helper, a
// reserved word, a moved constant, a removed fold, and three changed amplitudes
// — and this file went red on all fourteen. Six of them tripped only the hash,
// which is the honest measure of how much of this is lexical rather than
// semantic: without a compiler, quite a lot of it.
const {test} = require('node:test');
const assert = require('node:assert/strict');
const path = require('node:path');
const crypto = require('node:crypto');

const MODULE_PATH = path.resolve(__dirname, '../../spheres-web/ui/surface-wear.js');
const chunk = require(MODULE_PATH);
// The real geometry the treatment ships against, and the real repaint that
// rewrites its vertex colours before upload. Building the inspection tank costs
// about 40 ms, which is a cheap price for testing the thing that ships instead
// of testing random numbers.
const MESH_PATH = path.resolve(__dirname, '../../spheres-web/ui/equipment-mesh.js');
const MODEL_PATH = path.resolve(__dirname, '../../spheres-web/ui/equipment-model.js');
const {build} = require(MESH_PATH);
const {finishColors} = require(MODEL_PATH);

// --------------------------------------------------------------- the port
// Transcribed from chunk.glsl. Keep the numeric literals spelled exactly as the
// shader spells them: the literal-coverage test below is what stops this port
// and the shader drifting apart, and it is the only reason the statistics in
// this file say anything about the thing that actually ships.
const SW_PERIOD = 62.8318530718;
const SW_INV_PERIOD = 0.015915494309;
const SW_AXIS_A = [0.89050, -0.26438, 0.37030];
const SW_AXIS_B = [0.37232, 0.89121, -0.25907];
const SW_AXIS_C = [0.26152, -0.36857, -0.89205];
const SW_SOURCE_LIFT = [-0.495845, 1.671464, -0.691253];
const SW_DUST = [0.520, 0.470, 0.380];
const SW_GRIME = [0.075, 0.058, 0.042];
const PORT_CONSTANTS = [SW_PERIOD, SW_INV_PERIOD, ...SW_AXIS_A, ...SW_AXIS_B, ...SW_AXIS_C,
  ...SW_SOURCE_LIFT, ...SW_DUST, ...SW_GRIME];

const fract = (x) => x - Math.floor(x);
const clamp = (x, a, b) => Math.max(a, Math.min(b, x));
const mix = (a, b, t) => a + (b - a) * t;
const smoothstep = (e0, e1, x) => {
  const t = clamp((x - e0) / (e1 - e0), 0.0, 1.0);
  return t * t * (3.0 - 2.0 * t);
};
const dot3 = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const lumOf = (c) => dot3(c, [0.299, 0.587, 0.114]);

function swBasis(p) {
  return [dot3(p, SW_AXIS_A), dot3(p, SW_AXIS_B), dot3(p, SW_AXIS_C)];
}
function swWrap(x) {
  return x.map((v) => (fract(v * SW_INV_PERIOD + 0.5) - 0.5) * SW_PERIOD);
}
function swField(q) {
  const w = [q[1], q[2], q[0]];               // q.yzx
  // Mapped rather than unrolled, so each constant appears exactly once here as
  // it does in the shader — the transcription test compares the two in order.
  const s = q.map((v, i) => Math.sin(v + 0.85 * Math.sin(w[i] * 1.7 + 0.6)));
  return s[0] * s[1] * s[2];
}
function swSoft(x) {
  const c = Math.max(x, 0.0);
  return c * (1.0 / Math.sqrt(1.0 + c * c));  // GLSL spells this inversesqrt
}
function swWeather(albedoIn, N, P, depth) {
  let albedo = albedoIn.slice();
  const fFine = 1.0 - smoothstep(10.0, 22.0, depth);
  const fStreak = 1.0 - smoothstep(12.0, 26.0, depth);
  const fMid = 1.0 - smoothstep(48.0, 100.0, depth);
  const fCoarse = 1.0 - smoothstep(78.0, 165.0, depth);
  const lum = dot3(albedo, [0.299, 0.587, 0.114]);
  const hi = Math.max(albedo[0], Math.max(albedo[1], albedo[2]));
  const lo = Math.min(albedo[0], Math.min(albedo[1], albedo[2]));
  const sat = (hi - lo) / Math.max(hi, 1e-4);
  const darkness = 1.0 - smoothstep(0.085, 0.175, lum);
  const shiny = smoothstep(0.52, 0.88, lum);
  const mark = smoothstep(0.42, 0.66, sat);
  const h = P[1];
  const splash = 1.0 - smoothstep(-0.20, 0.85, h);
  const lowly = 1.0 - smoothstep(0.15, 3.40, h);
  const up = N[1];
  const upFace = Math.max(up, 0.0) * Math.max(up, 0.0);
  const downFace = Math.max(-up, 0.0);
  const vertFace = (1.0 - Math.abs(up)) * (1.0 - Math.abs(up));
  const b = swBasis(P);
  const m1 = swField(swWrap(b.map((v) => v * 5.60)));
  const m2 = swField(swWrap(b.map((v, i) => v * 3.41 + SW_SOURCE_LIFT[i])));
  const fine = swField(swWrap(b.map((v) => v * 27.0 + 2.13)));
  const streakN = swField(swWrap([P[0] * 21.0 + P[2] * 8.0, P[1] * 3.0, P[2] * 21.0 - P[0] * 8.0]));
  const mottle = 0.80 * m1 * fMid + 0.62 * m2 * fCoarse;
  const blotch = clamp(0.5 + 0.5 * mottle, 0.0, 1.0);
  const tooth = (0.130 * fine * fFine + 0.090 * mottle) * (1.0 - 0.45 * shiny);
  const headroom = albedo.map((v) => Math.min(v, 1.0 - v));
  albedo = albedo.map((v, i) => v + Math.max(tooth, 0.0) * headroom[i] + Math.min(tooth, 0.0) * v);
  const dust = upFace * (0.55 + 0.45 * lowly) * (0.35 + 0.65 * blotch)
    * (1.0 - 0.55 * darkness) * (1.0 - 0.55 * mark);
  albedo = albedo.map((v, i) => mix(v, SW_DUST[i], swSoft(dust) * 0.26));
  const source = smoothstep(-0.05, 0.45, m2);
  const streak = smoothstep(0.04, 0.42, streakN) * source * vertFace * fStreak
    * (0.30 + 0.70 * lowly);
  const grime = (0.90 * splash * (0.25 + 0.75 * (1.0 - upFace))
    + 0.60 * downFace * lowly
    + 0.75 * streak) * (0.55 + 0.45 * blotch)
    * (1.0 - 0.45 * shiny) * (1.0 - 0.50 * mark);
  albedo = albedo.map((v, i) => mix(v, SW_GRIME[i], swSoft(grime) * 0.42));
  const an = N.map(Math.abs);
  const diag = clamp((an[0] + an[1] + an[2] - Math.max(an[0], Math.max(an[1], an[2])) - 0.36) * 1.7, 0.0, 1.0);
  const wear = diag * (0.12 + 0.88 * upFace) * (1.0 - splash)
    * (1.0 - darkness) * (1.0 - shiny)
    * smoothstep(-0.15, 0.55, 0.70 * m1 + 0.50 * fine);
  const worn = clamp(lum * 1.45 + 0.03, 0.0, 1.0);
  albedo = albedo.map((v) => mix(v, worn, clamp(wear, 0.0, 1.0) * 0.24));
  PRE_CLAMP.length = 0;
  PRE_CLAMP.push(...albedo);                  // for the clamp-never-fires test
  return albedo.map((v) => clamp(v, 0.0, 1.0));
}
// The value swWeather was about to return before its final clamp. Kept outside
// the function so the function stays a line-for-line transcription of the
// shader and the literal test keeps working; read it immediately after a call.
const PRE_CLAMP = [];

// In the shader's own order — constants, then the four noise functions, then
// the treatment — because the test below compares the two as SEQUENCES.
const PORT_SOURCE = PORT_CONSTANTS.join(' ') + '\n'
  + [swBasis, swWrap, swField, swSoft, swWeather].map((f) => f.toString()).join('\n');

// ------------------------------------------------------------- sampling aid
// A named linear congruential generator rather than Math.random, so a failure
// names the same sample on the next run and can be looked at.
function rng(seed) {
  let state = seed >>> 0;
  return () => { state = (Math.imul(state, 1664525) + 1013904223) >>> 0; return state / 4294967296; };
}
function moments(values) {
  let mean = 0;
  for (const v of values) mean += v;
  mean /= values.length;
  let acc = 0;
  for (const v of values) acc += (v - mean) * (v - mean);
  return {mean, sd: Math.sqrt(acc / values.length), variance: acc / values.length};
}
function percentile(sorted, q) {
  const i = (sorted.length - 1) * q;
  const lo = Math.floor(i), hi = Math.ceil(i);
  return sorted[lo] + (sorted[hi] - sorted[lo]) * (i - lo);
}
const AXES = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];
// Real vertex colours measured out of arsenal-models.js, plus the two repaints
// equipment-model.js produces from the mid olive.
const BASES = {
  olive: [0.198, 0.232, 0.176], darkOlive: [0.117, 0.120, 0.109],
  rubber: [0.086, 0.092, 0.101], steel: [0.346, 0.358, 0.371],
  optic: [0.560, 0.580, 0.600], lamp: [0.950, 0.950, 0.920],
  marking: [0.504, 0.189, 0.161], concrete: [0.440, 0.430, 0.410],
  sandRepaint: [0.421, 0.348, 0.218], winterRepaint: [0.457, 0.494, 0.472],
};
// Luminance of a panel sampled over a blotch of hull, one facing, one distance.
function panel(base, normal, hFrom, hTo, depth, count, seed) {
  const next = rng(seed);
  const out = [];
  for (let i = 0; i < count; i += 1) {
    const n = normal || AXES[Math.floor(next() * 6)];
    out.push(lumOf(swWeather(base, n,
      [(next() - 0.5) * 3.2, hFrom + next() * (hTo - hFrom), (next() - 0.5) * 7.0], depth)));
  }
  return out;
}
// The inspection tank: 47,288 triangles, 141,864 vertices, Y from 0 to 3.87 m,
// and arsenal3d frames it at 15.5 m for a card of any pixel size. Built once.
let TANK = null;
function tank() {
  if (!TANK) TANK = build({platform: 'tank_standard'});
  return TANK;
}
const TANK_CARD_DISTANCE = 15.5;
// Every vertex of the tank in one finish, as {albedo, normal, position}.
function tankVertices(finish) {
  const mesh = tank();
  const colours = finish === 'olive' ? mesh.colors : finishColors(mesh.colors, finish);
  const out = [];
  for (let i = 0; i < mesh.positions.length; i += 3) {
    out.push({
      c: [colours[i], colours[i + 1], colours[i + 2]],
      n: [mesh.normals[i], mesh.normals[i + 1], mesh.normals[i + 2]],
      p: [mesh.positions[i], mesh.positions[i + 1], mesh.positions[i + 2]],
    });
  }
  return out;
}

// ------------------------------------------------------- the module's shape
test('the module exports a frozen, interchangeable chunk', () => {
  assert.equal(typeof chunk, 'object');
  assert.ok(Object.isFrozen(chunk), 'a proposal that can be edited after loading is not a proposal');
  assert.deepEqual(Object.keys(chunk).sort(), ['glsl', 'id', 'notes']);
  assert.equal(typeof chunk.id, 'string');
  assert.match(chunk.id, /^[a-z][a-z0-9-]*$/, 'id is a key, not a sentence');
  assert.equal(typeof chunk.glsl, 'string');
  assert.ok(chunk.glsl.length > 400, 'glsl is present');
  assert.equal(typeof chunk.notes, 'string');
  assert.ok(chunk.notes.length > 80, 'notes actually document the thing');
  // The one host assumption has to be stated where a reader of the manifest
  // will see it, not only where a reader of the shader will — and so does the
  // fact that it is a distance and NOT a pixel size, which is the thing a
  // reader would otherwise assume and act on.
  assert.match(chunk.notes, /gl_FragCoord/, 'the built-in it reads is documented in notes');
  assert.match(chunk.notes, /NOT pixel size/i, 'notes say what depth cannot tell it');
});

// ----------------------------------------------------------- the GLSL text
// Everything below reads the COMMENT-STRIPPED source, because that is what a
// compiler sees. Checking banned tokens against the commented source would
// forbid the comments from explaining why the token is banned.
function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, ' ').replace(/\/\/[^\n]*/g, ' ');
}
const CODE = stripComments(chunk.glsl);

test('the chunk is a chunk: no directives, no declarations, no entry function of its own', () => {
  assert.ok(!/\/\*|\*\//.test(CODE), 'no unterminated block comment survived stripping');
  assert.ok(!CODE.includes('#'), 'no preprocessor directive of any kind, version pragma included');
  for (const banned of ['texture', 'sampler', 'main(', 'fwidth', 'dFdx', 'dFdy', 'discard']) {
    assert.ok(!CODE.includes(banned), `banned token present: ${banned}`);
  }
  // No clock, under any of the names one would arrive under. Determinism is
  // the rule this protects; a shimmering surface would also be motion that the
  // reduced-motion setting has no way to stop.
  assert.ok(!/\b(time|uTime|iTime|frame|elapsed|seconds)\b/i.test(CODE), 'no clock of any name');
  // Storage qualifiers. A chunk that declares its own varyings or uniforms is
  // not spliceable, and the brief allows uniforms only if they are documented;
  // this one documents none, so it may declare none.
  for (const qualifier of ['uniform', 'attribute', 'varying', 'layout', 'precision',
    'in', 'out', 'inout', 'highp', 'mediump', 'lowp', 'flat', 'smooth', 'centroid']) {
    assert.ok(!new RegExp(`\\b${qualifier}\\b`).test(CODE), `declares or qualifies with: ${qualifier}`);
  }
});

test('no identifier in the chunk is a word some GLSL front end has taken', () => {
  // The nearest thing to a compiler that can honestly run here. ESSL 3.00's
  // own reserved list, plus the words later ESSL and desktop GLSL took, because
  // several drivers share a front end with desktop GLSL and will reject a
  // variable named after a keyword the target language does not have. This is
  // how `patch` — fine in ESSL 3.00, a tessellation keyword from 3.20 — was
  // found and renamed before anybody had to see a card fail to draw.
  const reserved = ['common', 'partition', 'active', 'asm', 'class', 'union', 'enum',
    'typedef', 'template', 'this', 'packed', 'goto', 'inline', 'noinline', 'volatile',
    'public', 'static', 'extern', 'external', 'interface', 'long', 'short', 'double',
    'half', 'fixed', 'unsigned', 'superp', 'input', 'output', 'filter', 'sizeof', 'cast',
    'namespace', 'using', 'row_major', 'patch', 'sample', 'subroutine', 'resource',
    'coherent', 'restrict', 'readonly', 'writeonly', 'atomic_uint', 'noperspective',
    'shared', 'buffer', 'invariant', 'struct', 'switch', 'case', 'default', 'while',
    'do', 'for', 'if', 'else', 'break', 'continue', 'return', 'void', 'true', 'false'];
  const declared = [...CODE.matchAll(/\b(?:float|int|bool|vec[234]|mat[234]|uint|ivec[234]|bvec[234])\s+([A-Za-z_]\w*)/g)]
    .map((m) => m[1]);
  assert.ok(declared.length > 25, 'the chunk really does declare the things this walks');
  for (const name of declared) {
    assert.ok(!reserved.includes(name), `declared name is a reserved word somewhere: ${name}`);
  }
});

test('every bracket in the chunk closes', () => {
  const pairs = {')': '(', '}': '{', ']': '['};
  const stack = [];
  for (const ch of CODE) {
    if (ch === '(' || ch === '{' || ch === '[') stack.push(ch);
    else if (pairs[ch]) assert.equal(stack.pop(), pairs[ch], `unbalanced ${ch}`);
  }
  assert.equal(stack.length, 0, `${stack.length} bracket(s) left open`);
});

test('the declared entry point is present, exactly once, with the exact signature', () => {
  const signature = /vec3\s+surface\s*\(\s*vec3\s+albedo\s*,\s*vec3\s+N\s*,\s*vec3\s+P\s*,\s*vec3\s+V\s*\)/g;
  assert.equal((CODE.match(signature) || []).length, 1, 'exactly one surface(vec3,vec3,vec3,vec3)');
});

test('everything except the entry point is namespaced, so two proposals can share a page', () => {
  const functions = [...CODE.matchAll(/(?:^|\n)\s*(?:void|float|int|bool|vec[234]|mat[234])\s+([A-Za-z_]\w*)\s*\(/g)]
    .map((m) => m[1]);
  assert.ok(functions.includes('surface'));
  for (const name of functions) {
    assert.ok(name === 'surface' || /^sw[A-Z]/.test(name), `un-namespaced function: ${name}`);
  }
  const constants = [...CODE.matchAll(/\bconst\s+(?:float|int|bool|vec[234]|mat[234])\s+([A-Za-z_]\w*)/g)]
    .map((m) => m[1]);
  assert.ok(constants.length >= 6, 'the constants are declared as constants');
  for (const name of constants) assert.ok(/^SW_/.test(name), `un-namespaced constant: ${name}`);
});

test('gl_FragCoord is the only built-in read, and only inside surface()', () => {
  const builtins = new Set([...CODE.matchAll(/\bgl_\w+/g)].map((m) => m[0]));
  assert.deepEqual([...builtins], ['gl_FragCoord']);
  // It must sit in the entry point alone: swWeather has to stay a pure
  // function of its arguments or the port below proves nothing about it.
  const body = CODE.slice(CODE.indexOf('vec3 swWeather'), CODE.indexOf('vec3 surface'));
  assert.ok(!body.includes('gl_'), 'swWeather() reads no built-in and is portable');
});

// ------------------------------------------- the port matches what it ports
test('the shader has not changed under the port without anyone looking', () => {
  // MEASURED, NOT ASSUMED, AND THIS IS WHY IT IS HERE. Before this test
  // existed, ten deliberate defects were injected into the shader and this file
  // was run against each. Eight went red. Two did not — inverting the splash
  // gradient, and pinning the fine octave's fade open — because both change
  // STRUCTURE without changing a single numeric literal, and every behavioural
  // test below measures the PORT rather than the shader. Nothing that runs in
  // node can close that gap: it would take a compiler and a rasteriser.
  //
  // So this closes it the only honest way, by refusing to let the shader move
  // quietly. The hash covers the comment-stripped, whitespace-collapsed source
  // from the top through the entry point, so comments and indentation are free
  // to change and no token is. IF THIS TRIPS: re-transcribe the port from the
  // new source, re-run the goldens in this file, and only then re-pin the
  // hash. Re-pinning first defeats the whole file.
  const canonical = CODE.replace(/\s+/g, ' ').trim();
  const digest = crypto.createHash('sha256').update(canonical).digest('hex');
  assert.equal(digest, '8b16735582c05b5e3eb9871d95a40f56dbf947a19e33856bba6ebf2664037a49', 'shader body changed; re-transcribe the port before re-pinning');
});

test('the port is a literal-for-literal transcription, in order', () => {
  // EVERYTHING STATISTICAL IN THIS FILE RESTS ON THIS ONE TEST. The port is
  // only evidence about the shader for as long as it IS the shader; a set
  // comparison is not enough, because changing 0.42 to 0.30 in the shader
  // leaves a set that still fits inside the port's. So the two are compared as
  // ordered sequences with 0, 1 and 2 dropped — those are the structural
  // constants (0.0, 1.0) and the swizzle indices the port needs where the
  // shader writes .x/.y/.z, and they carry no tuning. Any real edit to the
  // shader lands here as a mismatch at the exact index where the two diverge.
  const literals = (src) => [...src.matchAll(/(?<![A-Za-z0-9_.])\d+(?:\.\d*)?(?:[eE][+-]?\d+)?/g)]
    .map((m) => Number(m[0])).filter((v) => Number.isFinite(v) && v !== 0 && v !== 1 && v !== 2);
  // The entry point has no counterpart in the port: it exists to turn a
  // built-in into the depth argument, which is the one thing node cannot see.
  const treatment = CODE.slice(0, CODE.indexOf('vec3 surface'));
  const inShader = literals(treatment);
  const inPort = literals(PORT_SOURCE);
  assert.ok(inShader.length > 90, 'the shader really does carry the tuning this port claims to mirror');
  for (let i = 0; i < Math.max(inShader.length, inPort.length); i += 1) {
    assert.equal(inPort[i], inShader[i],
      `port and shader diverge at constant ${i}: shader has ${inShader[i]}, port has ${inPort[i]}`);
  }
});

test('the noise axes are orthonormal and clear of every direction the models use', () => {
  for (const axis of [SW_AXIS_A, SW_AXIS_B, SW_AXIS_C]) {
    assert.ok(Math.abs(Math.hypot(...axis) - 1) < 1e-4, 'axis is a unit vector');
  }
  assert.ok(Math.abs(dot3(SW_AXIS_A, SW_AXIS_B)) < 1e-4);
  assert.ok(Math.abs(dot3(SW_AXIS_A, SW_AXIS_C)) < 1e-4);
  assert.ok(Math.abs(dot3(SW_AXIS_B, SW_AXIS_C)) < 1e-4);
  // THIS IS THE ONE THAT EARNED ITS PLACE. The deck is axis-aligned boxes with
  // chamfers between them, so the 26 lattice directions — 6 face normals, 12
  // edge diagonals, 8 corner diagonals — are the directions its geometry
  // actually points in, and a noise axis lying along one of them puts the
  // field's planes flat against real panels. The first frame written for this
  // shader sat 5 degrees off (0,1,1), which is the direction of every top
  // chamfer in the deck, and nothing else here would have noticed.
  const lattice = [];
  for (let i = -1; i <= 1; i += 1) {
    for (let j = -1; j <= 1; j += 1) {
      for (let k = -1; k <= 1; k += 1) {
        if (i || j || k) { const len = Math.hypot(i, j, k); lattice.push([i / len, j / len, k / len]); }
      }
    }
  }
  let worst = 180;
  for (const axis of [SW_AXIS_A, SW_AXIS_B, SW_AXIS_C]) {
    for (const dir of lattice) {
      worst = Math.min(worst, Math.acos(Math.min(1, Math.abs(dot3(axis, dir)))) * 180 / Math.PI);
    }
  }
  // 26.69 degrees is the numerical maximum for any orthonormal frame; below
  // about 20 the pattern starts to line up with the panels it sits on.
  assert.ok(worst > 20, `a noise axis is only ${worst.toFixed(2)} degrees off a lattice direction`);
});

test('SW_SOURCE_LIFT really is the coarse coordinate lifted 0.55 m', () => {
  // The streak source is meant to be sampled ABOVE the fragment. That claim is
  // a folded constant in the shader, so it can go wrong silently; here it is,
  // recomputed from the definition it is supposed to be.
  const derived = swBasis([0.0, 0.55, 0.0]).map((v) => v * 3.41);
  for (let i = 0; i < 3; i += 1) assert.ok(Math.abs(derived[i] - SW_SOURCE_LIFT[i]) < 1e-5,
    `SW_SOURCE_LIFT[${i}] is ${SW_SOURCE_LIFT[i]}, definition gives ${derived[i]}`);
});

test('SW_PERIOD is derived from the warp rate, not chosen', () => {
  // THE CONSTANT THIS WHOLE FILE TURNS ON. swField warps phase at some rate r
  // times the carrier, so the field repeats after the SMALLEST number of carrier
  // periods n for which r*n is a whole number — n = 10 for r = 1.7. Fold
  // anywhere else and the fold is a cliff, not an identity. So the rate is read
  // back out of the shader source and the period is recomputed from it: change
  // 1.7 to 1.6 and this fails, pointing at the period that would then be right.
  const warp = CODE.match(/q\.yzx\s*\*\s*([0-9.]+)/);
  assert.ok(warp, 'the warp rate is where this expects to find it');
  const rate = Number(warp[1]);
  let n = 0;
  for (n = 1; n <= 1000; n += 1) {
    const k = rate * n;
    if (Math.abs(k - Math.round(k)) < 1e-9) break;
  }
  assert.ok(n <= 1000, `warp rate ${rate} has no small period; the field never repeats and cannot be folded`);
  const required = n * 2 * Math.PI;
  assert.ok(Math.abs(SW_PERIOD - required) < 1e-6,
    `warp rate ${rate} needs a period of ${n} * 2pi = ${required}, but SW_PERIOD is ${SW_PERIOD}`);
  assert.ok(Math.abs(SW_PERIOD * SW_INV_PERIOD - 1) < 1e-9,
    'SW_INV_PERIOD is not the reciprocal of SW_PERIOD, so the fold lands off-period');
  // And the shader must fold by the period, not by 2*pi: the whole bug was a
  // constant that looked right in isolation.
  assert.ok(!/\b6\.283185/.test(CODE), 'a bare 2*pi is back in the source; that is the seam bug');
});

// ------------------------------------------------------ the noise, measured
test('the fold is an identity THROUGH the field, not merely through a sine', () => {
  // THE TEST THAT REPLACES THE ONE THAT LIED. The old version of this measured
  // sin(x) against sin(swWrap(x)) and passed, because a bare sine really is
  // 2*pi periodic. The shader does not evaluate a bare sine: swField warps
  // phase at 1.7 times the carrier, so under a 2*pi fold it jumped by up to
  // 0.73 of its own +/-1 range at every fold plane. The claim that matters is
  // about swField, so that is what is measured.
  const next = rng(5150);
  let worst = 0;
  for (let i = 0; i < 20000; i += 1) {
    const q = [(next() - 0.5) * 12, (next() - 0.5) * 12, (next() - 0.5) * 12];
    const axis = Math.floor(next() * 3);
    const k = [1, -1, 2, -3, 7][Math.floor(next() * 5)];
    const shifted = q.slice();
    shifted[axis] += k * SW_PERIOD;
    worst = Math.max(worst, Math.abs(swField(q) - swField(shifted)));
  }
  assert.ok(worst < 1e-8, `folding by one period changes the field by ${worst}; the fold is a seam`);
  // The bare-sine identity is still true and still worth stating, because it is
  // why the fold is safe at all; it is simply not sufficient.
  let sine = 0;
  for (let i = 0; i < 20000; i += 1) {
    const x = (next() - 0.5) * 24000;
    sine = Math.max(sine, Math.abs(Math.sin(x) - Math.sin(swWrap([x, 0, 0])[0])));
  }
  assert.ok(sine < 1e-5, `bare fold drifts by ${sine} at 12,000 radians`);
  // And nothing above 10*pi ever reaches a sine, which is the fp32 argument.
  for (const v of swWrap([1e5, -1e5, 12345.678])) {
    assert.ok(Math.abs(v) <= SW_PERIOD / 2 + 1e-9, 'the fold does not bound its output');
  }
});

test('every octave is band limited: no step a continuous field could not make', () => {
  // THE REGRESSION TEST FOR THE SEAM, and the only one that would have caught
  // it from the outside. A field built from sines of angular frequency w is
  // bounded in slope by |f'| <= w_max, so walking it at 0.1 mm can only ever
  // step by w_max * 1e-4. The seam stepped by 0.73 in one 0.1 mm, i.e. a slope
  // of 7300 per metre in an octave whose carrier is 3.41 per metre. The bounds
  // below are the measured peak slopes with a factor of two of headroom.
  const octave = (p, f, off) => swField(swWrap(swBasis(p).map((v, i) => v * f + (off ? off[i] : 0))));
  const bands = [
    ['fine', 27.0, [2.13, 2.13, 2.13], 100.0],
    ['mid', 5.60, null, 22.0],
    ['coarse', 3.41, SW_SOURCE_LIFT, 14.0],
  ];
  const STEP = 1e-4;
  for (const [name, freq, off, maxSlope] of bands) {
    const dir = [0.7071, 0.0, 0.7071];
    const origin = [-8.0, 1.4, -8.0];
    let previous = octave(origin, freq, off), worst = 0, at = 0;
    for (let t = STEP; t < 12.0; t += STEP) {
      const p = [origin[0] + dir[0] * t, origin[1], origin[2] + dir[2] * t];
      const v = octave(p, freq, off);
      const slope = Math.abs(v - previous) / STEP;
      if (slope > worst) { worst = slope; at = t; }
      previous = v;
    }
    assert.ok(worst < maxSlope,
      `${name} octave slopes at ${worst.toFixed(1)}/m near t=${at.toFixed(3)} m (bound ${maxSlope}/m): that is a discontinuity, not a feature`);
  }
});

test('each octave repeats further apart than the model it sits on', () => {
  // Folding makes every octave strictly periodic, so the question is whether
  // the repeat is longer than the thing it is painted on. The mid and coarse
  // bands carry the mottle a viewer reads as pattern; at the old 2*pi fold they
  // repeated every 1.12 m and 1.84 m, i.e. eight times across one tank.
  const TANK_LENGTH = 8.9;
  for (const [name, freq] of [['mid', 5.60], ['coarse', 3.41]]) {
    const repeat = SW_PERIOD / freq;
    assert.ok(repeat > TANK_LENGTH,
      `${name} octave repeats every ${repeat.toFixed(2)} m, inside a ${TANK_LENGTH} m model`);
  }
  // And the property that actually holds every band together, stated as a ratio
  // so it does not depend on any one frequency: folding on the FIELD's period
  // rather than the sine's makes every octave repeat after ten of its own
  // carrier wavelengths instead of one. A regression to a 2*pi fold makes this
  // ratio exactly 1 and reds here as well as in the band-limit test — which is
  // the point of having it, since a seam and a tight repeat are the same defect
  // seen from two sides. The bar is 8 rather than 10 only so that a future
  // change of warp rate to, say, 9/8 is judged on its own arithmetic.
  for (const [name, freq] of [['fine', 27.0], ['mid', 5.60], ['coarse', 3.41]]) {
    const carrier = 2 * Math.PI / freq;
    const repeat = SW_PERIOD / freq;
    assert.ok(repeat / carrier > 8,
      `${name} octave repeats after only ${(repeat / carrier).toFixed(1)} of its own wavelengths`);
  }
});

test('the field is deterministic, bounded, zero-mean and unclipped', () => {
  const next = rng(20240907);
  let count = 0, sum = 0, sumSq = 0, min = Infinity, max = -Infinity, nonFinite = 0, pinned = 0;
  for (let i = 0; i < 120000; i += 1) {
    // The whole spatial range the deck occupies: a 0.8 m loitering munition
    // through a 400 m task group, above and below the ground plane.
    const p = [(next() - 0.5) * 400, (next() - 0.25) * 100, (next() - 0.5) * 400];
    const v = swField(swWrap(swBasis(p).map((x) => x * 5.60)));
    if (!Number.isFinite(v)) { nonFinite += 1; continue; }
    count += 1; sum += v; sumSq += v * v;
    if (v < min) min = v;
    if (v > max) max = v;
    if (Math.abs(v) > 0.999999) pinned += 1;
  }
  assert.equal(nonFinite, 0, 'no NaN and no infinity anywhere in the field');
  assert.ok(min > -1.0 && max < 1.0, `field escaped [-1,1]: ${min} .. ${max}`);
  assert.equal(pinned, 0, 'the field never clips against its own bound');
  const mean = sum / count, variance = sumSq / count - mean * mean;
  // Designed for: mean 0 by symmetry, variance 1/8 for three independent sines.
  // The band is tight around the measured 0.1247 on purpose. It used to be
  // [0.12, 0.15] around a measured 0.1337 — and that excess over the design
  // value was not the phase warp correlating the sines, as the comment here
  // used to claim, it was the seam: a discontinuous field carries extra energy.
  // Removing the seam moved the variance onto 1/8, which is the design value,
  // and a band this tight will notice if it ever moves off again.
  assert.ok(Math.abs(mean) < 0.01, `field mean ${mean} is not zero`);
  assert.ok(variance > 0.118 && variance < 0.132, `field variance ${variance} is off design (0.125)`);
});

test('the field returns the same numbers it returned when it was tuned', () => {
  // Goldens, so that a future edit to the noise has to be a deliberate one.
  const goldens = [
    [[0, 0, 0], 0.098439214308],
    [[1.5, 0.75, -2.25], 0.684567597071],
    [[-3.125, 2.5, 7.0625], -0.950753770689],
    [[133.0, -5.0, 60.5], 0.080226309995],
    [[-400.0, 51.0, 333.25], -0.001755831816],
  ];
  for (const [p, want] of goldens) {
    const got = swField(swWrap(swBasis(p).map((x) => x * 5.60)));
    assert.ok(Math.abs(got - want) < 1e-9, `swField at ${p}: ${got} != ${want}`);
    assert.equal(got, swField(swWrap(swBasis(p).map((x) => x * 5.60))), 'and it is stable across calls');
  }
});

test('the two coarse octaves do not share a period', () => {
  // If they did, their sum would repeat every 11.2 m and a hull side would read
  // as wallpaper. 5.60 / 3.41 = 1.6422; the check is that the cheap rational
  // approximations are not close enough to matter over a model's length.
  const ratio = 5.60 / 3.41;
  for (let q = 1; q <= 12; q += 1) {
    const p = Math.round(ratio * q);
    const err = Math.abs(ratio - p / q);
    assert.ok(err > 0.0008, `5.60/3.41 is within ${err} of ${p}/${q}: the sum repeats every ${(q * SW_PERIOD / 5.60).toFixed(1)} m`);
  }
});

test('soft saturation is the identity where it should be and never flattens', () => {
  // The curve that replaced clamp(x, 0, 1). Three properties are load bearing:
  // it barely touches a coverage in the working range, it never reaches 1, and
  // — the one the art depends on — its slope is never zero, so a layer that
  // overshoots still carries the mottle that was modulating it.
  assert.equal(swSoft(0.0), 0.0);
  assert.ok(swSoft(0.2) > 0.19 && swSoft(0.2) < 0.2, 'near identity at small coverage');
  let previous = -1;
  for (let x = 0; x <= 4.0; x += 0.01) {
    const v = swSoft(x);
    assert.ok(v > previous, `not monotone at ${x}`);
    assert.ok(v < 1.0, `swSoft(${x}) reached the bound`);
    previous = v;
  }
  for (const x of [1.0, 1.6, 2.5, 4.0]) {
    const slope = (swSoft(x + 1e-4) - swSoft(x - 1e-4)) / 2e-4;
    assert.ok(slope > 0.01, `swSoft has all but flattened by ${x} (slope ${slope})`);
  }
  assert.equal(swSoft(-3.0), 0.0, 'negative coverage is no coverage');
});

// -------------------------------------------------- the treatment, measured
test('the treatment stays in range, stays finite, and stays restrained', () => {
  const next = rng(31337);
  let nonFinite = 0, outOfRange = 0, worst = 0, sumAbs = 0;
  for (let i = 0; i < 200000; i += 1) {
    // Deck-like albedos: luminance 0.08 to 1.0, which is the measured range of
    // every vertex colour in arsenal-models.js.
    const grey = 0.08 + next() * 0.92;
    const base = [grey * (0.7 + 0.6 * next()), grey * (0.7 + 0.6 * next()), grey * (0.7 + 0.6 * next())]
      .map((v) => Math.min(1, v));
    const theta = next() * 2 * Math.PI, z = next() * 2 - 1, r = Math.sqrt(Math.max(0, 1 - z * z));
    const n = [r * Math.cos(theta), z, r * Math.sin(theta)];
    const p = [(next() - 0.5) * 80, (next() - 0.25) * 30, (next() - 0.5) * 80];
    const out = swWeather(base, n, p, Math.pow(10, next() * 3));
    if (out.some((v) => !Number.isFinite(v))) nonFinite += 1;
    if (out.some((v) => v < 0 || v > 1)) outOfRange += 1;
    const delta = Math.abs(lumOf(out) - lumOf(base));
    sumAbs += delta;
    if (delta > worst) worst = delta;
  }
  assert.equal(nonFinite, 0, 'no NaN reaches the framebuffer');
  assert.equal(outOfRange, 0, 'the returned albedo is always a legal albedo');
  // A 1990 army vehicle is maintained, not abandoned. The worst single fragment
  // is the underside of a bright hull inside the splash band; the typical one
  // moves by three hundredths. The bound was 0.34 while dust was boosted on
  // dark material; it is 0.28 now that it is capped there instead.
  assert.ok(worst < 0.28, `worst luminance change ${worst} is past restraint`);
  const meanAbs = sumAbs / 200000;
  assert.ok(meanAbs > 0.020, `mean luminance change ${meanAbs} is too small to see`);
  assert.ok(meanAbs < 0.070, `mean luminance change ${meanAbs} is a repaint, not a weathering pass`);
});

test('no fragment of a real tank moves further than the finish selector moves it', () => {
  // THE BOUND THAT DECIDES WHETHER THIS IS WEATHERING OR A REPAINT, measured on
  // every one of the 141,864 vertices of the inspection tank in all three
  // finishes rather than on random numbers. The yardstick is the loudest
  // legitimate colour change in this UI: the finish selector itself moves a
  // vertex's luminance by 64% (sand) and 122% (winter), and a surface treatment
  // that outshouts the colour control has stopped being a surface treatment.
  //
  // The first draft of this shader reached 274% here, on the near-black rubber
  // of a track — luminance 0.041 to 0.156, which is not a dusty black tyre, it
  // is a grey one. The bound below is the round number under both repaints:
  // nothing may double.
  const excursions = [];
  for (const finish of ['olive', 'sand', 'winter']) {
    for (const v of tankVertices(finish)) {
      const out = swWeather(v.c, v.n, v.p, TANK_CARD_DISTANCE);
      const before = lumOf(v.c);
      if (before < 1e-4) continue;
      excursions.push(Math.abs(lumOf(out) - before) / before);
    }
  }
  excursions.sort((a, b) => a - b);
  const p50 = percentile(excursions, 0.5), p95 = percentile(excursions, 0.95);
  const p99 = percentile(excursions, 0.99), worst = excursions[excursions.length - 1];
  assert.ok(excursions.length > 400000, 'the whole tank really was walked, three times over');
  assert.ok(worst < 1.00,
    `a fragment changed luminance by ${(100 * worst).toFixed(0)}%: it more than doubled, which is louder than the winter repaint`);
  assert.ok(p99 < 0.60, `the 99th percentile fragment moved ${(100 * p99).toFixed(0)}%`);
  assert.ok(p95 < 0.35, `the 95th percentile fragment moved ${(100 * p95).toFixed(0)}%`);
  // And the other end: a treatment nobody can see is not worth 425 ops.
  assert.ok(p50 > 0.02, `the median fragment moved only ${(100 * p50).toFixed(1)}%`);
  assert.ok(p50 < 0.12, `the median fragment moved ${(100 * p50).toFixed(1)}%, which is a re-grade`);
});

test('the final clamp never fires, and no layer pins flat against a bound', () => {
  // TWO STATEMENTS THAT SOUND THE SAME AND ARE NOT.
  //
  // "The output is in range" is satisfied by a clamp. "The clamp never fires"
  // says the value was already legal, which is the difference between a
  // gradient and a flat spot: a clamped channel has lost its variation, and a
  // flat spot on a hull is precisely what this whole chunk exists to remove.
  // The tooth layer used to clip on the winter repaint, where bright trim sits
  // at 1.0 and a multiply can only run off the top.
  //
  // The same argument applies inside: a coverage term that saturates against
  // clamp(x, 0, 1) has had its mottle deleted. On this mesh that was 26% of
  // every dark up-facing surface — the tops of tracks and road wheels — drawn
  // as one uniform maximum-dust field.
  let clampWouldFire = 0, worstOver = 0;
  for (const finish of ['olive', 'sand', 'winter']) {
    for (const v of tankVertices(finish)) {
      swWeather(v.c, v.n, v.p, TANK_CARD_DISTANCE);
      for (const value of PRE_CLAMP) {
        if (value < -1e-12 || value > 1 + 1e-12) {
          clampWouldFire += 1;
          worstOver = Math.max(worstOver, value > 1 ? value - 1 : -value);
        }
      }
    }
  }
  assert.equal(clampWouldFire, 0,
    `the final clamp fires on ${clampWouldFire} channels, worst by ${worstOver}: that is a flat spot, not a safety net`);
  // Coverage, recomputed from the same inputs. swSoft is asymptotic to 1, so
  // "does not pin" is the statement that it stays a comfortable distance below.
  let pinned = 0, worstCoverage = 0;
  for (const v of tankVertices('olive')) {
    const lum = lumOf(v.c);
    const hi = Math.max(...v.c), lo = Math.min(...v.c);
    const sat = (hi - lo) / Math.max(hi, 1e-4);
    const darkness = 1.0 - smoothstep(0.085, 0.175, lum);
    const shiny = smoothstep(0.52, 0.88, lum);
    const mark = smoothstep(0.42, 0.66, sat);
    const splash = 1.0 - smoothstep(-0.20, 0.85, v.p[1]);
    const lowly = 1.0 - smoothstep(0.15, 3.40, v.p[1]);
    const up = v.n[1];
    const upFace = Math.max(up, 0.0) * Math.max(up, 0.0);
    const downFace = Math.max(-up, 0.0);
    const vertFace = (1.0 - Math.abs(up)) * (1.0 - Math.abs(up));
    const b = swBasis(v.p);
    const m1 = swField(swWrap(b.map((x) => x * 5.60)));
    const m2 = swField(swWrap(b.map((x, i) => x * 3.41 + SW_SOURCE_LIFT[i])));
    const streakN = swField(swWrap([v.p[0] * 21.0 + v.p[2] * 8.0, v.p[1] * 3.0, v.p[2] * 21.0 - v.p[0] * 8.0]));
    const blotch = clamp(0.5 + 0.5 * (0.80 * m1 + 0.62 * m2), 0.0, 1.0);
    const dust = upFace * (0.55 + 0.45 * lowly) * (0.35 + 0.65 * blotch)
      * (1.0 - 0.55 * darkness) * (1.0 - 0.55 * mark);
    const streak = smoothstep(0.04, 0.42, streakN) * smoothstep(-0.05, 0.45, m2) * vertFace
      * (0.30 + 0.70 * lowly);
    const grime = (0.90 * splash * (0.25 + 0.75 * (1.0 - upFace)) + 0.60 * downFace * lowly
      + 0.75 * streak) * (0.55 + 0.45 * blotch) * (1.0 - 0.45 * shiny) * (1.0 - 0.50 * mark);
    for (const raw of [dust, grime]) {
      worstCoverage = Math.max(worstCoverage, swSoft(raw));
      if (swSoft(raw) > 0.92) pinned += 1;
    }
  }
  assert.equal(pinned, 0,
    `${pinned} coverage samples sit above 0.92 (worst ${worstCoverage.toFixed(4)}); the mottle there is being squeezed flat`);
});

test('dirt goes down: a hull is grubbier at its foot than at its roof', () => {
  const low = moments(panel(BASES.olive, [1, 0, 0], 0.0, 0.3, 12.0, 8000, 101));
  const high = moments(panel(BASES.olive, [1, 0, 0], 3.0, 6.0, 12.0, 8000, 101));
  assert.ok(low.mean < high.mean * 0.92,
    `splash band only darkens by ${(100 * (1 - low.mean / high.mean)).toFixed(1)}%`);
  assert.ok(low.mean > high.mean * 0.70, 'and it is a weathered vehicle, not a burnt one');
  // The same statement at building scale and at a building's viewing distance:
  // this is the construction-site read, and it is the one thing that has to
  // survive all the way out to a 40-pixel model.
  const site = BASES.concrete;
  const foot = moments(panel(site, [1, 0, 0], 0.0, 0.5, 139.0, 6000, 202));
  const roof = moments(panel(site, [1, 0, 0], 8.0, 11.8, 139.0, 6000, 202));
  assert.ok(foot.mean < roof.mean * 0.93,
    `a site at 139 m is only ${(100 * (1 - foot.mean / roof.mean)).toFixed(1)}% darker at the bottom`);
});

test('the gravity gradient does not fight the host contact shading', () => {
  // The host darkens the bottom 42% of every model by up to 22% on its own, and
  // this shader darkens the same region again. They stack, and stacking two
  // gradients in the same direction is how a hull ends up looking burnt rather
  // than muddy. Transcribed from arsenal3d's FRAG_TEMPLATE: the multiplier is
  // 1 - 0.22 * (1 - smoothstep(0, height * 0.42, P.y)).
  const HEIGHT = 3.87;
  const hostLow = (y) => 1.0 - 0.22 * (1.0 - smoothstep(0.0, HEIGHT * 0.42, y));
  const composite = (hFrom, hTo) => {
    const values = panel(BASES.olive, [1, 0, 0], hFrom, hTo, TANK_CARD_DISTANCE, 6000, 707);
    const next = rng(707);
    // Same height sampling the panel used, so the host term lines up.
    return values.map((v, i) => { next(); const y = hFrom + next() * (hTo - hFrom); next(); return v * hostLow(y); });
  };
  const foot = moments(composite(0.0, 0.3)).mean;
  const head = moments(composite(3.0, 3.8)).mean;
  const hostOnly = hostLow(0.15) / hostLow(3.4);
  const both = foot / head;
  assert.ok(both > 0.55,
    `foot is ${(100 * both).toFixed(0)}% of head once the host's own contact term is included; that is burnt, not muddy`);
  assert.ok(both < hostOnly * 0.97, 'the weathering is not adding anything the host was not already doing');
});

test('dust goes up: sky-facing is lighter than ground-facing, and a flat side is neutral', () => {
  const up = moments(panel(BASES.olive, [0, 1, 0], 1.2, 1.8, 12.0, 8000, 303));
  const down = moments(panel(BASES.olive, [0, -1, 0], 1.2, 1.8, 12.0, 8000, 303));
  const side = moments(panel(BASES.olive, [1, 0, 0], 1.2, 1.8, 12.0, 8000, 303));
  assert.ok(up.mean > down.mean * 1.15, `up-face is only ${(up.mean / down.mean).toFixed(3)}x the under-face`);
  assert.ok(up.mean > side.mean * 1.06, 'up-face collects dust a vertical face does not');
  assert.ok(down.mean < side.mean * 0.97, 'under-face collects grime a vertical face does not');
  // A vertical panel at mid height is the reference surface, and the treatment
  // must not quietly re-grade the whole deck brighter or darker.
  const base = lumOf(BASES.olive);
  assert.ok(Math.abs(side.mean / base - 1) < 0.03,
    `a plain vertical panel shifted by ${(100 * (side.mean / base - 1)).toFixed(1)}%`);
});

test('detail collapses with distance and the gravity gradient does not', () => {
  // One facing, one height band, so the only thing that can move the spread is
  // the noise. At map LOD a flat panel must be its own colour again.
  const spread = (depth) => moments(panel(BASES.olive, [1, 0, 0], 1.0, 2.6, depth, 10000, 404)).sd;
  const close = spread(4.0), card = spread(12.0), far = spread(120.0), map = spread(400.0);
  assert.ok(close > 0.010, `nothing to see at arm's length: sd ${close}`);
  assert.ok(card > 0.008, `nothing to see on a card: sd ${card}`);
  assert.ok(far < close * 0.4, `detail still at ${(100 * far / close).toFixed(0)}% at 120 m`);
  assert.ok(map < 1e-6, `a flat panel at 400 m still carries noise: sd ${map}`);
  // The gradient is what survives, and it must: it is the whole read at 40 px.
  const foot = moments(panel(BASES.olive, [1, 0, 0], 0.0, 0.3, 400.0, 6000, 505));
  const head = moments(panel(BASES.olive, [1, 0, 0], 3.0, 6.0, 400.0, 6000, 505));
  assert.ok(foot.mean < head.mean * 0.92, 'the mud line is gone at map LOD');
});

test('what the distance fade cannot do: a sprite and a card are the same depth', () => {
  // SAID OUT LOUD IN A TEST, because the fades above read as if they solve map
  // LOD and they do not. arsenal3d.sprite(id, 64) and a 200-pixel card both
  // call fitDistance(entry, aspect, pitch, yaw), which depends on the model and
  // the aspect ratio and NOT on the pixel size. So the same tank is 15.5 m away
  // in both, every fade returns the same number, and depth cannot tell a
  // 64-pixel sprite from a 200-pixel card. What the fades actually separate is
  // a tank from a frigate.
  //
  // This test pins that fact so nobody reads the fade constants as a pixel
  // budget. If the host ever passes a real screen-space scale, this is the test
  // that should be deleted, and the fades retuned in the same commit.
  const spread = (depth) => moments(panel(BASES.olive, [1, 0, 0], 1.0, 2.6, depth, 4000, 808)).sd;
  const atCard = spread(TANK_CARD_DISTANCE);
  const atSprite = spread(TANK_CARD_DISTANCE);
  assert.equal(atCard, atSprite, 'depth is the same for both, so the treatment is identical');
  // The frigate case, which the fades DO handle: 133 m of hull framed at 288 m.
  assert.ok(spread(288.0) < atCard * 0.05,
    'a frigate at its own card distance should have almost no grain left');
});

test('the treatment is band limited enough for the smallest pixel it is drawn at', () => {
  // THE ALIASING QUESTION, ANSWERED RATHER THAN ARGUED, because the fades above
  // cannot answer it: depth does not know pixel size, so a 64-pixel map sprite
  // gets the same treatment a 200-pixel card does and something has to say
  // whether that is survivable.
  //
  // Aliasing IS the difference between a point sample and the average over the
  // pixel's footprint, so that is what is measured, on the real mesh, at the
  // real footprints. arsenal3d frames a model to fill the frame at any size, so
  // one pixel of an N-pixel view of an 8.9 m tank covers 8.9/N metres of
  // surface; the treatment is sampled at the fragment and again over an 8x8 grid
  // across that footprint, on the surface's own tangent plane.
  //
  // A software rasteriser of the whole card was written to cross-check this and
  // agrees: at 64 px the treatment adds 0.006 of point-sampling error at the
  // 95th percentile where the geometry itself already contributes 0.170, so the
  // grain is an order of magnitude quieter than the triangles it sits on. It is
  // not in this file because rasterising four images takes half a minute and
  // this measures the same quantity in a tenth of a second.
  const mesh = tank();
  const TANK_SPAN = 8.9;
  const next = rng(4242);
  const triangles = mesh.positions.length / 9;
  const errorAt = (pixels) => {
    const foot = TANK_SPAN / pixels;
    const errors = [], effects = [];
    for (let s = 0; s < 3000; s += 1) {
      const t = Math.floor(next() * triangles) * 9;
      let u = next(), v = next();
      if (u + v > 1) { u = 1 - u; v = 1 - v; }
      const w = 1 - u - v;
      const at = (buf, k) => w * buf[t + k] + u * buf[t + 3 + k] + v * buf[t + 6 + k];
      const P = [0, 1, 2].map((k) => at(mesh.positions, k));
      const raw = [0, 1, 2].map((k) => at(mesh.normals, k));
      const len = Math.hypot(...raw) || 1;
      const N = raw.map((x) => x / len);
      const C = [0, 1, 2].map((k) => at(mesh.colors, k));
      // Any orthonormal frame in the surface: the footprint lies in the plane.
      const helper = Math.abs(N[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0];
      const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
      const t0 = cross(N, helper);
      const tl = Math.hypot(...t0) || 1;
      const T = t0.map((x) => x / tl);
      const B = cross(N, T);
      const point = lumOf(swWeather(C, N, P, TANK_CARD_DISTANCE));
      let acc = 0;
      const M = 8;
      for (let i = 0; i < M; i += 1) {
        for (let j = 0; j < M; j += 1) {
          const du = (i + 0.5) / M - 0.5, dv = (j + 0.5) / M - 0.5;
          const q = [0, 1, 2].map((k) => P[k] + T[k] * du * foot + B[k] * dv * foot);
          acc += lumOf(swWeather(C, N, q, TANK_CARD_DISTANCE));
        }
      }
      errors.push(Math.abs(point - acc / (M * M)));
      // The effect itself, on the SAME fragment, so signal and sampling error
      // are the same statistic of the same population and can be compared.
      effects.push(Math.abs(point - lumOf(C)));
    }
    errors.sort((a, b) => a - b);
    effects.sort((a, b) => a - b);
    return {
      p50: percentile(errors, 0.5), p95: percentile(errors, 0.95), max: errors[errors.length - 1],
      effectP95: percentile(effects, 0.95),
    };
  };
  const sprite = errorAt(64);      // 13.9 cm a pixel: the map sprite, the worst case
  const card = errorAt(200);       // 4.5 cm a pixel: the catalogue card
  const retina = errorAt(400);     // 2.2 cm a pixel: the same card at dpr 2
  assert.ok(sprite.p95 < 0.030, `a 64 px sprite point-samples ${sprite.p95.toFixed(4)} away from its own pixel average`);
  assert.ok(card.p95 < 0.010, `a 200 px card point-samples ${card.p95.toFixed(4)} away from its own pixel average`);
  // BAND LIMITED, not white: the sampling error has to FALL as the footprint
  // shrinks. A field with a discontinuity in it — which is what this shader had
  // before the fold was fixed — carries energy at every frequency, and its error
  // falls far more slowly than the footprint does.
  assert.ok(card.p95 < sprite.p95 * 0.55, 'the error does not fall with the footprint: the field is not band limited');
  assert.ok(retina.p95 < card.p95 * 0.55, 'the error does not keep falling: there is broadband energy in the field');
  // And the effect must be much louder than its own sampling error, or it is
  // being drawn as noise rather than as surface. Same fragments, same statistic:
  // the 95th percentile of what the treatment DOES against the 95th percentile
  // of how badly one sample of it misrepresents its own pixel.
  for (const [what, m] of [['64 px sprite', sprite], ['200 px card', card], ['400 px card', retina]]) {
    assert.ok(m.effectP95 > m.p95 * 4,
      `at ${what} the treatment moves ${m.effectP95.toFixed(4)} and mis-samples by ${m.p95.toFixed(4)}: it is being drawn as noise`);
  }
});

test('it reads as surface on any base hue, including both repaints', () => {
  // The finish pass rewrites vertex colours before upload wherever
  // g > r*1.025 && g > b*1.06 && g > 0.12, so olive becomes sand or winter and
  // an effect that assumed olive would vanish. Every base here has to end up
  // with real variation, and the three paints have to end up with the SAME
  // amount of it.
  const relative = {};
  for (const [name, colour] of Object.entries(BASES)) {
    const stats = moments(panel(colour, null, 0.4, 2.2, 12.0, 8000, 606));
    relative[name] = stats.sd / stats.mean;
    assert.ok(stats.sd > 0.004, `${name}: sd ${stats.sd} is a flat field again`);
    assert.ok(relative[name] > 0.03, `${name}: only ${(100 * relative[name]).toFixed(1)}% relative variation`);
  }
  const paints = [relative.olive, relative.sandRepaint, relative.winterRepaint];
  const ratio = Math.max(...paints) / Math.min(...paints);
  assert.ok(ratio < 1.5, `olive, sand and winter weather by different amounts (spread ${ratio.toFixed(2)}x)`);
  // The two ends of the material guess. Rubber still varies MORE than steel in
  // relative terms even though dust is now capped on it rather than boosted —
  // mixing toward a fixed light tan is inherently a bigger relative change on a
  // dark base, which is exactly why the boost was double-counting and had to go.
  assert.ok(relative.rubber > relative.steel, 'dust reads hardest on the darkest surfaces');
  assert.ok(relative.lamp < relative.concrete, 'bright lenses and markings are protected');
});

test('the same fragment is the same colour every time it is asked', () => {
  // The card cache, the sprite cache and the goldens in every other suite all
  // rest on this. It is one line and it is the first rule in the project.
  const next = rng(909090);
  for (let i = 0; i < 2000; i += 1) {
    const base = [next(), next(), next()];
    const n = [next() * 2 - 1, next() * 2 - 1, next() * 2 - 1];
    const len = Math.hypot(...n) || 1;
    const unit = n.map((v) => v / len);
    const p = [(next() - 0.5) * 60, (next() - 0.3) * 20, (next() - 0.5) * 60];
    const depth = 1 + next() * 300;
    assert.deepEqual(swWeather(base, unit, p, depth), swWeather(base, unit, p, depth));
  }
  // And the same numbers it produced when the amplitudes were chosen.
  const goldens = [
    [[0.198, 0.232, 0.176], [0, 1, 0], [0.4, 1.6, -1.2], 15.0, [0.2133972898, 0.2364135929, 0.1816422969]],
    [[0.198, 0.232, 0.176], [1, 0, 0], [0.9, 0.15, 2.4], 15.0, [0.1824675274, 0.2074516896, 0.1569521838]],
    [[0.086, 0.092, 0.101], [0, -1, 0], [-1.1, 0.35, 0.8], 9.0, [0.0848781189, 0.0847349455, 0.0871117260]],
    [[0.44, 0.43, 0.41], [0.5774, 0.5774, 0.5774], [3.0, 6.5, -8.0], 139.0, [0.4465414381, 0.4354151735, 0.4134795946]],
    [[0.95, 0.95, 0.92], [0, 0, 1], [0.2, 1.1, 0.05], 4.0, [0.9477988321, 0.9477988321, 0.9178683426]],
  ];
  for (const [base, n, p, depth, want] of goldens) {
    const got = swWeather(base, n, p, depth);
    for (let i = 0; i < 3; i += 1) {
      assert.ok(Math.abs(got[i] - want[i]) < 1e-9, `swWeather${JSON.stringify([base, n, p, depth])}[${i}]: ${got[i]} != ${want[i]}`);
    }
  }
});

test('the degenerate inputs a real frame will hand it', () => {
  // A black albedo, a white one, a fragment exactly on the ground plane, a
  // fragment below it (every hull in the deck has one), a zero depth from an
  // orthographic host, and an enormous depth.
  const cases = [
    [[0, 0, 0], [0, 1, 0], [0, 0, 0], 10.0],
    [[1, 1, 1], [0, -1, 0], [0, 0, 0], 10.0],
    [[0.2, 0.2, 0.2], [0, 1, 0], [0, -7.0, 0], 1.0],
    [[0.2, 0.2, 0.2], [1, 0, 0], [0, 0, 0], 0.0],
    [[0.2, 0.2, 0.2], [1, 0, 0], [0, 0, 0], 1e9],
    [[0.2, 0.2, 0.2], [0, 0, 1], [1e5, 1e5, 1e5], 50.0],
  ];
  for (const [base, n, p, depth] of cases) {
    const out = swWeather(base, n, p, depth);
    assert.ok(out.every(Number.isFinite), `non-finite at ${JSON.stringify([base, n, p, depth])}`);
    assert.ok(out.every((v) => v >= 0 && v <= 1), `out of range at ${JSON.stringify([base, n, p, depth])}`);
  }
  // Black stays black rather than becoming grey — this is the bound that the
  // first draft failed, at 0.156 against a base of 0.041 — and white stays
  // white. Both are tightened from where they were, because capping dust on
  // dark material is what made the tighter numbers available.
  assert.ok(lumOf(swWeather([0, 0, 0], [0, 1, 0], [0, 0.1, 0], 10.0)) < 0.07);
  assert.ok(lumOf(swWeather([1, 1, 1], [0, 1, 0], [0, 4.0, 0], 10.0)) > 0.93);
});
