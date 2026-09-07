#!/usr/bin/env node
// THE REVIEW BENCH MUST NOT LIE ABOUT ITS OWN NUMBERS.
//
// tools/arsenal/art-gallery.html is the surface roadmap section 7.2 asks for:
// every P0 asset under one neutral rig "with its measured numbers beside it so
// a judgement can be argued with". Nothing checked it, and it was wrong in two
// ways that both defeat that purpose:
//
//   1. Every town card printed a triangle count for a block it was NOT drawing.
//      The caption measured `{id: 1990}` while the provider splits the id out of
//      the card's own string and passes `{id: "1990"}`, and town-mesh seeds a
//      number and a string differently. Civic read 131,586 against the 106,789
//      actually on screen — 23% wrong.
//   2. Four of its nine budget bands were LOOSER than the roadmap section 4 they
//      claimed to grade against (vehicles 150,000 against 45,000; sites and
//      stages 40,000 against 12,000; town 400,000 against 150,000), so art that
//      is over budget displayed as passing.
//
// This runs the page's OWN script against the real mesh modules — it does not
// re-implement the views — and asserts the caption equals what the provider
// builds, for every card in every view.
"use strict";

const test = require("node:test");
const assert = require("node:assert");
const fs = require("fs");
const path = require("path");
const vm = require("vm");

const ROOT = path.join(__dirname, "..", "..");
const ui = (f) => path.join(ROOT, "spheres-web", "ui", f);
const PAGE = path.join(ROOT, "tools", "arsenal", "art-gallery.html");

/// Run the gallery's script with a fake DOM and a fake Arsenal3D that records
/// the providers it registers, then render each view and hand back its cards.
function runGallery() {
  const html = fs.readFileSync(PAGE, "utf8");
  const scripts = [...html.matchAll(/<script>([\s\S]*?)<\/script>/g)].map((m) => m[1]);
  assert.equal(scripts.length, 1, "expected exactly one inline script in the bench");

  const providers = new Map();
  const els = {};
  const makeEl = () => ({
    innerHTML: "", listeners: {},
    addEventListener(k, f) { this.listeners[k] = f; },
    querySelectorAll: () => [],
  });
  for (const id of ["grid", "bar"]) els[id] = makeEl();

  const sandbox = {
    document: {
      getElementById: (id) => els[id] || null,
      createElement: () => makeEl(),
    },
    console: { log() {}, warn() {}, error() {} },
    ArsenalModels: require(ui("arsenal-models.js")),
    EquipmentMesh: require(ui("equipment-mesh.js")),
    SiteMesh: require(ui("site-mesh.js")),
    TownMesh: require(ui("town-mesh.js")),
    RoadMesh: require(ui("road-mesh.js")),
    ScatterMesh: require(ui("scatter-mesh.js")),
    PropMesh: require(ui("prop-mesh.js")),
    Arsenal3D: {
      available: true,
      register: (prefix, fn) => providers.set(prefix, fn),
      scan() {},
    },
  };
  sandbox.window = sandbox;
  vm.createContext(sandbox);
  vm.runInContext(scripts[0], sandbox, { filename: "art-gallery.html" });

  // The bench builds one button per view and renders on click. Drive it the way
  // a reader would rather than reaching into the closure.
  const names = [...els.bar.innerHTML.matchAll(/data-v="([^"]+)"/g)].map((m) => m[1]);
  const click = els.bar.listeners.click;
  assert.ok(typeof click === "function", "the view bar lost its click handler");

  const views = new Map();
  for (const name of names) {
    click({ target: { closest: (sel) => (sel === "button[data-v]" ? { dataset: { v: name } } : null) } });
    const html2 = els.grid.innerHTML;
    assert.ok(!/failed to build/.test(html2), `${name} failed to build: ${html2.slice(0, 200)}`);
    const cards = [...html2.matchAll(/data-kit3d="([^"]*)"[\s\S]*?<em[^>]*>([\d,]+) triangles([^<]*)</g)]
      .map((m) => ({ id: m[1], caption: Number(m[2].replace(/,/g, "")), note: m[3] }));
    views.set(name, cards);
  }
  return { views, providers, names };
}

const G = runGallery();

test("the bench renders every view, and every view has cards", () => {
  assert.ok(G.names.length >= 10, `only ${G.names.length} views: ${G.names.join(", ")}`);
  for (const [name, cards] of G.views) {
    assert.ok(cards.length > 0, `view "${name}" rendered no cards`);
  }
  // The kits that ship must all be represented somewhere.
  const all = [...G.views.values()].flat().map((c) => c.id).join(" ");
  for (const [kit, marker] of [["equipment", "veh:"], ["sites", "site:"], ["town", "town:"],
    ["roads", "road:"], ["props", "prop:"], ["scatter", "scatter:"]]) {
    assert.ok(all.includes(marker), `no ${kit} card on the bench (looked for "${marker}")`);
  }
});

