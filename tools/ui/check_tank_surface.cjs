/* Tank paint/material, bounded contact bake, and isolated viewer integration. */
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const Surface=require('../../spheres-web/ui/tank-surface.js'),Mesh=require('../../spheres-web/ui/equipment-mesh.js'),Export=require('../../spheres-web/ui/equipment-export.js');
const digest=array=>crypto.createHash('sha256').update(Buffer.from(array.buffer,array.byteOffset,array.byteLength)).digest('hex');
function triangles(items){const positions=[],normals=[],colors=[],materialClasses=[];for(const [points,normal,color,kind] of items)for(const point of points){positions.push(...point);normals.push(...normal);colors.push(...color);materialClasses.push(kind);}
  const min=[0,1,2].map(k=>Math.min(...positions.filter((_,i)=>i%3===k))),max=[0,1,2].map(k=>Math.max(...positions.filter((_,i)=>i%3===k)));
  return {positions:new Float32Array(positions),normals:new Float32Array(normals),colors:new Float32Array(colors),materialClasses:new Uint8Array(materialClasses),parts:[{name:'Hull',label:'Hull',slot:'protection',first:0,count:positions.length/3}],bounds:{min,max},specification:{platform:'tank_standard'},triangleCount:positions.length/9};}
function materials(){return triangles([[.35,.40,.27],[.20,.22,.20],[.13,.15,.14],[.055,.068,.060],[.13,.33,.34],[.30,.31,.21],[.25,.27,.24],[.62,.37,.16]].map((color,kind)=>[[[kind*2,0,0],[kind*2+1,0,0],[kind*2,0,1]],[0,1,0],color,kind]));}

