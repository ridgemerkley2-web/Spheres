/* Original Spheres game art, read back in. equipment-export.js writes the
   designer's mesh as glTF 2.0 binary; this is that writer inverted, so geometry
   that left the pipeline can return as the exact object the viewer already
   draws. Metres, +Z forward, +Y up, Y=0 at ground contact.

   It reads THIS pipeline's dialect: one non-indexed vertex-coloured triangle
   list at the origin, three tightly packed VEC3/FLOAT accessors, no textures and
   no extensions. Our marked aircraft dialect may draw bounded opaque/glass
   slices of those SAME arrays; the slices never replace the original vertices.
   Everything else is refused by name. A half-import would put
   geometry on screen that the part ranges, the picker and the re-export no
   longer describe, and a wrong model that renders is worse than a refusal. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.EquipmentImport = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  const MAGIC = 0x46546c67, JSON_CHUNK = 0x4e4f534a, BIN_CHUNK = 0x004e4942, FLOAT = 5126;
  const SEMANTICS = ["POSITION", "NORMAL", "COLOR_0"];
  const MODES = ["points", "lines", "a line loop", "a line strip", "triangles", "a triangle strip", "a triangle fan"];
  const COMPONENTS = {5120: "BYTE", 5121: "UNSIGNED_BYTE", 5122: "SHORT", 5123: "UNSIGNED_SHORT", 5125: "UNSIGNED_INT", 5126: "FLOAT"};
  // Every refusal names what was found and why this runtime cannot draw it.
  const refuse = detail => { throw new Error(`Cannot import this GLB: ${detail}`); };
  const isObject = value => value !== null && typeof value === "object" && !Array.isArray(value);
  const list = value => (Array.isArray(value) ? value : []);
  const hex = value => `0x${(value >>> 0).toString(16).padStart(8, "0")}`;
  const named = thing => (typeof thing.name === "string" && thing.name ? `"${thing.name}"` : "(unnamed)");

  function asBytes(source) {
    const tag = Object.prototype.toString.call(source);
    if (tag === "[object Uint8Array]") return source;
    if (tag === "[object ArrayBuffer]") return new Uint8Array(source);
    if (ArrayBuffer.isView(source)) return new Uint8Array(source.buffer, source.byteOffset, source.byteLength);
    throw new TypeError("GLB bytes are required, as an ArrayBuffer or a Uint8Array.");
  }

  // Walk the chunk table rather than assume JSON-then-BIN at fixed offsets: our
  // own writer is fixed, but the format permits other chunks between the two.
  function container(bytes) {
    if (bytes.byteLength < 20)
      refuse(`the file is ${bytes.byteLength} bytes, shorter than a 12-byte GLB header plus one chunk header.`);
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const magic = view.getUint32(0, true);
    if (magic !== MAGIC)
      refuse(`it does not begin with the glTF magic 0x46546c67 (found ${hex(magic)}); a .gltf JSON file or an archive is not a GLB.`);
    const version = view.getUint32(4, true);
    if (version !== 2) refuse(`it declares GLB container version ${version}; only version 2 exists, and only version 2 is read here.`);
    const total = view.getUint32(8, true);
    if (total !== bytes.byteLength)
      refuse(`its header declares ${total} bytes but ${bytes.byteLength} were supplied; the file is truncated or padded.`);
    let offset = 12, json = null, binary = null;
    while (offset + 8 <= total) {
      const length = view.getUint32(offset, true), type = view.getUint32(offset + 4, true);
      if (length % 4 !== 0)
        refuse(`the chunk at byte ${offset} is ${length} bytes long, breaking the 4-byte padding every chunk must carry.`);
      if (offset + 8 + length > total)
        refuse(`the chunk at byte ${offset} claims ${length} bytes and runs past the end of the file.`);
      if (offset === 12 && type !== JSON_CHUNK)
        refuse(`the first chunk is type ${hex(type)}, not the JSON chunk 0x4e4f534a the format requires first.`);
      if (type === JSON_CHUNK) {
        if (json) refuse("it carries two JSON chunks; exactly one is allowed.");
        json = bytes.subarray(offset + 8, offset + 8 + length);
      } else if (type === BIN_CHUNK) {
        if (binary) refuse("it carries two BIN chunks; at most one is allowed.");
        binary = {start: offset + 8, length};
      }
      // Unknown chunk types are skipped, as the format instructs, not refused.
      offset += 8 + length;
    }
    if (offset !== total) refuse(`${total - offset} bytes trail the last complete chunk.`);
    if (!json) refuse("it has no JSON chunk.");
    if (!binary) refuse("it has no BIN chunk; only self-contained models with embedded geometry are read.");
    let document = null;
    try { document = JSON.parse(new TextDecoder().decode(json)); }
    catch (error) { refuse(`its JSON chunk does not parse (${error.message}).`); }
    if (!isObject(document)) refuse("its JSON chunk is not a glTF document object.");
    return {document, view, binary};
  }

  function fromGlb(source) {
    const bytes = asBytes(source);
    const {document, view, binary} = container(bytes);

    if (!isObject(document.asset) || document.asset.version !== "2.0")
      refuse(`it states glTF version ${JSON.stringify(isObject(document.asset) ? document.asset.version : undefined)}; this reader implements glTF 2.0.`);

    const required = list(document.extensionsRequired), used = list(document.extensionsUsed);
    if (required.length)
      refuse(`it requires the extension${required.length > 1 ? "s" : ""} ${required.join(", ")}; none are implemented here, and a model needing Draco or a KHR material would arrive wrong rather than not at all.`);
    if (used.length)
      refuse(`it uses the extension${used.length > 1 ? "s" : ""} ${used.join(", ")}; the viewer draws plain vertex-coloured triangles and would drop whatever they carry.`);
    if (isObject(document.extensions) && Object.keys(document.extensions).length)
      refuse(`its root carries the extension block${Object.keys(document.extensions).length > 1 ? "s" : ""} ${Object.keys(document.extensions).join(", ")}, which nothing here reads.`);

    for (const [field, what, why] of [
      ["images", "images", "there is no texture path at all: surfaces are shaded from per-vertex colour"],
      ["textures", "textures", "there is no texture path at all: surfaces are shaded from per-vertex colour"],
      ["samplers", "texture samplers", "no texture is ever sampled: surfaces are shaded from per-vertex colour"],
      ["animations", "animations", "the equipment viewer draws a static mesh and can play no channel"],
      ["skins", "skins", "the mesh is rigid; nothing here skins vertices to joints"]
    ]) {
      const count = list(document[field]).length;
      if (count) refuse(`its ${field} list holds ${count} ${count > 1 ? what : what.replace(/s$/, "")}, and ${why}.`);
    }

    const materials = list(document.materials);
    const meshes = list(document.meshes);
    if (meshes.length !== 1)
      refuse(`it defines ${meshes.length} meshes; the viewer holds one mesh with one set of part ranges.`);
    const mesh = meshes[0], primitives = list(mesh.primitives);
    const extras = isObject(mesh.extras) ? mesh.extras : {}, aircraft = extras.assetKind === "aircraft";
    if (extras.surfaces !== undefined && !aircraft)
      refuse("its surface ranges have no aircraft dialect marker; arbitrary multi-material models are not supported.");
    if (!aircraft && materials.length > 1)
      refuse(`it defines ${materials.length} materials; only the marked aircraft dialect supports separate surface shading.`);
    if (!aircraft && primitives.length !== 1)
      refuse(`its mesh ${named(mesh)} holds ${primitives.length} primitives; the part ranges index one continuous vertex list, and concatenating primitives would move every range.`);
    if (aircraft && (!primitives.length || document.asset.generator !== "Spheres Equipment Designer" || extras.gameArt !== true))
      refuse("its aircraft dialect requires a Spheres Equipment Designer game-art mesh with draw primitives.");

    function primitiveAttributes(primitive) {
    if (!isObject(primitive)) refuse("its primitive is not an object.");
    if (primitive.indices !== undefined)
      refuse(`its primitive is indexed (indices accessor ${primitive.indices}); nothing here uploads an index buffer, and the part ranges address vertices directly, so an indexed mesh cannot keep its parts.`);
    const mode = primitive.mode === undefined ? 4 : primitive.mode;
    if (mode !== 4)
      refuse(`its primitive is drawn as ${MODES[mode] || `mode ${mode}`}; only mode 4, independent triangles, matches the runtime.`);
    if (isObject(primitive.extensions) && Object.keys(primitive.extensions).length)
      refuse(`its primitive carries the extension${Object.keys(primitive.extensions).length > 1 ? "s" : ""} ${Object.keys(primitive.extensions).join(", ")}, which nothing here decodes.`);
    if (primitive.targets !== undefined)
      refuse("its primitive carries morph targets; the viewer draws only the original rigid triangles.");

    const attributes = isObject(primitive.attributes) ? primitive.attributes : {};
    const foreign = Object.keys(attributes).filter(name => !SEMANTICS.includes(name));
    if (foreign.length)
      refuse(`its primitive carries ${foreign.join(", ")}; only POSITION, NORMAL and COLOR_0 are bound, so importing it would drop that data without saying so.`);
    for (const name of SEMANTICS) {
      if (attributes[name] === undefined) refuse(`its primitive has no ${name} attribute; all three are required to draw it and to export it again.`);
      if (!Number.isSafeInteger(attributes[name])) refuse(`its ${name} attribute is ${JSON.stringify(attributes[name])} rather than an accessor index.`);
    }
    return attributes;
    }
    const drawAttributes = primitives.map(primitiveAttributes);
    const attributes = aircraft ? {POSITION: 0, NORMAL: 1, COLOR_0: 2} : drawAttributes[0];

    const nodes = list(document.nodes);
    if (nodes.length !== 1)
      refuse(`it describes ${nodes.length} nodes; the viewer draws a single mesh at the origin and applies no hierarchy.`);
    const node = nodes[0];
    if (list(node.children).length)
      refuse(`node ${named(node)} declares ${list(node.children).length} child node(s); one root node is drawn, so a child's geometry and its placement would both be lost.`);
    if (node.mesh !== 0)
      refuse(`node ${named(node)} carries mesh ${JSON.stringify(node.mesh)} rather than mesh 0; the mesh in this file is not the one the scene draws.`);
    // Refused rather than baked: baking would hand back vertices the generator
    // never produced, and would move the Y=0 ground datum and the accessor
    // bounds with them. A transform belongs in the generator, before export.
    for (const [field, identity] of [
      ["matrix", [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]],
      ["translation", [0, 0, 0]], ["rotation", [0, 0, 0, 1]], ["scale", [1, 1, 1]]
    ]) {
      if (node[field] === undefined) continue;
      const values = list(node[field]);
      if (values.length !== identity.length || values.some((value, i) => value !== identity[i]))
        refuse(`node ${named(node)} carries a non-identity ${field} [${values.join(", ")}]; transforms are not baked here, and applying one would move the model off its ground contact.`);
    }
    const scenes = list(document.scenes);
    if (scenes.length) {
      const scene = scenes[document.scene === undefined ? 0 : document.scene];
      if (!isObject(scene) || list(scene.nodes).length !== 1 || scene.nodes[0] !== 0)
        refuse("its default scene does not draw exactly node 0; the viewer draws one root node and would ignore whatever else the scene lists.");
    }

    const buffers = list(document.buffers);
    if (buffers.length !== 1)
      refuse(`it declares ${buffers.length} buffers; a self-contained GLB has exactly one, the embedded BIN chunk.`);
    if (buffers[0].uri !== undefined)
      refuse(`its buffer points at the external resource ${JSON.stringify(buffers[0].uri)}; only geometry embedded in the BIN chunk is read.`);
    if (Number.isSafeInteger(buffers[0].byteLength) && buffers[0].byteLength > binary.length)
      refuse(`its buffer declares ${buffers[0].byteLength} bytes but the BIN chunk holds ${binary.length}.`);

    function plan(name, index = attributes[name]) {
      const accessor = list(document.accessors)[index];
      if (!isObject(accessor)) refuse(`${name} points at accessor ${index}, which the document does not define.`);
      if (accessor.type !== "VEC3")
        refuse(`${name} is ${JSON.stringify(accessor.type)}, not VEC3; three floats per vertex are bound for position, normal and colour alike, so even a VEC4 colour carrying alpha cannot be taken.`);
      if (accessor.componentType !== FLOAT)
        refuse(`${name} stores ${COMPONENTS[accessor.componentType] || `component type ${accessor.componentType}`}, not FLOAT; float32 attributes are uploaded as they are and nothing here dequantises.`);
      if (accessor.normalized)
        refuse(`${name} is marked normalized, which describes integer components mapped into a unit range; float components only are read here.`);
      if (accessor.sparse)
        refuse(`${name} is sparse; one contiguous run per attribute is read, so the substituted vertices would be missed.`);
      if (!Number.isSafeInteger(accessor.count) || accessor.count <= 0)
        refuse(`${name} declares a vertex count of ${JSON.stringify(accessor.count)}.`);
      if (accessor.bufferView === undefined)
        refuse(`${name} has no bufferView, so every one of its ${accessor.count} vertices would read as zero.`);
      if (!Number.isSafeInteger(accessor.bufferView) || accessor.bufferView < 0)
        refuse(`${name} has an invalid bufferView index.`);
      const bufferView = list(document.bufferViews)[accessor.bufferView];
      if (!isObject(bufferView)) refuse(`${name} points at buffer view ${accessor.bufferView}, which the document does not define.`);
      if ((bufferView.buffer || 0) !== 0) refuse(`${name} reads buffer ${bufferView.buffer}; only the embedded buffer 0 exists here.`);
      if (bufferView.byteStride !== undefined && bufferView.byteStride !== 12)
        refuse(`${name} reads an interleaved buffer view with a byteStride of ${bufferView.byteStride} bytes; one tightly packed VEC3 float array per attribute is bound, which is a stride of 12.`);
      const first = bufferView.byteOffset === undefined ? 0 : bufferView.byteOffset;
      const offset = accessor.byteOffset === undefined ? 0 : accessor.byteOffset;
      if (!Number.isSafeInteger(first) || first < 0 || !Number.isSafeInteger(offset) || offset < 0)
        refuse(`${name} has an invalid or negative byte offset.`);
      const start = first + offset, length = accessor.count * 12;
      if (start % 4 !== 0)
        refuse(`${name} starts at byte ${start} of the buffer, which is not the 4-byte alignment a float accessor requires.`);
      if (!Number.isSafeInteger(bufferView.byteLength) || start + length > first + bufferView.byteLength)
        refuse(`${name} needs ${length} bytes from byte ${start} but its buffer view ends at byte ${first + (bufferView.byteLength || 0)}.`);
      if (start + length > binary.length)
        refuse(`${name} reads to byte ${start + length}, past the end of the ${binary.length}-byte BIN chunk.`);
      return {accessor, start, count: accessor.count};
    }

    // glTF is little-endian by definition, and the BIN chunk is only 4-byte
    // aligned inside a file that may itself sit at any offset, so read each
    // float explicitly rather than view the bytes as a Float32Array.
    function read(name, source) {
      const out = new Float32Array(source.count * 3);
      for (let i = 0; i < out.length; i++) {
        const value = view.getFloat32(binary.start + source.start + i * 4, true);
        if (!Number.isFinite(value))
          refuse(`${name} holds ${Number.isNaN(value) ? "NaN" : "an infinity"} at component ${i}; the camera framing, the picker and the exporter all require finite values.`);
        out[i] = value;
      }
      return out;
    }

    const plans = SEMANTICS.map(name => plan(name)), count = plans[0].count;
    if (plans[1].count !== count || plans[2].count !== count)
      refuse(`POSITION has ${count} vertices, NORMAL has ${plans[1].count} and COLOR_0 has ${plans[2].count}; the three must describe the same vertices.`);
    if (count % 3 !== 0)
      refuse(`the primitive holds ${count} vertices, which is not a whole number of triangles.`);
    if (aircraft) {
      const views = list(document.bufferViews);
      if (views.length !== 3 || buffers[0].byteLength !== count * 36 || binary.length !== count * 36)
        refuse("its aircraft global arrays must occupy exactly three packed buffer views in one embedded buffer.");
      for (let a = 0; a < 3; a++) {
        if (plans[a].accessor.bufferView !== a || (plans[a].accessor.byteOffset || 0) !== 0 ||
            views[a].buffer !== 0 || views[a].byteOffset !== a * count * 12 || views[a].byteLength !== count * 12)
          refuse("its aircraft global accessors 0/1/2 do not describe the original continuous arrays.");
      }
    }
    const positions = read("POSITION", plans[0]), normals = read("NORMAL", plans[1]), colors = read("COLOR_0", plans[2]);
    // A zero-length normal has no direction to shade with, and equipment-export.js
    // refuses to write one back, so accepting it would import a model that draws
    // black and then cannot be saved — the half-import this reader exists to stop.
    // Lengths other than one are left alone: the exporter renormalises, and every
    // three finite floats that are not all zero have a length to divide by.
    for (let i = 0; i < normals.length; i += 3) {
      if (normals[i] === 0 && normals[i + 1] === 0 && normals[i + 2] === 0)
        refuse(`NORMAL is zero at vertex ${i / 3}; a zero-length normal cannot be shaded or written back out.`);
    }
    for (let i = 0; i < colors.length; i++) {
      if (colors[i] < 0 || colors[i] > 1)
        refuse(`COLOR_0 component ${i} is ${colors[i]}, outside 0..1; the shader raises vertex colour through gamma, and the exporter would refuse to write it back.`);
    }

    // Rebuilt from the POSITION accessor's min/max, because the exporter writes no
    // bounds into extras. Those were measured over the float32 vertex data, so
    // they are the generator's float64 bounds rounded to float32: equal to within
    // a float32 ulp, and never wider than the geometry. The viewer only frames a
    // camera from them, so a difference in the seventh digit is invisible.
    const extent = field => {
      const values = list(plans[0].accessor[field]);
      if (values.length !== 3 || !values.every(Number.isFinite))
        refuse(`the POSITION accessor carries no usable ${field}; glTF requires it on POSITION, and the camera is framed from those bounds.`);
      return values.map(Number);
    };
    const min = extent("min"), max = extent("max");
    for (let i = 0; i < positions.length; i++) {
      const axis = i % 3, slack = Math.max(1, Math.abs(positions[i])) * 1e-6;
      if (positions[i] < min[axis] - slack || positions[i] > max[axis] + slack)
        refuse(`the declared POSITION bounds do not contain the geometry: component ${i} is ${positions[i]}, outside ${min[axis]} to ${max[axis]} on ${"XYZ"[axis]}. A camera framed on those bounds would cut the model off.`);
    }

    if (extras.parts !== undefined && !Array.isArray(extras.parts))
      refuse("its mesh extras.parts is not an array; the part selector reads a list of named vertex ranges.");
    const parts = list(extras.parts).map((part, index) => {
      const where = isObject(part) && typeof part.name === "string" && part.name ? `part "${part.name}"` : `part ${index}`;
      if (!isObject(part)) refuse(`${where} is not an object.`);
      if (typeof part.name !== "string" || !part.name)
        refuse(`${where} has no name; the selector, the highlight and the click target all address a part by name.`);
      if (!Number.isSafeInteger(part.first) || !Number.isSafeInteger(part.count) || part.first < 0 || part.count < 0)
        refuse(`${where} declares first ${JSON.stringify(part.first)} and count ${JSON.stringify(part.count)}, which is not a vertex range.`);
      if (part.first % 3 !== 0 || part.count % 3 !== 0)
        refuse(`${where} covers vertices ${part.first} to ${part.first + part.count}, which is not a whole number of triangles; a highlight drawn from it would begin mid-triangle.`);
      if (part.first + part.count > count)
        refuse(`${where} ends at vertex ${part.first + part.count} but the primitive holds ${count}; that highlight would read past the uploaded buffer.`);
      return {name: part.name, first: part.first, count: part.count,
        ...(typeof part.slot === "string" ? {slot: part.slot} : {}),
        ...(typeof part.label === "string" ? {label: part.label} : {})};
    });

    const triangleCount = count / 3;
    if (extras.triangleCount !== undefined && extras.triangleCount !== triangleCount)
      refuse(`its extras record ${JSON.stringify(extras.triangleCount)} triangles but the geometry holds ${triangleCount}; one of the two is stale and neither can be trusted.`);
    if (extras.specification !== undefined && !isObject(extras.specification))
      refuse("its extras.specification is not an object; a platform and a component map are read from it.");

    let surfaces;
    if (aircraft) {
      // This is a checked inverse of our writer, not a generic glTF primitive
      // merger. Every draw range must describe exactly the metadata the picker
      // and translucent pass will use after import.
      if (!Array.isArray(extras.surfaces)) refuse("its aircraft surface ranges are not an array.");
      let end = 0;
      surfaces = extras.surfaces.map(surface => {
        if (!isObject(surface) || Object.keys(surface).some(key => !["first", "count", "material", "opacity"].includes(key)) ||
            surface.material !== "glass" || !Number.isSafeInteger(surface.first) || !Number.isSafeInteger(surface.count) ||
            surface.first < 0 || surface.count <= 0 || surface.first % 3 || surface.count % 3 || surface.first + surface.count > count)
          refuse("its aircraft surface range is not a bounded glass triangle range.");
        if (!Number.isFinite(surface.opacity) || surface.opacity <= 0 || surface.opacity >= 1)
          refuse("its aircraft glass opacity must be strictly between zero and one.");
        if (surface.first < end) refuse("its aircraft surface ranges overlap or are out of order.");
        end = surface.first + surface.count;
        const owners = parts.filter(p => p.count > 0 && surface.first < p.first + p.count && end > p.first);
        if (owners.length !== 1 || surface.first < owners[0].first || end > owners[0].first + owners[0].count)
          refuse("its aircraft glass must belong to exactly one model part.");
        return {first: surface.first, count: surface.count, material: "glass", opacity: surface.opacity};
      });
      const opaque = [], transparent = [], opacities = [];
      end = 0;
      for (const surface of surfaces) {
        if (surface.first > end) opaque.push({first: end, count: surface.first - end, material: 0});
        end = surface.first + surface.count;
        let index = opacities.indexOf(surface.opacity);
        if (index < 0) { index = opacities.length; opacities.push(surface.opacity); }
        transparent.push({...surface, material: index + 1});
      }
      if (end < count) opaque.push({first: end, count: count - end, material: 0});
      const ranges = [...opaque, ...transparent], accessors = list(document.accessors);
      if (primitives.length !== ranges.length || accessors.length !== (surfaces.length ? 3 + ranges.length * 3 : 3))
        refuse("its aircraft primitive/accessor counts do not match the declared surface partition.");
      if (materials.length !== opacities.length + 1)
        refuse("its aircraft material count does not match the declared glass opacities.");
      for (let index = 0; index < materials.length; index++) {
        const material = materials[index], pbr = isObject(material) && material.pbrMetallicRoughness, glass = index > 0;
        const allowed = glass ? ["name", "pbrMetallicRoughness", "alphaMode", "doubleSided"] : ["name", "pbrMetallicRoughness"];
        if (!isObject(material) || Object.keys(material).some(key => !allowed.includes(key)) || !isObject(pbr) ||
            Object.keys(pbr).some(key => !["baseColorFactor", "metallicFactor", "roughnessFactor"].includes(key)) ||
            JSON.stringify(pbr.baseColorFactor) !== JSON.stringify([1, 1, 1, glass ? opacities[index - 1] : 1]) ||
            pbr.metallicFactor !== (glass ? 0 : .08) || pbr.roughnessFactor !== (glass ? .12 : .62) ||
            (glass && (material.alphaMode !== "BLEND" || material.doubleSided !== true)))
          refuse(`its aircraft material ${index} does not match the supported paint/glass shading and opacity.`);
      }
      for (let i = 0; i < ranges.length; i++) {
        const range = ranges[i], expected = surfaces.length ? 3 + i * 3 : 0;
        if (primitives[i].material !== range.material)
          refuse("its aircraft primitive material does not match its opaque/glass surface range.");
        for (let a = 0; a < 3; a++) {
          const name = SEMANTICS[a];
          if (drawAttributes[i][name] !== expected + a)
            refuse("its aircraft primitive attributes are not the expected slices of global accessors 0/1/2.");
          const slice = plan(name, expected + a);
          if (slice.accessor.bufferView !== a || slice.count !== range.count || slice.start !== plans[a].start + range.first * 12)
            refuse("its aircraft primitive accessor slice does not match the declared surface range.");
        }
        const position = accessors[expected], minimum = [Infinity, Infinity, Infinity], maximum = [-Infinity, -Infinity, -Infinity];
        for (let j = range.first * 3; j < (range.first + range.count) * 3; j++) {
          minimum[j % 3] = Math.min(minimum[j % 3], positions[j]); maximum[j % 3] = Math.max(maximum[j % 3], positions[j]);
        }
        if (JSON.stringify(position.min) !== JSON.stringify(minimum) || JSON.stringify(position.max) !== JSON.stringify(maximum))
          refuse("its aircraft POSITION slice bounds do not match the geometry in that surface range.");
      }
    }

    return {
      positions, normals, colors, bounds: {min, max}, parts, triangleCount,
      description: typeof extras.description === "string" ? extras.description : "",
      specification: extras.specification,
      ...(aircraft ? {assetKind: "aircraft", surfaces} : {})
    };
  }

  return Object.freeze({fromGlb});
});
