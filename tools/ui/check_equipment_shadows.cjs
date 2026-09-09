/* Resource and render-pass tests. Real shader compilation/visual checks use the
   military-inspection browser bench; these mocks deliberately validate the GL
   object ownership, framebuffer state and cached draw behavior. */
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const source=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/equipment-model.js'),'utf8');
const surface=require('../../spheres-web/ui/military-surface.js');
const viewer=require('../../spheres-web/ui/equipment-model.js');
const bounds={min:[-2,0,-4],max:[2,4,6]};

function fixture(initialFailure=null){
  let next=1,failure=initialFailure,framebuffer=null,texture=null,renderbuffer=null,boundBuffer=null,currentProgram=null,mode=0,viewport=[],clearColor=[],lost=false,builds=0;
  const gpu=new Map(),created=[],deleted=[],frames=new Map(),draws=[],clears=[],sources=[],contextCalls=[],enabledAttributes=new Set(),pointers=new Map(),uniformValues=new Map();
  function create(kind){if(failure===kind+' allocation')return null;const id={id:next++,kind};gpu.set(id,id);created.push(id);return id;}
  function remove(id){assert(!lost,'No GL deletion during context loss');assert(gpu.has(id),'Delete only a live owned resource');gpu.delete(id);deleted.push(id);}
  const gl={ARRAY_BUFFER:1,STATIC_DRAW:2,VERTEX_SHADER:3,FRAGMENT_SHADER:4,COMPILE_STATUS:5,LINK_STATUS:6,DEPTH_TEST:7,LEQUAL:8,FLOAT:9,TRIANGLES:10,COLOR_BUFFER_BIT:16,DEPTH_BUFFER_BIT:32,
    FRAMEBUFFER:40,FRAMEBUFFER_COMPLETE:41,RENDERBUFFER:42,TEXTURE_2D:43,RGBA:44,UNSIGNED_BYTE:45,DEPTH_COMPONENT16:46,COLOR_ATTACHMENT0:47,DEPTH_ATTACHMENT:48,TEXTURE0:49,
    TEXTURE_MIN_FILTER:50,TEXTURE_MAG_FILTER:51,TEXTURE_WRAP_S:52,TEXTURE_WRAP_T:53,NEAREST:54,CLAMP_TO_EDGE:55,MAX_TEXTURE_SIZE:56,MAX_RENDERBUFFER_SIZE:57,MAX_TEXTURE_IMAGE_UNITS:58,HIGH_FLOAT:59,DITHER:60,NO_ERROR:0,
    createShader:type=>{const s=create('shader');if(s)s.type=type;return s;},deleteShader:remove,
    shaderSource(s,text){s.source=text;sources.push(text);},compileShader(){},getShaderInfoLog:()=>'',getProgramInfoLog:()=>'',
    getShaderParameter(s){
      if(failure==='shadow vertex compilation'&&s.source.includes('uniform mat4 uLightVP')&&!s.source.includes('aNormal'))return false;
      if(failure==='shadow fragment compilation'&&s.source.includes('gl_FragCoord.z'))return false;
      if(failure==='receiver compilation'&&s.source.includes('texture2D(uShadowMap'))return false;
      if(failure==='all fragment compilation'&&s.type===gl.FRAGMENT_SHADER)return false;
      return true;
    },
    createProgram:()=>create('program'),deleteProgram:remove,
    attachShader(p,s){(p.shaders||(p.shaders=[])).push(s);},linkProgram(){},
    getProgramParameter(p){const depth=p.shaders.some(s=>s.source.includes('gl_FragCoord.z'));return !(failure==='shadow link'&&depth)&&!(failure==='receiver link'&&p.shaders.some(s=>s.source.includes('texture2D(uShadowMap')));},
    getAttribLocation:(_,name)=>({aPosition:0,aNormal:1,aColor:2}[name]??-1),getUniformLocation:(p,name)=>({program:p,name}),
    createBuffer:()=>create('buffer'),deleteBuffer:remove,bindBuffer:(_,value)=>{boundBuffer=value;},bufferData:(_,data)=>{assert(gpu.has(boundBuffer));boundBuffer.data=data;},
    createTexture:()=>create('texture'),deleteTexture:remove,bindTexture:(_,value)=>{texture=value;},texParameteri(){},
    texImage2D(...args){if(failure==='texture upload')throw new Error('Synthetic texture allocation failure');assert(gpu.has(texture));texture.width=args[3];texture.height=args[4];},
    createRenderbuffer:()=>create('renderbuffer'),deleteRenderbuffer:remove,bindRenderbuffer:(_,value)=>{renderbuffer=value;},
    renderbufferStorage(_,format,w,h){if(failure==='depth storage')throw new Error('Synthetic depth allocation failure');assert(gpu.has(renderbuffer));Object.assign(renderbuffer,{format,w,h});},
    createFramebuffer:()=>create('framebuffer'),deleteFramebuffer:remove,bindFramebuffer:(_,value)=>{framebuffer=value;},
    framebufferTexture2D:(_a,_b,_c,value)=>{framebuffer.texture=value;},framebufferRenderbuffer:(_a,_b,_c,value)=>{framebuffer.depth=value;},
    checkFramebufferStatus:()=>failure==='incomplete framebuffer'?0:gl.FRAMEBUFFER_COMPLETE,
    getParameter(name){if(failure==='capability query')throw new Error('Synthetic unsupported capability');if(name===gl.MAX_TEXTURE_IMAGE_UNITS)return 8;return failure==='small device'?256:failure==='512 device'?512:4096;},
    getShaderPrecisionFormat:()=>({precision:failure==='no high precision'?0:23}),
    getError:()=>failure==='shadow GL error'?1282:0,
    activeTexture(){},disable(){},enable(){},depthFunc(){},
    enableVertexAttribArray:index=>enabledAttributes.add(index),disableVertexAttribArray:index=>enabledAttributes.delete(index),
    vertexAttribPointer:index=>pointers.set(index,boundBuffer),viewport:(...v)=>{viewport=v;},clearColor:(...v)=>{clearColor=v;},
    clear:()=>clears.push({framebuffer,color:[...clearColor]}),useProgram:p=>{assert(gpu.has(p));currentProgram=p;},
    uniformMatrix4fv:(u,_transpose,value)=>uniformValues.set(u.name,Array.from(value)),uniform3fv:(u,value)=>uniformValues.set(u.name,Array.from(value)),
    uniform1f(u,value){uniformValues.set(u.name,value);if(u.name==='uMode')mode=value;},uniform1i:(u,value)=>uniformValues.set(u.name,value),
    drawArrays(_primitive,first,count){
      assert(!lost);for(const index of enabledAttributes)assert(gpu.has(pointers.get(index)),'No enabled attribute points to a deleted buffer');
      if(framebuffer){assert(gpu.has(framebuffer));assert(gpu.has(framebuffer.texture));assert(gpu.has(framebuffer.depth));assert.equal(texture,null,'Depth target is not bound for sampling while rendering it');if(failure==='shadow draw')throw new Error('Synthetic depth draw failure');}
      else if(currentProgram.shaders.some(s=>s.source.includes('texture2D(uShadowMap')))assert(gpu.has(texture),'A shadow receiver always samples a live complete texture');
      draws.push({framebuffer,mode,first,count,program:currentProgram,position:pointers.get(0),viewport:[...viewport],uniforms:new Map(uniformValues)});
    }
  };
  if(failure==='missing methods')delete gl.createFramebuffer;
  function node(){return {listeners:new Map(),style:{},children:[],isConnected:true,dataset:{},setAttribute(){},appendChild(c){this.children.push(c);c.parentNode=this;},remove(){this.isConnected=false;},
    addEventListener(e,f){this.listeners.set(e,f);},removeEventListener(e,f){if(this.listeners.get(e)===f)this.listeners.delete(e);},getBoundingClientRect:()=>({left:0,top:0,width:800,height:500})};}
  const doc=node();doc.hidden=false;doc.body=node();const canvas=node();canvas.getContext=type=>{contextCalls.push(type);return gl;};doc.createElement=tag=>tag==='canvas'?canvas:node();
  const host=node(),slot=node(),status=node();host.ownerDocument=doc;host.querySelector=s=>s==='[data-model-canvas]'?slot:s==='[data-model-status]'?status:null;host.querySelectorAll=()=>[];
  const mesh={positions:new Float32Array([0,0,0,1,0,0,0,1,0]),normals:new Float32Array([0,0,1,0,0,1,0,0,1]),colors:new Float32Array([.25,.32,.20,.25,.32,.20,.25,.32,.20]),bounds,parts:[{name:'barrel',slot:'armament',first:0,count:3}],triangleCount:1};
  class Observer{observe(){}disconnect(){}}
  const ctx={module:{exports:{}},Float32Array,Math,Number,Map,Set,Array,Object,JSON,document:doc,devicePixelRatio:1,ResizeObserver:Observer,IntersectionObserver:Observer,MilitarySurface:surface,
    EquipmentMesh:{build(){builds++;return mesh;}},EquipmentExport:{glb:m=>m.colors},requestAnimationFrame:f=>{const id=next++;frames.set(id,f);return id;},cancelAnimationFrame:id=>frames.delete(id)};
  vm.runInNewContext(source,ctx);const controller=ctx.module.exports.mount(host,{platform:'tank_standard',components:{}});
  function flush(){const batch=[...frames];frames.clear();for(const [,fn] of batch)fn(100);}
  function lose(){gpu.clear();lost=true;canvas.listeners.get('webglcontextlost')({preventDefault(){}});}
  function restore(){lost=false;enabledAttributes.clear();pointers.clear();framebuffer=texture=renderbuffer=null;canvas.listeners.get('webglcontextrestored')();}
  return {controller,gl,ctx,canvas,doc,gpu,created,deleted,frames,draws,clears,sources,contextCalls,flush,lose,restore,status,setFailure:value=>{failure=value;},builds:()=>builds,
    count:kind=>[...gpu.values()].filter(value=>value.kind===kind).length,depthDraws:()=>draws.filter(d=>d.framebuffer),mainDraws:()=>draws.filter(d=>!d.framebuffer)};
}

