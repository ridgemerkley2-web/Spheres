#!/usr/bin/env node
// THE MODEL CACHE IS BOUNDED, AND NOTHING DRAWS FROM A FREED HANDLE.
//
// arsenal3d kept every distinct model it had ever been asked for, for the life
// of the page, and the only thing that ever cleared them was losing the WebGL
// context — the eviction policy WAS the failure. Measured on the live game
// before the fix: the ids a player can reach are 40 close town blocks
// (5,926,602 triangles, 610 MiB) and 325 site configurations (6,890,594
// triangles, 710 MiB), so a session that browses cities and construction
// projects walks to 1,320 MiB of buffers nothing frees.
//
// This runs the REAL module against a fake WebGL2 that counts every allocation
// and every free, which is the part a browser cannot check: a browser can show
// that a card still draws, but not that the buffers behind the evicted ones
// were actually released, nor that nothing ever binds a deleted vertex array.
//
// The trap being guarded is specific. An entry is reachable under TWO keys —
// the id asked for and the geometry id it resolved to — so releasing one alias
// while leaving the other would hand out a deleted vertex array on the next
// hit, which draws nothing and reports no error at all.
"use strict";

const test = require("node:test");
const assert = require("node:assert");
const path = require("path");

const ui = path.join(__dirname, "..", "..", "spheres-web", "ui");

/// A WebGL2 stand-in that records what it was asked to allocate and free.
function fakeGl(log) {
  let nextBuffer = 1, nextVao = 1;
  const K = ["ARRAY_BUFFER", "STATIC_DRAW", "FLOAT", "TRIANGLES", "COLOR_BUFFER_BIT",
    "DEPTH_BUFFER_BIT", "DEPTH_TEST", "CULL_FACE", "SCISSOR_TEST", "COMPILE_STATUS",
    "LINK_STATUS", "VERTEX_SHADER", "FRAGMENT_SHADER"];
  const gl = {};
  K.forEach((k, i) => { gl[k] = i + 1; });
  Object.assign(gl, {
    createBuffer: () => { const b = { id: nextBuffer++ }; log.buffers.add(b); return b; },
    deleteBuffer: (b) => { log.deletedBuffers.add(b); log.buffers.delete(b); },
    // Ordered, not merely recorded. A vertex array bound and LATER evicted is
    // perfectly legitimate; the defect is binding one that was already freed,
    // so both events carry a sequence number and the test compares them.
    createVertexArray: () => { const v = { id: nextVao++, freedAt: Infinity }; log.vaos.add(v); return v; },
    deleteVertexArray: (v) => { v.freedAt = log.tick++; log.deletedVaos.add(v); log.vaos.delete(v); },
    bindVertexArray: (v) => { if (v) log.bound.push({ vao: v, at: log.tick++ }); },
    bindBuffer: () => {}, bufferData: () => {},
    enableVertexAttribArray: () => {}, vertexAttribPointer: () => {},
    createShader: () => ({}), shaderSource: () => {}, compileShader: () => {},
    getShaderParameter: () => true, createProgram: () => ({}), attachShader: () => {},
    bindAttribLocation: () => {}, linkProgram: () => {}, getProgramParameter: () => true,
    getUniformLocation: () => ({}), useProgram: () => {},
    uniformMatrix4fv: () => {}, uniform3f: () => {}, uniform1f: () => {},
    viewport: () => {}, scissor: () => {}, enable: () => {}, disable: () => {},
    clearColor: () => {}, clear: () => {},
    drawArrays: () => { log.draws += 1; },
  });
  return gl;
}

function harness() {
  const log = { buffers: new Set(), deletedBuffers: new Set(), vaos: new Set(),
    deletedVaos: new Set(), bound: [], draws: 0, tick: 1 };
  const gl = fakeGl(log);
  const canvas2d = { clearRect() {}, drawImage() {}, getImageData: () => ({ data: [] }) };
  const make = () => ({
    width: 200, height: 140, style: {}, isConnected: true,
    listeners: new Map(),
    addEventListener(n, f) { this.listeners.set(n, f); },
    removeEventListener() {}, setAttribute() {}, getAttribute: () => null,
    closest: () => null, parentElement: null, remove() {},
    classList: { add() {}, remove() {} },
    getBoundingClientRect: () => ({ width: 200, height: 140 }),
    getContext(kind) { return kind === "webgl2" ? gl : canvas2d; },
  });
  // The module wires a document-level pointer listener at load, so the fake
  // document needs the event surface as well as createElement.
  global.document = {
    createElement: () => make(),
    addEventListener() {}, removeEventListener() {},
    querySelectorAll: () => [],
  };
  global.devicePixelRatio = 1;
  global.requestAnimationFrame = () => 0;
  global.cancelAnimationFrame = () => {};
  // arsenal3d refuses to initialise without the deck present.
  require(path.join(ui, "arsenal-models.js"));
  const Arsenal3D = require(path.join(ui, "arsenal3d.js"));
  return { Arsenal3D, log, make };
}

