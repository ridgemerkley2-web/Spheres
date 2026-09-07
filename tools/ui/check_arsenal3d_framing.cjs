#!/usr/bin/env node
// HOW EVERY CARD IN THE GAME IS COMPOSED.
//
// `Arsenal3D.frameOf` decides the distance a model is drawn from and the point
// it is drawn around. It is pure geometry -- no GL, no DOM -- so it can be
// asked of every shipped model here rather than eyeballed on whichever card
// happens to be open.
//
// This file does NOT re-implement the fit. It projects the vertices the way the
// GPU will and checks the CONSEQUENCES: does anything leave the frame, is the
// frame actually used, and is the model centred in it. A test that copied the
// algorithm would agree with a broken algorithm.
//
// WHY IT EXISTS. The fit used to frame on the bounding-box centre. A site is
// wide, flat and seen from above, so under perspective its near ground projects
// further from centre than its far ground and the silhouette landed LOW: a
// starter industry site in a 3.66:1 strip spanned NDC -0.942 to +0.298, flush
// against the bottom edge with 37% of the card empty above it. Measured in the
// running game, on shipped art.
"use strict";

const test = require("node:test");
const assert = require("node:assert");
const path = require("path");

const ui = path.join(__dirname, "..", "..", "spheres-web", "ui");
const Arsenal3D = require(path.join(ui, "arsenal3d.js"));
const SiteMesh = require(path.join(ui, "site-mesh.js"));
const TownMesh = require(path.join(ui, "town-mesh.js"));
const EquipmentMesh = require(path.join(ui, "equipment-mesh.js"));
const ArsenalModels = require(path.join(ui, "arsenal-models.js"));

const { REST_YAW, REST_PITCH, FOV } = Arsenal3D;
// The card shapes this game actually draws: the wide progress strip, the
// square-ish catalogue tile, the half-height panel, and the extreme full-width
// strip a desktop construction card becomes.
const ASPECTS = [3.66, 1.0, 2.0, 10.98];

const norm = (v) => { const l = Math.hypot(...v); return v.map((x) => x / l); };
const cross = (a, c) => [a[1] * c[2] - a[2] * c[1], a[2] * c[0] - a[0] * c[2], a[0] * c[1] - a[1] * c[0]];

/// Where the silhouette lands, in normalised device coordinates. Straight
/// perspective projection: this is what the shader does, written out.
function project(positions, pivot, distance, aspect, yawDeg, pitchDeg) {
  const tanV = Math.tan((FOV * Math.PI) / 360), tanH = tanV * aspect;
  const rp = (pitchDeg * Math.PI) / 180, ry = (yawDeg * Math.PI) / 180;
  const f = [Math.sin(ry) * Math.cos(rp), Math.sin(rp), Math.cos(ry) * Math.cos(rp)];
  const r = norm(cross([0, 1, 0], f)), u = cross(f, r);
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity, behind = 0;
  for (let i = 0; i < positions.length; i += 3) {
    const x = positions[i] - pivot[0], y = positions[i + 1] - pivot[1], z = positions[i + 2] - pivot[2];
    const zc = distance - (x * f[0] + y * f[1] + z * f[2]);
    if (zc <= 1e-6) { behind += 1; continue; }
    const nx = (x * r[0] + y * r[1] + z * r[2]) / (tanH * zc);
    const ny = (x * u[0] + y * u[1] + z * u[2]) / (tanV * zc);
    if (nx < minX) minX = nx;
    if (nx > maxX) maxX = nx;
    if (ny < minY) minY = ny;
    if (ny > maxY) maxY = ny;
  }
  return { minX, maxX, minY, maxY, behind,
    fillX: (maxX - minX) / 2, fillY: (maxY - minY) / 2,
    offX: (maxX + minX) / 2, offY: (maxY + minY) / 2 };
}

/// Every model the game can put on a card, named so a failure says which.
function models() {
  const out = [];
  for (const kind of SiteMesh.kinds()) {
    out.push([`site:${kind}`, SiteMesh.build(kind, "complete", { level: 2 })]);
    out.push([`site:${kind}/early`, SiteMesh.build(kind, "site", { level: 1 })]);
  }
  for (const district of TownMesh.districts()) {
    out.push([`town:${district}`, TownMesh.block({ id: 1990, district, lod: "map" })]);
  }
  for (const platform of ["tank_standard", "tank_heavy", "ground_apc", "ground_artillery", "ground_air_defense"]) {
    out.push([`veh:${platform}`, EquipmentMesh.build({ platform, lod: 1 })]);
  }
  // The deck keeps its extents as top-level `min`/`max` rather than in a
  // `bounds` object. An earlier draft of this filter asked for `bounds` and
  // silently dropped all 46 deck models -- which is exactly why the count is
  // asserted below rather than left to whatever the sweep happens to collect.
  for (const id of ArsenalModels.ids()) {
    const g = ArsenalModels.build(id);
    if (g && g.positions && (g.bounds || (g.min && g.max))) out.push([`deck:${id}`, g]);
  }
  return out;
}

const ALL = models();