test('one RGBA depth target renders actual mesh once and replaces projected floor shadow',()=>{
  const f=fixture();f.flush();assert.equal(f.count('framebuffer'),1);assert.equal(f.count('texture'),1);assert.equal(f.count('renderbuffer'),1);assert.equal(f.count('program'),2);assert.equal(f.count('buffer'),6);assert.equal(f.count('shader'),0);
  const depth=f.depthDraws();assert.equal(depth.length,1);assert.equal(depth[0].count,3);assert.equal(depth[0].position.data.length,9);assert.deepEqual(depth[0].viewport,[0,0,1024,1024]);
  assert.deepEqual(f.mainDraws().map(d=>d.mode),[1,0]);assert.deepEqual(f.clears.find(c=>c.framebuffer).color,[1,1,1,1]);assert.deepEqual(f.clears.at(-1).color,[.063,.088,.105,1]);
  assert.equal(f.mainDraws()[0].uniforms.get('uShadowTexel'),1/1024);assert(f.mainDraws()[0].uniforms.get('uShadowNormalBias')>0);assert(f.mainDraws()[0].uniforms.get('uShadowDepthBias')>0);
  assert.deepEqual(f.contextCalls,['webgl']);assert.equal(f.frames.size,0);f.controller.dispose();assert.equal(f.gpu.size,0);
});