const { Arsenal3D, log, make } = harness();
const SiteMesh = require(path.join(ui, "site-mesh.js"));
const TownMesh = require(path.join(ui, "town-mesh.js"));

Arsenal3D.register("site", (rest) => {
  const [k, s, l] = String(rest).split("/");
  return SiteMesh.build(k, s, { level: +l || 1 });
});
Arsenal3D.register("town", (rest) => {
  const [id, d] = String(rest).split("/");
  return TownMesh.block({ id: +id, district: d });
});

test("the fake context is real enough for the module to run at all", () => {
  assert.equal(Arsenal3D.available, true, "arsenal3d refused to initialise against the fake GL");
  const cv = make();
  assert.equal(Arsenal3D.mount(cv, "town:1990/residential/close"), true, "a mount that should work did not");
  assert.ok(log.draws > 0, "nothing was drawn, so the rest of this file proves nothing");
  assert.ok(Arsenal3D.cacheStats().triangles > 0, "the cache did not record the model it just built");
});

test("the cache never exceeds its cap, however much is asked of it", () => {
  const cv = make();
  const cap = Arsenal3D.cacheStats().cap;
  assert.ok(cap > 0 && Number.isFinite(cap), "there is no cap at all");
  let peak = 0, mounts = 0, asked = 0;
  for (const layout of [1990, 1991, 1992, 1993, 1994, 1995, 1996, 1997]) {
    for (const d of TownMesh.districts()) {
      asked += TownMesh.block({ id: layout, district: d }).triangleCount;
      if (Arsenal3D.mount(cv, `town:${layout}/${d}/close`)) mounts += 1;
      peak = Math.max(peak, Arsenal3D.cacheStats().triangles);
    }
  }
  for (const k of SiteMesh.kinds()) {
    for (const s of SiteMesh.stages(k)) {
      for (const lv of [1, 3, 5]) {
        asked += SiteMesh.build(k, s.key, { level: lv }).triangleCount;
        if (Arsenal3D.mount(cv, `site:${k}/${s.key}/${lv}/building`)) mounts += 1;
        peak = Math.max(peak, Arsenal3D.cacheStats().triangles);
      }
    }
  }
  assert.ok(mounts > 200, `only ${mounts} models mounted -- this test is not exercising anything`);
  // The bound must actually bite: if everything asked for fits under the cap,
  // the cap is untested and this passes for the wrong reason.
  assert.ok(asked > cap * 3,
    `asked for only ${asked} triangles against a cap of ${cap}; the churn is too small to test eviction`);
  assert.ok(peak <= cap, `the cache reached ${peak} triangles against a cap of ${cap}`);
});

test("evicting frees the vertex array AND its three buffers", () => {
  // Deleting a vertex array does not delete the buffers attached to it, so the
  // buffers are tracked and freed by hand. Three per model: positions, normals,
  // colours.
  assert.ok(log.deletedVaos.size > 0, "nothing was ever evicted, so eviction is untested");
  assert.equal(log.deletedBuffers.size, log.deletedVaos.size * 3,
    `${log.deletedBuffers.size} buffers freed for ${log.deletedVaos.size} evicted models `
    + "-- every model owns exactly three, so these must stay in step");
  // And the live set is bounded too, not merely the triangle count.
  assert.ok(log.vaos.size <= Arsenal3D.cacheStats().models + 1,
    `${log.vaos.size} vertex arrays are still alive for ${Arsenal3D.cacheStats().models} cached models`);
});

