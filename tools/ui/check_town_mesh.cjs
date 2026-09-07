#!/usr/bin/env node
// Regression anchor for the temperate town kit, `spheres-web/ui/town-mesh.js`.
//
// What this file is FOR. P0 asks one question of that generator — does a
// settlement read well at useful map zoom — and the two ways to answer it
// wrongly are to ship geometry the renderer chokes on, and to ship a town that
// is a different town on the next reload. Everything below is aimed at one or
// the other. The budgets come from roadmap section 4; the determinism bar is
// iron rule 1 restated for art.
//
// It runs under `node --test` alongside the other check_*.cjs via run-unit.cjs,
// and it deliberately reads the SOURCE as well as the meshes: a wall-clock call
// or an entropy source would not necessarily show up as a failing digest inside
// one process, and it is exactly the kind of thing that survives review.
"use strict";
const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");

const meshPath = path.resolve(__dirname, "../../spheres-web/ui/town-mesh.js");
const town = require(meshPath);
const source = fs.readFileSync(meshPath, "utf8");

// Roadmap section 4, "Building close view / map" and "Scene assembly".
const CLOSE_MIN = 2000, CLOSE_MAX = 12000, MAP_MIN = 100, MAP_MAX = 800, SCENE_MAX = 150000;

let checks = 0;
function check(label, fn) { fn(); checks += 1; process.stdout.write("PASS " + label + "\n"); }
function digest(mesh) {
  return crypto.createHash("sha256")
    .update(Buffer.from(mesh.positions.buffer))
    .update(Buffer.from(mesh.normals.buffer))
    .update(Buffer.from(mesh.colors.buffer))
    .digest("hex");
}

/// Everything the renderer assumes about a buffer before it ever draws it.
/// Written once because fifteen kinds, two LODs and five districts all have to
/// pass it, and a check that only the headline asset passes is not a check.
function validate(mesh, tag) {
  assert.ok(mesh, tag + ": built");
  assert.ok(mesh.positions instanceof Float32Array, tag + ": positions");
  assert.ok(mesh.normals instanceof Float32Array, tag + ": normals");
  assert.ok(mesh.colors instanceof Float32Array, tag + ": colors");
  assert.equal(mesh.positions.length, mesh.normals.length, tag + ": normal alignment");
  assert.equal(mesh.positions.length, mesh.colors.length, tag + ": colour alignment");
  assert.equal(mesh.positions.length, mesh.triangleCount * 9, tag + ": complete triangles");
  assert.ok(mesh.triangleCount > 0, tag + ": non-empty");

  for (let i = 0; i < mesh.positions.length; i += 3) {
    for (let k = 0; k < 3; k += 1) {
      assert.ok(Number.isFinite(mesh.positions[i + k]), tag + ": finite position at " + i);
      assert.ok(Number.isFinite(mesh.normals[i + k]), tag + ": finite normal at " + i);
      assert.ok(mesh.colors[i + k] >= 0 && mesh.colors[i + k] <= 1, tag + ": colour in 0..1 at " + i);
      assert.ok(mesh.positions[i + k] >= mesh.bounds.min[k] - 1e-4
        && mesh.positions[i + k] <= mesh.bounds.max[k] + 1e-4, tag + ": vertex inside bounds");
    }
    assert.ok(Math.abs(Math.hypot(mesh.normals[i], mesh.normals[i + 1], mesh.normals[i + 2]) - 1) < 1e-5,
      tag + ": unit normal at " + i);
  }
  // A degenerate triangle is invisible, shades black and exports as a hole; the
  // generator drops them at emit, so any survivor here is a real defect. The
  // facing test catches the other half of the same class — a normal that does
  // not follow its own winding, which is what mirroring gets wrong.
  for (let i = 0; i < mesh.positions.length; i += 9) {
    const a = mesh.positions.subarray(i, i + 3);
    const b = mesh.positions.subarray(i + 3, i + 6);
    const c = mesh.positions.subarray(i + 6, i + 9);
    const u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    const n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    const area = Math.hypot(n[0], n[1], n[2]);
    assert.ok(area > 1e-7, tag + ": non-degenerate triangle " + (i / 9));
    const facing = (n[0] * mesh.normals[i] + n[1] * mesh.normals[i + 1] + n[2] * mesh.normals[i + 2]) / area;
    assert.ok(facing > 0.999, tag + ": normal follows winding at " + (i / 9));
  }
  // Ground contact. The generator seats every mesh on its lowest vertex, so
  // this is exact rather than approximate; a tolerance here would hide a
  // building that had started floating.
  assert.equal(mesh.bounds.min[1], 0, tag + ": ground contact at Y=0");
  assert.ok(mesh.bounds.max[1] > 1.5, tag + ": has height");
  assert.ok(mesh.size.every((value) => value > 0), tag + ": visible volume");

  // Semantic ranges must tile the buffer exactly once and in order, or picking
  // a building in the browser selects the one next to it.
  let cursor = 0;
  for (const part of mesh.parts) {
    assert.equal(part.first, cursor, tag + ": part " + part.name + " starts where the last ended");
    assert.ok(part.count > 0 && part.count % 3 === 0, tag + ": part " + part.name + " is whole triangles");
    assert.ok(typeof part.name === "string" && part.name.length > 0, tag + ": part is named");
    cursor = part.first + part.count;
  }
  assert.equal(cursor, mesh.positions.length / 3, tag + ": parts cover every vertex");
  assert.ok(typeof mesh.description === "string" && mesh.description.length > 20, tag + ": described");
}