test('camera, paint, name and selection reuse light depth; geometry changes invalidate it without allocating another target',()=>{
  const f=fixture();f.flush();const target=f.depthDraws()[0].framebuffer,allocations=f.created.filter(r=>r.kind!=='shader').length;
  for(const change of [()=>f.controller.rotate(.2,.1),()=>f.controller.zoom(-1),()=>f.controller.view('side'),()=>f.controller.resize(),()=>f.controller.setFinish('sand'),()=>f.controller.selectPart('armament'),()=>f.controller.update({platform:'tank_standard',components:{},name:'Renamed'})]){change();f.flush();assert.equal(f.depthDraws().length,1);}
  assert.equal(f.builds(),1);assert.equal(f.created.filter(r=>r.kind!=='shader').length,allocations);
  for(let i=0;i<8;i++){f.controller.update({platform:'tank_standard',components:{armament:'barrel_'+i}});f.flush();assert.equal(f.depthDraws().length,i+2);assert.equal(f.depthDraws().at(-1).framebuffer,target);assert.equal(f.count('buffer'),6);assert.equal(f.count('framebuffer'),1);}
  f.controller.dispose();assert.equal(f.gpu.size,0);
});

test('512 targets work on smaller devices while missing capabilities use the unchanged projected-shadow path',()=>{
  const small=fixture('512 device');small.flush();assert.deepEqual(small.depthDraws()[0].viewport,[0,0,512,512]);assert.equal(small.mainDraws()[0].uniforms.get('uShadowTexel'),1/512);small.controller.dispose();assert.equal(small.gpu.size,0);
  for(const failure of ['small device','missing methods','no high precision','capability query']){
    const f=fixture(failure);f.flush();assert.equal(f.depthDraws().length,0,failure);assert.deepEqual(f.mainDraws().map(d=>d.mode),[1,2,0],failure);assert.equal(f.gpu.size,7,failure);f.controller.dispose();assert.equal(f.gpu.size,0,failure);
  }
});

