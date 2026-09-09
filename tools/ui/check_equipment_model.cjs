const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const source=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/equipment-model.js'),'utf8');
const viewer=require('../../spheres-web/ui/equipment-model.js');
const bounds={min:[-2,0,-4],max:[2,4,6]};
test('perspective views contain a complete tank at normal zoom without clipping across aspect ratios',()=>{
  for(const aspect of [.55,1,1.8])for(const [yaw,pitch] of Object.values(viewer.views)){
    const c=viewer.frame(bounds,aspect,yaw,pitch,1);assert([...c.matrix,...c.eye].every(Number.isFinite));
    for(const x of [-2,2])for(const y of [0,4])for(const z of [-4,6]){const p=[x,y,z,1],clip=[0,0,0,0];for(let r=0;r<4;r++)for(let k=0;k<4;k++)clip[r]+=c.matrix[k*4+r]*p[k];
      assert(clip[3]>0);assert(Math.abs(clip[0]/clip[3])<1);assert(Math.abs(clip[1]/clip[3])<1);assert(Math.abs(clip[2]/clip[3])<1);
    }
  }
});
test('model identity follows configuration rather than display names or property insertion order',()=>{
  const a={name:'First',platform:'tank_standard',components:{sensors:'one',mobility:'two'}};
  const b={name:'Second',platform:'tank_standard',components:{mobility:'two',sensors:'one'}};
  assert.equal(viewer.modelKey(a),viewer.modelKey(b));b.components.sensors='three';assert.notEqual(viewer.modelKey(a),viewer.modelKey(b));
});
test('cosmetic paint preserves unpainted materials and never mutates original colors',()=>{
  const colors=new Float32Array([.25,.32,.20,.07,.08,.08,.8,.1,.1,.08,.2,.5]),before=new Float32Array(colors);
  for(const finish of ['sand','winter']){const painted=viewer.finishColors(colors,finish);assert.notDeepEqual(painted.slice(0,3),colors.slice(0,3));assert.deepEqual(painted.slice(3),colors.slice(3));assert(painted.every(v=>v>=0&&v<=1));}
  assert.deepEqual(colors,before);assert.equal(viewer.finishColors(colors,'olive'),colors);
});

