// Exercise the actual renderer dispatch: military lighting must not recolor portraits or cities.
const test=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const ui=path.resolve(__dirname,'../../spheres-web/ui');
function fixture({materialStub=false,materialInitFailure=false}={}){
  const shaders=[],uniforms=new Map(),draws=[],canvases=[],packs=[],contextRequests=[],programs=[];
  let currentProgram=null,textureRequests=0;
  const gl={};
  ['ARRAY_BUFFER','STATIC_DRAW','FLOAT','TRIANGLES','COLOR_BUFFER_BIT','DEPTH_BUFFER_BIT','DEPTH_TEST','CULL_FACE','SCISSOR_TEST','COMPILE_STATUS','LINK_STATUS','VERTEX_SHADER','FRAGMENT_SHADER'].forEach((key,i)=>gl[key]=i+1);
  for(const name of ['bindBuffer','bufferData','enableVertexAttribArray','vertexAttribPointer','compileShader','attachShader','bindAttribLocation','linkProgram','useProgram','uniformMatrix4fv','uniform3f','viewport','scissor','enable','disable','clearColor','clear','bindVertexArray','deleteBuffer','deleteVertexArray','deleteShader','deleteProgram'])gl[name]=()=>{};
  for(const name of ['createShader','createProgram','createBuffer','createVertexArray'])gl[name]=()=>({});
  gl.createProgram=()=>{const program={id:programs.length+1};programs.push(program);return program;};
  gl.useProgram=program=>{currentProgram=program;};
  gl.getShaderParameter=gl.getProgramParameter=()=>true;
  gl.shaderSource=(_,source)=>shaders.push(source);
  gl.getUniformLocation=(_,name)=>name;
  gl.uniform1f=(name,value)=>uniforms.set(name,value);
  if(materialInitFailure){
    gl.MAX_TEXTURE_IMAGE_UNITS=100;gl.MAX_TEXTURE_SIZE=101;
    gl.getParameter=name=>name===gl.MAX_TEXTURE_IMAGE_UNITS?8:4096;
    gl.createTexture=()=>{textureRequests++;return null;};
    for(const name of ['deleteTexture','bindTexture','texImage2D','texParameteri','generateMipmap','activeTexture','pixelStorei','uniform1i'])gl[name]=()=>{};
  }
  gl.drawArrays=()=>draws.push({...Object.fromEntries(uniforms),program:currentProgram});
  const make=()=>{const copies=[],flat={clearRect(){},drawImage(...args){copies.push(args);},getImageData(){return {data:[]};}};
    const canvas={width:200,height:140,style:{},isConnected:true,listeners:{},copies,setAttribute(){},getAttribute(){return null;},closest(){return null;},classList:{add(){},remove(){}},getBoundingClientRect(){return {width:200,height:140};},addEventListener(name,fn){this.listeners[name]=fn;},removeEventListener(){},remove(){},getContext(kind){contextRequests.push({canvas,kind});return kind==='webgl2'?gl:flat;}};canvases.push(canvas);return canvas;};
  const geometry={id:'arm_gen3',positions:new Float32Array([0,0,0,1,0,0,0,1,0]),normals:new Float32Array([0,0,1,0,0,1,0,0,1]),colors:new Float32Array(9).fill(.3),count:3,min:[0,0,0],max:[1,1,1],size:[1,1,1],centre:[.5,.5,.5]};
  const context={Float32Array,Uint8Array,Math,Number,Map,Set,Array,Object,JSON,devicePixelRatio:1,requestAnimationFrame:()=>0,cancelAnimationFrame(){},document:{createElement:make,addEventListener(){},removeEventListener(){},querySelectorAll(){return [];}},ArsenalModels:{ids:()=>['arm_gen3'],build:()=>geometry}};
  if(materialInitFailure){context.URL=URL;context.Image=class{};context.location={href:'http://localhost:7836/'};context.document.currentScript={src:'http://localhost:7836/military-surface.js'};}
  vm.createContext(context);
  vm.runInContext(fs.readFileSync(path.join(ui,'military-surface.js'),'utf8'),context);
  if(materialStub)context.MilitarySurface={...context.MilitarySurface,create(contextGl,onReady){
    const pack={gl:contextGl,binds:[],disposals:[],loaded:false,closed:false,
      bind(program){assert.equal(this.closed,false,'A disposed pack must never be rebound');this.binds.push(program);contextGl.uniform1f('uSurfaceMaps',this.loaded?1:0);},
      dispose(lost=false){this.disposals.push(lost);this.closed=true;},
      complete(){assert.equal(this.closed,false);this.loaded=true;onReady();}};
    packs.push(pack);return pack;
  }};
  vm.runInContext(fs.readFileSync(path.join(ui,'arsenal3d.js'),'utf8'),context);
  for(const prefix of ['person','town','site','veh'])context.Arsenal3D.register(prefix,rest=>({...geometry,id:prefix+':'+rest,assetKind:prefix==='person'?'character':undefined}));
  return {api:context.Arsenal3D,shaders,draws,canvases,make,gl,packs,programs,contextRequests,textureRequests:()=>textureRequests};
}
test('military meshes use the new material response while portraits and civil geometry keep their lighting',()=>{
  const f=fixture();assert.equal(f.api.available,true);
  for(const [id,expected] of [['arm_gen3',1],['veh:test',1],['person:test',0],['town:test',0],['site:test',0]]){
    assert.equal(f.api.mount(f.make(),id),true,id);assert.equal(f.draws.at(-1).uMilitary,expected,id);
    assert.equal(f.draws.at(-1).uCharacter,id.startsWith('person:')?1:0,id);
  }
  const fragment=f.shaders.find(source=>source.includes('militaryLighting'));
  assert(fragment,'local material shader must be installed');
  assert(!fragment.includes('__MILITARY'),'no unresolved shader insertion');
  assert(fragment.indexOf('if (uCharacter > 0.5)')<fragment.indexOf('if(uMilitary>.5)'), 'portrait shading exits before military treatment');
});
test('reinstalling a civil surface keeps military uniforms active',()=>{
  const f=fixture();assert.equal(f.api.available,true);
  assert.equal(f.api.setSurface('vec3 surface(vec3 a,vec3 n,vec3 p,vec3 v){return a*.9;}'),true);
  assert.equal(f.api.mount(f.make(),'arm_gen3'),true);assert.equal(f.draws.at(-1).uMilitary,1);
  assert.equal(f.api.mount(f.make(),'town:test'),true);assert.equal(f.draws.at(-1).uMilitary,0);
});
test('the embedded game and art bench load the material module before either renderer',()=>{
  for(const name of ['spheres-web/ui/index.html','tools/arsenal/military-inspection.html']){
    const html=fs.readFileSync(path.resolve(ui,'../..',name),'utf8');
    const scripts=Array.from(html.matchAll(/<script[^>]+src="([^"]+)"/g),match=>new URL(match[1],'https://spheres.invalid/').pathname.split('/').at(-1));
    const material=scripts.indexOf('military-surface.js');
    assert(material>=0,name);
    for(const renderer of ['equipment-model.js','arsenal3d.js'])assert(material<scripts.indexOf(renderer),`${name}: ${renderer}`);
  }
});

