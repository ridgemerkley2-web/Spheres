#!/usr/bin/env node
// Dump every equipment model to Wavefront OBJ.
//
// The models are built in JavaScript because the browser has no build step to
// load them any other way (see the header of arsenal-models.js). That is a good
// reason to author them there and a bad reason for them to be TRAPPED there, so
// this exists: the same file, run under node, writing one .obj per kit that
// Blender, MeshLab, three.js or an engine importer will open.
//
// The output is not checked in. It is derived — twenty thousand triangles of it
// — and regenerating takes under a second:
//
//     node tools/arsenal/export_obj.js                  -> tools/arsenal/obj/
//     node tools/arsenal/export_obj.js path/to/dir      -> somewhere else
//     node tools/arsenal/export_obj.js --id f22         -> one kit, to stdout
//
// Colour rides on the vertex lines as `v x y z r g b`, which is the extension
// every one of those tools reads. Units are metres, +Z forward, +Y up.
"use strict";

const fs = require("fs");
const path = require("path");
const models = require("../../spheres-web/ui/arsenal-models.js");

const args = process.argv.slice(2);
const idFlag = args.indexOf("--id");
if (idFlag !== -1) {
  const id = args[idFlag + 1];
  const obj = models.toOBJ(id);
  if (!obj) {
    console.error(`no model for "${id}". Known ids:\n  ${models.ids().join("\n  ")}`);
    process.exit(1);
  }
  process.stdout.write(obj);
  return;
}

const outDir = args.find((a) => !a.startsWith("--")) || path.join(__dirname, "obj");
fs.mkdirSync(outDir, { recursive: true });

let triangles = 0;
const rows = models.ids().map((id) => {
  const geom = models.build(id);
  fs.writeFileSync(path.join(outDir, `${id}.obj`), models.toOBJ(id), "utf8");
  triangles += geom.count / 3;
  return {
    id,
    name: geom.name,
    class: geom.cls,
    triangles: geom.count / 3,
    metres: geom.size.map((v) => Math.round(v * 10) / 10),
  };
});
// An index beside the meshes, so whatever reads them does not have to parse
// forty-six files to find out what they are.
fs.writeFileSync(path.join(outDir, "index.json"), `${JSON.stringify(rows, null, 2)}\n`, "utf8");

console.log(`${rows.length} models, ${triangles} triangles -> ${outDir}`);
for (const r of rows) {
  console.log(`  ${r.id.padEnd(14)} ${String(r.triangles).padStart(5)} tris  `
    + `${r.metres.join(" x ").padEnd(22)} ${r.name}`);
}
