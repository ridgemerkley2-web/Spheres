const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const viewer=require('../../spheres-web/ui/equipment-model.js'),{build}=require('../../spheres-web/ui/equipment-mesh.js');
const source=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/equipment-model.js'),'utf8');
function fixture({noGraphics=false,lod=2}={}){
  let next=1,currentMesh=build({platform:'air_tactical_strike',lod});const frames=new Map(),gpu=new Set(),uniforms=new Map(),draws=[];
  const create=()=>{const id=next++;gpu.add(id);return id;},remove=id=>{assert(gpu.has(id));gpu.delete(id);};
  const gl={createShader:create,deleteShader:remove,createProgram:create,deleteProgram:remove,createBuffer:create,deleteBuffer:remove,getShaderParameter:()=>true,getProgramParameter:()=>true,getAttribLocation:(_p,name)=>({aPosition:0,aNormal:1,aColor:2}[name]??-1),getUniformLocation:(_p,name)=>name,ARRAY_BUFFER:1,STATIC_DRAW:2,VERTEX_SHADER:3,FRAGMENT_SHADER:4,COMPILE_STATUS:5,LINK_STATUS:6,DEPTH_TEST:7,LEQUAL:8,FLOAT:9,TRIANGLES:10,COLOR_BUFFER_BIT:16,DEPTH_BUFFER_BIT:32,
    uniformMatrix4fv:(name,_transpose,matrix)=>uniforms.set(name,new Float32Array(matrix)),uniform3fv:(name,value)=>uniforms.set(name,Array.from(value)),uniform1f:(name,value)=>uniforms.set(name,value),drawArrays:()=>draws.push(new Map(uniforms))};
  for(const name of ['shaderSource','compileShader','attachShader','linkProgram','bindBuffer','bufferData','enable','depthFunc','clearColor','enableVertexAttribArray','vertexAttribPointer','viewport','clear','useProgram'])gl[name]=()=>{};
  function node(){return {listeners:new Map(),attrs:{},style:{},children:[],dataset:{},isConnected:true,setAttribute(k,v){this.attrs[k]=v;},appendChild(c){this.children.push(c);c.parentNode=this;},replaceChildren(...children){this.children=[];children.forEach(c=>this.appendChild(c));},remove(){this.isConnected=false;},dispatchEvent(e){this.lastEvent=e;},addEventListener(e,fn){this.listeners.set(e,fn);},removeEventListener(e){this.listeners.delete(e);},getBoundingClientRect:()=>({left:0,top:0,width:1000,height:600})};}
  const doc=node(),canvas=node(),host=node(),slot=node(),status=node(),part=node(),buttons=Array.from({length:3},node);buttons[0].dataset.modelFocus='air_avionics';buttons[1].dataset.modelFocus='air_engine';buttons[2].dataset={modelFocus:'air_engine',modelFocusRegion:'front'};
  doc.hidden=false;doc.body=node();doc.createElement=tag=>tag==='canvas'?canvas:node();canvas.getContext=()=>noGraphics?null:gl;host.ownerDocument=doc;host.querySelector=s=>({'[data-model-canvas]':slot,'[data-model-status]':status,'[data-model-part]':part}[s]||null);host.querySelectorAll=s=>s==='[data-model-focus]'?buttons:[];
  class Observer{observe(){}disconnect(){}}
  const context={module:{exports:{}},Float32Array,Math,Number,Map,Set,Array,Object,JSON,document:doc,ResizeObserver:Observer,IntersectionObserver:Observer,CustomEvent:class{constructor(type,options){this.type=type;Object.assign(this,options);}},EquipmentMesh:{build:()=>currentMesh},EquipmentExport:{glb:mesh=>mesh},requestAnimationFrame:fn=>{const id=next++;frames.set(id,fn);return id;},cancelAnimationFrame:id=>frames.delete(id)};
  vm.runInNewContext(source,context);const controller=context.module.exports.mount(host,{platform:'air_tactical_strike',lod});
  return {controller,buttons,part,status,host,gpu,draws,mesh:()=>currentMesh,setMesh:m=>{currentMesh=m;},flush(){const pending=[...frames.values()];frames.clear();pending.forEach(fn=>fn(100));},camera:()=>draws.at(-1),lose(){gpu.clear();canvas.listeners.get('webglcontextlost')({preventDefault(){}});},restore(){canvas.listeners.get('webglcontextrestored')();}};
}
function projected(matrix,p){const clip=[0,0,0,0];for(let r=0;r<4;r++)for(let k=0;k<4;k++)clip[r]+=matrix[k*4+r]*[...p,1][k];return clip.slice(0,3).map(v=>v/clip[3]);}

