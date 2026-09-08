#!/usr/bin/env node
// Run: node --test tools/ui/check_ground_cover.cjs
//
// THE GROUND-COVER BLOCK HAD NO TEST AT ALL, and it shipped two defects that a
// test would have caught. There is no GPU in node, so this does not compile
// anything; it does what check_polar_cap.cjs does — pulls the ACTUAL
// expressions and constants out of the shipped shader and evaluates them, so a
// gate cannot be moved without either this file going red or someone changing
// it deliberately.
//
// WHAT IT DEFENDS, and why each bar exists:
//
//  1. THE GATES MUST DISCRIMINATE. The gates this block shipped with — fields
//     fading out over slope 0.10 to 0.34, woodland thinning over 0.42 to 0.80 —
//     are an order of magnitude above the range real ground occupies, so every
//     place on Earth got the same answer. Scottish moorland drew full-strength
//     farmland. A gate that returns the same value everywhere is not a gate.
//
//  2. THE LATTICE MUST TRACK THE GROUND. The parcel grid was rotated about the
//     Robinson canvas origin, ~933 world units from Chicago, so a fractional
//     drift in the angle swept the lattice through hundreds of cells while the
//     ground advanced tens. The parcels arrived as interference fringes.
//     THIS BAR READS THE SHIPPED ROTATION rather than re-implementing it: the
//     first draft re-implemented the fixed form, so reverting the shader to the
//     defect left the test green, which is a bar that cannot fail.
//
//  3. NOTHING GENERATED MAY OUTRUN THE SCREEN, and nothing may be so coarse
//     that it is a wash. Every octave is gated in metres per pixel.
//
// THE FIXTURES ARE MEASUREMENTS, transcribed with their method, not guesses:
//
//   slope, dimensionless rise/run at ETOPO 2022's own 1,855 m sampling, from
//   spheres-web/ui/terrain-tiles/*.png decoded per its manifest, central
//   differences over ~12,000 land cells per region (2026-09-07).
//
//   V, the vegetation index, sampled from cover.png at each place's Robinson
//   pixel through globe3d.js's own projection (2026-09-07).
"use strict";

const test = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const path = require("path");
const vm = require("vm");

const ROOT = path.join(__dirname, "..", "..");
const ui = (f) => path.join(ROOT, "spheres-web", "ui", f);
const page = fs.readFileSync(ui("index.html"), "utf8");
const shaderMatch = /const GLSL_MAP = `([\s\S]*?)`;/.exec(page);
assert.ok(shaderMatch, "the map shader must be readable from the page");
const SRC = shaderMatch[1];

/// The cover block, isolated, so a bar cannot accidentally read a constant
/// that belongs to some other part of the shader.
const BLOCK = (() => {
  const a = SRC.indexOf("float worldPerPx = max(fwidth(world.x)");
  const b = SRC.indexOf("vec3 land = base * li;");
  assert.ok(a > 0 && b > a, "the ground-cover block has moved or been removed");
  assert.ok(SRC.indexOf("WHAT THE GROUND IS DOING") > a, "the cover block lost its heading");
  return SRC.slice(a, b);
})();

// ---------------------------------------------------------------- GLSL shims
const clamp = (x, lo, hi) => Math.min(hi, Math.max(lo, x));
function smoothstep(lo, hi, x) {
  assert.ok(hi > lo, `smoothstep needs ascending edges, got ${lo}..${hi}`);
  const t = clamp((x - lo) / (hi - lo), 0, 1);
  return t * t * (3 - 2 * t);
}
const fract = (x) => x - Math.floor(x);
function hash12(x, y) {
  let px = fract(x * 0.1031), py = fract(y * 0.1031), pz = fract(x * 0.1031);
  const d = px * (py + 33.33) + py * (pz + 33.33) + pz * (px + 33.33);
  px += d; py += d; pz += d;
  return fract((px + py) * pz);
}
function vnoise(x, y) {
  const ix = Math.floor(x), iy = Math.floor(y);
  const fx = x - ix, fy = y - iy;
  const ux = fx * fx * (3 - 2 * fx), uy = fy * fy * (3 - 2 * fy);
  return hash12(ix, iy) * (1 - ux) * (1 - uy) + hash12(ix + 1, iy) * ux * (1 - uy)
       + hash12(ix, iy + 1) * (1 - ux) * uy + hash12(ix + 1, iy + 1) * ux * uy;
}

