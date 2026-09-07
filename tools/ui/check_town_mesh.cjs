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
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");

const meshPath = path.resolve(__dirname, "../../spheres-web/ui/town-mesh.js");
const town = require(meshPath);
const source = fs.readFileSync(meshPath, "utf8");

// Roadmap section 4, "Building close view / map" and "Scene assembly", with
// the close band raised for the detail pass.
//
// WHY THE CLOSE CEILING MOVED, 12,000 -> 30,000. Section 4 says in its own
// text that its numbers are initial budgets to validate on the machine, not
// measured performance promises, and 12,000 was where "massing with roofs on
// it" happened to land. A building you can walk up to needs window reveals,
// cills, lintels, gutters, downpipes, slate courses, dormers and a kerb, and
// those cost triangles that a card and a map do not have to pay: that is what
// the third LOD below is for.
//
// WHAT DID NOT MOVE, and must not. The map band is unchanged at 100..800 per
// building and the map assembly is now pinned at 6,000 rather than floating at
// a tenth of the scene budget, because a map full of towns is the one place
// where the cost is multiplied by the number of settlements on screen.
const CLOSE_MIN = 4000, CLOSE_MAX = 30000;
const MID_MIN = 900, MID_MAX = 12000;
const MAP_MIN = 100, MAP_MAX = 800;
const SCENE_MAX = 400000, SCENE_MID_MAX = 150000, SCENE_MAP_MAX = 6000;

let checks = 0;
function check(label, fn) { fn(); checks += 1; process.stdout.write("PASS " + label + "\n"); }
function digest(mesh) {
  return crypto.createHash("sha256")
    .update(Buffer.from(mesh.positions.buffer))
    .update(Buffer.from(mesh.normals.buffer))
    .update(Buffer.from(mesh.colors.buffer))
    .digest("hex");
}

