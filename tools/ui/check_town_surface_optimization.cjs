#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");

// Access the private builders only inside this test realm. The browser API
// stays unchanged. Closed solids are the independent visibility reference:
// omitting a face must not open a hole or move the nearest exterior surface.
const source = fs.readFileSync(path.resolve(__dirname, "../../spheres-web/ui/town-mesh.js"), "utf8");
const exportLine = "    block, building, kinds, kindInfo,";
assert.ok(source.includes(exportLine));
const context = vm.createContext({});
vm.runInContext(source.replace(exportLine,
  exportLine + " _test: { Builder, glazingBar, sash, railing, chimney, tube, KINDS, SLOT, TALL_STOREY },"), context);
const { Builder, glazingBar, sash, railing, chimney, tube, KINDS, SLOT, TALL_STOREY } = context.TownMesh._test;

function triangles(b) {
  const out = [];
  for (let i = 0; i < b.pos.length; i += 9) {
    const a = b.pos.slice(i, i + 3);
    out.push([a, b.pos.slice(i + 3, i + 6).map((v, j) => v - a[j]),
      b.pos.slice(i + 6, i + 9).map((v, j) => v - a[j])]);
  }
  return out;
}
const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
function nearest(mesh, origin, direction) {
  let best = Infinity;
  for (const [a, e1, e2] of mesh) {
    const p = cross(direction, e2), det = dot(e1, p);
    if (Math.abs(det) < 1e-10) continue;
    const t = origin.map((v, j) => v - a[j]), u = dot(t, p) / det;
    if (u < 0 || u > 1) continue;
    const q = cross(t, e1), v = dot(direction, q) / det;
    if (v < 0 || u + v > 1) continue;
    const distance = dot(e2, q) / det;
    if (distance > 0 && distance < best) best = distance;
  }
  return best;
}
let rays = 0;
function sameExterior(label, closed, trimmed, min, max, cameras) {
  assert.ok(trimmed.pos.length < closed.pos.length, label + ": actually removes triangles");
  const before = triangles(closed), after = triangles(trimmed);
  for (const origin of cameras) {
    // Offset samples avoid coincident quad diagonals and hit all narrow frame
    // members repeatedly. Include oblique/grazing views from all four sides.
    for (let ix = 0; ix < 67; ix += 1) for (let iy = 0; iy < 71; iy += 1) {
      const aim = [min[0] + (max[0] - min[0]) * (ix + 0.37) / 67,
        min[1] + (max[1] - min[1]) * (iy + 0.61) / 71, min[2]];
      const direction = aim.map((v, j) => v - origin[j]);
      const a = nearest(before, origin, direction), b = nearest(after, origin, direction);
      assert.ok(a === b || Math.abs(a - b) < 1e-8,
        label + ": exterior changed from " + origin + " towards " + aim + " (" + a + "/" + b + ")");
      rays += 1;
    }
  }
}
const cameras = [[0, 2, 8], [8, 2, 0.04], [-8, 2, 0.04], [0, 20, 0.04],
  [0, -10, 0.04], [10, 10, 0.5], [-10, -10, 0.5]];

function frameFixture(trimmed) {
  const b = new Builder(2);
  b.wall(-4, 4, 0, 5, -2, 0, 0, 1);
  b.opening(-0.75, 0.75, 1, 3, 0);
  // Reveals and opaque glazing enclose the aperture, just as in a real sash.
  for (const x of [-0.75, 0.75]) b.box(x - 0.001, x + 0.001, 1, 3, -0.17, 0, 0, 1);
  for (const y of [1, 3]) b.box(-0.75, 0.75, y - 0.001, y + 0.001, -0.17, 0, 0, 1);
  b.box(-0.75, 0.75, 1, 3, -0.18, -0.17, 3, 1);
  const bars = [[-0.75, -0.68, 1, 3, -0.165, -0.095], [0.68, 0.75, 1, 3, -0.165, -0.095],
    [-0.75, 0.75, 1, 1.07, -0.165, -0.095], [-0.75, 0.75, 2.93, 3, -0.165, -0.095],
    [-0.75, 0.75, 1.95, 2.05, -0.165, -0.075], [-0.026, 0.026, 1, 3, -0.17, -0.12]];
  for (const r of bars) {
    if (trimmed) glazingBar(b, ...r, 1, -0.75, 0.75, 1, 3);
    else b.box(...r, 2, 1);
  }
  b.flushFaces();
  return b;
}
sameExterior("sealed frame faces", frameFixture(false), frameFixture(true), [-0.95, 0.85, -0.14], [1, 3.2], cameras);

