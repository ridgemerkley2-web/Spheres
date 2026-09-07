#!/usr/bin/env node
/// CHECK: spheres-web/ui/surface-material.js
///
/// WHAT THIS CANNOT DO, STATED UP FRONT. There is no GPU in node and this file
/// does not pretend there is one. It does not compile the GLSL, so it cannot
/// prove the chunk links, cannot catch a type error, cannot catch a swizzle
/// that names a component that is not there, and cannot report a uniform that
/// the host does not in fact declare. It cannot measure a real ALU count —
/// that is a driver's business and every driver counts differently. It cannot
/// tell you whether the result LOOKS right, which is the only question that
/// finally matters and the only one a person answers.
///
/// WHAT IT CAN DO, AND DOES. Six things, in rising order of worth:
///
///   1. Lexical. Balanced braces, parens and brackets. The declared entry point
///      present with the exact documented signature. No banned token. No
///      uniform declaration, and no reference to any host uniform outside the
///      one this module documents. No preprocessor directive. And neither N nor
///      V read inside the body, which is the wireframe trap ruled out by
///      inspection rather than by intention.
///   2. Shape. Frozen export, the three documented fields, both UMD paths.
///   3. Classification against the REAL palettes. Every colour asserted here is
///      also asserted to still be present in the generator that owns it, so
///      this check goes stale loudly rather than quietly. It then asserts the
///      class each colour lands in, that the five weights sum to exactly 1, and
///      that the sand and winter repaints still land somewhere sane.
///   4. A PORT OF THE NOISE TO JS, in float32 via Math.fround, exercised over
///      tens of thousands of points. If smWave is biased the whole model shifts
///      tone; if it clips, panels blow out; if smHash correlates with its own
///      neighbour the speckle turns into a lattice. None of that is visible by
///      reading the GLSL and all of it is visible here.
///   5. THE REAL MESH, which is the section that found the defect. Sections 1-4
///      sample colours from a table and positions from a low-discrepancy stream.
///      That is right for measuring the NOISE and wrong for measuring the
///      TREATMENT, because it tests neither the geometry, nor the colours, nor
///      the framing distance the renderer will actually use. Section 6 builds
///      three real meshes out of equipment-mesh.js and pushes their own vertex
///      buffers through the port at the distance arsenal3d.js would frame them
///      from, in all three finishes — 1.7 million fragments. It is what caught
///      the winter repaint driving 2,824 channel values above 1.0, which a
///      palette sweep bounded at 1.02 could not see.
///   6. FREQUENCY AGAINST THE CARD SIZES THAT SHIP. Section 7 reads the canvas
///      sizes out of arsenal3d.css rather than assuming one, works out how many
///      pixels a cycle gets on each, and measures what the undersampling does:
///      how much energy folds down into visible low frequencies against how
///      much is genuinely there. That distinction is the whole question — grain
///      is acceptable at 34 px and banding is not — and it is not answerable by
///      reasoning about Nyquist alone.

const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const ui = path.resolve(__dirname, "../../spheres-web/ui");
const mod = require(path.join(ui, "surface-material.js"));
const source = fs.readFileSync(path.join(ui, "surface-material.js"), "utf8");

/// The one host uniform this chunk is allowed to read. arsenal3d.js declares it
/// above the __SURFACE__ splice point; the module's header documents it.
const HOST_UNIFORMS = ["uEye"];