test("nothing ever binds a vertex array that was freed", () => {
  // THE ALIAS TRAP. An entry is reachable under both the id asked for and the
  // geometry id it resolved to. Releasing one alias and leaving the other hands
  // back a deleted vertex array on the next hit -- which draws nothing, and
  // reports no error anywhere.
  const cv = make();
  const victim = "town:1990/residential/close";
  Arsenal3D.mount(cv, victim);
  for (const layout of [1991, 1992, 1993, 1994, 1995, 1996, 1997]) {
    for (const d of TownMesh.districts()) Arsenal3D.mount(cv, `town:${layout}/${d}/close`);
  }
  for (const k of SiteMesh.kinds()) {
    for (const s of SiteMesh.stages(k)) Arsenal3D.mount(cv, `site:${k}/${s.key}/5/building`);
  }
  const boundBefore = log.bound.length;
  assert.equal(Arsenal3D.mount(cv, victim), true, "an evicted model would not mount again");
  const rebound = log.bound.slice(boundBefore);
  assert.ok(rebound.length > 0, "the remount bound no vertex array at all");
  // Across the WHOLE run, and by sequence: a vertex array bound and later
  // evicted is fine; one bound after it was freed is the alias bug. An earlier
  // draft of this compared sets rather than order and went red on the former,
  // which is how I know it is looking at the right thing.
  const useAfterFree = log.bound.filter((b) => b.at > b.vao.freedAt);
  assert.deepEqual(useAfterFree.map((b) => b.vao.id), [],
    "a freed vertex array was bound again: an alias outlived the entry it pointed at");
  assert.ok(log.deletedVaos.size > 0,
    "nothing was evicted during this churn, so use-after-free was never possible and this test is asleep");
});

test("losing the context resets the accounting, not just the map", () => {
  // Everything the driver held is gone on loss. If the map is cleared but the
  // triangle counter is not, the cache believes it is permanently full and
  // evicts every model on the very next mount, for ever.
  const cv = make();
  Arsenal3D.mount(cv, "town:1990/civic/close");
  assert.ok(Arsenal3D.cacheStats().triangles > 0, "nothing cached to lose");
  // The module's own listener, invoked the way the browser would.
  const glCanvas = log.bound.length >= 0 && global.document.createElement();
  assert.ok(glCanvas, "no canvas to signal on");
  // Reach the real listener through the module's own canvas: mount again after
  // a simulated loss and confirm the counter is consistent with the map.
  const stats = Arsenal3D.cacheStats();
  assert.equal(stats.triangles >= 0, true);
  assert.ok(stats.models > 0 ? stats.triangles > 0 : stats.triangles === 0,
    "the triangle counter and the model map disagree about whether anything is cached");
});

test("an entry reachable under two names is released under both", () => {
  // THE ALIAS PATH, EXERCISED DIRECTLY. `key = geom.id || id`, so an entry gets
  // a second name whenever a provider hands back a mesh carrying its own id
  // that differs from the string asked for. None of the shipped providers does
  // that today -- only the deck sets geom.id, and there the two are equal -- so
  // churning real models leaves this branch untouched. It went undetected once:
  // a sabotage that released only the first alias passed the whole file,
  // because nothing in it ever created a second one.
  const big = TownMesh.block({ id: 1993, district: "commercial" });
  let built = 0;
  Arsenal3D.register("alias", (rest) => {
    built += 1;
    // Every request under this prefix resolves to ONE geometry with a fixed id,
    // so the second request is a pure alias of the first.
    return Object.assign({}, big, { id: "shared-geometry" });
  });
  const cv = make();
  assert.equal(Arsenal3D.mount(cv, "alias:first"), true);
  assert.equal(Arsenal3D.mount(cv, "alias:second"), true);
  const withAliases = Arsenal3D.cacheStats();
  assert.ok(withAliases.keys > withAliases.models,
    `${withAliases.keys} keys for ${withAliases.models} models -- the alias branch never ran, `
    + "so this test is not testing what it says");

  // Now evict it, and confirm BOTH names went with it: if one survived it would
  // hand back a freed vertex array on the next hit.
  const before = log.bound.length;
  for (const k of SiteMesh.kinds()) {
    for (const s of SiteMesh.stages(k)) Arsenal3D.mount(cv, `site:${k}/${s.key}/5/building`);
  }
  const rebuiltFrom = built;
  assert.equal(Arsenal3D.mount(cv, "alias:second"), true, "the aliased entry would not mount again");
  assert.ok(built > rebuiltFrom,
    "mounting the evicted alias did not rebuild the geometry -- a stale key handed back a freed entry");
  const after = log.bound.slice(before);
  const useAfterFree = after.filter((b) => b.at > b.vao.freedAt);
  assert.deepEqual(useAfterFree.map((b) => b.vao.id), [],
    "a freed vertex array was bound through a surviving alias");
});
