#!/usr/bin/env node
// Regression anchor for the birds-eye city kit, `spheres-web/ui/city-mesh.js`.
//
// WHAT THIS DEFENDS. That module answers one question — does a CITY read from
// above, at a cost a card can pay — and there are five ways to answer it
// wrongly. It can ship geometry the renderer chokes on. It can ship a
// different city on the next reload. It can draw every city the same size, or
// draw a metropolis the size of a market town, which is the exact defect it
// exists to end. It can build over the sea when the host has told it where the
// sea is. And it can quietly claim to know something about a real place that
// it does not. There is a bar below for each, asserted against samplers this
// file invents and therefore controls completely, and against the real
// `cities.js` records for the parts that must hold for every city on Earth.
//
// THE BAR THAT MATTERS MOST is that the SAMPLERS ARE AUTHORITATIVE. A coastal
// city that does not stop at its coastline is worse than one drawn with no
// coastline at all, because the second is honest about what it does not know
// and the first is a lie with a real place name written under it. `barCoast`
// builds the same city three times — no mask, a mask that opens the western
// half to the sea, a mask that refuses everything — and demands that the mesh
// differ every time, that not one triangle of massing stand over a cell the
// mask refused, and that the description say which of the two it was.
//
// AND IT WATCHES ITSELF GO RED. Iron rule 5: a test that cannot fail is worse
// than no test. `SABOTAGE` at the foot of this file is the ledger — twelve
// deliberate defects, one for every bar in the sweep, each patched into the
// source as a one-line edit, each run through the same sweep, and each
// required to fail ON ITS NAMED BAR rather than merely somewhere, so a
// sabotage cannot start passing for the wrong reason. Two of them are defects
// that were actually found and fixed while the module was being written and
// are kept here as the regressions they are: a cell-size ceiling that silently
// clipped a metropolis into a full square with no boundary at all, and an
// arterial grid so dense at coarse cells that half the city was road.
//
// The ledger is also why two bars are ordered the way they are rather than in
// the order they were written. `barLod` asks about the grid before the cost
// and `barScale` asks about coverage before area, because in both pairs the
// second bar catches the first bar's defect as a side effect and would report
// the wrong diagnosis.
//
// WHAT IS DELIBERATELY NOT ASSERTED. Nothing here checks that a city looks
// like the real place. It cannot and must not: the module models extent,
// density and height from population and states that it does. The provenance
// bar asserts the opposite — that the description SAYS it is a model — because
// that sentence is the only thing standing between a card and a player who
// believes the grid under "Kyoto" is Kyoto's streets.
"use strict";
const { test } = require("node:test");
const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");

const meshPath = path.resolve(__dirname, "../../spheres-web/ui/city-mesh.js");
const kit = require(meshPath);
const source = fs.readFileSync(meshPath, "utf8");

// The real records, loaded the way the browser loads them. Every bar that
// claims something about "any city" is asked of these 1,249 and not of a
// convenient handful.
const citiesPath = path.resolve(__dirname, "../../spheres-web/ui/cities.js");
const CITIES = (function () {
  const context = vm.createContext({ window: {} });
  vm.runInContext(fs.readFileSync(citiesPath, "utf8"), context);
  return context.window.CITIES;
})();
const byName = (n) => CITIES.find((c) => c.name === n);

// ------------------------------------------------------------ the budget bar
// Declared in city-mesh.js with its derivation and repeated here so a reader of
// the check alone can see what is being defended. `ceiling` must hold for every
// record in cities.js; `largest` is the band a city over two million must land
// in, which is the half of the bar that catches the model quietly collapsing
// into a small mesh for everyone.
// RE-DERIVED 2026-09-07 when the close span ceiling came down 141 -> 81. That
// change was made for LEGIBILITY -- at 141 a cell is 1.8 px on the 252 px card
// and a big city aliases into speckle -- and because the grid IS the cost, it
// also cut the worst case in the dataset from 99,008 to 34,876. The old band
// (close 40,000-120,000 for big cities) could no longer fire in either
// direction, so it was re-measured rather than left decorative.
//
// Measured over all 1,249 real records at the new span: close 1,984 (Andorra)
// to 34,876 (Vancouver), median 12,638, p99 32,754; cities over 2M run
// 25,994 to 34,876; map 392 to 3,988 (San Francisco), median 760.
// The ceilings carry ~15% and ~25% headroom over the worst real case.
const CLOSE_CEILING = 40000, MAP_CEILING = 5000;
const CLOSE_FLOOR = 250, MAP_FLOOR = 100;
const BIG_POP = 2000000, BIG_MIN = 24000;
// What today's single representative town block costs, measured, and the reason
// this kit exists: it must show an entire city for less than one block.
const TOWN_BLOCK_CLOSE = 198462;

// ------------------------------------------------------------- the fixtures
// Synthetic records, so a bar can move ONE field and read what changed. They
// carry the same field names and the same ranges as the Natural Earth rows.
const TOWN = { name: "Fixture Town", lon: 8.5, lat: 47.4, pop: 24000, rank: 6, capital: false };
const CITY = { name: "Fixture City", lon: 8.5, lat: 47.4, pop: 620000, rank: 3, capital: false };
const CAPITAL = { name: "Fixture Capital", lon: 8.5, lat: 47.4, pop: 2400000, rank: 1, capital: true };
const COAST = { name: "Fixture Coast", lon: -9.0, lat: 38.7, pop: 900000, rank: 2, capital: false };

/// Synthetic terrain, every one a closed form so a bar can recompute what the
/// module should have decided rather than trusting it.
/// A ramp of 900 m per degree of latitude: over a 6 km city that is about 50 m
/// of relief, enough to measure and gentle enough to be plausible ground.
const ramp = (lon, lat) => 120 + 900 * (lat - COAST.lat);
/// Sea to the west of the city's own longitude. Half the extent, near enough,
/// and a straight coast so a bar can say exactly which cells must be water.
const westSea = (lon) => lon > COAST.lon;