test('civil-only cards and cached sprites never create the military texture pack',()=>{
  const f=fixture({materialStub:true});
  for(const id of ['person:test','town:test','site:test']){assert.equal(f.api.mount(f.make(),id),true);assert(f.api.sprite(id,64));}
  assert.equal(f.packs.length,0);assert(f.draws.every(draw=>draw.uMilitary===0));assert.equal(f.contextRequests.filter(request=>request.kind==='webgl2').length,1);
});

test('all military cards and sprites share one pack and one WebGL context',()=>{
  const f=fixture({materialStub:true});
  for(const id of ['arm_gen3','veh:tank','veh:aircraft','veh:ship'])assert.equal(f.api.mount(f.make(),id),true);
  assert(f.api.sprite('arm_gen3',64));assert(f.api.sprite('veh:ship',64));
  assert.equal(f.packs.length,1);assert.equal(f.packs[0].gl,f.gl);assert.equal(f.packs[0].binds.length,f.draws.length);assert.equal(new Set(f.packs[0].binds).size,1);
  assert(f.draws.every(draw=>draw.uMilitary===1&&draw.uSurfaceMaps===0));assert.equal(f.contextRequests.filter(request=>request.kind==='webgl2').length,1);
});

test('texture readiness repaints connected mounted cards and invalidates previously baked sprites',()=>{
  const f=fixture({materialStub:true}),tank=f.make(),aircraft=f.make(),civil=f.make(),removed=f.make();
  for(const [canvas,id] of [[tank,'arm_gen3'],[aircraft,'veh:aircraft'],[civil,'town:test'],[removed,'veh:removed']])assert.equal(f.api.mount(canvas,id),true);
  const before=f.api.sprite('arm_gen3',64),civilBefore=f.api.sprite('town:test',64),count=f.draws.length;
  assert.equal(f.api.sprite('arm_gen3',64),before);assert.equal(f.draws.length,count,'A cached sprite is not rerendered');removed.isConnected=false;
  const copies=[tank,aircraft,civil,removed].map(canvas=>canvas.copies.length);f.packs[0].complete();
  assert.equal(f.draws.length,count+3);assert.deepEqual([tank,aircraft,civil,removed].map(canvas=>canvas.copies.length),[copies[0]+1,copies[1]+1,copies[2]+1,copies[3]]);
  assert(f.draws.slice(-3).every(draw=>draw.uSurfaceMaps===1));assert.deepEqual(f.draws.slice(-3).map(draw=>draw.uMilitary),[1,1,0]);
  const rebuilt=f.api.sprite('arm_gen3',64);assert.notEqual(rebuilt,before);assert.notEqual(f.api.sprite('town:test',64),civilBefore);assert.equal(f.api.sprite('arm_gen3',64),rebuilt);
  assert.equal(f.packs.length,1);assert.equal(f.contextRequests.filter(request=>request.kind==='webgl2').length,1);
});