test('aircraft close-up bounds use real cockpit glass and finite end zones of the actual engine meshes',()=>{
  for(const platform of ['air_light_attack','air_tactical_strike'])for(const lod of [0,2]){
    const mesh=build({platform,lod}),cockpit=viewer.partBounds(mesh,'air_avionics'),engine=viewer.partBounds(mesh,'air_engine'),rear=viewer.partBounds(mesh,'air_engine','rear'),front=viewer.partBounds(mesh,'air_engine','front');
    assert(cockpit&&engine&&rear&&front);if(mesh.surfaces?.length)assert(cockpit.bounds.min[2]>0,'Authored cockpit glass excludes rear radio aerials');
    assert(rear.bounds.max[2]-rear.bounds.min[2]<=1.601);assert(front.bounds.max[2]-front.bounds.min[2]<=1.601);
    assert.equal(rear.bounds.min[2],engine.bounds.min[2]);assert.equal(front.bounds.max[2],engine.bounds.max[2]);assert(rear.bounds.max[2]<front.bounds.min[2]);
    for(const target of [cockpit,rear,front])for(let axis=0;axis<3;axis++){assert(Number.isFinite(target.bounds.min[axis]));assert(target.bounds.max[axis]>=target.bounds.min[axis]);}
  }
});
test('cockpit falls back to real avionics bounds without glass; malformed and missing semantic ranges are unavailable',()=>{
  const mesh=build({platform:'air_light_attack',lod:2});const fallback=viewer.partBounds({...mesh,surfaces:[]},'air_avionics');assert(fallback);assert.equal(viewer.partBounds(mesh,'unknown'),null);assert.equal(viewer.partBounds(null,'air_engine'),null);
  const broken={...mesh,parts:[{name:'broken',slot:'air_engine',first:0,count:mesh.positions.length}]};assert.equal(viewer.partBounds(broken,'air_engine'),null);
});
test('Cockpit, Engines and Intakes buttons point the camera at contained component detail, with selection synchronized',()=>{
  const f=fixture();f.flush();const fullEye=f.camera().get('uEye');
  for(const [index,slot,region] of [[0,'air_avionics','rear'],[1,'air_engine','rear'],[2,'air_engine','front']]){
    f.buttons[index].listeners.get('click')();f.flush();const target=viewer.partBounds(f.mesh(),slot,region),camera=f.camera(),eye=camera.get('uEye');
    assert.notDeepEqual(eye,fullEye);assert.equal(f.part.value,target.part.name);assert.equal(f.host.lastEvent.detail.slot,slot);assert.equal(f.buttons[index].attrs['aria-pressed'],'true');
    const b=target.bounds;for(const x of [b.min[0],b.max[0]])for(const y of [b.min[1],b.max[1]])for(const z of [b.min[2],b.max[2]])assert(projected(camera.get('uVP'),[x,y,z]).every(v=>Math.abs(v)<1),'The focused detail is inside every clipping plane');
    if(index===0){assert(eye[2]<b.min[2],'Cockpit looks over the seats toward the instrument faces');assert(eye[1]>b.max[1]);}else if(index===1)assert(eye[2]<b.min[2]);else assert(eye[2]>b.max[2]);
  }
  f.controller.dispose();assert.equal(f.gpu.size,0);assert(f.buttons.every(b=>b.listeners.size===0));
});
test('intake inspection looks into an outboard engine before hitting the fuselage, without hiding any mesh',()=>{
  for(const lod of [0,2]){const f=fixture({lod});f.buttons[2].listeners.get('click')();f.flush();const target=viewer.partBounds(f.mesh(),'air_engine','front'),eye=f.camera().get('uEye'),center=target.bounds.min.map((v,i)=>(v+target.bounds.max[i])/2),direction=center.map((v,i)=>v-eye[i]),length=Math.hypot(...direction);
  assert.equal(target.parts.length,1);assert(eye[0]>1);const hit=viewer.raycast(f.mesh(),eye,direction.map(v=>v/length));assert.equal(hit?.part?.name,target.part.name,'The first visible surface is the intake engine rather than an occluding airframe');assert.equal(f.controller.exportGlb().positions,f.mesh().positions,'Inspection preserves every exported triangle');assert.equal(f.controller.exportGlb().parts,f.mesh().parts);f.controller.dispose();assert.equal(f.gpu.size,0);}
});
test('ordinary views restore overview zoom, Reset fits the full model and focused picking uses the close-up camera',()=>{
  const f=fixture();f.controller.zoom(-1);f.controller.view('side');f.flush();const before=new Float32Array(f.camera().get('uVP'));
  f.controller.focusPart('air_engine');f.flush();f.controller.zoom(-1);f.controller.rotate(.15,.02);f.flush();
  const camera=f.camera(),eye=camera.get('uEye'),part=f.mesh().parts.find(p=>p.slot==='air_engine');let picked=false;
  for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){const p=[0,0,0];for(let a=0;a<3;a++)p[a]=(f.mesh().positions[i+a]+f.mesh().positions[i+3+a]+f.mesh().positions[i+6+a])/3;const screen=projected(camera.get('uVP'),p);if(screen.slice(0,2).some(v=>Math.abs(v)>.95))continue;const direction=p.map((v,a)=>v-eye[a]),length=Math.hypot(...direction),hit=viewer.raycast(f.mesh(),eye,direction.map(v=>v/length));if(hit?.part?.slot==='air_engine'){assert.equal(f.controller.pickAt((screen[0]+1)*500,(1-screen[1])*300)?.slot,'air_engine');picked=true;break;}}
  assert(picked,'An actual visible engine triangle can be selected while focused');
  f.controller.view('side');f.flush();assert.deepEqual(f.camera().get('uVP'),before);f.controller.focusPart('air_avionics');f.controller.reset();f.flush();assert.deepEqual(f.camera().get('uVP'),viewer.frame(f.mesh().bounds,1000/600,...viewer.views.hero).matrix);f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('focus survives context restoration but resets safely when model geometry changes',()=>{
  const f=fixture();f.controller.focusPart('air_avionics');f.flush();const before=new Float32Array(f.camera().get('uVP'));f.lose();assert(f.buttons.every(b=>b.disabled));assert.equal(f.controller.focusPart('air_engine'),null);f.restore();f.flush();assert.deepEqual(f.camera().get('uVP'),before);assert(f.buttons.every(b=>!b.disabled));
  f.setMesh(build({platform:'air_light_attack',lod:2}));f.controller.update({platform:'air_light_attack',lod:2});f.flush();assert(f.buttons.every(b=>b.attrs['aria-pressed']==='false'));assert.equal(f.mesh().parts.find(p=>p.name===f.part.value).slot,'air_avionics');
  assert.equal(f.buttons[0].disabled,false);assert.equal(f.buttons[1].disabled,false);assert.equal(f.buttons[2].disabled,true);assert.match(f.buttons[2].attrs.title,/tactical/);assert.equal(f.controller.focusPart('air_engine','front'),null,'The legacy light airframe has no modeled intake interior to inspect');
  assert.deepEqual(f.camera().get('uVP'),viewer.frame(f.mesh().bounds,1000/600,...viewer.views.hero).matrix);f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('unavailable graphics keeps close-up controls disabled and avoids false ready messages',()=>{const f=fixture({noGraphics:true});assert(f.buttons.every(b=>b.disabled));assert.equal(f.controller.focusPart('air_avionics'),null);assert.match(f.status.textContent,/unavailable/);f.controller.dispose();assert.equal(f.controller.focusPart('air_engine'),null);});