const kinds = town.kinds();

check("the kit covers the fifteen building kinds roadmap section G names", () => {
  const wanted = ["house", "row_house", "low_apartment", "mid_apartment", "high_apartment", "office",
    "shop", "warehouse", "civic", "school", "hospital", "university", "stadium", "park", "utility"];
  assert.deepEqual(kinds.slice().sort(), wanted.slice().sort());
  assert.equal(new Set(kinds).size, kinds.length);
  for (const kind of kinds) {
    const info = town.kindInfo(kind);
    assert.ok(info && info.label && info.widths.length > 0 && info.storeys.length > 0, kind + ": described");
    assert.ok(info.depth > 8 && info.depth < 200, kind + ": plausible lot depth");
    for (const width of info.widths) assert.ok(width > 4 && width < 260, kind + ": plausible frontage");
  }
});

check("every kind builds sound geometry at both LODs", () => {
  for (const kind of kinds) {
    validate(town.building(kind, { id: 3 }), kind + " close");
    validate(town.building(kind, { id: 3, lod: "map" }), kind + " map");
  }
});

check("every kind, at every frontage and storey count it offers, lands inside the section 4 budget", () => {
  for (const kind of kinds) {
    const info = town.kindInfo(kind);
    for (const width of info.widths) {
      for (const storeys of info.storeys) {
        const close = town.building(kind, { id: 11, width, storeys });
        const map = town.building(kind, { id: 11, width, storeys, lod: "map" });
        assert.ok(close.triangleCount >= CLOSE_MIN && close.triangleCount <= CLOSE_MAX,
          kind + " " + width + "m/" + storeys + ": close view " + close.triangleCount + " outside " + CLOSE_MIN + ".." + CLOSE_MAX);
        assert.ok(map.triangleCount >= MAP_MIN && map.triangleCount <= MAP_MAX,
          kind + " " + width + "m/" + storeys + ": map " + map.triangleCount + " outside " + MAP_MIN + ".." + MAP_MAX);
      }
    }
  }
});