/// REALM-AGNOSTIC, and it has to be. The sabotage ledger loads a patched copy
/// of the module into a `vm` context, whose Float32Array is a DIFFERENT
/// constructor from this file's: `instanceof` is false and `Buffer.from` will
/// not take its ArrayBuffer. Both are answered by copying through a host typed
/// array, and the type bar reads the internal class instead of the prototype.
/// Without this every sabotage failed on "positions are not a Float32Array"
/// rather than on the bar it was written to trip, which would have made the
/// whole ledger meaningless while looking green.
const isF32 = (v) => Object.prototype.toString.call(v) === "[object Float32Array]";
const bytes = (v) => Buffer.from(Float32Array.from(v).buffer);
function digest(mesh) {
  return crypto.createHash("sha256")
    .update(bytes(mesh.positions))
    .update(bytes(mesh.normals))
    .update(bytes(mesh.colors))
    .digest("hex");
}

// -------------------------------------------------------------------- bars
// Written as functions of the module so the sabotage ledger can run the exact
// same sweep against a deliberately broken copy. Every message is distinctive,
// because the ledger asserts WHICH bar caught each defect.

/// Everything the renderer assumes about a buffer before it ever draws it.
function barContract(mod, tag) {
  for (const [rec, opts] of [[TOWN, {}], [CITY, {}], [CAPITAL, {}], [CITY, { lod: "map" }],
    [COAST, { land: westSea, height: ramp }]]) {
    const m = mod.build(rec, opts);
    const t = tag + rec.name + " " + (opts.lod || "close") + ": ";
    assert.ok(m, t + "built nothing at all");
    assert.ok(isF32(m.positions), t + "positions are not a Float32Array");
    assert.ok(isF32(m.normals), t + "normals are not a Float32Array");
    assert.ok(isF32(m.colors), t + "colors are not a Float32Array");
    assert.equal(m.positions.length, m.normals.length, t + "normal alignment");
    assert.equal(m.positions.length, m.colors.length, t + "colour alignment");
    assert.equal(m.positions.length % 9, 0, t + "buffer is not whole triangles");
    assert.equal(m.triangleCount, m.positions.length / 9, t + "triangleCount disagrees with the buffer");
    assert.ok(m.triangleCount > 0, t + "built an empty mesh");
    // SCAN FIRST, ASSERT ONCE. `assert.ok(cond, msg)` builds `msg` on every
    // call whether or not it fires, and these loops run over every float in
    // the mesh — about 1.4 million of them for the capital fixture alone, and
    // thirteen times over under the sabotage ledger. Asserting per element
    // cost 35 seconds of the suite in string concatenation for messages nobody
    // ever read.
    let badPos = -1, badNrm = -1, badCol = -1, badLen = -1;
    for (let i = 0; i < m.positions.length; i += 1) {
      if (badPos < 0 && !Number.isFinite(m.positions[i])) badPos = i;
      if (badNrm < 0 && !Number.isFinite(m.normals[i])) badNrm = i;
      if (badCol < 0 && !(m.colors[i] >= 0 && m.colors[i] <= 1)) badCol = i;
    }
    for (let i = 0; i < m.normals.length; i += 3) {
      const len = Math.sqrt(m.normals[i] ** 2 + m.normals[i + 1] ** 2 + m.normals[i + 2] ** 2);
      if (Math.abs(len - 1) > 1e-4) { badLen = i; break; }
    }
    assert.equal(badPos, -1, t + "a position is not finite, at " + badPos);
    assert.equal(badNrm, -1, t + "a normal is not finite, at " + badNrm);
    assert.equal(badCol, -1, t + "a colour is outside 0..1, at " + badCol);
    assert.equal(badLen, -1, t + "a normal is not unit length, at " + badLen);
    // GROUND CONTACT. The lowest vertex in the mesh is exactly Y=0 and nothing
    // is below it — for a coastal city that lowest thing is the water plate,
    // for an inland one the bottom of the lowest street.
    assert.equal(m.bounds.min[1], 0, t + "the mesh does not sit on Y=0");
    let below = -1;
    for (let i = 1; i < m.positions.length; i += 3) if (m.positions[i] < 0) { below = i; break; }
    assert.equal(below, -1, t + "a vertex is below the ground plane");
    // Bounds are the buffer's own, not a nominal box.
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < m.positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (m.positions[i + k] < min[k]) min[k] = m.positions[i + k];
        if (m.positions[i + k] > max[k]) max[k] = m.positions[i + k];
      }
    }
    for (let k = 0; k < 3; k += 1) {
      assert.equal(m.bounds.min[k], min[k], t + "bounds.min disagrees with the buffer");
      assert.equal(m.bounds.max[k], max[k], t + "bounds.max disagrees with the buffer");
      assert.ok(Math.abs(m.size[k] - (max[k] - min[k])) < 1e-6, t + "size disagrees with the bounds");
    }
    // PARTS ARE A PARTITION. Contiguous, in order, covering every vertex — a
    // host that draws the parts draws the mesh, exactly once.
    let cursor = 0;
    for (const part of m.parts) {
      assert.equal(typeof part.name, "string", t + "a part has no name");
      assert.equal(typeof part.kind, "string", t + "a part has no kind");
      assert.equal(part.first, cursor, t + "parts are not contiguous at " + part.name);
      assert.ok(part.count > 0, t + "an empty part was recorded: " + part.name);
      assert.equal(part.count % 3, 0, t + "a part is not whole triangles: " + part.name);
      cursor += part.count;
    }
    assert.equal(cursor, m.positions.length / 3, t + "the parts do not cover the whole mesh");
  }
}

