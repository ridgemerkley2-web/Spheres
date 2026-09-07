#!/usr/bin/env node
// A CITY HAS TO LAND IN THE SAME SPACE AS THE GROUND UNDER IT.
//
// city-layer.js moves a city-mesh from its own model space (metres, +X east,
// +Y up, +Z south, seated at Y = 0) into the sphere space the globe's camera
// works in. That space is defined in exactly one place — terrain-surface.js
// builds its vertices as
//
//     r = 1 + max(0, height) * exaggeration / EARTH_METRES
//     (r*cos(lat)*sin(lon), r*sin(lat), r*cos(lat)*cos(lon))
//
// — and if the two ever disagree the city sinks into the terrain or floats over
// it. So this file does not restate the formula from the module under test: it
// reads the CONVENTION out of terrain-surface.js's own source and checks the
// placement against it.
"use strict";

const test = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const path = require("path");

const ROOT = path.join(__dirname, "..", "..");
const ui = (f) => path.join(ROOT, "spheres-web", "ui", f);
const CityLayer = require(ui("city-layer.js"));
const CityMesh = require(ui("city-mesh.js"));

const DEG = Math.PI / 180;
const R = 6371000;
const EXAG = 3;

const CITY = { name: "Denver", lon: -104.99, lat: 39.74, pop: 1900000, rank: 2, capital: false };

/// TERRAIN THAT VARIES, and it has to. A constant height leaves city-mesh's
/// `relief` at [0, 0], which makes the relief floor in baseElevation() a no-op
/// and lets a placement that ignores it pass every bar in this file. That is
/// exactly what happened: a sabotage that read `datum` alone and dropped the
/// floor -- which buries a city by the depth of the deepest valley in it --
/// went green until this fixture started giving the ground a shape.
const HILLY = (lon, lat) => 1600 + 420 * Math.sin(lon * 37) + 260 * Math.cos(lat * 51);
const COASTAL = { name: "Harbour", lon: 151.21, lat: -33.87, pop: 4630000, rank: 2, capital: false };

/// The base elevation, derived from the MESH rather than from the module under
/// test. city-mesh seats every mesh so its lowest point is Y = 0 and reports
/// the terrain it baked as `datum` plus a `relief` range about it, so Y = 0 is
/// at datum + relief[0]. Calling CityLayer.baseElevation() here instead would
/// make every bar below self-referential: a sabotage that dropped the relief
/// floor -- which buries a city by the depth of the deepest valley in it --
/// passed this whole file until the reference stopped coming from the code it
/// was checking. tools/ui/check_road_mesh.cjs records the same trap.
function expectedBase(mesh) {
  return mesh.datum + mesh.relief[0];
}

/// The reference transform, written from the terrain module's own numbers.
function sphere(lonDeg, latDeg, height) {
  const lon = lonDeg * DEG, lat = latDeg * DEG;
  const r = 1 + Math.max(0, height) * EXAG / R;
  return [r * Math.cos(lat) * Math.sin(lon), r * Math.sin(lat), r * Math.cos(lat) * Math.cos(lon)];
}

test("the constants are still the terrain surface's own, read from its source", () => {
  // If terrain-surface changes its exaggeration or its earth radius, the city
  // silently ends up at a different scale from the ground. Read them rather
  // than trust them.
  const src = fs.readFileSync(ui("terrain-surface.js"), "utf8");
  assert.match(src, /EARTH_METRES = 6371000/, "the terrain surface changed its earth radius");
  assert.match(src, /exaggeration = options\.exaggeration \?\? 3/,
    "the terrain surface changed its default exaggeration");
  assert.match(src, /r \* cosLat \* Math\.sin\(lon\)/,
    "the terrain surface changed how it builds a sphere vertex");
  assert.equal(CityLayer.EARTH_METRES, 6371000, "city-layer disagrees about the earth radius");
  assert.equal(CityLayer.EXAGGERATION, 3, "city-layer disagrees about the exaggeration");
});