check("the map LOD is the same building seen small, not a different one", () => {
  for (const kind of kinds) {
    const close = town.building(kind, { id: 3 }), map = town.building(kind, { id: 3, lod: "map" });
    assert.ok(close.triangleCount / map.triangleCount > 8, kind + ": map LOD is actually cheaper");
    const ratio = map.size[1] / close.size[1];
    assert.ok(ratio > 0.8 && ratio < 1.1, kind + ": map height " + ratio.toFixed(3) + " of close height");
    // Both roots sit on the centre of the footprint, so the two LODs swap in
    // place instead of sliding when the camera crosses the threshold.
    for (const axis of [0, 2]) {
      assert.ok(Math.abs(close.bounds.min[axis] + close.bounds.max[axis]) < 2.5, kind + ": close root centred on axis " + axis);
      assert.ok(Math.abs(map.bounds.min[axis] + map.bounds.max[axis]) < 2.5, kind + ": map root centred on axis " + axis);
    }
  }
});

const districts = town.districts();
const blocks = {};
for (const district of districts) blocks[district] = town.block({ id: 1990, district });

check("every district assembles a populated block inside the scene budget", () => {
  for (const district of districts) {
    const close = blocks[district];
    const map = town.block({ id: 1990, district, lod: "map" });
    validate(close, district + " block close");
    validate(map, district + " block map");
    assert.ok(close.triangleCount < SCENE_MAX, district + ": " + close.triangleCount + " over the " + SCENE_MAX + " scene budget");
    assert.ok(map.triangleCount < SCENE_MAX / 10, district + ": map assembly stays cheap");
    assert.ok(close.lotCount >= 8, district + ": " + close.lotCount + " lots is not a block");
    assert.equal(close.lots.length, close.lotCount);
    assert.equal(close.tile[0], close.bounds.max[0] - close.bounds.min[0], district + ": tile width is the mesh width");
    assert.equal(close.tile[1], close.bounds.max[2] - close.bounds.min[2], district + ": tile depth is the mesh depth");
  }
});

check("a block is assembled from shared building meshes, not from unique buildings", () => {
  for (const district of districts) {
    const block = blocks[district];
    assert.ok(block.variantCount < block.lotCount,
      district + ": " + block.lotCount + " lots from " + block.variantCount + " variants is not reuse");
    const keys = new Set(block.variants.map((v) => v.key));
    let uses = 0, repeated = 0;
    for (const variant of block.variants) {
      assert.ok(variant.uses >= 1 && variant.triangleCount > 0, district + ": variant " + variant.key + " is used and non-empty");
      uses += variant.uses;
      if (variant.uses > 1) repeated += 1;
    }
    assert.equal(uses, block.lotCount, district + ": every lot draws exactly one variant");
    assert.ok(repeated >= 1, district + ": at least one mesh is actually stamped twice");
    for (const lot of block.lots) assert.ok(keys.has(lot.variant), district + ": lot " + lot.index + " names a listed variant");
  }
});

check("lots tile the block without overlapping each other or leaving the tile", () => {
  for (const id of [0, 1, 2, 3, 5, 17, 42, 1990]) {
    for (const district of districts) {
      const block = town.block({ id, district });
      const halfW = block.tile[0] / 2, halfD = block.tile[1] / 2;
      const rects = block.lots.map((lot) => {
        const turned = lot.yaw % 2 === 1;
        const w = turned ? lot.depth : lot.width, d = turned ? lot.width : lot.depth;
        return [lot.x - w / 2, lot.x + w / 2, lot.z - d / 2, lot.z + d / 2];
      });
      rects.forEach((r, i) => {
        assert.ok(r[0] >= -halfW - 0.01 && r[1] <= halfW + 0.01 && r[2] >= -halfD - 0.01 && r[3] <= halfD + 0.01,
          id + "/" + district + ": lot " + i + " leaves the tile");
        for (let j = i + 1; j < rects.length; j += 1) {
          const s = rects[j];
          const overlapX = Math.min(r[1], s[1]) - Math.max(r[0], s[0]);
          const overlapZ = Math.min(r[3], s[3]) - Math.max(r[2], s[2]);
          assert.ok(!(overlapX > 0 && overlapZ > 0), id + "/" + district + ": lots " + i + " and " + j + " overlap");
        }
      });
    }
  }
});

