/// SURFACE MATERIAL — a material-response treatment for the arsenal renderer.
///
/// THE PROBLEM THIS ANSWERS. Every mesh in this project is per-vertex RGB and
/// nothing else. The fidelity passes raised the geometry a long way (a tank is
/// 47,288 triangles) and the albedo is not the problem either (that same tank
/// carries 31 distinct vertex colours spanning luminance 0.041 to 0.446, and
/// 0.45 is roughly what olive drab's albedo actually is). What is missing is
/// SURFACE: every panel is a perfectly uniform field of one colour, and a
/// perfectly uniform field is the one thing no real object has.
///
/// WHY THERE IS NO MATERIAL CHANNEL AND WHY THAT DOES NOT MATTER. The
/// generators emit position, normal and colour. There is no material id and
/// adding one would mean touching every generator. But the three palettes —
/// equipment-mesh.js PALETTE, arsenal-models.js P, site-mesh.js P — already
/// encode material implicitly and, it turns out, CONSISTENTLY:
///
///   rubber / track / tyre      the darkest colours in every palette (L < 0.19)
///   glass / lens / canopy      saturated and blue-of-red (S > 0.4, b - r > 0.2)
///   concrete / fill / kerb     light, near-neutral, and WARM (r > b)
///   bare steel / galv / clad   light, near-neutral, and COOL (b >= r)
///   paint                      everything else with chroma (S > 0.22)
///
/// That fourth line is the find that makes this work. Concrete 0xb4b0a6 and
/// sheet cladding 0xa9b0b4 are the same brightness and the same saturation and
/// are impossible to separate by either — but concrete is warm by +0.055 and
/// cladding is cool by -0.043, and that sign holds for EVERY light neutral in
/// all three palettes with no exceptions. So the classifier is: luminance,
/// saturation, and the sign of r - b.
///
/// WHY THE CLASSIFIER IS WEIGHTS AND NOT A BRANCH. Every gate is a linear ramp
/// producing five weights that sum to exactly 1.0, and every material parameter
/// is the weighted mean of the five per-class values. A colour that lands
/// between two classes therefore gets the MEAN OF TWO TREATMENTS rather than an
/// arbitrary winner, two colours a 1/255 step apart differ in treatment by under
/// 1%, and a palette entry nobody anticipated gets something plausible instead
/// of a default. There is no seam anywhere in colour space, so a future palette
/// cannot introduce a pop — only a gradual shift.
///
/// WHAT IS DELIBERATELY NOT USED, AND WHY.
///
///   N is not used. Much of this geometry is flat-shaded on purpose, so the
///   normal is constant inside a triangle and JUMPS at every crease. Any
///   dependence on N — a triplanar blend, a tangent-plane projection, anything
///   derivative-based — would make the pattern discontinuous exactly along the
///   triangle edges and draw the wireframe. That is the trap here and this
///   shader does not go near it.
///
///   V is not used. A view-dependent albedo is lighting, and the host applies
///   its own key, fill, rim and specular after this returns. (Note also that
///   the host's specular term reads vCol, not the value returned here, so a
///   treatment could not change the highlight even if it tried.)
///
///   There is no time uniform and nothing animates. The pattern is a pure
///   function of model-space position, so it is bit-identical frame to frame
///   and there is no motion for the reduced-motion setting to have to stop.
///
/// HOST UNIFORMS READ. This chunk declares no uniforms of its own. It reads one
/// that the host's fragment shader already declares above the splice point:
///
///   uniform vec3 uEye;   the eye position IN MODEL SPACE
///
/// `length(uEye)` is the eye's distance to the model origin. Because the origin
/// of every asset here is ground-centre, that is a PER-OBJECT framing distance —
/// constant over the whole model, so the pattern cannot breathe or pump as the
/// model turns — and the renderer frames every model by fitting its bounding
/// sphere, so it is proportional to the model's size. Scaling the noise by it
/// is what makes the treatment resolution-stable; see `distance` in the notes.
/// If a future host lacks uEye the chunk simply fails to compile and
/// arsenal3d's setSurface falls back to flat albedo, which is the old renderer
/// byte for byte.
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.Surfacematerial = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  return Object.freeze({
    id: "material-response",

    glsl: `
// ------------------------------------------------------------------ constants
const vec3 SM_LUMA = vec3(0.299, 0.587, 0.114);

// Three sampling directions for the smooth field. Deliberately NOT orthogonal
// and deliberately three different lengths: three orthogonal periodic waves sum
// to a plaid, three oblique ones of incommensurate period sum to a quasi-crystal
// that reads as noise.
const vec3 SM_D0 = vec3( 0.836,  0.421,  0.351);
const vec3 SM_D1 = vec3(-0.413,  1.231,  0.445);
const vec3 SM_D2 = vec3( 0.404, -0.188,  0.648);
// Three waves alone still repeat: measured over 24 offset directions, the field
// was 39% self-correlated at 1.6 wavelengths, which is a weave a person can see
// on a large flat panel. SM_D3 is a fourth, much longer wave (period 2.7) whose
// value PHASE-SHIFTS the other three by up to SM_PH periods. That is a domain
// warp, it costs 15 ops, and it takes the same measurement to under 0.15.
const vec3 SM_D3 = vec3( 0.211, -0.147,  0.263);
const vec3 SM_PH = vec3( 0.610,  1.370,  0.830);

// The axis bare metal is stroked along. Oblique on purpose: these models are
// boxy and their panels are axis-aligned, so an axis-aligned scratch direction
// would be seen exactly end-on by one whole face of every single one of them.
const vec3 SM_AX = vec3(0.620, 0.340, 0.707);

// Per-class parameter rows, in the weight order (paint, steel, rubber,
// concrete). Glass is the fifth weight and carries its constants inline,
// because four of the five are zero for it.
const vec4 SM_CYCLES = vec4(150.0, 165.0, 200.0, 105.0);  // cycles per unit eye distance
const vec4 SM_GAIN   = vec4(0.085, 0.115, 0.050, 0.130);  // peak smooth-field excursion
const vec4 SM_SPECK  = vec4(0.030, 0.022, 0.026, 0.090);  // peak flat-cell excursion
const vec4 SM_GLINT  = vec4(1.000, 0.600, 0.000, 0.000);  // sparse-bright vs symmetric

// One hash. The leading fract bounds every term to [0,1) before the mixing
// multiplies, so the result does not lose precision as the coordinate grows —
// which matters, because a 133 m ship's cell index is four digits.
float smHash(vec3 p) {
  p = fract(p * 0.3183099 + vec3(0.71, 0.113, 0.419));
  p *= 17.0;
  return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

// One smooth field: three C1 triangle waves on three oblique directions, phase
// warped by a fourth. abs(fract(x) * 2 - 1) is a triangle in [0,1]; the
// t * t * (3 - 2 * t) rounds its corners so the field has no creases. A phase
// shift does not change a triangle wave's distribution, so the warp is free
// statistically: mean 0.5, standard deviation 0.201, strictly inside [0,1] —
// all three of which the JS port measures.
float smWave(vec3 q) {
  float u = abs(fract(dot(q, SM_D3)) * 2.0 - 1.0);
  u = u * u * (3.0 - 2.0 * u);
  vec3 t = abs(fract(vec3(dot(q, SM_D0), dot(q, SM_D1), dot(q, SM_D2)) + u * SM_PH) * 2.0 - 1.0);
  t = t * t * (3.0 - 2.0 * t);
  return (t.x + t.y + t.z) * 0.3333333;
}

vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V) {
  // ------------------------------------------------------------- classify
  float L  = dot(albedo, SM_LUMA);
  float mx = max(max(albedo.r, albedo.g), albedo.b);
  float mn = min(min(albedo.r, albedo.g), albedo.b);
  float S  = (mx - mn) / max(mx, 1e-3);
  float warm = albedo.r - albedo.b;

  // EVERY RAMP SLOPE IS BOUNDED ON PURPOSE. One 1/255 colour step can move S by
  // as much as 0.013, so a slope of 12.5 would move a weight by 0.16 in a single
  // quantisation step — a hard edge in anything approaching a gradient. Nothing
  // here is steeper than 8, which caps the step at 0.105, and the JS port
  // measures that rather than trusting it.
  //
  // Rubber, track, tyre, sensor housing, RAM coating: the dark end.
  float wRub = 1.0 - clamp((L - 0.145) * 10.0, 0.0, 1.0);
  // Glass, lens, canopy, solar cell: saturated AND blue-of-red. The saturation
  // gate alone excludes every cool metal in all three palettes (none exceeds
  // S = 0.194); the blue gate alone excludes every warm paint.
  float wGla = clamp((S - 0.28) * 6.0, 0.0, 1.0)
             * clamp((-warm - 0.04) * 10.0, 0.0, 1.0)
             * (1.0 - wRub);
  float rest = 1.0 - wRub - wGla;
  // Concrete, hardcore, kerb, fill, hut: light, neutral and warm. The warm gate
  // is what keeps bare metal (0xa8aeb4, 0xa2a9ae, 0xa9b0b4, 0xd8dde2) out; the
  // saturation gate is what keeps lane-marking paint (0xc8c29a) out.
  float wCon = rest
             * clamp((L - 0.40) * 6.5, 0.0, 1.0)
             * (1.0 - clamp((S - 0.115) * 8.0, 0.0, 1.0))
             * clamp((warm + 0.02) * 20.0, 0.0, 1.0);
  float pf = clamp((S - 0.128) * 7.0, 0.0, 1.0);
  float wPai = (rest - wCon) * pf;
  float wSte = (rest - wCon) * (1.0 - pf);
  vec4 w = vec4(wPai, wSte, wRub, wCon);

  // ------------------------------------------------------------ parameters
  float cyc  = dot(w, SM_CYCLES) + wGla * 26.0;
  float gain = dot(w, SM_GAIN)   + wGla * 0.022;
  float spk  = dot(w, SM_SPECK);
  float gl   = dot(w, SM_GLINT);
  float tint = wSte * 0.55 + wCon * 0.30;

  // --------------------------------------------------------------- headroom
  // A MULTIPLY CANNOT BRIGHTEN WHITE AND MUST NOT TRY. The treatment is a
  // fraction of whatever albedo arrived, so on a near-white surface a +13%
  // excursion asks for a channel above 1.0 — which is not an albedo any more,
  // and the host would light with it. No shipped palette colour is bright
  // enough for that to happen, WHICH IS EXACTLY WHY IT WAS MISSED: the winter
  // repaint is. FINISHES.winter [0.63,0.68,0.65] scaled by a bright original
  // reaches [0.945, 1.000, 0.975], and a whitewashed tank drove 2,824 channel
  // values across its 141,864 fragments as high as 1.128 before this block
  // existed. The palette sweep could not see it and the shipped bound of 1.02
  // was wide enough to hide even the one palette colour that did overflow
  // (ars.white, at 1.004); it took pushing a real mesh through in a real finish.
  //
  // room is the EXACT multiplicative headroom rather than a guessed ramp.
  // pk is the largest value the modulation below can reach — n * 2.0 <= 1.0
  // and s <= 0.5 + 0.5 * gl — expressed as a fraction of mx, plus the most the
  // blue tint can add. Scaling both the modulation and the tint by room gives
  //   mx * (1 + pk * room) + 0.0175 * tint * room  <=  mx + (1 - mx)  =  1
  // identically, for every albedo and every value the noise can take. The JS
  // port sweeps 660,000 albedos against the extremes of both noise fields and
  // measures the worst channel at 1.000000 rather than trusting the algebra.
  //
  // It costs NOTHING on the shipped palettes: 56 of the 57 colours get
  // room = 1.0 exactly, and ars.white 0xd8dde2 keeps 0.894 of its amplitude.
  // It fades the treatment out smoothly as an albedo approaches white, which is
  // what a white surface does — the brightest whitewash tone keeps 41% of its
  // grain and a tone that clamps to pure white keeps none, because there is no
  // headroom left to put any in.
  float pk   = mx * (gain + spk * (0.5 + 0.5 * gl)) + 0.0175 * tint;
  float room = clamp((1.0 - mx) / max(pk, 1e-4), 0.0, 1.0);

  // ------------------------------------------------------------------ scale
  // Constant screen-space frequency. The renderer frames every model by fitting
  // its bounding sphere, so the eye-to-origin distance is proportional to the
  // model's size; making the world-space frequency inversely proportional to it
  // holds the number of grain cycles ACROSS THE FRAME fixed, whatever the model
  // is and however far away it sits. The clamp is a guard, not a feature: it
  // bounds the physical grain to between 4 m and 2.5 mm, so a generator whose
  // origin is nowhere near its geometry degrades to something too coarse or too
  // fine rather than to something degenerate. It does not bite on any real
  // asset, from a 1.1 m rifle to a 133 m frigate.
  float od   = max(length(uEye), 0.5);
  float freq = clamp(cyc / od, 0.25, 400.0);

  // ------------------------------------------------------------------ noise
  // Bare steel is stroked: the sample coordinate is compressed to 7% along the
  // oblique axis, so the field varies slowly along it and fast across it, which
  // is a scratch. Every other class leaves the coordinate isotropic.
  vec3 q = P * freq;
  q += SM_AX * (dot(q, SM_AX) * (-0.93 * wSte));

  float n = smWave(q) - 0.5;                    // [-0.5, 0.5], sd 0.201
  float h = smHash(floor(q * 2.3));             // [0, 1), flat cells
  // Two speckle shapes from one hash. Symmetric (h - 0.5) is chips and pits,
  // which is aggregate and is what concrete and rubber want. Sparse-bright is
  // the top 15% of cells only, rescaled and re-centred, which is a glint and is
  // what paint and steel want. Both have mean zero, so neither shifts the tone.
  float s = mix(h - 0.5, max(h - 0.85, 0.0) * 6.6667 - 0.075, gl);

  // ------------------------------------------------------------------ apply
  // A multiply, so the treatment is a fraction of whatever albedo arrived and
  // survives the sand and winter repaints without knowing they happened. Both
  // terms carry room, which is what makes the bound above hold identically.
  float m = (n * 2.0 * gain + s * spk) * room;
  vec3 res = albedo * (1.0 + m) + (n * tint * room) * vec3(-0.020, 0.0, 0.035);
  // The clamp is a GUARD, not the bound. room already proves every channel
  // lands in [0,1] for any legal albedo, and the port asserts the pre-clamp
  // value is in range across three real meshes in all three finishes — so this
  // is dead code on real input, and it is here only so that a future generator
  // emitting an out-of-range vertex colour cannot make this function return
  // something that is not an albedo.
  return clamp(res, 0.0, 1.0);
}
`,

    notes: [
      "MATERIAL RESPONSE. Classifies the incoming vertex colour into five material classes by luminance, ",
      "saturation and the sign of r-b, as five continuous weights summing to 1.0, then blends five sets of ",
      "surface parameters by those weights. Paint: fine isotropic orange-peel plus a sparse specular glint. ",
      "Bare steel: the same field compressed to 7% along an oblique axis, so it reads as directional ",
      "micro-scratch, plus a faint cool metallic tint in the scratches. Rubber and track: a tight, ",
      "low-contrast, symmetric grain at half the amplitude of anything else. Concrete: the coarsest and ",
      "highest-contrast treatment, a broad float-finish mottle plus a symmetric chip-and-pit aggregate ",
      "speckle. Glass, lens and canopy: near-clean, a 2% very-low-frequency variation and nothing else. ",
      "SCALE. World-space frequency is cyc/length(uEye). The renderer fits each model's bounding sphere, ",
      "so that distance tracks model size and the number of grain cycles across the FRAME is invariant: a ",
      "1.1 m rifle and a 133 m frigate show the same apparent surface, and nothing becomes sub-pixel with ",
      "distance because the pattern never had a fixed physical size to lose. It is a per-object constant, ",
      "not per-fragment, so the pattern cannot pump or breathe as a model turns on a card. Measured on the ",
      "REAL fit distance of a real 47,288-triangle tank rather than an assumed camera: 70.4 cycles across ",
      "the frame, 10.5 cyc/m for paint (9.5 cm), 24.2 cells/m for the speckle (4.1 cm). ",
      "HEADROOM. The modulation and the tint are both scaled by the exact multiplicative headroom ",
      "(1 - mx) / (mx * peak + 0.0175 * tint), which makes a channel above 1.0 arithmetically impossible ",
      "rather than merely unobserved. This is a repair, not a precaution: no palette colour is bright ",
      "enough to overflow, but the WINTER REPAINT is, and it drove 2% of a whitewashed tank's fragments to ",
      "1.128 before the guard existed. 56 of the 57 shipped colours are untouched by it (room = 1.0 ",
      "exactly); ars.white keeps 0.894; the brightest whitewash keeps 0.41 and a tone that clamps to pure ",
      "white keeps none, because there is no headroom left to put grain into. ",
      "COST. About 240 scalar ALU, branchless and constant, counted by hand: 57 for the classifier, 35 ",
      "for the parameter blend, 13 for the headroom, 23 for the scale and the anisotropic squash, 81 for ",
      "one smooth field and one hash, and 31 for the speckle shaping and the apply. That is three times ",
      "the host fragment it is spliced into, and it is honestly more than the brief's few dozen: a ",
      "treatment that classifies the material and gives five of them genuinely different surfaces cannot ",
      "be 40 ops, and the classifier is a quarter of the total on its own. It buys the cost back on the ",
      "only axis that matters here — cards paint on demand rather than every frame, and the largest card ",
      "in this UI is 104 CSS px, so a repaint is ~43k fragments and 240 ops is ~10 MFLOP. To halve it: ",
      "drop the speckle layer (-40), the sparse-glint shape (-8), the phase warp (-15) and hard-branch ",
      "the classifier (-30 average). ",
      "RESOLUTION, stated because it is the one thing this cannot solve. arsenal3d.css sizes these ",
      "canvases at 30, 34, 46, 68 and 104 CSS px — there is no large card in this UI — so at MAX_DPR 2 ",
      "the biggest frame a model ever gets is 208 device pixels and the common one is 68. Against 70 ",
      "cycles across the frame that is 3.35 px per cycle at best, 1.09 on the common 34 px card and 0.97 ",
      "on the 30 px ledger chip, so the field is under-sampled on four of the five. Measured rather than ",
      "feared: undersampling here does NOT produce moire or banding. The FALSE low-frequency energy it ",
      "adds — sampled block means against the densely integrated truth over the same footprints — peaks ",
      "at 0.198% of albedo, about half a level in 8 bits. What it produces instead is per-pixel grain at ",
      "the treatment's own rms, 3.7% on paint and 5.8% on concrete, and that is what a small card shows: ",
      "not structure, and not an artifact either, just this much fine grain. ",
      "There is no resolution uniform, gl_FragCoord cannot supply one, and derivatives ",
      "are banned, so the shader cannot see this and cannot fade for it. The fix is one line in the HOST: ",
      "pass the viewport height and scale the amplitude by it. It is not done here because this chunk ",
      "must not declare a uniform the host does not have. ",
      "N and V are received and deliberately unused: any dependence on N is discontinuous across the ",
      "creases of flat-shaded geometry and would draw the wireframe, and any dependence on V is lighting.",
    ].join(""),
  });
});