test("every vertex lands where the terrain formula says it should", () => {
  const mesh = CityMesh.build(CITY, { height: HILLY });
  const placed = CityLayer.place(mesh, CITY);
  assert.ok(placed && placed.count > 1000, "nothing was placed");
  const base = expectedBase(mesh);

  let worst = 0, worstAt = null;
  const step = Math.max(1, Math.floor(placed.count / 4000));
  for (let i = 0; i < placed.count; i += step) {
    const p = i * 3;
    const X = mesh.positions[p], Y = mesh.positions[p + 1], Z = mesh.positions[p + 2];
    const lon = CITY.lon + (X / (R * Math.cos(CITY.lat * DEG))) / DEG;
    const lat = CITY.lat - (Z / R) / DEG;
    const want = sphere(lon, lat, base + Y);
    const got = [placed.positions[p], placed.positions[p + 1], placed.positions[p + 2]];
    const err = Math.hypot(got[0] - want[0], got[1] - want[1], got[2] - want[2]) * R;
    if (err > worst) { worst = err; worstAt = { X, Y, Z }; }
  }
  // A metre is float32 noise at this radius; anything structural is far larger.
  assert.ok(worst < 1.0,
    `worst vertex is ${worst.toFixed(2)} m from the terrain convention at ${JSON.stringify(worstAt)}`);
});

test("the city sits ON its ground, not through it and not above it", () => {
  const mesh = CityMesh.build(CITY, { height: HILLY });
  const placed = CityLayer.place(mesh, CITY);
  const base = expectedBase(mesh);
  let lo = Infinity, hi = -Infinity;
  for (let i = 0; i < placed.count; i += 1) {
    const p = i * 3;
    const r = Math.hypot(placed.positions[p], placed.positions[p + 1], placed.positions[p + 2]);
    lo = Math.min(lo, r); hi = Math.max(hi, r);
  }
  const loM = (lo - 1) * R, hiM = (hi - 1) * R;
  assert.ok(Math.abs(loM - Math.max(0, base) * EXAG) < 2,
    `the base of the city is at ${loM.toFixed(0)} m, not the ${(base * EXAG).toFixed(0)} m its ground is drawn at`);
  assert.ok(Math.abs(hiM - (base + mesh.bounds.max[1]) * EXAG) < 3,
    `the top of the city is at ${hiM.toFixed(0)} m, not the ${((base + mesh.bounds.max[1]) * EXAG).toFixed(0)} m expected`);
  // AND the exaggeration is really applied: at 1x these two would be equal.
  assert.ok(hiM > (base + mesh.bounds.max[1]) * 1.5,
    "the city is not being exaggerated with the ground it stands on");
});

test("+Z is south, and getting it backwards is caught", () => {
  // The single easiest thing to get wrong, and it mirrors the whole city
  // north-south without changing a bounding box or a triangle count.
  const mesh = CityMesh.build(CITY, { height: HILLY });
  const placed = CityLayer.place(mesh, CITY);
  let southMost = null, southZ = -Infinity;
  for (let i = 0; i < placed.count; i += 1) {
    const p = i * 3;
    if (mesh.positions[p + 2] > southZ) { southZ = mesh.positions[p + 2]; southMost = p; }
  }
  // Its latitude must be LOWER than the city's own.
  const y = placed.positions[southMost + 1];
  const r = Math.hypot(placed.positions[southMost], y, placed.positions[southMost + 2]);
  const lat = Math.asin(y / r) / DEG;
  assert.ok(lat < CITY.lat,
    `the +Z end of the mesh landed at ${lat.toFixed(4)} deg, north of the city at ${CITY.lat} — +Z is south`);
});