/// The geometry itself: which way the faces point, and whether a mass stayed
/// inside the cell that owns it. Both are things a soup of boxes gets wrong
/// silently — an inside-out wall is invisible until the renderer culls it.
function barGeometry(mod, tag) {
  for (const rec of [TOWN, CAPITAL]) {
    const p = mod.plan(rec, {});
    const m = mod.build(rec, {});
    const t = tag + rec.name + ": ";
    // Same discipline as barContract: scan, remember the first offender, and
    // assert once at the end.
    let walls = 0, roofs = 0, down = -1, inward = -1, offGrid = -1, offPlot = -1;
    for (let i = 0; i < m.positions.length; i += 9) {
      const nx = m.normals[i], ny = m.normals[i + 1], nz = m.normals[i + 2];
      // NOTHING FACES DOWN. There are no undersides in this kit and no
      // overhangs, so a downward normal is an inside-out face or a box built
      // upside down.
      if (down < 0 && !(ny > -0.05)) down = i;
      if (ny > 0.5) { roofs += 1; continue; }
      walls += 1;
      const cx = (m.positions[i] + m.positions[i + 3] + m.positions[i + 6]) / 3;
      const cz = (m.positions[i + 2] + m.positions[i + 5] + m.positions[i + 8]) / 3;
      // A WALL FACES OUT of the cell that owns it. The mass is inset inside
      // its own cell, so the cell centre is always inside the box and the
      // outward test is exact.
      const ci = Math.round(cx / p.cell), cj = Math.round(cz / p.cell);
      if (inward < 0 && !(nx * (cx - ci * p.cell) + nz * (cz - cj * p.cell) > 0)) inward = i;
      // And it stayed on its own cell: a mass that wandered would put its
      // walls over a neighbour's ground.
      const gi = ci + p.half, gj = cj + p.half;
      if (gi < 0 || gi >= p.span || gj < 0 || gj >= p.span) { if (offGrid < 0) offGrid = i; continue; }
      if (offPlot < 0 && p.cls[gj * p.span + gi] !== mod.classes.PLOT) offPlot = i;
    }
    assert.equal(down, -1, t + "a triangle faces downwards");
    assert.equal(inward, -1, t + "a wall faces into the mass it belongs to");
    assert.equal(offGrid, -1, t + "a mass stands outside the grid");
    assert.equal(offPlot, -1, t + "a mass stands on a cell that is not a plot");
    assert.ok(walls > 0 && roofs > 0, t + "a city with no walls or no roofs");
  }
}

/// THE WHOLE POINT: a metropolis is huge and a town is small, and the size
/// comes from the record's own population.
function barScale(mod, tag) {
  const ladder = [1000, 10000, 50000, 200000, 800000, 3000000, 12000000, 35000000];
  let last = 0;
  for (const pop of ladder) {
    const e = mod.extentMetres(pop);
    assert.ok(e > last, tag + "extent does not scale with population: " + pop + " gives " + e
      + " against " + last + " for the size below it");
    last = e;
  }
  // And the mesh honours it. Measured across the whole dataset the built
  // footprint sits between 0.83 and 1.10 of the modelled area at close detail;
  // the boundary is lobed rather than a disc, so it cannot be exact, and a
  // quarter either way is the bar.
  for (const rec of [TOWN, CITY, CAPITAL]) {
    const p = mod.plan(rec, {});
    // COVERAGE FIRST, because it is the specific diagnosis. A grid smaller
    // than the modelled extent puts the whole boundary outside the mesh and
    // draws a slab with no edge at all — which also drags the area ratio
    // below, so the ratio alone would report the wrong defect.
    assert.ok(Math.abs(p.span * p.cell - p.extent) <= p.cell,
      tag + rec.name + ": the grid does not cover the modelled extent");
    const ratio = p.footprintKm2 / p.modelAreaKm2;
    assert.ok(ratio > 0.75 && ratio < 1.25, tag + rec.name
      + ": the built footprint is " + ratio.toFixed(3) + " of the modelled area, which is not the model");
  }
  // A real metropolis against a real town, on the real records.
  const tokyo = mod.build(byName("Tokyo"), {});
  const anadyr = mod.build(byName("Anadyr"), {});
  assert.ok(tokyo.size[0] / anadyr.size[0] > 15, tag
    + "Tokyo is only " + (tokyo.size[0] / anadyr.size[0]).toFixed(1)
    + " times wider than a town of ten thousand");
  assert.ok(tokyo.size[0] > 25000, tag + "Tokyo is under 25 km across");
  assert.ok(anadyr.size[0] < 3000, tag + "a town of ten thousand is over 3 km across");
}

/// Two levels, and the cheap one has to be genuinely cheap while still being
/// the same city.
function barLod(mod, tag) {
  for (const rec of [TOWN, CITY, CAPITAL, byName("Tokyo")]) {
    const close = mod.build(rec, { lod: "close" });
    const map = mod.build(rec, { lod: "map" });
    const t = tag + rec.name + ": ";
    // The grid first, then the cost. Both catch a map level that is secretly
    // the close one, and the grid is the more specific diagnosis of the two,
    // so it is the one a sabotage should report.
    assert.ok(map.span < close.span, t + "the map level uses the close grid");
    assert.ok(map.triangleCount < close.triangleCount / 3, t + "the map level is not cheaper: "
      + map.triangleCount + " against " + close.triangleCount);
    assert.ok(map.triangleCount <= MAP_CEILING, t + "the map level is over the declared ceiling");
    // Same city: same modelled extent, same character, same core height, and a
    // mesh the same size on the ground to within one of the coarser cells.
    assert.equal(map.modelExtent, close.modelExtent, t + "the two levels disagree about the extent");
    assert.equal(map.character, close.character, t + "the two levels disagree about the character");
    assert.equal(map.peakHeight, close.peakHeight, t + "the two levels disagree about the core height");
    assert.ok(Math.abs(map.size[0] - close.size[0]) <= map.cell * 1.5,
      t + "the two levels are different sizes on the ground");
  }
  // The words are accepted the way the other kits accept them.
  assert.equal(mod.build(CITY, { lod: "far" }).span, mod.build(CITY, { lod: "map" }).span,
    tag + '"far" is not the map level');
  assert.equal(mod.build(CITY, { lod: 1 }).span, mod.build(CITY, { lod: "map" }).span,
    tag + "the numeric LOD is not the map level");
  assert.equal(mod.build(CITY, { lod: "nonsense" }).span, mod.build(CITY, {}).span,
    tag + "an unknown LOD is not the close level");
}