test('every partial shadow allocation, shader and link failure releases resources and leaves a functioning fallback',()=>{
  for(const failure of ['texture allocation','texture upload','renderbuffer allocation','depth storage','framebuffer allocation','incomplete framebuffer','shadow vertex compilation','shadow fragment compilation','shadow link','receiver compilation','receiver link']){
    const f=fixture(failure);f.flush();assert.equal(f.depthDraws().length,0,failure);assert.deepEqual(f.mainDraws().map(d=>d.mode),[1,2,0],failure);assert.equal(f.gpu.size,7,failure);assert.equal(f.count('shader'),0,failure);assert.equal(f.count('framebuffer'),0,failure);assert.equal(f.count('texture'),0,failure);assert.equal(f.count('renderbuffer'),0,failure);
    f.controller.dispose();assert.equal(f.gpu.size,0,failure);
  }
});

test('a failing optional depth draw returns to the default framebuffer and replaces its sampling shader',()=>{
  for(const failure of ['shadow draw','shadow GL error']){
    const f=fixture(failure);f.flush();assert.deepEqual(f.mainDraws().map(d=>d.mode),[1,2,0]);assert.equal(f.gpu.size,7);assert.equal(f.count('texture'),0);assert.equal(f.count('program'),1);
    const depthCount=f.depthDraws().length;f.controller.rotate(.1,0);f.flush();assert.equal(f.depthDraws().length,depthCount);assert.equal(f.mainDraws().length,6);f.controller.dispose();assert.equal(f.gpu.size,0);
  }
});

test('context loss discards shadow handles without GL calls and restores the latest geometry on the same canvas',()=>{
  const f=fixture();f.flush();const before=f.deleted.length,target=f.depthDraws()[0].framebuffer;f.lose();assert.equal(f.deleted.length,before);assert.equal(f.frames.size,0);
  f.controller.update({platform:'tank_heavy',components:{armament:'long'}});f.controller.setFinish('winter');f.controller.selectPart('armament');assert.equal(f.gpu.size,0);
  f.restore();f.flush();assert.equal(f.depthDraws().length,2);assert.notEqual(f.depthDraws()[1].framebuffer,target);assert.equal(f.count('framebuffer'),1);assert.equal(f.count('program'),2);assert.equal(f.builds(),2);assert.deepEqual(f.contextCalls,['webgl','webgl']);
  f.controller.dispose();assert.equal(f.gpu.size,0);assert.equal(f.canvas.listeners.size,0);assert.equal(f.doc.listeners.size,0);
  const lost=fixture();lost.flush();lost.lose();lost.controller.dispose();assert.equal(lost.gpu.size,0);assert.equal(lost.frames.size,0);
});