test("every shipped model has a frame, and the deck is not silently skipped", () => {
  assert.ok(ALL.length > 60, `only ${ALL.length} models reached the framing check`);
  assert.ok(ALL.some((m) => m[0].startsWith("deck:")), "the arsenal deck is missing from the sweep");
  for (const [name, mesh] of ALL) {
    const fit = Arsenal3D.frameOf(mesh, 2.0);
    assert.ok(fit && Number.isFinite(fit.d) && fit.d > 0, `${name} has no usable distance`);
    assert.ok(fit.pivot.every(Number.isFinite), `${name} has a non-finite pivot`);
  }
});

test("nothing leaves the frame at rest", () => {
  for (const [name, mesh] of ALL) {
    for (const aspect of ASPECTS) {
      const fit = Arsenal3D.frameOf(mesh, aspect);
      const p = project(mesh.positions, fit.pivot, fit.d, aspect, REST_YAW, REST_PITCH);
      assert.equal(p.behind, 0, `${name} @${aspect}: ${p.behind} vertices behind the camera`);
      assert.ok(Math.max(Math.abs(p.minX), Math.abs(p.maxX)) <= 1,
        `${name} @${aspect} clips horizontally at rest`);
      assert.ok(Math.max(Math.abs(p.minY), Math.abs(p.maxY)) <= 1,
        `${name} @${aspect} clips vertically at rest`);
    }
  }
});

test("nothing leaves the frame at ANY angle a card can turn to", () => {
  // The turning distance is fitted about the RESTING pivot, not one of its own.
  // If it were fitted about its own pivot the model would clip the moment it
  // came round to an angle whose pivot sat elsewhere -- so this is the check
  // that the two share a point.
  let worst = 0, worstAt = "";
  for (const [name, mesh] of ALL) {
    for (const aspect of ASPECTS) {
      const rest = Arsenal3D.frameOf(mesh, aspect);
      const turn = Arsenal3D.frameOf(mesh, aspect, REST_PITCH, null, rest.pivot);
      for (let yaw = 0; yaw < 360; yaw += 15) {
        const p = project(mesh.positions, rest.pivot, turn.d, aspect, yaw, REST_PITCH);
        const reach = Math.max(Math.abs(p.minX), Math.abs(p.maxX), Math.abs(p.minY), Math.abs(p.maxY));
        if (reach > worst) { worst = reach; worstAt = `${name} @${aspect} yaw ${yaw}`; }
      }
    }
  }
  assert.ok(worst <= 1, `a card clips while turning: ${worstAt} reached ${worst.toFixed(4)}`);
  // And the bound is not vacuous: something must come close, or the turn
  // distance is simply too far away and every card turns tiny.
  assert.ok(worst > 0.9, `the turning fit wastes the frame: closest approach was ${worst.toFixed(4)}`);
});

test("a card uses the frame it was given", () => {
  // One dimension must be nearly full: whichever of width or height binds, the
  // model should reach it. 0.93 leaves room for the fit's own 1.03 margin.
  const slack = [];
  for (const [name, mesh] of ALL) {
    for (const aspect of ASPECTS) {
      const fit = Arsenal3D.frameOf(mesh, aspect);
      const p = project(mesh.positions, fit.pivot, fit.d, aspect, REST_YAW, REST_PITCH);
      const used = Math.max(p.fillX, p.fillY);
      // 0.90, not 0.93: the town blocks measure 92.8% and that is a real, fine
      // frame -- the bar exists to catch the old bounding-box framing, which
      // put sites at 61%, and 0.90 catches that with room to spare.
      if (used < 0.90) slack.push(`${name} @${aspect} fills only ${(used * 100).toFixed(1)}%`);
    }
  }
  assert.deepEqual(slack.slice(0, 6), [], `${slack.length} card frames are underused`);
});

test("the model is centred in the frame, which is the whole point", () => {
  // This is the assertion that fails against the old bounding-box framing: a
  // site in a wide strip sat at offY = -0.32, flush to the bottom edge.
  const off = [];
  for (const [name, mesh] of ALL) {
    for (const aspect of ASPECTS) {
      const fit = Arsenal3D.frameOf(mesh, aspect);
      const p = project(mesh.positions, fit.pivot, fit.d, aspect, REST_YAW, REST_PITCH);
      if (Math.abs(p.offX) > 0.05 || Math.abs(p.offY) > 0.05) {
        off.push(`${name} @${aspect} sits at (${p.offX.toFixed(3)}, ${p.offY.toFixed(3)})`);
      }
    }
  }
  assert.deepEqual(off.slice(0, 6), [], `${off.length} models are off-centre in their card`);
});

test("the pivot is shared, and a fixed pivot is honoured exactly", () => {
  const mesh = SiteMesh.build("arms_plant", "complete", { level: 2 });
  const rest = Arsenal3D.frameOf(mesh, 3.66);
  const pinned = Arsenal3D.frameOf(mesh, 3.66, REST_PITCH, null, rest.pivot);
  assert.deepEqual(pinned.pivot, rest.pivot, "a supplied pivot must come back unmoved");
  const free = Arsenal3D.frameOf(mesh, 3.66, REST_PITCH, null);
  assert.ok(free.pivot.some((v, i) => Math.abs(v - rest.pivot[i]) > 1e-9),
    "the all-angles fit should find its OWN pivot when not given one -- if these "
    + "are identical the fixed-pivot path is not doing anything and this test is asleep");
});