test('changing detail level replaces the displayed and exported geometry; equivalent detail reuses it',()=>{
  const f=fixture();f.flush();assert.equal(f.builds(),1);
  for(const lod of [1,2,0]){const before=f.builds();f.controller.update({platform:'tank_standard',components:{},lod});f.flush();assert.equal(f.builds(),before+1);}
  const before=f.builds();f.controller.update({platform:'tank_standard',components:{},lod:'0'});assert.equal(f.builds(),before);
  f.controller.dispose();assert.equal(f.gpu.size,0);
});
function fixture({noGraphics=false,withMaterials=false}={}){
  let next=1,builds=0,draws=0,clears=0;const gpu=new Set(),frames=new Map(),observers=[],uploads=[];
  const allocate=()=>{const id=next++;gpu.add(id);return id;},release=id=>gpu.delete(id);
  const gl={createShader:allocate,deleteShader:release,createProgram:allocate,deleteProgram:release,createBuffer:allocate,deleteBuffer:release,
    getShaderParameter:()=>true,getProgramParameter:()=>true,getAttribLocation:()=>1,getUniformLocation:()=>({}),drawArrays:()=>draws++,
    ARRAY_BUFFER:1,STATIC_DRAW:2,VERTEX_SHADER:3,FRAGMENT_SHADER:4,COMPILE_STATUS:5,LINK_STATUS:6,DEPTH_TEST:7,LEQUAL:8,FLOAT:9,TRIANGLES:10,COLOR_BUFFER_BIT:16,DEPTH_BUFFER_BIT:32};
  for(const k of ['shaderSource','compileShader','attachShader','linkProgram','bindBuffer','bufferData','enable','depthFunc','clearColor','enableVertexAttribArray','vertexAttribPointer','viewport','clear','useProgram','uniformMatrix4fv','uniform3fv','uniform1f'])gl[k]=()=>{};
  gl.clear=()=>clears++;gl.bufferData=(_,data)=>uploads.push(data);
  function node(){return {listeners:new Map(),attrs:{},style:{},children:[],dispatched:[],isConnected:true,dataset:{},setAttribute(k,v){this.attrs[k]=v;},appendChild(c){this.children.push(c);c.parentNode=this;},replaceChildren(...children){this.children=[];children.forEach(c=>this.appendChild(c));},remove(){this.isConnected=false;},focus(){doc.activeElement=this;},dispatchEvent(e){this.dispatched.push(e);},
    addEventListener(e,f){this.listeners.set(e,f);},removeEventListener(e,f){if(this.listeners.get(e)===f)this.listeners.delete(e);},getBoundingClientRect:()=>({left:100,top:70,width:900,height:500})};}
  const doc=node();doc.hidden=false;doc.body=node();const canvas=node();canvas.getContext=()=>noGraphics?null:gl;
  doc.createElement=tag=>tag==='canvas'?canvas:node();const status=node(),slot=node(),host=node(),exportButton=node(),partSelect=node();host.ownerDocument=doc;host.querySelector=s=>s==='[data-model-canvas]'?slot:s==='[data-model-part]'?partSelect:s==='[data-model-status]'?status:null;host.querySelectorAll=s=>s==='[data-model-export]'?[exportButton]:[];
  const mesh={positions:new Float32Array([0,0,0,1,0,0,0,1,0]),normals:new Float32Array([0,0,1,0,0,1,0,0,1]),colors:new Float32Array([.25,.32,.20,.25,.32,.20,.25,.32,.20]),bounds,parts:[{name:'armament / test barrel',slot:'armament',label:'Test barrel',first:0,count:3}],triangleCount:1};
  class Observer{constructor(callback){this.callback=callback;this.closed=false;observers.push(this);}observe(){}disconnect(){this.closed=true;}}
  const ctx={module:{exports:{}},Float32Array,Math,Number,Map,Set,Array,Object,JSON,document:doc,devicePixelRatio:4,ResizeObserver:Observer,IntersectionObserver:Observer,
    CustomEvent:class{constructor(type,options){this.type=type;Object.assign(this,options);}},EquipmentMesh:{build(){builds++;return mesh;}},EquipmentExport:{glb:m=>m.colors},requestAnimationFrame:f=>{const id=next++;frames.set(id,f);return id;},cancelAnimationFrame:id=>frames.delete(id)};
  const materialPacks=[];
  if(withMaterials)ctx.MilitarySurface={glsl:require('../../spheres-web/ui/military-surface.js').glsl,create(context,ready){assert.equal(context,gl);const pack={ready,binds:[],disposals:[],bind(program){this.binds.push(program);},dispose(lost){this.disposals.push(lost);}};materialPacks.push(pack);return pack;}};
  vm.runInNewContext(source,ctx);const controller=ctx.module.exports.mount(host,{platform:'tank_standard',components:{},name:'Test'});
  function flush(){const batch=[...frames];frames.clear();for(const [,fn] of batch)fn(100);}
  return {controller,ctx,host,slot,status,canvas,doc,gl,gpu,frames,observers,uploads,exportButton,partSelect,flush,materialPacks,builds:()=>builds,draws:()=>draws,clears:()=>clears};
}

test('material readiness redraws the designer and paint/geometry updates reuse its texture pack',()=>{
  const f=fixture({withMaterials:true});f.flush();assert.equal(f.materialPacks.length,1);const pack=f.materialPacks[0],draws=f.draws(),builds=f.builds();
  pack.ready();f.flush();assert(f.draws()>draws);assert.equal(f.builds(),builds);
  f.controller.setFinish('sand');f.controller.update({platform:'tank_heavy',components:{}});f.flush();
  assert.equal(f.materialPacks.length,1);assert(pack.binds.length>=3);assert.deepEqual(pack.disposals,[]);
  f.controller.dispose();assert.deepEqual(pack.disposals,[false]);
});