/// How much of a mesh is smooth shaded. A triangle counts as flat when its
/// three vertex normals are bit-identical, which is exactly what `tri` emits
/// and exactly what the weld replaces.
function shading(mesh) {
  let flat = 0, smooth = 0;
  const n = mesh.normals;
  for (let i = 0; i < n.length; i += 9) {
    let same = true;
    for (let v = 1; v < 3; v += 1) for (let k = 0; k < 3; k += 1) if (n[i + v * 3 + k] !== n[i + k]) same = false;
    if (same) flat += 1; else smooth += 1;
  }
  return { flat, smooth, share: smooth / (flat + smooth) };
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
  //
  // THE FACING BAR MOVED, AND WHY IT IS NOT WEAKER. It used to be "every
  // vertex normal is the face normal to within 1e-3", which is a statement
  // that the mesh is FLAT SHADED, not a statement that it is correctly wound —
  // and it was the single thing stopping this kit from smoothing a drainpipe.
  // What replaced it tests both halves separately and tests more of them:
  //   * every one of the three vertex normals, not just the first, must sit on
  //     the outward side of its own face. An inverted winding, a mirrored
  //     part or a normal welded across a solid still fails, exactly as before.
  //   * the deviation is capped at SMOOTH_MAX_DEV, which is tighter than the
  //     generator's own 51.7-degree crease, so a weld that ran away across an
  //     edge it had no business crossing fails here rather than shading badly.
  //   * a FLAT triangle — all three normals bit-identical, which is still most
  //     of the town — must satisfy the old 0.999 test unchanged.
  const SMOOTH_MAX_DEV = 0.42;   // cos ~65 deg, outside anything the crease can produce
  let flat = 0, smooth = 0;
  for (let i = 0; i < mesh.positions.length; i += 9) {
    const a = mesh.positions.subarray(i, i + 3);
    const b = mesh.positions.subarray(i + 3, i + 6);
    const c = mesh.positions.subarray(i + 6, i + 9);
    const u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    const n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    const area = Math.hypot(n[0], n[1], n[2]);
    assert.ok(area > 1e-7, tag + ": non-degenerate triangle " + (i / 9));
    let identical = true;
    for (let v3 = 1; v3 < 3; v3 += 1) {
      for (let k = 0; k < 3; k += 1) if (mesh.normals[i + v3 * 3 + k] !== mesh.normals[i + k]) identical = false;
    }
    for (let v3 = 0; v3 < 3; v3 += 1) {
      const j = i + v3 * 3;
      const facing = (n[0] * mesh.normals[j] + n[1] * mesh.normals[j + 1] + n[2] * mesh.normals[j + 2]) / area;
      assert.ok(facing > (identical ? 0.999 : SMOOTH_MAX_DEV),
        tag + ": normal follows winding at " + (i / 9) + " vertex " + v3 + " (" + facing.toFixed(3) + ")");
    }
    if (identical) flat += 1; else smooth += 1;
  }
  assert.equal(flat + smooth, mesh.triangleCount, tag + ": every triangle classified");
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

check("every kind builds sound geometry at all three LODs", () => {
  for (const kind of kinds) {
    validate(town.building(kind, { id: 3 }), kind + " close");
    validate(town.building(kind, { id: 3, lod: "mid" }), kind + " mid");
    validate(town.building(kind, { id: 3, lod: "map" }), kind + " map");
  }
});

check("every kind, at every frontage and storey count it offers, lands inside the section 4 budget", () => {
  for (const kind of kinds) {
    const info = town.kindInfo(kind);
    for (const width of info.widths) {
      for (const storeys of info.storeys) {
        const close = town.building(kind, { id: 11, width, storeys });
        const mid = town.building(kind, { id: 11, width, storeys, lod: "mid" });
        const map = town.building(kind, { id: 11, width, storeys, lod: "map" });
        assert.ok(close.triangleCount >= CLOSE_MIN && close.triangleCount <= CLOSE_MAX,
          kind + " " + width + "m/" + storeys + ": close view " + close.triangleCount + " outside " + CLOSE_MIN + ".." + CLOSE_MAX);
        assert.ok(mid.triangleCount >= MID_MIN && mid.triangleCount <= MID_MAX,
          kind + " " + width + "m/" + storeys + ": card " + mid.triangleCount + " outside " + MID_MIN + ".." + MID_MAX);
        assert.ok(map.triangleCount >= MAP_MIN && map.triangleCount <= MAP_MAX,
          kind + " " + width + "m/" + storeys + ": map " + map.triangleCount + " outside " + MAP_MIN + ".." + MAP_MAX);
        // The three levels must be ORDERED. A card that costs more than the
        // close view, or a map heavier than a card, is a level that has
        // stopped being a cheaper view of the same building.
        assert.ok(close.triangleCount > mid.triangleCount && mid.triangleCount > map.triangleCount,
          kind + " " + width + "m/" + storeys + ": LODs are not ordered ("
          + close.triangleCount + " / " + mid.triangleCount + " / " + map.triangleCount + ")");
      }
    }
  }
});

// SMOOTH SHADING IS A CONTRACT, not a nicety. Every curved surface in this kit
// — drainpipes, chimney pots, gutters, lamp columns, bollards, bins, columns,
// tree trunks and crowns, car wheels, the stadium skin — is welded across its
// own segments, and the whole reason the facing bar above was reopened was to
// let that happen. If a refactor silently drops the weld the geometry still
// validates and the town quietly goes back to reading as a bag of prisms, so
// the share is measured here and floored per kind.
check("curved surfaces are smooth shaded, and flat ones are not", () => {
  const floors = {
    house: 0.06, row_house: 0.05, low_apartment: 0.04, mid_apartment: 0.03, high_apartment: 0.05,
    office: 0.05, shop: 0.05, warehouse: 0.05, civic: 0.05, school: 0.05, hospital: 0.05,
    university: 0.04, stadium: 0.01, park: 0.15, utility: 0.05,
  };
  for (const kind of kinds) {
    const share = shading(town.building(kind, { id: 3 })).share;
    assert.ok(share >= floors[kind], kind + ": only " + (share * 100).toFixed(1)
      + "% of triangles are smooth shaded, below the " + (floors[kind] * 100).toFixed(0) + "% this kit's round parts account for");
    // And the other half of the claim: a town is mostly masonry, glazing and
    // roof planes, and those must stay flat. A mesh that came back mostly
    // smooth would mean a weld had escaped its group and rounded the walls.
    assert.ok(share < 0.6, kind + ": " + (share * 100).toFixed(1) + "% smooth — flat surfaces are being welded");
  }
  const blockShare = shading(town.block({ id: 1990 })).share;
  assert.ok(blockShare > 0.05, "an assembled block is " + (blockShare * 100).toFixed(1) + "% smooth shaded");
});

// A ROOF IS THE BIGGEST SINGLE SURFACE ON A TEMPERATE BUILDING and there is no
// texture path here, so the only thing that can tell slate from a grey sheet is
// the course lines. `slopeCourses` builds them — a tread and a lip, four
// triangles a course, up to twenty courses a slope — and for the whole of the
// first detail pass they did nothing at all: both ends of every tread were
// pushed out by the same 35 mm, so all twenty treads landed on ONE plane and
// every lip was buried underneath it. Measured on the house: 34 of its 140
// slope triangles in a single plane, and a roof that rendered as the bare sheet
// the courses exist to replace. It cost 160 triangles a roof and bought nothing,
// and nothing in this file went red.
//
// So the bar is on the property that failed. A pitched slope may not put more
// than 24 triangles on one plane. Measured against the flat version: 33 to 40
// on all seven pitched kinds. Measured against the tapered courses: 4 (house,
// shop, office) to 20 (the low block's mansard deck, which is legitimately one
// plane). The gap is wide and the bar sits in it.
check("a pitched roof is laid in courses, not drawn as one grey sheet", () => {
  for (const kind of ["house", "row_house", "shop", "school", "university", "civic", "low_apartment"]) {
    const mesh = town.building(kind, { id: 3 });
    const P = mesh.positions, N = mesh.normals;
    const eaves = mesh.bounds.min[1] + (mesh.bounds.max[1] - mesh.bounds.min[1]) * 0.55;
    const planes = new Map();
    for (let i = 0; i < P.length; i += 9) {
      // A PITCHED slope: facing up but not level, which excludes both the walls
      // and the flat roofs, and above the eaves, which excludes the ground.
      if (N[i + 1] < 0.35 || N[i + 1] > 0.985) continue;
      if ((P[i + 1] + P[i + 4] + P[i + 7]) / 3 < eaves) continue;
      const d = P[i] * N[i] + P[i + 1] * N[i + 1] + P[i + 2] * N[i + 2];
      const key = N[i].toFixed(3) + "," + N[i + 1].toFixed(3) + "," + N[i + 2].toFixed(3) + "|" + d.toFixed(3);
      planes.set(key, (planes.get(key) || 0) + 1);
    }
    let total = 0, biggest = 0;
    for (const count of planes.values()) { total += count; if (count > biggest) biggest = count; }
    assert.ok(total >= 100, kind + ": only " + total + " triangles of pitched roof — the slopes have gone");
    assert.ok(biggest <= 24, kind + ": " + biggest + " of its " + total
      + " slope triangles lie on one plane — the courses are coplanar again and the roof is a sheet");
  }
});

// THE OPENINGS HAVE TO BE OPEN. Every window, shopfront and fanlight in this
// kit is authored as a reveal running back from the wall plane to a pane 170 mm
// behind it — and for the whole of the first pass they were set into a SOLID
// box, whose own face was drawn straight across the front of them. Nothing
// failed: the buffers validated, the budgets held, the digests were stable, and
// the town was a set of blank walls with painted-on surrounds. Every pane,
// glazing bar, reveal and mullion was buried where no camera could reach it.
//
// So this test looks through the wall. It finds the glass by its palette ratio
// — GLASS is the one fixed slot whose r:g:b is 0.20:0.26:0.30 — steps a
// centimetre out along each pane's own normal, and asks whether anything stands
// in front of it within half a metre. Against the buried version almost every
// pane is occluded; against a punched wall almost none is.
function glassTriangles(mesh) {
  const out = [];
  for (let i = 0; i < mesh.positions.length; i += 9) {
    const r = mesh.colors[i], g = mesh.colors[i + 1], b = mesh.colors[i + 2];
    if (!(b > 0.02)) continue;
    if (Math.abs(r / b - 0.20 / 0.30) > 0.02 || Math.abs(g / b - 0.26 / 0.30) > 0.02) continue;
    out.push(i);
  }
  return out;
}
/// Is anything standing between this pane and the outside world? The near
/// limit is 105 mm rather than zero, and that is the whole subtlety of the
/// test: a glazing bar, a mullion and a frame section all sit within 75 mm of
/// their own glass by design, and counting those as blockers would fail a
/// perfectly good window. The wall a pane is set into is 170 mm away.
function blocked(mesh, i, reach) {
  const P = mesh.positions;
  const cx = (P[i] + P[i + 3] + P[i + 6]) / 3, cy = (P[i + 1] + P[i + 4] + P[i + 7]) / 3;
  const cz = (P[i + 2] + P[i + 5] + P[i + 8]) / 3;
  const nx = mesh.normals[i], ny = mesh.normals[i + 1], nz = mesh.normals[i + 2];
  const ox = cx + nx * 0.012, oy = cy + ny * 0.012, oz = cz + nz * 0.012;
  for (let t = 0; t < P.length; t += 9) {
    if (t === i) continue;
    const ax = P[t], ay = P[t + 1], az = P[t + 2];
    const e1x = P[t + 3] - ax, e1y = P[t + 4] - ay, e1z = P[t + 5] - az;
    const e2x = P[t + 6] - ax, e2y = P[t + 7] - ay, e2z = P[t + 8] - az;
    const px = ny * e2z - nz * e2y, py = nz * e2x - nx * e2z, pz = nx * e2y - ny * e2x;
    const det = e1x * px + e1y * py + e1z * pz;
    if (Math.abs(det) < 1e-9) continue;
    const inv = 1 / det, tx = ox - ax, ty = oy - ay, tz = oz - az;
    const u = (tx * px + ty * py + tz * pz) * inv;
    if (u < 0 || u > 1) continue;
    const qx = ty * e1z - tz * e1y, qy = tz * e1x - tx * e1z, qz = tx * e1y - ty * e1x;
    const v = (nx * qx + ny * qy + nz * qz) * inv;
    if (v < 0 || u + v > 1) continue;
    const hit = (e2x * qx + e2y * qy + e2z * qz) * inv;
    if (hit > 0.105 && hit < reach) return true;
  }
  return false;
}
check("a window is a hole in the wall, not a pane buried behind one", () => {
  // Only OUTWARD-FACING vertical glazing is asked the question. The back and
  // return faces of a glazed box — a fanlight, a shopfront pane with
  // thickness, a roof light — are inside their own solid and are supposed to
  // be, and a test that counted those would be measuring modelling style
  // rather than whether the town has windows.
  for (const kind of ["house", "row_house", "shop", "civic", "office", "university"]) {
    const mesh = town.building(kind, { id: 3 });
    const cx = (mesh.bounds.min[0] + mesh.bounds.max[0]) / 2;
    const cz = (mesh.bounds.min[2] + mesh.bounds.max[2]) / 2;
    const glass = glassTriangles(mesh).filter((i) => {
      if (Math.abs(mesh.normals[i + 1]) > 0.3) return false;
      const dx = (mesh.positions[i] + mesh.positions[i + 3] + mesh.positions[i + 6]) / 3 - cx;
      const dz = (mesh.positions[i + 2] + mesh.positions[i + 5] + mesh.positions[i + 8]) / 3 - cz;
      const len = Math.hypot(dx, dz) || 1;
      return (mesh.normals[i] * dx + mesh.normals[i + 2] * dz) / len > 0.25;
    });
    assert.ok(glass.length >= 24, kind + ": only " + glass.length + " outward panes — the openings have gone");
    let seen = 0;
    for (const i of glass) if (!blocked(mesh, i, 0.6)) seen += 1;
    const share = seen / glass.length;
    // Measured against the buried version this replaced: house 14%, terrace
    // 16%, shop 13%, civic 0%, office 16%, university 0%. Measured against
    // the punched walls: 77% to 98%. The bar sits between the two, well clear
    // of both.
    assert.ok(share > 0.65, kind + ": only " + (share * 100).toFixed(0)
      + "% of its glazing can be seen from outside — the wall is being drawn across the openings again");
  }
});

// A HOUSE'S BACK EXTENSION BELONGS AT THE BACK, and the elevations in this kit
// are drawn by pushing a cardinal yaw, which mirrors BOTH x and z — so a sign
// that reads correctly in the head lands on the opposite side of the building.
// The terrace's outriggers did exactly that: 2.4 m of blank brick standing in
// the FRONT garden of every unit, across its ground-floor sashes and its front
// door and through its own dwarf wall. Nothing caught it. It is on the right
// lot, so the lot checks pass; it is a sound closed box, so validate passes;
// and it stands 2.4 m clear of the wall, so the glazing test — which looks for
// a blocker within 600 mm, because it was written to find a wall drawn across
// an opening — walks straight past it.
//
// The band is chosen to see the building and nothing else: 1.6 m is above a
// parked car's roof, a hedge and a garden railing, and 2.6 m is below the
// first-floor cills. Measured with the outriggers on the street: 2.16 m proud.
// Measured with them behind the house, which is where they are now: -0.09 m,
// i.e. nothing at all stands in front of the wall. The bar is 0.60 m, which
// still admits a porch, a door hood or a bay.
//
// It asks this of the two kinds that draw a back addition. It is not a general
// rule and it does not pretend to be one: a civic portico stands 1.8 m proud
// on purpose, a park's trees stand 5 m proud of the pavilion behind them, and
// a check that measured "the furthest thing in front of the wall" across the
// whole kit would be measuring landscape.
check("a back addition is behind the house, not in its front garden", () => {
  for (const kind of ["row_house", "house"]) {
    const mesh = town.building(kind, { id: 3 });
    const P = mesh.positions, N = mesh.normals;
    let wall = -Infinity;
    for (let i = 0; i < P.length; i += 9) {
      if (N[i + 2] < 0.9) continue;                       // square on to the street
      const y = (P[i + 1] + P[i + 4] + P[i + 7]) / 3;
      if (y < 4.0 || y > 6.5) continue;                   // an upper storey, above anything projecting
      wall = Math.max(wall, P[i + 2], P[i + 5], P[i + 8]);
    }
    assert.ok(wall > -Infinity, kind + ": no upper-storey street wall to measure against");
    let proud = -Infinity;
    for (let i = 0; i < P.length; i += 9) {
      const y = (P[i + 1] + P[i + 4] + P[i + 7]) / 3;
      if (y < 1.6 || y > 2.6) continue;
      proud = Math.max(proud, P[i + 2] - wall, P[i + 5] - wall, P[i + 8] - wall);
    }
    assert.ok(proud <= 0.60, kind + ": something stands " + proud.toFixed(2)
      + " m in front of its own street wall at head height — a back addition drawn on the front elevation");
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
    const mid = town.block({ id: 1990, district, lod: "mid" });
    const map = town.block({ id: 1990, district, lod: "map" });
    validate(close, district + " block close");
    validate(mid, district + " block mid");
    validate(map, district + " block map");
    assert.ok(close.triangleCount < SCENE_MAX, district + ": " + close.triangleCount + " over the " + SCENE_MAX + " scene budget");
    assert.ok(mid.triangleCount < SCENE_MID_MAX, district + ": card assembly " + mid.triangleCount + " over " + SCENE_MID_MAX);
    // PINNED, not derived. This used to read SCENE_MAX / 10, so raising the
    // close budget would have raised the map budget with it — and the map is
    // the one level where the cost is multiplied by every settlement on
    // screen. 6,000 is a hard number and it does not move when the close
    // view's does.
    assert.ok(map.triangleCount < SCENE_MAP_MAX, district + ": map assembly " + map.triangleCount + " over the hard " + SCENE_MAP_MAX);
    assert.ok(close.triangleCount > mid.triangleCount * 2 && mid.triangleCount > map.triangleCount * 4,
      district + ": the three assembled levels are not separated ("
      + close.triangleCount + " / " + mid.triangleCount + " / " + map.triangleCount + ")");
    assert.ok(close.lotCount >= 8, district + ": " + close.lotCount + " lots is not a block");
    assert.equal(close.lots.length, close.lotCount);
    assert.equal(close.tile[0], close.bounds.max[0] - close.bounds.min[0], district + ": tile width is the mesh width");
    assert.equal(close.tile[1], close.bounds.max[2] - close.bounds.min[2], district + ": tile depth is the mesh depth");
  }
});

// The scene budget was only ever asked of the DEFAULT tile, and `block` takes
// a size from 70x60 up to 420x320 — 8.7 times the area. Nothing tested that,
// and a caller asking for the largest tile gets a block with three times the
// lots on it. The budget that actually means something across that range is a
// DENSITY: a tile may cost the scene budget per default-tile-area of ground,
// and no more. Measured today: 14.1 triangles per square metre close against a
// 26.0 ceiling, and 0.27 per square metre at map scale against 0.39.
check("the budgets hold at every tile size the API accepts, not only the default one", () => {
  const area = town.defaultTile[0] * town.defaultTile[1];
  for (const size of [[70, 60], [420, 320]]) {        // the two ends of the range
    const scale = Math.max(1, (size[0] * size[1]) / area);
    for (const district of districts) {
      const of = (lod) => town.block({ id: 43, district, size, lod }).triangleCount;
      const close = of("close"), mid = of("mid"), map = of("map");
      const tag = size.join("x") + "/" + district + ": ";
      assert.ok(close <= SCENE_MAX * scale, tag + close + " close over " + Math.round(SCENE_MAX * scale));
      assert.ok(mid <= SCENE_MID_MAX * scale, tag + mid + " card over " + Math.round(SCENE_MID_MAX * scale));
      assert.ok(map <= SCENE_MAP_MAX * scale, tag + map + " map over " + Math.round(SCENE_MAP_MAX * scale));
      assert.ok(close > mid && mid > map, tag + "levels out of order");
    }
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

// The geometric sweep below is the most expensive thing in this file — a close
// block is ~40 ms warm and the two checks that read one want the same blocks —
// so the blocks are built ONCE here and both read them.
//
// WHICH IDS, AND WHY THESE. The list was eleven arbitrary ids and it hid a
// defect for a whole pass: on block 9's civic district a school and a civic
// building stood 50.4 mm into each other over a 21 m run, which is over the
// 50 mm bar twenty lines down, and the check stayed green because 9 was not one
// of the eleven. So the four ids that exposed the two defects this pass fixed
// are named — 9 for the frontage gap, 2 / 142 / 198 for the LOD-dependent
// anchor — and they stay named. A sampled sweep is only as good as its sample,
// and the honest way to keep one honest is to pin the cases that broke it.
const SWEEP_IDS = [0, 1, 2, 3, 5, 9, 17, 42, 142, 198, 777, 1990, "Gdansk", "coventry-north"];
const sweep = [];
for (const id of SWEEP_IDS) {
  for (const district of districts) sweep.push({ id, district, block: town.block({ id, district }) });
}

check("lots tile the block without overlapping each other or leaving the tile", () => {
  for (const { id, district, block } of sweep) {
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
});

// A TOWN THAT REARRANGES ITSELF WHEN THE CAMERA PULLS BACK is not a cheaper
// view of the same town, and the per-BUILDING map test above cannot see it —
// it compares one building against itself. The layout can still move, and it
// did: the block anchor gate baked its candidate at the level being built, and
// a map-scale hospital, which has no eaves overhang, no gutter and no portico,
// measured small enough to pass a gate the close-scale one failed. Over 1,000
// blocks that put a hospital in the middle of civic blocks 2, 142 and 198 at
// map zoom and a park in the same ground the moment the camera came in.
//
// So the lot table — kind, position, size and orientation of every lot — must
// be IDENTICAL at all three levels. The mid and map blocks are cheap (1-15 ms
// against 40 for a close one), so this rides along on the sweep already built.
check("the card and the map lay the same town out as the close view does", () => {
  const table = (block) => JSON.stringify(block.lots.map((lot) =>
    [lot.kind, lot.x, lot.z, lot.width, lot.depth, lot.yaw, lot.storeys]));
  for (const { id, district, block } of sweep) {
    const wanted = table(block);
    for (const lod of ["mid", "map"]) {
      const other = town.block({ id, district, lod });
      assert.equal(table(other), wanted, id + "/" + district + ": the " + lod
        + " level lays the block out differently from the close view");
      assert.equal(other.anchor, block.anchor, id + "/" + district + ": " + lod + " anchors a different kind");
      assert.equal(other.tile[0], block.tile[0], id + "/" + district + ": " + lod + " is a different tile");
      assert.equal(other.tile[1], block.tile[1], id + "/" + district + ": " + lod + " is a different tile");
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
//
// AND THE SAMPLE IS THE THING THAT DECIDES WHETHER IT WORKS. The list of ids
// this used to walk was its own, eleven long, and it missed block 9 — where the
// frontage gap had closed to 1.0 m against 1.14 m of trim on the two sides of
// it, and a school and a civic building stood 50.4 mm into each other over a
// 21 m run. The bar was right; the sample was not. It now walks the shared
// SWEEP_IDS above, which names that case.
check("what is drawn stays on its own lot and out of its neighbour", () => {
  const EAVE = 1.5; // trim may hang over the boundary; a wing may not.
  // And a SECOND, tighter bar on the same measurement, because the generator's
  // frontage gap is what keeps two neighbours apart and it is 1.6 m. Two lots
  // at that minimum have 1.6 m between their boundaries, so no kind may hang
  // more than half of it — 0.80 m — past its own line without the two meeting
  // in the middle. Measured across this sweep: 0.70 m, the shop's fascia, with
  // the civic portico at 0.59 and the school's eaves at 0.55. A kind that grows
  // a wider eave than that has to be paid for in the gap, and this is where the
  // bill arrives.
  const TRIM = 0.80;
  let worstTrim = 0, worstTrimTag = "";
  for (const { id, district, block } of sweep) {
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
      if (over > worstTrim) { worstTrim = over; worstTrimTag = tag; }

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
  assert.ok(worstTrim <= TRIM, worstTrimTag + " hangs " + worstTrim.toFixed(2)
    + " m past its lot line, more than half the 1.6 m the layout guarantees between two lots"
    + " — widen the gap in `subdivide` or pull the trim in");
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
  // ALL THREE LEVELS, not just the close one. The card and the map are welded
  // by the same `smoothed` pass, which gathers faces into a Map keyed by
  // quantised position and sums float normals in whatever order it meets them:
  // insertion order fixes both, and that claim is a test here rather than a
  // comment in the generator. The card block digest is the one that matters
  // most, because a block runs the weld over twenty stamped variants.
  for (const kind of kinds) {
    for (const lod of ["close", "mid", "map"]) {
      assert.equal(digest(fresh.building(kind, { id: 7, lod })), digest(town.building(kind, { id: 7, lod })),
        kind + " " + lod + ": stable across loads");
    }
  }
  for (const lod of ["mid", "map"]) {
    assert.equal(digest(fresh.block({ id: 1990, lod })), digest(town.block({ id: 1990, lod })),
      "assembled block at " + lod + ": stable across loads");
  }
  // String ids hash too, so a town can be keyed by a city name and still be
  // the same town on the next session.
  assert.equal(digest(town.block({ id: "coventry-north" })), digest(fresh.block({ id: "coventry-north" })));
});

// A SECOND PROCESS, which the same-process reload above cannot stand in for.
// A reload re-runs the factory but keeps this process's own heap layout, its
// warmed JIT and whatever floating-point state the tests before it left; a town
// saved as an id on one machine and rebuilt on another gets none of that. The
// weld is the part that could plausibly differ — it sums float normals gathered
// in a Map — and iron rule 1 says determinism is sacred, so the claim is made
// here where it can go red rather than in a comment.
check("a second node process builds the same town, byte for byte", () => {
  const probe = "const town = require(" + JSON.stringify(meshPath) + ");"
    + "const h = require('node:crypto').createHash('sha256');"
    + "const add = (m) => h.update(Buffer.from(m.positions.buffer))"
    + ".update(Buffer.from(m.normals.buffer)).update(Buffer.from(m.colors.buffer));"
    + "for (const d of town.districts()) for (const lod of ['close','mid','map']) add(town.block({id:1990,district:d,lod}));"
    + "for (const k of town.kinds()) for (const lod of ['close','mid','map']) add(town.building(k,{id:7,lod}));"
    + "add(town.block({id:'coventry-north'}));"
    + "process.stdout.write(h.digest('hex'));";
  const child = cp.execFileSync(process.execPath, ["-e", probe], { encoding: "utf8" });
  const local = crypto.createHash("sha256");
  const add = (m) => local.update(Buffer.from(m.positions.buffer))
    .update(Buffer.from(m.normals.buffer)).update(Buffer.from(m.colors.buffer));
  for (const d of districts) for (const lod of ["close", "mid", "map"]) add(town.block({ id: 1990, district: d, lod }));
  for (const k of kinds) for (const lod of ["close", "mid", "map"]) add(town.building(k, { id: 7, lod }));
  add(town.block({ id: "coventry-north" }));
  assert.equal(child, local.digest("hex"), "a second node process disagrees about the town");
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
    // All THREE levels, the card included. The card is the close call graph
    // with the small stuff switched off, and the failure it can produce is a
    // description that still describes the close view — a card claiming slate
    // courses and glazing bars it does not draw. So it must say in its own
    // words that it is the cheaper view, and it must not still say "massing
    // only", which is the close view's phrase.
    for (const lod of [0, "mid", "map"]) {
      const text = town.building(kind, { id: 2, lod }).description;
      assert.ok(/Original game art/.test(text), kind + " " + lod + ": states the art is original");
      assert.ok(/representative|scenery/i.test(text), kind + " " + lod + ": states it is representative");
      if (lod === "mid") {
        assert.ok(/Card/.test(text), kind + ": the card LOD does not say it is a card");
        assert.ok(!/Massing only/i.test(text), kind + ": the card LOD still carries the close view's own words");
        assert.notEqual(text, town.building(kind, { id: 2 }).description,
          kind + ": the card LOD claims to be the close view");
      }
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
const headlineMid = town.block({ id: 1990, lod: "mid" });
const headlineMap = town.block({ id: 1990, lod: "map" });
const headlineShade = shading(headline);
process.stdout.write(checks + " town mesh checks passed; representative temperate block "
  + headline.lotCount + " lots / " + headline.variantCount + " shared meshes, "
  + headline.triangleCount.toLocaleString("en-US") + " triangles close / "
  + headlineMid.triangleCount.toLocaleString("en-US") + " card / "
  + headlineMap.triangleCount.toLocaleString("en-US") + " map, "
  + (headlineShade.share * 100).toFixed(1) + "% of them smooth shaded, "
  + headline.tile[0] + "m x " + headline.tile[1] + "m.\n");
