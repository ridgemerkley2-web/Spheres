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
// WHAT READS AT FOUR PIXELS A CELL. The card draws a cell at about 3 px and
// the globe at about 4.5; nothing finer than a cell can be resolved at either,
// so "more detail" cannot mean a finer grid. It means STRUCTURE inside and
// between the cells that reads at that size, and this file draws five kinds
// of it at close detail and none of them at map detail:
//
//   - A built cell carries one to four MASSES, not one: different footprints,
//     different heights, an alley between them, the largest fronting the
//     nearest road. `lots` below is the whole of that decision.
//   - A ROAD HIERARCHY: an arterial is a whole cell of the darkest ground; a
//     local street is the ring a block's pad leaves round itself, one quarter
//     of a cell wide; a lane is the alley on the block. Three widths, always
//     in that order, reported in `streets`.
//   - HEIGHT that steps rather than slopes: a district term six cells wide on
//     top of the falloff, one mass in twenty-five inside the inner two thirds
//     standing 1.6-2.2x over its block, pitched roofs on the low edge, and a
//     roof colour per mass.
//   - EDGES: a tree line on every side of a park that faces something that is
//     not park, a line of steps round a plaza, and a QUAY on every side of a
//     land cell that faces water the host's mask refused.
//   - A capital's PLAZA decided before its roads, so the avenues radiate from
//     it instead of paving it over, a civic precinct of pale single masses
//     round it, and a landmark drawn in four narrowing stages.
//
// TRIANGLE BUDGET, declared with its derivation and then measured against
// every one of the 1,249 records in cities.js, AT DEFAULT SPAN — the card's
// ceiling of 81 cells. A caller that raises the span (the map does, to 181)
// buys geometry in proportion and is not graded here; see the end of this
// block for what it pays.
//
//   close   44,000 - 80,000 for cities over two million; 80,000 is a hard
//           ceiling for any city at all
//   map     250 - 5,000 for any city at all
//
// WHERE THE NUMBERS COME FROM. The grid is capped at 81 cells across at close
// detail, so at most 6,561 cells exist and about 70% of the square, ~4,600
// cells, is inside the lobed boundary. Ground costs a 2-triangle quad per
// cell, plus 2 more for the pad on every built cell and on every open cell
// with a built or road neighbour: ~13,000 at most. Parks and plazas pay 2 a
// side for their edges and quays 2 a side where land meets water, ~2,000
// together. Of the cells left after 16-33% arterial and 8-20% park, between
// a third and nine tenths carry masses, and at close detail a built cell
// carries one to three of them under 260 m and two to four over it, about 2
// on average. A mass is 10 triangles — four walls and a flat roof, no
// underside, because nothing in a birds-eye can see one — 14 with a pitched
// roof and 20 with a tower setback. That is 2,500-3,000 built cells,
// 5,000-6,000 masses and 50,000-65,000 triangles of massing for the biggest
// cities.
//
// MEASURED, over all 1,249 records, both levels, 2026-09-07:
//   close  max 69,036 (Sao Paulo), min 2,868 (Melekeok), median 23,992,
//          p99 62,790; every city over two million lands between 48,188
//          (Abidjan) and 69,036. The 80,000 ceiling therefore carries 16% of
//          headroom over the worst real case and the 44,000 floor of the band
//          sits 9% under the leanest metropolis: both can still fail.
//   map    max 3,988 (San Francisco), min 392 (Mazar-e Sharif), median 760 —
//          unchanged, because none of the close detail is drawn at map level.
//
// WHAT THE CEILING WAS, AND WHY IT MOVED. It was 40,000 against a worst case
// of 34,876 (Vancouver) when a built cell was one mass and a plot's ground
// was one quad. One to four masses a cell, the pad under them and the edges
// on parks and shores roughly doubled the close cost of every city; the
// ceiling was re-measured and moved WITH that change rather than widened
// ahead of it, and it still cannot pass a city that quietly doubles again.
//
// ABOVE THREE AND A HALF MILLION PEOPLE THE TRIANGLE COUNT STOPS GROWING and
// the cell grows instead: the grid is pinned at 81 and Tokyo is drawn in
// 400 m district cells where Toronto gets 176 m superblocks. Tokyo is still
// twice as wide on the ground. That is the trade this file makes deliberately
// — the EXTENT is the thing the model exists to show, and resolution is what
// pays for it. The map buys the resolution back with `maxSpan`.
//
// AT THE MAP'S SPAN of 181 the same records measure up to 302,396 (Lagos):
// Chicago 300,982, Tokyo 254,572, Toronto 193,448, against 158,822 for
// Chicago when a cell was one mass. That is the map's budget to keep, and it
// keeps it by asking for fewer cells when a city is smaller on screen and by
// bounding its cache. A Chicago at 181 builds in about 120 ms on one core.
//
// THE COMPARISON THAT MATTERS: today's single town block is 198,462 triangles
// at close detail and 3,422 at map. An entire city here is 69,036 at worst —
// a third of the cost of the one block it replaces — and its map level is the
// same order as the block's while showing a city rather than a street corner.
//
// WIRED IN. The city card in index.html builds this at default span, and
// city-layer.js moves it onto the globe at up to 181 cells.
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
  /// primitive: one block, one patch of ground, and at close detail the one
  /// to four masses `lots` puts on it. 100 m is the right size for it
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
  /// grid is the cost, so coarsening it CUT the worst case in the whole
  /// 1,249-record dataset from 99,008 triangles to 34,876 when a cell was one
  /// mass — which is the rare case of legibility and budget pulling the same
  /// way. The price is honest and stated: Tokyo's cell becomes 400 m, so its
  /// masses are districts rather than superblocks, and the extent is what
  /// this asset exists to show.
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
    /// what makes the street grid legible from above. An arterial is a whole
    /// cell of this.
    road: [0.19, 0.19, 0.20],
    /// A LOCAL STREET: the ring of carriageway around a block, between it and
    /// its neighbours. A shade lighter than an arterial so the two widths read
    /// as two ranks of road and not as one road drawn twice.
    lane: [0.22, 0.22, 0.23],
    /// The plot's own ground: pavement, yards, parking. A shade lighter than
    /// the carriageway so a street reads against the block it serves.
    plot: [0.31, 0.30, 0.29],
    /// Vacant and low-density edge — dust, verge, the ground between things.
    open: [0.38, 0.36, 0.31],
    park: [0.24, 0.36, 0.20],
    /// The tree line or path along a park's edge. Darker than the park so the
    /// park has an outline rather than being a green cell.
    treeline: [0.15, 0.24, 0.12],
    plaza: [0.55, 0.53, 0.48],
    /// The steps and colonnade line around a plaza, and the quay where a built
    /// cell meets water: both civic stone a shade darker than the plaza.
    plazaEdge: [0.44, 0.42, 0.38],
    quay: [0.58, 0.56, 0.50],
    water: [0.13, 0.22, 0.29],
  };

  /// How far below the lowest LAND corner the water plate sits. Bathymetry is
  /// not sourced and must not be invented, so water is a plane 1.5 m under the
  /// shore rather than a modelled depth. It is enough to keep the coast from
  /// z-fighting the land and honest enough to be describable in one line.
  const WATER_DROP = 1.5;
  /// THE KERB. Everything drawn OVER a ground quad — a block's pad inside its
  /// ring of street, a park's tree line, a quay — is lifted this far above the
  /// ground it sits on, so two coplanar quads never fight for the same depth.
  /// One metre, because the host draws the city on a unit sphere in float32
  /// where a radial ulp is about a quarter of a metre and the terrain is
  /// exaggerated three times with the city: a metre is four ulps of clearance
  /// and three metres of drawn height, which is a kerb, not a plinth.
  const KERB = 1.0;
  /// STREET WIDTHS, as a share of the cell. A LOCAL street is the ring around
  /// a block — a quarter of the cell, floored at 7 m so a village lane is
  /// still a lane, capped at 34 m so a superblock is not mostly tarmac. An
  /// ARTERIAL is a whole cell, so it is always wider than a local street by
  /// the cell's own ratio: 4x at 100 m, 12x at 400 m. The LANE is the alley
  /// between two masses on one block, and it is the narrowest of the three.
  const STREET = { local: [0.26, 7, 34], lane: [0.07, 3, 12] };
  /// The strips that give parks, plazas and shores an edge, as a share of the
  /// cell with a floor and a cap in metres.
  const EDGE = { verge: [0.10, 3, 14], quay: [0.12, 4, 16] };
  function streetWidth(cell, spec) { return q(clamp(cell * spec[0], spec[1], spec[2]), 0.1); }

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
  /// the one before it: extent, then boundary, then water, then a capital's
  /// plaza, then roads, then parks, then plots. That is what makes the
  /// coastline authoritative — a mass cannot appear in the sea because the sea
  /// is decided steps earlier — and it is why `land` is asked before anything
  /// is built rather than used to cull afterwards. The plaza sits before the
  /// roads for the same reason in the other direction: the avenues radiate
  /// from it, so it has to exist before they are drawn.
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

        // 3. THE CIVIC PLAZA, and it comes BEFORE the roads. A capital's
        //    plaza is nine cells at the core and the avenues radiate FROM it:
        //    decided the other way round, the cross and the two diagonals
        //    between them covered all nine cells at close detail and the
        //    plaza never existed — only the landmark survived, by overwriting
        //    the centre road cell. Measured on the first render of every
        //    capital, and the reason this step is out of the "roads first"
        //    order the rest of the file keeps.
        //    The PRECINCT is the two rings outside the plaza: whatever is not
        //    road or water there is built, civic stone, and one mass to the
        //    cell, so the plaza sits in a block of pale government fabric
        //    rather than in the same grain as the suburbs.
        const precinct = capital && span >= 25 ? Math.max(Math.abs(di), Math.abs(dj)) : 99;
        if (precinct <= 1) {
          cls[idx] = CLS.PARK; flag[idx] |= FLAG.CIVIC; counts.park += 1; continue;
        }

        // 4. ROADS. The arterial grid, plus a ring and two diagonal avenues
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

        // 5. OPEN SPACE. A noise field of its own, so parks come in patches
        //    rather than as salt. Not inside a capital's precinct, which is
        //    built by definition.
        if (precinct > 3 && noise(seed, i, j, 3.0, 21) > 0.52) {
          cls[idx] = CLS.PARK; counts.park += 1; continue;
        }

        // 6. PLOTS. Density falls with urbanity, so the edge thins into open
        //    ground instead of stopping at a wall of buildings.
        const chance = clamp(density * (0.34 + 0.72 * clamp(u, 0, 1)), 0, 0.97);
        if (precinct > 3 && askUnit(cs, 31) >= chance) {
          cls[idx] = CLS.OPEN; counts.open += 1; continue;
        }

        cls[idx] = CLS.PLOT; counts.plot += 1;

        // The precinct: civic stone, a uniform civic height a little under
        // half the core scale, no tower, no pitch, one mass. Decided here and
        // not in the lot pass so the map level sees the same precinct.
        if (precinct <= 3) {
          flag[idx] |= FLAG.CIVIC;
          hgt[idx] = q(clamp(peak * askSpan(cs, 39, 0.36, 0.52), 5, 320), 0.1);
          if (hgt[idx] > tallest) tallest = hgt[idx];
          continue;
        }

        // 7. HEIGHT. A quadratic falloff from the core with a floor, times a
        //    DISTRICT term, times a per-mass variation. Quadratic rather than
        //    exponential for the determinism reason at the top of the file,
        //    and it happens to be the better shape anyway: a city's height
        //    profile is flatter in the middle and steeper at the edge than an
        //    exponential is.
        //    THE DISTRICT TERM is what keeps the profile from being a smooth
        //    cone: lattice noise six cells wide, plus or minus thirty per
        //    cent, so heights come in neighbourhoods — a taller quarter here,
        //    a low one there — the way a city's skyline steps rather than
        //    slopes. It is smooth, so neighbouring blocks agree; the per-mass
        //    spread on top of it is what keeps them from matching, and the
        //    check measures that neighbours agree more than chance would.
        const profile = 0.15 + 0.85 * (1 - clamp(t, 0, 1)) * (1 - clamp(t, 0, 1));
        const district = 1 + 0.30 * noise(seed, i, j, 6.0, 71);
        let h = peak * profile * district * askSpan(cs, 33, 0.58, 1.32);
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
        // 1.6x the core scale: over the precinct around it (0.36-0.52x) and
        // over any plain fabric (1.6x at most), under the tallest tower a
        // commercial core can raise (2.6x). A landmark is the most DISTINCT
        // thing in a capital, not necessarily the tallest, and the silhouette
        // it is drawn with below is what makes it read.
        hgt[idx] = q(clamp(peak * 1.6, 14, 320), 0.1);
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

  /// Ground height at a fraction (fx, fz) of the way across cell (i, j),
  /// bilinear in the four shared corners. The strips and pads drawn over a
  /// ground quad take their heights from this so they follow the slope the
  /// quad follows; across the width of a strip the difference between the
  /// bilinear surface and the quad's two triangles is far under the kerb.
  function groundAt(p, i, j, fx, fz) {
    const { cornerN, corner } = p;
    const y00 = corner[j * cornerN + i], y10 = corner[j * cornerN + i + 1];
    const y01 = corner[(j + 1) * cornerN + i], y11 = corner[(j + 1) * cornerN + i + 1];
    const top = y00 + (y10 - y00) * fx, bot = y01 + (y11 - y01) * fx;
    return q(top + (bot - top) * fz, 0.01);
  }
  /// A quad over part of a cell — the rectangle [fx0, fx1] x [fz0, fz1] in
  /// cell fractions — lifted a kerb above the ground. Wound the same way as
  /// groundQuad so its normal is +Y.
  function overlayQuad(b, p, i, j, fx0, fx1, fz0, fz1, lift, base, mat) {
    const { cell, half } = p;
    const ox = (i - half - 0.5) * cell, oz = (j - half - 0.5) * cell;
    const x0 = q(ox + fx0 * cell, 0.1), x1 = q(ox + fx1 * cell, 0.1);
    const z0 = q(oz + fz0 * cell, 0.1), z1 = q(oz + fz1 * cell, 0.1);
    b.quad(
      [x0, groundAt(p, i, j, fx0, fz0) + lift, z0], [x0, groundAt(p, i, j, fx0, fz1) + lift, z1],
      [x1, groundAt(p, i, j, fx1, fz1) + lift, z1], [x1, groundAt(p, i, j, fx1, fz0) + lift, z0],
      base, mat,
    );
  }
  /// A strip of `width` metres along one side of a cell: 0 east, 1 south,
  /// 2 west, 3 north. The park edge, the plaza edge and the quay are all this.
  function edgeStrip(b, p, i, j, side, width, base, mat) {
    const f = Math.min(0.5, width / p.cell);
    if (side === 0) overlayQuad(b, p, i, j, 1 - f, 1, 0, 1, KERB, base, mat);
    else if (side === 1) overlayQuad(b, p, i, j, 0, 1, 1 - f, 1, KERB, base, mat);
    else if (side === 2) overlayQuad(b, p, i, j, 0, f, 0, 1, KERB, base, mat);
    else overlayQuad(b, p, i, j, 0, 1, 0, f, KERB, base, mat);
  }
  /// The class of the 4-neighbour on `side`, or OUT beyond the grid.
  function neighbour(p, i, j, side) {
    const ni = i + (side === 0 ? 1 : side === 2 ? -1 : 0);
    const nj = j + (side === 1 ? 1 : side === 3 ? -1 : 0);
    if (ni < 0 || nj < 0 || ni >= p.span || nj >= p.span) return CLS.OUT;
    return p.cls[nj * p.span + ni];
  }
  function neighbourFlag(p, i, j, side) {
    const ni = i + (side === 0 ? 1 : side === 2 ? -1 : 0);
    const nj = j + (side === 1 ? 1 : side === 3 ? -1 : 0);
    if (ni < 0 || nj < 0 || ni >= p.span || nj >= p.span) return 0;
    return p.flag[nj * p.span + ni];
  }
  /// Whether an open cell is drawn with the street ring round it: only when
  /// it has a built or road neighbour, so the local grid runs through the
  /// vacancies inside a city and dies out where the city does.
  function ringed(p, i, j) {
    for (let s = 0; s < 4; s += 1) {
      const c = neighbour(p, i, j, s);
      if (c === CLS.PLOT || c === CLS.ROAD) return true;
    }
    return false;
  }

  /// The mass a plot carries when it carries ONE. Footprint is the cell inset
  /// by a street, then varied per plot so a block is not a row of identical
  /// stamps; base is the LOWEST of the cell's four ground corners, so on a
  /// slope a mass digs into the hill rather than floating over it. Towers,
  /// the landmark, the civic precinct, every cell at map detail and every
  /// cell too small to divide come through here.
  function massFor(p, i, j) {
    const { cell, half, cornerN, corner } = p;
    const idx = j * p.span + i;
    const cs = cellSeed(p.seed, i, j);
    const street = streetWidth(cell, STREET.local);
    const room = cell - street;
    const w = q(room * askSpan(cs, 41, 0.80, 1.0), 0.1);
    const d = q(room * askSpan(cs, 43, 0.80, 1.0), 0.1);
    const cx = (i - half) * cell + q((room - w) * (askUnit(cs, 45) - 0.5), 0.1);
    const cz = (j - half) * cell + q((room - d) * (askUnit(cs, 47) - 0.5), 0.1);
    const base = Math.min(
      corner[j * cornerN + i], corner[j * cornerN + i + 1],
      corner[(j + 1) * cornerN + i], corner[(j + 1) * cornerN + i + 1],
    );
    const h = p.hgt[idx];
    return {
      x0: q(cx - w / 2, 0.1), x1: q(cx + w / 2, 0.1),
      z0: q(cz - d / 2, 0.1), z1: q(cz + d / 2, 0.1),
      y0: q(base - 0.4, 0.05), y1: q(base + h, 0.05),
      seed: cs, flag: p.flag[idx], front: true,
      pitched: p.lod === "close" && (p.flag[idx] & FLAG.PITCHED) !== 0 && h <= 16,
    };
  }

  // ------------------------------------------------------------------- lots
  /// HOW A BUILT CELL IS DIVIDED, at close detail. One cell, one mass is the
  /// unit the budget was written in and it is still the map level's; at close
  /// detail a cell is a city block, and a block is not one building. It is two
  /// to four, of different footprints and heights, with an alley between them
  /// and the biggest of them fronting the main road — which is what makes a
  /// block read as buildings from above rather than as a tile.
  ///
  /// HOW MANY, from the cell's size in metres and nothing else: a block under
  /// 260 m holds two, a superblock over it three, each spread one either way
  /// by hash so a street is not a row of pairs — one to three on a block, two
  /// to four on a superblock. Below LOT.minCell the cell is a single plot with
  /// its lane — a town of ten thousand drawn at 41 m — and dividing it would
  /// draw sheds. The counts are deliberately LOW for what a real block holds,
  /// and the reason is the screen, not the budget: the map draws a cell at
  /// about 4.5 px and the card at 3, so a fourth mass on a 200 m cell is
  /// geometry nobody can resolve. A first cut at 2/3/4 by size measured
  /// 83,082 triangles for Sao Paulo at default span, for no visible return
  /// over 2/3.
  ///
  /// WHICH IS BIGGEST, and where. The FRONT lot faces the arterial: it takes
  /// 58-68% of the block's depth, sits on its street line and shrinks least,
  /// so it is the largest mass on the block whatever the hashes say about the
  /// others — guaranteed by arithmetic, not by luck. Two lots: the front is
  /// at least 0.58 x 0.90^2 = 0.47 of the block against at most
  /// 0.42 x 0.88^2 = 0.33 for the back. Four lots, the tight case, the front
  /// row is split across at 40-60% with a lane out of it: the smaller front
  /// half is at least 0.58 x 0.465 x 0.81 = 0.218 against a back half of at
  /// most 0.42 x 0.60 x 0.774 = 0.195. The check re-measures this rather
  /// than trusting it. The arterial side is whichever 4-neighbour is a road,
  /// chosen by hash when several are; a block with no road beside it fronts
  /// a hashed side.
  ///
  /// HOW TALL. Each lot takes the cell's height times its own spread — the
  /// front 0.95-1.25, the rest 0.55-1.0 — and one lot in twenty-five inside
  /// the inner two thirds of a city that is not a small town is a TALL
  /// OUTLIER at 1.6-2.2x, which is the one office block on a residential
  /// street. Plain lots are capped at 1.6x the core scale so a town of
  /// twenty-four thousand stays a town; outliers keep the tower cap.
  const LOT = {
    minCell: 55,
    frontShare: [0.58, 0.68], frontShrink: [0.90, 1.0], backShrink: [0.70, 0.88],
    frontRise: [0.95, 1.25], backRise: [0.55, 1.0],
    outlier: { chance: 0.04, rise: [1.6, 2.2], within: 0.65 },
    plainCap: 1.6, outlierCap: 2.6,
  };
  function lotCount(p, cs) {
    const cell = p.cell;
    let n = cell < 260 ? 2 : 3;
    const r = askUnit(cs, 71);
    if (r < 0.22) n -= 1; else if (r > 0.82) n += 1;
    return clamp(n, 1, 4);
  }
  /// lots(plan, i, j) -> the masses cell (i, j) carries, each as the same box
  /// massFor returns: {x0, x1, z0, z1, y0, y1, seed, flag, front, pitched}.
  /// Public, so a check can test the layout without walking a buffer, and pure:
  /// it reads the plan and writes nothing.
  function lotsFor(p, i, j) {
    const { cell, half, span, cls, flag, hgt, seed, peak } = p;
    const idx = j * span + i;
    const f = flag[idx];
    const cs = cellSeed(seed, i, j);
    const single = p.lod !== "close" || cell < LOT.minCell
      || (f & (FLAG.TOWER | FLAG.LANDMARK | FLAG.CIVIC)) !== 0;
    const n = single ? 1 : lotCount(p, cs);
    if (n <= 1) return [massFor(p, i, j)];

    const street = streetWidth(cell, STREET.local);
    const lane = streetWidth(cell, STREET.lane);
    const room = q(cell - street, 0.1);
    // The block's pad: the cell inset by half a street on every side.
    const ox = (i - half) * cell - room / 2, oz = (j - half) * cell - room / 2;
    const base = Math.min(
      p.corner[j * p.cornerN + i], p.corner[j * p.cornerN + i + 1],
      p.corner[(j + 1) * p.cornerN + i], p.corner[(j + 1) * p.cornerN + i + 1],
    );
    const y0 = q(base - 0.4, 0.05);
    const h = hgt[idx];
    const x = (i - half) * cell, z = (j - half) * cell;
    const t = p.radius > 0 ? Math.sqrt(x * x + z * z) / p.radius : 1;
    const outliers = p.character !== "small" && t < LOT.outlier.within;

    // The front: a road neighbour, hashed among several, hashed when none.
    const k = ask(cs, 73) % 4;
    let front = k;
    for (let s = 0; s < 4; s += 1) {
      const side = (k + s) % 4;
      if (neighbour(p, i, j, side) === CLS.ROAD) { front = side; break; }
    }
    // Lot space: `a` runs from the back of the block to its street line at
    // the front, `c` runs across it. Mapped to world by which side is front.
    const map = (a0, a1, c0, c1) => {
      if (front === 0) return [ox + a0, ox + a1, oz + c0, oz + c1];
      if (front === 1) return [ox + c0, ox + c1, oz + a0, oz + a1];
      if (front === 2) return [ox + room - a1, ox + room - a0, oz + c0, oz + c1];
      return [ox + c0, ox + c1, oz + room - a1, oz + room - a0];
    };
    const place = (a0, a1, c0, c1, slot, isFront) => {
      const ls = ask(cs, 100 + slot);
      const shrink = isFront ? LOT.frontShrink : LOT.backShrink;
      const la = a1 - a0, lc = c1 - c0;
      const wa = q(la * askSpan(ls, 41, shrink[0], shrink[1]), 0.1);
      const wc = q(lc * askSpan(ls, 43, shrink[0], shrink[1]), 0.1);
      // The front mass stands ON its street line; a back mass floats in its lot.
      const oa = isFront ? a1 - wa : a0 + q((la - wa) * askUnit(ls, 45), 0.1);
      const oc = c0 + q((lc - wc) * askUnit(ls, 47), 0.1);
      const rise = isFront ? LOT.frontRise : LOT.backRise;
      let hh = h * askSpan(ls, 81, rise[0], rise[1]);
      let cap = LOT.plainCap;
      if (outliers && askUnit(ls, 83) < LOT.outlier.chance) {
        hh *= askSpan(ls, 85, LOT.outlier.rise[0], LOT.outlier.rise[1]);
        cap = LOT.outlierCap;
      }
      hh = q(clamp(hh, 4, peak * cap), 0.1);
      const r = map(oa, oa + wa, oc, oc + wc);
      return {
        x0: q(r[0], 0.1), x1: q(r[1], 0.1), z0: q(r[2], 0.1), z1: q(r[3], 0.1),
        y0, y1: q(base + hh, 0.05),
        seed: ls, flag: f, front: isFront,
        pitched: (f & FLAG.PITCHED) !== 0 && hh <= 16,
      };
    };

    const share = askSpan(cs, 75, LOT.frontShare[0], LOT.frontShare[1]);
    const split = q(room * (1 - share), 0.1);         // where the front lot begins
    const back = split - lane;                         // the back lots' depth
    const out = [];
    if (n === 2) {
      out.push(place(split, room, 0, room, 0, true));
      out.push(place(0, back, 0, room, 1, false));
    } else if (n === 3) {
      const c = q(room * askSpan(cs, 77, 0.40, 0.60), 0.1);
      out.push(place(split, room, 0, room, 0, true));
      out.push(place(0, back, 0, c - lane / 2, 1, false));
      out.push(place(0, back, c + lane / 2, room, 2, false));
    } else {
      const c1 = q(room * askSpan(cs, 77, 0.40, 0.60), 0.1);
      const c2 = q(room * askSpan(cs, 79, 0.40, 0.60), 0.1);
      out.push(place(split, room, 0, c1 - lane / 2, 0, true));
      out.push(place(split, room, c1 + lane / 2, room, 1, true));
      out.push(place(0, back, 0, c2 - lane / 2, 2, false));
      out.push(place(0, back, c2 + lane / 2, room, 3, false));
    }
    return out;
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

    // GROUND, in passes so each is one contiguous part: the hard surfaces,
    // then open space and parks, then water, then the quays. Ground is drawn
    // under the masses as well as between them — a mass is inset by a street,
    // so most of the plot's own ground is visible from above anyway, and the
    // saving from skipping it would be a rounding error against the mass.
    //
    // THE LOCAL STREET GRID is drawn here, at close detail, and it is what
    // makes the road hierarchy read: a built cell's ground is a whole cell of
    // carriageway with the block's PAD lifted a kerb above it, inset by half
    // a street on every side, so the ring between two neighbouring pads is a
    // street one `local` wide and continuous across the city, while an
    // arterial is a whole cell of darker road. An open cell gets the ring only
    // when it has a built or road neighbour, so the grid runs through the
    // vacancies inside the city and stops where the city does. Four triangles
    // a built cell instead of two; the map level keeps the two.
    const local = streetWidth(p.cell, STREET.local);
    const padInset = Math.min(0.45, (local / 2) / p.cell);
    b.part("ground / streets, plots and open ground", "ground", () => {
      for (let j = 0; j < span; j += 1) {
        for (let i = 0; i < span; i += 1) {
          const c = cls[j * span + i];
          if (c !== CLS.ROAD && c !== CLS.PLOT && c !== CLS.OPEN) continue;
          const mat = 0.9 + 0.2 * askUnit(cellSeed(p.seed, i, j), 61);
          if (c === CLS.ROAD || !close || (c === CLS.OPEN && !ringed(p, i, j))) {
            groundQuad(b, p, i, j, c === CLS.ROAD ? GROUND.road : c === CLS.PLOT ? GROUND.plot : GROUND.open, mat);
            continue;
          }
          groundQuad(b, p, i, j, GROUND.lane, mat);
          overlayQuad(b, p, i, j, padInset, 1 - padInset, padInset, 1 - padInset, KERB,
            c === CLS.PLOT ? GROUND.plot : GROUND.open, mat);
        }
      }
    });
    // PARKS WITH AN EDGE. At close detail a park cell gets a tree line along
    // every side that does not face another park, so a park of six cells is
    // one outlined shape rather than six green tiles; the plaza gets a line of
    // steps along every side that does not face more plaza — which includes
    // the four sides facing the landmark in its centre, so the monument stands
    // in a ring of stone. Two triangles a side, and only on the sides that
    // are edges.
    const verge = streetWidth(p.cell, EDGE.verge);
    b.part("ground / parks and civic plaza", "park", () => {
      for (let j = 0; j < span; j += 1) {
        for (let i = 0; i < span; i += 1) {
          const idx = j * span + i;
          if (cls[idx] !== CLS.PARK) continue;
          const civic = (flag[idx] & FLAG.CIVIC) !== 0;
          const mat = 0.92 + 0.16 * askUnit(cellSeed(p.seed, i, j), 63);
          groundQuad(b, p, i, j, civic ? GROUND.plaza : GROUND.park, mat);
          if (!close) continue;
          for (let s = 0; s < 4; s += 1) {
            const nc = neighbour(p, i, j, s);
            const same = nc === CLS.PARK && ((neighbourFlag(p, i, j, s) & FLAG.CIVIC) !== 0) === civic;
            if (same) continue;
            edgeStrip(b, p, i, j, s, verge, civic ? GROUND.plazaEdge : GROUND.treeline, mat);
          }
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
      // THE WATERFRONT. Where a land cell meets a water cell it gets a quay
      // along the shared side: a strip of stone on the land, a kerb up, so the
      // coast is a hard line rather than a colour change. The land mask
      // decided where the water is; this only draws the edge it already made.
      if (close) {
        const quay = streetWidth(p.cell, EDGE.quay);
        b.part("ground / quays", "quay", () => {
          for (let j = 0; j < span; j += 1) {
            for (let i = 0; i < span; i += 1) {
              const c = cls[j * span + i];
              if (c === CLS.OUT || c === CLS.WATER) continue;
              const mat = 0.94 + 0.12 * askUnit(cellSeed(p.seed, i, j), 67);
              for (let s = 0; s < 4; s += 1) {
                if (neighbour(p, i, j, s) !== CLS.WATER) continue;
                edgeStrip(b, p, i, j, s, quay, GROUND.quay, mat);
              }
            }
          }
        });
      }
    }

    // MASSING, in three passes for the same reason. Fabric first — everything
    // that is not a tower and not the landmark — then the towers, then the one
    // landmark, so a host can highlight a city's core without walking the
    // whole buffer. A built cell carries the masses `lots` gives it: one at
    // map detail, one to four at close.
    let masses = 0;
    b.part("massing / " + p.character + " fabric", "fabric", () => {
      for (let j = 0; j < span; j += 1) {
        for (let i = 0; i < span; i += 1) {
          const idx = j * span + i;
          if (cls[idx] !== CLS.PLOT || (flag[idx] & (FLAG.TOWER | FLAG.LANDMARK))) continue;
          const lots = lotsFor(p, i, j);
          for (let k = 0; k < lots.length; k += 1) {
            const m = lots[k];
            const col = fabricColour(p, m.seed, m.flag);
            const mat = askSpan(m.seed, 55, 0.86, 1.12);
            masses += 1;
            if (m.pitched) {
              boxRidge(b, m.x0, m.x1, m.z0, m.z1, m.y0, m.y1, col.wall, col.roof, mat,
                q(2.0 + 2.5 * askUnit(m.seed, 57), 0.1));
            } else {
              boxTop(b, m.x0, m.x1, m.z0, m.z1, m.y0, m.y1, col.wall, col.roof, mat);
            }
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
            masses += 1;
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
        masses += 1;
        // A broad plinth, a hall on it, a tower out of the hall and a spire
        // on the tower: four stages, each narrower than the one below, which
        // is the silhouette that says "the monument" from any angle — a
        // parliament, a cathedral, a column — without naming one. Forty
        // triangles at close detail, twenty at map, for the one thing on the
        // card a player will look for, and the only piece of geometry in the
        // file that is not a cell of city.
        const H = m.y1 - m.y0;
        const plinth = q(m.y0 + H * 0.08, 0.05);
        const hall = q(m.y0 + H * 0.32, 0.05);
        const tower = q(m.y0 + H * 0.80, 0.05);
        const w = (m.x1 - m.x0), d = (m.z1 - m.z0);
        const inset = (f) => [q(w * f, 0.1), q(d * f, 0.1)];
        const [hx, hz] = inset(0.10), [tx, tz] = inset(0.27), [sx, sz] = inset(0.40);
        boxTop(b, m.x0, m.x1, m.z0, m.z1, m.y0, plinth, CIVIC.wall, CIVIC.roof, 1.0);
        boxTop(b, m.x0 + hx, m.x1 - hx, m.z0 + hz, m.z1 - hz, plinth, hall, CIVIC.wall, CIVIC.roof, 1.05);
        if (close) {
          boxTop(b, m.x0 + tx, m.x1 - tx, m.z0 + tz, m.z1 - tz, hall, tower, CIVIC.wall, CIVIC.roof, 1.1);
          boxTop(b, m.x0 + sx, m.x1 - sx, m.z0 + sz, m.z1 - sz, tower, m.y1, CIVIC.wall, CIVIC.roof, 1.15);
        } else {
          boxTop(b, m.x0 + tx, m.x1 - tx, m.z0 + tz, m.z1 - tz, hall, m.y1, CIVIC.wall, CIVIC.roof, 1.1);
        }
      });
    }

    return b.finish(describe(p, masses), meta(p, masses));
  }

  /// The metadata a card, a check or a debug overlay reads. Everything here is
  /// either transcribed from the record or derived from the declared models,
  /// and the two are kept in separate fields on purpose: `pop` is the record's
  /// own number, `modelAreaKm2` is the model's.
  function meta(p, masses) {
    return {
      city: { name: p.name, lon: p.lon, lat: p.lat, pop: p.pop, rank: p.rank, capital: p.capital },
      lod: p.lod, seed: p.seed,
      span: p.span, cell: p.cell,
      extent: [q(p.span * p.cell, 0.5), q(p.span * p.cell, 0.5)],
      modelExtent: p.extent,
      character: p.character, layout: p.layout, arterialEvery: p.every,
      /// The three ranks of road, in metres: an arterial is a whole cell, a
      /// local street is the ring round a block, a lane is the alley between
      /// two masses on one block. Always in that order of width.
      streets: {
        arterial: p.cell,
        local: streetWidth(p.cell, STREET.local),
        lane: streetWidth(p.cell, STREET.lane),
      },
      /// Masses drawn against built cells: one to one at map detail, one to
      /// four at close. `cells.plot` is the blocks; this is the buildings.
      masses,
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
  function describe(p, masses) {
    // Formatted, not rounded: `q(12.1, 0.01)` is 12.100000000000001 in a
    // double and a description is read by a person.
    const km = (v) => (v >= 100 ? String(Math.round(v)) : v >= 10 ? v.toFixed(1) : v.toFixed(2));
    const name = p.name || "an unnamed place";
    const head = "Original game art: birds-eye massing of " + name + " — "
      + masses.toLocaleString("en-US") + " masses on " + p.counts.plot.toLocaleString("en-US")
      + " built cells of a " + p.span + " x " + p.span
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
    /// lots(plan, i, j): the masses a cell carries, as boxes in model space.
    /// Pure, and the same answer build drew.
    lots: lotsFor,
    areaKm2, extentMetres, peakHeight, character,
    lot: Object.freeze({ minCell: LOT.minCell, max: 4 }),
    kerb: KERB,
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
      ceiling: Object.freeze({ close: 80000, map: 5000 }),
      floor: Object.freeze({ close: 250, map: 100 }),
      largest: Object.freeze({ overPop: 2000000, close: [44000, 80000] }),
    }),
    baseCell: Object.freeze(Object.assign({}, BASE_CELL)),
    spanLimits: Object.freeze({ close: Object.assign({}, SPAN.close), map: Object.assign({}, SPAN.map) }),
    populationFloor: POP_FLOOR,
    metresPerDegree: (lat) => ({ lon: lonMetres(lat), lat: M_LAT }),
    version: "urban.city_massing.v1",
    kit: "city_massing",
  });
});