test('failed context restoration cleans both renderer and shadow resources before a successful retry',()=>{
  const f=fixture();f.flush();f.lose();f.setFailure('all fragment compilation');f.restore();assert.equal(f.gpu.size,0);assert.equal(f.frames.size,0);assert.match(f.status.textContent,/unavailable/);
  f.setFailure(null);f.restore();f.flush();assert.equal(f.count('framebuffer'),1);assert.equal(f.count('program'),2);f.controller.dispose();assert.equal(f.gpu.size,0);
});

test('mesh corners and their ground projections fit a stable light volume across vehicle sizes',()=>{
  const light=[-.55,.85,.65];
  for(const b of [bounds,{min:[-35,0,-45],max:[35,17,52]},{min:[-1,.05,-1],max:[1,.7,1]},{min:[20,4,-19],max:[30,20,-7]}]){
    const f=viewer.shadowFrame(b,1024);assert([...f.matrix,f.normalBias,f.depthBias,f.floorExtent].every(Number.isFinite));assert(f.normalBias>0&&f.depthBias>0);
    for(const x of [b.min[0],b.max[0]])for(const y of [b.min[1],b.max[1]])for(const z of [b.min[2],b.max[2]])for(const p of [[x,y,z,1],[x-light[0]*y/light[1],0,z-light[2]*y/light[1],1]]){
      const clip=[0,0,0,0];for(let r=0;r<4;r++)for(let c=0;c<4;c++)clip[r]+=f.matrix[c*4+r]*p[c];
      for(let i=0;i<3;i++)assert(Math.abs(clip[i]/clip[3])<1,'The shadow volume contains the complete mesh and receiver footprint');assert(Math.abs(p[0])<f.floorExtent&&Math.abs(p[2])<f.floorExtent);
    }
    assert.deepEqual(f,viewer.shadowFrame(b,1024));assert(viewer.shadowFrame(b,512).normalBias>f.normalBias);
  }
});

test('packed depth preserves ordering and receiver shaders share the correct precision and material visibility',()=>{
  // RGBA8 stores multiples of 1/255, whereas packing uses powers of 256.
  // The reciprocal scale pair prevents discontinuities at byte boundaries.
  const pack=d=>{const values=[16777216,65536,256,1].map(scale=>(Math.min(d,.9999999)*scale)%1),old=[...values];for(let i=1;i<4;i++)values[i]-=old[i-1]/256;return values.map(v=>Math.min(255,Math.round(v*256))/255);};
  const unpack=rgba=>rgba.reduce((n,v,i)=>n+v*[1/16777216,1/65536,1/256,1][i],0)*255/256;
  let previous=-1;for(let i=0;i<=1000;i++){const depth=i/1000,roundtrip=unpack(pack(depth));assert(roundtrip>=previous);assert(Math.abs(roundtrip-depth)<.00000011);previous=roundtrip;}
  assert(unpack([1,1,1,1])>.9999999);
  const f=fixture();f.flush();const receiver=f.sources.find(s=>s.includes('texture2D(uShadowMap'));assert(receiver.includes('militaryLighting(vColor,vNormal,vPosition,uEye-vPosition,visibility,uHeight)'));assert(receiver.includes('for(int y=-1;y<=1;y++)for(int x=-1;x<=1;x++)'));
  for(const s of f.sources.filter(s=>s.includes('uniform mediump float uMode')))assert(s.includes('varying highp vec3 vPosition'));
  f.controller.dispose();assert.equal(f.gpu.size,0);
});