/// The XZ box each lot's OWN triangles occupy, read out of `parts` rather than
/// out of `lots`. `minY` throws away everything below it so a ground pad, a
/// kerb upstand and a garden step cannot manufacture a clash between two
/// buildings that never touch.
function builtFootprint(block, part, minY) {
  let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity, found = false;
  for (let t = part.first; t < part.first + part.count; t += 3) {
    let top = -Infinity;
    for (let v = 0; v < 3; v += 1) top = Math.max(top, block.positions[(t + v) * 3 + 1]);
    if (top < minY) continue;
    found = true;
    for (let v = 0; v < 3; v += 1) {
      const x = block.positions[(t + v) * 3], z = block.positions[(t + v) * 3 + 2];
      if (x < x0) x0 = x;
      if (x > x1) x1 = x;
      if (z < z0) z0 = z;
      if (z > z1) z1 = z;
    }
  }
  return found ? [x0, x1, z0, z1] : null;
}

// The check above compares the lot TABLE against itself, which is only worth
// something if the table describes the geometry. It did not: the block anchor
// gated on a clamped request, so a 40 m university was recorded as an 18 m lot,
// passed the table check with room to spare, and drove 22 m through the school
// behind it — 540 m^2 of two buildings in the same ground on the worst civic
// block, 23 clashing pairs across 60 blocks, and every one of them invisible to
// a test that never looks at a vertex. So this one reads the triangles.
check("what is drawn stays on its own lot and out of its neighbour", () => {
  const EAVE = 1.5; // trim may hang over the boundary; a wing may not.
  for (const id of [0, 1, 2, 3, 5, 17, 42, 777, 1990, "Gdansk", "coventry-north"]) {
    for (const district of districts) {
      const block = town.block({ id, district });
      const boxes = [];
      for (const part of block.parts) {
        if (part.lot == null || part.lot < 0) continue;
        const lot = block.lots[part.lot];
        const tag = id + "/" + district + " lot " + part.lot + " (" + lot.kind + ")";

        // Against its own allotment, trim tolerance included.
        const whole = builtFootprint(block, part, -Infinity);
        assert.ok(whole, tag + ": emits no geometry");
        const turned = lot.yaw % 2 === 1;
        const w = turned ? lot.depth : lot.width, d = turned ? lot.width : lot.depth;
        const over = Math.max(lot.x - w / 2 - whole[0], whole[1] - (lot.x + w / 2),
          lot.z - d / 2 - whole[2], whole[3] - (lot.z + d / 2));
        assert.ok(over <= EAVE, tag + ": geometry runs " + over.toFixed(2)
          + " m past the lot it was given, which is a building in the wrong place and not an eave");

        // Against the neighbours, above the ground works.
        const solid = builtFootprint(block, part, 1.0);
        if (solid) boxes.push({ tag, box: solid });
      }
      for (let i = 0; i < boxes.length; i += 1) {
        for (let j = i + 1; j < boxes.length; j += 1) {
          const a = boxes[i].box, b = boxes[j].box;
          const ox = Math.min(a[1], b[1]) - Math.max(a[0], b[0]);
          const oz = Math.min(a[3], b[3]) - Math.max(a[2], b[2]);
          assert.ok(!(ox > 0.05 && oz > 0.05), boxes[i].tag + " and " + boxes[j].tag
            + " stand in the same ground, " + ox.toFixed(1) + " m x " + oz.toFixed(1) + " m of it");
        }
      }
    }
  }
});

