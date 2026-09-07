// BIRDS-EYE CITY MASSING. One mesh per city record, built from the Natural
// Earth 1:50m fields the game already ships in `cities.js`. Asset row
// `urban.city_massing.v1`.
//
// WHY THIS EXISTS, and why it is not town-mesh.js. The city card today shows
// ONE representative town block — 148 m x 104 m of brick, slate and parked
// cars — picked by hashing the city name into eight layouts. It is good
// geometry and it answers the wrong question: Tokyo and a market town of
// ninety thousand get the same block. What a card beside the name "Tokyo"
// should show is TOKYO'S SHAPE: how far it runs, where it is dense, where it
// stops at the sea.
//
// WHY A BLOCK CANNOT BE SCALED UP INTO A CITY, measured rather than asserted.
// A million-person city under the area model below is 40 km2 of built-up
// ground. That is 2,599 of town-mesh's blocks. At the 198,462 triangles one
// block costs at close detail that is 515 MILLION triangles, and 8.9 million
// at the block's own map detail. Neither number is a rendering budget; the
// second is not even a loading budget. The primitive has to change: not a
// building, not a block, but a CELL OF CITY about a hundred metres across
// carrying one mass. The same million-person city is a 71 x 71 grid of 100.5 m
// cells, 3,764 of them inside its boundary and 1,407 carrying a mass, and it
// measures 24,740 triangles — an eighth of the cost of the single block it
// replaces, showing the entire city instead of one street corner.
//
// SO THE HONEST CLAIM IS: this file draws the FORM of a city — extent,
// density, height falloff, street grid, parks, coastline — and never a
// building. Nothing here is a survey. Read `description` on any mesh it
// returns; it says the same thing to the player.
//
// WHAT IS SOURCED AND WHAT IS MODELLED. This distinction is the whole of the
// file's honesty and iron rule 4 is why.
//
//   SOURCED, from the record: the name, the position, the population, the
//   scale rank and the national-capital flag. Position drives every sampler
//   query, so where the city sits on the planet, what relief it sits on and
//   where its coastline runs are real to the extent the host's samplers are.
//
//   MODELLED, and stated as such everywhere it is reported: how much ground a
//   population occupies (the area model below), how tall its core is, how its
//   density falls off, where its arterials, ring road, parks and towers are.
//   None of it is this city's plan. A player must be able to read the caption
//   and know that the grid under the label "Kyoto" is generic urban fabric.
//
//   INVENTED NOWHERE: no district names, no landmarks by name, no claim about
//   any real street, and no simulation capability of any kind.
//
// THE AREA MODEL, declared once, here, and labelled representative wherever it
// is reported:
//
//     A(P) = 40 * (P / 1,000,000) ^ 0.85   square kilometres of built-up land
//
// The exponent is sublinear because large cities are denser than small ones,
// which is the one robust thing about settlement scaling. The coefficient
// anchors a million people at 40 km2 — 25,000 people per built km2. THAT IS A
// DENSE ANCHOR and it should be read as one: it is Manila or Dhaka, roughly
// three times the density of a European urban area and five times a North
// American one, where the same million people would sprawl over 120-200 km2.
// The dense anchor is a deliberate engineering choice and not a claim about
// cities: at 150 km2 a million-person city is 14 km across, Tokyo is 63 km
// across, and at the triangle budget below the massing cell for Tokyo would be
// 570 m — a superblock the size of a district, which stops reading as a city
// at all. The model is tuned to the thing being drawn. It is not a source.
//
// THE MODEL IS EVALUATED FROM A TABLE, NOT FROM Math.pow. `AREA_KM2` below is
// the closed form evaluated at eleven half-decade anchors and linearly
// interpolated between them. Two reasons. One, determinism: Math.pow with a
// fractional exponent is implementation-defined in the last place, and this
// file must produce byte-identical buffers on every machine forever. Two, it
// puts the model on the page as numbers a reader can check against the formula
// without running anything. Maximum departure from the closed form across the
// whole range is 2.27%, measured, at a population of 5,211 — the midpoint of
// the widest-spanning anchor pair in log terms. That is noise against a model
// that is itself representative, and the check re-measures it rather than
// taking this sentence on trust.
//
// NO TRANSCENDENTAL ARITHMETIC ANYWHERE ELSE EITHER, for the same reason. No
// pow, no exp, no log, no sin, no cos in any decision or any vertex. Falloffs
// are polynomials, the boundary noise is a hashed lattice with a smoothstep,
// and the only irrational operation in the file is Math.sqrt, which IEEE-754
// requires to be correctly rounded. The single exception is `Math.cos` in the
// metres-per-degree of longitude — it converts a metre offset into the degrees
// a sampler is asked about, it is the same constant world-scatter.js uses, and
// it can move a sampler query by a fraction of a micrometre without moving a
// vertex, because every derived length is quantised before it is used.
//
// MODEL SPACE. Metres. +X east, +Y up, +Z SOUTH — right-handed, since
// east x up = south. The city core is the origin and Y=0 is the lowest ground
// in the mesh, so north is -Z and a birds-eye camera looking down -Y with +Z
// down the screen sees the map the right way up. `bounds.min[1] === 0` always.
//
// DETERMINISM. No clock, no entropy source, no DOM, no globals. Every choice a
// cell makes is an integer hash of (city seed, cell, a salt naming the
// decision), so the same record builds the same city on every machine forever
// and a city is saveable as its record.
//
// THE SAMPLERS ARE THE ONE GENUINELY SOURCED PART. `opts.height(lon, lat)` and
// `opts.land(lon, lat)` are injected by the host exactly as world-scatter.js
// takes them, which is what keeps this module DOM-free and testable: it never
// loads a raster, never decodes an image and never knows a projection. With
// them a coastal city stops at its coastline and a hill city sits on its hill.
// Without them the ground is flat and everything is land, and the description
// SAYS SO rather than quietly pretending — the `assumed` block is the same
// discipline world-scatter.js uses.
//
// TRIANGLE BUDGET, declared with its derivation and then measured against
// every one of the 1,249 records in cities.js.
//
//   close   40,000 - 120,000 for the largest cities; 120,000 is a hard
//           ceiling for any city at all
//   map     250 - 6,000 for any city at all
//
// WHERE THE NUMBERS COME FROM. The grid is capped at 141 cells across at close
// detail, so at most 19,881 cells exist. Every cell inside the boundary costs
// a 2-triangle ground quad and about 70% of the square is inside the lobed
// boundary, so ground is at most ~28,000 triangles. Of those cells, 16-33% are
// arterial and 8-20% are park, and between a third and nine tenths of what is
// left carries a mass. A mass is 10 triangles — four walls and a flat roof, no
// underside, because nothing in a birds-eye can see one — 14 with a pitched
// roof at close detail and 20 with a tower setback. That is 4,600-6,200 masses
// and 50,000-70,000 triangles of massing for the biggest cities.
//
// MEASURED, over all 1,249 records, both levels:
//   close  max 99,008 (Toronto, 141 x 141 at 101 m), min 1,984 (Andorra),
//          median 12,638; every city over two million lands between 43,204
//          (Sanaa) and 99,008. The 120,000 ceiling therefore carries 21% of
//          headroom over the worst real case.
//   map    max 3,988 (San Francisco), min 392 (Mazar-e Sharif), median 760.
//
// ABOVE THREE AND A HALF MILLION PEOPLE THE TRIANGLE COUNT STOPS GROWING and
// the cell grows instead: the grid is pinned at 141 and Tokyo is drawn in
// 230 m superblocks where Toronto gets 101 m blocks. Tokyo is still four times
// wider on the ground. That is the trade this file makes deliberately — the
// EXTENT is the thing the model exists to show, and resolution is what pays
// for it.
//
// THE COMPARISON THAT MATTERS: today's single town block is 198,462 triangles
// at close detail and 3,422 at map. An entire city here is 99,008 at worst —
// half the cost of the one block it replaces — and its map level is the same
// order as the block's while showing a city rather than a street corner.
//
// NOT WIRED IN. Nothing calls this yet; the card is somebody else's commit.
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.CityMesh = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  // ------------------------------------------------------------------- hash
  // Deliberately the same hash town-mesh.js and world-scatter.js use —
  // Murmur3's finaliser with a stir in front of it — so a city, the town block
  // and the scatter around it decorrelate small integers identically. There is
  // no entropy source in this file and no wall clock.
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
  /// A cell's own seed. Mixed from both indices so that (3, 7) and (7, 3) are
  /// unrelated, which a plain xor would not give.
  function cellSeed(seed, i, j) {
    return mix32((seed ^ Math.imul(i | 0, 0x27d4eb2f) ^ Math.imul(j | 0, 0x165667b1)) | 0);
  }

  /// Smooth value noise on a hashed lattice. `period` is in cells, so a period
  /// of 4 makes lobes about four cells wide whatever the cell is worth in
  /// metres — the boundary looks the same shape on a hamlet and on a
  /// metropolis, which is what makes the silhouette read as a city rather than
  /// as a resolution. Smoothstep rather than cosine interpolation: same curve
  /// to the eye, exact arithmetic, no transcendental.
  function noise(seed, i, j, period, salt) {
    const fx = i / period, fy = j / period;
    const ix = Math.floor(fx), iy = Math.floor(fy);
    const tx = fx - ix, ty = fy - iy;
    const sx = tx * tx * (3 - 2 * tx), sy = ty * ty * (3 - 2 * ty);
    const at = (gx, gy) => askUnit(cellSeed(seed, gx, gy), salt) * 2 - 1;
    const a = at(ix, iy), b = at(ix + 1, iy), c = at(ix, iy + 1), d = at(ix + 1, iy + 1);
    const top = a + (b - a) * sx, bot = c + (d - c) * sx;
    return top + (bot - top) * sy;
  }

  // -------------------------------------------------------------- constants
  /// Metres per degree, the same two constants world-scatter.js uses and for
  /// the same reason: 110540 is the mean meridional degree on WGS84, 111320 is
  /// a degree of longitude at the equatorial radius. They convert a cell's
  /// metre offset into the degrees a sampler is asked about, and nothing else.
  const M_LAT = 110540;
  const M_LON = 111320;
  const DEG = Math.PI / 180;
  function lonMetres(lat) { return M_LON * Math.max(1e-3, Math.cos(lat * DEG)); }

  /// One sun, fixed, high and to the right — the same vector and the same
  /// ambient/diffuse split town-mesh.js bakes, so a city and a town block sit
  /// under one sun and can share a card without disagreeing about the time of
  /// day. Lighting is baked into the vertex colour because the renderer this
  /// feeds is the flat vertex-colour path the rest of the game uses.
  const SUN = (function () {
    const v = [0.42, 0.80, 0.40], n = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    return [v[0] / n, v[1] / n, v[2] / n];
  })();
  const AMBIENT = 0.52, DIFFUSE = 0.58;

  /// THE AREA MODEL AS A TABLE. `A(P) = 40 * (P/1e6)^0.85` km2, evaluated at
  /// half-decade anchors. Reproduced to four decimals so a reader can check
  /// the arithmetic; interpolated linearly in population between anchors and
  /// clamped at both ends.
  const AREA_KM2 = [
    [1e3, 0.1127], [3e3, 0.2867], [1e4, 0.7981], [3e4, 2.0305], [1e5, 5.6502],
    [3e5, 14.3752], [1e6, 40.0000], [3e6, 101.7684], [1e7, 283.1783],
    [3e7, 720.4653], [1e8, 2004.7489],
  ];
  /// CORE HEIGHT, metres, against population. Also a table and also
  /// representative. Its shape is the one claim it makes: a hamlet is two
  /// storeys, a provincial city is six to ten, a metropolis has a core of
  /// towers. The anchors are round numbers chosen to read, not measured
  /// building heights, and no real skyline was consulted.
  const PEAK_M = [
    [1e3, 8], [1e4, 11], [5e4, 16], [1e5, 21], [4e5, 34],
    [1e6, 55], [3e6, 92], [1e7, 150], [4e7, 250],
  ];
  /// A record with no population at all — Natural Earth carries one, and a
  /// handful of research stations under a hundred — still has to draw
  /// something. The model floors at a thousand and the description says the
  /// record's own number, so the floor is visible rather than silent.
  const POP_FLOOR = 1000;
  const POP_CEIL = 5e7;

  /// THE CELL, and why it is not a constant 100 m. A cell is the massing
  /// primitive: one mass, one patch of ground. 100 m is the right size for it
  /// — it is a city block plus its streets, the scale at which "is this built,
  /// how tall, is it a road" are real questions — and for any city between
  /// about nine thousand and three and a half million people that is within a
  /// couple of metres of what it is, because BASE_CELL is what the span is
  /// derived from. Outside that range the cell moves, in both directions, and
  /// the reasons are different at each end.
  ///
  /// LARGE: a 32.4 km city at 100 m is 324 cells across, 105,000 cells and
  /// somewhere near half a million triangles. Something has to give and it can
  /// only be the resolution: the extent is the thing the model exists to show,
  /// so the cell grows and Tokyo is drawn in 230 m superblocks. That is what a
  /// birds-eye of a metropolis is anyway — at that zoom a block is not
  /// resolvable, and a 230 m cell is a superblock, which is a real unit of a
  /// real city.
  ///
  /// SMALL: a 1 km town at 100 m is ten cells across, which is not a town, it
  /// is a smudge. The cell shrinks to keep at least 25 cells across, so a town
  /// of ten thousand is drawn at 40.5 m — a single plot with its lane — and
  /// reads as streets.
  const BASE_CELL = { close: 100, map: 400 };
  /// THE CLOSE CEILING IS A LEGIBILITY BAR, NOT A COST ONE, and it was set at
  /// 141 where a big city stops reading. The card is 252 device pixels wide, so
  /// the span IS the pixels per cell: at 141 a cell is 1.8 px and Tokyo, Mumbai
  /// and Hong Kong alias into uniform speckle, while Kazan at span 75 (3.4 px)
  /// and Reykjavik at 33 (7.6 px) read as cities. Measured on the shipped card,
  /// and the boundary between the two groups sits at about 3 px.
  ///
  /// 81 puts every city at 3.1 px or better. It costs nothing to do it — the
  /// grid is the cost, so coarsening it CUTS the worst case in the whole
  /// 1,249-record dataset from 99,008 triangles to 34,876 — which is the rare
  /// case of legibility and budget pulling the same way. The price is honest
  /// and stated: Tokyo's cell becomes 400 m, so its masses are districts rather
  /// than superblocks, and the extent is what this asset exists to show.
  const SPAN = { close: { min: 25, max: 81 }, map: { min: 13, max: 27 } };
  /// GUARDS, not design parameters, and the asymmetry between them is
  /// deliberate. The FLOOR bites — a hamlet's extent divided by the minimum
  /// span can be a couple of metres — and when it does it takes the span down
  /// with it, so the mesh gets SMALLER rather than finer and the grid still
  /// covers the modelled extent exactly.
  ///
  /// The CEILING must never bite, and it is set where it cannot. A ceiling
  /// that bites is a silent lie: the grid stops covering the extent, the
  /// boundary falls entirely outside the grid, and every cell in the square is
  /// "inside the city" — which is exactly what a 700 m ceiling did to Tokyo's
  /// map level, drawing it as a full 27 x 27 slab with no edge at all. The
  /// worst cell any record can ask for is a 50 million-person city at map
  /// detail: 50,523 m of extent over 27 cells, 1,871 m. Four thousand is that
  /// with room, and `plan` reports `cellClamped` when the guard fires so a
  /// caller is never lied to quietly.
  const CELL_LIMIT = { min: 12, max: 4000 };

  /// Character bands. These are the SAME thresholds the existing card uses to
  /// pick a town district, deliberately: a city that reads as commercial on
  /// the old card must not read as residential on the new one. Character
  /// drives fabric colour, plot density, tower behaviour and roof pitch, and
  /// it is derived from the record's population and capital flag alone.
  const BANDS = [
    { at: 2000000, name: "commercial" },
    { at: 400000, name: "mixed" },
    { at: 90000, name: "residential" },
    { at: 0, name: "small" },
  ];

  /// Fabric palettes. Wall and roof pairs per character, picked per mass by
  /// hash. Muted on purpose: a birds-eye reads through VALUE — dark asphalt,
  /// mid roofs, light plazas — and a saturated roof at this scale turns the
  /// city into confetti.
  const FABRIC = {
    small: {
      walls: [[0.66, 0.61, 0.53], [0.60, 0.54, 0.46], [0.70, 0.66, 0.58]],
      roofs: [[0.44, 0.28, 0.22], [0.38, 0.33, 0.30], [0.34, 0.30, 0.27]],
    },
    residential: {
      walls: [[0.62, 0.56, 0.49], [0.57, 0.49, 0.43], [0.67, 0.62, 0.55], [0.54, 0.47, 0.44]],
      roofs: [[0.42, 0.27, 0.22], [0.36, 0.32, 0.31], [0.31, 0.29, 0.28], [0.45, 0.31, 0.24]],
    },
    mixed: {
      walls: [[0.58, 0.55, 0.50], [0.63, 0.60, 0.54], [0.52, 0.49, 0.47], [0.66, 0.62, 0.55]],
      roofs: [[0.34, 0.33, 0.32], [0.38, 0.34, 0.31], [0.30, 0.30, 0.31], [0.40, 0.29, 0.25]],
    },
    commercial: {
      walls: [[0.52, 0.53, 0.55], [0.58, 0.58, 0.59], [0.46, 0.48, 0.52], [0.62, 0.61, 0.59]],
      roofs: [[0.32, 0.33, 0.35], [0.36, 0.36, 0.37], [0.29, 0.30, 0.32], [0.34, 0.34, 0.33]],
    },
  };
  /// Civic stone. A capital's core precinct and its one landmark are drawn in
  /// this and nothing else is, so the civic core reads as a different kind of
  /// ground from the fabric around it.
  const CIVIC = { wall: [0.76, 0.74, 0.68], roof: [0.55, 0.54, 0.51] };
  /// Glass, for the top stage of a tower setback. 1990 glazing is a dark hole
  /// rather than a mirror, which is the same call town-mesh.js made.
  const TOWER_GLASS = [0.30, 0.34, 0.38];

  const GROUND = {
    /// Carriageway and hard standing. The darkest thing in the mesh, which is
    /// what makes the street grid legible from above.
    road: [0.19, 0.19, 0.20],
    /// The plot's own ground: pavement, yards, parking. A shade lighter than
    /// the carriageway so a street reads against the block it serves.
    plot: [0.31, 0.30, 0.29],
    /// Vacant and low-density edge — dust, verge, the ground between things.
    open: [0.38, 0.36, 0.31],
    park: [0.24, 0.36, 0.20],
    plaza: [0.55, 0.53, 0.48],
    water: [0.13, 0.22, 0.29],
  };

  /// How far below the lowest LAND corner the water plate sits. Bathymetry is
  /// not sourced and must not be invented, so water is a plane 1.5 m under the
  /// shore rather than a modelled depth. It is enough to keep the coast from
  /// z-fighting the land and honest enough to be describable in one line.
  const WATER_DROP = 1.5;

  // ------------------------------------------------------------------ maths
  function clamp(v, lo, hi) { return v < lo ? lo : v > hi ? hi : v; }
  /// Quantisation is what makes the sampler-driven parts deterministic without
  /// pretending the samplers are. Every length derived from a division, from
  /// the cosine of a latitude or from a host callback is rounded to a fixed
  /// grid before it reaches a vertex, so a difference in the last place of a
  /// double cannot become a difference in a buffer.
  function q(x, step) { return Math.round(x / step) * step; }
  function num(v, fallback) { const n = Number(v); return Number.isFinite(n) ? n : fallback; }

  /// Linear interpolation through a table of [x, y] anchors, clamped at both
  /// ends. The two models in this file are both tables and both read through
  /// this.
  function through(table, x) {
    if (x <= table[0][0]) return table[0][1];
    const last = table[table.length - 1];
    if (x >= last[0]) return last[1];
    for (let i = 1; i < table.length; i += 1) {
      const a = table[i - 1], b = table[i];
      if (x <= b[0]) return a[1] + (b[1] - a[1]) * ((x - a[0]) / (b[0] - a[0]));
    }
    return last[1];
  }

  /// Built-up area in km2 for a population. Public, because a host that wants
  /// to label the card with the extent should read the same number the mesh
  /// was built from rather than recompute it.
  function areaKm2(pop) {
    return q(through(AREA_KM2, clamp(num(pop, 0), POP_FLOOR, POP_CEIL)), 0.0001);
  }
  /// The diameter, in metres, of a disc of that area. The built shape is not a
  /// disc — the boundary is lobed and the coast cuts it — but the model gives
  /// an area and the grid needs a width, and the equivalent disc is the one
  /// conversion between them that preserves the model exactly.
  function extentMetres(pop) {
    const a = areaKm2(pop) * 1e6;
    return q(2 * Math.sqrt(a / Math.PI), 1);
  }
  function peakHeight(pop) {
    return q(through(PEAK_M, clamp(num(pop, 0), POP_FLOOR, POP_CEIL)), 0.1);
  }
  function character(city) {
    const pop = clamp(num(city && city.pop, 0), 0, POP_CEIL);
    for (const band of BANDS) if (pop > band.at) return band.name;
    return "small";
  }

  function normaliseLod(v) {
    // The same vocabulary town-mesh.js and site-mesh.js agreed on: "map",
    // "far" and the numeric forms all name the cheap mesh. This kit has two
    // levels, not three, so "mid" and "card" resolve to close — a card is
    // where the close mesh is meant to be seen.
    if (v === "map" || v === "far" || v === 1 || v === 2 || v === "lod1" || v === "lod2") return "map";
    return "close";
  }

  // ---------------------------------------------------------------- builder
  /// Triangle soup in world axes. There is no transform stack and no yaw:
  /// every face in a massing model is axis-aligned or a roof plane, so a
  /// transform would be machinery with no user. Colour resolves at emit,
  /// because unlike town-mesh.js there is no variant cache to keep colourless
  /// — a city is built once and every mass is already unique.
  function Builder() {
    this.pos = []; this.nrm = []; this.col = [];
    this.parts = [];
  }
  /// A degenerate triangle is DROPPED rather than emitted with a fallback
  /// normal, exactly as the other kits do: a zero-area face is a modelling
  /// slip, and letting one through costs a NaN in a buffer or a black facet
  /// nobody can explain later.
  Builder.prototype.tri = function (a, b, c, base, mat) {
    const ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2];
    const vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
    let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
    const len = Math.sqrt(nx * nx + ny * ny + nz * nz);
    if (!(len > 1e-7)) return this;
    nx /= len; ny /= len; nz /= len;
    const lambert = nx * SUN[0] + ny * SUN[1] + nz * SUN[2];
    const k = (mat == null ? 1 : mat) * (AMBIENT + DIFFUSE * (lambert > 0 ? lambert : 0));
    const r = clamp(base[0] * k, 0, 1), g = clamp(base[1] * k, 0, 1), bl = clamp(base[2] * k, 0, 1);
    const tri = [a, b, c];
    for (let i = 0; i < 3; i += 1) {
      this.pos.push(tri[i][0], tri[i][1], tri[i][2]);
      this.nrm.push(nx, ny, nz);
      this.col.push(r, g, bl);
    }
    return this;
  };
  Builder.prototype.quad = function (a, b, c, d, base, mat) {
    return this.tri(a, b, c, base, mat).tri(a, c, d, base, mat);
  };
  /// Semantic ranges, counted in vertices, the convention equipment-mesh.js
  /// set and town-mesh.js follows. A part is contiguous, so every pass below
  /// walks the whole grid for one part rather than emitting cell by cell.
  Builder.prototype.part = function (name, kind, draw) {
    const first = this.pos.length / 3;
    draw();
    const count = this.pos.length / 3 - first;
    if (count > 0) this.parts.push({ name, kind, first, count });
    return this;
  };
  /// Resolve to render buffers and seat the whole thing on Y=0. Grade comes
  /// from the LOWEST vertex — for a coastal city that is the water plate, for
  /// an inland one the bottom of the lowest street — because the renderer only
  /// needs nothing to float.
  Builder.prototype.finish = function (description, extra) {
    const n = this.pos.length / 3;
    const positions = new Float32Array(this.pos);
    const normals = new Float32Array(this.nrm);
    const colors = new Float32Array(this.col);
    // SEAT FIRST, MEASURE SECOND, and the order is not cosmetic. Subtracting
    // the drop from a bounds computed beforehand does the arithmetic in
    // doubles while the buffer does it in float32, so the two disagree in the
    // last few places — 16.200000762939453 reported against 16.200000196695328
    // actually in the buffer. A host that sizes a camera from `bounds` and
    // then reads the buffer must get the same numbers, so the bounds are
    // computed from the FINAL positions.
    let drop = Infinity;
    for (let i = 1; i < positions.length; i += 3) if (positions[i] < drop) drop = positions[i];
    if (Number.isFinite(drop) && drop !== 0) {
      for (let i = 1; i < positions.length; i += 3) positions[i] -= drop;
    }
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    if (n === 0) { min[0] = min[1] = min[2] = 0; max[0] = max[1] = max[2] = 0; }
    const out = {
      positions, normals, colors,
      bounds: { min, max },
      size: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
      triangleCount: n / 3,
      parts: this.parts,
      description,
    };
    if (extra) for (const key of Object.keys(extra)) out[key] = extra[key];
    return out;
  };

  // ------------------------------------------------------------------- plan
  /// Cell classes. `plan` returns these as a Uint8Array so a host or a check
  /// can read the city's layout without walking a triangle buffer.
  const CLS = { OUT: 0, WATER: 1, ROAD: 2, PARK: 3, OPEN: 4, PLOT: 5 };
  /// Per-cell flags, same array shape.
  const FLAG = { TOWER: 1, LANDMARK: 2, PITCHED: 4, CIVIC: 8 };

  /// plan(city, opts) -> the grid, before any geometry exists.
  ///
  ///   city  { name, lon, lat, pop, rank, capital } — a Natural Earth 1:50m
  ///         populated-places record, the same shape `cities.js` ships. Only
  ///         `name` and `pop` are required to be meaningful; a record with no
  ///         coordinates builds at 0,0 and simply asks the samplers about the
  ///         Gulf of Guinea, which is the host's problem to avoid.
  ///
  ///   opts  lod     "close" (default) or "map".
  ///         height  (lon, lat) -> metres above sea level, or null/NaN for no
  ///                 data. Missing: the ground is flat, reported in
  ///                 `assumed.height`. Present but NaN at a point: that corner
  ///                 falls back to the city datum and is counted in
  ///                 `noHeightSamples`.
  ///         land    (lon, lat) -> boolean. Missing: everything is land,
  ///                 reported in `assumed.land`.
  ///         seed    anything; hashed. Default: the city's own name, longitude
  ///                 and latitude, so two records that share a name in
  ///                 different places are different cities and the same record
  ///                 is the same city forever without the caller doing
  ///                 anything.
  ///
  /// THE ORDER OF DECISIONS IS FIXED and each one can only remove ground from
  /// the one before it: extent, then boundary, then water, then roads, then
  /// parks, then plots. That is what makes the coastline authoritative — a
  /// mass cannot appear in the sea because the sea is decided two steps
  /// earlier — and it is why `land` is asked before anything is built rather
  /// than used to cull afterwards.
  function plan(city, opts) {
    const rec = city && typeof city === "object" ? city : {};
    const o = opts && typeof opts === "object" ? opts : {};
    const lod = normaliseLod(o.lod);
    const name = rec.name == null ? "" : String(rec.name);
    const lon0 = num(rec.lon, 0), lat0 = clamp(num(rec.lat, 0), -89.9, 89.9);
    const popRaw = Math.max(0, num(rec.pop, 0));
    const pop = clamp(popRaw, POP_FLOOR, POP_CEIL);
    const rank = num(rec.rank, 10);
    const capital = rec.capital === true;
    const seed = o.seed == null
      ? seedOf(name + "|" + q(lon0, 0.0001) + "|" + q(lat0, 0.0001))
      : seedOf(o.seed);
    const height = typeof o.height === "function" ? o.height : null;
    const land = typeof o.land === "function" ? o.land : null;

    const extent = extentMetres(pop);
    // Span first, cell second. The span is what the budget is written against,
    // so it is the quantity that gets clamped; the cell is then whatever makes
    // the span cover the modelled extent exactly. Forced odd so there is a
    // centre cell for the core to sit on.
    const limits = SPAN[lod];
    // THE CEILING IS THE CARD'S, AND THE MAP HAS THREE TIMES THE PIXELS. The
    // close span cap of 81 was derived for a 252 px vignette, where a finer
    // grid aliases into speckle. Drawn on the globe at 20 m per pixel the same
    // city is 900 px across, and 81 cells leave Chicago with 224 m superblocks
    // at 11 px each. A caller that knows its own pixel budget may raise the
    // ceiling -- never lower the floor, and never above the module's own hard
    // bound -- and the budget bars below keep grading the DEFAULT, so the card's
    // numbers do not move when the map asks for more.
    const maxSpan = Number.isFinite(o.maxSpan) ? clamp(Math.round(o.maxSpan), limits.min, 255) : limits.max;
    let span = clamp(Math.round(extent / BASE_CELL[lod]), limits.min, maxSpan);
    if ((span & 1) === 0) span += 1;
    let cell = q(extent / span, 0.5);
    let cellClamped = false;
    if (cell < CELL_LIMIT.min) {
      cell = CELL_LIMIT.min;
      span = Math.max(5, Math.round(extent / cell));
      if ((span & 1) === 0) span += 1;
      cellClamped = true;
    }
    if (cell > CELL_LIMIT.max) { cell = CELL_LIMIT.max; cellClamped = true; }
    const half = (span - 1) / 2;
    const radius = q(extent / 2, 0.5);
    const band = character(rec);
    const fabric = FABRIC[band] || FABRIC.mixed;
    const peak = peakHeight(pop);

    // THE STREET PLAN, from the record and nothing else. A capital gets
    // radials and a ring, which is what a planned capital tends to have and
    // what makes one legible from above; a large city gets a dense arterial
    // grid; a small one gets a crossroads. It is character, not cartography:
    // no real city's plan is claimed and the description says so.
    const layout = capital && span >= 25 ? "radial" : band === "small" ? "crossroads" : "grid";
    // ARTERIAL SPACING, about a kilometre of city between main roads, rounded
    // to whole cells and then clamped at BOTH ends — and the clamp is the
    // interesting half. An arterial occupies a whole cell, so the fraction of
    // the city given over to main roads is 1 - ((e-1)/e)^2: 17% at a spacing
    // of 12 cells, 30% at 6. Below 6 a metropolis drawn in 230 m cells would
    // be more road than city, which is both wrong and expensive — it was 55%
    // at a spacing of 3, and Tokyo came out with fewer masses than San
    // Francisco. Above 12 the grid stops being legible from above.
    const every = clamp(Math.round(1000 / cell), 6, 12);
    const ringAt = layout === "radial" ? q(radius * 0.6, 0.5) : 0;

    // Rank is the record's own prominence field and it is used for exactly one
    // thing: how concentrated the core is. A rank-0 or rank-1 city — a world
    // city on Natural Earth's own scale — gets a harder core than a rank-8
    // town of the same population. It cannot change the extent, the height
    // table or the character band, because those are the population's job and
    // double-counting a correlated field would make the model say things the
    // record does not.
    const coreBoost = rank <= 1 ? 1.18 : rank <= 3 ? 1.06 : rank <= 6 ? 1.0 : 0.94;

    // ------------------------------------------------------------ elevation
    // Corner-sampled, not centre-sampled. A ground quad takes its four corner
    // heights from the shared corner grid, so adjacent cells share an edge
    // exactly and the terrain is continuous — centre sampling would put a
    // vertical crack at every cell boundary on any slope at all.
    const cornerN = span + 1;
    const corner = new Float32Array(cornerN * cornerN);
    const mLon = lonMetres(lat0);
    const lonAt = (x) => lon0 + x / mLon;
    const latAt = (z) => clamp(lat0 - z / M_LAT, -90, 90);
    let datum = 0, noHeightSamples = 0;
    if (height) {
      const d = height(lon0, lat0);
      datum = Number.isFinite(d) ? d : 0;
      for (let cj = 0; cj < cornerN; cj += 1) {
        const z = (cj - half - 0.5) * cell;
        const lat = latAt(z);
        for (let ci = 0; ci < cornerN; ci += 1) {
          const x = (ci - half - 0.5) * cell;
          const h = height(lonAt(x), lat);
          if (Number.isFinite(h)) corner[cj * cornerN + ci] = q(h - datum, 0.05);
          else { corner[cj * cornerN + ci] = 0; noHeightSamples += 1; }
        }
      }
    }

    // ---------------------------------------------------------------- cells
    const cls = new Uint8Array(span * span);
    const flag = new Uint8Array(span * span);
    const hgt = new Float32Array(span * span);
    const counts = { footprint: 0, water: 0, road: 0, park: 0, open: 0, plot: 0, tower: 0 };
    let minLand = Infinity, maxLand = -Infinity, tallest = 0;
    // Density ceiling by character. A small town is mostly gaps; a commercial
    // metropolis is mostly built. This is the single largest lever on both the
    // read and the triangle count, which is why it is a named table rather
    // than a number inside the loop.
    const density = band === "commercial" ? 0.92 : band === "mixed" ? 0.84
      : band === "residential" ? 0.74 : 0.58;

    for (let j = 0; j < span; j += 1) {
      const z = (j - half) * cell;
      const dj = j - half;
      for (let i = 0; i < span; i += 1) {
        const x = (i - half) * cell;
        const di = i - half;
        const idx = j * span + i;
        const dist = Math.sqrt(x * x + z * z);
        const t = radius > 0 ? dist / radius : 1;

        // 1. THE BOUNDARY. A falloff plus two octaves of lattice noise, so the
        //    edge is lobed the way a city's edge is — fingers along valleys
        //    and roads, bites out of the middle — rather than a circle. `u` is
        //    urbanity: 1 at the core, 0 at the edge, negative outside.
        const lobes = noise(seed, i, j, 4.5, 11) * 0.30 + noise(seed, i, j, 2.0, 12) * 0.12;
        const u = (1 - t) * coreBoost + lobes;
        if (u <= 0) { cls[idx] = CLS.OUT; continue; }
        counts.footprint += 1;

        // 2. WATER. Asked of the host, at the cell centre, before anything is
        //    built here. The coastline is therefore resolved at the cell —
        //    a 100 m staircase at close detail and a 400 m one at map detail,
        //    which is stated in the description rather than hidden.
        if (land && !land(lonAt(x), latAt(z))) {
          cls[idx] = CLS.WATER; counts.water += 1; continue;
        }

        const cs = cellSeed(seed, i, j);
        const cornerMin = Math.min(
          corner[j * cornerN + i], corner[j * cornerN + i + 1],
          corner[(j + 1) * cornerN + i], corner[(j + 1) * cornerN + i + 1],
        );
        if (cornerMin < minLand) minLand = cornerMin;
        if (cornerMin > maxLand) maxLand = cornerMin;

        // 3. ROADS. The arterial grid, plus a ring and two diagonal avenues
        //    for a capital, plus the crossroads a small town gets instead of a
        //    grid. Roads are decided before plots so a road always wins: a
        //    street that a building can land on is not a street.
        // The primary cross is in every city. A small town gets that and
        // nothing else: the lanes between its plots are already drawn as the
        // street inset every mass carries, and an arterial grid on a village
        // is a claim about a village.
        let road = di === 0 || dj === 0;
        // THE ARTERIAL GRID IS A CLOSE-DETAIL FEATURE, and this is the one
        // place the two levels genuinely differ rather than differing in
        // resolution. A road occupies a whole cell, so a grid of spacing e
        // costs 1 - ((e-1)/e)^2 of the city whatever the cell is worth in
        // metres: 30% at a spacing of 6. At close detail that buys a legible
        // 1.4 km arterial grid. At map detail the cell is 1.2 km, the same
        // grid would be 7 km apart, and it consumed HALF of Tokyo's map mesh
        // to draw roads nobody can resolve at that zoom. So the map level
        // keeps the cross and the ring and drops the grid; extent, boundary,
        // coast, density and height are identical between the two.
        if (!road && lod === "close" && layout !== "crossroads") {
          road = di % every === 0 || dj % every === 0;
        }
        if (!road && layout === "radial") {
          // TWO CELLS WIDE AT CLOSE DETAIL, and that is not decoration. A
          // diagonal one cell wide is a line of cells touching only at their
          // corners, and from above it reads as a dotted line rather than as
          // an avenue — it was visibly dashed on the first render of every
          // capital. `d - 0` and `d - 1` together are a continuous staircase
          // and cost about 4/span of the city, under 3% at close detail.
          if (lod === "close") {
            const d1 = di - dj, d2 = di + dj;
            if (d1 === 0 || d1 === 1 || d2 === 0 || d2 === 1) road = true;
          }
          if (!road && ringAt > 0 && Math.abs(dist - ringAt) <= cell * 0.5) road = true;
        }
        if (road) { cls[idx] = CLS.ROAD; counts.road += 1; continue; }

        // 4. OPEN SPACE. A noise field of its own, so parks come in patches
        //    rather than as salt. A capital also gets a civic plaza of nine
        //    cells at the core, which is the one piece of deliberate
        //    composition in the plan.
        const civicPlaza = capital && Math.abs(di) <= 1 && Math.abs(dj) <= 1 && span >= 25;
        if (civicPlaza) {
          cls[idx] = CLS.PARK; flag[idx] |= FLAG.CIVIC; counts.park += 1; continue;
        }
        if (noise(seed, i, j, 3.0, 21) > 0.52) {
          cls[idx] = CLS.PARK; counts.park += 1; continue;
        }

        // 5. PLOTS. Density falls with urbanity, so the edge thins into open
        //    ground instead of stopping at a wall of buildings.
        const chance = clamp(density * (0.34 + 0.72 * clamp(u, 0, 1)), 0, 0.97);
        if (askUnit(cs, 31) >= chance) { cls[idx] = CLS.OPEN; counts.open += 1; continue; }

        cls[idx] = CLS.PLOT; counts.plot += 1;

        // 6. HEIGHT. A quadratic falloff from the core with a floor, times a
        //    per-mass variation. Quadratic rather than exponential for the
        //    determinism reason at the top of the file, and it happens to be
        //    the better shape anyway: a city's height profile is flatter in
        //    the middle and steeper at the edge than an exponential is.
        const profile = 0.15 + 0.85 * (1 - clamp(t, 0, 1)) * (1 - clamp(t, 0, 1));
        let h = peak * profile * askSpan(cs, 33, 0.58, 1.32);
        // Towers. Only near the core, only for cities big enough to have a
        // core worth the name, and rare enough that they read as a cluster.
        const towerZone = (band === "commercial" || band === "mixed") && t < 0.32;
        if (towerZone && askUnit(cs, 35) < (band === "commercial" ? 0.20 : 0.10)) {
          h *= askSpan(cs, 37, 1.5, 2.4);
          flag[idx] |= FLAG.TOWER; counts.tower += 1;
        }
        h = q(clamp(h, 5, peak * 2.6), 0.1);
        // A pitched roof, at close detail only, for anything low enough to
        // have one. From above a ridge line is the single cheapest thing that
        // says "houses" rather than "boxes", and it costs four triangles.
        if (lod === "close" && h <= 16 && (band === "small" || band === "residential" || t > 0.45)) {
          flag[idx] |= FLAG.PITCHED;
        }
        hgt[idx] = h;
        if (h > tallest) tallest = h;
      }
    }

    // THE LANDMARK. One per capital, on the civic plaza, taller than anything
    // else in the city. It is placed on the first plaza cell found in a fixed
    // spiral from the core, so it is deterministic and it still exists when
    // the core cell itself turned out to be in the water.
    let landmark = null;
    if (capital && counts.footprint > 0) {
      const ring = [[0, 0], [1, 0], [0, 1], [-1, 0], [0, -1], [1, 1], [-1, 1], [1, -1], [-1, -1],
        [2, 0], [0, 2], [-2, 0], [0, -2], [3, 1], [-3, -1]];
      for (const [oi, oj] of ring) {
        const i = half + oi, j = half + oj;
        if (i < 0 || j < 0 || i >= span || j >= span) continue;
        const idx = j * span + i;
        if (cls[idx] === CLS.OUT || cls[idx] === CLS.WATER) continue;
        // The cell it takes was a plaza, a road or open ground a moment ago,
        // and the counts have to follow it — `cells.park` and `cells.plot` are
        // read by the description and by the checks, and a landmark that
        // silently rewrote a cell without paying for it made them disagree
        // with the mesh by one.
        if (cls[idx] === CLS.PARK) counts.park -= 1;
        else if (cls[idx] === CLS.ROAD) counts.road -= 1;
        else if (cls[idx] === CLS.OPEN) counts.open -= 1;
        if (cls[idx] !== CLS.PLOT) counts.plot += 1;
        cls[idx] = CLS.PLOT;
        flag[idx] |= FLAG.LANDMARK | FLAG.CIVIC;
        flag[idx] &= ~FLAG.PITCHED;
        hgt[idx] = q(clamp(peak * 1.35, 14, 320), 0.1);
        if (hgt[idx] > tallest) tallest = hgt[idx];
        landmark = { i, j, height: hgt[idx] };
        break;
      }
    }

    if (!Number.isFinite(minLand)) { minLand = 0; maxLand = 0; }
    const waterLevel = q(minLand - WATER_DROP, 0.05);
    const cellKm2 = (cell * cell) / 1e6;
    return {
      name, lon: lon0, lat: lat0, pop: popRaw, rank, capital,
      seed, lod, span, cell, half, radius, extent, cellClamped,
      character: band, fabric, layout, every, ringAt, peak, coreBoost, density,
      cls, flag, hgt, corner, cornerN, counts, landmark,
      waterLevel, datum, noHeightSamples,
      relief: [q(minLand, 0.05), q(maxLand, 0.05)],
      tallest: q(tallest, 0.1),
      modelAreaKm2: areaKm2(pop),
      // Three different areas and they are not interchangeable.
      // `footprintKm2` is every cell the boundary claimed, water included —
      // the modelled extent. `landKm2` is what survived the host's land mask,
      // which is the city's actual ground. `builtKm2` is the cells that carry
      // a mass. Conflating the first two is how a mesh ends up claiming a
      // drowned city is built-up.
      footprintKm2: q(counts.footprint * cellKm2, 0.0001),
      landKm2: q((counts.footprint - counts.water) * cellKm2, 0.0001),
      builtKm2: q(counts.plot * cellKm2, 0.0001),
      waterKm2: q(counts.water * cellKm2, 0.0001),
      assumed: { height: !height, land: !land },
      popFloored: popRaw < POP_FLOOR,
      origin: { lon: lon0, lat: lat0 },
      metresPerDegree: { lon: q(mLon, 0.001), lat: M_LAT },
    };
  }

  // --------------------------------------------------------------- geometry
  /// A ground quad, taking its four corners from the shared corner grid. Wound
  /// so the normal is +Y: (x0,z0) -> (x0,z1) -> (x1,z1) -> (x1,z0).
  function groundQuad(b, p, i, j, base, mat) {
    const { cell, half, cornerN, corner } = p;
    const x0 = (i - half - 0.5) * cell, x1 = x0 + cell;
    const z0 = (j - half - 0.5) * cell, z1 = z0 + cell;
    const y00 = corner[j * cornerN + i], y10 = corner[j * cornerN + i + 1];
    const y01 = corner[(j + 1) * cornerN + i], y11 = corner[(j + 1) * cornerN + i + 1];
    b.quad([x0, y00, z0], [x0, y01, z1], [x1, y11, z1], [x1, y10, z0], base, mat);
  }

  /// Four walls and a flat roof: ten triangles, the unit the whole budget is
  /// written in. No bottom face — nothing in a birds-eye can see one, and it
  /// would be a fifth of the cost of the entire model for nothing.
  function boxTop(b, x0, x1, z0, z1, y0, y1, wall, roof, mat) {
    b.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], wall, mat);          // south
    b.quad([x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0], wall, mat);          // north
    b.quad([x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1], wall, mat * 1.06);   // east
    b.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], wall, mat * 0.94);   // west
    b.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], roof, mat);          // roof
  }

  /// The same box with a ridged roof instead of a flat one. Fourteen
  /// triangles: the four walls, two roof planes and two gable ends. The ridge
  /// runs along the longer axis, which is what a real terrace does and what
  /// makes a street of them line up when seen from above.
  function boxRidge(b, x0, x1, z0, z1, y0, y1, wall, roof, mat, pitch) {
    const rise = Math.min(pitch, (Math.min(x1 - x0, z1 - z0)) * 0.42);
    const ridge = y1 + rise;
    if (x1 - x0 >= z1 - z0) {
      const zm = (z0 + z1) / 2;
      b.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], wall, mat);
      b.quad([x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0], wall, mat);
      b.quad([x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1], wall, mat * 1.06);
      b.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], wall, mat * 0.94);
      b.quad([x0, y1, z1], [x1, y1, z1], [x1, ridge, zm], [x0, ridge, zm], roof, mat * 1.08);
      b.quad([x1, y1, z0], [x0, y1, z0], [x0, ridge, zm], [x1, ridge, zm], roof, mat * 0.9);
      b.tri([x1, y1, z1], [x1, y1, z0], [x1, ridge, zm], wall, mat * 1.06);
      b.tri([x0, y1, z0], [x0, y1, z1], [x0, ridge, zm], wall, mat * 0.94);
    } else {
      const xm = (x0 + x1) / 2;
      b.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], wall, mat);
      b.quad([x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0], wall, mat);
      b.quad([x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1], wall, mat * 1.06);
      b.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], wall, mat * 0.94);
      b.quad([x1, y1, z1], [x1, y1, z0], [xm, ridge, z0], [xm, ridge, z1], roof, mat * 1.08);
      b.quad([x0, y1, z0], [x0, y1, z1], [xm, ridge, z1], [xm, ridge, z0], roof, mat * 0.9);
      b.tri([x0, y1, z1], [x1, y1, z1], [xm, ridge, z1], wall, mat);
      b.tri([x1, y1, z0], [x0, y1, z0], [xm, ridge, z0], wall, mat);
    }
  }

  /// The mass a plot carries. Footprint is the cell inset by a street, then
  /// varied per plot so a block is not a row of identical stamps; base is the
  /// LOWEST of the cell's four ground corners, so on a slope a mass digs into
  /// the hill rather than floating over it.
  function massFor(p, i, j) {
    const { cell, half, cornerN, corner } = p;
    const idx = j * p.span + i;
    const cs = cellSeed(p.seed, i, j);
    // Street width: a quarter of the cell, floored at 7 m so a village lane is
    // still a lane, capped at 34 m so a superblock is not mostly tarmac.
    const street = q(clamp(cell * 0.26, 7, 34), 0.1);
    const room = cell - street;
    const w = q(room * askSpan(cs, 41, 0.80, 1.0), 0.1);
    const d = q(room * askSpan(cs, 43, 0.80, 1.0), 0.1);
    const cx = (i - half) * cell + q((room - w) * (askUnit(cs, 45) - 0.5), 0.1);
    const cz = (j - half) * cell + q((room - d) * (askUnit(cs, 47) - 0.5), 0.1);
    const base = Math.min(
      corner[j * cornerN + i], corner[j * cornerN + i + 1],
      corner[(j + 1) * cornerN + i], corner[(j + 1) * cornerN + i + 1],
    );
    return {
      x0: q(cx - w / 2, 0.1), x1: q(cx + w / 2, 0.1),
      z0: q(cz - d / 2, 0.1), z1: q(cz + d / 2, 0.1),
      y0: q(base - 0.4, 0.05), y1: q(base + p.hgt[idx], 0.05),
      seed: cs, flag: p.flag[idx],
    };
  }

  function fabricColour(p, cs, flag) {
    if (flag & FLAG.CIVIC) return { wall: CIVIC.wall, roof: CIVIC.roof };
    const walls = p.fabric.walls, roofs = p.fabric.roofs;
    return {
      wall: walls[ask(cs, 51) % walls.length],
      roof: roofs[ask(cs, 53) % roofs.length],
    };
  }

  // ------------------------------------------------------------------ build
  /// build(city, opts) -> mesh. Same option bag as `plan`, which it calls.
  function build(city, opts) {
    const p = plan(city, opts);
    const b = new Builder();
    const close = p.lod === "close";
    const { span, cls, flag, hgt } = p;

    // GROUND, in three passes so each is one contiguous part: the hard
    // surfaces, then open space and parks, then water. Ground is drawn under
    // the masses as well as between them — a mass is inset by a street, so
    // most of the plot's own ground is visible from above anyway, and the
    // saving from skipping it would be a rounding error against the mass.
    b.part("ground / streets, plots and open ground", "ground", () => {
      for (let j = 0; j < span; j += 1) {
        for (let i = 0; i < span; i += 1) {
          const c = cls[j * span + i];
          if (c !== CLS.ROAD && c !== CLS.PLOT && c !== CLS.OPEN) continue;
          const base = c === CLS.ROAD ? GROUND.road : c === CLS.PLOT ? GROUND.plot : GROUND.open;
          const mat = 0.9 + 0.2 * askUnit(cellSeed(p.seed, i, j), 61);
          groundQuad(b, p, i, j, base, mat);
        }
      }
    });
    b.part("ground / parks and civic plaza", "park", () => {
      for (let j = 0; j < span; j += 1) {
        for (let i = 0; i < span; i += 1) {
          const idx = j * span + i;
          if (cls[idx] !== CLS.PARK) continue;
          const civic = (flag[idx] & FLAG.CIVIC) !== 0;
          const mat = 0.92 + 0.16 * askUnit(cellSeed(p.seed, i, j), 63);
          groundQuad(b, p, i, j, civic ? GROUND.plaza : GROUND.park, mat);
        }
      }
    });
    if (p.counts.water > 0) {
      b.part("ground / water", "water", () => {
        const { cell, half, waterLevel } = p;
        for (let j = 0; j < span; j += 1) {
          const z0 = (j - half - 0.5) * cell, z1 = z0 + cell;
          for (let i = 0; i < span; i += 1) {
            if (cls[j * span + i] !== CLS.WATER) continue;
            const x0 = (i - half - 0.5) * cell, x1 = x0 + cell;
            b.quad([x0, waterLevel, z0], [x0, waterLevel, z1], [x1, waterLevel, z1],
              [x1, waterLevel, z0], GROUND.water, 0.94 + 0.12 * askUnit(cellSeed(p.seed, i, j), 65));
          }
        }
      });
    }

    // MASSING, in three passes for the same reason. Fabric first — everything
    // that is not a tower and not the landmark — then the towers, then the one
    // landmark, so a host can highlight a city's core without walking the
    // whole buffer.
    b.part("massing / " + p.character + " fabric", "fabric", () => {
      for (let j = 0; j < span; j += 1) {
        for (let i = 0; i < span; i += 1) {
          const idx = j * span + i;
          if (cls[idx] !== CLS.PLOT || (flag[idx] & (FLAG.TOWER | FLAG.LANDMARK))) continue;
          const m = massFor(p, i, j);
          const col = fabricColour(p, m.seed, m.flag);
          const mat = askSpan(m.seed, 55, 0.86, 1.12);
          if (close && (m.flag & FLAG.PITCHED)) {
            boxRidge(b, m.x0, m.x1, m.z0, m.z1, m.y0, m.y1, col.wall, col.roof, mat,
              q(2.0 + 2.5 * askUnit(m.seed, 57), 0.1));
          } else {
            boxTop(b, m.x0, m.x1, m.z0, m.z1, m.y0, m.y1, col.wall, col.roof, mat);
          }
        }
      }
    });
    if (p.counts.tower > 0) {
      b.part("massing / core towers", "tower", () => {
        for (let j = 0; j < span; j += 1) {
          for (let i = 0; i < span; i += 1) {
            const idx = j * span + i;
            if (cls[idx] !== CLS.PLOT || !(flag[idx] & FLAG.TOWER) || (flag[idx] & FLAG.LANDMARK)) continue;
            const m = massFor(p, i, j);
            const col = fabricColour(p, m.seed, m.flag);
            const mat = askSpan(m.seed, 55, 0.9, 1.14);
            // A setback at close detail: a podium to a third of the height and
            // a slimmer shaft above it, which is the silhouette that separates
            // a tower from a tall box. Twenty triangles instead of ten, and
            // only the few per cent of masses that are towers pay it.
            if (close && m.y1 - m.y0 > 45) {
              const podium = q(m.y0 + (m.y1 - m.y0) * 0.32, 0.05);
              const ix = q((m.x1 - m.x0) * 0.16, 0.1), iz = q((m.z1 - m.z0) * 0.16, 0.1);
              boxTop(b, m.x0, m.x1, m.z0, m.z1, m.y0, podium, col.wall, col.roof, mat);
              boxTop(b, m.x0 + ix, m.x1 - ix, m.z0 + iz, m.z1 - iz, podium, m.y1,
                TOWER_GLASS, col.roof, mat);
            } else {
              boxTop(b, m.x0, m.x1, m.z0, m.z1, m.y0, m.y1, col.wall, col.roof, mat);
            }
          }
        }
      });
    }
    if (p.landmark) {
      b.part("civic / landmark", "landmark", () => {
        const m = massFor(p, p.landmark.i, p.landmark.j);
        // A plinth, a shaft and a capped top. Thirty triangles for the one
        // thing on the card a player will look for, and the only piece of
        // geometry in the file that is not a cell of city.
        const plinth = q(m.y0 + (m.y1 - m.y0) * 0.14, 0.05);
        const shaftTop = q(m.y0 + (m.y1 - m.y0) * 0.86, 0.05);
        const w = (m.x1 - m.x0), d = (m.z1 - m.z0);
        const sx = q(w * 0.28, 0.1), sz = q(d * 0.28, 0.1);
        boxTop(b, m.x0, m.x1, m.z0, m.z1, m.y0, plinth, CIVIC.wall, CIVIC.roof, 1.0);
        boxTop(b, m.x0 + sx, m.x1 - sx, m.z0 + sz, m.z1 - sz, plinth, shaftTop,
          CIVIC.wall, CIVIC.roof, 1.05);
        if (close) {
          const cx = q(w * 0.4, 0.1), cz = q(d * 0.4, 0.1);
          boxTop(b, m.x0 + cx, m.x1 - cx, m.z0 + cz, m.z1 - cz, shaftTop, m.y1,
            CIVIC.wall, CIVIC.roof, 1.1);
        }
      });
    }

    return b.finish(describe(p), meta(p));
  }

  /// The metadata a card, a check or a debug overlay reads. Everything here is
  /// either transcribed from the record or derived from the declared models,
  /// and the two are kept in separate fields on purpose: `pop` is the record's
  /// own number, `modelAreaKm2` is the model's.
  function meta(p) {
    return {
      city: { name: p.name, lon: p.lon, lat: p.lat, pop: p.pop, rank: p.rank, capital: p.capital },
      lod: p.lod, seed: p.seed,
      span: p.span, cell: p.cell,
      extent: [q(p.span * p.cell, 0.5), q(p.span * p.cell, 0.5)],
      modelExtent: p.extent,
      character: p.character, layout: p.layout, arterialEvery: p.every,
      peakHeight: p.peak, tallest: p.tallest,
      modelAreaKm2: p.modelAreaKm2, footprintKm2: p.footprintKm2,
      landKm2: p.landKm2, builtKm2: p.builtKm2, waterKm2: p.waterKm2,
      cells: {
        footprint: p.counts.footprint, land: p.counts.footprint - p.counts.water,
        plot: p.counts.plot, road: p.counts.road, park: p.counts.park,
        open: p.counts.open, water: p.counts.water, tower: p.counts.tower,
      },
      coastal: p.counts.water > 0,
      relief: p.relief.slice(), datum: p.datum, noHeightSamples: p.noHeightSamples,
      landmark: p.landmark ? { i: p.landmark.i, j: p.landmark.j, height: p.landmark.height } : null,
      assumed: { height: p.assumed.height, land: p.assumed.land },
      origin: { lon: p.origin.lon, lat: p.origin.lat },
      metresPerDegree: p.metresPerDegree,
      areaModel: "A_km2 = 40 * (pop / 1e6) ^ 0.85, representative",
      waterLevel: p.waterLevel,
      popFloored: p.popFloored,
      cellClamped: p.cellClamped,
    };
  }

  /// WHAT THE PLAYER IS TOLD. Every clause here exists because the alternative
  /// is a player assuming the grid under a real place name is that place's
  /// streets. The order is: what it is, what the extent is and where it came
  /// from, what the samplers did or did not provide, and what it grants
  /// (nothing).
  function describe(p) {
    // Formatted, not rounded: `q(12.1, 0.01)` is 12.100000000000001 in a
    // double and a description is read by a person.
    const km = (v) => (v >= 100 ? String(Math.round(v)) : v >= 10 ? v.toFixed(1) : v.toFixed(2));
    const name = p.name || "an unnamed place";
    const head = "Original game art: birds-eye massing of " + name + " — "
      + p.counts.plot.toLocaleString("en-US") + " masses on a " + p.span + " x " + p.span
      + " grid of " + km(p.cell) + " m cells, " + km(p.landKm2) + " km2 of land inside a "
      + km(p.footprintKm2) + " km2 modelled extent about " + km(p.span * p.cell / 1000)
      // `peak` is the core SCALE PARAMETER, not a height: masses are drawn at
      // peak x profile x a spread, towers take another 1.5-2.4x and the clamp
      // is peak x 2.6. Reporting it as "core N m" understated Tokyo by 2.5x
      // (236 m said against 591 m built) and Toronto by 2.5x. `tallest` is the
      // measured maximum and is what a reader means by how tall the city is.
      + " km across, tallest " + km(p.tallest) + " m."
      + (p.counts.footprint > 0 && p.counts.water === p.counts.footprint
        ? " THERE IS NO LAND HERE: the host's mask refused every cell in the extent, so this is"
        + " open water and no city was built on it." : "");
    const model = " EXTENT IS A MODEL, NOT A MEASUREMENT: built-up area is "
      + "A = 40 x (population/1e6)^0.85 km2, a representative dense-city curve, "
      + "and the record's population of " + p.pop.toLocaleString("en-US")
      + (p.popFloored ? " is floored at " + POP_FLOOR.toLocaleString("en-US") + " to draw at all" : "")
      + ". The street grid, the " + p.layout + " plan, heights, parks and towers are generic urban "
      + "fabric derived from population, scale rank and the capital flag — not "
      + name + "'s own plan, and no street, district or building here is that city's.";
    const corners = p.cornerN * p.cornerN;
    const relief = p.assumed.height
      ? " Ground is assumed flat: no elevation sampler was supplied."
      : p.noHeightSamples >= corners
        ? " Ground is flat because the elevation sampler returned no data at any of its "
          + corners.toLocaleString("en-US") + " corners."
        : " Relief is the host's elevation sampler, " + km(p.relief[1] - p.relief[0])
          + " m of it across the extent, at 1:1 with no vertical exaggeration"
          + (p.noHeightSamples ? " (" + p.noHeightSamples.toLocaleString("en-US")
            + " corners had no data and sit at the datum)" : "") + ".";
    const coast = p.assumed.land
      ? " Everything inside the extent is assumed to be land: no land mask was supplied."
      : " The coastline is the host's land mask, resolved at the " + km(p.cell)
        + " m cell, which refused " + p.counts.water.toLocaleString("en-US")
        + " of " + p.counts.footprint.toLocaleString("en-US") + " cells as water.";
    const ground = relief + coast;
    return head + model + ground + " It grants no simulation capability.";
  }

  // --------------------------------------------------------------- exports
  return Object.freeze({
    build, plan,
    areaKm2, extentMetres, peakHeight, character,
    classes: Object.freeze(Object.assign({}, CLS)),
    flags: Object.freeze(Object.assign({}, FLAG)),
    areaModel: Object.freeze({
      formula: "A_km2 = 40 * (pop / 1e6) ^ 0.85",
      anchors: AREA_KM2.map((r) => r.slice()),
      note: "Representative, not sourced. 25,000 people per built km2 at a million.",
    }),
    heightModel: Object.freeze({ anchors: PEAK_M.map((r) => r.slice()) }),
    /// `ceiling` is the hard bar for ANY record; `largest` is the band a city
    /// over two million must land in. Both are asserted in
    /// tools/ui/check_city_mesh.cjs against the whole of cities.js.
    budget: Object.freeze({
      ceiling: Object.freeze({ close: 40000, map: 5000 }),
      floor: Object.freeze({ close: 250, map: 100 }),
      largest: Object.freeze({ overPop: 2000000, close: [24000, 40000] }),
    }),
    baseCell: Object.freeze(Object.assign({}, BASE_CELL)),
    spanLimits: Object.freeze({ close: Object.assign({}, SPAN.close), map: Object.assign({}, SPAN.map) }),
    populationFloor: POP_FLOOR,
    metresPerDegree: (lat) => ({ lon: lonMetres(lat), lat: M_LAT }),
    version: "urban.city_massing.v1",
    kit: "city_massing",
  });
});