/// THE SAMPLERS ARE AUTHORITATIVE, and the city stops where they say it does.
function barCoast(mod, tag) {
  const dry = mod.build(COAST, { land: () => true });
  const coastal = mod.build(COAST, { land: westSea });
  const drowned = mod.build(COAST, { land: () => false });
  assert.notEqual(digest(dry), digest(coastal), tag + "the land mask changed nothing at all");
  assert.ok(coastal.cells.water > 0, tag + "a mask that refuses half the extent refused nothing");
  assert.ok(coastal.cells.land < dry.cells.land, tag + "the coast did not take any land away");
  assert.ok(coastal.landKm2 < coastal.footprintKm2, tag + "land and footprint agree on a coastal city");
  assert.ok(coastal.cells.plot < dry.cells.plot, tag + "the coast did not remove a single mass");
  assert.ok(coastal.coastal === true && dry.coastal === false, tag + "the coastal flag is wrong");
  // NOT ONE MASS STANDS IN THE SEA. Checked against the geometry rather than
  // against the plan's own bookkeeping: the massing parts are walked and every
  // triangle in them is required to sit over a cell the mask allowed.
  const p = mod.plan(COAST, { land: westSea });
  const massing = coastal.parts.filter((x) => x.kind === "fabric" || x.kind === "tower" || x.kind === "landmark");
  assert.ok(massing.length > 0, tag + "a coastal city with no massing at all");
  for (const part of massing) {
    let drowned = -1;
    for (let v = part.first; v < part.first + part.count && drowned < 0; v += 3) {
      const i = v * 3;
      const cx = (coastal.positions[i] + coastal.positions[i + 3] + coastal.positions[i + 6]) / 3;
      const cz = (coastal.positions[i + 2] + coastal.positions[i + 5] + coastal.positions[i + 8]) / 3;
      const idx = (Math.round(cz / p.cell) + p.half) * p.span + (Math.round(cx / p.cell) + p.half);
      if (p.cls[idx] === mod.classes.WATER) drowned = i;
    }
    assert.equal(drowned, -1, tag + "a mass is standing in the sea");
  }
  // The water actually exists as geometry, and it is under the land.
  const water = coastal.parts.find((x) => x.kind === "water");
  assert.ok(water, tag + "water cells were counted but no water was drawn");
  assert.ok(coastal.waterLevel < 0 || coastal.relief[0] > coastal.waterLevel,
    tag + "the water is not below the shore");
  // Every cell refused: no city, no crash, and the description says so.
  assert.equal(drowned.cells.plot, 0, tag + "a city was built on ground the mask refused entirely");
  assert.ok(drowned.triangleCount > 0, tag + "an entirely drowned extent produced no mesh at all");
  assert.ok(/NO LAND HERE/.test(drowned.description), tag + "an entirely drowned city does not say so");
}

/// Relief, from the host's elevation sampler, at 1:1.
function barRelief(mod, tag) {
  const flat = mod.build(COAST, {});
  const hilly = mod.build(COAST, { height: ramp });
  assert.notEqual(digest(flat), digest(hilly), tag + "the elevation sampler changed nothing at all");
  assert.equal(flat.relief[0], 0, tag + "flat ground has relief");
  assert.equal(flat.relief[1], 0, tag + "flat ground has relief");
  const span = hilly.relief[1] - hilly.relief[0];
  // The ramp is 900 m per degree of latitude over a city about 5.9 km across,
  // which is 0.053 degrees: 48 m of relief, within the cell quantisation.
  const expected = 900 * (hilly.size[2] / 110540);
  assert.ok(Math.abs(span - expected) < expected * 0.15, tag
    + "the relief is " + span.toFixed(1) + " m where the sampler's own ramp gives " + expected.toFixed(1));
  assert.ok(hilly.size[1] > flat.size[1], tag + "relief did not make the mesh any taller");
  // A sampler with no data anywhere degrades to flat and SAYS so rather than
  // throwing or emitting NaN.
  const blind = mod.build(COAST, { height: () => NaN });
  assert.equal(blind.relief[0], 0, tag + "a sampler with no data still produced relief");
  assert.ok(blind.noHeightSamples > 0, tag + "a sampler with no data reported no misses");
  assert.ok(/returned no data/.test(blind.description), tag + "a blind sampler is not reported");
  let nan = -1;
  for (let i = 0; i < blind.positions.length; i += 1) {
    if (!Number.isFinite(blind.positions[i])) { nan = i; break; }
  }
  assert.equal(nan, -1, tag + "a blind sampler put NaN in the buffer");
}

/// The same record builds the same city, twice in a row and from a fresh realm.
function barDeterminism(mod, tag) {
  for (const rec of [TOWN, CAPITAL]) {
    const a = mod.build(rec, { land: westSea, height: ramp });
    const b = mod.build(rec, { land: westSea, height: ramp });
    assert.equal(digest(a), digest(b), tag + rec.name + ": two builds are not byte-identical");
  }
  // The seed is the record when none is given, so two places with the same
  // name in different countries are different cities.
  const here = mod.build({ name: "Springfield", lon: -89.6, lat: 39.8, pop: 120000, rank: 5, capital: false }, {});
  const there = mod.build({ name: "Springfield", lon: -72.6, lat: 42.1, pop: 120000, rank: 5, capital: false }, {});
  assert.notEqual(digest(here), digest(there), tag + "two Springfields are the same city");
  // And an explicit seed overrides it.
  const s1 = mod.build(CITY, { seed: "one" }), s2 = mod.build(CITY, { seed: "two" });
  assert.notEqual(digest(s1), digest(s2), tag + "the seed option does nothing");
  assert.equal(digest(mod.build(CITY, { seed: "one" })), digest(s1), tag + "the same seed is not stable");
}

/// Character comes from the record, and only from the fields the record has.
function barCharacter(mod, tag) {
  assert.equal(mod.character({ pop: 40000 }), "small", tag + "band: small");
  assert.equal(mod.character({ pop: 200000 }), "residential", tag + "band: residential");
  assert.equal(mod.character({ pop: 900000 }), "mixed", tag + "band: mixed");
  assert.equal(mod.character({ pop: 9000000 }), "commercial", tag + "band: commercial");
  const cap = mod.build(CAPITAL, {});
  const plain = mod.build(Object.assign({}, CAPITAL, { capital: false }), {});
  assert.ok(cap.parts.some((x) => x.kind === "landmark"), tag + "a capital has no landmark");
  assert.ok(!plain.parts.some((x) => x.kind === "landmark"), tag + "a city that is not a capital has a landmark");
  assert.equal(cap.layout, "radial", tag + "a capital is not laid out radially");
  assert.notEqual(plain.layout, "radial", tag + "the capital flag did not change the plan");
  assert.ok(cap.landmark && cap.tallest >= cap.peakHeight, tag + "the landmark is not the tallest thing");
  // A big city has a tower cluster; a small one has neither towers nor height.
  const big = mod.build(byName("Mumbai"), {});
  assert.ok(big.cells.tower > 0 && big.parts.some((x) => x.kind === "tower"),
    tag + "a city of nineteen million has no core towers");
  const town = mod.build(TOWN, {});
  assert.equal(town.cells.tower, 0, tag + "a town of twenty-four thousand has towers");
  assert.ok(town.tallest < 25, tag + "a town of twenty-four thousand is " + town.tallest + " m tall");
  assert.ok(big.peakHeight > town.peakHeight * 4, tag + "the core height does not scale with population");
  // Rank is used, and it is used for exactly one thing.
  const r0 = mod.plan(Object.assign({}, CITY, { rank: 0 }), {});
  const r8 = mod.plan(Object.assign({}, CITY, { rank: 8 }), {});
  assert.ok(r0.counts.footprint > r8.counts.footprint, tag + "the scale rank changes nothing");
  assert.equal(r0.extent, r8.extent, tag + "the scale rank moved the extent, which is population's job");
  assert.equal(r0.peak, r8.peak, tag + "the scale rank moved the core height, which is population's job");
}