test('every tank level carries authored classes including steel, rubber, glazing and canvas without unknown tokens',()=>{
  for(const platform of ['tank_standard','tank_heavy','tank_light','tank_destroyer'])for(const lod of [0,1,2]){const mesh=Mesh.build({platform,lod});assert(Surface.supports(mesh));assert.equal(mesh.materialClasses.length,mesh.positions.length/3);assert(!mesh.materialClasses.includes(255),platform+' '+lod);if(lod===0)for(const kind of [0,1,2,3,4,5])assert(mesh.materialClasses.includes(kind),platform+' lacks '+kind);}
});
test('sand, winter and woodland repaint only authored paint even when steel and tracks are green-biased',()=>{
  const mesh=materials(),olive=Surface.bake(mesh,{finish:'olive',wear:'factory'});
  for(const finish of ['sand','winter','woodland']){const colors=Surface.bake(mesh,{finish,wear:'factory'});assert.notDeepEqual(colors.slice(0,9),olive.slice(0,9));assert.deepEqual(colors.slice(9),olive.slice(9),finish+' changed a nonpaint material');}
});
test('painted bevels stay close to the plate tone and sand cannot become pale cream',()=>{
  const mesh=triangles([[.22,.27,.18],[.35,.40,.27],[.40,.44,.31]].map((c,i)=>[[[i*2,0,0],[i*2+1,0,0],[i*2,0,1]],[0,1,0],c,0]));
  const colors=Surface.bake(mesh,{finish:'sand',wear:'factory',baseOnly:true});assert(Math.max(...colors)<.44);assert(colors[18]/colors[0]<1.07,'bright lid edges must not receive old 1.39x recolour boost');
});
test('service and field deposits affect running gear and paint but retain clear optics and lamps',()=>{
  const mesh=materials(),factory=Surface.bake(mesh,{wear:'factory'}),service=Surface.bake(mesh,{wear:'service'}),field=Surface.bake(mesh,{wear:'field'});
  for(const kind of [0,1,2,3,5,6]){assert.notDeepEqual(factory.slice(kind*9,kind*9+9),service.slice(kind*9,kind*9+9));assert.notDeepEqual(service.slice(kind*9,kind*9+9),field.slice(kind*9,kind*9+9));}
  for(const kind of [4,7])assert.deepEqual(factory.slice(kind*9,kind*9+9),field.slice(kind*9,kind*9+9));
});
test('authoritative roughness and metalness remain separate from finish colors',()=>{
  const mesh=materials(),data=Surface.prepare(mesh);assert.equal(data.parameters.length,mesh.positions.length/3*4);
  for(const kind of [0,3,4,5,7])assert.equal(data.parameters[kind*12+1],0);
  assert(data.parameters[0]>.8);assert(data.parameters[12+1]>.85);assert(data.parameters[24+1]>.8);assert(data.parameters[4*12]<.2);assert(data.parameters[3*12]>.9);
  for(let kind=0;kind<8;kind++)assert.equal(data.parameters[kind*12+2],kind);
});
test('contact bake detects neighboring faces instead of applying a world-height shadow',()=>{
  const floor=[[[0,0,0],[1,0,0],[0,0,1]],[0,1,0],[.3,.35,.2],0];
  const wall1=[[[0,0,.22],[1,0,.22],[0,1,.22]],[0,0,-1],[.3,.35,.2],0],wall2=[[[1,0,.22],[1,1,.22],[0,1,.22]],[0,0,-1],[.3,.35,.2],0];
  const flat=triangles([floor]),corner=triangles([floor,wall1,wall2]);
  const flatAO=Surface.prepare(flat).ao,cornerAO=Surface.prepare(corner).ao;assert.equal(flatAO[0],1);assert(cornerAO[0]<flatAO[0],String(cornerAO));
  const raised=triangles([floor,wall1,wall2].map(([points,n,c,k])=>[points.map(p=>[p[0],p[1]+4,p[2]]),n,c,k]));assert.deepEqual(Surface.prepare(raised).ao,cornerAO);
});
test('baking is deterministic, cached per geometry, and cannot mutate pick/export buffers',()=>{
  const mesh=Mesh.build({platform:'tank_heavy',lod:1}),keys=['positions','normals','colors','materialClasses'],before=keys.map(k=>digest(mesh[k])),parts=JSON.stringify(mesh.parts);
  const a=Surface.bake(mesh,{finish:'woodland',wear:'field'}),b=Surface.bake(mesh,{finish:'woodland',wear:'field'});assert.deepEqual(a,b);assert.equal(Surface.prepare(mesh),Surface.prepare(mesh));
  assert.deepEqual(keys.map(k=>digest(mesh[k])),before);assert.equal(JSON.stringify(mesh.parts),parts);assert(a.every(v=>Number.isFinite(v)&&v>=0&&v<=1));
});
test('inspection contact bake has explicit memory and raster work limits',()=>{
  const mesh=Mesh.build({platform:'tank_standard'}),data=Surface.prepare(mesh);assert(data.stats.voxelCells<2000000);assert(data.stats.rasterSamples<=2400000);assert.equal(data.stats.vertices,mesh.positions.length/3);assert(data.stats.uniqueSamples<=data.stats.vertices);assert(data.ao.every(v=>v>=.63&&v<=1));
});
test('missing or malformed metadata never selects the armour path for another equipment family',()=>{
  const mesh=materials();for(const platform of ['air_light_attack','ground_unreleased','nav_blue','__proto__',undefined])assert.equal(Surface.supports(mesh,platform),platform===undefined);
  assert.equal(Surface.supports({...mesh,materialClasses:null}),false);assert.equal(Surface.supports({...mesh,materialClasses:new Uint8Array(1)}),false);
  assert.throws(()=>Surface.prepare({...mesh,normals:new Float32Array(1)}),/matching/);assert.throws(()=>Surface.prepare({...mesh,colors:new Float32Array(mesh.colors.length).fill(NaN)}),/finite/);assert.throws(()=>Surface.prepare({...mesh,materialClasses:new Uint8Array(mesh.materialClasses.length).fill(8)}),/class/);
  assert.deepEqual(Surface.bake(mesh,{finish:'__proto__',wear:'constructor'}),Surface.bake(mesh));
});
test('baked finishes remain ordinary GLB colors while exact semantic ranges and positions survive',()=>{
  const mesh=materials(),colors=Surface.bake(mesh,{finish:'woodland',wear:'field'}),bytes=Export.glb({...mesh,colors},'Tank finish'),view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength),jsonLength=view.getUint32(12,true),document=JSON.parse(new TextDecoder().decode(bytes.slice(20,20+jsonLength)));
  assert.deepEqual(document.meshes[0].extras.parts,mesh.parts);assert.deepEqual(document.meshes[0].primitives[0].attributes,{POSITION:0,NORMAL:1,COLOR_0:2});
  const start=28+jsonLength+document.bufferViews[2].byteOffset;for(let i=0;i<colors.length;i++)assert.equal(view.getFloat32(start+i*4,true),colors[i]);
});

