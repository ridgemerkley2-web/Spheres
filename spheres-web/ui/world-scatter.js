// Terrain-following SCATTER PLACEMENT. `scatter-mesh.js` builds the pieces —
// thirty kinds across nine biomes, trees, scrub, grass, rock. Nothing places
// them on a map. This does, and it does ONLY that: it answers "what stands
// here, facing which way, at what height", and it never builds a triangle,
// never touches a graphics context and never draws.
//
// WHY IT IS A SEPARATE FILE. The kit already has a `scatter()` that arranges a
// stand inside a flat local box in metres. That is the right shape for a page
// scene and the wrong shape for a map: a map is panned, and a box in metres
// re-laid on every pan is a different forest on every frame. This module speaks
// degrees, takes the terrain as CALLBACKS the host owns, and is built around
// one property the local scatter never needed.
//
// THE PROPERTY, and everything below is designed around it: an instance's
// position, kind, yaw and scale depend ONLY on which fixed world cell it falls
// in and on the seed. Never on the extent it was asked for. Pan the view a
// degree and every instance still on screen is byte-identical — same place,
// same species, same facing. That is what makes a landscape saveable as a seed,
// which is iron rule 1 restated for art. The world is gridded ONCE, globally,
// at a fixed cell size; a plan visits the cells that intersect the requested
// extent and emits from them. It is a lookup, not a layout.
//
// THE HOST OWNS THE DATA. Height, land, cover and the exclusion mask arrive as
// functions. This file does not load, decode or know about a raster, an image
// or a projection; it knows longitude, latitude and metres. Every sampler is
// optional and every missing one degrades to a stated default rather than
// throwing, because a plan that refuses to exist is worse on screen than a
// plan that says what it could not check.
//
// WHAT IT REFUSES TO PLANT. Water, by the host's own land test. Ground steeper
// than a stated limit, measured rather than guessed. Anything the host's
// exclusion callback claims — labels, city symbols, road and rail corridors,
// which are roadmap section G's list and not this file's invention. And
// everything, above a stated zoom-out threshold: section G's ladder puts relief
// and clean markers at world zoom, clusters at regional zoom and detail at city
// zoom, so scatter lives at the close end and returns NOTHING rather than a
// thinned sprinkle when the view is too far out to read it.
//
// THE BIOME IS DERIVED PRESENTATION, NOT SOURCED FACT, and this is the one
// claim in the file that has to be read carefully. Iron rule 4 says starting
// data is transcribed, not invented. A land-cover map is data; the table below
// is not one. It is a plausible classification of three signals the host
// already has — a vegetation index, latitude and elevation — onto the nine
// biome names the kit already owns, so that a palm does not appear in Finland.
// It grants no forest, no farmland, no timber, no forage and no habitat, it is
// not a land-cover measurement, and no simulation value may ever be read off
// it. Where the vegetation signal is absent it degrades to a latitude-and-
// elevation zonation, which is coarser still and is reported as such.
//
// NOT WIRED IN. Nothing in the game calls this yet. It is a placement engine
// and a contract; integration is somebody else's commit.
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.WorldScatter = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  // ------------------------------------------------------------------- hash
  // The only source of variety here, and deliberately the SAME one
  // scatter-mesh.js uses — Murmur3's finaliser with a stir in front of it — so
  // the placement kit and the geometry kit decorrelate small integers
  // identically and a cell index cannot accidentally correlate with a species.
  // There is no entropy source in this file and no wall clock: the same cell
  // and the same seed give the same answer on every machine forever.
  function mix32(x) {
    let h = x | 0;
    h = Math.imul(h ^ (h >>> 16), 0x7feb352d);
    h = Math.imul(h ^ (h >>> 15), 0x846ca68b);
    h ^= h >>> 16;
    return h >>> 0;
  }
  function seedOf(id) {
    if (typeof id === "number" && Number.isFinite(id)) return mix32(Math.round(id));
    let h = 0x811c9dc5;
    const s = String(id == null ? "" : id);
    for (let i = 0; i < s.length; i += 1) h = Math.imul(h ^ s.charCodeAt(i), 0x01000193);
    return mix32(h);
  }
  function ask(seed, salt) { return mix32((seed ^ Math.imul((salt | 0) + 1, 0x9e3779b1)) | 0); }
  function askUnit(seed, salt) { return ask(seed, salt) / 4294967296; }
  function askSpan(seed, salt, lo, hi) { return lo + (hi - lo) * askUnit(seed, salt); }

  // ------------------------------------------------------------------- grid
  /// CELL SIZE, and why this number. 1/256 of a degree: 0.00390625 deg, which
  /// is 432 m of latitude and 432*cos(lat) m of longitude. Three reasons for
  /// that value and not a rounder one.
  ///
  /// 1. It is a POWER-OF-TWO fraction of a whole degree, so the global grid is
  ///    92160 x 46080 cells exactly, a cell index is an exact integer at every
  ///    longitude, and there is no accumulated float drift to make cell 40000
  ///    disagree with itself between two calls.
  /// 2. 92160 = 2^11 * 45 and 46080 = 2^10 * 45, so every power-of-two STRIDE
  ///    up to 1024 divides the global grid. That matters at the antimeridian:
  ///    the coarsening lattice used at far zooms wraps cleanly at +-180 instead
  ///    of putting a seam of double or missing density down the Pacific.
  /// 3. 432 m is the scale of the decision being made. The question a cell
  ///    answers is "is this patch of ground plantable" — is it land, is it too
  ///    steep, what grows here — and all three are patch-scale questions. A
  ///    10 m cell would ask them of ground the host's elevation raster cannot
  ///    resolve; a 5 km cell would put a single Scots pine in the middle of a
  ///    valley and call it a wood.
  const PER = 256;
  const CELL = 1 / PER;
  const NX = 360 * PER;
  const NY = 180 * PER;

  /// Metres per degree. A degree of latitude is 110540 m (the mean meridional
  /// degree on the WGS84 ellipsoid, which runs from 110574 at the equator to
  /// 111694 at the pole; the mean is the honest constant for a slope estimate).
  /// A degree of longitude is 111320*cos(lat) m at the equatorial radius. These
  /// are used for exactly one thing each: converting the degree offsets of the
  /// slope probe into metres, and converting a piece's metre footprint into the
  /// degree radius the exclusion callback is asked about.
  const M_LAT = 110540;
  const M_LON = 111320;
  const DEG = Math.PI / 180;
  /// cos(lat) at the pole is zero and a metre-per-degree of zero is a division
  /// by nothing. Floored at 1e-3, which is |lat| = 89.943 deg — well inside the
  /// ice cap, where the land test refuses everything anyway.
  function lonMetres(lat) { return M_LON * Math.max(1e-3, Math.cos(lat * DEG)); }

  /// SLOPE LIMIT, 0.55 rise over run, which is 28.8 degrees. Stated rather than
  /// guessed, and the reasoning is three-fold.
  ///
  /// It is just below the angle of repose. Loose soil, scree and talus stand at
  /// 30-35 degrees and no steeper; ground past that is bare rock or moving, and
  /// a tree standing on it is a claim the ground cannot support.
  ///
  /// It is where a vertical instance stops sitting on the surface. Every piece
  /// the kit builds is seated at Y=0 and stood up straight — `finish` puts the
  /// lowest vertex exactly on the ground plane. On a 28.8-degree face, a piece
  /// with a 3.2 m footprint has its uphill edge 0.9 m into the hill and its
  /// downhill edge 0.9 m in the air. That is the visible limit; past it the
  /// scenery reads as floating, which section G names as the defect to avoid.
  ///
  /// And the kit already has the right answer for steeper ground: `cliff_face`
  /// is a tiling dressing kind built for exactly this. Refusing steep ground
  /// here hands it to the cliff system rather than planting on it.
  ///
  /// MEASURED AT THE CELL SCALE, not the micro scale. The probe is half a cell
  /// either side, so the baseline is one full cell — 432 m. This is a judgement
  /// about a patch of hillside, not about a boulder on it, and a host whose
  /// elevation raster is coarser than 432 m will read every slope as zero: pass
  /// `slopeSample` to widen the probe to at least one texel of the real raster
  /// or the steep-ground test silently becomes a no-op.
  const SLOPE_LIMIT = 0.55;
  const SLOPE_SAMPLE = CELL / 2;

  /// The zoom vocabulary is the globe's own: `Globe3D.ZOOM_MIN` is 1 and
  /// `ZOOM_MAX` is 192, and `ui.cam.k` rides the same scale.
  ///
  /// NOTHING BELOW 32. That is the rung `city-detail.js` already uses to swap a
  /// city's symbol for a skyline, so it is the codebase's own definition of
  /// "city zoom", and section G puts detail at city zoom and clean markers
  /// above it. Below 32 this returns an empty plan, not a thin one: a handful
  /// of trees scattered over a continent is noise on the map, and section G
  /// asks for relief and clean markers there instead.
  const MIN_ZOOM = 32;

  /// THE DENSITY LADDER, closed at both ends and a function of ZOOM ALONE.
  ///
  ///  zoom      stride  slots   cells/deg^2   candidates/deg^2
  ///  < 32      -       0       0             0        (empty plan)
  ///  32 .. 56  4       1       4096          4096
  ///  56 .. 96  2       2       16384         32768
  ///  >= 96     1       2       65536         131072
  ///
  /// `stride` coarsens the visited grid: only cells whose global index is a
  /// multiple of the stride on both axes are visited, so a far view costs a
  /// sixteenth of the cells rather than a full walk it throws away. `slots` is
  /// how many candidate positions a visited cell offers.
  ///
  /// THE POINT OF DOING IT THIS WAY. Both numbers depend on zoom and nothing
  /// else, and the stride lattice is GLOBAL, so panning never shifts which
  /// cells are on it. Zooming changes how many instances appear; it never moves
  /// one. An instance fades in where it always was rather than the field
  /// re-rolling, which is the difference between detail arriving and the
  /// landscape flickering.
  ///
  /// The band is closed above as well as below: past zoom 96 the density stops
  /// rising. One instance per ~300 m is the most this ever emits, because past
  /// that the budget is doing all the work anyway and a denser ladder would
  /// only mean culling more of it.
  const LADDER = [
    { zoom: 96, stride: 1, slots: 2, lod: 0 },
    { zoom: 56, stride: 2, slots: 2, lod: 1 },
    { zoom: MIN_ZOOM, stride: 4, slots: 1, lod: 1 },
  ];

  const DEFAULT_BUDGET = 3000;
  /// The most cells one plan will walk. Past this the stride doubles, which
  /// thins the field rather than refusing it — and it is derived from the
  /// EXTENT SPAN, which a pan does not change, so the thinning cannot happen
  /// mid-pan. Resizing the view can change it, exactly as zooming can.
  const MAX_CELLS = 200000;

  // -------------------------------------------------------------- vocabulary
  // MIRRORED FROM scatter-mesh.js, not invented beside it. These are that
  // module's own `BIOMES[b].mix` weights and its own `kindInfo(k).radius`
  // footprints, copied because this file must load with no dependency of any
  // kind — the browser global path has no loader — and pinned by
  // `tools/ui/check_world_scatter.cjs`, which asserts every name, every weight
  // and every radius against the live module. A rename or a reweight there
  // fails here, which is the whole point of copying rather than paraphrasing.
  const MIX = {
    temperate_broadleaf: [["oak", 30], ["hazel_scrub", 18], ["grass_tuft", 38], ["boulder", 9], ["rock_cluster", 5]],
    temperate_conifer: [["pine", 32], ["heather", 22], ["grass_tuft", 32], ["boulder", 10], ["rock_cluster", 4]],
    boreal: [["spruce", 42], ["heather", 20], ["sedge_tussock", 24], ["boulder", 10], ["rock_cluster", 4]],
    mediterranean: [["holm_oak", 22], ["maquis", 28], ["bunchgrass", 30], ["boulder", 10], ["rock_cluster", 10]],
    arid: [["saguaro", 10], ["creosote", 32], ["bunchgrass", 20], ["boulder", 14], ["rock_cluster", 12], ["scree_patch", 12]],
    tropical: [["palm", 26], ["tropical_thicket", 30], ["grass_tuft", 32], ["boulder", 8], ["rock_cluster", 4]],
    alpine: [["krummholz", 16], ["alpine_cushion", 24], ["sedge_tussock", 22], ["boulder", 16], ["rock_cluster", 12], ["scree_patch", 10]],
    tundra: [["dwarf_birch", 26], ["heather", 22], ["sedge_tussock", 30], ["boulder", 14], ["scree_patch", 8]],
    savanna: [["acacia", 16], ["thorn_scrub", 24], ["bunchgrass", 42], ["boulder", 10], ["rock_cluster", 8]],
  };
  /// Footprint in metres, used for one thing only: telling the exclusion
  /// callback how much room the instance wants, so a road corridor refuses an
  /// oak at three metres and a grass tuft at half of one.
  const RADIUS = {
    oak: 3.2, holm_oak: 3.2, acacia: 3.2, pine: 3.2, spruce: 3.2, krummholz: 3.2,
    palm: 3.2, saguaro: 3.2,
    hazel_scrub: 1.3, heather: 1.3, maquis: 1.3, creosote: 1.3, thorn_scrub: 1.3,
    tropical_thicket: 1.3, alpine_cushion: 1.3, dwarf_birch: 1.3,
    grass_tuft: 0.5, bunchgrass: 0.5, sedge_tussock: 0.5,
    boulder: 1.6, rock_cluster: 2.6, scree_patch: 3.0,
  };
  const BIOME_NAMES = Object.keys(MIX);
  const MIX_TOTAL = {};
  for (const b of BIOME_NAMES) {
    let t = 0;
    for (const row of MIX[b]) t += row[1];
    MIX_TOTAL[b] = t;
  }

  // ---------------------------------------------------------------- classify
  /// BIOME FROM THREE SIGNALS. First match wins, and the order is the whole of
  /// the rule — read it as a decision list, not as a set of independent tests.
  ///
  ///  #   test                                          biome
  ///  --  --------------------------------------------  -------------------
  ///  1   |lat| >= 66.5                                 tundra
  ///  2   elevation > treeline(lat)                     alpine
  ///  3   |lat| >= 55 and veg < 0.20                    tundra
  ///  4   veg < 0.12                                    arid
  ///  5   |lat| >= 55                                   boreal
  ///  6   |lat| < 23.5 and veg >= 0.50                  tropical
  ///  7   |lat| < 23.5                                  savanna
  ///  8   |lat| < 40 and veg < 0.40                     mediterranean
  ///  9   elevation >= 900 m, or |lat| >= 48            temperate_conifer
  ///  10  otherwise                                     temperate_broadleaf
  ///
  /// treeline(lat) = max(0, 4000 - 55*|lat|) metres. 4000 m in the tropics,
  /// 1800 m at 40 deg, sea level at 72.7 deg. That is the shape of the real
  /// treeline and roughly its magnitude; it is a curve chosen to look right,
  /// not a fitted one, and rule 2 firing is why Tibet reads alpine and the Alps
  /// do above the passes.
  ///
  /// THE POLAR TEST COMES FIRST, and the order is doing real work there. Past
  /// 72.7 degrees the treeline curve is at sea level, so an unordered version
  /// calls the whole Arctic coastal plain "alpine" — krummholz and scree on
  /// flat ground at 200 m. Asking the latitude first sends it to tundra, which
  /// is dwarf birch and sedge, and leaves alpine to mean what it says: ground
  /// above the local treeline, wherever that is.
  ///
  /// The latitude cuts are the ones an atlas already draws — 23.5 the tropic,
  /// 66.5 the polar circle, 55 the usual southern edge of the boreal forest,
  /// 48 the pine belt, 40 the northern edge of the mediterranean climates.
  /// The vegetation cuts are chosen so bare desert (near zero cover) leaves
  /// through rule 4, closed canopy leaves through rule 6, and the open-canopy
  /// middle lands on savanna or maquis by latitude.
  ///
  /// AND IT IS A GUESS, in the precise sense that matters. Three signals cannot
  /// separate a eucalypt forest from an oak wood, know nothing of rainfall
  /// seasonality, ocean currents, soil or fire, and will call the Pampas
  /// temperate broadleaf and the Atacama arid for the same reason. It exists so
  /// the scenery is not absurd, and it is presentation. Nothing in the
  /// simulation may read it.
  ///
  /// WITHOUT COVER. If the host has no vegetation sampler, `veg` is null and
  /// this uses VEG_UNKNOWN = 0.45 — a value chosen because it is the only band
  /// that leaves every latitude and elevation branch reachable while routing
  /// neither to arid nor to tropical, so the fallback degrades to a pure
  /// latitude-and-elevation zonation rather than to a single biome. It will
  /// call the Sahara mediterranean and the Amazon savanna. `plan` reports it in
  /// `assumed.vegetation` and says so in the description; do not ship a screen
  /// that leans on the biome without a cover raster behind it.
  const VEG_UNKNOWN = 0.45;
  function treeline(lat) {
    const t = 4000 - 55 * Math.abs(lat);
    return t > 0 ? t : 0;
  }
  function classify(veg, lat, elevation) {
    const y = Number.isFinite(lat) ? Math.abs(lat) : 0;
    const e = Number.isFinite(elevation) ? elevation : 0;
    const v = Number.isFinite(veg) ? (veg < 0 ? 0 : veg > 1 ? 1 : veg) : VEG_UNKNOWN;
    if (y >= 66.5) return "tundra";
    if (e > treeline(y)) return "alpine";
    if (y >= 55 && v < 0.20) return "tundra";
    if (v < 0.12) return "arid";
    if (y >= 55) return "boreal";
    if (y < 23.5) return v >= 0.50 ? "tropical" : "savanna";
    if (y < 40 && v < 0.40) return "mediterranean";
    if (e >= 900 || y >= 48) return "temperate_conifer";
    return "temperate_broadleaf";
  }

  // -------------------------------------------------------------- the ladder
  function rung(zoom) {
    const z = Number.isFinite(zoom) ? zoom : 0;
    for (let i = 0; i < LADDER.length; i += 1) if (z >= LADDER[i].zoom) return LADDER[i];
    return null;
  }

  // ----------------------------------------------------------------- helpers
  function fn(v) { return typeof v === "function" ? v : null; }
  function num(v, dflt) { return typeof v === "number" && Number.isFinite(v) ? v : dflt; }
  function clampLat(v) { return v < -90 ? -90 : v > 90 ? 90 : v; }
  /// Power-of-two ceiling, with a relative slack so a value that IS a power of
  /// two does not round up because of one ulp of float noise. This is what
  /// keeps the extent guard stable: two views of the same size, computed by
  /// different arithmetic, must land on the same step.
  function pow2ceil(x) {
    if (!(x > 1)) return 1;
    return Math.pow(2, Math.ceil(Math.log2(x) - 1e-9));
  }
  function cellSeedOf(base, gx, gy) {
    let h = mix32(base ^ Math.imul(gx | 0, 0x27d4eb2f));
    h = mix32(h ^ Math.imul(gy | 0, 0x165667b1));
    return h;
  }
  function pickKind(slotSeed, biome) {
    const mix = MIX[biome], total = MIX_TOTAL[biome];
    let roll = ask(slotSeed, 3) % total;
    for (let i = 0; i < mix.length; i += 1) {
      roll -= mix[i][1];
      if (roll < 0) return mix[i][0];
    }
    return mix[mix.length - 1][0];
  }

  function emptyPlan(reason, budget, zoom) {
    return {
      items: [], cells: 0, candidates: 0, budget, zoom,
      culled: { water: 0, slope: 0, excluded: 0, noHeight: 0, budget: 0 },
      stride: 0, slots: 0, lod: 1, cellSize: CELL,
      assumed: { land: false, height: false, vegetation: false },
      description: `No scatter: ${reason}. A placement plan, not a survey.`,
    };
  }

  // -------------------------------------------------------------------- plan
  /// plan(bounds, opts) -> { items, cells, culled, budget, ... }
  ///
  ///   bounds  { west, east, south, north } in degrees. west > east is read as
  ///           an extent crossing the antimeridian and is walked through the
  ///           seam; emitted longitudes are always normalised to [-180, 180),
  ///           because the world cell is the identity and it has one longitude.
  ///
  ///           EVERY CELL THAT INTERSECTS IS EMITTED WHOLE, so an item can lie
  ///           up to one cell — 0.0039 degrees, 432 m — outside the extent
  ///           asked for. That is deliberate and it is the alternative to
  ///           clipping: a clipped edge pops as the view moves, and a skirt one
  ///           cell wide is exactly what a host needs to draw to the edge of
  ///           the viewport without a bare margin. Size the extent to what is
  ///           drawn, not to what is clipped.
  ///
  ///   opts    zoom        the globe's own 1..192 scale. Below 32, empty.
  ///           budget      hard cap on returned items. Default 3000.
  ///           height      (lon, lat) -> metres, or null/NaN for "no data".
  ///                       Missing: every elevation is 0 and NO SLOPE TEST IS
  ///                       MADE, reported in `assumed.height`.
  ///           land        (lon, lat) -> bool. Missing: everything is land,
  ///                       reported in `assumed.land`. This is the water test
  ///                       section G asks for and it is the host's to answer.
  ///           vegetation  (lon, lat) -> 0..1. Missing: VEG_UNKNOWN, reported.
  ///           exclude     ([lon, lat], radiusDeg) -> bool, true to refuse.
  ///                       Labels, city symbols, transport corridors. Asked
  ///                       last, with the instance's own scaled footprint
  ///                       converted to degrees by the SMALLER of the two axes'
  ///                       metres-per-degree, so the radius is never understated.
  ///           seed        anything; hashed. Default 0.
  ///           slopeLimit  rise/run, default 0.55.
  ///           slopeSample probe half-offset in degrees, default half a cell.
  ///           lod         0 near / 1 far, overriding the ladder's choice.
  ///           maxCells    walk guard, default 200000.
  ///
  /// Each item: { lon, lat, elevation, kind, biome, yaw, scale, lod }, plus
  /// `seed` (feed it to `ScatterMesh.piece` for a stable specimen) and the pair
  /// `cell` ("gx:gy", the world cell) and `slot` (which of that cell's
  /// positions this is). `cell` + `slot` is the instance's IDENTITY: two plans
  /// that both contain it must agree on every other field, and that is the
  /// pan-stability contract stated as something a check can hold.
  ///
  /// THE BUDGET IS HARD and the cull is STABLE. Every candidate carries a rank,
  /// which is a hash of its own cell and slot and of nothing else. Candidates
  /// are taken in rank order and the walk stops when the budget is full, so
  /// halving the budget returns a strict PREFIX of the same instances rather
  /// than a different draw, and the survivors do not change as the view moves.
  /// `culled.budget` counts the candidates never reached, which is a count of
  /// what was dropped and not a claim about how many would have passed.
  function plan(bounds, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const b = bounds && typeof bounds === "object" ? bounds : {};
    const zoom = num(o.zoom, 0);
    const budget = Math.max(0, Math.round(num(o.budget, DEFAULT_BUDGET)));
    const step = rung(zoom);
    if (!step) return emptyPlan(`zoom ${zoom} is below the ${MIN_ZOOM} threshold for terrain detail`, budget, zoom);
    if (budget === 0) return emptyPlan("the budget is zero", budget, zoom);

    const west = num(b.west, NaN), east = num(b.east, NaN);
    let south = num(b.south, NaN), north = num(b.north, NaN);
    if (![west, east, south, north].every(Number.isFinite)) {
      return emptyPlan("the extent is not four finite degrees", budget, zoom);
    }
    if (south > north) { const t = south; south = north; north = t; }
    south = clampLat(south); north = clampLat(north);
    let spanLon = east - west;
    if (!(spanLon > 0)) spanLon += 360;
    if (spanLon > 360) spanLon = 360;
    const spanLat = north - south;
    if (!(spanLat > 0) || !(spanLon > 0)) return emptyPlan("the extent has no area", budget, zoom);

    // The walk guard. Derived from the SPAN, quantised up to a power of two so
    // a fractional pan — which changes the cell count by one at each edge —
    // cannot move it, and so the same-sized view always thins the same way.
    let stride = step.stride;
    const guardCells = pow2ceil(spanLon * PER) * pow2ceil(spanLat * PER);
    const maxCells = Math.max(1, Math.round(num(o.maxCells, MAX_CELLS)));
    while (guardCells / (stride * stride) > maxCells) stride *= 2;

    const base = seedOf(o.seed == null ? 0 : o.seed);
    const height = fn(o.height), land = fn(o.land);
    const cover = fn(o.vegetation), exclude = fn(o.exclude);
    const slopeLimit = Math.abs(num(o.slopeLimit, SLOPE_LIMIT));
    const probe = Math.abs(num(o.slopeSample, SLOPE_SAMPLE));
    const lod = o.lod === 0 || o.lod === 1 ? o.lod : step.lod;

    // ------------------------------------------------------------ candidates
    // One pass over the visited cells, collecting nothing but a rank and an
    // identity. Terrain is NOT sampled here: sampling is the expensive part and
    // most candidates never survive the budget, so it is deferred until a
    // candidate is actually reached in rank order.
    //
    // Ranks are bucketed by their top ten bits rather than sorted whole. The
    // bucket index is monotone in the rank, so walking buckets in order is
    // walking ranks in order, and only the buckets the budget actually consumes
    // ever get sorted. A view offering a hundred thousand candidates for a
    // three thousand item budget sorts about three percent of them.
    const BUCKETS = 1024;
    const buckets = new Array(BUCKETS);
    const i0 = Math.floor((west + 180) * PER);
    const i1 = Math.floor((west + spanLon + 180) * PER);
    const j0 = Math.max(0, Math.floor((south + 90) * PER));
    const j1 = Math.min(NY - 1, Math.floor((north + 90) * PER));
    let cells = 0, candidates = 0;
    for (let j = j0; j <= j1; j += 1) {
      if (j % stride !== 0) continue;
      // Area correction. An equal-ANGLE cell is not an equal-AREA cell: at 60
      // degrees a cell is half as wide in metres as it is at the equator, so an
      // uncorrected grid doubles the trees per square kilometre in Scandinavia.
      // The slot count is scaled by cos(lat) and the fractional part is spent
      // as a per-cell hash, so the correction is exact on average and still
      // depends only on the cell.
      const latC = (j + 0.5) * CELL - 90;
      const want = step.slots * Math.max(0, Math.cos(latC * DEG));
      const whole = Math.floor(want), frac = want - whole;
      for (let i = i0; i <= i1; i += 1) {
        const gx = ((i % NX) + NX) % NX;
        if (gx % stride !== 0) continue;
        cells += 1;
        const cellSeed = cellSeedOf(base, gx, j);
        const slots = whole + (askUnit(cellSeed, 7) < frac ? 1 : 0);
        for (let s = 0; s < slots; s += 1) {
          const rank = ask(mix32(cellSeed ^ Math.imul(s + 1, 0x9e3779b1)), 0);
          const k = rank >>> 22;
          const bucket = buckets[k] || (buckets[k] = []);
          bucket.push(rank, gx, j, s);
          candidates += 1;
        }
      }
    }

    // -------------------------------------------------------------- decision
    const items = [];
    const culled = { water: 0, slope: 0, excluded: 0, noHeight: 0, budget: 0 };
    let reached = 0;
    for (let k = 0; k < BUCKETS && items.length < budget; k += 1) {
      const bucket = buckets[k];
      if (!bucket) continue;
      const n = bucket.length / 4;
      const order = new Array(n);
      for (let q = 0; q < n; q += 1) order[q] = q * 4;
      // A total order, not merely a rank order: two candidates sharing a rank
      // must still resolve the same way in every plan that contains both.
      order.sort((p, q) => (bucket[p] - bucket[q]) || (bucket[p + 1] - bucket[q + 1])
        || (bucket[p + 2] - bucket[q + 2]) || (bucket[p + 3] - bucket[q + 3]));
      for (let q = 0; q < n; q += 1) {
        if (items.length >= budget) break;
        const at = order[q];
        reached += 1;
        const gx = bucket[at + 1], gy = bucket[at + 2], s = bucket[at + 3];
        const slotSeed = mix32(cellSeedOf(base, gx, gy) ^ Math.imul(s + 1, 0x9e3779b1));
        // Jitter is inset from the cell edge, so two instances in neighbouring
        // cells cannot land on the same metre across the boundary.
        const lon = (gx + askSpan(slotSeed, 1, 0.06, 0.94)) * CELL - 180;
        const lat = (gy + askSpan(slotSeed, 2, 0.06, 0.94)) * CELL - 90;

        if (land && !land(lon, lat)) { culled.water += 1; continue; }

        let elevation = 0;
        if (height) {
          const h = height(lon, lat);
          if (!Number.isFinite(h)) { culled.noHeight += 1; continue; }
          elevation = h;
          const hE = height(lon + probe, lat), hW = height(lon - probe, lat);
          const hN = height(lon, clampLat(lat + probe)), hS = height(lon, clampLat(lat - probe));
          if (Number.isFinite(hE) && Number.isFinite(hW) && Number.isFinite(hN) && Number.isFinite(hS)) {
            const dx = (hE - hW) / (2 * probe * lonMetres(lat));
            const dy = (hN - hS) / (2 * probe * M_LAT);
            if (Math.hypot(dx, dy) > slopeLimit) { culled.slope += 1; continue; }
          }
        }

        const veg = cover ? cover(lon, lat) : null;
        const biome = classify(Number.isFinite(veg) ? veg : null, lat, elevation);
        const kind = pickKind(slotSeed, biome);
        const scale = askSpan(slotSeed, 5, 0.72, 1.34);
        const yaw = askUnit(slotSeed, 4) * Math.PI * 2;

        if (exclude) {
          const metres = (RADIUS[kind] || 1) * scale;
          const wide = metres / Math.min(M_LAT, lonMetres(lat));
          if (exclude([lon, lat], wide)) { culled.excluded += 1; continue; }
        }

        items.push({
          lon, lat, elevation, kind, biome, yaw, scale, lod,
          seed: slotSeed, cell: gx + ":" + gy, slot: s,
        });
      }
    }
    culled.budget = candidates - reached;

    const assumed = { land: !land, height: !height, vegetation: !cover };
    const refused = [];
    if (culled.water) refused.push(`${culled.water} in water`);
    if (culled.slope) refused.push(`${culled.slope} on ground steeper than ${slopeLimit}`);
    if (culled.noHeight) refused.push(`${culled.noHeight} with no elevation`);
    if (culled.excluded) refused.push(`${culled.excluded} refused by the host`);
    if (culled.budget) refused.push(`${culled.budget} beyond the ${budget} budget`);
    const caveat = assumed.vegetation
      ? "Vegetation cover was unavailable, so the biome is a latitude and elevation zonation only. "
      : "";
    return {
      items, cells, candidates, budget, zoom, stride, slots: step.slots, lod,
      cellSize: CELL, culled, assumed,
      description: `${items.length} scatter instances over ${cells} world cells of `
        + `${CELL} degrees, seeded and stable under pan`
        + (refused.length ? `; refused ${refused.join(", ")}` : "")
        + `. ${caveat}A placement plan of representative scenery: the biome is derived `
        + `from cover, latitude and elevation and is presentation, not a land-cover `
        + `measurement, and it grants no forest, farmland, timber or forage.`,
    };
  }

  // --------------------------------------------------------------------- api
  function kinds() {
    const seen = {};
    for (const b of BIOME_NAMES) for (const row of MIX[b]) seen[row[0]] = true;
    return Object.keys(seen);
  }
  function biomes() { return BIOME_NAMES.slice(); }
  function mixFor(biome) {
    return Object.prototype.hasOwnProperty.call(MIX, biome)
      ? MIX[biome].map((row) => row.slice()) : null;
  }
  function radiusFor(kind) {
    return Object.prototype.hasOwnProperty.call(RADIUS, kind) ? RADIUS[kind] : null;
  }
  /// The slope a plan would measure at one point, in rise over run, exposed so
  /// a host can debug its own elevation sampler against the same arithmetic
  /// the exclusion uses. Null when the sampler has no data there.
  function slopeAt(height, lon, lat, offset) {
    if (typeof height !== "function") return null;
    const probe = Math.abs(num(offset, SLOPE_SAMPLE));
    const hE = height(lon + probe, lat), hW = height(lon - probe, lat);
    const hN = height(lon, clampLat(lat + probe)), hS = height(lon, clampLat(lat - probe));
    if (![hE, hW, hN, hS].every(Number.isFinite)) return null;
    const dx = (hE - hW) / (2 * probe * lonMetres(lat));
    const dy = (hN - hS) / (2 * probe * M_LAT);
    return Math.hypot(dx, dy);
  }

  return {
    plan, classify, rung, kinds, biomes, mixFor, radiusFor, slopeAt, treeline,
    metresPerDegree: (lat) => ({ lon: lonMetres(lat), lat: M_LAT }),
    cellsPerDegree: PER,
    cellSize: CELL,
    gridSize: [NX, NY],
    minZoom: MIN_ZOOM,
    slopeLimit: SLOPE_LIMIT,
    slopeSample: SLOPE_SAMPLE,
    defaultBudget: DEFAULT_BUDGET,
    maxCells: MAX_CELLS,
    unknownVegetation: VEG_UNKNOWN,
    ladder: LADDER.map((r) => ({ zoom: r.zoom, stride: r.stride, slots: r.slots, lod: r.lod })),
  };
});