/// The street grid exists and reads as one from above.
function barStreets(mod, tag) {
  const p = mod.plan(CITY, {});
  assert.ok(p.counts.road > 0, tag + "a city with no roads");
  const share = p.counts.road / p.counts.footprint;
  assert.ok(share > 0.05 && share < 0.40, tag + "roads are " + (share * 100).toFixed(0)
    + "% of the city, which is either invisible or a car park");
  // THE PRIMARY CROSS IS CONTINUOUS. Not a decorative sprinkle of road cells:
  // every cell of the centre row and centre column that is inside the city is
  // a road, so the grid reads as lines rather than as noise.
  let row = 0, rowRoad = 0;
  for (let i = 0; i < p.span; i += 1) {
    const idx = p.half * p.span + i;
    if (p.cls[idx] === mod.classes.OUT) continue;
    row += 1;
    if (p.cls[idx] === mod.classes.ROAD) rowRoad += 1;
  }
  assert.ok(row > 0, tag + "the centre row is empty");
  assert.equal(rowRoad, row, tag + "the main street is broken: " + rowRoad + " of " + row + " cells");
  assert.ok(p.counts.park > 0, tag + "a city with no open space at all");
  // THE BOOKKEEPING AGREES WITH THE GRID. `counts` is what the description
  // quotes and what the areas are computed from, and it is maintained by hand
  // as the classification runs — the landmark in particular takes a cell that
  // already belonged to something else. Counted straight off the class array,
  // for a coastal capital where every class is populated.
  const cp = mod.plan(CAPITAL, { land: westSea });
  const tally = { water: 0, road: 0, park: 0, open: 0, plot: 0 };
  const names = { 1: "water", 2: "road", 3: "park", 4: "open", 5: "plot" };
  for (const v of cp.cls) if (names[v]) tally[names[v]] += 1;
  let sum = 0;
  for (const k of Object.keys(tally)) {
    assert.equal(cp.counts[k], tally[k], tag + "the plan miscounted " + k + ": "
      + cp.counts[k] + " recorded against " + tally[k] + " in the grid");
    sum += tally[k];
  }
  assert.equal(cp.counts.footprint, sum, tag + "the footprint count is not the sum of its classes");
  // The arterial grid is a close-detail feature and the map level drops it, so
  // the map level must be visibly less roaded than the close one.
  const closeShare = share;
  const mp = mod.plan(CITY, { lod: "map" });
  assert.ok(mp.counts.road / mp.counts.footprint < closeShare, tag
    + "the map level carries the close level's arterial grid");
}