test('designer context loss abandons old material handles and restoration owns a fresh pack',()=>{
  const f=fixture({withMaterials:true});f.flush();f.gpu.clear();f.canvas.listeners.get('webglcontextlost')({preventDefault(){}});
  assert.deepEqual(f.materialPacks[0].disposals,[true]);f.canvas.listeners.get('webglcontextrestored')();f.flush();
  assert.equal(f.materialPacks.length,2);assert(f.materialPacks[1].binds.length>0);f.controller.dispose();assert.deepEqual(f.materialPacks[1].disposals,[false]);
});
test('repainting/name edits reuse geometry, a component edit replaces buffers, disposal releases all GPU resources and listeners',()=>{
  const f=fixture();f.flush();assert.equal(f.builds(),1);assert(f.draws()>=3);const allocated=f.gpu.size;
  for(let i=0;i<20;i++)f.controller.update({platform:'tank_standard',components:{},name:'Name '+i});
  assert.equal(f.builds(),1);assert.equal(f.gpu.size,allocated);
  f.controller.update({platform:'tank_standard',components:{mobility:'drive_mobile'},name:'Changed'});assert.equal(f.builds(),2);assert.equal(f.gpu.size,allocated);
  assert(f.canvas.width*f.canvas.height<=2250000+3000);f.controller.dispose();assert.equal(f.gpu.size,0);assert.equal(f.frames.size,0);assert.equal(f.canvas.listeners.size,0);assert.equal(f.doc.listeners.size,0);assert(f.observers.every(o=>o.closed));
  f.controller.dispose();f.controller.update({});assert.equal(f.gpu.size,0);
});
test('idle preview is demand-rendered and rotation suspends while hidden or outside the viewport',()=>{
  const f=fixture();f.flush();assert.equal(f.frames.size,0);f.controller.toggleRotation();f.flush();assert.equal(f.frames.size,1);
  f.doc.hidden=true;f.doc.listeners.get('visibilitychange')();assert.equal(f.frames.size,0);
  f.doc.hidden=false;f.doc.listeners.get('visibilitychange')();assert.equal(f.frames.size,1);
  f.observers[1].callback([{isIntersecting:false}]);assert.equal(f.frames.size,0);f.observers[1].callback([{isIntersecting:true}]);assert.equal(f.frames.size,1);f.controller.dispose();
});
test('WebGL loss pauses rendering and restores current geometry, rather than creating a second renderer',()=>{
  const f=fixture();f.flush();f.gpu.clear();let prevented=false;f.canvas.listeners.get('webglcontextlost')({preventDefault(){prevented=true;}});assert(prevented);assert.equal(f.frames.size,0);
  f.controller.update({platform:'tank_heavy',components:{}});f.canvas.listeners.get('webglcontextrestored')();f.flush();assert(f.draws()>=6);assert.equal(f.builds(),2);f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('partial context restoration releases GPU allocations while preserving the selected part and downloadable finish',()=>{
  for(const failure of ['allocation','upload','fragment shader']){
    const f=fixture();f.controller.selectPart('armament');f.controller.setFinish('sand');const exported=new Float32Array(f.controller.exportGlb());f.flush();
    f.gpu.clear();f.canvas.listeners.get('webglcontextlost')({preventDefault(){}});
    const allocate=f.gl.createBuffer,upload=f.gl.bufferData,compile=f.gl.getShaderParameter;let calls=0;
    if(failure==='allocation')f.gl.createBuffer=()=>++calls===2?null:allocate();
    if(failure==='upload')f.gl.bufferData=()=>{if(++calls===2)throw new Error('Synthetic upload failure');};
    if(failure==='fragment shader')f.gl.getShaderParameter=()=>++calls!==2;
    f.gl.getShaderInfoLog=()=>'';f.canvas.listeners.get('webglcontextrestored')();
    assert.equal(f.gpu.size,0,failure);assert.equal(f.frames.size,0);assert.deepEqual(f.controller.exportGlb(),exported);assert.equal(f.partSelect.value,'armament / test barrel');
    f.gl.createBuffer=allocate;f.gl.bufferData=upload;f.gl.getShaderParameter=compile;
    f.canvas.listeners.get('webglcontextrestored')();f.flush();assert(f.gpu.size>0);assert.deepEqual(f.controller.exportGlb(),exported);
    f.controller.dispose();assert.equal(f.gpu.size,0);
  }
});
test('unavailable graphics retain a clear fallback and a downloadable actual mesh',()=>{
  const f=fixture({noGraphics:true});assert.match(f.status.textContent,/3D rendering is unavailable/);assert.equal(f.gpu.size,0);assert.equal(f.frames.size,0);assert(f.controller.exportGlb() instanceof Float32Array);f.controller.dispose();
});

test('downloads follow the selected finish even when WebGL is unavailable',()=>{
  const f=fixture({noGraphics:true}),original=new Float32Array(f.controller.exportGlb());
  for(const finish of ['sand','winter','olive']){
    f.controller.setFinish(finish);
    assert.deepEqual(f.controller.exportGlb(),viewer.finishColors(original,finish));
    assert.equal(f.exportButton.disabled,false);
  }
  f.controller.dispose();
});

test('changing finish during context loss updates downloads and the restored GPU colors',()=>{
  const f=fixture(),original=new Float32Array(f.controller.exportGlb());f.flush();
  f.gpu.clear();f.canvas.listeners.get('webglcontextlost')({preventDefault(){}});
  const before=f.uploads.length;f.controller.setFinish('winter');
  const expected=viewer.finishColors(original,'winter');
  assert.deepEqual(f.controller.exportGlb(),expected);assert.equal(f.uploads.length,before);
  f.canvas.listeners.get('webglcontextrestored')();f.flush();
  assert.deepEqual(f.uploads.at(-1),expected);assert.deepEqual(f.controller.exportGlb(),expected);
  f.controller.dispose();assert.equal(f.gpu.size,0);
});

test('failed rebuild clears stale imagery and exports, releases model buffers, and allows the same specification to retry',()=>{
  const f=fixture();f.flush();const build=f.ctx.EquipmentMesh.build,allocated=f.gpu.size,clears=f.clears(),draws=f.draws();
  f.ctx.EquipmentExport.glb=(mesh,name)=>({mesh,name});
  f.ctx.EquipmentMesh.build=()=>{throw new Error('Synthetic geometry failure');};
  const next={platform:'tank_heavy',components:{protection:'protection_heavy'},name:'Heavy revision'};
  f.controller.update(next);
  assert.equal(f.controller.exportGlb(),null);assert.equal(f.exportButton.disabled,true);
  assert.equal(f.gpu.size,allocated-3);assert(f.clears()>clears);assert.equal(f.frames.size,0);
  f.flush();assert.equal(f.draws(),draws);assert.match(f.status.textContent,/could not be displayed/);
  f.ctx.EquipmentMesh.build=build;f.controller.update(next);f.flush();
  assert.equal(f.builds(),2);assert.equal(f.gpu.size,allocated);assert.equal(f.exportButton.disabled,false);
  assert.equal(f.controller.exportGlb().name,'Heavy revision');assert.match(f.status.textContent,/3D model ready/);
  f.controller.dispose();assert.equal(f.gpu.size,0);
});

test('a partial GPU upload failure releases newly allocated model buffers and can recover',()=>{
  const f=fixture();f.flush();const allocate=f.gl.createBuffer,allocated=f.gpu.size,clears=f.clears();let requests=0;
  f.gl.createBuffer=()=>++requests===2?null:allocate();
  const next={platform:'tank_heavy',components:{},name:'Heavy revision'};
  f.controller.update(next);
  assert.equal(f.controller.exportGlb(),null);assert.equal(f.exportButton.disabled,true);
  assert.equal(f.gpu.size,allocated-3);assert(f.clears()>clears);assert.equal(f.frames.size,0);
  f.gl.createBuffer=allocate;f.controller.update(next);f.flush();
  assert.equal(f.gpu.size,allocated);assert.equal(f.exportButton.disabled,false);assert(f.controller.exportGlb());
  f.controller.dispose();assert.equal(f.gpu.size,0);
});

function targetPoint(){
  const camera=viewer.frame(bounds,1.8,...viewer.views.hero),p=[.25,.25,0,1],clip=[0,0,0,0];
  for(let r=0;r<4;r++)for(let c=0;c<4;c++)clip[r]+=camera.matrix[c*4+r]*p[c];
  return {clientX:100+(clip[0]/clip[3]+1)*450,clientY:70+(1-clip[1]/clip[3])*250};
}
function pointer(f,type,coordinates,id=1){f.canvas.listeners.get(type)({pointerId:id,pointerType:'mouse',button:0,preventDefault(){},...coordinates});}
test('ray picking returns the nearest actual triangle and respects empty space and occlusion',()=>{
  const mesh={positions:new Float32Array([-1,-1,0,1,-1,0,0,1,0,-1,-1,-2,1,-1,-2,0,1,-2]),parts:[{name:'Front',slot:'protection',first:0,count:3},{name:'Rear',slot:'mobility',first:3,count:3}]};
  assert.equal(viewer.raycast(mesh,[0,0,3],[0,0,-1]).part.name,'Front');
  assert.equal(viewer.raycast(mesh,[0,0,-3],[0,0,1]).part.name,'Rear');
  assert.equal(viewer.raycast(mesh,[2,0,3],[0,0,-1]),null);
  assert.equal(viewer.raycast(mesh,[0,0,3],[1,0,0]),null);
  mesh.parts.shift();assert.equal(viewer.raycast(mesh,[0,0,3],[0,0,-1]).part,null,'Unselectable front geometry still occludes rear parts');
});
test('a canvas click picks visible geometry, bubbles its slot, and preserves camera and exported finish',()=>{
  const f=fixture();f.flush();f.controller.setFinish('sand');const before=new Float32Array(f.controller.exportGlb()),point=targetPoint(),uploads=f.uploads.length;
  pointer(f,'pointerdown',point);pointer(f,'pointerup',point);f.flush();
  const event=f.canvas.dispatched.at(-1);assert.equal(event.type,'equipment-part-select');assert.equal(event.bubbles,true);
  assert.deepEqual({...event.detail},{slot:'armament',part:'armament / test barrel',label:'Test barrel'});
  assert.equal(f.partSelect.value,'armament / test barrel');assert.equal(f.uploads.length,uploads,'Highlight is a shader effect');
  assert.deepEqual(f.controller.exportGlb(),before);assert.equal(f.controller.pickAt(point.clientX,point.clientY).slot,'armament','Camera did not move');
  f.controller.dispose();assert.equal(f.partSelect.listeners.size,0);
});
test('orbit drags, cancelled gestures and two-pointer gestures do not select parts',()=>{
  for(const gesture of ['drag','cancel','pinch','context loss']){
    const f=fixture(),point=targetPoint();f.flush();pointer(f,'pointerdown',point);
    if(gesture==='drag')pointer(f,'pointermove',{clientX:point.clientX+40,clientY:point.clientY});
    if(gesture==='cancel')pointer(f,'pointercancel',point);
    if(gesture==='pinch'){pointer(f,'pointerdown',{clientX:point.clientX+20,clientY:point.clientY},2);pointer(f,'pointerup',{clientX:point.clientX+20,clientY:point.clientY},2);}
    if(gesture==='context loss'){f.gpu.clear();f.canvas.listeners.get('webglcontextlost')({preventDefault(){}});f.canvas.listeners.get('webglcontextrestored')();}
    pointer(f,'pointerup',point);assert.equal(f.canvas.dispatched.length,0,gesture);f.controller.dispose();
  }
});
test('all specialist hulls, tall radar and extended barrels fit the unchanged camera views',()=>{
  const {build}=require('../../spheres-web/ui/equipment-mesh.js');
  for(const platform of ['ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense']){
    const mesh=build({platform,components:{recon_package:'ground_recon_mast',radar:'ground_radar_tracking',armament:platform==='ground_artillery'?'ground_howitzer_155':undefined}});
    for(const aspect of [.55,1,1.8])for(const [yaw,pitch] of Object.values(viewer.views)){
      const camera=viewer.frame(mesh.bounds,aspect,yaw,pitch),m=camera.matrix;
      for(const x of [mesh.bounds.min[0],mesh.bounds.max[0]])for(const y of [mesh.bounds.min[1],mesh.bounds.max[1]])for(const z of [mesh.bounds.min[2],mesh.bounds.max[2]]){
        const p=[x,y,z,1],clip=[0,0,0,0];for(let r=0;r<4;r++)for(let c=0;c<4;c++)clip[r]+=m[c*4+r]*p[c];
        assert(clip[3]>0);for(let i=0;i<3;i++)assert(Math.abs(clip[i]/clip[3])<1,`${platform} fits ${aspect}`);
      }
    }
  }
});
test('projected visible mission installations and weapons ray-pick their true specification slots',()=>{
  const {build}=require('../../spheres-web/ui/equipment-mesh.js');
  for(const [platform,mission] of [['ground_ifv','troop_compartment'],['ground_apc','troop_compartment'],['ground_recon','recon_package'],['ground_artillery','artillery_loader'],['ground_air_defense','radar']]){
    const mesh=build({platform});
    for(const slot of ['armament',mission]){
      const part=mesh.parts.find(p=>p.slot===slot);assert(part);let found=false;
      for(const [yaw,pitch] of [viewer.views.hero,viewer.views.rear,viewer.views.top]){
        const camera=viewer.frame(mesh.bounds,1.8,yaw,pitch),m=camera.matrix;
        const step=Math.max(3,Math.floor(part.count/90/3)*3);
        for(let vertex=part.first;vertex<part.first+part.count;vertex+=step){
          const p=[0,0,0,1],clip=[0,0,0,0];for(let axis=0;axis<3;axis++)p[axis]=(mesh.positions[vertex*3+axis]+mesh.positions[(vertex+1)*3+axis]+mesh.positions[(vertex+2)*3+axis])/3;
          for(let r=0;r<4;r++)for(let c=0;c<4;c++)clip[r]+=m[c*4+r]*p[c];
          const ray=viewer.rayAt(camera,clip[0]/clip[3],clip[1]/clip[3],1.8),hit=viewer.raycast(mesh,ray.origin,ray.direction);
          if(hit?.part?.slot===slot){found=true;break;}
        }
        if(found)break;
      }
      assert(found,`${platform} exposes pickable ${slot} geometry`);
    }
  }
});
test('native keyboard part selection works without graphics and selection survives context restoration',()=>{
  const f=fixture({noGraphics:true});assert.equal(f.partSelect.children[1].textContent,'Test barrel');
  f.partSelect.value='armament / test barrel';f.partSelect.listeners.get('change')();assert.equal(f.host.dispatched[0].detail.slot,'armament');f.controller.dispose();
  const live=fixture();live.controller.selectPart('armament');live.flush();const before=new Float32Array(live.controller.exportGlb());
  live.gpu.clear();live.canvas.listeners.get('webglcontextlost')({preventDefault(){}});assert.equal(live.controller.pickAt(...Object.values(targetPoint())),null);
  live.canvas.listeners.get('webglcontextrestored')();live.flush();assert.equal(live.partSelect.value,'armament / test barrel');assert.deepEqual(live.controller.exportGlb(),before);
  live.controller.dispose();assert.equal(live.canvas.listeners.size,0);assert.equal(live.partSelect.listeners.size,0);
});

test('aircraft fit all camera views and expose each specification through visible triangle picking',()=>{
  const {build}=require('../../spheres-web/ui/equipment-mesh.js');
  for(const platform of ['air_light_attack','air_tactical_strike']){
    const mesh=build({platform});
    for(const aspect of [.55,1,1.8])for(const [yaw,pitch] of Object.values(viewer.views)){
      const camera=viewer.frame(mesh.bounds,aspect,yaw,pitch),m=camera.matrix;
      for(const x of [mesh.bounds.min[0],mesh.bounds.max[0]])for(const y of [0,mesh.bounds.max[1]])for(const z of [mesh.bounds.min[2],mesh.bounds.max[2]]){
        const p=[x,y,z,1],clip=[0,0,0,0];for(let r=0;r<4;r++)for(let k=0;k<4;k++)clip[r]+=m[k*4+r]*p[k];
        assert(clip[3]>0&&clip.slice(0,3).every(v=>Math.abs(v/clip[3])<1),platform+' fits '+aspect);
      }
    }
    for(const slot of Object.keys(mesh.specification.components)){
      let found=false;
      for(const [yaw,pitch] of Object.values(viewer.views)){
        const camera=viewer.frame(mesh.bounds,1.8,yaw,pitch),m=camera.matrix;
        for(const part of mesh.parts.filter(p=>p.slot===slot)){
          const step=Math.max(3,Math.floor(part.count/45/3)*3);
          for(let v=part.first;v<part.first+part.count;v+=step){
            const p=[0,0,0,1],clip=[0,0,0,0];for(let a=0;a<3;a++)p[a]=(mesh.positions[v*3+a]+mesh.positions[(v+1)*3+a]+mesh.positions[(v+2)*3+a])/3;
            for(let r=0;r<4;r++)for(let c=0;c<4;c++)clip[r]+=m[c*4+r]*p[c];
            const ray=viewer.rayAt(camera,clip[0]/clip[3],clip[1]/clip[3],1.8);
            if(viewer.raycast(mesh,ray.origin,ray.direction)?.part?.slot===slot){found=true;break;}
          }
          if(found)break;
        }
        if(found)break;
      }
      assert(found,`${platform} exposes visible ${slot} geometry`);
    }
  }
});