function fixture(platform='tank_standard',options={}){
  let id=0,builds=0;const frames=new Map(),gpu=new Set(),shaders=[],uniforms=new Map(),uploaded=[],deleted=[];
  const make=()=>{const value=++id;gpu.add(value);return value;},remove=value=>{deleted.push(value);gpu.delete(value);};
  const gl={createShader:make,createProgram:make,createBuffer:make,deleteShader:remove,deleteProgram:remove,deleteBuffer:remove,getShaderParameter:()=>true,getProgramParameter:()=>true,getAttribLocation:(_,name)=>({aPosition:0,aNormal:1,aColor:2,aSurface:3}[name]??-1),getUniformLocation:(_,name)=>name,ARRAY_BUFFER:1,STATIC_DRAW:2,VERTEX_SHADER:3,FRAGMENT_SHADER:4,COMPILE_STATUS:5,LINK_STATUS:6,DEPTH_TEST:7,LEQUAL:8,FLOAT:9,TRIANGLES:10,COLOR_BUFFER_BIT:16,DEPTH_BUFFER_BIT:32,shaderSource:(_,s)=>shaders.push(s),bufferData:(_,data)=>uploaded.push(data),uniform1f:(name,value)=>uniforms.set(name,value)};
  for(const name of ['compileShader','attachShader','linkProgram','bindBuffer','enable','depthFunc','clearColor','enableVertexAttribArray','disableVertexAttribArray','vertexAttribPointer','vertexAttrib4f','viewport','clear','useProgram','uniformMatrix4fv','uniform3fv','drawArrays'])gl[name]=()=>{};
  function node(){return {listeners:new Map(),style:{},dataset:{},children:[],isConnected:true,setAttribute(){},addEventListener(n,f){this.listeners.set(n,f);},removeEventListener(n){this.listeners.delete(n);},appendChild(n){this.children.push(n);n.parentNode=this;},replaceChildren(...children){this.children=children;},remove(){this.isConnected=false;},getBoundingClientRect:()=>({width:800,height:500,left:0,top:0})};}
  const doc=node(),canvas=node(),host=node(),slot=node(),status=node(),armourControls=node(),wearControl=node();wearControl.tagName='SELECT';doc.hidden=false;doc.createElement=tag=>tag==='canvas'?canvas:node();canvas.getContext=()=>gl;host.ownerDocument=doc;host.querySelector=s=>s==='[data-model-canvas]'?slot:s==='[data-model-status]'?status:null;host.querySelectorAll=s=>s==='[data-model-tank-only]'?[armourControls]:s==='[data-model-wear]'?[wearControl]:[];
  const mesh=options.mesh||materials(),ctx={module:{exports:{}},document:doc,Float32Array,Uint8Array,...(options.noModule?{}:{TankSurface:Surface}),addEventListener(){},removeEventListener(){},EquipmentMesh:{build(spec){builds++;return {...mesh,specification:{platform:spec.platform}};}},EquipmentExport:{glb:mesh=>mesh},requestAnimationFrame:fn=>{const key=++id;frames.set(key,fn);return key;},cancelAnimationFrame:key=>frames.delete(key)};
  vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/equipment-model.js'),'utf8'),ctx);const controller=ctx.module.exports.mount(host,{platform});
  const flush=()=>{const work=[...frames.values()];frames.clear();work.forEach(fn=>fn(1));};flush();return {controller,flush,canvas,gpu,gl,status,armourControls,wearControl,uniforms,shaders,uploaded,mesh,builds:()=>builds};
}
test('tank viewer uploads authored attributes, routes fragment finish/wear, and exports without rebuilding geometry',()=>{
  const f=fixture(),builds=f.builds();assert.equal(f.uniforms.get('uTankEnabled'),1);assert(f.shaders.some(s=>s.includes('tankFinishColor')&&s.includes('uTankCamo')));assert(f.uploaded.some(data=>data.length===f.mesh.materialClasses.length*4));
  f.controller.setFinish('woodland');f.controller.setWear('field');f.flush();assert.equal(f.uniforms.get('uTankCamo'),1);assert.equal(f.uniforms.get('uTankWear'),1);assert.equal(f.builds(),builds);
  const exported=f.controller.exportGlb();assert.deepEqual(exported.colors,Surface.bake(f.mesh,{finish:'woodland',wear:'field'}));assert.deepEqual(exported.positions,f.mesh.positions);
  f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('the same viewer disables armour treatment and extra buffers for aircraft and unrecognised vehicle families',()=>{
  for(const platform of ['air_light_attack','air_tactical_strike','ground_unreleased']){const f=fixture(platform);assert.equal(f.uniforms.get('uTankEnabled'),0);assert(!f.uploaded.some(data=>data.length===f.mesh.materialClasses.length*4));const old=f.controller.exportGlb().colors;
    f.controller.setFinish('woodland');f.controller.setWear('field');f.flush();assert.deepEqual(f.controller.exportGlb().colors,old);f.controller.dispose();assert.equal(f.gpu.size,0);}
});
test('context loss preserves paint/wear and restoration recreates the authored tank attribute',()=>{
  const f=fixture();f.controller.setFinish('sand');f.controller.setWear('field');const before=f.controller.exportGlb().colors;f.gpu.clear();f.canvas.listeners.get('webglcontextlost')({preventDefault(){}});assert.deepEqual(f.controller.exportGlb().colors,before);
  f.controller.setFinish('winter');const after=f.controller.exportGlb().colors;f.canvas.listeners.get('webglcontextrestored')();f.flush();assert.equal(f.uniforms.get('uTankEnabled'),1);assert.equal(f.uniforms.get('uTankWear'),1);assert.deepEqual(f.controller.exportGlb().colors,after);f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('one controller releases the tank material attribute when switching to aircraft and restores it when returning',()=>{
  const f=fixture(),tankAllocation=f.gpu.size;f.controller.setFinish('woodland');f.controller.setWear('field');f.flush();
  f.controller.update({platform:'air_light_attack'});f.flush();assert.equal(f.uniforms.get('uTankEnabled'),0);assert.equal(f.uniforms.get('uTankCamo'),0);assert.equal(f.gpu.size,tankAllocation-1);assert.deepEqual(f.controller.exportGlb().colors,f.mesh.colors);
  f.controller.setFinish('woodland');f.controller.setWear('factory');f.controller.update({platform:'tank_heavy'});f.flush();assert.equal(f.uniforms.get('uTankEnabled'),1);assert.equal(f.uniforms.get('uTankCamo'),0);assert.equal(f.uniforms.get('uTankWear'),1,'hidden aircraft wear control must not change the tank condition');assert.equal(f.gpu.size,tankAllocation);
  f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('repainting and orbiting reuse geometry, material parameters, and GPU allocations',()=>{
  const f=fixture(),count=f.uploaded.length,allocated=f.gpu.size,builds=f.builds();f.controller.setFinish('sand');f.flush();
  const repaint=f.uploaded.slice(count);assert.equal(repaint.length,1);assert.equal(repaint[0].length,f.mesh.colors.length);assert.notEqual(repaint[0],f.mesh.positions);assert.notEqual(repaint[0],f.mesh.normals);
  const after=f.uploaded.length;f.controller.rotate(.2,.1);f.controller.zoom(-1);f.controller.selectPart('protection');f.flush();assert.equal(f.uploaded.length,after);assert.equal(f.builds(),builds);assert.equal(f.gpu.size,allocated);
  f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('a failed extra material-buffer upload clears stale tank geometry and a retry recovers without leaking',()=>{
  const f=fixture(),allocated=f.gpu.size,upload=f.gl.bufferData;let failed=false;
  f.gl.bufferData=(target,data)=>{if(!failed&&data.length===f.mesh.materialClasses.length*4){failed=true;throw new Error('material allocation failed');}upload(target,data);};
  f.controller.update({platform:'tank_heavy'});f.flush();assert(failed);assert.equal(f.controller.exportGlb(),null);assert.equal(f.gpu.size,allocated-4);assert.match(f.status.textContent,/could not be displayed/);
  f.gl.bufferData=upload;f.controller.update({platform:'tank_heavy'});f.flush();assert(f.controller.exportGlb());assert.equal(f.uniforms.get('uTankEnabled'),1);assert.equal(f.gpu.size,allocated);f.controller.dispose();assert.equal(f.gpu.size,0);
});

const SPECIALISTS=['ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense'];
test('all five specialist armoured platforms have complete authored materials at every detail level',()=>{
  for(const platform of SPECIALISTS)for(const lod of [0,1,2]){const mesh=Mesh.build({platform,lod});assert(Surface.supports(mesh),platform);assert.equal(mesh.materialClasses.length,mesh.positions.length/3);assert(!mesh.materialClasses.includes(255),platform+' '+lod);
    if(lod===0)for(const kind of [0,1,3,4])assert(mesh.materialClasses.includes(kind),platform+' lacks paint/steel/rubber/glass class '+kind);}
});
test('each specialist repaint leaves steel, tyres, optics, cables and canvas unchanged',()=>{
  for(const platform of SPECIALISTS){const mesh=Mesh.build({platform,lod:1}),before=['positions','normals','colors','materialClasses'].map(k=>digest(mesh[k])),base=Surface.bake(mesh,{finish:'olive',wear:'factory'});
    for(const finish of ['sand','winter','woodland']){const out=Surface.bake(mesh,{finish,wear:'factory'});let paintChanges=0,materialChanges=0;for(let i=0;i<mesh.materialClasses.length;i++){const at=i*3,changed=out[at]!==base[at]||out[at+1]!==base[at+1]||out[at+2]!==base[at+2];if(changed){if(mesh.materialClasses[i]===0)paintChanges++;else materialChanges++;}}
      assert(paintChanges>0,platform+' '+finish+' must repaint armour');assert.equal(materialChanges,0,platform+' '+finish+' must preserve other materials');}
    assert.deepEqual(['positions','normals','colors','materialClasses'].map(k=>digest(mesh[k])),before);}
});
test('specialist field wear changes painted bodies and running gear while optics and lamps stay clear',()=>{
  for(const platform of SPECIALISTS){const mesh=Mesh.build({platform,lod:1}),base=Surface.bake(mesh,{finish:'sand',wear:'factory'}),out=Surface.bake(mesh,{finish:'sand',wear:'field'}),changed=new Set();let clearChanges=0;
    for(let i=0;i<mesh.materialClasses.length;i++){const at=i*3,kind=mesh.materialClasses[i];if(out[at]!==base[at]||out[at+1]!==base[at+1]||out[at+2]!==base[at+2]){changed.add(kind);if(kind===4||kind===7)clearChanges++;}}
    for(const kind of [0,1,3])assert(changed.has(kind),platform+' weathering must reach class '+kind);assert.equal(clearChanges,0,platform+' must retain clear optical surfaces');}
});
test('real specialist meshes activate native finish controls, export their selected condition and preserve material data on restore',()=>{
  for(const platform of SPECIALISTS){const mesh=Mesh.build({platform,lod:1}),f=fixture(platform,{mesh}),allocation=f.gpu.size;assert.equal(f.uniforms.get('uTankEnabled'),1);assert.equal(f.armourControls.hidden,false);assert.equal(f.wearControl.disabled,false);
    f.controller.setFinish('woodland');f.controller.setWear('field');f.flush();assert.equal(f.uniforms.get('uTankCamo'),1);assert.equal(f.uniforms.get('uTankWear'),1);assert.equal(f.wearControl.value,'field');assert.deepEqual(f.controller.exportGlb().colors,Surface.bake(mesh,{finish:'woodland',wear:'field'}));
    f.gpu.clear();f.canvas.listeners.get('webglcontextlost')({preventDefault(){}});f.canvas.listeners.get('webglcontextrestored')();f.flush();assert.equal(f.uniforms.get('uTankEnabled'),1);assert.equal(f.uniforms.get('uTankWear'),1);assert.equal(f.gpu.size,allocation);f.controller.dispose();assert.equal(f.gpu.size,0);}
});
test('missing armour module or malformed specialist metadata retains the ordinary usable viewer and hides unavailable controls',()=>{
  for(const options of [{noModule:true},{mesh:{...materials(),materialClasses:null}},{mesh:{...materials(),materialClasses:new Uint8Array(1)}}]){const f=fixture('ground_ifv',options);assert.equal(f.uniforms.get('uTankEnabled'),0);assert.equal(f.armourControls.hidden,true);assert.equal(f.wearControl.disabled,true);assert.match(f.status.textContent,/3D model ready/);
    f.controller.setFinish('sand');f.flush();assert.deepEqual(f.controller.exportGlb().colors,require('../../spheres-web/ui/equipment-model.js').finishColors(f.mesh.colors,'sand'));f.controller.dispose();assert.equal(f.gpu.size,0);}
});
test('switching between specialist armour and aircraft hides controls and releases the extra attribute without changing aircraft finishes',()=>{
  const f=fixture('ground_apc'),allocation=f.gpu.size;f.controller.setFinish('woodland');f.controller.setWear('field');f.controller.update({platform:'air_tactical_strike'});f.flush();assert.equal(f.armourControls.hidden,true);assert.equal(f.wearControl.disabled,true);assert.equal(f.uniforms.get('uTankEnabled'),0);assert.equal(f.gpu.size,allocation-1);
  f.controller.setFinish('sand');f.flush();assert.deepEqual(f.controller.exportGlb().colors,require('../../spheres-web/ui/equipment-model.js').finishColors(f.mesh.colors,'sand'));
  f.controller.update({platform:'ground_air_defense'});f.flush();assert.equal(f.armourControls.hidden,false);assert.equal(f.wearControl.disabled,false);assert.equal(f.uniforms.get('uTankEnabled'),1);assert.equal(f.uniforms.get('uTankWear'),1);assert.equal(f.gpu.size,allocation);f.controller.dispose();assert.equal(f.gpu.size,0);
});
test('approved tank shader and representative four-platform appearance buffers remain byte-identical through the specialist extension',()=>{
  assert.equal(crypto.createHash('sha256').update(Surface.glsl).digest('hex'),'277e7524d8e0bf281133fb1e6a2a198bad796a372b29570a703b4cb5f0e4a8d6');
  const snapshots=[
    ['tank_standard',0,'8160d8a538b0fd79beebafad0179d67cfd980c2feb12915157447db08ef95700','1b82ee4dfa0ca8743faca80764a789885e3bdb66bd35ee339dafa899be5f205a','540a295b872195aa9357dc34628a0dea6fb55987b5b526933d7aacede44b3da5','199b413cf7b41569ad34752041edd8d853f1a56b0fc11c902f588cb4484c1ec0'],
    ['tank_heavy',1,'e776f6710250c29cb671ae222bce586d6a43d6b5c87992d7df440a0798570a34','0b476ec1b4e501bb50f594600c9112c9b48660a7c47ca9b93059ba6b0aa5d62e','d80b81df287ef4928c7e218ff988c2090006e1df897586352e6a2fcf11e3fb94','be924e87f0b4898288833bf3221f02356ce6a67e6fd6936d14c565652dad915d'],
    ['tank_light',2,'0d952996f77b2dd857163153e8801a20b6bd63e0f3c6eaad62082a65da0726dc','9e337b4b069da07137cce025fd3c70944d510db499919523824c6627a0d8bb87','c0b0b43c86e6d04c767525f86e3f15964fa7f88e5f8021de71d0f9758aa211b6','e43d9511b759a856b6f2eac56d9f3dce91549f9506a8da1bf4b17f8f5294c66c'],
    ['tank_destroyer',0,'8160d8a538b0fd79beebafad0179d67cfd980c2feb12915157447db08ef95700','1b82ee4dfa0ca8743faca80764a789885e3bdb66bd35ee339dafa899be5f205a','540a295b872195aa9357dc34628a0dea6fb55987b5b526933d7aacede44b3da5','199b413cf7b41569ad34752041edd8d853f1a56b0fc11c902f588cb4484c1ec0']
  ];
  for(const [platform,lod,parameters,preview,olive,woodland] of snapshots){const mesh=Mesh.build({platform,lod});assert.equal(digest(Surface.prepare(mesh).parameters),parameters,platform+' material parameters');assert.equal(digest(Surface.bake(mesh,{finish:'sand',wear:'service',baseOnly:true})),preview,platform+' preview');assert.equal(digest(Surface.bake(mesh,{finish:'olive',wear:'factory'})),olive,platform+' factory finish');assert.equal(digest(Surface.bake(mesh,{finish:'woodland',wear:'field'})),woodland,platform+' field camouflage');}
});