// A kind that ignores the depth asked of it is the trap the check above exists
// for, so name the ones that do. This is not a defect — a university has the
// depth a university has — but the layout must gate on `extent` and never on
// the request, and this goes red if a kind quietly changes side.
check("a baked variant reports the footprint it drew, not the one it was asked for", () => {
  const rigid = [];
  for (const kind of kinds) {
    const shallow = town.building(kind, { id: 3, depth: 18 });
    const deeper = town.building(kind, { id: 3, depth: 26 });
    if (Math.abs(shallow.size[2] - deeper.size[2]) < 0.5) rigid.push(kind);
    for (const mesh of [shallow, deeper]) {
      const dx = mesh.bounds.max[0] - mesh.bounds.min[0];
      const dz = mesh.bounds.max[2] - mesh.bounds.min[2];
      assert.ok(Math.abs(mesh.size[0] - dx) < 1e-3 && Math.abs(mesh.size[2] - dz) < 1e-3,
        kind + ": reported size disagrees with its own vertices");
    }
  }
  // Measured, not assumed: eight of the fifteen draw at a fixed depth today.
  assert.ok(rigid.length > 0, "the trap this guards is gone — re-read the anchor gate before deleting it");
  const anchorKinds = new Set();
  for (const district of districts) for (const id of [0, 1, 3, 1990]) {
    const anchor = town.block({ id, district }).anchor;
    if (anchor) anchorKinds.add(anchor);
  }
  for (const kind of anchorKinds) {
    for (const district of districts) {
      const block = town.block({ id: 1990, district });
      if (block.anchor !== kind) continue;
      const lot = block.lots.find((l) => l.kind === kind);
      const built = town.building(kind, { id: 1990, width: lot.width, depth: lot.depth, storeys: lot.storeys });
      assert.ok(built.size[2] <= lot.depth + 1.5 && built.size[0] <= lot.width + 1.5,
        district + ": the " + kind + " anchor was given a lot smaller than the mesh it stamps");
    }
  }
});

check("the same block id builds the same town, in this process and in a fresh load", () => {
  const first = town.block({ id: 1990 });
  const again = town.block({ id: 1990 });
  assert.equal(digest(first), digest(again), "same instance");
  assert.equal(JSON.stringify(first.lots), JSON.stringify(again.lots), "same lot table");

  delete require.cache[require.resolve(meshPath)];
  const fresh = require(meshPath);
  const cold = fresh.block({ id: 1990 });
  assert.equal(digest(first), digest(cold), "fresh module load — the variant cache changes nothing");
  assert.deepEqual(fresh.kinds(), kinds);
  for (const kind of kinds) {
    assert.equal(digest(fresh.building(kind, { id: 7 })), digest(town.building(kind, { id: 7 })), kind + ": stable across loads");
  }
  // String ids hash too, so a town can be keyed by a city name and still be
  // the same town on the next session.
  assert.equal(digest(town.block({ id: "coventry-north" })), digest(fresh.block({ id: "coventry-north" })));
});

check("two different block ids really do build different towns", () => {
  const seen = new Map();
  for (const id of [1, 2, 3, 4, 5, 6, 7, 8]) {
    const block = town.block({ id });
    const key = digest(block);
    assert.ok(!seen.has(key), "id " + id + " built the same geometry as id " + seen.get(key));
    seen.set(key, id);
  }
  // Not just different bytes — a different arrangement. Compare the lot tables,
  // which is where the variation is supposed to live.
  const a = town.block({ id: 1 }), b = town.block({ id: 2 });
  assert.notEqual(JSON.stringify(a.lots.map((l) => l.kind)), JSON.stringify(b.lots.map((l) => l.kind)), "different kinds along the frontage");
  const schemesA = new Set(a.lots.map((l) => l.scheme));
  assert.ok(schemesA.size >= 3, "one block uses several finish schemes");
  // And a district is a different place, not a recolour of the same place.
  assert.notEqual(digest(town.block({ id: 1, district: "residential" })), digest(town.block({ id: 1, district: "industrial" })));
});