test("normals come out unit length and pointing away from the ground", () => {
  const mesh = CityMesh.build(CITY, { height: HILLY });
  const placed = CityLayer.place(mesh, CITY);
  let bad = 0, skyward = 0, n = 0;
  for (let i = 0; i < placed.count; i += 7) {
    const p = i * 3;
    const len = Math.hypot(placed.normals[p], placed.normals[p + 1], placed.normals[p + 2]);
    if (Math.abs(len - 1) > 1e-3) bad += 1;
    const r = Math.hypot(placed.positions[p], placed.positions[p + 1], placed.positions[p + 2]);
    const dot = (placed.normals[p] * placed.positions[p] + placed.normals[p + 1] * placed.positions[p + 1]
      + placed.normals[p + 2] * placed.positions[p + 2]) / r;
    if (dot > 0.5) skyward += 1;
    n += 1;
  }
  assert.equal(bad, 0, `${bad} normals are not unit length`);
  // A city seen from above is mostly roofs and ground, so most faces look up.
  assert.ok(skyward / n > 0.3, `only ${(100 * skyward / n).toFixed(0)}% of faces point skyward`);
});

test("selection is a screen question, and the budget is a hard stop", () => {
  const cities = [
    { name: "Huge", lon: 0, lat: 0, pop: 20000000 },
    { name: "Big", lon: 0.1, lat: 0.1, pop: 5000000 },
    { name: "Mid", lon: 0.2, lat: 0.2, pop: 900000 },
    { name: "Small", lon: 0.3, lat: 0.3, pop: 60000 },
    { name: "Elsewhere", lon: 120, lat: 40, pop: 20000000 },
  ];
  const box = { west: -1, east: 1, south: -1, north: 1 };
  const close = CityLayer.select(cities, box, { mPerPx: 20, minPx: 110, budget: 4 });
  assert.ok(close.length > 0, "nothing selected at 20 m/px");
  assert.ok(!close.some((h) => h.city.name === "Elsewhere"), "a city outside the window was selected");
  // Ordered biggest-on-screen first, so a budget that bites drops the least.
  for (let i = 1; i < close.length; i += 1) {
    assert.ok(close[i].screenPx <= close[i - 1].screenPx, "selection is not ordered by screen size");
  }
  assert.ok(CityLayer.select(cities, box, { mPerPx: 20, minPx: 110, budget: 2 }).length <= 2,
    "the budget did not bite");
  // Far out, nothing is worth drawing — this is what keeps world zoom free.
  assert.equal(CityLayer.select(cities, box, { mPerPx: 5000, minPx: 110, budget: 4 }).length, 0,
    "cities were selected at 5 km per pixel, where none of them covers 110 px");
});

test("a bad record is refused rather than placed at the origin", () => {
  const mesh = CityMesh.build(CITY, { height: HILLY });
  assert.equal(CityLayer.place(mesh, { name: "?", lon: NaN, lat: 10 }), null);
  assert.equal(CityLayer.place(mesh, { name: "?", lon: 10, lat: undefined }), null);
  assert.equal(CityLayer.place(null, CITY), null);
});

test("the area model matches the one city-mesh declares", () => {
  // The selector sizes a city WITHOUT building it, so it carries a copy of
  // city-mesh's area curve. A copy that drifts would select the wrong cities.
  const src = fs.readFileSync(ui("city-mesh.js"), "utf8");
  assert.match(src, /40 \* \(pop \/ 1e6\) ?\^ ?0\.85|A_km2 = 40/,
    "city-mesh's declared area model moved; city-layer.extentFor still copies the old one");
  for (const pop of [50000, 1e6, 8e6, 3e7]) {
    const km2 = 40 * Math.pow(pop / 1e6, 0.85);
    assert.ok(Math.abs(CityLayer.extentFor(pop) - Math.sqrt(km2) * 1000) < 1,
      `extentFor(${pop}) does not follow the declared curve`);
  }
});

test("a coastal city loses ground to the sea when a land sampler says so", () => {
  const dry = CityMesh.build(COASTAL, { height: () => 0 });
  const wet = CityMesh.build(COASTAL, { height: () => 0, land: (lon) => lon < COASTAL.lon });
  const a = CityLayer.place(dry, COASTAL), b = CityLayer.place(wet, COASTAL);
  assert.ok(a && b);
  assert.ok(b.count < a.count,
    "the land mask removed nothing: a city that should stop at the water did not");
});