test('surface-program replacement rebinds the existing ready pack and repaints without downloading again',()=>{
  const f=fixture({materialStub:true}),canvas=f.make();assert.equal(f.api.mount(canvas,'arm_gen3'),true);f.packs[0].complete();
  const pack=f.packs[0],old=pack.binds.at(-1),copies=canvas.copies.length,draws=f.draws.length;
  assert.equal(f.api.setSurface('vec3 surface(vec3 a,vec3 n,vec3 p,vec3 v){return a*.85;}'),true);
  assert.equal(f.packs.length,1);assert.notEqual(pack.binds.at(-1),old);assert.equal(pack.binds.at(-1),f.programs.at(-1));assert.equal(canvas.copies.length,copies+1);assert.equal(f.draws.length,draws+1);assert.equal(f.draws.at(-1).uSurfaceMaps,1);assert.equal(pack.disposals.length,0);
});

test('context loss discards the pack and restoration creates one replacement on the original GL canvas',()=>{
  const f=fixture({materialStub:true}),tank=f.make(),civil=f.make();assert.equal(f.api.mount(tank,'arm_gen3'),true);assert.equal(f.api.mount(civil,'town:test'),true);f.packs[0].complete();
  const oldPack=f.packs[0],oldSprite=f.api.sprite('arm_gen3',64),request=f.contextRequests.find(entry=>entry.kind==='webgl2'),count=f.draws.length;let prevented=false;
  request.canvas.listeners.webglcontextlost({preventDefault(){prevented=true;}});assert(prevented);assert.deepEqual(oldPack.disposals,[true]);assert.equal(f.api.sprite('arm_gen3',64),null);assert.equal(f.draws.length,count);
  request.canvas.listeners.webglcontextrestored();assert.equal(f.packs.length,2);assert.equal(f.packs[1].gl,f.gl);assert.notEqual(f.packs[1],oldPack);assert.equal(f.contextRequests.filter(entry=>entry.kind==='webgl2').length,1);
  assert.equal(f.draws.length,count+2);assert.equal(f.draws.at(-2).uSurfaceMaps,0);assert.notEqual(f.api.sprite('arm_gen3',64),oldSprite);const fresh=f.packs[1],copies=tank.copies.length;fresh.complete();assert.equal(tank.copies.length,copies+1);assert.equal(f.draws.at(-2).uSurfaceMaps,1);assert.deepEqual(oldPack.disposals,[true]);
});

test('a real texture allocation failure cannot repaint a city over an in-progress military sprite draw',()=>{
  const f=fixture({materialInitFailure:true}),city=f.make();assert.equal(f.api.mount(city,'town:test'),true);assert.equal(f.textureRequests(),0);
  const count=f.draws.length,copies=city.copies.length,sprite=f.api.sprite('arm_gen3',64);assert(sprite);assert.equal(f.textureRequests(),1,'The real loader attempted the bounded allocation');
  assert.equal(f.draws.length,count+1,'No reentrant mounted-card redraw occurs while creating the optional pack');assert.equal(city.copies.length,copies);assert.equal(f.draws.at(-1).uMilitary,1);assert.equal(f.draws.at(-1).uCharacter,0);assert.equal(f.draws.at(-1).uSurfaceMaps,0);
  assert.equal(f.api.sprite('arm_gen3',64),sprite);assert.equal(f.draws.length,count+1,'The cached sprite contains the correctly shaded military frame');
});