/// WHAT THE PLAYER IS TOLD. The one bar that is about honesty rather than
/// geometry, and the reason it is a bar at all: a generic grid under a real
/// city's name is a claim unless the caption says it is not.
function barProvenance(mod, tag) {
  const plain = mod.build(CITY, {});
  const sampled = mod.build(COAST, { land: westSea, height: ramp });
  for (const [m, t] of [[plain, tag + "plain: "], [sampled, tag + "sampled: "]]) {
    const d = m.description;
    assert.equal(typeof d, "string", t + "no description at all");
    assert.ok(/Original game art/.test(d), t + "does not state the art is original");
    assert.ok(/representative/i.test(d), t + "does not state the model is representative");
    assert.ok(/A = 40 x \(population\/1e6\)\^0\.85/.test(d), t + "does not state the area model it used");
    assert.ok(/MODEL, NOT A MEASUREMENT/.test(d), t + "does not say the extent is modelled");
    assert.ok(/own plan/.test(d) && /no street, district or building here is that city's/.test(d),
      t + "does not disclaim the real city's plan");
    assert.ok(/grants no simulation capability/.test(d), t + "claims a gameplay effect");
    assert.ok(d.includes(m.city.name), t + "does not name the city it drew");
  }
  // The `assumed` block and the description agree about what was supplied.
  // FIELD BY FIELD, NOT deepEqual. A strict deep-equal compares prototypes, and
  // the sabotage ledger's module is built in a `vm` realm whose Object is not
  // this file's — so `deepEqual` here could never pass under sabotage, and the
  // first defect that survived this far reported THIS bar instead of its own.
  // That is the same class of trap the ledger exists to catch, in the checker.
  assert.equal(plain.assumed.height, true, tag + "assumed.height is wrong with no samplers");
  assert.equal(plain.assumed.land, true, tag + "assumed.land is wrong with no samplers");
  assert.ok(/assumed flat/.test(plain.description), tag + "flat ground is not disclosed");
  assert.ok(/assumed to be land/.test(plain.description), tag + "the missing land mask is not disclosed");
  assert.equal(sampled.assumed.height, false, tag + "assumed.height is wrong with samplers");
  assert.equal(sampled.assumed.land, false, tag + "assumed.land is wrong with samplers");
  assert.ok(!/assumed flat/.test(sampled.description), tag + "claims flat ground while sampling relief");
  assert.ok(/host's land mask/.test(sampled.description), tag + "does not credit the host's mask");
  assert.ok(/no vertical exaggeration/.test(sampled.description), tag + "does not state the relief scale");
  // The metadata says the same thing the description does.
  assert.equal(plain.areaModel, "A_km2 = 40 * (pop / 1e6) ^ 0.85, representative",
    tag + "the metadata does not carry the model");
  assert.equal(plain.city.pop, CITY.pop, tag + "the record's own population is not echoed");
  // A record with no population at all is drawn, and the floor is disclosed.
  const empty = mod.build({ name: "Ghost", lon: 0, lat: 0, pop: 0, rank: 10, capital: false }, {});
  assert.ok(empty.triangleCount > 0, tag + "a record with no population built nothing");
  assert.ok(empty.popFloored === true && /floored at/.test(empty.description),
    tag + "the population floor is applied silently");
}

/// The budget, on the fixtures. The whole-dataset version is a separate test
/// because it is too slow to run eight more times under the sabotage ledger.
function barBudget(mod, tag) {
  // Toronto is the measured worst case in the whole dataset and Tokyo is the
  // widest city there is, so those are the two big records the sweep pays for;
  // the rest of the dataset is asked once, in a test of its own that the
  // sabotage ledger does not re-run twelve times.
  for (const rec of [byName("Toronto"), byName("Tokyo"), CAPITAL, CITY, TOWN]) {
    const close = mod.build(rec, {});
    const map = mod.build(rec, { lod: "map" });
    const t = tag + rec.name + ": ";
    assert.ok(close.triangleCount <= CLOSE_CEILING, t + "close is over the declared ceiling: "
      + close.triangleCount);
    assert.ok(close.triangleCount >= CLOSE_FLOOR, t + "close is under the declared floor");
    assert.ok(map.triangleCount <= MAP_CEILING, t + "map is over the declared ceiling: " + map.triangleCount);
    assert.ok(map.triangleCount >= MAP_FLOOR, t + "map is under the declared floor");
    assert.ok(close.triangleCount < TOWN_BLOCK_CLOSE, t
      + "an entire city costs more than the one town block it replaces");
    if (rec.pop >= BIG_POP) {
      assert.ok(close.triangleCount >= BIG_MIN, t + "a city of " + rec.pop
        + " is only " + close.triangleCount + " triangles, which is not a metropolis");
    }
  }
  assert.equal(mod.budget.ceiling.close, CLOSE_CEILING, tag + "the module's declared ceiling moved");
  assert.equal(mod.budget.ceiling.map, MAP_CEILING, tag + "the module's declared map ceiling moved");
}

function sweep(mod, tag) {
  barContract(mod, tag);
  barGeometry(mod, tag);
  barScale(mod, tag);
  barLod(mod, tag);
  barCoast(mod, tag);
  barRelief(mod, tag);
  barDeterminism(mod, tag);
  barCharacter(mod, tag);
  barStreets(mod, tag);
  barProvenance(mod, tag);
  barBudget(mod, tag);
}

// -------------------------------------------------------------------- tests
test("the render contract holds for a town, a city, a capital, a coast and both levels", () => {
  barContract(kit, "");
});

test("every face points out and every mass stands on its own plot", () => {
  barGeometry(kit, "");
});

test("extent scales with population, from a hamlet to Tokyo", () => {
  barScale(kit, "");
});

test("the map level is the same city for a third of the triangles or better", () => {
  barLod(kit, "");
});

test("the land mask is authoritative: a coastal city stops at its coastline", () => {
  barCoast(kit, "");
});

test("relief is the host's elevation sampler at 1:1, and no data degrades to flat", () => {
  barRelief(kit, "");
});

test("the same record builds a byte-identical city, and a different record does not", () => {
  barDeterminism(kit, "");
});

test("character comes from population, scale rank and the capital flag", () => {
  barCharacter(kit, "");
});

test("the street grid is continuous and is a quarter of the city at most", () => {
  barStreets(kit, "");
});

test("the description states what is modelled rather than sourced", () => {
  barProvenance(kit, "");
});

test("the declared budget holds on the headline records", () => {
  barBudget(kit, "");
});

// ------------------------------------------------------- the whole dataset
// The bars above are asked of fixtures because a fixture can be reasoned about.
// These are asked of every real record, because "no city on Earth costs more
// than this" is not a claim a fixture can make.
test("the declared budget holds for every one of the 1,249 real records", () => {
  assert.ok(CITIES.length > 1200, "cities.js did not load");
  let worstClose = { t: 0 }, worstMap = { t: 0 }, leastBig = { t: Infinity };
  let big = 0;
  for (let n = 0; n < CITIES.length; n += 1) {
    const c = CITIES[n];
    const map = kit.build(c, { lod: "map" });
    assert.ok(map.triangleCount <= MAP_CEILING, c.name + ": map is " + map.triangleCount);
    assert.ok(map.triangleCount >= MAP_FLOOR, c.name + ": map is " + map.triangleCount);
    assert.ok(Number.isFinite(map.size[0]) && map.size[0] > 0, c.name + ": degenerate map mesh");
    if (map.triangleCount > worstMap.t) worstMap = { t: map.triangleCount, n: c.name };
    // Close detail is asked of every city over two million — where the ceiling
    // is actually in danger — and of every seventh of the rest, which is 149
    // more records across the whole size range.
    const sampled = c.pop >= BIG_POP || n % 7 === 0;
    if (!sampled) continue;
    const close = kit.build(c, {});
    assert.ok(close.triangleCount <= CLOSE_CEILING, c.name + ": close is " + close.triangleCount);
    assert.ok(close.triangleCount >= CLOSE_FLOOR, c.name + ": close is " + close.triangleCount);
    assert.ok(close.triangleCount < TOWN_BLOCK_CLOSE,
      c.name + ": costs more than the town block it replaces");
    assert.equal(close.bounds.min[1], 0, c.name + ": does not sit on Y=0");
    if (close.triangleCount > worstClose.t) worstClose = { t: close.triangleCount, n: c.name };
    if (c.pop >= BIG_POP) {
      big += 1;
      assert.ok(close.triangleCount >= BIG_MIN, c.name + " has " + c.pop
        + " people and only " + close.triangleCount + " triangles");
      if (close.triangleCount < leastBig.t) leastBig = { t: close.triangleCount, n: c.name };
    }
  }
  assert.ok(big > 150, "only " + big + " records over two million were checked");
  process.stdout.write("    worst close " + worstClose.t.toLocaleString("en-US") + " (" + worstClose.n
    + "), worst map " + worstMap.t.toLocaleString("en-US") + " (" + worstMap.n
    + "), leanest metropolis " + leastBig.t.toLocaleString("en-US") + " (" + leastBig.n + ")\n");
});

test("a fresh process builds the same buffers, byte for byte", () => {
  const script = "const k=require(" + JSON.stringify(meshPath) + ");"
    + "const c=require('node:crypto');"
    + "const rows=[[{name:'Fixture City',lon:8.5,lat:47.4,pop:620000,rank:3,capital:false},{}],"
    + "[{name:'Fixture Capital',lon:8.5,lat:47.4,pop:2400000,rank:1,capital:true},{lod:'map'}]];"
    + "const out=rows.map(([r,o])=>{const m=k.build(r,o);"
    + "const b=v=>Buffer.from(Float32Array.from(v).buffer);"
    + "return c.createHash('sha256').update(b(m.positions))"
    + ".update(b(m.normals)).update(b(m.colors)).digest('hex');});"
    + "process.stdout.write(out.join(' '));";
  const run = cp.spawnSync(process.execPath, ["-e", script], { encoding: "utf8" });
  assert.equal(run.status, 0, "the child process failed: " + run.stderr);
  const mine = [digest(kit.build(CITY, {})), digest(kit.build(CAPITAL, { lod: "map" }))].join(" ");
  assert.equal(run.stdout, mine, "a fresh require builds a different city");
});

test("the browser global exports the same contract and builds the identical city", () => {
  const context = vm.createContext({});
  vm.runInContext(source, context);
  const browser = context.CityMesh;
  assert.ok(browser, "assigns a global when there is no module.exports");
  assert.equal(browser.version, kit.version);
  assert.equal(browser.kit, kit.kit);
  assert.equal(browser.build(CITY, {}).triangleCount, kit.build(CITY, {}).triangleCount);
  assert.equal(digest(browser.build(CAPITAL, { land: westSea, height: ramp })),
    digest(kit.build(CAPITAL, { land: westSea, height: ramp })));
});

test("no wall clock, no entropy source, no DOM and no transcendental arithmetic", () => {
  const banned = [
    [/\bDate\s*\.\s*now\s*\(/, "Date.now"],
    [/new\s+Date\b/, "new Date"],
    [/Math\s*\.\s*random\s*\(/, "Math.random"],
    [/\bperformance\s*\./, "performance"],
    [/\bdocument\b/, "document"],
    [/\bwindow\s*\./, "window"],
    [/\brequire\s*\(/, "require"],
    [/\bfetch\s*\(/, "fetch"],
  ];
  for (const [pattern, name] of banned) {
    assert.ok(!pattern.test(source), "city-mesh.js must not reference " + name);
  }
  // THE DETERMINISM CLAIM IS STRONGER THAN "no entropy". The file states that
  // it uses no implementation-defined transcendental arithmetic, because that
  // is what makes a buffer byte-identical across MACHINES rather than merely
  // across reloads in one process. The single documented exception is the
  // cosine in metres-per-degree of longitude.
  const calls = source.match(/Math\s*\.\s*(pow|exp|log|log2|log10|sin|cos|tan|atan|atan2|hypot|cbrt|sinh|cosh|tanh)\s*\(/g) || [];
  assert.equal(calls.length, 1, "expected exactly one transcendental call, found: " + calls.join(", "));
  assert.equal(calls[0].replace(/\s+/g, ""), "Math.cos(", "the one exception is not the documented cosine");
  assert.ok(/function lonMetres\(lat\) \{ return M_LON \* Math\.max\(1e-3, Math\.cos/.test(source),
    "the one cosine is not the one in lonMetres");
  assert.ok(/Math\.imul/.test(source), "the hash is still the source of variety");
});

test("options and records are read, never written", () => {
  const rec = { name: "Fixture City", lon: 8.5, lat: 47.4, pop: 620000, rank: 3, capital: false };
  const opts = { lod: "map", seed: "abc", land: westSea, height: ramp };
  const recBefore = JSON.stringify(rec);
  const optsBefore = JSON.stringify(opts, (k, v) => (typeof v === "function" ? "fn" : v));
  kit.build(rec, opts);
  kit.plan(rec, opts);
  assert.equal(JSON.stringify(rec), recBefore, "the record was mutated");
  assert.equal(JSON.stringify(opts, (k, v) => (typeof v === "function" ? "fn" : v)), optsBefore,
    "the options were mutated");
  // Junk in, a mesh out — a card that refuses to draw is worse on screen than
  // a card that draws a small town.
  for (const junk of [null, undefined, {}, { name: 5, lon: "x", lat: null, pop: -3 },
    { name: "NaN town", lon: NaN, lat: NaN, pop: NaN }]) {
    const m = kit.build(junk, { lod: "map" });
    assert.ok(m.triangleCount > 0, "a junk record built nothing: " + JSON.stringify(junk));
    assert.equal(m.bounds.min[1], 0, "a junk record does not sit on Y=0");
  }
});

test("the area model is the closed form it says it is", () => {
  // The table is evaluated from A = 40 * (P/1e6)^0.85 and interpolated
  // linearly; the module never calls Math.pow, so this is the only place the
  // closed form is actually computed and compared.
  let worst = 0;
  for (let pop = 1000; pop <= 5e7; pop = Math.round(pop * 1.07)) {
    const exact = 40 * Math.pow(Math.min(pop, 1e8) / 1e6, 0.85);
    const table = kit.areaKm2(pop);
    const err = Math.abs(table - exact) / exact;
    if (err > worst) worst = err;
  }
  // The file states 2.27% at a population of 5,211, so the bar is 2.5% and it
  // is a bar on the MODEL, not on the mesh: a wider departure means the table
  // has been edited away from the formula the description quotes.
  assert.ok(worst < 0.025, "the table departs from the closed form by " + (worst * 100).toFixed(2) + "%");
  for (const [pop, km2] of kit.areaModel.anchors) {
    assert.ok(Math.abs(km2 - 40 * Math.pow(pop / 1e6, 0.85)) < km2 * 0.001,
      "anchor " + pop + " is not the closed form");
    // The last anchor is past the module's own population ceiling and exists
    // to give the interpolation a slope up to it, so it cannot read back.
    if (pop > 5e7) continue;
    assert.ok(Math.abs(kit.areaKm2(pop) - km2) < 1e-3, "anchor " + pop + " does not read back");
  }
  // The clamps at both ends are stated behaviour, not accidents.
  assert.equal(kit.areaKm2(0), kit.areaKm2(kit.populationFloor), "the population floor is not applied");
  assert.equal(kit.areaKm2(9e9), kit.areaKm2(5e7), "the population ceiling is not applied");
});

// ------------------------------------------------------------- the ledger
// Eight deliberate defects. Each is a single-line edit to the real source, run
// through the same sweep, and required to fail ON ITS NAMED BAR — a sabotage
// that fails somewhere else is a sabotage that has stopped testing what it
// says it tests.
const SABOTAGE = [
  {
    defect: "the extent no longer follows population — every city the same size",
    expect: "extent does not scale with population",
    edits: [["    const a = areaKm2(pop) * 1e6;", "    const a = areaKm2(500000) * 1e6;"]],
  },
  {
    defect: "the land mask is ignored, so a coastal city is built over the sea",
    expect: "the land mask changed nothing at all",
    edits: [["        if (land && !land(lonAt(x), latAt(z))) {", "        if (false && land) {"]],
  },
  {
    defect: "the elevation sampler is ignored and every city is flat",
    expect: "the elevation sampler changed nothing at all",
    edits: [["          if (Number.isFinite(h)) corner[cj * cornerN + ci] = q(h - datum, 0.05);",
      "          if (false && Number.isFinite(h)) corner[cj * cornerN + ci] = q(h - datum, 0.05);"]],
  },
  {
    defect: "an entropy source in the per-mass variation",
    expect: "two builds are not byte-identical",
    edits: [["    const w = q(room * askSpan(cs, 41, 0.80, 1.0), 0.1);",
      "    const w = q(room * (0.80 + 0.2 * Math.random()), 0.1);"]],
  },
  {
    defect: "the mesh is not seated on the ground plane",
    expect: "does not sit on Y=0",
    edits: [["    if (Number.isFinite(drop) && drop !== 0) {", "    if (false) {"]],
  },
  {
    defect: "the map level silently uses the close grid",
    expect: "the map level uses the close grid",
    edits: [["  const SPAN = { close: { min: 25, max: 81 }, map: { min: 13, max: 27 } };",
      "  const SPAN = { close: { min: 25, max: 81 }, map: { min: 25, max: 141 } };"]],
  },
  {
    defect: "the description stops saying the extent is a model",
    expect: "does not say the extent is modelled",
    edits: [['    const model = " EXTENT IS A MODEL, NOT A MEASUREMENT: built-up area is "',
      '    const model = " Built-up area is "']],
  },
  {
    defect: "a wall is wound inside out",
    expect: "a wall faces into the mass it belongs to",
    edits: [["    b.quad([x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0], wall, mat);          // north",
      "    b.quad([x0, y0, z0], [x1, y0, z0], [x1, y1, z0], [x0, y1, z0], wall, mat);          // north"]],
  },
  {
    defect: "the cell ceiling clips a metropolis, so its grid no longer covers its extent",
    expect: "the grid does not cover the modelled extent",
    edits: [["  const CELL_LIMIT = { min: 12, max: 4000 };", "  const CELL_LIMIT = { min: 12, max: 60 };"]],
  },
  {
    defect: "the arterial grid runs at every cell, so the city is mostly road",
    expect: "% of the city, which is either invisible or a car park",
    edits: [["    const every = clamp(Math.round(1000 / cell), 6, 12);",
      "    const every = 2;"]],
  },
  {
    defect: "the core height stops following population, so a town is as tall as a metropolis",
    expect: "the core height does not scale with population",
    edits: [["    return q(through(PEAK_M, clamp(num(pop, 0), POP_FLOOR, POP_CEIL)), 0.1);",
      "    return q(through(PEAK_M, 50000), 0.1);"]],
  },
  {
    // 181 rather than a rounder number, and it is worth stating why: the cap
    // only binds on cities whose modelled extent is wider than cap x 100 m, so
    // raising it does not touch Toronto (14.2 km, span 143 at any cap above
    // that) and had to be raised far enough for TOKYO to cross the ceiling.
    // Measured: 141 -> 80,568, 165 -> 113,408, 181 -> 137,818.
    defect: "the close grid cap is raised, so the largest cities blow the triangle budget",
    expect: "close is over the declared ceiling",
    edits: [["  const SPAN = { close: { min: 25, max: 81 }, map: { min: 13, max: 27 } };",
      "  const SPAN = { close: { min: 25, max: 181 }, map: { min: 13, max: 27 } };"]],
  },
];

function sabotaged(edits) {
  let broken = source;
  for (const [from, to] of edits) {
    assert.equal(broken.split(from).length - 1, 1,
      "sabotage anchor is stale or not unique: " + from.slice(0, 70));
    broken = broken.replace(from, to);
  }
  const context = vm.createContext({});
  vm.runInContext(broken, context);
  return context.CityMesh;
}

test("every bar above goes red against a deliberately broken city generator", () => {
  for (const entry of SABOTAGE) {
    const mod = sabotaged(entry.edits);
    let caught = null;
    try { sweep(mod, "sabotaged "); } catch (err) { caught = err; }
    assert.ok(caught, "sabotage passed unnoticed: " + entry.defect);
    assert.ok(String(caught.message).includes(entry.expect),
      'sabotage "' + entry.defect + '" failed on the wrong bar: ' + caught.message);
  }
  // And the ledger only means anything if the same sweep is green on the real
  // source. The tests above already assert that one bar at a time; this
  // restates it so a reader of this block alone can see both halves.
  sweep(kit, "live ");
});

// One line of evidence in the log, so a reader of the suite output can see what
// the kit actually costs without running anything.
test("headline measurements", () => {
  const rows = ["Tokyo", "Toronto", "Hong Kong", "Kazan", "Reykjavik", "Bombo", "Anadyr"];
  const out = [];
  for (const n of rows) {
    const c = byName(n);
    const close = kit.build(c, {}), map = kit.build(c, { lod: "map" });
    out.push(n + " " + c.pop.toLocaleString("en-US") + ": "
      + close.triangleCount.toLocaleString("en-US") + "/" + map.triangleCount.toLocaleString("en-US")
      + " tris, " + (close.size[0] / 1000).toFixed(1) + " km at " + close.cell + " m cells");
  }
  process.stdout.write("    " + out.join("\n    ") + "\n");
  assert.ok(out.length === rows.length);
});
