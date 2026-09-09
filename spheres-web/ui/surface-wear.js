// SURFACE WEAR — weathering and accumulation, as a fragment-shader chunk.
//
// WHAT THIS IS FOR. Every model in this project is shaded from per-vertex RGB
// and nothing else: no UVs are emitted anywhere, so no image can be sampled,
// and the result is that a hull panel is a perfectly uniform field of one
// colour no matter how many chamfers and weld beads the fidelity pass put into
// it. A 47,288-triangle tank carrying 62 distinct vertex colours still reads as
// a toy, and the reason is not the triangles and not the albedo — it is that
// real surfaces are not uniform. This chunk supplies the missing variation
// from world position alone, which needs no UV and therefore touches no
// generator.
//
// THE DIRECTION IT COMMITS TO. Dirt obeys gravity and geometry, and both are
// in hand: P.y is height above the ground plane for every asset here, and N
// says which way a surface faces. Upward faces collect dust. Low and downward
// faces collect grime. Near-vertical faces carry streaks running down. Diagonal
// faces — the chamfers this deck is full of — get a little paint rubbed thin.
// The target is a 1990 army vehicle in service: MAINTAINED, not abandoned.
//
// HOW LOUD IT IS ALLOWED TO BE, AND WHY THAT NUMBER. The finish selector is the
// loudest legitimate colour change in this UI: measured on the real tank mesh,
// the sand repaint moves a vertex's luminance by 64% and the winter repaint by
// 122%. A weathering pass that moves a colour further than the colour control
// does has stopped being weathering, because the player can no longer tell
// which one they are looking at. So the binding rule here is that NO FRAGMENT
// MAY MORE THAN DOUBLE OR HALVE ITS OWN LUMINANCE — measured, over every vertex
// of a real tank in all three finishes, at 86% worst case and 5% median. An
// earlier draft of this file reached 274% on black rubber; that is recorded
// below because the fix is a load-bearing part of the design and not a tweak.
//
// WHY SINES AND NOT A HASH. The obvious noise is a lattice value noise with an
// integer hash. It costs eight hashes an octave and — more to the point — a
// hash amplifies the last bit of its input, so two drivers that disagree by one
// ULP produce two DIFFERENT surfaces. Determinism is this project's first rule.
// A product of warped sines is a SMOOTH function of position: a driver whose
// sin() differs in the seventh digit produces a picture that differs in the
// seventh digit.
//
// It is also cheaper, though not by as much as it first looks and the number is
// worth writing down honestly rather than waving at. Four octaves of lattice
// value noise cost about 280 scalar ops — eight hashes an octave at five ops
// each, plus the trilinear blend. Four octaves of this cost about 140 scalar
// ops, but 24 of those are sin, and a GPU issues sin on a special-function unit
// at roughly a quarter rate, so call it 210 ALU-equivalent cycles against 280.
// That is a 25% saving, not a 75% one. COST IS NOT THE REASON THIS IS HERE.
// Determinism is. The saving is a bonus and should not be quoted as the case.
//
// THE FOLD, AND THE BUG THAT LIVED IN IT. Every octave folds its coordinate
// into one period before it meets a sine, so that a 400 m task group never
// hands a driver ten thousand radians and never depends on the quality of its
// range reduction. The first version of this file folded by 2*pi, on the
// reasoning that sin is 2*pi periodic. THAT REASONING WAS WRONG AND THE FIELD
// WAS NOT: swField is sin(q + 0.85*sin(1.7*q' + 0.6)), so the INNER sine sees
// 1.7*q, and shifting q by 2*pi shifts the warp by 1.7 periods — not a whole
// number of them. The fold was therefore a real discontinuity. Measured, before
// the fix: the field jumped by up to 0.73 of its own ±1 range across hard
// planes spaced every 23 cm in the fine octave and every 1.07 m in the mid one,
// which is a grid of value cliffs across every panel in the deck — precisely
// the printed-grid artefact the oblique noise frame exists to avoid.
//
// The fix is the period, not the warp: 1.7 is 17/10, so the field's true period
// is TEN times 2*pi, and folding there is an exact identity (1e-11 in doubles,
// and the check asserts it THROUGH swField rather than through a bare sine,
// which is how the bug survived its first test). Two things came free with it.
// The fine octave's peak slope fell from 1622 per metre to 45, i.e. its highest
// real frequency went from 258 cycles/m — a 4 mm feature, which was the cliff
// edge, not a feature — down to 7.2 cycles/m. And every octave's repeat length
// grew tenfold, from 1.12 m to 11.2 m for the mid band, so the mottle no longer
// tiles eight times across one tank. The arguments a sine sees are now bounded
// by 10*pi instead of pi, which costs nothing at all in fp32.
//
// NO CLOCK, NO ANIMATION, NO DERIVATIVES. Nothing here reads a clock: the same
// fragment of the same model is the same colour on every frame and in every
// session, which is what lets a card be drawn once and cached as a bitmap.
// Nothing here reads dFdx/dFdy/fwidth either, and that is not squeamishness:
// much of this geometry is deliberately flat-shaded, so the derivative of the
// normal is exactly zero inside a triangle and spikes at its edges, and any
// "curvature" built from it draws a wireframe.
//
// THE ONE BUILT-IN IT READS. surface() needs to know how far away the fragment
// is, so that a 10 cm grain is not drawn onto a model that is forty pixels
// wide. The host's entry point offers no distance, so it is recovered from
// gl_FragCoord.w, which under any ordinary perspective matrix is 1/(-z_eye):
// its reciprocal is the distance from the eye plane in metres. That is the
// ONLY built-in read and the ONLY place a host assumption lives, which is why
// it sits alone in surface() and the whole treatment lives in swWeather(), a
// pure function of (albedo, N, P, depth) that the check can port to JS and
// measure. Under an orthographic matrix w is 1 and every fragment reads as one
// metre away: the failure mode is FULL detail, not a crash and not a blank.
//
// WHAT DEPTH CANNOT TELL IT, said here because it undercuts the obvious reading
// of those fades. arsenal3d.sprite(id, 64) and the 200-pixel catalogue card
// call the same fitDistance(), which depends on the model and the aspect ratio
// and NOT on the pixel size — so a tank is 15.5 m from the eye whether it is
// being drawn at 200 pixels or at 64. Distance therefore separates a frigate
// from a tank, which is what the fades are really doing, and it CANNOT separate
// a card from a map sprite. See the notes and the check for what was measured
// instead: the treatment adds 0.006 of point-sampling error at 64 px where the
// geometry itself already contributes 0.170, and it is static rather than
// shimmering because there is no clock.
(function (root, factory) { const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.Surfacewear = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () { "use strict";
  return Object.freeze({
    id: "weathering",
    // GLSL 3.00 ES source, function definitions only — no version pragma, no
    // in/out declarations, no uniforms, no entry function of the host's own.
    // It is spliced into an existing fragment shader.
    glsl: `
// ---------------------------------------------------------------- constants
// THE FIELD'S TRUE PERIOD, which is not 2*pi. swField phase-warps at 17/10 of
// the carrier, so it only repeats after ten carrier periods, and folding a
// coordinate anywhere else puts a hard value cliff at every fold plane. The
// check derives this number from the warp rate rather than trusting it.
const float SW_PERIOD = 62.8318530718;
const float SW_INV_PERIOD = 0.015915494309;

// THREE NOISE AXES, ORTHONORMAL AND OFF EVERY AXIS OF THE MODELS. A sine
// product evaluated on x, y and z draws a rectangular lattice, and this deck's
// geometry is overwhelmingly axis-aligned boxes and chamfers between them, so
// a pattern lined up with either would land exactly on the panels and read as
// a printed grid. This frame is the numerical maximum of the worst angle to
// any of the 26 lattice directions — the 6 face normals, the 12 edge diagonals
// and the 8 corner diagonals — which is 26.7 degrees, and it is orthonormal to
// 8e-7 after rounding to five places, so the field's frequency is the same in
// every direction. The first frame written here missed this and sat 5 degrees
// off the (0,1,1) edge diagonal, which is the direction of every top chamfer
// in the deck; the check caught it, which is what the check is for.
const vec3 SW_AXIS_A = vec3( 0.89050, -0.26438,  0.37030);
const vec3 SW_AXIS_B = vec3( 0.37232,  0.89121, -0.25907);
const vec3 SW_AXIS_C = vec3( 0.26152, -0.36857, -0.89205);

// swBasis(vec3(0.0, 0.55, 0.0)) * 3.41, folded to a constant. Adding this to
// the coarse coordinate samples the field 0.55 m ABOVE the fragment, which is
// what makes a streak hang from a blotch rather than appear on its own.
const vec3 SW_SOURCE_LIFT = vec3(-0.495845, 1.671464, -0.691253);

// Road dust and wet grime, as absolute colours rather than as a scaling of the
// albedo. This is the whole answer to the finish repaint: sand and winter
// rewrite the vertex colour before upload, and dust that is MIXED TOWARD a
// fixed tan lands the same way on olive, on sand and on winter white, because
// in life dust does not care what it settles on.
const vec3 SW_DUST = vec3(0.520, 0.470, 0.380);
const vec3 SW_GRIME = vec3(0.075, 0.058, 0.042);

// ------------------------------------------------------------------- noise
vec3 swBasis(vec3 p) {
  return vec3(dot(p, SW_AXIS_A), dot(p, SW_AXIS_B), dot(p, SW_AXIS_C));
}

// FOLD ANY COORDINATE INTO ONE PERIOD OF THE FIELD before it meets a sine.
// This is an identity — the function it feeds has exactly the same value
// either side of a fold — PROVIDED the period is the field's own and not the
// sine's. It is not the sine's: see the header. What it buys is that no
// argument ever exceeds 10*pi, which matters because the largest model in the
// deck is 400 m across and the finest octave would otherwise hand a driver
// 10,800 radians and depend on the quality of its range reduction. Every
// octave folds its OWN coordinate; folding once and multiplying afterwards
// would put a real discontinuity at every cell wall.
vec3 swWrap(vec3 x) {
  return (fract(x * SW_INV_PERIOD + 0.5) - 0.5) * SW_PERIOD;
}

// One field: three sines multiplied together, each one phase-warped by its
// neighbour. The product alone is a soft blob per cell of an oblique lattice
// and its zero set is three families of flat planes, which reads as a pattern;
// the warp bends those planes and the result reads as mottle. Zero mean,
// bounded by 1, measured variance 0.1247 (standard deviation 0.353) against a
// design value of 1/8 for three independent sines — see the port in
// tools/ui/check_surface_wear.cjs, which is where those numbers are asserted
// rather than assumed. The warp rate 1.7 is what sets SW_PERIOD: change it and
// the period must change with it, which the check enforces.
float swField(vec3 q) {
  vec3 s = sin(q + 0.85 * sin(q.yzx * 1.7 + 0.6));
  return s.x * s.y * s.z;
}

// SOFT SATURATION FOR A COVERAGE TERM, in place of clamp(x, 0.0, 1.0). A hard
// clamp is not free here: it has zero slope above 1, so wherever a layer's
// coverage overshoots, the mottle that was modulating it is DELETED and the
// area goes flat. Measured on the real tank, that was 26% of every dark
// up-facing surface — the tops of tracks and road wheels — rendered as one
// uniform maximum-dust field, which is exactly the uniform-panel failure this
// whole chunk exists to remove, reintroduced by the chunk itself. This curve is
// the identity to first order at small x (0.196 at 0.2), monotone, asymptotic
// to 1 without ever reaching it, and never has zero slope, so a coverage of 1.6
// still carries every bit of its variation. One inversesqrt.
float swSoft(float x) {
  float c = max(x, 0.0);
  return c * inversesqrt(1.0 + c * c);
}

// --------------------------------------------------------------- the paint
// A pure function of albedo, normal, model-space position and eye distance.
// It returns an albedo. It must not light anything: the host applies its own
// key, fill, rim, contact darkening and specular after this returns, and a
// second lighting model layered under the first is how a render starts looking
// like a photograph of a screen.
vec3 swWeather(vec3 albedo, vec3 N, vec3 P, float depth) {
  // FOUR DETAIL BANDS, FADED WHERE THEY WOULD GO SUB-PIXEL. Every card in this
  // page frames its model to fill the frame, so metres-per-pixel is roughly
  // depth * 0.0023 at a 200-pixel card and a 26-degree lens. A feature is worth
  // drawing down to about two pixels and no further. Each band therefore fades
  // out over the distance where its own feature size crosses five pixels down
  // to two: 10 cm grain by 22 m, 15 cm streaks by 26 m, 48 cm mottle by 100 m,
  // 77 cm mottle by 165 m. Nothing fades the gravity gradient, because that is
  // metres tall and is the only thing that still reads at forty pixels.
  //
  // WHAT THIS CANNOT DO, because it is the honest half of the same sentence:
  // depth is not pixel size. A map sprite and a catalogue card of the same tank
  // are drawn at the SAME distance, so these fades separate a frigate from a
  // tank and never separate a 64-pixel sprite from a 200-pixel card.
  float fFine = 1.0 - smoothstep(10.0, 22.0, depth);
  float fStreak = 1.0 - smoothstep(12.0, 26.0, depth);
  float fMid = 1.0 - smoothstep(48.0, 100.0, depth);
  float fCoarse = 1.0 - smoothstep(78.0, 165.0, depth);

  // MATERIAL, GUESSED FROM THE ONLY CHANNEL THERE IS. There is no material
  // buffer, so this reads luminance and saturation exactly as the host's
  // specular term already does. Dark and neutral is track, tyre and shadowed
  // rubber. Very bright is lens, lamp and white marking: it takes dirt at less
  // than half strength. Highly saturated is an insignia or a light: protected,
  // so that weathering never eats a thing that has to stay readable. The ramps
  // start high enough that a sand or winter repaint — which lands around 0.26
  // to 0.55 luminance — is treated as paint and weathers in full.
  float lum = dot(albedo, vec3(0.299, 0.587, 0.114));
  float hi = max(albedo.x, max(albedo.y, albedo.z));
  float lo = min(albedo.x, min(albedo.y, albedo.z));
  float sat = (hi - lo) / max(hi, 1e-4);
  float darkness = 1.0 - smoothstep(0.085, 0.175, lum);
  float shiny = smoothstep(0.52, 0.88, lum);
  float mark = smoothstep(0.42, 0.66, sat);

  // HEIGHT IS MEANT LITERALLY AND IN METRES. Y = 0 is the ground for every
  // vehicle, building and site here, and it is the WATERLINE for every hull,
  // which is the same statement about where the dirt goes. A wheel throws
  // spray to about 0.8 m whether it belongs to a tank or a truck, so the
  // splash band is absolute rather than a fraction of the model; the broad
  // band over the first 3.4 m is what makes a construction site grubby at the
  // bottom and clean at the top.
  float h = P.y;
  float splash = 1.0 - smoothstep(-0.20, 0.85, h);
  float lowly = 1.0 - smoothstep(0.15, 3.40, h);

  // N ARRIVES ALREADY FLIPPED TOWARD THE VIEWER, which for closed geometry is
  // the true outward normal and costs nothing. For the handful of open shells
  // in the deck — a rotodome, a solar sheet, a wing plate — the underside seen
  // from below reports as an up-face and collects dust. That is a real defect
  // and it comes from the host's contract, not from a choice made here.
  float up = N.y;
  float upFace = max(up, 0.0) * max(up, 0.0);
  float downFace = max(-up, 0.0);
  float vertFace = (1.0 - abs(up)) * (1.0 - abs(up));

  // Two coarse octaves at incommensurate frequencies (5.60 and 3.41, a ratio
  // of 1.642) so their sum does not repeat inside any model, plus a fine
  // octave on its own folded coordinate. The second coarse octave doubles as
  // the streak source and is therefore sampled 0.55 m higher up.
  vec3 b = swBasis(P);
  float m1 = swField(swWrap(b * 5.60));
  float m2 = swField(swWrap(b * 3.41 + SW_SOURCE_LIFT));
  float fine = swField(swWrap(b * 27.0 + 2.13));
  float streakN = swField(swWrap(vec3(
    P.x * 21.0 + P.z * 8.0,
    P.y * 3.0,
    P.z * 21.0 - P.x * 8.0)));
  float mottle = 0.80 * m1 * fMid + 0.62 * m2 * fCoarse;
  float blotch = clamp(0.5 + 0.5 * mottle, 0.0, 1.0);

  // 1. TOOTH. A multiplicative value jitter, and the only layer that is a pure
  // scaling rather than a mix toward a colour. That is deliberate: a mix is
  // invisible on a base that already IS the mix target, and after a sand
  // repaint a great deal of this deck sits close to the dust colour.
  //
  // The scaling is applied with its POSITIVE swing limited by the headroom the
  // channel actually has. A plain albedo *= 1.0 + tooth is the same thing for
  // any channel at or below 0.5, and every paint in the deck is — but the
  // winter repaint pushes bright trim to 1.0, and a multiply there can only run
  // off the top of the range and be clamped, which is a flat spot rather than
  // grain. Measured before this: a handful of fragments an IFV. After it, the
  // final clamp in this function provably never fires on real geometry, which
  // the check asserts on every vertex of a tank in all three finishes.
  float tooth = (0.130 * fine * fFine + 0.090 * mottle) * (1.0 - 0.45 * shiny);
  vec3 headroom = min(albedo, 1.0 - albedo);
  albedo += max(tooth, 0.0) * headroom + min(tooth, 0.0) * albedo;

  // 2. DUST settles on what faces the sky, more of it low down where wheels
  // throw it, and in patches rather than as a wash.
  //
  // THE DARKNESS TERM IS A RESTRAINT CAP AND IS NOT PHYSICS — say so plainly.
  // Dust does not know what it lands on, and it does not need to: mixing toward
  // a fixed light tan is already +14% on olive and would be +276% on the near
  // black rubber of a track, because the same absolute film is a far larger
  // RELATIVE change on a dark base. The first draft of this file multiplied
  // dust UP by darkness on the argument that dust reads hardest on black. It
  // does — automatically — and boosting it as well took a track from luminance
  // 0.041 to 0.156, which is not a dusty black tyre, it is a grey one, and it
  // moved the colour more than twice as far as the winter repaint does. So the
  // sign is the other way: the darker the base, the less coverage it takes, and
  // the treatment stays quieter than the finish selector on every material.
  // The enlarged tank exposes an upward-facing black grille lip inside a
  // strong dust patch. The former dark cap still doubled that lip's luminance
  // (0.04144 to 0.08438). Reduce the dark-surface allowance from 45% to 40%;
  // painted material above the darkness band keeps the same coverage. This
  // retains the smooth patch variation and does not clamp the finished colour.
  float dust = upFace * (0.55 + 0.45 * lowly) * (0.35 + 0.65 * blotch)
    * (1.0 - 0.60 * darkness) * (1.0 - 0.55 * mark);
  albedo = mix(albedo, SW_DUST, swSoft(dust) * 0.26);

  // 3. GRIME: the splash band, the undersides, and streaks. THE STREAKS ARE AN
  // APPROXIMATION AND HERE IS EXACTLY WHAT KIND. Real runoff needs to know
  // where water pools, which needs neighbour geometry this shader will never
  // have. What it does instead is sample a field stretched seven to one in Y —
  // 15 cm across, 105 cm tall — so it is vertically coherent by construction,
  // gate it to near-vertical faces so it only appears where water would run
  // rather than sit, and multiply it by that second coarse octave sampled
  // half a metre ABOVE the fragment, so a run hangs beneath a blotch instead of
  // starting nowhere. It is stretched noise with a source term. It is not a
  // simulation and it will not find the drip edge under a fender.
  float source = smoothstep(-0.05, 0.45, m2);
  float streak = smoothstep(0.04, 0.42, streakN) * source * vertFace * fStreak
    * (0.30 + 0.70 * lowly);
  float grime = (0.90 * splash * (0.25 + 0.75 * (1.0 - upFace))
    + 0.60 * downFace * lowly
    + 0.75 * streak) * (0.55 + 0.45 * blotch)
    * (1.0 - 0.45 * shiny) * (1.0 - 0.50 * mark);
  albedo = mix(albedo, SW_GRIME, swSoft(grime) * 0.42);

  // 4. RUBBED PAINT, AND THIS IS THE HONEST-APPROXIMATION ONE. There is no
  // curvature here and none can be had: the geometry is flat-shaded, so a
  // derivative of N is zero across a face and spikes at its border. What IS
  // available is that a flat panel in this deck points along an axis and an
  // edge does not, so the amount by which a normal's components exceed its own
  // largest component measures how diagonal it is: 0 on any axis-aligned face,
  // 0.71 on a two-way chamfer, 1.15 on a three-way corner. Weighted toward
  // up-facing, kept out of the mud zone, kept off rubber and off glass, and
  // broken up by noise at two scales so it lands as patches of thin paint
  // rather than as a clean stripe. WHAT IT GETS WRONG: a cylinder's normals
  // sweep through every diagonal, so a gun barrel or a road wheel picks up
  // bands of wear where a real one has none. That is why the amplitude is 0.24
  // and not 0.5, and why the blotch gate is there — a scatter of rubbed spots
  // along a barrel reads as use, where a clean band would read as a bug. The
  // product below is bounded by 1 term by term, so its clamp is documentation.
  vec3 an = abs(N);
  float diag = clamp((an.x + an.y + an.z - max(an.x, max(an.y, an.z)) - 0.36) * 1.7, 0.0, 1.0);
  float wear = diag * (0.12 + 0.88 * upFace) * (1.0 - splash)
    * (1.0 - darkness) * (1.0 - shiny)
    * smoothstep(-0.15, 0.55, 0.70 * m1 + 0.50 * fine);
  albedo = mix(albedo, clamp(vec3(lum * 1.45 + 0.03), 0.0, 1.0), clamp(wear, 0.0, 1.0) * 0.24);

  // A safety net that no longer catches anything: tooth is headroom-limited and
  // every other layer is a mix toward an in-range target, so the value here is
  // already legal. The check asserts that on real geometry rather than assuming
  // it, because "the clamp never fires" is a much stronger statement than "the
  // output is in range" and only one of them survives a future edit.
  return clamp(albedo, 0.0, 1.0);
}

// The host's entry point.
//   surface(albedo, N, P, V)
//     albedo: the interpolated vertex colour, linear, 0..1
//     N: unit normal, already flipped to face the viewer
//     P: position in MODEL space, metres, with Y=0 at ground contact
//     V: unit vector from P toward the eye
//   returns the albedo to shade with; it lights nothing.
// Everything host-specific is the one line that turns gl_FragCoord.w back into
// metres. V is deliberately unread: the only use for it here would be a
// grazing-angle sheen, and that is lighting — which this must not do — and it
// would make the weathering swim as a card turns, which is motion the
// reduced-motion setting could not stop.
vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V) {
  return swWeather(albedo, N, P, 1.0 / max(gl_FragCoord.w, 1e-4));
}
`,
    notes: [
      "Weathering and accumulation. Dust on up-faces, grime in a splash band at Y=0 and on",
      "undersides, stretched streaks running down near-vertical faces from a source sampled",
      "0.55 m above, and rubbed paint on diagonal normals. Model space, metres, Y=0 = ground",
      "(and = waterline for hulls). No clock, no derivatives, no samplers, no animation.",
      "BOUNDED BY MEASUREMENT, not by taste: over every vertex of a real 47,288-triangle tank in",
      "all three finishes, a fragment's luminance moves 5% at the median and 86% at the very",
      "worst, so nothing ever doubles — deliberately quieter than the finish selector itself,",
      "which moves the same vertices 64% (sand) and 122% (winter). The final clamp provably",
      "never fires, and no layer's coverage ever pins flat against a bound.",
      "READS ONE BUILT-IN: gl_FragCoord.w, whose reciprocal is eye-plane distance in metres",
      "under a perspective matrix; it drives four per-band distance fades. IT IS NOT PIXEL SIZE:",
      "arsenal3d frames a 64-pixel map sprite and a 200-pixel card at the SAME distance, so the",
      "fades separate a frigate from a tank and cannot separate a sprite from a card. Measured",
      "instead, by software-rasterising the real mesh through the real camera: at 64 px the",
      "treatment adds 0.006 of point-sampling error where the geometry itself already",
      "contributes 0.170, and it is static rather than shimmering because there is no clock.",
      "Under an orthographic matrix depth degrades to full detail rather than failing.",
      "Declares no uniforms. All helpers are namespaced sw*/SW_* so two proposals can be",
      "spliced into one page side by side. Colours are mixed toward absolute dust and grime",
      "targets, plus one purely multiplicative value layer, so the effect survives the sand and",
      "winter repaints on any base hue. COST, recounted after the fixes rather than guessed:",
      "about 190 GPU ALU instructions if vec3 work issues as one, about 430 scalar float ops if",
      "it does not, including 8 vec3 sines (24 scalar), 4 vec3 fracts, 2 inversesqrt and one",
      "divide — call it 505 ALU-equivalent cycles with sin and inversesqrt at quarter rate on a",
      "special-function unit. The four fixes added 22 scalar ops: 6 for the two soft saturations",
      "over the clamps they replaced, and 16 for the headroom-limited tooth. That is roughly five",
      "times the few dozen the brief suggested. It is still the right call because cards render",
      "once and cache as bitmaps: single-digit milliseconds on the 298 ms catalogue open, and",
      "under 0.2 ms a frame for the one card that spins. The cheapest cuts, in order, are the",
      "streak layer (about 60 scalar ops) and the fine octave (about 45).",
    ].join(" "),
  });
});