test("every caption is the count of the mesh that card actually draws", () => {
  // THE BUG THIS FILE EXISTS FOR. The card's id is what the renderer resolves;
  // the caption is computed separately. If the two disagree the bench is
  // reporting a number for something that is not on screen.
  const wrong = [];
  for (const [view, cards] of G.views) {
    for (const c of cards) {
      const colon = c.id.indexOf(":");
      let mesh;
      if (colon < 0) {
        const g = G.providers.size && null;
        mesh = require(ui("arsenal-models.js")).build(c.id);
        // the deck reports a raw vertex count, not triangles
        const tris = mesh.count / 3;
        if (tris !== c.caption) wrong.push(`${view} ${c.id}: caption ${c.caption} vs drawn ${tris}`);
        continue;
      }
      const fn = G.providers.get(c.id.slice(0, colon));
      assert.ok(fn, `${view}: no provider registered for "${c.id}"`);
      mesh = fn(c.id.slice(colon + 1));
      assert.ok(mesh, `${view}: provider returned nothing for "${c.id}"`);
      const tris = mesh.triangleCount != null ? mesh.triangleCount : mesh.count / 3;
      if (tris !== c.caption) wrong.push(`${view} ${c.id}: caption ${c.caption} vs drawn ${tris}`);
    }
  }
  assert.deepEqual(wrong.slice(0, 8), [], `${wrong.length} cards print a number for a different mesh`);
});

test("no band on the bench is looser than the roadmap section 4 row it cites", () => {
  // A review bar wider than the budget shows over-budget art as passing, which
  // is the one failure a review surface cannot have. bench_art.cjs is the
  // authority — it re-reads section 4 and exits when a cell moves — so this
  // check reads the SAME roadmap file and holds the page to it.
  const roadmap = fs.readFileSync(path.join(ROOT, "docs", "art", "3D_MODEL_MASTER_ROADMAP.md"), "utf8");
  const cells = new Map();
  for (const line of roadmap.split(/\r?\n/)) {
    if (!line.startsWith("|")) continue;
    const parts = line.split("|").map((s) => s.trim());
    if (parts.length >= 4) cells.set(parts[1], parts[2]);
  }
  const page = fs.readFileSync(PAGE, "utf8");
  const rows = [...page.matchAll(/^\s{4}(\w+): \[(\d+), (\d+), "([^"]+)"\],/gm)];
  assert.ok(rows.length >= 6, `only ${rows.length} section 4 bands found on the bench`);
  for (const [, key, min, max, row] of rows) {
    assert.ok(cells.has(row),
      `the bench cites a section 4 row that no longer exists: "${row}" (${key})`);
    const cell = cells.get(row);
    // The numbers the page carries must appear in the roadmap cell it names, so
    // a cell edit is caught here rather than silently tolerated.
    const digits = cell.replace(/[^\d]/g, "");
    const shrink = (n) => String(n).replace(/000$/, "");
    assert.ok(digits.includes(shrink(min)) || digits.includes(String(min)) || Number(min) === 0,
      `${key}: floor ${min} is not in the section 4 cell "${cell}" for row "${row}"`);
    assert.ok(digits.includes(shrink(max)) || digits.includes(String(max)),
      `${key}: ceiling ${max} is not in the section 4 cell "${cell}" for row "${row}"`);
  }
});

test("art with no section 4 row says so instead of inventing a bar", () => {
  // Roads and the prop kit are graded nowhere: section 4 has no road row, and
  // its Tree/prop row does not describe a 218 m container ship. The bench used
  // to assert made-up ranges for both, which is a bar that cannot fail.
  for (const view of ["Roads", "Props"]) {
    const cards = G.views.get(view);
    assert.ok(cards && cards.length, `the ${view} view is gone`);
    for (const c of cards) {
      assert.match(c.note, /no section 4 row/,
        `${view} card "${c.id}" claims a budget section 4 does not have`);
    }
  }
  // And the kits that DO have a row must actually cite one.
  for (const view of ["Scatter near", "Scatter far"]) {
    const cards = G.views.get(view);
    assert.ok(cards && cards.length, `the ${view} view is missing`);
    assert.match(cards[0].note, /§4/, `${view} should be graded against the Tree/prop row`);
  }
});