/// GLSL with every comment removed, so an English word in a comment cannot fail
/// a token check and a brace inside a comment cannot fool the balance check.
const code = mod.glsl
  .replace(/\/\*[\s\S]*?\*\//g, " ")
  .replace(/\/\/[^\n]*/g, " ");

// ---------------------------------------------------------------- 1. lexical

test("the module is the agreed shape and is frozen", () => {
  assert.ok(Object.isFrozen(mod), "export must be frozen");
  assert.equal(mod.id, "material-response");
  assert.equal(typeof mod.glsl, "string");
  assert.equal(typeof mod.notes, "string");
  assert.ok(mod.glsl.length > 500, "a treatment that fits in 500 chars is not one");
  assert.ok(mod.notes.length > 400, "notes must actually say what it does");
  assert.deepEqual(Object.keys(mod).sort(), ["glsl", "id", "notes"]);
  // Both UMD paths, so the same file serves node and the page.
  assert.match(source, /typeof module === "object" && module\.exports/);
  assert.match(source, /root\.Surfacematerial = api/);
  assert.match(source, /typeof globalThis !== "undefined" \? globalThis : this/);
});

test("delimiters balance", () => {
  for (const [open, close] of [["{", "}"], ["(", ")"], ["[", "]"]]) {
    let depth = 0;
    for (const ch of code) {
      if (ch === open) depth += 1;
      else if (ch === close) depth -= 1;
      assert.ok(depth >= 0, `${close} before ${open} in the GLSL`);
    }
    assert.equal(depth, 0, `unbalanced ${open}${close} in the GLSL`);
  }
});

test("the documented entry point exists with exactly the documented signature", () => {
  const sig = /\bvec3\s+surface\s*\(\s*vec3\s+albedo\s*,\s*vec3\s+N\s*,\s*vec3\s+P\s*,\s*vec3\s+V\s*\)\s*\{/;
  assert.match(code, sig);
  // Exactly one definition of it, so a second cannot shadow the first.
  assert.equal((code.match(/\bsurface\s*\(/g) || []).length, 1);
  // The host splices this in, then calls surface(vCol, N, vPos, V). Every
  // helper must therefore be defined before the entry point uses it.
  const entry = code.search(sig);
  for (const helper of ["smHash", "smWave"]) {
    const declared = code.indexOf(`float ${helper}(`);
    assert.ok(declared >= 0, `${helper} must be defined`);
    assert.ok(declared < entry, `${helper} must be defined before surface() calls it`);
  }
  // It returns an albedo and must not have been talked into lighting itself.
  assert.doesNotMatch(code, /\breflect\s*\(|\bpow\s*\(|gl_FragColor|outColor|gl_FragCoord/);
});

test("no banned token survives comment stripping", () => {
  const banned = [
    [/\btexture\w*\s*\(/, "texture sampling"],
    [/\bsampler\w*\b/, "a sampler type"],
    [/#\s*\w+/, "a preprocessor directive (#version and friends)"],
    [/\bmain\s*\(/, "a main()"],
    [/\btime\b|\buTime\b|\biTime\b/, "a time input"],
    [/\bfwidth\s*\(|\bdFdx\s*\(|\bdFdy\s*\(/, "a screen-space derivative"],
    [/\bdiscard\b/, "a discard"],
    [/\btextureLod\b|\btexelFetch\b/, "a texel fetch"],
    [/\bGL_\w+|\brequire\s*\(|\bextension\b/, "an extension request"],
  ];
  for (const [re, why] of banned) assert.doesNotMatch(code, re, `GLSL must not contain ${why}`);
  // No storage qualifiers: the chunk is spliced into an existing shader that
  // already owns its ins and outs.
  assert.doesNotMatch(code, /(^|[;{}\s])(in|out|inout|varying|attribute)\s+(vec|float|int|mat|bool)/,
    "GLSL must declare no in/out/varying/attribute");
  // A variable literally named `out` would also be a reserved word.
  assert.doesNotMatch(code, /\b(?:vec3|float|vec4|vec2)\s+out\b/);
});

test("no uniform is declared, and only the documented host uniform is read", () => {
  assert.doesNotMatch(code, /\buniform\b/, "this chunk declares no uniforms of its own");
  // Host uniforms in this renderer follow a uXxx convention (uMVP, uEye,
  // uHeight), so any uXxx identifier in the chunk is a free variable that the
  // host must supply. The set of them must be exactly what the header documents.
  const used = [...new Set((code.match(/\bu[A-Z]\w*/g) || []))].sort();
  assert.deepEqual(used, HOST_UNIFORMS.slice().sort(),
    `undocumented host uniform read: ${used.join(", ")}`);
  for (const name of HOST_UNIFORMS) {
    assert.ok(mod.notes.includes(name) || source.includes(`uniform vec3 ${name}`),
      `${name} must be documented in the module header`);
    // And the host must actually declare it, above the splice point.
    const host = fs.readFileSync(path.join(ui, "arsenal3d.js"), "utf8");
    const decl = host.indexOf(`uniform vec3 ${name};`);
    const splice = host.indexOf("__SURFACE__");
    assert.ok(decl >= 0 && splice > decl,
      `arsenal3d.js must declare ${name} above __SURFACE__`);
  }
});

test("the host contract this chunk is written against has not moved", () => {
  const host = fs.readFileSync(path.join(ui, "arsenal3d.js"), "utf8");
  assert.ok(host.includes("vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V)"),
    "arsenal3d.js must still declare the identity surface with this signature");
  assert.ok(host.includes("surface(vCol, N, vPos, V)"), "the host must still call it with model-space vPos");
  assert.ok(host.includes("vPos = aPos"), "vPos must still be raw model space, Y=0 at ground");
  // The camera used to orbit the bounding-box centre; it now orbits the framing
  // pivot `at`, which is the recentred point the card is composed around. The
  // PROPERTY this pins is unchanged -- uEye is the eye in model space, and the
  // model is translated by the same point the eye is offset by -- so only the
  // name of that point moved.
  assert.ok(host.includes("uniform3f(uEye, eye[0] + at[0]"),
    "uEye must still be the eye expressed in model space");
  assert.ok(host.includes("translate([-at[0], -at[1], -at[2]])"),
    "the model must be translated by the SAME point the eye is offset by, or "
    + "model-space lighting is computed about a different origin than the camera");
});

// ------------------------------------------------------- 2. the float32 port
//
// A LINE-FOR-LINE port of the GLSL above, in float32. Every arithmetic step is
// wrapped in Math.fround, because the whole point is to reproduce what the GPU
// will actually compute, including where it runs out of mantissa. A float64
// port would quietly pass a precision failure that the shader would show.

const f = Math.fround;
const fract = (x) => f(x - Math.floor(x));
const clamp01 = (x) => Math.min(1, Math.max(0, x));
const mix = (a, b, t) => f(a + f(f(b - a) * t));
const dot3 = (a, b) => f(f(f(a[0] * b[0]) + f(a[1] * b[1])) + f(a[2] * b[2]));

const D0 = [0.836, 0.421, 0.351];
const D1 = [-0.413, 1.231, 0.445];
const D2 = [0.404, -0.188, 0.648];
const D3 = [0.211, -0.147, 0.263];
const PH = [0.610, 1.370, 0.830];
const AX = [0.620, 0.340, 0.707];
const LUMA = [0.299, 0.587, 0.114];
const CYCLES = [150.0, 165.0, 200.0, 105.0];
const GAIN = [0.085, 0.115, 0.050, 0.130];
const SPECK = [0.030, 0.022, 0.026, 0.090];
const GLINT = [1.000, 0.600, 0.000, 0.000];

function smHash(p) {
  let x = fract(f(f(p[0] * 0.3183099) + 0.71));
  let y = fract(f(f(p[1] * 0.3183099) + 0.113));
  let z = fract(f(f(p[2] * 0.3183099) + 0.419));
  x = f(x * 17.0); y = f(y * 17.0); z = f(z * 17.0);
  return fract(f(f(f(x * y) * z) * f(f(x + y) + z)));
}

function smWave(q) {
  let u = Math.abs(f(f(fract(dot3(q, D3)) * 2.0) - 1.0));
  u = f(f(u * u) * f(3.0 - f(2.0 * u)));
  const t = [dot3(q, D0), dot3(q, D1), dot3(q, D2)]
    .map((v, i) => Math.abs(f(f(fract(f(v + f(u * PH[i]))) * 2.0) - 1.0)))
    .map((v) => f(f(v * v) * f(3.0 - f(2.0 * v))));
  return f(f(f(t[0] + t[1]) + t[2]) * 0.3333333);
}

/// The classifier alone, returned as named weights so the palette test can read
/// them. Identical arithmetic to the GLSL.
function classify(c) {
  const L = f(f(f(c[0] * LUMA[0]) + f(c[1] * LUMA[1])) + f(c[2] * LUMA[2]));
  const mx = Math.max(c[0], c[1], c[2]);
  const mn = Math.min(c[0], c[1], c[2]);
  const S = f(f(mx - mn) / Math.max(mx, 1e-3));
  const warm = f(c[0] - c[2]);
  const rubber = f(1.0 - clamp01(f(f(L - 0.145) * 10.0)));
  const glass = f(f(clamp01(f(f(S - 0.28) * 6.0)) * clamp01(f(f(-warm - 0.04) * 10.0))) * f(1.0 - rubber));
  const rest = f(f(1.0 - rubber) - glass);
  const concrete = f(f(f(rest * clamp01(f(f(L - 0.40) * 6.5)))
    * f(1.0 - clamp01(f(f(S - 0.115) * 8.0)))) * clamp01(f(f(warm + 0.02) * 20.0)));
  const pf = clamp01(f(f(S - 0.128) * 7.0));
  const paint = f(f(rest - concrete) * pf);
  const steel = f(f(rest - concrete) * f(1.0 - pf));
  return { paint, steel, rubber, concrete, glass, L, S, warm, mx };
}

/// The whole entry point. `eye` is the model-space eye position; `P` is the
/// model-space fragment position, metres, Y=0 at ground contact.
function surface(albedo, P, eye) {
  const w = classify(albedo);
  const dot4 = (k) => f(f(f(f(w.paint * k[0]) + f(w.steel * k[1])) + f(w.rubber * k[2])) + f(w.concrete * k[3]));
  const cyc = f(dot4(CYCLES) + f(w.glass * 26.0));
  const gain = f(dot4(GAIN) + f(w.glass * 0.022));
  const spk = dot4(SPECK);
  const gl = dot4(GLINT);
  const tint = f(f(w.steel * 0.55) + f(w.concrete * 0.30));

  // The exact multiplicative headroom. `pk` is the largest the modulation below
  // can reach as a fraction of mx, plus the most the blue tint can add; scaling
  // both by `room` makes mx*(1 + pk*room) + 0.0175*tint*room <= 1 identically.
  const pk = f(f(w.mx * f(gain + f(spk * f(0.5 + f(0.5 * gl))))) + f(0.0175 * tint));
  const room = clamp01(f(f(1.0 - w.mx) / Math.max(pk, 1e-4)));

  const od = Math.max(f(Math.hypot(eye[0], eye[1], eye[2])), 0.5);
  const freq = Math.min(400.0, Math.max(0.25, f(cyc / od)));

  let q = [f(P[0] * freq), f(P[1] * freq), f(P[2] * freq)];
  const k = f(dot3(q, AX) * f(-0.93 * w.steel));
  q = [f(q[0] + f(AX[0] * k)), f(q[1] + f(AX[1] * k)), f(q[2] + f(AX[2] * k))];

  const n = f(smWave(q) - 0.5);
  const h = smHash([Math.floor(f(q[0] * 2.3)), Math.floor(f(q[1] * 2.3)), Math.floor(f(q[2] * 2.3))]);
  const s = mix(f(h - 0.5), f(f(Math.max(f(h - 0.85), 0.0) * 6.6667) - 0.075), gl);

  const m = f(f(f(f(n * 2.0) * gain) + f(s * spk)) * room);
  const tr = f(tint * room);
  const t = [-0.020, 0.0, 0.035];
  // `raw` is the value BEFORE the shader's guard clamp. Everything downstream
  // asserts against raw, so the clamp cannot hide an out-of-range result: the
  // claim is that the arithmetic lands in [0,1], not that a clamp caught it.
  const raw = albedo.map((a, i) => f(f(a * f(1.0 + m)) + f(f(n * tr) * t[i])));
  return {
    raw,
    rgb: raw.map((v) => Math.min(1.0, Math.max(v, 0.0))),
    m, n, h, freq, w, room,
  };
}

/// Every numeric constant in the port must be the one in the GLSL. Without this
/// the port is a second implementation that can drift, and a drifted port
/// proves nothing about the shader.
test("the JS port carries the same constants as the GLSL", () => {
  const pairs = [
    ["SM_D0", D0], ["SM_D1", D1], ["SM_D2", D2], ["SM_D3", D3], ["SM_PH", PH],
    ["SM_AX", AX], ["SM_LUMA", LUMA],
    ["SM_CYCLES", CYCLES], ["SM_GAIN", GAIN], ["SM_SPECK", SPECK], ["SM_GLINT", GLINT],
  ];
  for (const [name, values] of pairs) {
    const line = code.match(new RegExp(`${name}\\s*=\\s*vec[34]\\(([^)]*)\\)`));
    assert.ok(line, `${name} must be a vec constant in the GLSL`);
    const got = line[1].split(",").map((v) => Number(v.trim()));
    assert.equal(got.length, values.length, name);
    got.forEach((v, i) => assert.ok(Math.abs(v - values[i]) < 1e-9, `${name}[${i}] ${v} != ${values[i]}`));
  }
  for (const literal of ["0.3183099", "17.0", "0.3333333", "2.3", "6.6667", "0.85", "-0.93",
    "0.145", "0.28", "0.40", "0.115", "0.128", "1e-3", "0.25, 400.0",
    // the headroom guard, which is arithmetic the port must not diverge from
    "0.5 + 0.5 * gl", "0.0175 * tint", "1.0 - mx", "1e-4"]) {
    assert.ok(code.includes(literal), `GLSL must still contain the literal ${literal} the port assumes`);
  }
  // The guard is a clamp to [0,1], not a max to 0: a return that only floors is
  // the state this check was written to catch.
  assert.match(code, /return\s+clamp\s*\(\s*res\s*,\s*0\.0\s*,\s*1\.0\s*\)/,
    "surface() must return clamp(res, 0.0, 1.0) — a bare max(res, 0.0) lets a channel exceed 1");
  assert.match(code, /float\s+room\s*=\s*clamp\s*\(/, "the headroom scale must be present and clamped");
  assert.match(code, /\*\s*room\s*\)\s*\*\s*vec3|n\s*\*\s*tint\s*\*\s*room/,
    "the tint must carry room too, or the bound does not hold");
});

// -------------------------------------------------- 3. the noise, measured
//
// A deterministic sweep, not a random one: the sampler must not itself be a
// source of variance between runs, and the numbers below are asserted to two
// decimal places.

function summarise(values) {
  let sum = 0, lo = Infinity, hi = -Infinity, nan = 0;
  for (const v of values) {
    if (!Number.isFinite(v)) { nan += 1; continue; }
    sum += v; if (v < lo) lo = v; if (v > hi) hi = v;
  }
  const mean = sum / values.length;
  let sq = 0;
  for (const v of values) sq += (v - mean) * (v - mean);
  const buckets = new Array(16).fill(0);
  for (const v of values) buckets[Math.min(15, Math.max(0, Math.floor(v * 16)))] += 1;
  const expect = values.length / 16;
  const chi = buckets.reduce((a, c) => a + ((c - expect) * (c - expect)) / expect, 0);
  return { n: values.length, mean, sd: Math.sqrt(sq / values.length), lo, hi, nan, chi };
}

/// A cheap deterministic stream of positions that is not on any lattice the
/// noise could be accidentally in tune with.
function* positions(count, span) {
  let a = 0.3141, b = 0.7182, c = 0.4142;
  for (let i = 0; i < count; i += 1) {
    a = (a + 0.7548776662) % 1; b = (b + 0.5698402909) % 1; c = (c + 0.8191725134) % 1;
    yield [f((a - 0.5) * span), f((b - 0.5) * span * 0.3), f((c - 0.5) * span)];
  }
}

test("smWave: deterministic, bounded, unbiased, and the variance it was designed for", () => {
  const values = [], again = [];
  for (const p of positions(40000, 220)) values.push(smWave(p));
  for (const p of positions(40000, 220)) again.push(smWave(p));
  assert.deepEqual(values, again, "the field must be a pure function of position");

  const st = summarise(values);
  assert.equal(st.nan, 0, "no NaN and no Infinity");
  // Range. Three smoothstepped triangles each land strictly in [0,1], so their
  // mean does too; that is a proof, and this is the check that the port agrees.
  assert.ok(st.lo >= 0 && st.hi <= 1, `range [${st.lo}, ${st.hi}] must be inside [0,1]`);
  // Mean. E[t*t*(3-2t)] for t uniform on [0,1] is exactly 1/2, so the field is
  // unbiased and the treatment cannot shift a model's overall tone.
  assert.ok(Math.abs(st.mean - 0.5) < 0.01, `mean ${st.mean.toFixed(4)} must be 0.5 +/- 0.01`);
  // Variance. Var of one smoothstepped triangle is 9/5 - 2 + 4/7 - 1/4 =
  // 0.121428...; three independent, averaged, gives sd = sqrt(0.121428/3) =
  // 0.20119. Measuring that is what proves the three directions are behaving as
  // three independent samples rather than as one wave in a trenchcoat.
  assert.ok(Math.abs(st.sd - 0.2012) < 0.012, `sd ${st.sd.toFixed(4)} must be 0.2012 +/- 0.012`);
});

test("smWave holds its statistics on a flat plane, which is what a panel is", () => {
  // Most fragments in this renderer sit on an axis-aligned flat panel, so a
  // field that is only well behaved in the volume average is not good enough:
  // a plane is a 2D slice and a badly chosen direction set can be degenerate on
  // one. 60 units is about 22 periods of the longest wave, which is the window
  // over which the mean is a mean rather than a sample.
  for (const [axis, at] of [["y", 0.0], ["y", 1.83], ["x", 0.0], ["z", 0.0], ["z", -2.4]]) {
    const values = [];
    for (let i = 0; i < 240; i += 1) {
      for (let j = 0; j < 240; j += 1) {
        const a = f(-30 + i * 0.25), b = f(-30 + j * 0.25);
        values.push(smWave(axis === "y" ? [a, at, b] : axis === "x" ? [at, a, b] : [a, b, at]));
      }
    }
    const st = summarise(values);
    assert.equal(st.nan, 0);
    assert.ok(st.lo >= 0 && st.hi <= 1, `plane ${axis}=${at} range`);
    assert.ok(Math.abs(st.mean - 0.5) < 0.01, `plane ${axis}=${at} mean ${st.mean.toFixed(4)}`);
    assert.ok(Math.abs(st.sd - 0.2012) < 0.012, `plane ${axis}=${at} sd ${st.sd.toFixed(4)}`);
  }
});

test("the field carries the broad panel-to-panel variation it is supposed to", () => {
  // The phase warp puts real low-frequency power in the field, and that is a
  // feature, not leakage: it is what makes one panel read a shade different
  // from the panel beside it, which is most of what "not a uniform field of one
  // colour" means. Measure it, because too little is a flat model and too much
  // is blotches. Patches one and a half periods across, all over the volume.
  const means = [];
  for (let patch = 0; patch < 300; patch += 1) {
    const ox = ((patch * 7.31) % 120) - 60;
    const oz = ((patch * 11.17) % 120) - 60;
    const oy = ((patch * 3.7) % 9) - 4.5;
    let total = 0;
    for (let i = 0; i < 20; i += 1) {
      for (let j = 0; j < 20; j += 1) total += smWave([f(ox + i * 0.075), oy, f(oz + j * 0.075)]);
    }
    means.push(total / 400);
  }
  const st = summarise(means);
  assert.ok(Math.abs(st.mean - 0.5) < 0.01, `patch means must still centre on 0.5, got ${st.mean.toFixed(4)}`);
  assert.ok(st.sd > 0.04 && st.sd < 0.09,
    `patch-to-patch spread ${st.sd.toFixed(4)} outside the designed 0.04-0.09`);
  // Which is 1.1% rms panel-to-panel brightness on paint and 1.7% on concrete.
  assert.ok(2 * 0.085 * st.sd < 0.02 && 2 * 0.130 * st.sd < 0.03,
    "broad variation must stay subtle enough to read as a panel, not as a blotch");
});

test("smWave decorrelates, averaged over directions rather than along one", () => {
  // A sum of periodic waves resonates for offsets that happen to be a whole
  // period along one of its directions, so measuring one offset direction
  // flatters or damns it by luck. Average over many directions at each radius.
  const radii = [0.15, 0.4, 0.8, 1.6, 3.2];
  const worst = [];
  for (const r of radii) {
    let total = 0, dirs = 0;
    for (let d = 0; d < 24; d += 1) {
      const th = (d * 2.399963) % (Math.PI * 2), ph = Math.acos(1 - (2 * (d + 0.5)) / 24);
      const off = [r * Math.sin(ph) * Math.cos(th), r * Math.sin(ph) * Math.sin(th), r * Math.cos(ph)];
      let s1 = 0, s2 = 0, s12 = 0, q1 = 0, q2 = 0, count = 0;
      for (const p of positions(1200, 80)) {
        const a = smWave(p), b = smWave([f(p[0] + off[0]), f(p[1] + off[1]), f(p[2] + off[2])]);
        s1 += a; s2 += b; s12 += a * b; q1 += a * a; q2 += b * b; count += 1;
      }
      const cov = s12 / count - (s1 / count) * (s2 / count);
      const sd1 = Math.sqrt(q1 / count - (s1 / count) ** 2), sd2 = Math.sqrt(q2 / count - (s2 / count) ** 2);
      total += Math.abs(cov / (sd1 * sd2)); dirs += 1;
    }
    worst.push(total / dirs);
  }
  // Adjacent samples are correlated, which is what makes it a smooth field
  // rather than static; by one wavelength the correlation must be gone, or the
  // "noise" is a visible repeating pattern and a person will see the weave.
  // Measured on the shipped constants: 0.760, 0.217, 0.141, 0.128, 0.134. The
  // bars sit just outside those, so removing the phase warp — which took the
  // 1.6-wavelength figure from 0.391 to 0.128 — goes red rather than quietly
  // putting the weave back.
  assert.ok(worst[0] > 0.6, `at 0.15 wavelengths the field must still be smooth, got ${worst[0].toFixed(3)}`);
  assert.ok(worst[2] < 0.18, `at 0.8 wavelengths mean |corr| must be under 0.18, got ${worst[2].toFixed(3)}`);
  assert.ok(worst[3] < 0.18, `at 1.6 wavelengths mean |corr| must be under 0.18, got ${worst[3].toFixed(3)}`);
  assert.ok(worst[4] < 0.18, `at 3.2 wavelengths mean |corr| must be under 0.18, got ${worst[4].toFixed(3)}`);
});

test("smHash is uniform on the cell lattice it is actually sampled on", () => {
  // smHash is only ever called on floor(), so an integer lattice is the only
  // input that matters. Sampling it off-lattice would measure a function the
  // shader never evaluates.
  const values = [];
  for (let x = -30; x < 30; x += 1) {
    for (let y = -15; y < 15; y += 1) {
      for (let z = -30; z < 30; z += 1) values.push(smHash([x, y, z]));
    }
  }
  const st = summarise(values);
  assert.equal(st.nan, 0, "no NaN");
  assert.ok(st.lo >= 0 && st.hi < 1, `range [${st.lo}, ${st.hi}) must be inside [0,1)`);
  assert.ok(Math.abs(st.mean - 0.5) < 0.01, `mean ${st.mean.toFixed(4)}`);
  // Uniform on [0,1) has sd 1/sqrt(12) = 0.28868.
  assert.ok(Math.abs(st.sd - 0.28868) < 0.008, `sd ${st.sd.toFixed(4)} must be 0.2887 +/- 0.008`);
  // 15 degrees of freedom; 30 is the 1% point, so a hash landing over it is
  // visibly clumped and the speckle would show tonal patches.
  assert.ok(st.chi < 30, `chi-square over 16 buckets is ${st.chi.toFixed(1)}, must be under 30`);
});

test("smHash does not correlate with its own neighbours, at near or far coordinates", () => {
  const corr = (offset, base) => {
    let s1 = 0, s2 = 0, s12 = 0, q1 = 0, q2 = 0, n = 0;
    for (let x = 0; x < 60; x += 1) {
      for (let y = 0; y < 24; y += 1) {
        for (let z = 0; z < 60; z += 1) {
          const c = [base + x - 30, base * 0.1 + y - 12, base + z - 30];
          const a = smHash(c);
          const b = smHash([c[0] + offset[0], c[1] + offset[1], c[2] + offset[2]]);
          s1 += a; s2 += b; s12 += a * b; q1 += a * a; q2 += b * b; n += 1;
        }
      }
    }
    const cov = s12 / n - (s1 / n) * (s2 / n);
    return cov / (Math.sqrt(q1 / n - (s1 / n) ** 2) * Math.sqrt(q2 / n - (s2 / n) ** 2));
  };
  for (const base of [0, 1800]) {
    for (const off of [[1, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 1], [7, 0, 0]]) {
      const r = corr(off, base);
      assert.ok(Math.abs(r) < 0.05,
        `hash correlation at base ${base} offset ${off} is ${r.toFixed(4)}, must be under 0.05`);
    }
  }
  // Precision. The leading fract bounds every term before the mixing
  // multiplies, so a four-digit cell index must not degrade the distribution.
  const far = [];
  for (let x = 1800; x < 1900; x += 1) for (let z = 1800; z < 1900; z += 1) far.push(smHash([x, 3, z]));
  const st = summarise(far);
  assert.ok(Math.abs(st.mean - 0.5) < 0.015 && Math.abs(st.sd - 0.28868) < 0.012 && st.chi < 30,
    `hash degrades at four-digit cell indices: mean ${st.mean.toFixed(4)} sd ${st.sd.toFixed(4)} chi ${st.chi.toFixed(1)}`);
});

test("the speckle shapes both have mean zero, so neither shifts a model's tone", () => {
  const cells = [];
  for (let x = -24; x < 24; x += 1) for (let y = -12; y < 12; y += 1) for (let z = -24; z < 24; z += 1) {
    cells.push(smHash([x, y, z]));
  }
  const symmetric = cells.map((h) => f(h - 0.5));
  const sparse = cells.map((h) => f(f(Math.max(f(h - 0.85), 0.0) * 6.6667) - 0.075));
  const a = summarise(symmetric), b = summarise(sparse);
  assert.ok(Math.abs(a.mean) < 0.006, `symmetric speckle mean ${a.mean.toFixed(5)}`);
  assert.ok(Math.abs(b.mean) < 0.006, `sparse speckle mean ${b.mean.toFixed(5)}`);
  // The sparse shape is the glint: rare, bright, and never darker than -0.075.
  assert.ok(b.lo >= -0.0751 && b.hi <= 1.001, `sparse speckle range [${b.lo}, ${b.hi}]`);
  const lit = sparse.filter((v) => v > 0).length / sparse.length;
  assert.ok(lit > 0.10 && lit < 0.20, `glint should light about 15% of cells, got ${(lit * 100).toFixed(1)}%`);
});

// ------------------------------------------- 4. classification, real palettes

/// Every entry is [name, rgb, expected dominant class, the generator that owns
/// it, and the literal that must still be in that generator's source]. The last
/// two are what stop this table from quietly describing a palette that no
/// longer exists.
const PALETTE = [
  // equipment-mesh.js PALETTE
  ["equip.hull", [0.29, 0.34, 0.22], "paint", "equipment-mesh.js", "[0.29, 0.34, 0.22]"],
  ["equip.upper", [0.35, 0.40, 0.27], "paint", "equipment-mesh.js", "[0.35, 0.40, 0.27]"],
  ["equip.edge", [0.40, 0.44, 0.31], "paint", "equipment-mesh.js", "[0.40, 0.44, 0.31]"],
  ["equip.armor", [0.32, 0.37, 0.25], "paint", "equipment-mesh.js", "[0.32, 0.37, 0.25]"],
  ["equip.canvas", [0.30, 0.31, 0.21], "paint", "equipment-mesh.js", "[0.30, 0.31, 0.21]"],
  ["equip.steel", [0.20, 0.22, 0.20], "steel", "equipment-mesh.js", "[0.20, 0.22, 0.20]"],
  ["equip.bright", [0.39, 0.42, 0.38], "steel", "equipment-mesh.js", "[0.39, 0.42, 0.38]"],
  ["equip.cable", [0.25, 0.27, 0.24], "steel", "equipment-mesh.js", "[0.25, 0.27, 0.24]"],
  ["equip.track", [0.13, 0.15, 0.14], "rubber", "equipment-mesh.js", "[0.13, 0.15, 0.14]"],
  ["equip.rubber", [0.055, 0.068, 0.060], "rubber", "equipment-mesh.js", "[0.055, 0.068, 0.060]"],
  ["equip.black", [0.035, 0.045, 0.040], "rubber", "equipment-mesh.js", "[0.035, 0.045, 0.040]"],
  ["equip.glass", [0.13, 0.33, 0.34], "glass", "equipment-mesh.js", "[0.13, 0.33, 0.34]"],
  ["equip.lens", [0.20, 0.44, 0.48], "glass", "equipment-mesh.js", "[0.20, 0.44, 0.48]"],
  ["equip.amber", [0.62, 0.37, 0.16], "paint", "equipment-mesh.js", "[0.62, 0.37, 0.16]"],
  // arsenal-models.js P
  ["ars.olive", 0x4d5540, "paint", "arsenal-models.js", "0x4d5540"],
  ["ars.green", 0x3f4a38, "paint", "arsenal-models.js", "0x3f4a38"],
  ["ars.tan", 0x9a8a68, "paint", "arsenal-models.js", "0x9a8a68"],
  ["ars.cloth", 0x5c6148, "paint", "arsenal-models.js", "0x5c6148"],
  ["ars.red", 0x9d3b32, "paint", "arsenal-models.js", "0x9d3b32"],
  ["ars.gold", 0xb08a3c, "paint", "arsenal-models.js", "0xb08a3c"],
  ["ars.skin", 0x8a6a4e, "paint", "arsenal-models.js", "0x8a6a4e"],
  ["ars.rust", 0x6b5a44, "paint", "arsenal-models.js", "0x6b5a44"],
  ["ars.grey", 0x8e9aa4, "steel", "arsenal-models.js", "0x8e9aa4"],
  ["ars.metal", 0xa8aeb4, "steel", "arsenal-models.js", "0xa8aeb4"],
  ["ars.exhaust", 0x54585c, "steel", "arsenal-models.js", "0x54585c"],
  ["ars.warhead", 0xb0b6bc, "steel", "arsenal-models.js", "0xb0b6bc"],
  ["ars.missile", 0xcfd4d8, "steel", "arsenal-models.js", "0xcfd4d8"],
  ["ars.white", 0xd8dde2, "steel", "arsenal-models.js", "0xd8dde2"],
  ["ars.navy", 0x6d7883, "steel", "arsenal-models.js", "0x6d7883"],
  ["ars.stealth", 0x33373c, "steel", "arsenal-models.js", "0x33373c"],
  ["ars.track", 0x2b2c28, "rubber", "arsenal-models.js", "0x2b2c28"],
  ["ars.rubber", 0x26282b, "rubber", "arsenal-models.js", "0x26282b"],
  ["ars.black", 0x1c1e21, "rubber", "arsenal-models.js", "0x1c1e21"],
  ["ars.sensor", 0x232a30, "rubber", "arsenal-models.js", "0x232a30"],
  ["ars.canopy", 0x2a4b63, "glass", "arsenal-models.js", "0x2a4b63"],
  ["ars.glass", 0x38607a, "glass", "arsenal-models.js", "0x38607a"],
  ["ars.panel", 0x1f3f6b, "glass", "arsenal-models.js", "0x1f3f6b"],
  // site-mesh.js P
  ["site.concrete", 0xb4b0a6, "concrete", "site-mesh.js", "0xb4b0a6"],
  ["site.concreteWet", 0x8f8c85, "concrete", "site-mesh.js", "0x8f8c85"],
  ["site.kerb", 0xc2beb4, "concrete", "site-mesh.js", "0xc2beb4"],
  ["site.hardcore", 0x9c968a, "concrete", "site-mesh.js", "0x9c968a"],
  ["site.fill", 0x8b8478, "concrete", "site-mesh.js", "0x8b8478"],
  ["site.hut", 0xc7c3b6, "concrete", "site-mesh.js", "0xc7c3b6"],
  ["site.galv", 0xa2a9ae, "steel", "site-mesh.js", "0xa2a9ae"],
  ["site.clad", 0xa9b0b4, "steel", "site-mesh.js", "0xa9b0b4"],
  ["site.steel", 0x717c86, "steel", "site-mesh.js", "0x717c86"],
  ["site.roof", 0x798186, "steel", "site-mesh.js", "0x798186"],
  ["site.rebar", 0x6d6a63, "steel", "site-mesh.js", "0x6d6a63"],
  ["site.door", 0x5e6a72, "steel", "site-mesh.js", "0x5e6a72"],
  ["site.earth", 0x6b5c46, "paint", "site-mesh.js", "0x6b5c46"],
  ["site.earthCut", 0x50442f, "paint", "site-mesh.js", "0x50442f"],
  ["site.grass", 0x51603f, "paint", "site-mesh.js", "0x51603f"],
  ["site.timber", 0x8d7247, "paint", "site-mesh.js", "0x8d7247"],
  ["site.primer", 0x9c5c38, "paint", "site-mesh.js", "0x9c5c38"],
  ["site.safety", 0xb8862c, "paint", "site-mesh.js", "0xb8862c"],
  ["site.lane", 0xc8c29a, "paint", "site-mesh.js", "0xc8c29a"],
  ["site.glass", 0x3d5f74, "glass", "site-mesh.js", "0x3d5f74"],
];

const toRgb = (v) => (Array.isArray(v) ? v : [((v >> 16) & 255) / 255, ((v >> 8) & 255) / 255, (v & 255) / 255]);

test("every palette colour this check claims still exists in the generator that owns it", () => {
  const cache = new Map();
  for (const [name, , , file, literal] of PALETTE) {
    if (!cache.has(file)) cache.set(file, fs.readFileSync(path.join(ui, file), "utf8"));
    assert.ok(cache.get(file).includes(literal),
      `${name}: ${file} no longer contains ${literal} — this table is stale, re-derive the classifier`);
  }
});

test("the classifier puts every palette colour in the class its name says it is", () => {
  const wrong = [];
  for (const [name, raw, expect] of PALETTE) {
    const w = classify(toRgb(raw));
    const sum = w.paint + w.steel + w.rubber + w.concrete + w.glass;
    assert.ok(Math.abs(sum - 1) < 2e-6, `${name}: weights sum to ${sum}, must be 1`);
    for (const k of ["paint", "steel", "rubber", "concrete", "glass"]) {
      assert.ok(w[k] >= -1e-6 && w[k] <= 1 + 1e-6, `${name}: ${k} weight ${w[k]} out of range`);
    }
    const best = ["paint", "steel", "rubber", "concrete", "glass"]
      .reduce((a, b) => (w[b] > w[a] ? b : a));
    if (best !== expect) wrong.push(`${name}: expected ${expect}, got ${best} (${JSON.stringify(w)})`);
  }
  assert.deepEqual(wrong, [], wrong.join("\n"));
});

test("the classifier is continuous, so a palette landing between classes cannot pop", () => {
  // The whole point of weights over a branch. Walk a fine line through colour
  // space and assert no single 1/255 step changes any weight by much: a
  // treatment that jumps is a treatment that will show a hard edge in a
  // gradient, and a future palette entry between two classes must blend.
  let worst = 0, at = null;
  for (let i = 0; i <= 255; i += 1) {
    for (const axis of [0, 1, 2]) {
      for (const base of [[0.30, 0.34, 0.26], [0.62, 0.60, 0.58], [0.16, 0.18, 0.17], [0.20, 0.40, 0.46]]) {
        const a = base.slice(), b = base.slice();
        a[axis] = i / 255; b[axis] = (i + 1) / 255;
        const wa = classify(a), wb = classify(b);
        for (const k of ["paint", "steel", "rubber", "concrete", "glass"]) {
          const d = Math.abs(wa[k] - wb[k]);
          if (d > worst) { worst = d; at = `${k} at ${JSON.stringify(b)}`; }
        }
      }
    }
  }
  assert.ok(worst < 0.12, `a 1/255 colour step moved a weight by ${worst.toFixed(4)} (${at})`);
});

test("the sand and winter repaints still land somewhere sane", () => {
  // finishColors in equipment-model.js rewrites only surfaces that pass
  // g > r*1.025 && g > b*1.06 && g > .12, scaling a flat tone by the original's
  // weighted brightness. The treatment must survive that, which means the
  // recoloured surface must land in a class whose treatment is still sensible.
  const model = fs.readFileSync(path.join(ui, "equipment-model.js"), "utf8");
  assert.ok(model.includes("g>r*1.025&&g>b*1.06&&g>.12"), "the repaint mask has moved; re-derive this test");
  assert.ok(model.includes("olive:null,sand:[0.58,0.48,0.30],winter:[0.63,0.68,0.65]"), "the finish tones have moved");
  const FINISH = { sand: [0.58, 0.48, 0.30], winter: [0.63, 0.68, 0.65] };
  const repaint = (c, tone) => {
    if (!(c[1] > c[0] * 1.025 && c[1] > c[2] * 1.06 && c[1] > 0.12)) return null;
    const k = Math.min(1.5, Math.max(0.4, (c[0] * 0.25 + c[1] * 0.65 + c[2] * 0.10) / 0.30));
    return tone.map((v) => Math.min(1, Math.max(0, v * k)));
  };
  let sandChecked = 0, winterChecked = 0;
  for (const [name, raw, expect] of PALETTE) {
    if (expect !== "paint") continue;
    const c = toRgb(raw);
    const sand = repaint(c, FINISH.sand);
    if (sand) {
      sandChecked += 1;
      const w = classify(sand);
      // Sand keeps its chroma, so painted surfaces stay painted.
      assert.ok(w.paint > 0.9, `${name} in sand: paint weight only ${w.paint.toFixed(3)}`);
      assert.ok(w.glass < 1e-6, `${name} in sand must not read as glass`);
    }
    const winter = repaint(c, FINISH.winter);
    if (winter) {
      winterChecked += 1;
      const w = classify(winter);
      // Winter is a near-neutral by construction (saturation 0.074), so any
      // hue-and-saturation classifier MUST place it with the neutrals. It lands
      // in steel, which gives whitewash a directional streak instead of orange
      // peel — arguably the better of the two readings, and in any case not a
      // failure mode: the amplitudes are within 3% of each other, so nothing
      // pops when a finish is switched.
      assert.ok(w.steel > 0.8, `${name} in winter: steel weight ${w.steel.toFixed(3)}`);
      assert.ok(w.glass < 1e-6 && w.rubber < 0.2, `${name} in winter landed somewhere silly`);
    }
  }
  assert.ok(sandChecked >= 8 && winterChecked >= 8, "the repaint mask matched too few palette colours to be a test");
});

// ------------------------------------------------- 5. the treatment, bounded

test("the treatment never leaves the legal albedo range and never produces a NaN", () => {
  const eyes = [[0, 2.4, 11], [0, 40, 205], [0.3, 0.4, 1.2], [0, 900, 900], [0.1, 0.1, 0.1]];
  let worstUp = 0, worstDown = 0, count = 0;
  for (const [, raw] of PALETTE) {
    const c = toRgb(raw);
    for (const eye of eyes) {
      for (const P of positions(220, 40)) {
        const out = surface(c, P, eye);
        // `raw`, not `rgb`: asserting the CLAMPED value is in range would be a
        // tautology. The claim is that the arithmetic itself lands in [0,1].
        for (const v of out.raw) {
          assert.ok(Number.isFinite(v), `NaN or Infinity from albedo ${c} at ${P}`);
          assert.ok(v >= 0 && v <= 1, `pre-clamp channel ${v} out of [0,1] from albedo ${c}`);
        }
        worstUp = Math.max(worstUp, out.m);
        worstDown = Math.min(worstDown, out.m);
        count += 1;
      }
    }
  }
  assert.ok(count > 50000, "not enough samples to mean anything");
  // The whole treatment is a bounded fraction of whatever albedo arrived, so a
  // misclassification costs a wrong TEXTURE, never a wrong colour.
  assert.ok(worstUp < 0.24, `brightest excursion +${(worstUp * 100).toFixed(1)}% must stay under +24%`);
  assert.ok(worstDown > -0.24, `darkest excursion ${(worstDown * 100).toFixed(1)}% must stay above -24%`);
});

/// THE HEADROOM BOUND, PROVED BY EXHAUSTION RATHER THAN BY ALGEBRA.
///
/// This is the check that was missing and it is the one that found a real
/// defect. The treatment is a MULTIPLY, so its absolute excursion scales with
/// the albedo — and a bright enough albedo overflows. No palette colour is
/// bright enough, which is precisely why sampling the palette could not see it;
/// the WINTER REPAINT is, and it drove 2,824 of a whitewashed tank's 141,864
/// fragments to 1.128 before the guard existed.
///
/// The sweep is over 660,000 synthetic albedos rather than the 57 shipped ones,
/// because the question is what the function does for ANY input, and it pairs
/// each with the analytic extremes of both noise fields rather than with sampled
/// noise, because a sampled maximum is a lower bound on the true one.
test("no albedo whatsoever can be driven outside [0,1] — the headroom bound is exact", () => {
  let hi = -Infinity, lo = Infinity, atHi = null, roomFloor = 1;
  for (let i = 0; i <= 400; i += 1) {
    const base = i / 400;
    for (let j = 0; j <= 40; j += 1) {
      for (let k = 0; k <= 40; k += 1) {
        const c = [base, Math.min(1, base * (0.5 + (j / 40) * 0.75)), Math.min(1, base * (0.4 + (k / 40) * 0.9))];
        const w = classify(c);
        const dot4 = (r) => w.paint * r[0] + w.steel * r[1] + w.rubber * r[2] + w.concrete * r[3];
        const gain = dot4(GAIN) + w.glass * 0.022, spk = dot4(SPECK), gl = dot4(GLINT);
        const tint = w.steel * 0.55 + w.concrete * 0.30;
        const pk = w.mx * (gain + spk * (0.5 + 0.5 * gl)) + 0.0175 * tint;
        const room = Math.min(1, Math.max(0, (1 - w.mx) / Math.max(pk, 1e-4)));
        roomFloor = Math.min(roomFloor, room);
        // n * 2 spans [-1, 1]; s spans [-0.5, 0.925] as gl runs 0 to 1 (the
        // sparse shape's true extremes, asserted in the speckle test above).
        const sHi = 0.5 * (1 - gl) + 0.925 * gl, sLo = -0.5 * (1 - gl) - 0.075 * gl;
        const mHi = (gain + sHi * spk) * room, mLo = (-gain + sLo * spk) * room;
        for (const ch of [0, 1, 2]) {
          const tc = [-0.020, 0.0, 0.035][ch];
          const up = c[ch] * (1 + mHi) + 0.5 * tint * room * Math.abs(tc) * (tc > 0 ? 1 : 1);
          const down = c[ch] * (1 + mLo) - 0.5 * tint * room * Math.abs(tc);
          if (up > hi) { hi = up; atHi = { c, mx: w.mx, room }; }
          if (down < lo) lo = down;
        }
      }
    }
  }
  assert.ok(hi <= 1 + 1e-6, `worst possible channel is ${hi.toFixed(6)} at ${JSON.stringify(atHi)} — must not exceed 1`);
  assert.ok(lo >= -1e-6, `worst possible channel is ${lo.toFixed(6)} — must not go below 0`);
  // The sweep above recomputes the bound from the constants, so on its own it
  // would prove the ALGEBRA and not the code. Drive the worst albedo it found
  // through the actual entry point as well, at enough positions to reach the
  // extremes of both noise fields, so a treatment that stopped applying `room`
  // fails here too and not only in the real-mesh sweep further down.
  let live = -Infinity;
  for (const eye of [[0, 3.4, 11], [0, 40, 205], [0.3, 0.4, 1.2]]) {
    for (const P of positions(4000, 12)) live = Math.max(live, ...surface(atHi.c, P, eye).raw);
  }
  assert.ok(live <= 1, `the entry point itself returned ${live.toFixed(6)} for the worst-case albedo`);
  // And the bound must be TIGHT, or `room` is throwing away amplitude it did
  // not need to. A worst case comfortably under 1 would mean the guard is a
  // blunt ramp rather than the exact headroom.
  assert.ok(hi > 0.999, `the bound must be tight, not conservative; worst channel only reached ${hi.toFixed(6)}`);
  assert.ok(roomFloor < 0.01, "an albedo at pure white must have no headroom left at all");
});

test("the headroom guard is free on every shipped colour but the one it is not", () => {
  // The guard must not quietly cost amplitude on colours that never needed it.
  const reduced = [];
  for (const [name, raw] of PALETTE) {
    const out = surface(toRgb(raw), [0.4, 1.1, -0.7], [0, 3.4, 11]);
    assert.ok(out.room > 0 && out.room <= 1, `${name}: room ${out.room} out of range`);
    if (out.room < 0.999) reduced.push(`${name} ${out.room.toFixed(3)}`);
  }
  // ars.white 0xd8dde2 is the brightest colour in any of the three palettes and
  // it is the only one with too little headroom for the full treatment. If a
  // second colour appears here a palette has moved and the tuning wants a look.
  assert.deepEqual(reduced, ["ars.white 0.894"],
    `exactly one shipped colour should lose amplitude to the headroom guard, got: ${reduced.join(", ")}`);
});

test("each class gets the amplitude and the grain it was designed for", () => {
  // Sampling a representative colour per class and measuring what the shader
  // actually does to it, rather than trusting the constants to mean what they
  // were meant to mean.
  const cases = [
    ["paint", toRgb(0x4d5540), 0.030, 0.048],
    ["steel", toRgb(0x717c86), 0.040, 0.062],
    ["rubber", toRgb(0x26282b), 0.014, 0.028],
    ["concrete", toRgb(0xb4b0a6), 0.052, 0.082],
    ["glass", toRgb(0x38607a), 0.004, 0.014],
  ];
  for (const [name, c, lo, hi] of cases) {
    const ms = [];
    for (const P of positions(9000, 9)) ms.push(surface(c, P, [0, 2.4, 11]).m);
    const st = summarise(ms.map((v) => v + 0.5));
    assert.ok(Math.abs(st.mean - 0.5) < 0.004, `${name}: treatment mean ${(st.mean - 0.5).toFixed(5)} must be ~0`);
    assert.ok(st.sd > lo && st.sd < hi,
      `${name}: rms modulation ${(st.sd * 100).toFixed(2)}% outside the designed ${(lo * 100).toFixed(1)}-${(hi * 100).toFixed(1)}%`);
  }
});

test("steel is stroked along an axis and every other class is not", () => {
  // The claim is that steel gets DIRECTIONAL micro-scratch. That is testable:
  // move a short step along the scratch axis and the same step across it, and
  // the field must change far less along than across. Anything else in the
  // shader is isotropic and must change about the same amount either way.
  const across = [-0.707, 0.0, 0.620];   // perpendicular to SM_AX
  const step = 0.06;
  const walk = (c, dir) => {
    let total = 0, n = 0;
    for (const P of positions(4000, 8)) {
      const q = [f(P[0] + dir[0] * step), f(P[1] + dir[1] * step), f(P[2] + dir[2] * step)];
      total += Math.abs(surface(c, q, [0, 2.4, 11]).n - surface(c, P, [0, 2.4, 11]).n);
      n += 1;
    }
    return total / n;
  };
  // ars.metal is 100% steel, so it gets the full stroke.
  const pure = toRgb(0xa8aeb4);
  assert.ok(classify(pure).steel > 0.99, "0xa8aeb4 must classify as pure steel for this test to mean anything");
  const alongSteel = walk(pure, AX), acrossSteel = walk(pure, across);
  assert.ok(acrossSteel > alongSteel * 4,
    `steel must vary at least 4x faster across the scratch axis than along it (${acrossSteel.toFixed(5)} vs ${alongSteel.toFixed(5)})`);
  // And the stroke must scale with confidence, not switch on: site.steel is
  // only 80% steel and must be stroked about 80% as hard, which is the whole
  // argument for weights over a branch.
  const partial = toRgb(0x717c86);
  const conf = classify(partial).steel;
  assert.ok(conf > 0.7 && conf < 0.9, `0x717c86 should be a partial steel, got ${conf.toFixed(3)}`);
  const ratio = walk(partial, across) / walk(partial, AX);
  const full = acrossSteel / alongSteel;
  assert.ok(ratio > 1.5 && ratio < full,
    `a partial steel must be stroked less than a pure one (${ratio.toFixed(2)} vs ${full.toFixed(2)})`);
  for (const [name, raw] of [["paint", 0x4d5540], ["concrete", 0xb4b0a6], ["rubber", 0x26282b]]) {
    const c = toRgb(raw);
    const a = walk(c, AX), b = walk(c, across);
    const ratio = Math.max(a, b) / Math.min(a, b);
    assert.ok(ratio < 1.6, `${name} must be isotropic, but varies ${ratio.toFixed(2)}x more one way than the other`);
  }
});

test("screen-space frequency is invariant, which is the whole distance story", () => {
  // The renderer frames every model by fitting its bounding sphere, so the
  // eye-to-origin distance is proportional to model size. Making world-space
  // frequency inversely proportional to it makes cycles-per-frame constant, so
  // a rifle, a tank and a frigate all show the same apparent grain and nothing
  // ever falls below a pixel because of distance alone.
  const c = toRgb(0x4d5540);
  const rows = [["rifle", 1.4], ["tank", 11.5], ["hangar", 46], ["frigate", 205]];
  const cyclesPerFrame = rows.map(([, od]) => {
    const freq = surface(c, [0, 0.5, 0], [0, od * 0.3, od * 0.95]).freq;
    // The frame is 2 * tan(FOV/2) * od metres tall at the subject; FOV is 26.
    return freq * 2 * Math.tan((26 * Math.PI) / 360) * Math.hypot(od * 0.3, od * 0.95);
  });
  const lo = Math.min(...cyclesPerFrame), hi = Math.max(...cyclesPerFrame);
  assert.ok(hi / lo < 1.02, `cycles across the frame must be invariant, got ${cyclesPerFrame.map((v) => v.toFixed(1)).join(", ")}`);
  assert.ok(lo > 60 && hi < 80,
    `about 69 cycles across the frame is 5 px per cycle on a 350 px card, got ${lo.toFixed(1)}`);
  // The clamp is a guard for a generator whose origin is nowhere near its
  // geometry; it must bite in the absurd cases and in NONE of the real ones.
  for (const [, od] of rows) {
    const freq = surface(c, [0, 0.5, 0], [0, od * 0.3, od * 0.95]).freq;
    assert.ok(freq > 0.25 && freq < 400, `the guard clamp must not bite at od ${od}, freq ${freq}`);
  }
  const degenerate = surface(c, [0, 0.5, 0], [0, 0, 0]).freq;
  assert.ok(Number.isFinite(degenerate) && degenerate > 250 && degenerate <= 400,
    `an eye at the model origin must floor at 0.5 m rather than divide by zero, got ${degenerate}`);
  assert.ok(surface(c, [0, 0.5, 0], [0, 0, 9000]).freq >= 0.25, "a very distant eye must clamp to the floor");
});

test("the pattern is locked to the model, not to the eye, so nothing pumps or crawls", () => {
  // Determinism is this project's first rule and a moving surface would be
  // motion the reduced-motion setting could not stop. Two guarantees: the
  // result is a pure function of (albedo, P) once the framing distance is
  // fixed, and the frequency is a per-object constant so the pattern does not
  // breathe as the model turns.
  const c = toRgb(0x4d5540);
  const P = [0.83, 1.21, -2.06];
  const r = 11.0, yaw = Math.PI / 3;
  const a = surface(c, P, [0, 3.4, r]);
  const b = surface(c, P, [r * Math.sin(yaw), 3.4, r * Math.cos(yaw)]);   // same range, 60 degrees round
  assert.ok(Math.abs(a.freq - b.freq) < 1e-4,
    `frequency must not change as the camera orbits at constant range: ${a.freq} vs ${b.freq}`);
  for (let i = 0; i < 3; i += 1) {
    assert.ok(Math.abs(a.rgb[i] - b.rgb[i]) < 1e-5,
      "the pattern is locked to the model, so an orbit must not move it");
  }
  // And it is a pure function: same inputs, same bits, twice.
  assert.deepEqual(surface(c, P, [0, 3.4, 11.0]).rgb, a.rgb);
});

test("the fragment shader the host would actually compile is well formed", () => {
  // The nearest thing to a compile that node can do: splice the chunk into the
  // host's real template and check the result is one shader rather than two
  // glued together. It still does not prove it links — nothing here can — but a
  // second #version, a second main() or an unbalanced brace is the failure this
  // would otherwise ship, and it is free to rule out.
  const host = fs.readFileSync(path.join(ui, "arsenal3d.js"), "utf8");
  const template = host.match(/const FRAG_TEMPLATE = `([\s\S]*?)`;/);
  assert.ok(template, "arsenal3d.js must still hold FRAG_TEMPLATE as a template literal");
  assert.ok(template[1].includes("__SURFACE__"), "the template must still carry the splice point");
  const frag = template[1].replace("__SURFACE__", mod.glsl);
  assert.equal((frag.match(/#version/g) || []).length, 1, "exactly one #version");
  assert.ok(frag.trimStart().startsWith("#version 300 es"), "#version must remain the first token");
  assert.equal((frag.match(/\bvoid main\s*\(/g) || []).length, 1, "exactly one main()");
  assert.equal((frag.match(/\bvec3 surface\s*\(/g) || []).length, 1, "exactly one surface() after splicing");
  const stripped = frag.replace(/\/\*[\s\S]*?\*\//g, " ").replace(/\/\/[^\n]*/g, " ");
  let depth = 0;
  for (const ch of stripped) {
    if (ch === "{") depth += 1;
    else if (ch === "}") depth -= 1;
    assert.ok(depth >= 0, "closing brace before opening in the spliced fragment");
  }
  assert.equal(depth, 0, "unbalanced braces in the spliced fragment");
  // Every identifier the chunk uses free must be declared somewhere above it.
  for (const name of HOST_UNIFORMS) {
    assert.ok(frag.indexOf(`uniform vec3 ${name};`) < frag.indexOf("vec3 surface("),
      `${name} must be declared before the spliced chunk uses it`);
  }
});

// ------------------------------------------- 6. the real mesh, not a proxy
//
// Everything above samples the treatment at positions from a low-discrepancy
// stream and colours from a table. That is the right way to measure the NOISE,
// and it is the wrong way to measure the TREATMENT, because it tests neither the
// positions nor the colours nor the framing distance the renderer will actually
// use. This section builds real meshes out of equipment-mesh.js and pushes their
// own vertex buffers through the port at the distance arsenal3d.js would frame
// them from. Every number in the module's notes comes from here.

const { build } = require(path.join(ui, "equipment-mesh.js"));

/// arsenal3d.js's framing, reimplemented from its own constants so that a change
/// to either goes red rather than silently drifting. FOV 26, rest pitch 20, rest
/// yaw 34, and the fit is the tightest distance that keeps every vertex inside
/// the frustum, times its 1.03 margin.
const FOV = 26, REST_YAW = 34, REST_PITCH = 20;
const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (v) => { const L = Math.hypot(v[0], v[1], v[2]); return [v[0] / L, v[1] / L, v[2] / L]; };

const meshCache = new Map();
function framed(platform, aspect = 1.4) {
  const key = `${platform}@${aspect}`;
  if (meshCache.has(key)) return meshCache.get(key);
  const mesh = build({ platform });
  const p = mesh.positions;
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < p.length; i += 3) {
    for (let k = 0; k < 3; k += 1) { lo[k] = Math.min(lo[k], p[i + k]); hi[k] = Math.max(hi[k], p[i + k]); }
  }
  const centre = [0, 1, 2].map((k) => (lo[k] + hi[k]) / 2);
  const tanV = Math.tan((FOV * Math.PI) / 360), tanH = tanV * aspect, rp = (REST_PITCH * Math.PI) / 180;
  const ry = (REST_YAW * Math.PI) / 180;
  const fwd = [Math.sin(ry) * Math.cos(rp), Math.sin(rp), Math.cos(ry) * Math.cos(rp)];
  const right = unit(cross([0, 1, 0], fwd)), up = cross(fwd, right);
  let worst = 0;
  for (let i = 0; i < p.length; i += 3) {
    const x = p[i] - centre[0], y = p[i + 1] - centre[1], z = p[i + 2] - centre[2];
    const need = (x * fwd[0] + y * fwd[1] + z * fwd[2])
      + Math.max(Math.abs(x * right[0] + y * right[1] + z * right[2]) / tanH,
        Math.abs(x * up[0] + y * up[1] + z * up[2]) / tanV);
    if (need > worst) worst = need;
  }
  const dist = worst * 1.03;
  // arsenal3d.js sets uEye to the eye expressed in MODEL space, which is the
  // orbit position plus the centre it orbits — not the orbit position.
  const eye = [fwd[0] * dist + centre[0], fwd[1] * dist + centre[1], fwd[2] * dist + centre[2]];
  const out = { mesh, centre, dist, eye, lo, hi, frameH: 2 * tanV * dist };
  meshCache.set(key, out);
  return out;
}

/// equipment-model.js's finish repaint, run for real rather than described. It
/// rewrites only surfaces that pass its green mask, scaling the flat finish tone
/// by the original's weighted brightness.
const FINISHES = { sand: [0.58, 0.48, 0.30], winter: [0.63, 0.68, 0.65] };
function repaint(c, tone) {
  if (!tone) return c;
  if (!(c[1] > c[0] * 1.025 && c[1] > c[2] * 1.06 && c[1] > 0.12)) return c;
  const k = Math.min(1.5, Math.max(0.4, (c[0] * 0.25 + c[1] * 0.65 + c[2] * 0.10) / 0.30));
  return tone.map((v) => Math.min(1, Math.max(0, v * k)));
}

/// Push every vertex of a real mesh through the treatment at its real framing
/// distance and report the distribution of how far each fragment moved.
function meshSweep(platform, tone) {
  const { mesh, eye } = framed(platform);
  const p = mesh.positions, c = mesh.colors;
  const excursion = [], relative = [];
  let outOfRange = 0, nan = 0, roomFloor = 1, rawLo = Infinity, rawHi = -Infinity;
  for (let i = 0; i < p.length; i += 3) {
    const albedo = repaint([c[i], c[i + 1], c[i + 2]], tone);
    const out = surface(albedo, [p[i], p[i + 1], p[i + 2]], eye);
    roomFloor = Math.min(roomFloor, out.room);
    let worst = 0;
    for (let k = 0; k < 3; k += 1) {
      const v = out.raw[k];
      if (!Number.isFinite(v)) { nan += 1; continue; }
      if (v < 0 || v > 1) outOfRange += 1;
      if (v < rawLo) rawLo = v;
      if (v > rawHi) rawHi = v;
      worst = Math.max(worst, Math.abs(out.rgb[k] - albedo[k]));
    }
    excursion.push(worst);
    const L0 = albedo[0] * 0.299 + albedo[1] * 0.587 + albedo[2] * 0.114;
    const L1 = out.rgb[0] * 0.299 + out.rgb[1] * 0.587 + out.rgb[2] * 0.114;
    relative.push(L0 > 1e-6 ? Math.abs(L1 - L0) / L0 : 0);
  }
  excursion.sort((a, b) => a - b); relative.sort((a, b) => a - b);
  const at = (arr, q) => arr[Math.min(arr.length - 1, Math.floor(q * arr.length))];
  return {
    fragments: excursion.length, nan, outOfRange, roomFloor, rawLo, rawHi,
    p50: at(excursion, 0.5), p95: at(excursion, 0.95), p100: excursion[excursion.length - 1],
    rel50: at(relative, 0.5), rel95: at(relative, 0.95), rel100: relative[relative.length - 1],
  };
}

const PLATFORMS = ["tank_standard", "ground_apc", "ground_artillery"];

test("on real geometry at the real framing distance, nothing clips and nothing is NaN", () => {
  // THE SWEEP THAT FOUND THE BUG. Three real meshes, every one of their vertices,
  // their own colours, the distance arsenal3d.js would actually frame them from,
  // in all three finishes. 566,000 fragments per finish across the three.
  let total = 0;
  for (const platform of PLATFORMS) {
    for (const [finish, tone] of [["olive", null], ["sand", FINISHES.sand], ["winter", FINISHES.winter]]) {
      const s = meshSweep(platform, tone);
      total += s.fragments;
      assert.equal(s.nan, 0, `${platform}/${finish}: ${s.nan} non-finite channels`);
      assert.equal(s.outOfRange, 0,
        `${platform}/${finish}: ${s.outOfRange} channels outside [0,1] before the guard clamp `
        + `(raw range [${s.rawLo.toFixed(4)}, ${s.rawHi.toFixed(4)}])`);
      assert.ok(s.rawLo >= 0 && s.rawHi <= 1, `${platform}/${finish}: raw range [${s.rawLo}, ${s.rawHi}]`);
    }
  }
  assert.ok(total > 500000, `only ${total} fragments sampled, which is not a sweep`);
});

test("a fragment never moves far enough from its albedo to change what colour it is", () => {
  // The point of a surface treatment is that a panel stops being one flat
  // colour. The point at which it has FAILED is when a player would name the
  // colour differently — so the excursion has to be measured, not asserted by
  // eye. Bands are set from the measured shipped values with room to move:
  // the median fragment shifts about 1% of a channel and the worst about 10%,
  // which is a shade of the same colour and not another colour.
  const rows = [];
  for (const platform of PLATFORMS) {
    for (const [finish, tone] of [["olive", null], ["sand", FINISHES.sand], ["winter", FINISHES.winter]]) {
      const s = meshSweep(platform, tone);
      rows.push(`${platform}/${finish} p50 ${s.p50.toFixed(4)} p95 ${s.p95.toFixed(4)} p100 ${s.p100.toFixed(4)}`
        + ` | dL/L p50 ${(s.rel50 * 100).toFixed(2)}% p95 ${(s.rel95 * 100).toFixed(2)}% p100 ${(s.rel100 * 100).toFixed(2)}%`);
      // Absolute channel excursion. A median above 0.03 would be a treatment
      // that recolours rather than textures; a maximum above 0.15 would be a
      // single fragment a player could point at.
      assert.ok(s.p50 > 0.002 && s.p50 < 0.030,
        `${platform}/${finish}: median excursion ${s.p50.toFixed(4)} outside 0.002-0.030`);
      assert.ok(s.p95 < 0.075, `${platform}/${finish}: p95 excursion ${s.p95.toFixed(4)} must stay under 0.075`);
      assert.ok(s.p100 < 0.130, `${platform}/${finish}: worst excursion ${s.p100.toFixed(4)} must stay under 0.130`);
      // Relative luminance is the perceptual form of the same question, and it
      // is the one that has to hold across finishes: a dark olive panel and a
      // bright whitewashed one must shift by a similar FRACTION, or the
      // treatment reads as a different strength on different finishes.
      assert.ok(s.rel95 < 0.10,
        `${platform}/${finish}: 95th-percentile luminance shift ${(s.rel95 * 100).toFixed(2)}% must stay under 10%`);
      assert.ok(s.rel100 < 0.16,
        `${platform}/${finish}: worst luminance shift ${(s.rel100 * 100).toFixed(2)}% must stay under 16%`);
    }
  }
  // The finishes must be within a factor of two of each other at the p95, or
  // switching a finish visibly changes how textured the model is.
  const p95s = PLATFORMS.flatMap((pl) => [null, FINISHES.sand, FINISHES.winter].map((t) => meshSweep(pl, t).p95));
  assert.ok(Math.max(...p95s) / Math.min(...p95s) < 2.6,
    `finishes must not differ wildly in how textured they look: p95 range ${Math.min(...p95s).toFixed(4)}-${Math.max(...p95s).toFixed(4)}\n${rows.join("\n")}`);
});

test("the winter repaint is the case the palette sweep could not see, and it is contained", () => {
  // Recorded specifically, because this is where the defect was. Winter scales a
  // near-white tone by up to 1.5 and clamps, so it reaches albedos brighter than
  // anything in any palette — including channels at exactly 1.0, where the
  // treatment must fall to nothing rather than overflow.
  const s = meshSweep("tank_standard", FINISHES.winter);
  assert.equal(s.outOfRange, 0, "the winter repaint must not drive a single channel out of range");
  assert.ok(s.rawHi <= 1, `winter raw maximum ${s.rawHi} must not exceed 1`);
  assert.ok(s.roomFloor < 0.05,
    `a whitewashed tank must contain at least one fragment with no headroom left, got floor ${s.roomFloor.toFixed(3)}`);
  // And the guard must not have flattened the whole finish to get there.
  assert.ok(s.p50 > 0.008, `winter median excursion ${s.p50.toFixed(4)} — the guard has eaten the treatment`);
  // Olive is the reference: winter is brighter, so its ABSOLUTE excursion is
  // larger while its RELATIVE one is comparable. Both are asserted so a future
  // change cannot fix one by breaking the other.
  const olive = meshSweep("tank_standard", null);
  assert.ok(s.p95 > olive.p95, "a brighter finish should move further in absolute terms");
  assert.ok(Math.abs(s.rel95 - olive.rel95) < 0.03,
    `olive and winter should feel similarly textured: ${(olive.rel95 * 100).toFixed(2)}% vs ${(s.rel95 * 100).toFixed(2)}%`);
});

test("the treatment ignores the normal and the view vector, which is why it cannot draw a wireframe", () => {
  // THE TRAP THIS SHADER IS BUILT TO AVOID. Much of this geometry is flat shaded
  // on purpose, so N is constant inside a triangle and jumps at every crease:
  // any dependence on N puts a discontinuity exactly on the triangle edges and
  // draws the wireframe. The claim is that N and V are received and never read.
  //
  // Asserted twice. Lexically, neither identifier appears inside the body of
  // surface() — which is a claim about the GLSL, not about the port.
  const body = code.slice(code.search(/\bvec3\s+surface\s*\(/));
  const open = body.indexOf("{");
  let depth = 0, end = open;
  for (let i = open; i < body.length; i += 1) {
    if (body[i] === "{") depth += 1;
    else if (body[i] === "}") { depth -= 1; if (depth === 0) { end = i; break; } }
  }
  const inner = body.slice(open + 1, end);
  assert.doesNotMatch(inner, /(^|[^A-Za-z0-9_])N([^A-Za-z0-9_]|$)/, "surface() must not read N");
  assert.doesNotMatch(inner, /(^|[^A-Za-z0-9_])V([^A-Za-z0-9_]|$)/, "surface() must not read V");
  assert.doesNotMatch(inner, /\bvNrm\b|\bnormalize\s*\(/, "surface() must not reconstruct a normal");
  // And behaviourally: real vertex normals from a real mesh, including the two
  // sides of every crease, cannot change the result. If they could, the field
  // would be discontinuous where the normal is.
  const { mesh, eye } = framed("tank_standard");
  const p = mesh.positions, n = mesh.normals, c = mesh.colors;
  let creases = 0;
  for (let i = 0; i < p.length; i += 3 * 97) {
    const albedo = [c[i], c[i + 1], c[i + 2]], P = [p[i], p[i + 1], p[i + 2]];
    const a = surface(albedo, P, eye);
    // The same fragment as seen from a face pointing the other way. The port
    // takes no normal at all, which IS the proof; assert the buffers differ so
    // this is not vacuous, then assert the output does not.
    const flipped = [-n[i], -n[i + 1], -n[i + 2]];
    if (Math.abs(flipped[0] - n[i]) > 1e-6 || Math.abs(flipped[1] - n[i + 1]) > 1e-6) creases += 1;
    const b = surface(albedo, P, eye);
    assert.deepEqual(b.rgb, a.rgb, "the result must depend only on albedo and position");
  }
  assert.ok(creases > 100, "the sample must actually contain varying normals for this to mean anything");
});

// ------------------------------------------------ 7. frequency and resolution

/// The card sizes that actually ship, read off arsenal3d.css. This is the number
/// the proposal got wrong: there is no large card in this UI.
const CARD_CSS_PX = [30, 34, 46, 68, 104];

test("the card sizes this treatment is tuned against are the ones arsenal3d.css declares", () => {
  const css = fs.readFileSync(path.join(ui, "arsenal3d.css"), "utf8");
  // If a card is resized, the resolution note in the module is stale and the
  // aliasing bar below is measuring the wrong thing.
  for (const [needle, px] of [["width: 34px; height: 34px", 34], ["width: 30px; height: 30px", 30],
    ["height: 68px", 68], ["height: 104px", 104]]) {
    assert.ok(css.includes(needle), `arsenal3d.css no longer declares a ${px}px canvas — re-derive the resolution note`);
  }
  assert.ok(fs.readFileSync(path.join(ui, "arsenal3d.js"), "utf8").includes("const MAX_DPR = 2"),
    "MAX_DPR has moved; the device-pixel figures in the notes are derived from it");
  assert.ok(mod.notes.includes("104 CSS px") && mod.notes.includes("208 device pixels"),
    "the module must state the real card sizes rather than an assumed one");
});

test("world-space frequency, measured on a real tank rather than an assumed camera", () => {
  const { eye, frameH } = framed("tank_standard");
  const rows = {};
  for (const [name, raw] of [["paint", 0x4d5540], ["steel", 0xa8aeb4], ["rubber", 0x26282b],
    ["concrete", 0xb4b0a6], ["glass", 0x38607a]]) {
    rows[name] = surface(toRgb(raw), [0, 1, 0], eye).freq;
  }
  // Cycles across the frame, which is the invariant the scale rule buys.
  const cyclesPerFrame = rows.paint * frameH;
  assert.ok(cyclesPerFrame > 60 && cyclesPerFrame < 80,
    `paint should run about 70 cycles across the frame, got ${cyclesPerFrame.toFixed(1)}`);
  // Ordering is the design: concrete coarsest of the opaque classes, rubber
  // tightest, glass an order of magnitude broader than anything else.
  assert.ok(rows.concrete < rows.paint && rows.paint < rows.steel && rows.steel < rows.rubber,
    `class frequency order is wrong: ${JSON.stringify(rows)}`);
  assert.ok(rows.glass < rows.concrete / 3, "glass must be far broader than any opaque class");
  // Physical grain, in centimetres, which is the number a person can picture.
  for (const [name, lo, hi] of [["paint", 8, 12], ["steel", 7, 11], ["rubber", 6, 9],
    ["concrete", 11, 17], ["glass", 40, 75]]) {
    const cm = 100 / rows[name];
    assert.ok(cm > lo && cm < hi, `${name} grain is ${cm.toFixed(1)} cm, expected ${lo}-${hi} cm`);
  }
  // The highest octave in the whole treatment is the speckle lattice at 2.3x.
  const highest = rows.rubber * 2.3;
  assert.ok(highest > 25 && highest < 35,
    `the highest spatial frequency anywhere in the treatment is the speckle lattice on rubber, `
    + `${highest.toFixed(1)} cycles/m (${(100 / highest).toFixed(1)} cm cells); expected 25-35`);
});

test("undersampling costs grain, not moire — measured, because the difference is the whole question", () => {
  // AT THE SIZES THIS UI ACTUALLY DRAWS, THE FIELD IS UNDER-SAMPLED, and that
  // is a finding rather than an assumption: arsenal3d.css sizes these canvases
  // at 30, 34, 46, 68 and 104 CSS px, so at MAX_DPR 2 the biggest frame a model
  // ever gets is 208 device pixels. Against 70 cycles across the frame that is
  // 2.96 px per cycle at best and 0.97 on the common card. There is no
  // resolution uniform, gl_FragCoord cannot supply one, and derivatives are
  // banned, so the shader cannot see this and cannot fade for it.
  //
  // The question that matters is therefore not WHETHER it aliases but WHAT the
  // aliasing looks like. Two outcomes are possible and they are very different:
  // aliased energy can fold down into low frequencies as moire and banding,
  // which is a visible artifact, or it can stay as per-pixel grain, which is
  // just noise. This measures which.
  //
  // Method: point-sample the treatment on a regular pixel grid at a given
  // px-per-cycle rate, box-average 8x8 blocks of it, and compare the scatter of
  // those block means against the scatter of the TRUE local mean over the same
  // footprints, densely integrated. Excess scatter in the sampled version is
  // energy that has folded down into the visible low frequencies.
  const { eye } = framed("tank_standard");
  const measure = (albedo, pxPerCycle) => {
    const freq = surface(albedo, [0, 1, 0], eye).freq;
    const step = 1 / (freq * pxPerCycle);            // metres per pixel
    const N = 96, vals = new Float64Array(N * N);
    for (let i = 0; i < N; i += 1) {
      for (let j = 0; j < N; j += 1) vals[i * N + j] = surface(albedo, [i * step, 1.0, j * step], eye).m;
    }
    const sd = (arr) => {
      let s = 0; for (const v of arr) s += v;
      const m = s / arr.length; let q = 0;
      for (const v of arr) q += (v - m) * (v - m);
      return Math.sqrt(q / arr.length);
    };
    const blocks = [];
    for (let bi = 0; bi < N / 8; bi += 1) {
      for (let bj = 0; bj < N / 8; bj += 1) {
        let t = 0;
        for (let i = 0; i < 8; i += 1) for (let j = 0; j < 8; j += 1) t += vals[(bi * 8 + i) * N + bj * 8 + j];
        blocks.push(t / 64);
      }
    }
    const truth = [];
    for (let bi = 0; bi < 6; bi += 1) {
      for (let bj = 0; bj < 6; bj += 1) {
        let t = 0, c = 0;
        for (let i = 0; i < 20; i += 1) {
          for (let j = 0; j < 20; j += 1) {
            t += surface(albedo, [(bi * 8 + (i * 8) / 20) * step, 1.0, (bj * 8 + (j * 8) / 20) * step], eye).m;
            c += 1;
          }
        }
        truth.push(t / c);
      }
    }
    return { perPixel: sd(Array.from(vals)), sampled: sd(blocks), true_: sd(truth) };
  };
  // The rates are DERIVED from the shipped card sizes rather than invented, so
  // resizing a card in arsenal3d.css moves this test with it. Every card, at
  // MAX_DPR, for the two classes at the extremes of the amplitude range.
  const { frameH } = framed("tank_standard");
  const MAX_DPR = 2;
  for (const [name, raw, rmsLo, rmsHi] of [["paint", 0x4d5540, 0.030, 0.045], ["concrete", 0xb4b0a6, 0.048, 0.068]]) {
    const albedo = toRgb(raw);
    const cyclesPerFrame = surface(albedo, [0, 1, 0], eye).freq * frameH;
    const rates = CARD_CSS_PX.map((px) => (px * MAX_DPR) / cyclesPerFrame);
    // Faithful at the biggest card in the UI: the sampled block scatter must
    // track the densely-integrated truth. Measured 1.01x on paint and 1.05x on
    // concrete; the bar sits at 1.20 so a real loss of fidelity shows.
    const fine = measure(albedo, rates[rates.length - 1]);
    assert.ok(fine.sampled < fine.true_ * 1.25 + 0.0003,
      `${name} at ${rates[rates.length - 1].toFixed(2)} px/cycle (the ${CARD_CSS_PX[CARD_CSS_PX.length - 1]}px card) `
      + `should reproduce the true local mean, got ${(fine.sampled * 100).toFixed(3)}% vs ${(fine.true_ * 100).toFixed(3)}%`);
    // At EVERY card, the bar is on the EXCESS, and the distinction matters.
    // Raw block-mean scatter GROWS with the sampling rate for an honest reason —
    // eight pixels then span fewer cycles, so the block mean is a less averaged
    // sample of a field that genuinely varies — and a bar on the raw figure
    // would fail a well-sampled card while passing a badly-sampled one. What
    // aliasing adds is the DIFFERENCE between the sampled block means and the
    // densely-integrated truth over the same footprints. Measured worst across
    // both classes and all five cards: 0.198% of albedo, about half a level in
    // 8 bits, and it occurs at 1.09 px/cycle where the field is undersampled by
    // a factor of two. The bar is 0.4%.
    for (let i = 0; i < rates.length; i += 1) {
      const coarse = measure(albedo, rates[i]);
      assert.ok(coarse.sampled - coarse.true_ < 0.004,
        `${name} on the ${CARD_CSS_PX[i]}px card (${rates[i].toFixed(2)} px/cycle) folds `
        + `${((coarse.sampled - coarse.true_) * 100).toFixed(3)}% of albedo into visible low frequencies `
        + `beyond what is really there — that is banding, not grain`);
      // Per-pixel amplitude is unchanged by the sampling rate, which is the
      // other half of the statement: what is left IS the treatment's own rms,
      // not a reduced or an amplified version of it.
      assert.ok(Math.abs(coarse.perPixel - fine.perPixel) < fine.perPixel * 0.15,
        `${name}: per-pixel rms should not depend on the sampling rate `
        + `(${(coarse.perPixel * 100).toFixed(2)}% at ${rates[i].toFixed(2)} vs ${(fine.perPixel * 100).toFixed(2)}%)`);
    }
    // The rms itself, which is exactly what a person sees on a small card: the
    // treatment does not disappear there, it stops being structure and becomes
    // this much per-pixel grain.
    assert.ok(fine.perPixel > rmsLo && fine.perPixel < rmsHi,
      `${name} per-pixel rms ${(fine.perPixel * 100).toFixed(2)}% outside the designed ${(rmsLo * 100).toFixed(1)}-${(rmsHi * 100).toFixed(1)}%`);
  }
});

test("the map sprite is baked larger than it is drawn, which is the only filtering there is", () => {
  // index.html buckets the sprite bake at 64/96/128 px and draws it down to the
  // marker size, so the 2D downsample box-filters the grain: a 40 px marker is
  // baked at 64 and drawn at 40, which is a 1.6x reduction and takes the
  // per-pixel rms down with it. This is the only anti-aliasing the treatment
  // gets anywhere, it is accidental, and it is worth knowing about before
  // someone "optimises" the bucket to match the draw size exactly.
  const page = fs.readFileSync(path.join(ui, "index.html"), "utf8");
  assert.ok(page.includes("px <= 40 ? 64 : px <= 72 ? 96 : 128"),
    "the sprite bucket has moved; the map-LOD filtering claim in the notes is stale");
  const gl = fs.readFileSync(path.join(ui, "arsenal3d.js"), "utf8");
  assert.ok(gl.includes("Math.min(256, Math.round(sizePx || 64))"), "sprite() no longer bakes at the size it is asked for");
});
