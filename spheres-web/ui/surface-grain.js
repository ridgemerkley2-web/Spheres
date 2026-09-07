/* Surface grain, proposal P0-A. A fragment-stage albedo treatment: model-space
   value-noise fbm, band-limited to the pixel footprint, plus a per-face tint
   drift. It gives a flat panel a surface without giving it dirt, wear or a
   story. No textures, no UVs, no time, no samplers, no derivatives.

   The whole point is restraint. A 2 m expanse of hull should stop being a
   perfectly uniform field of one colour and start reading as painted steel in
   even light. If a screenshot looks speckled the amplitude constants at the top
   of the chunk are wrong, and they are the only thing that needs to change.

   Exported as a string so the host splices it into whichever fragment shader it
   already has; the JavaScript port in tools/ui/check_surface_grain.cjs is what
   proves the arithmetic behaves. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.Surfacegrain = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  const glsl = `
// ---------------------------------------------------------------------------
// SURFACE GRAIN
//
// vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V) returns an albedo. It does
// not light anything; the host's key, fill, rim and specular all run afterwards
// on whatever comes back.
//
// The treatment is four octaves of value noise sampled from MODEL SPACE, so it
// needs no UVs and applies to every mesh in the deck without a generator
// changing. Each octave is weighted by how many pixels one of its cycles
// covers, and switched off entirely once that falls under about two pixels, so
// the fbm is a band-limited window that slides with the camera rather than a
// fixed pattern that turns to fizz when the model gets small. At map range
// every octave is off and the function returns its input unchanged, bit for
// bit.
//
// Everything is multiplicative on the albedo, which is what makes it survive
// the sand and winter repaints: those rewrite VERTEX colours before upload, and
// a percentage of whatever colour arrives reads the same on olive, on desert
// tan and on white.
//
// PRECISION. The hash is fract/multiply/add only - no sin, no transcendental,
// nothing a driver is free to approximate - so the pattern is identical on
// every device with a real 32-bit float. It needs highp. Spliced into a
// mediump fragment shader on a phone where mediump is fp16, the hash collapses;
// declare highp for float, or do not use this.
// ---------------------------------------------------------------------------

// The one uniform, and it is optional. Metres of model surface across one pixel
// at one metre of view distance:
//
//   gl.uniform1f(loc, 2.0 * Math.tan(fovYradians * 0.5) / drawingBufferHeight)
//
// A uniform the host never sets reads back as zero, and zero selects SG_TEXEL
// below, which is that figure for a 256-pixel-tall card at the 26 degree lens
// arsenal3d.js uses. Leaving it unset is safe; it only means the fade is
// calibrated for a 256 px viewport. Measured on a tank at its resting framing,
// a 40 px sprite then keeps three octaves where two are resolvable, and the
// third runs at 0.65 real pixels a cycle - aliased, though not shimmering, since
// arsenal3d.js bakes each sprite once per size and there is no clock in here.
// Setting it is one line and it is the line that makes the detail honest.
uniform float uSurfaceTexel;

const float SG_TEXEL  = 0.0018;   // fallback metres/pixel/metre: 26 deg, 256 px
const float SG_GAIN   = 1.0;      // 1.0 in BOTH hosts; read SPLICING before changing
const vec4  SG_FREQ   = vec4(0.40, 1.44, 5.20, 18.70);   // cycles per metre
const vec4  SG_AMP    = vec4(0.026, 0.026, 0.024, 0.022); // fraction of albedo
const float SG_CUT_LO = 0.18;     // cycles/pixel where an octave starts fading
const float SG_CUT_HI = 0.44;     // cycles/pixel where it is gone (Nyquist is 0.5)
const float SG_FACE   = 0.017;    // per-face tone drift, plus or minus
const float SG_TEMP   = 0.012;    // per-face warm/cool drift, plus or minus
const float SG_FLOOR  = 0.03;     // additive share, so dark parts are not dead
const float SG_BUCKET = 3.7;      // normal quantisation, buckets per unit
const float SG_DITHER = 0.30;     // ragged the bucket edges by this much
const float SG_SEED   = 0.31;     // keeps the hash off its zero fixed point
const float SG_GRAZE  = 0.15;     // floor on cos(N,V) for the footprint stretch
const float SG_HASH_A = 1031.0;   // hash sensitivity; see LATTICE GHOST below
const float SG_HASH_B = 27.3;     // hash offset
const vec3  SG_KEY    = vec3(0.1031, 0.1197, 0.0973);    // lattice decorrelators

// Hash. Lattice coordinates are multiplied by SG_KEY before they arrive, which
// keeps the argument small and leaves the whole mantissa for the fraction; a
// large integer key hashed through fract() throws away the low bits and the
// noise goes blocky in the distance where the model is biggest.
//
// SG_SEED matters more than it looks. Without it a key of exactly zero - the
// cell at the model origin, which is dead centre of every asset in this
// project - hashes to exactly zero and that one lattice point is always the
// darkest value the function can produce.
//
// LATTICE GHOST, and why SG_HASH_A is 1031 and not the 43 this shipped with.
// The corner key is a 2D-to-1D projection, i.x*Kx + i.y*Ky, and integer offsets
// (a,b) exist for which a*Kx + b*Ky lands within a thousandth of a whole
// number - (14,13) lands within 0.0005 of 3. Two corners that far apart get
// almost the same key, and if the hash is SMOOTH on that scale they get almost
// the same value, so the whole field repeats at a fixed diagonal offset. At 43
// the measured self-correlation at (14,13) was r = 0.74, with 71 of 1676 cell
// offsets past |r| = 0.08: a tiling ghost about a metre across at the finest
// octave. At 1031 the same sweep is r = 0.05 with none past 0.08, which is
// what an exact integer-mixing hash scores on the same test. The cost is the
// same instruction; only the constant changed.
float sgHash(float k) {
  float h = fract(k) + SG_SEED;
  return fract(h * (h * SG_HASH_A + SG_HASH_B));
}

vec4 sgHash4(vec4 k) {
  vec4 h = fract(k) + SG_SEED;
  return fract(h * (h * SG_HASH_A + SG_HASH_B));
}

// Value noise on a 2D lattice, returned centred on zero across -1..1. The two
// interpolations are written out rather than handed to mix(): mix() is specified
// as x*(1-a)+y*a but every GPU fuses it into a single lerp, and the JavaScript
// port that checks this file should be running the arithmetic the hardware runs.
//
// CRACKED CORNERS, and why each corner key is built from its own coordinates.
// This used to compute one key for the cell and add SG_KEY to reach the other
// three corners. That is a corner shared by two cells reached by two different
// float expressions - base + Kx from the left cell, (i.x+1)*Kx from the right -
// and those differ by an ulp. An ulp is nothing until it lands inside fract(),
// where it can flip a corner from 0.999 to 0.001 and put a full-amplitude step
// down a cell boundary. Measured on the shipped form: 0.004% of corners near
// the origin, 0.24% at 2600 cells out, worst case a complete 1.0 jump - and at
// the sensitivity this hash now needs, 0.16% and 43%. Building every corner key
// from that corner's own integer coordinates makes the two evaluations bit
// identical and the crack rate exactly zero at every distance. It costs three
// vec4 operations and it is not optional at SG_HASH_A = 1031.
float sgNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 t = p - i;
  t = t * t * (3.0 - 2.0 * t);
  vec4 cx = i.x + vec4(0.0, 1.0, 0.0, 1.0);
  vec4 cy = i.y + vec4(0.0, 0.0, 1.0, 1.0);
  vec4 h = sgHash4(cx * SG_KEY.x + cy * SG_KEY.y);
  float a = h.x + t.x * (h.y - h.x);
  float b = h.z + t.x * (h.w - h.z);
  return (a + t.y * (b - a)) * 2.0 - 1.0;
}

// Model space is three-dimensional and the noise is two-dimensional, so pick the
// plane the surface most nearly lies in. Two octaves of a full 3D lattice cost
// more than four octaves of this.
//
// The dropped axis is not dropped: it shears the other two. Without that shear
// the noise is extruded along the projection axis, and on a tank the hull roof
// and the turret roof - both facing up, both projecting to xz - would carry the
// identical pattern stacked one above the other.
//
// Where two axes are equally dominant the projection switches. On a flat-shaded
// panel the normal is constant so the whole face picks one plane and there is no
// seam inside it. On a smooth-shaded barrel the switch happens four times around
// the circumference, and what it produces is a change of pattern with no change
// of mean - decorrelation, not a line.
vec2 sgPlane(vec3 N, vec3 P) {
  vec3 a = abs(N);
  if (a.y >= a.x && a.y >= a.z) return P.xz + P.y * vec2(0.31, 0.53);
  if (a.x >= a.z) return P.zy + P.x * vec2(0.53, 0.31);
  return P.xy + P.z * vec2(0.31, 0.53);
}

vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V) {
  // How much surface one pixel covers here. gl_FragCoord.w is 1/w_clip, which
  // for any ordinary perspective matrix is one over the view depth, so this is
  // a distance measure that costs a reciprocal and needs no uniform and no
  // derivative. inversesqrt(cos) widens it at grazing angles: the footprint
  // there is an ellipse whose area grows as 1/cos, and the isotropic circle of
  // the same area has radius sqrt(1/cos). Using 1/cos instead over-blurs every
  // sloped panel, which is how a hull side loses its grain while the deck keeps
  // it.
  float texel = uSurfaceTexel > 0.0 ? uSurfaceTexel : SG_TEXEL;
  float mpp = texel / max(gl_FragCoord.w, 1e-4) * inversesqrt(max(abs(dot(N, V)), SG_GRAZE));

  // Cycles per pixel for each octave, turned into a weight. A linear ramp
  // rather than smoothstep: it is four ALU cheaper per octave and the only
  // thing that ever crosses it is a slow zoom.
  vec4 w = clamp((SG_CUT_HI - mpp * SG_FREQ) / (SG_CUT_HI - SG_CUT_LO), 0.0, 1.0);

  // Past the coarsest octave there is nothing left to say. Returning albedo
  // itself rather than albedo times one keeps map range exactly free and
  // exactly identical to the current renderer.
  if (w.x <= 0.002) return albedo;

  vec2 q = sgPlane(N, P);
  float m = 0.0;
  float fine = 0.0;
  // Each octave after the first is rotated by an angle that is not a multiple
  // of anything, because a value-noise lattice has axis-aligned structure and
  // these models are built on the axes. Stacking unrotated octaves on a hull
  // draws a faint grid down it.
  m += SG_AMP.x * w.x * sgNoise(q * SG_FREQ.x);
  if (w.y > 0.002) {
    fine = sgNoise(mat2(0.8018, 0.5976, -0.5976, 0.8018) * q * SG_FREQ.y + 5.7);
    m += SG_AMP.y * w.y * fine;
  }
  if (w.z > 0.002) {
    fine = sgNoise(mat2(0.6549, -0.7558, 0.7558, 0.6549) * q * SG_FREQ.z + 11.3);
    m += SG_AMP.z * w.z * fine;
  }
  if (w.w > 0.002) {
    fine = sgNoise(mat2(0.9111, 0.4122, -0.4122, 0.9111) * q * SG_FREQ.w + 23.9);
    m += SG_AMP.w * w.w * fine;
  }

  // Per-face drift. There is no face id in this pipeline and no flat qualifier
  // in use, so the face is identified by its own normal: quantise N and hash the
  // bucket. A flat-shaded panel has one normal, so it gets one constant, and the
  // panel beside it at a different angle gets an unrelated one - which is the
  // whole ask, two panels of the same paint that are not the same pixel value.
  //
  // On a smooth-shaded surface the buckets would band, so the finest live octave
  // dithers the bucket edge. That trades a clean contour line for a ragged one
  // at the scale of the grain, which the eye reads as part of the grain.
  vec3 b = floor(N * SG_BUCKET + (0.5 + SG_DITHER * fine));
  float fh = sgHash(dot(b, SG_KEY));
  float v = (m + (fh * 2.0 - 1.0) * SG_FACE * w.x) * SG_GAIN;
  float t = (fract(fh * 7.13 + 0.37) * 2.0 - 1.0) * SG_TEMP * w.x * SG_GAIN;

  // Multiplicative, so the modulation is a percentage of whatever colour the
  // finish pass left behind. SG_FLOOR adds a sliver of absolute swing on top,
  // because a percentage of near-black is nothing and the tracks and tyres would
  // otherwise be the only perfectly flat things left in the frame. It is a
  // sliver and it does not fix them; see the notes.
  //
  // The last line is a colour temperature tilt, red against blue. Panels of one
  // paint differ in tone before they differ in hue, but they do differ in hue,
  // and a whole vehicle in one exact chroma is one of the things that reads as
  // computed rather than built.
  //
  // Clamped at both ends, not just at zero. The winter repaint multiplies a
  // bright olive panel up to a luminance of 0.945, and a positive excursion on
  // top of that leaves the unit interval: measured over the real tank, APC and
  // artillery meshes, a handful of fragments per model reached 1.017. Nothing
  // downstream in either renderer clamps the albedo before it lights with it,
  // so the ceiling belongs here, where the contract that albedo is 0..1 is
  // stated. It costs three operations and it is the difference between
  // returning an albedo and returning something that only usually is one.
  vec3 c = albedo * (1.0 + v) + v * SG_FLOOR;
  return clamp(c * vec3(1.0 + t, 1.0, 1.0 - t), 0.0, 1.0);
}
`;

  const notes = [
    "ENTRY POINT. vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V) -> albedo to shade with.",
    "It performs no lighting: the host's key, fill, rim, specular and contact terms all run after it.",
    "V is used only to widen the pixel footprint at grazing angles, never to light or to tint.",
    "",
    "SPLICING. GLSL 3.00 ES function definitions and constants only: no #version, no in/out, no main,",
    "no precision statement. Paste above the host's main() and below its precision statement.",
    "",
    "  arsenal3d.js is ready for it and needs no edit at all. Its fragment template already carries a",
    "  __SURFACE__ marker, an identity default, precision highp float, and the call site is already",
    "  written: vec3 alb = surface(vCol, N, vPos, V), with N flipped toward the viewer first and the",
    "  specular term deliberately still reading the untouched vCol. Hand the string to setSurface().",
    "  It does not gamma-encode its output, so SG_GAIN stays 1.0. It does not set uSurfaceTexel; see",
    "  UNIFORM below for what that costs.",
    "",
    "  equipment-model.js needs two edits by its owner and they are not this file's to make. Its",
    "  fragment shader declares precision mediump float and passes its varyings mediump, which on a",
    "  device where mediump is fp16 collapses this hash entirely - it must say highp. And SG_GAIN at",
    "  the vColor call site is 1.0, NOT the 2.2 that a gamma-encoding host would normally want: that",
    "  shader decodes with pow(vColor, 2.2) and re-encodes with pow(lit, 1/2.2), so a multiplicative",
    "  v applied BEFORE the decode round-trips to v in the displayed pixel. Measured through its own",
    "  lighting, SG_GAIN 1.0 shows 1.66% for a designed 2% and SG_GAIN 2.0 shows 3.33%, which is",
    "  double the design and is the speckle this brief exists to avoid. 2.2 is the right number only",
    "  if the call is moved after the decode, onto the linear base.",
    "",
    "The chunk is also valid GLSL 1.00 ES for a WebGL1 viewer, provided that shader declares highp.",
    "",
    "UNIFORM. One, optional: uniform float uSurfaceTexel. Set it to",
    "2*tan(fovY/2)/drawingBufferHeight, which is metres of surface per pixel at one metre of depth.",
    "Unset it reads back as zero and the chunk falls back to the figure for a 256 px viewport at the",
    "26 degree lens arsenal3d.js uses, so nothing breaks. What it costs is measured rather than",
    "guessed, on a real tank at the real resting framing:",
    "",
    "  512 px  set: four octaves, the finest at 2.3 px a cycle. Unset: three, and the fourth is",
    "          given away for nothing.",
    "  256 px  set and unset agree - three octaves, the finest 5.20 c/m at 4.19 px a cycle.",
    "  128 px  set: two octaves. Unset: three, and the third runs at 2.09 REAL px a cycle.",
    "   64 px  set: two octaves. Unset: three, and the third runs at 1.05 REAL px a cycle.",
    "   40 px  set: two octaves. Unset: three, and the third runs at 0.65 REAL px a cycle.",
    "",
    "Below 256 the third octave is therefore past Nyquist and aliases. It does not SHIMMER: there is",
    "no clock in this chunk and arsenal3d.js bakes each sprite once per size and caches it, so the",
    "error is a fixed pattern in a still image. On a tank drawn at 40 px the median fragment moves",
    "1.67% of its albedo instead of the 1.22% it should, and the worst 7.13% instead of 4.69%. It is",
    "wrong, it is small, and one gl.uniform1f in renderTo - where w and h are already in hand - is",
    "the whole fix.",
    "",
    "PRECISION. Requires highp float. The hash uses only multiply, add, floor and fract, so the",
    "pattern is the same on every conformant 32-bit device; the single approximate operation in the",
    "chunk is the inversesqrt in the footprint estimate, and it only moves a fade weight.",
    "",
    "MEASURED, on the real meshes rather than on a sphere of random normals. Five platforms, three",
    "finishes, every vertex, framed the way arsenal3d.js frames a card at 256 px. The modulation is",
    "2.29 to 2.37% RMS; the median fragment moves 1.66 to 1.74% of its albedo, the 95th percentile",
    "4.33 to 4.47%, the 99.9th 6.12 to 6.28%, and the worst single fragment across all fifteen",
    "combinations 7.13%. In 255ths of a step that worst case is 10 on olive and 18 on the winter",
    "repaint, against a median of 1.1 and 2.4. Nothing leaves 0..1, nothing comes back NaN, and the",
    "whole-model mean sits between -0.40 and -0.61%, so no vehicle is quietly darkened.",
    "",
    "BY MATERIAL, tank_main, green channel in 255ths, median and worst. Paint (hull, upper, armor,",
    "edge, 0.34 to 0.44 albedo) 1.7-2.1 and 5.5-7.4, rising to 3.5-4.3 and 11-16 once winter has",
    "lifted it. Steel 0.22: 1.1 and 4.5. Shade 0.27: 1.1 and 4.8. Track 0.15: 0.8 and 2.8. Rubber",
    "0.068: 0.44 and 1.6. Black 0.045: 0.29 and 1.1 - a third of one step, which is to say nothing.",
    "Glass 0.33 and lens 0.44 are not repainted by any finish and keep 2.0 and 1.8 median in all",
    "three, which is the deliberate wrong answer for a windscreen; see SCOPE.",
    "",
    "TUNING. Every constant is at the top of the chunk. SG_AMP is the amplitude of each octave as a",
    "fraction of albedo and is the only knob that matters; halving it halves the effect. SG_GAIN is a",
    "single multiplier, and read the SPLICING note before assuming a gamma-encoding host wants 2.2 -",
    "where in the pipeline the call sits decides that, not whether the host encodes. Deleting the",
    "fourth octave is one block and costs nothing above about a two-metre asset; the three that",
    "remain must stay a geometric ladder, which the check asserts.",
    "",
    "SCOPE. This is grain and micro-variation only. There is deliberately no dirt, no rust, no wear,",
    "no edge highlight and no height term: nothing here knows which way is dirty. Near-black materials",
    "- rubber, track, tyre - stay essentially flat, because a percentage of 0.04 albedo is not a",
    "visible quantity and lifting them with an additive term would grey them out.",
    "",
    "TWO DEFECTS FOUND AFTER THE FIRST DRAFT, both by the JavaScript port and neither visible to any",
    "amount of reading. First, the 2D-to-1D corner key has near-collisions - offsets like (14,13)",
    "land within 0.0005 of a whole number - and against the original hash that repeated the whole",
    "field at a fixed diagonal offset with r = 0.74, on 71 of 1676 cell offsets. SG_HASH_A carries",
    "the fix. Second, a corner shared by two cells was reached by two different float expressions,",
    "and the ulp between them landed inside fract(): a full-amplitude step down a cell line on 0.24%",
    "of corners far from the origin. Building each corner key from its own coordinates carries that",
    "one. Both are asserted in tools/ui/check_surface_grain.cjs against the rejected form, which is",
    "shown there to go red.",
  ].join("\n");

  return Object.freeze({
    id: "surface-grain",
    glsl,
    notes,
  });
});
