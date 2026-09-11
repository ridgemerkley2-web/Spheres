// Render-state ownership and portable glass materials, independent of art pins.
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const viewer=require('../../spheres-web/ui/equipment-model.js'),{glb}=require('../../spheres-web/ui/equipment-export.js');
const source=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/equipment-model.js'),'utf8');
function model(){return {assetKind:'aircraft',positions:new Float32Array([0,0,0,1,0,0,0,1,0,0,0,1,1,0,1,0,1,1,0,0,2,1,0,2,0,1,2]),normals:new Float32Array(Array.from({length:27},(_,i)=>i%3===2?1:0)),colors:new Float32Array(27).fill(.4),bounds:{min:[0,0,0],max:[1,1,2]},triangleCount:3,parts:[{name:'Body',first:0,count:9,slot:'air_avionics'}],surfaces:[{first:3,count:3,material:'glass',opacity:.24}]};}
function decode(bytes){const header=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength),length=header.getUint32(12,true);return {json:JSON.parse(new TextDecoder().decode(bytes.subarray(20,20+length))),binary:new DataView(bytes.buffer,bytes.byteOffset+28+length)};}
function fixture({shadows=false,noGraphics=false}={}){
  const frames=new Map(),gpu=new Set(),draws=[],uniforms=new Map(),enabled=new Set(),sources=[];let next=1,framebuffer=null,writeDepth=true,cull=null,currentModel=model();
  const allocate=()=>{const resource={id:next++};gpu.add(resource);return resource;},remove=resource=>{assert(gpu.has(resource));gpu.delete(resource);};
  const gl={ARRAY_BUFFER:1,STATIC_DRAW:2,VERTEX_SHADER:3,FRAGMENT_SHADER:4,COMPILE_STATUS:5,LINK_STATUS:6,DEPTH_TEST:7,LEQUAL:8,FLOAT:9,TRIANGLES:10,COLOR_BUFFER_BIT:16,DEPTH_BUFFER_BIT:32,BLEND:33,SRC_ALPHA:34,ONE_MINUS_SRC_ALPHA:35,CULL_FACE:36,FRONT:37,BACK:38,
    FRAMEBUFFER:40,FRAMEBUFFER_COMPLETE:41,RENDERBUFFER:42,TEXTURE_2D:43,RGBA:44,UNSIGNED_BYTE:45,DEPTH_COMPONENT16:46,COLOR_ATTACHMENT0:47,DEPTH_ATTACHMENT:48,TEXTURE0:49,TEXTURE_MIN_FILTER:50,TEXTURE_MAG_FILTER:51,TEXTURE_WRAP_S:52,TEXTURE_WRAP_T:53,NEAREST:54,CLAMP_TO_EDGE:55,MAX_TEXTURE_SIZE:56,MAX_RENDERBUFFER_SIZE:57,MAX_TEXTURE_IMAGE_UNITS:58,HIGH_FLOAT:59,DITHER:60,NO_ERROR:0,
    createShader:allocate,deleteShader:remove,createProgram:allocate,deleteProgram:remove,createBuffer:allocate,deleteBuffer:remove,getShaderParameter:()=>true,getProgramParameter:()=>true,getAttribLocation:(_p,name)=>({aPosition:0,aNormal:1,aColor:2}[name]??-1),getUniformLocation:(_p,name)=>name,
    shaderSource:(_s,text)=>sources.push(text),uniform1f:(name,value)=>uniforms.set(name,value),uniform1i:(name,value)=>uniforms.set(name,value),enable:value=>enabled.add(value),disable:value=>enabled.delete(value),depthMask:value=>{writeDepth=value;},cullFace:value=>{cull=value;},blendFunc:(a,b)=>assert.deepEqual([a,b],[34,35]),
    drawArrays(_mode,first,count){draws.push({first,count,framebuffer,depth:writeDepth,blend:enabled.has(gl.BLEND),cull:enabled.has(gl.CULL_FACE)?cull:null,uniforms:new Map(uniforms)});}
  };
  for(const name of ['compileShader','attachShader','linkProgram','bindBuffer','bufferData','depthFunc','clearColor','enableVertexAttribArray','disableVertexAttribArray','vertexAttribPointer','viewport','clear','useProgram','uniformMatrix4fv','uniform3fv'])gl[name]=()=>{};
  if(shadows){Object.assign(gl,{createFramebuffer:allocate,deleteFramebuffer:remove,createTexture:allocate,deleteTexture:remove,createRenderbuffer:allocate,deleteRenderbuffer:remove,bindFramebuffer:(_target,value)=>{framebuffer=value;},checkFramebufferStatus:()=>gl.FRAMEBUFFER_COMPLETE,getParameter:name=>name===gl.MAX_TEXTURE_IMAGE_UNITS?8:1024,getShaderPrecisionFormat:()=>({precision:23}),getError:()=>0});for(const name of ['framebufferTexture2D','bindTexture','texImage2D','texParameteri','activeTexture','bindRenderbuffer','renderbufferStorage','framebufferRenderbuffer'])gl[name]=()=>{};}
  function node(){return {listeners:new Map(),style:{},children:[],dataset:{},isConnected:true,setAttribute(){},appendChild(child){this.children.push(child);child.parentNode=this;},replaceChildren(...children){this.children=[];children.forEach(c=>this.appendChild(c));},remove(){this.isConnected=false;},addEventListener(e,fn){this.listeners.set(e,fn);},removeEventListener(e){this.listeners.delete(e);},getBoundingClientRect:()=>({left:0,top:0,width:800,height:500})};}
  const doc=node(),canvas=node(),host=node(),slot=node(),status=node(),part=node();doc.hidden=false;doc.body=node();doc.createElement=tag=>tag==='canvas'?canvas:node();canvas.getContext=()=>noGraphics?null:gl;host.ownerDocument=doc;host.querySelector=query=>({'[data-model-canvas]':slot,'[data-model-status]':status,'[data-model-part]':part}[query]||null);host.querySelectorAll=()=>[];
  class Observer{observe(){}disconnect(){}}
  const context={module:{exports:{}},Float32Array,Math,Number,Map,Set,Array,Object,JSON,document:doc,ResizeObserver:Observer,IntersectionObserver:Observer,EquipmentMesh:{build:()=>currentModel},EquipmentExport:{glb},requestAnimationFrame:fn=>{const id=next++;frames.set(id,fn);return id;},cancelAnimationFrame:id=>frames.delete(id)};
  vm.runInNewContext(source,context);const controller=context.module.exports.mount(host,{platform:'air_tactical_strike',components:{}});
  return {gl,gpu,draws,sources,status,part,controller,enabled,flush(){const pending=[...frames.values()];frames.clear();pending.forEach(fn=>fn(100));},setModel(value){currentModel=value;},lose(){gpu.clear();canvas.listeners.get('webglcontextlost')({preventDefault(){}});},restore(){canvas.listeners.get('webglcontextrestored')();},depth:()=>writeDepth};
}

