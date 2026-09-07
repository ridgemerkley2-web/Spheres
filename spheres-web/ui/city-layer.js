/// PUTTING A CITY ON THE GLOBE.
///
/// city-mesh.js builds a birds-eye city in its own model space — metres, +X
/// east, +Y up, +Z south, seated so the lowest ground is Y = 0. This turns that
/// into the SAME sphere space the terrain mesh already lives in, so a city can
/// be drawn by the globe's own camera alongside the ground it stands on rather
/// than composited on top of it afterwards.
///
/// The convention is copied from terrain-surface.js and must stay copied:
///
///     r   = 1 + max(0, elevation) * exaggeration / EARTH_METRES
///     pos = r * (cos(lat) * sin(lon), sin(lat), cos(lat) * cos(lon))
///
/// with lon and lat in RADIANS. Anything that disagrees with that puts the city
/// through the ground or floating above it, so check_city_layer.cjs asserts the
/// two agree by building both and comparing.
///
/// THE EXAGGERATION APPLIES TO THE CITY TOO, and that is a deliberate choice
/// rather than an oversight. The terrain around it is drawn at 3x — the legend
/// says so — and a city drawn at 1x on ground drawn at 3x would sit in a
/// landscape three times too big for it. Consistency with the surface it stands
/// on is worth more than a true building height nobody can check by eye, and
/// the disclosure the terrain already carries covers both.
///
/// WHAT THIS MODULE IS NOT. It invents nothing. Every metre it moves comes from
/// the mesh city-mesh.js already built and the elevation the host sampled; it is
/// a change of coordinates and nothing else.
(function (root) {
  "use strict";

  const DEG = Math.PI / 180;
  const EARTH_METRES = 6371000;      // matches terrain-surface.js exactly
  const EXAGGERATION = 3;            // matches the terrain surface's default

  /// Where a city's model-space Y = 0 sits in real metres above the datum.
  ///
  /// city-mesh seats every mesh so its LOWEST point is Y = 0, and reports the
  /// terrain it baked as `datum` plus a `relief` range about it. So the ground
  /// under Y = 0 is the datum plus the BOTTOM of that range, and a vertex at Y
  /// is that much higher again. Reading `datum` alone would bury a city with a
  /// valley in it by however deep the valley goes.
  function baseElevation(mesh) {
    const datum = Number(mesh && mesh.datum);
    const relief = mesh && mesh.relief;
    const low = Array.isArray(relief) && Number.isFinite(relief[0]) ? relief[0] : 0;
    return (Number.isFinite(datum) ? datum : 0) + low;
  }

  /// Turn one city mesh into sphere-space buffers.
  ///
  /// Returns positions and normals in the globe's own frame plus the mesh's
  /// colours untouched, ready to hand straight to a vertex buffer.
  function place(mesh, city, options) {
    if (!mesh || !mesh.positions || !mesh.positions.length) return null;
    const o = options || {};
    const R = Number.isFinite(o.earthMetres) ? o.earthMetres : EARTH_METRES;
    const exaggeration = Number.isFinite(o.exaggeration) ? o.exaggeration : EXAGGERATION;
    const lon0 = Number(city && city.lon) * DEG;
    const lat0 = Number(city && city.lat) * DEG;
    if (!Number.isFinite(lon0) || !Number.isFinite(lat0)) return null;

    const base = baseElevation(mesh);
    // Metres per radian. cos(lat) collapses at the pole, so the east scale is
    // floored: a city at 89 degrees would otherwise be stretched across a
    // quarter of the world.
    const eastPerRad = R * Math.max(Math.cos(lat0), 0.02);

    // The local frame, taken once at the city's own origin. Over 32 km — the
    // widest city in the table — the frame turns by 0.3 of a degree, which is
    // below what a normal can show, and doing it per vertex would cost three
    // trig calls each for that.
    const sinLon0 = Math.sin(lon0), cosLon0 = Math.cos(lon0);
    const sinLat0 = Math.sin(lat0), cosLat0 = Math.cos(lat0);
    const east  = [cosLon0, 0, -sinLon0];
    const up    = [cosLat0 * sinLon0, sinLat0, cosLat0 * cosLon0];
    const north = [-sinLat0 * sinLon0, cosLat0, -sinLat0 * cosLon0];

    const src = mesh.positions, srcN = mesh.normals;
    const n = src.length / 3;
    const positions = new Float32Array(n * 3);
    const normals = new Float32Array(n * 3);

    for (let i = 0, p = 0; i < n; i += 1, p += 3) {
      const X = src[p], Y = src[p + 1], Z = src[p + 2];
      // POSITION: the offsets are real metres, so they become real angles. A
      // tangent plane would be simpler and would sink the edge of a 32 km city
      // 80 m into the ground, which is more than most of its buildings are tall.
      const lon = lon0 + X / eastPerRad;
      const lat = lat0 - Z / R;                 // +Z is south, so it lowers the latitude
      const elev = base + Y;
      const r = 1 + Math.max(0, elev) * exaggeration / R;
      const cosLat = Math.cos(lat), sinLat = Math.sin(lat);
      positions[p]     = r * cosLat * Math.sin(lon);
      positions[p + 1] = r * sinLat;
      positions[p + 2] = r * cosLat * Math.cos(lon);

      // NORMAL: model space is east/up/south, so south is minus north.
      const nx = srcN ? srcN[p] : 0, ny = srcN ? srcN[p + 1] : 1, nz = srcN ? srcN[p + 2] : 0;
      let wx = nx * east[0] + ny * up[0] - nz * north[0];
      let wy = nx * east[1] + ny * up[1] - nz * north[1];
      let wz = nx * east[2] + ny * up[2] - nz * north[2];
      const len = Math.hypot(wx, wy, wz) || 1;
      normals[p] = wx / len; normals[p + 1] = wy / len; normals[p + 2] = wz / len;
    }

    return {
      positions, normals,
      colors: mesh.colors,
      count: n,
      triangleCount: n / 3,
      city: { name: city.name, lon: city.lon, lat: city.lat, pop: city.pop },
      baseElevation: base,
      exaggeration,
      // The widest the city reaches on the ground, in metres. The caller needs
      // it to decide whether the thing is worth drawing at the current zoom.
      extentMetres: Number(mesh.extentMetres) ||
        (mesh.bounds ? mesh.bounds.max[0] - mesh.bounds.min[0] : 0),
    };
  }

  /// WHICH CITIES ARE WORTH DRAWING, and it is a screen question, not a
  /// population one. A city is worth its buffers when it covers enough pixels
  /// to read as a city; below that the map's own drawn symbol says more for
  /// nothing. The caller passes metres-per-pixel because only it knows the
  /// camera, and the budget is a hard count because each city is tens of
  /// thousands of triangles.
  function select(cities, bounds, options) {
    const o = options || {};
    // mPerPx is a NUMBER or a FUNCTION of the city. It has to be able to be a
    // function: measured once at the view's footprint midpoint it read 29.1 m
    // per pixel for a view whose city sat at 19.6, because the midpoint of a
    // pitched footprint is tilted toward the horizon. That under-sized Chicago
    // by 1.5x -- span 123 asked for where 181 was resolvable -- and it would
    // gate small cities out that were in fact big enough on screen.
    const scale = typeof o.mPerPx === "function" ? o.mPerPx : () => Number(o.mPerPx);
    const minPx = Number.isFinite(o.minPx) ? o.minPx : 90;
    const budget = Number.isFinite(o.budget) ? o.budget : 6;
    if (!Array.isArray(cities) || !bounds) return [];
    const { west, east, south, north } = bounds;
    const out = [];
    for (let i = 0; i < cities.length; i += 1) {
      const c = cities[i];
      if (!c || !Number.isFinite(c.lon) || !Number.isFinite(c.lat)) continue;
      if (c.lat < south || c.lat > north) continue;
      // The window can straddle the antimeridian, in which case west > east.
      const inLon = west <= east
        ? (c.lon >= west && c.lon <= east)
        : (c.lon >= west || c.lon <= east);
      if (!inLon) continue;
      const extent = extentFor(c.pop);
      const mPerPx = Number(scale(c));
      if (!Number.isFinite(mPerPx) || mPerPx <= 0) continue;
      if (extent / mPerPx < minPx) continue;
      out.push({ index: i, city: c, extentMetres: extent, screenPx: extent / mPerPx, mPerPx });
    }
    // Biggest on screen first, so a budget that bites drops the ones that would
    // have said least.
    out.sort((a, b) => b.screenPx - a.screenPx);
    return out.slice(0, budget);
  }

  /// The same area model city-mesh.js declares, and it is here ONLY so the
  /// selector can size a city without building it. It is a REPRESENTATIVE
  /// curve, not a measurement, and city-mesh.js says so at length; if the two
  /// ever disagree the check catches it.
  function extentFor(pop) {
    const p = Math.max(1000, Number(pop) || 1000);
    const km2 = 40 * Math.pow(p / 1e6, 0.85);
    return Math.sqrt(km2) * 1000;
  }

  const api = { place, select, extentFor, baseElevation,
    EARTH_METRES, EXAGGERATION };
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.CityLayer = api;
})(typeof window !== "undefined" ? window : globalThis);