function sashFixture(duplicate) {
  const b = new Builder(2);
  b.wall(-4, 4, 0, 5, -2, 0, 0, 1);
  sash(b, 0, 1, 1.5, 2, 0, 2, 2, true);
  if (duplicate) b.box(-0.75, 0.75, 2 - 0.028, 2 + 0.028, -0.17, -0.12, 2, 1.05);
  b.flushFaces();
  return b;
}
sameExterior("meeting rail covers its duplicate", sashFixture(true), sashFixture(false), [-0.95, 0.85, -0.14], [1, 3.2], cameras);

function railFixture(trimmed) {
  const b = new Builder(2), box = b.box;
  if (!trimmed) b.box = function (...args) { return box.apply(this, args.slice(0, 8)); };
  railing(b, -2, 2, 0, 0, 1, 0.24);
  return b;
}
sameExterior("covered baluster caps and retained end caps", railFixture(false), railFixture(true), [-2.1, -0.1, 0], [2.1, 1.1],
  [[0, 3, 4], [4, 2, 0.5], [-4, 2, 0.5], [0, -2, 2], [0, 3, -4]]);

function jambFixture(closed, dress) {
  const b = new Builder(2);
  b.wall(-4, 4, 0, 5, -2, 0, SLOT.WALL, 1);
  sash(b, 0, 1, 1.5, 2, 0, 2, 3, dress);
  if (closed) {
    // Restore the full old boxes. Their exposed sides coincide exactly; any
    // wrongly omitted cap or rear face would become the nearest hit here.
    b.box(-0.88, -0.75, 1, 3, -0.02, 0.05, SLOT.TRIM, 0.98);
    b.box(0.75, 0.88, 1, 3, -0.02, 0.05, SLOT.TRIM, 0.98);
  }
  b.flushFaces();
  return b;
}
for (const dress of [true, false]) sameExterior("jamb ends under " + (dress ? "beveled" : "plain") + " dressings",
  jambFixture(true, dress), jambFixture(false, dress), [-0.95, 0.85, -0.14], [1, 3.2], cameras);

function chimneyFixture(closed) {
  const b = new Builder(2);
  chimney(b, 0, 0, 0, 3, 1, 0.8, 2);
  if (closed) for (const x of [-0.3, 0.3]) {
    tube(b, x, 0, 3, 3.58, 0.145, 0.125, 9, SLOT.ROOF, 1.1);
    tube(b, x, 0, 3.5, 3.62, 0.09, 0.09, 9, SLOT.DARK, 0.7);
  }
  return b;
}
sameExterior("opaque chimney rims enclose their interior", chimneyFixture(true), chimneyFixture(false), [-0.7, -0.1, 0], [0.7, 3.8],
  [[0, 5, 6], [6, 3, 1], [-6, 3, -1], [0, 10, 0], [0, -4, 1]]);

// Count the actual glass triangles on the rear range, not source strings or
// the building's total. The two lower courtyard bays and every upper bay
// remain; only bays completely inside an attached wing disappear.
for (const detail of [1, 2]) for (const width of [36, 42]) for (const storeys of [3, 4]) {
  const b = new Builder(detail), wingH = TALL_STOREY + 3.5;
  KINDS.university.close(b, { w: width, d: 40, storeys });
  b.flushFaces();
  let exposed = 0, buried = 0;
  for (let i = 0; i < b.pos.length; i += 9) {
    if (b.slot[i / 3] !== SLOT.GLASS) continue;
    if (![2, 5, 8].every(j => Math.abs(b.pos[i + j] - 4.17) < 1e-7)) continue;
    const x = (b.pos[i] + b.pos[i + 3] + b.pos[i + 6]) / 3;
    const y = (b.pos[i + 1] + b.pos[i + 4] + b.pos[i + 7]) / 3;
    if (Math.abs(x) >= width / 2 - 12 && y <= wingH) buried += 1;
    else exposed += 1;
  }
  assert.equal(buried, 0, "no glass enclosed by an attached wing");
  assert.equal(exposed, (2 * 2 + (storeys - 2) * 6) * 2, "retain courtyard and upper rear windows");
}
console.log("6 town surface optimizations preserve nearest exterior geometry across " + rays
  + " ray samples; all 8 university width/storey/detail cases retain their exposed rear windows");
