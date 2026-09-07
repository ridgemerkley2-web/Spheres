/* Original Spheres game art. Export the designer's triangle mesh as glTF 2.0 binary. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.EquipmentExport = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  function glb(mesh, name = "Spheres tank") {
    if (!mesh || typeof mesh !== "object") throw new TypeError("A tank triangle mesh is required.");
    const attributes = [mesh.positions, mesh.normals, mesh.colors];
    const size = mesh.positions && mesh.positions.length;
    if (!Number.isSafeInteger(size) || size === 0 || size % 9 !== 0)
      throw new TypeError("Positions must contain complete, nonempty triangles.");
    if (attributes.some(values => !ArrayBuffer.isView(values) || Object.prototype.toString.call(values) !== "[object Float32Array]" || values.length !== size))
      throw new TypeError("Positions, normals, and colors must be matching Float32 arrays.");
    // A uint32 GLB length and accessor count put an upper bound on any export.
    if (size > 100000000) throw new RangeError("This mesh is too large to export.");
    const count = size / 3;
    if (mesh.triangleCount !== undefined && mesh.triangleCount !== count / 3)
      throw new TypeError("The triangle count does not match the geometry.");
    const minimum = [Infinity, Infinity, Infinity], maximum = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < size; i++) {
      if (attributes.some(values => !Number.isFinite(values[i])))
        throw new TypeError("Model attributes must contain only finite values.");
      if (mesh.colors[i] < 0 || mesh.colors[i] > 1)
        throw new RangeError("Vertex colors must be between zero and one.");
      minimum[i % 3] = Math.min(minimum[i % 3], mesh.positions[i]);
      maximum[i % 3] = Math.max(maximum[i % 3], mesh.positions[i]);
    }
    const parts = (mesh.parts || []).map(part => {
      if (!part || typeof part.name !== "string" || !Number.isSafeInteger(part.first) ||
          !Number.isSafeInteger(part.count) || part.first < 0 || part.count < 0 ||
          part.first % 3 !== 0 || part.count % 3 !== 0 || part.first + part.count > count)
        throw new TypeError("Model part ranges must refer to complete triangles.");
      return {name: part.name, first: part.first, count: part.count,
        ...(typeof part.slot==='string'?{slot:part.slot}:{}),...(typeof part.label==='string'?{label:part.label}:{})};
    });
    const byteLength = size * 4;
    const binary = new Uint8Array(byteLength * 3);
    const data = new DataView(binary.buffer);
    for (let i = 0; i < size; i++) {
      data.setFloat32(i * 4, mesh.positions[i], true);
      data.setFloat32(byteLength * 2 + i * 4, mesh.colors[i], true);
    }
    for (let i = 0; i < size; i += 3) {
      const magnitude = Math.hypot(mesh.normals[i], mesh.normals[i + 1], mesh.normals[i + 2]);
      if (!(magnitude > 0)) throw new TypeError("Model normals must have a nonzero length.");
      for (let axis = 0; axis < 3; axis++)
        data.setFloat32(byteLength + (i + axis) * 4, mesh.normals[i + axis] / magnitude, true);
    }
    const label = String(name || "Spheres tank").slice(0, 128);
    const description = typeof mesh.description === "string" ? mesh.description : "Original configurable Spheres tank game model.";
    const document = {
      asset: {version: "2.0", generator: "Spheres Equipment Designer", copyright: "Original Spheres procedural game art"},
      scene: 0,
      scenes: [{name: label, nodes: [0]}],
      nodes: [{name: label, mesh: 0}],
      meshes: [{name: label, primitives: [{attributes: {POSITION: 0, NORMAL: 1, COLOR_0: 2}, material: 0, mode: 4}],
        extras: {description, parts, triangleCount: count / 3, gameArt: true, ...(mesh.specification?{specification:mesh.specification}:{})}}],
      materials: [{name: "Painted vehicle", pbrMetallicRoughness: {
        baseColorFactor: [1, 1, 1, 1], metallicFactor: 0, roughnessFactor: 0.82
      }}],
      buffers: [{byteLength: binary.byteLength}],
      bufferViews: attributes.map((_, index) => ({buffer: 0, byteOffset: index * byteLength, byteLength, target: 34962})),
      accessors: [
        {bufferView: 0, componentType: 5126, count, type: "VEC3", min: minimum, max: maximum},
        {bufferView: 1, componentType: 5126, count, type: "VEC3"},
        {bufferView: 2, componentType: 5126, count, type: "VEC3"}
      ]
    };
    const json = new TextEncoder().encode(JSON.stringify(document));
    const jsonLength = (json.byteLength + 3) & ~3;
    const binaryLength = (binary.byteLength + 3) & ~3;
    const output = new Uint8Array(12 + 8 + jsonLength + 8 + binaryLength);
    const header = new DataView(output.buffer);
    header.setUint32(0, 0x46546c67, true); // glTF
    header.setUint32(4, 2, true);
    header.setUint32(8, output.byteLength, true);
    header.setUint32(12, jsonLength, true);
    header.setUint32(16, 0x4e4f534a, true); // JSON
    output.fill(0x20, 20, 20 + jsonLength);
    output.set(json, 20);
    header.setUint32(20 + jsonLength, binaryLength, true);
    header.setUint32(24 + jsonLength, 0x004e4942, true); // BIN
    output.set(binary, 28 + jsonLength);
    return output;
  }

  return {glb};
});