test('opaque and glass ranges partition the mesh once and preserve selectable part ownership',()=>{
  const mesh=model(),before=structuredClone(mesh),ranges=viewer.surfaceRanges(mesh),coverage=new Uint8Array(9);
  for(const range of [...ranges.opaque,...ranges.glass])for(let i=range.first;i<range.first+range.count;i++)coverage[i]++;
  assert.deepEqual([...coverage],Array(9).fill(1));assert.deepEqual(ranges.opaque,[{first:0,count:3},{first:6,count:3}]);assert.equal(ranges.glass[0].part,'Body');assert.deepEqual(mesh,before);
  assert.equal(viewer.raycast(mesh,[.1,.1,3],[0,0,-1]).part.name,'Body');
});
test('invalid surfaces fail closed in both display and GLB export',()=>{
  for(const bad of [{first:1},{count:4},{first:-3},{count:0},{count:30},{opacity:NaN},{opacity:0},{opacity:1},{material:'paint'}]){const mesh=model();Object.assign(mesh.surfaces[0],bad);assert.throws(()=>viewer.surfaceRanges(mesh));assert.throws(()=>glb(mesh));}
  for(const mutate of [m=>m.surfaces.push({...m.surfaces[0]}),m=>m.parts=[],m=>m.parts.push({...m.parts[0]}),m=>{m.surfaces[0].count=6;m.parts.push({name:'overlap',first:6,count:3});},m=>m.parts=[{name:'left',first:0,count:3},{name:'right',first:6,count:3}],m=>m.surfaces={}]){const mesh=model();mutate(mesh);assert.throws(()=>viewer.surfaceRanges(mesh));assert.throws(()=>glb(mesh));}
});
test('aircraft GLB keeps global buffers and adds bounded glass accessors with portable alpha materials',()=>{
  const mesh=model(),{json,binary}=decode(glb(mesh));assert.equal(json.meshes[0].extras.assetKind,'aircraft');assert.deepEqual(json.meshes[0].extras.surfaces,mesh.surfaces);
  assert.equal(json.accessors[0].count,9);assert.equal(json.accessors[1].count,9);assert.equal(json.accessors[2].count,9);
  const used=new Uint8Array(9);
  for(const primitive of json.meshes[0].primitives){
    const a=json.accessors[primitive.attributes.POSITION],view=json.bufferViews[a.bufferView];assert(a.byteOffset+a.count*12<=view.byteLength);
    for(let i=a.byteOffset/12;i<a.byteOffset/12+a.count;i++)used[i]++;
    const material=json.materials[primitive.material];if(material.alphaMode==='BLEND'){assert.equal(a.byteOffset,36);assert.equal(a.count,3);assert.equal(material.doubleSided,true);assert.equal(material.pbrMetallicRoughness.baseColorFactor[3],.24);}else assert.equal(material.pbrMetallicRoughness.baseColorFactor[3],1);
    for(let i=0;i<a.count*3;i++)assert.equal(binary.getFloat32(view.byteOffset+a.byteOffset+i*4,true),mesh.positions[a.byteOffset/4+i]);
  }
  assert.deepEqual([...used],Array(9).fill(1));assert.deepEqual(glb(mesh),glb(mesh));
});
test('nonaircraft export bytes and opaque ranges remain unchanged even with unrelated metadata',()=>{
  const mesh={positions:new Float32Array([0,0,0,1,0,0,0,1,0]),normals:new Float32Array([0,0,1,0,0,1,0,0,1]),colors:new Float32Array(9).fill(.4),parts:[{name:'Body',first:0,count:3}],triangleCount:1};
  assert.equal(crypto.createHash('sha256').update(glb(mesh,'Compatibility fixture')).digest('hex'),'953419431a825eb26fb660175c659f70ba61aff657f2645105d3b7470d6277ad');
  assert.deepEqual(glb({...mesh,surfaces:'unrelated metadata'}),glb(mesh));assert.deepEqual(viewer.surfaceRanges({...mesh,surfaces:'unrelated metadata'}),{opaque:[{first:0,count:3}],glass:[]});
});
test('glass draws after opaque with blending and no depth writes, then restores all owned GL state',()=>{
  const f=fixture();f.flush();const visible=f.draws.filter(d=>d.uniforms.get('uMode')===0),glass=visible.filter(d=>d.uniforms.get('uAircraftGlass')===1);
  assert.deepEqual(visible.map(d=>[d.first,d.count]),[[0,3],[6,3],[3,3],[3,3]]);assert.equal(glass.length,2);assert(glass.every(d=>d.blend&&!d.depth));assert.deepEqual(glass.map(d=>d.cull),[f.gl.FRONT,f.gl.BACK]);assert(glass.every(d=>d.uniforms.get('uSurfaceOpacity')===.24));
  assert.equal(f.depth(),true);assert.equal(f.enabled.has(f.gl.BLEND),false);assert.equal(f.enabled.has(f.gl.CULL_FACE),false);
  const shader=f.sources.find(s=>s.includes('void main(){if(uMode'));
  assert(shader.includes('if(uAircraftEnabled>.5)'));assert(shader.includes('if(!gl_FrontFacing)n=-n'));
  f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('selection highlights opaque intersections and glass without drawing an opaque canopy overlay',()=>{
  const f=fixture();f.controller.selectPart('air_avionics');f.flush();const highlighted=f.draws.filter(d=>d.uniforms.get('uMode')===0&&d.uniforms.get('uHighlight')===1);
  assert.deepEqual(highlighted.map(d=>[d.first,d.count,d.uniforms.get('uAircraftGlass')]),[[0,3,0],[6,3,0],[3,3,1],[3,3,1]]);assert.equal(f.part.value,'Body');f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('canopy is excluded from both cached depth shadows and fallback projected shadows',()=>{
  for(const shadows of [true,false]){const f=fixture({shadows});f.flush();const casts=f.draws.filter(d=>shadows?d.framebuffer:d.uniforms.get('uMode')===2);assert.deepEqual(casts.map(d=>[d.first,d.count]),[[0,3],[6,3]]);if(shadows){f.controller.rotate(.1,0);f.flush();assert.equal(f.draws.filter(d=>d.framebuffer).length,2);}f.controller.dispose();assert.equal(f.gpu.size,0);}
});
test('restoration preserves glass, selection and exports; bad replacement never retains stale geometry',()=>{
  const f=fixture({shadows:true});f.controller.selectPart('air_avionics');f.flush();const bytes=f.controller.exportGlb();f.lose();f.restore();f.flush();assert.deepEqual(f.controller.exportGlb(),bytes);assert.equal(f.part.value,'Body');assert(f.draws.at(-1).blend);
  const bad=model();bad.surfaces[0].count=6;bad.parts[0].count=6;f.setModel(bad);f.controller.update({platform:'air_tactical_strike',components:{air_engine:'different'}});assert.equal(f.controller.exportGlb(),null);assert.match(f.status.textContent,/could not be displayed/);f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('unavailable WebGL retains downloadable transparent geometry and a clear fallback',()=>{const f=fixture({noGraphics:true});assert.match(f.status.textContent,/unavailable/);assert(decode(f.controller.exportGlb()).json.materials.some(m=>m.alphaMode==='BLEND'));f.controller.dispose();assert.equal(f.gpu.size,0);});