check("variety comes from a hash of the id and the lot, with no clock and no entropy source in the file", () => {
  const banned = [
    [/Math\s*\.\s*random/, "Math.random"],
    [/\bDate\b/, "Date"],
    [/\bperformance\s*\./, "performance.*"],
    [/\bhrtime\b/, "process.hrtime"],
    [/getRandomValues/, "crypto.getRandomValues"],
    [/\bcrypto\s*\./, "crypto.*"],
    [/\brequire\s*\(/, "require()"],
    [/\bfetch\s*\(/, "fetch()"],
    [/XMLHttpRequest/, "XMLHttpRequest"],
    [/\bset(?:Timeout|Interval)\s*\(/, "timers"],
    [/\bdocument\s*\./, "document.*"],
    [/\bwindow\s*\./, "window.*"],
    [/\bnavigator\b/, "navigator"],
  ];
  for (const [pattern, name] of banned) {
    assert.ok(!pattern.test(source), "town-mesh.js must not reference " + name);
  }
  assert.ok(/Math\.imul/.test(source), "the hash is still the source of variety");
  // The block hands back the seed material it used, so a town can be rebuilt
  // from its record rather than from its geometry.
  const block = town.block({ id: 4242, district: "commercial" });
  assert.equal(block.blockId, 4242);
  assert.equal(block.district, "commercial");
  assert.equal(block.lod, "close");
});

check("art claims stay inside roadmap section 3 — original, representative, and granting nothing", () => {
  for (const kind of kinds) {
    for (const lod of [0, "map"]) {
      const text = town.building(kind, { id: 2, lod }).description;
      assert.ok(/Original game art/.test(text), kind + " " + lod + ": states the art is original");
      assert.ok(/representative|scenery/i.test(text), kind + " " + lod + ": states it is representative");
    }
  }
  const block = town.block({ id: 5 });
  assert.ok(/representative/.test(block.description) && /not a reconstruction/.test(block.description),
    "the block says what it is and is not");
  assert.ok(/grants no simulation capability/.test(block.description), "the block claims no gameplay effect");
  assert.equal(town.era, 1990);
  assert.equal(town.kit, "temperate_masonry");
});

check("unknown kinds and junk options fail cleanly instead of inventing a building", () => {
  assert.equal(town.building("skyscraper"), null);
  assert.equal(town.building("__proto__"), null);
  assert.equal(town.building(undefined), null);
  assert.equal(town.kindInfo("skyscraper"), null);
  // An unrecognised district falls back to the default rather than throwing or
  // producing an empty tile.
  const fallback = town.block({ id: 9, district: "atlantis" });
  assert.equal(fallback.district, "mixed");
  validate(fallback, "fallback block");
  for (const junk of [undefined, null, {}, { size: ["x", null], street: "wide", lod: "enormous" }]) {
    const block = town.block(junk);
    validate(block, "junk options");
    assert.ok(block.tile[0] >= 70 && block.tile[1] >= 60, "sizes are clamped to something buildable");
  }
});

check("options are read, never written", () => {
  const opts = { id: 12, district: "civic", size: [160, 110], lod: 0 };
  const before = JSON.stringify(opts);
  town.block(opts);
  assert.equal(JSON.stringify(opts), before);
  const one = { id: 12, width: 10.4, storeys: 2, lod: "map" };
  const snapshot = JSON.stringify(one);
  town.building("house", one);
  assert.equal(JSON.stringify(one), snapshot);
});

check("the browser global exports the same contract with no module system present", () => {
  const context = vm.createContext({});
  vm.runInContext(source, context);
  const browser = context.TownMesh;
  assert.ok(browser, "assigns a global when there is no module.exports");
  // Compared as text: the vm realm's Array has a different prototype, and a
  // strict deep-equal would fail on that rather than on the contents.
  assert.equal(browser.kinds().join(","), kinds.join(","));
  assert.equal(browser.version, town.version);
  assert.equal(browser.block({ id: 1990 }).triangleCount, town.block({ id: 1990 }).triangleCount);
  assert.equal(digest(browser.building("row_house", { id: 3 })), digest(town.building("row_house", { id: 3 })));
});

const headline = town.block({ id: 1990 });
const headlineMap = town.block({ id: 1990, lod: "map" });
process.stdout.write(checks + " town mesh checks passed; representative temperate block "
  + headline.lotCount + " lots / " + headline.variantCount + " shared meshes, "
  + headline.triangleCount.toLocaleString("en-US") + " triangles close and "
  + headlineMap.triangleCount.toLocaleString("en-US") + " at map scale, "
  + headline.tile[0] + "m x " + headline.tile[1] + "m.\n");
