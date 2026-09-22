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
  exportLine + " _test: { Builder, glazingBar, sash, railing },"), context);
const { Builder, glazingBar, sash, railing } = context.TownMesh._test;

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
console.log("3 town surface optimizations preserve nearest exterior geometry across " + rays + " ray samples");
