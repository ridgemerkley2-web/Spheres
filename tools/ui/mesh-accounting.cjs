// Offline payload accounting. These are mesh bytes, not driver memory or draws.
'use strict';
const assert = require('node:assert/strict');
function measureMesh(mesh, label = 'mesh') {
  const check = (condition, message) => assert(condition, `${label}: ${message}`);
  check(Number.isSafeInteger(mesh?.triangleCount) && mesh.triangleCount > 0, 'invalid triangle count');
  const vertices = mesh.triangleCount * 3;
  const layout = {positions: [Float32Array, 3], normals: [Float32Array, 3], colors: [Float32Array, 3]};
  if (mesh.materialClasses !== undefined) layout.materialClasses = [Uint8Array, 1];
  const views = Object.keys(mesh).filter(key => ArrayBuffer.isView(mesh[key]));
  check(views.every(key => key in layout), `unreviewed vertex layout: ${views.join(', ')}`);
  const allocations = new Set(), attributes = {};
  for (const [key, [Type, width]] of Object.entries(layout)) {
    const data = mesh[key];
    check(data instanceof Type && data.length === vertices * width, `invalid ${key} type or length`);
    allocations.add(data.buffer);
    attributes[key] = {type: Type.name, components: width, bytes: data.byteLength,
      usage: key === 'materialClasses' ? 'CPU material classification; not uploaded directly' : 'base vertex attribute'};
  }
  return {triangles: mesh.triangleCount, vertices, attributes,
    cpu_attribute_view_bytes: Object.values(attributes).reduce((n, a) => n + a.bytes, 0),
    cpu_backing_buffer_bytes: [...allocations].reduce((n, b) => n + b.byteLength, 0),
    base_attribute_upload_bytes: ['positions', 'normals', 'colors'].reduce((n, key) => n + mesh[key].byteLength, 0)};
}
function sumPayloads(rows) {
  return rows.reduce((sum, row) => ({n: sum.n + 1, tris: sum.tris + row.tris,
    bytes: sum.bytes + row.bytes, cpuBytes: sum.cpuBytes + row.accounting.cpu_backing_buffer_bytes}),
    {n: 0, tris: 0, bytes: 0, cpuBytes: 0});
}
module.exports = {measureMesh, sumPayloads};