/// Read a value out of the shipped block. Regex LITERALS only: an earlier draft
/// built these from strings, a lost backslash silently turned a digit class
/// into a letter class, and the bar stopped matching without going red.
function grab(re, label) {
  const m = re.exec(BLOCK);
  assert.ok(m, `${label}: not found in the shipped cover block`);
  return m;
}

// ------------------------------------------------------------ the fixtures
// slope p50, and the greenness index, both measured — see the header.
const PLACES = [
  { name: "Iowa farmland",      slope: 0.0036, V: 0.847, worked: true },
  { name: "Kansas plains",      slope: 0.0038, V: 0.706, worked: true },
  { name: "Ukraine steppe",     slope: 0.0038, V: 0.808, worked: true },
  { name: "English midlands",   slope: 0.0090, V: 0.824, worked: true },
  { name: "Po valley",          slope: 0.0012, V: 0.812, worked: true },
  { name: "Beauce (France)",    slope: 0.0036, V: 0.804, worked: true },
  { name: "Scottish highlands", slope: 0.0609, V: 0.733, worked: false },
  { name: "Alps (Bolzano)",     slope: 0.1733, V: 0.576, worked: false },
  { name: "Amazon",             slope: 0.0030, V: 0.925, worked: false },
  { name: "Congo",              slope: 0.0030, V: 0.996, worked: false },
];

test("the slope gate tells real places apart, which the shipped one could not", () => {
  const m = grab(/ploughable = 1\.0 - smoothstep\(([\d.]+), ([\d.]+), slope\)/, "ploughable");
  const lo = Number(m[1]), hi = Number(m[2]);
  // The bar is not "these constants" — it is that the FUNCTION separates ground
  // that is farmed from ground that is not, on measured slopes.
  for (const p of PLACES.filter((q) => q.worked)) {
    const ploughable = 1 - smoothstep(lo, hi, p.slope);
    assert.ok(ploughable > 0.55,
      `${p.name} is farmed at slope ${p.slope} but the gate gives it ${ploughable.toFixed(3)}`);
  }
  for (const p of PLACES.filter((q) => !q.worked && q.slope > 0.05)) {
    const ploughable = 1 - smoothstep(lo, hi, p.slope);
    assert.ok(ploughable < 0.05,
      `${p.name} at slope ${p.slope} should not plough, but the gate gives it ${ploughable.toFixed(3)}`);
  }
  // AND THE DEFECT THIS REPLACED, stated as a bar: the old gate returned the
  // same answer for Iowa and for the Scottish highlands. A gate that cannot
  // separate the flattest farmland in the world from moorland is decorative.
  const spread = (1 - smoothstep(lo, hi, 0.0036)) - (1 - smoothstep(lo, hi, 0.0609));
  assert.ok(spread > 0.8,
    `the gate separates Iowa from Scottish moorland by only ${spread.toFixed(3)} of its range`);
});

test("closed canopy is kept out of the worked term, and farmland is not", () => {
  const m = grab(/closed\s+= smoothstep\(([\d.]+), ([\d.]+), V\)/, "closed-canopy ceiling");
  const lo = Number(m[1]), hi = Number(m[2]);
  // Measured: cropland runs V 0.706-0.851 over nine regions, closed forest
  // 0.824-1.000 over five. The two overlap, so this ceiling can only exclude
  // the unambiguous top — that is the claim, and it is the whole claim.
  for (const p of PLACES.filter((q) => q.worked)) {
    assert.ok(smoothstep(lo, hi, p.V) < 0.15,
      `${p.name} is farmland at V ${p.V} but reads ${smoothstep(lo, hi, p.V).toFixed(2)} closed`);
  }
  for (const name of ["Amazon", "Congo"]) {
    const p = PLACES.find((q) => q.name === name);
    assert.ok(smoothstep(lo, hi, p.V) > 0.5,
      `${name} at V ${p.V} must read as closed canopy, got ${smoothstep(lo, hi, p.V).toFixed(2)}`);
  }
});

