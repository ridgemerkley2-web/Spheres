/* Original Spheres game art. Export the designer's triangle mesh as glTF 2.0 binary. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.EquipmentExport = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  function glb(mesh, name = "Spheres equipment") {
    if (!mesh || typeof mesh !== "object") throw new TypeError("An equipment triangle mesh is required.");
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
    let aircraftSurfaces=null;
    if(mesh.assetKind==='aircraft'){
      if(mesh.surfaces!==undefined&&!Array.isArray(mesh.surfaces))throw new TypeError('Invalid aircraft surface ranges.');
      aircraftSurfaces=(mesh.surfaces||[]).map(surface=>{
        if(!surface||surface.material!=='glass'||!Number.isSafeInteger(surface.first)||!Number.isSafeInteger(surface.count)||surface.first<0||surface.count<=0||surface.first%3||surface.count%3||surface.first+surface.count>count||!Number.isFinite(surface.opacity)||surface.opacity<=0||surface.opacity>=1)throw new TypeError('Invalid aircraft surface ranges.');
        const owners=parts.filter(p=>p.count>0&&surface.first<p.first+p.count&&surface.first+surface.count>p.first);
        if(owners.length!==1||surface.first<owners[0].first||surface.first+surface.count>owners[0].first+owners[0].count)throw new TypeError('Aircraft glass must belong to exactly one model part.');
        return {first:surface.first,count:surface.count,material:'glass',opacity:surface.opacity};
      }).sort((a,b)=>a.first-b.first);
      for(let i=1;i<aircraftSurfaces.length;i++)if(aircraftSurfaces[i].first<aircraftSurfaces[i-1].first+aircraftSurfaces[i-1].count)throw new TypeError('Aircraft surface ranges overlap.');
    }
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
    const label = String(name || "Spheres equipment").slice(0, 128);
    const description = typeof mesh.description === "string" ? mesh.description : "Original configurable Spheres equipment game model.";
    const document = {
      asset: {version: "2.0", generator: mesh.assetKind === "character" ? "Spheres Character Studio" : "Spheres Equipment Designer", copyright: "Original Spheres procedural game art"},
      scene: 0,
      scenes: [{name: label, nodes: [0]}],
      nodes: [{name: label, mesh: 0}],
      meshes: [{name: label, primitives: [{attributes: {POSITION: 0, NORMAL: 1, COLOR_0: 2}, material: 0, mode: 4}],
        extras: {description, parts, triangleCount: count / 3, gameArt: true, ...(mesh.specification?{specification:mesh.specification}:{})}}],
      materials: [{name: mesh.assetKind === "character" ? "Sculpted character" : "Painted vehicle", pbrMetallicRoughness: {
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
    if(aircraftSurfaces){
      // Global accessors 0/1/2 remain the exact original buffers for tooling.
      // Draw primitives use slices of those same buffers, without duplication.
      document.meshes[0].extras.assetKind='aircraft';
      document.meshes[0].extras.surfaces=aircraftSurfaces;
      document.materials[0]={name:'Aircraft matte paint and fittings',pbrMetallicRoughness:{baseColorFactor:[1,1,1,1],metallicFactor:.08,roughnessFactor:.62}};
      if(aircraftSurfaces.length){
        const opaque=[];let first=0;
        for(const surface of aircraftSurfaces){if(surface.first>first)opaque.push({first,count:surface.first-first,material:0});first=surface.first+surface.count;}
        if(first<count)opaque.push({first,count:count-first,material:0});
        const glassMaterials=new Map();
        const transparent=aircraftSurfaces.map(surface=>{
          if(!glassMaterials.has(surface.opacity)){
            glassMaterials.set(surface.opacity,document.materials.length);
            document.materials.push({name:'Aircraft canopy glass',pbrMetallicRoughness:{baseColorFactor:[1,1,1,surface.opacity],metallicFactor:0,roughnessFactor:.12},alphaMode:'BLEND',doubleSided:true});
          }
          return {...surface,material:glassMaterials.get(surface.opacity)};
        });
        document.meshes[0].primitives=[...opaque,...transparent].map(range=>{
          const firstAccessor=document.accessors.length,minimum=[Infinity,Infinity,Infinity],maximum=[-Infinity,-Infinity,-Infinity];
          for(let i=range.first*3;i<(range.first+range.count)*3;i++){minimum[i%3]=Math.min(minimum[i%3],mesh.positions[i]);maximum[i%3]=Math.max(maximum[i%3],mesh.positions[i]);}
          for(let index=0;index<3;index++)document.accessors.push({bufferView:index,byteOffset:range.first*12,componentType:5126,count:range.count,type:'VEC3',...(index===0?{min:minimum,max:maximum}:{})});
          return {attributes:{POSITION:firstAccessor,NORMAL:firstAccessor+1,COLOR_0:firstAccessor+2},material:range.material,mode:4};
        });
      }
    }
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