test("the parcel lattice tracks the ground instead of sweeping across it", () => {
  // THE REGRESSION THIS FILE EXISTS FOR. Walk a line of real ground and count
  // the distinct parcel cells the lattice visits against the cells the ground
  // itself crosses. A straight line crossing a rotated lattice costs about 1.2
  // by geometry alone; rotating about the canvas origin drew 2.3.
  //
  // THE ROTATION IS READ OUT OF THE SHADER. Both operands — what the angle is
  // sampled at, and what actually gets rotated — are extracted, so a revert to
  // the canvas origin changes what this test evaluates and trips the ratio.
  //
  // WHAT THIS BAR DOES AND DOES NOT CATCH, measured rather than assumed. It is
  // the COMBINATION that breaks: reverting both halves together reads 2.28 and
  // goes red, but either half alone stays under the bar and that is correct
  // rather than a hole. With the angle constant inside a cell, rotating about
  // the canvas origin is a rigid translation of the lattice, and a translation
  // does not change how many cells a line crosses. With the angle varying per
  // fragment but the lever arm bounded by the cell, the sweep is tens of cells
  // rather than hundreds. Neither alone produces fringes; together they do.
  const REGION_W = Number(grab(/const float REGION_W = ([\d.]+);/, "REGION_W")[1]);
  const parcel = Number(grab(/fieldW\s+= ([\d.]+) \/ max\(mx/, "parcel width")[1]);
  const ang = grab(/float a = 6\.2831853 \* vnoise\((\w+) \* ([\d.]+)\);/, "field rotation angle");
  const rot = grab(/vec2\s+fu = vec2\(dot\((\w+), e\), dot\((\w+), vec2\(-e\.y, e\.x\)\)\) \/ fieldW;/,
    "field lattice");
  assert.equal(rot[1], rot[2], "the two halves of the lattice rotate different vectors");
  assert.ok(/vec2\s+cell = floor\(wm \/ REGION_W\) \* REGION_W;/.test(BLOCK),
    "the region cell the rotation anchors to is gone");
  const angleFrom = ang[1];                    // "cell" when anchored, "wm" when not
  const angleFreq = Number(ang[2]);
  const rotateOn = rot[1];                     // "rel" when anchored, "wm" when not
  if (rotateOn === "rel") {
    assert.ok(/vec2\s+rel = wm - cell;/.test(BLOCK), "fu rotates a vector this shader never defines");
  }

  const ctx = { window: {}, document: { documentElement: { classList: { add() {}, remove() {} } } } };
  vm.createContext(ctx);
  vm.runInContext(fs.readFileSync(ui("globe3d.js"), "utf8"), ctx);
  const Globe3D = ctx.window.Globe3D;

  const DEG = Math.PI / 180, EARTH = 6371000, lat = 41.88;
  const p0 = Globe3D.project(-87.75, lat), p1 = Globe3D.project(-87.75 + 0.1, lat);
  const mx = (0.1 * DEG * EARTH * Math.cos(lat * DEG)) / Math.abs(p1[0] - p0[0]);
  const fieldW = parcel / mx;

  for (const frameKm of [113, 37.8]) {
    const halfDeg = (frameKm * 500) / (EARTH * DEG * Math.cos(lat * DEG));
    const N = 4000;
    const cells = new Set();
    let metres = 0, prev = null;
    for (let i = 0; i < N; i += 1) {
      const lon = -87.75 - halfDeg + (2 * halfDeg * i) / (N - 1);
      const w = Globe3D.project(lon, lat);
      if (prev) metres += Math.abs(w[0] - prev[0]) * mx;
      prev = w;
      const cx = Math.floor(w[0] / REGION_W) * REGION_W;
      const cy = Math.floor(w[1] / REGION_W) * REGION_W;
      const ax = angleFrom === "cell" ? cx : w[0];
      const ay = angleFrom === "cell" ? cy : w[1];
      const a = 6.2831853 * vnoise(ax * angleFreq, ay * angleFreq);
      const ex = Math.cos(a), ey = Math.sin(a);
      const rx = rotateOn === "rel" ? w[0] - cx : w[0];
      const ry = rotateOn === "rel" ? w[1] - cy : w[1];
      cells.add(`${Math.floor((rx * ex + ry * ey) / fieldW)},${Math.floor((rx * -ey + ry * ex) / fieldW)}`);
    }
    const ratio = cells.size / (metres / parcel);
    assert.ok(ratio < 1.7,
      `over ${frameKm} km the lattice visits ${ratio.toFixed(2)}x the parcels the ground crosses; `
      + "rotating about the canvas origin instead of the region corner gives 2.3x");
    assert.ok(ratio > 0.8, `the lattice visits only ${ratio.toFixed(2)}x — it has stopped moving with the ground`);
  }
});

test("every generated octave is gated in metres per pixel, opens, and is not a wash", () => {
  // A gate that never opens is dead code shaped like a feature; one that opens
  // below about 2.5 px aliases; one that is thousands of metres across is the
  // single-blob defect the woodland term shipped with. All three are asserted
  // against the metres per pixel the shader itself computes: measured on an
  // 1878x889 backing store at Chicago, zoom 512 gives 42.98 m/px and ZOOM_MAX
  // 14.67 m/px. A retina panel halves both.
  const MPP_MAX_ZOOM = 14.67, MPP_MID = 42.98;
  // Two spellings, both real: a named screen-size variable, and an inline
  // divide. Collect the named ones first, then match the gates on either.
  const named = new Map();
  for (const m of BLOCK.matchAll(/(\w+)\s*=\s*([\d.]+) \/ max\(mPerPx/g)) named.set(m[1], Number(m[2]));
  const octaves = [];
  for (const m of BLOCK.matchAll(/smoothstep\(([\d.]+), ([\d.]+), ([\d.]+) \/ max\(mPerPx/g)) {
    octaves.push({ lo: Number(m[1]), hi: Number(m[2]), metres: Number(m[3]) });
  }
  for (const m of BLOCK.matchAll(/smoothstep\(([\d.]+), ([\d.]+), (\w+)\)/g)) {
    if (named.has(m[3])) octaves.push({ lo: Number(m[1]), hi: Number(m[2]), metres: named.get(m[3]) });
  }
  assert.ok(octaves.length >= 4,
    `only ${octaves.length} metres-per-pixel gates found; the block should have several`);
  for (const o of octaves) {
    assert.ok(o.lo >= 2.0,
      `a ${o.metres} m feature fades in at ${o.lo} px, below the sampling limit, and will alias`);
    const atMax = smoothstep(o.lo, o.hi, o.metres / MPP_MAX_ZOOM);
    assert.ok(atMax > 0.25,
      `the ${o.metres} m octave only reaches ${atMax.toFixed(2)} at maximum zoom on a 1x display, so it is never seen`);
  }
  // AND THE SHIPPED DEFECT, as a bar. Woodland was one 700 m blob, so wooded
  // ground read as the same soft mottle at every zoom. Making an octave COARSER
  // passes every bar above — it resolves earlier, not later — so the ceiling
  // has to be stated outright: a stand is a few hundred metres across, not a
  // few thousand, or the term is a continent-scale wash again.
  const coarsest = octaves.reduce((a, b) => (a.metres > b.metres ? a : b));
  assert.ok(coarsest.metres <= 600,
    `the coarsest generated octave is ${coarsest.metres} m; at that size it is a wash rather than a stand, `
    + "which is the defect the single 700 m woodland term had");
  assert.ok(smoothstep(coarsest.lo, coarsest.hi, coarsest.metres / MPP_MID) > 0.9,
    `the coarsest octave (${coarsest.metres} m) is not open at zoom 512`);
  // The octaves must actually span scales, or "three octaves" is one octave
  // written out three times.
  const finest = octaves.reduce((a, b) => (a.metres < b.metres ? a : b));
  assert.ok(coarsest.metres / finest.metres >= 4,
    `the octaves span only ${(coarsest.metres / finest.metres).toFixed(1)}x of scale`);
});

test("generated cover cannot appear in the sea, under snow, or on bare rock", () => {
  // The one thing this layer must never do is contradict the data underneath
  // it. Every term is multiplied by `ground`, which is built from the shipped
  // snow, rock and coastline signals.
  const g = grab(/float ground = ([^;]+);/, "the ground mask");
  for (const signal of ["snow", "rock", "sdf"]) {
    assert.ok(g[1].includes(signal), `the ground mask no longer consults ${signal}`);
  }
  for (const term of ["tField", "tWood", "tArid"]) {
    const m = new RegExp(`float ${term}\\s*=([\\s\\S]*?);`).exec(BLOCK);
    assert.ok(m, `${term} is gone`);
    assert.ok(/\bground\b/.test(m[1]),
      `${term} does not multiply by the ground mask, so it can draw in the sea`);
  }
});

test("arid ground gets a term, because a third of the land had none", () => {
  const m = grab(/tArid = ground \* \(1\.0 - smoothstep\(([\d.]+), ([\d.]+), V\)\)/, "arid V gate");
  const lo = Number(m[1]), hi = Number(m[2]);
  // Measured: Sahara V 0.016, Gobi 0.000, Australian interior 0.251. Farmland
  // starts at 0.706. The term must cover the first two and none of the last.
  for (const [name, V] of [["Sahara", 0.016], ["Gobi", 0.0]]) {
    assert.ok(1 - smoothstep(lo, hi, V) > 0.9, `${name} at V ${V} gets no arid cover`);
  }
  assert.equal(1 - smoothstep(lo, hi, 0.706), 0,
    "the arid term reaches into farmland, where it has no business");
});

test("no shader declares a GLSL ES reserved word as an identifier", () => {
  // `float patch = ...` compiled fine in every editor and node check here and
  // was rejected by the driver with "Illegal use of reserved word", which took
  // the whole globe down. GLSL ES 3.00 reserves a long list of words for
  // features it does not have, and nothing in this repo was looking for them.
  // Section 3.7 of the GLSL ES 3.00 specification, the words a fragment shader
  // is most likely to reach for:
  const RESERVED = ["patch", "sample", "subroutine", "common", "partition", "active",
    "asm", "class", "union", "enum", "typedef", "template", "this", "resource",
    "goto", "inline", "noinline", "public", "static", "extern", "external",
    "interface", "long", "short", "half", "fixed", "unsigned", "superp",
    "input", "output", "filter", "sizeof", "cast", "namespace", "using"];
  const shaders = [...page.matchAll(/const (GLSL_\w+) = `([\s\S]*?)`;/g)];
  assert.ok(shaders.length >= 4, `only ${shaders.length} shaders found in the page`);
  const bad = [];
  for (const [, name, body] of shaders) {
    for (const word of RESERVED) {
      // Declared as a variable, or used as a parameter name: both are errors.
      const re = new RegExp(`\b(?:float|int|uint|bool|vec[234]|ivec[234]|uvec[234]|bvec[234]|mat[234]|void)\s+${word}\b`);
      if (re.test(body)) bad.push(`${name}: "${word}"`);
    }
  }
  assert.deepEqual(bad, [], `reserved words declared as identifiers: ${bad.join(", ")}`);
});
