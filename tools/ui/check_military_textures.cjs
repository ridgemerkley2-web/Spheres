const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const filename=path.join(__dirname,'../../spheres-web/ui/military-surface.js'),source=fs.readFileSync(filename,'utf8');

function fixture(options={}){
  let active=105,lost=false,ready=0,requests=0,error=0;
  const owned=new Set(),deleted=[],images=[],uploads=[],parameters=[],uniforms=[],queries=[],calls=[];
  const bindings=new Map([[100,{name:'shadow'}],[101,{name:'previous normal'}],[102,{name:'previous roughness'}],[105,{name:'previous active'}]]);
  const pixels=new Map([[41,false],[42,true],[43,44]]);
  const snapshot=()=>({active,bindings:[...bindings],pixels:[...pixels]});
  const gl={TEXTURE0:100,TEXTURE_2D:1,RGBA:2,UNSIGNED_BYTE:3,TEXTURE_MIN_FILTER:4,TEXTURE_MAG_FILTER:5,TEXTURE_WRAP_S:6,TEXTURE_WRAP_T:7,LINEAR:8,LINEAR_MIPMAP_LINEAR:9,REPEAT:10,
    MAX_TEXTURE_IMAGE_UNITS:21,MAX_TEXTURE_SIZE:22,ACTIVE_TEXTURE:23,TEXTURE_BINDING_2D:24,UNPACK_FLIP_Y_WEBGL:41,UNPACK_PREMULTIPLY_ALPHA_WEBGL:42,UNPACK_COLORSPACE_CONVERSION_WEBGL:43,BROWSER_DEFAULT_WEBGL:44,NONE:0,NO_ERROR:0,
    createTexture(){assert(!lost);requests++;if(requests===options.allocationFailure)return null;const texture={id:requests};owned.add(texture);calls.push(['create',texture]);return texture;},
    deleteTexture(texture){assert(!lost,'Context-loss disposal must not issue GPU commands');assert(owned.has(texture),'No double deletion');owned.delete(texture);deleted.push(texture);calls.push(['delete',texture]);for(const [unit,bound] of bindings)if(bound===texture)bindings.set(unit,null);},
    activeTexture(unit){assert(!lost);active=unit;calls.push(['active',unit]);},
    bindTexture(_,texture){assert(!lost);bindings.set(active,texture);calls.push(['bind',active,texture]);},
    getParameter(name){assert(!lost);if(name===gl.MAX_TEXTURE_IMAGE_UNITS)return options.units??8;if(name===gl.MAX_TEXTURE_SIZE)return options.maxSize??4096;if(name===gl.ACTIVE_TEXTURE)return active;if(name===gl.TEXTURE_BINDING_2D)return bindings.get(active)||null;return pixels.get(name);},
    pixelStorei(name,value){assert(!lost);pixels.set(name,value);calls.push(['pixel',name,value]);},
    texImage2D(...args){assert(!lost);const texture=bindings.get(active);assert(owned.has(texture));const image=args.length===6?args[5]:null;
      uploads.push({unit:active,texture,image,width:image?image.naturalWidth:args[3],height:image?image.naturalHeight:args[4],pixels:new Map(pixels)});
      if(options.uploadFailure==='placeholder'&&!image||options.uploadFailure==='image'&&image)throw new Error('Synthetic upload failure');
      if(options.glFailure&&image)error=1285;
    },
    texParameteri(target,name,value){assert(!lost);parameters.push({texture:bindings.get(active),target,name,value});},
    generateMipmap(){assert(!lost);calls.push(['mipmap',active,bindings.get(active)]);if(options.mipmapFailure)throw new Error('Synthetic mipmap failure');},
    getError(){const previous=error;error=0;return previous;},
    getUniformLocation(program,name){assert(!lost);queries.push({program,name});return {program,name};},
    uniform1i(location,value){assert(!lost);uniforms.push({location,value});},uniform1f(location,value){assert(!lost);uniforms.push({location,value});}
  };
  const page=new URL(options.pageUrl||'http://127.0.0.1:7841/tools/arsenal/military-inspection.html');
  const ctx={module:{exports:{}},URL,Uint8Array,WeakMap,Image:class{constructor(){images.push(this);}set src(value){this.url=value;}},location:page,
    document:{currentScript:{src:options.moduleUrl||'http://127.0.0.1:7841/spheres-web/ui/military-surface.js'},location:page,baseURI:page.href}};
  if(options.noImage)delete ctx.Image;
  if(options.noModule)ctx.document.currentScript=null;
  if(options.missingMethod)delete gl[options.missingMethod];
  vm.runInNewContext(source,ctx);
  const before=snapshot(),controller=ctx.module.exports.create(gl,()=>{ready++;});
  function load(index,width=1024,height=1024){const image=images[index];Object.assign(image,{naturalWidth:width,naturalHeight:height});image.onload?.();}
  function lose(){lost=true;owned.clear();}
  const value=(program,name)=>uniforms.filter(u=>u.location.program===program&&u.location.name===name).at(-1)?.value;
  return {ctx,gl,controller,before,snapshot,owned,deleted,images,uploads,parameters,uniforms,queries,calls,bindings,pixels,load,lose,value,ready:()=>ready};
}

test('texture paths are local and relative to the module in both embedded and inspection routes',()=>{
  for(const [moduleUrl,expected] of [['http://localhost:7836/military-surface.js','http://localhost:7836/military-textures/'],['http://127.0.0.1:7841/spheres-web/ui/military-surface.js?v=2','http://127.0.0.1:7841/spheres-web/ui/military-textures/']]){
    const f=fixture({moduleUrl,pageUrl:new URL('/game',moduleUrl).href});assert(f.controller);assert.deepEqual(f.images.map(image=>image.url),[expected+'paint-normal.jpg',expected+'paint-roughness.jpg']);assert.equal(f.owned.size,2);f.controller.dispose();assert.equal(f.owned.size,0);
  }
});

test('a cross-origin module cannot trigger external texture downloads',()=>{
  const f=fixture({moduleUrl:'https://cdn.example.invalid/military-surface.js'});assert.equal(f.controller,null);assert.equal(f.images.length,0);assert.equal(f.owned.size,0);
});

test('missing browser APIs and insufficient device limits remain allocation-free',()=>{
  for(const options of [{noImage:true},{noModule:true},{missingMethod:'generateMipmap'},{units:2},{maxSize:512}]){
    const f=fixture(options);assert.equal(f.controller,null,JSON.stringify(options));assert.equal(f.owned.size,0);assert.equal(f.images.length,0);assert.equal(f.ready(),0);
  }
});

test('readiness requires both validated 1K images; placeholders are complete and never enable partial detail',()=>{
  const f=fixture(),p={name:'program'};assert.equal(f.controller.state,'loading');assert.equal(f.uploads.length,2);assert(f.uploads.every(u=>u.width===1&&u.height===1));
  f.controller.bind(p);assert.equal(f.value(p,'uSurfaceMaps'),0);f.load(1);f.controller.bind(p);assert.equal(f.value(p,'uSurfaceMaps'),0);assert.equal(f.ready(),0);
  f.load(0);assert.equal(f.controller.state,'ready');assert.equal(f.ready(),1);f.controller.bind(p);assert.equal(f.value(p,'uSurfaceMaps'),1);assert.equal(f.owned.size,2);assert.equal(f.uploads.length,4);
  assert.equal(f.calls.filter(call=>call[0]==='mipmap').length,2);f.controller.dispose();assert.equal(f.owned.size,0);
});

test('uploads preserve active unit, bindings and pixel-store state, and reserve units 1 and 2 without touching shadow unit 0',()=>{
  const f=fixture();assert.deepEqual(f.snapshot(),f.before,'Placeholder initialization restores previous GL state');f.load(0);assert.deepEqual(f.snapshot(),f.before);f.load(1);assert.deepEqual(f.snapshot(),f.before);
  for(const upload of f.uploads){assert([101,102].includes(upload.unit));assert.equal(upload.pixels.get(f.gl.UNPACK_FLIP_Y_WEBGL),true);assert.equal(upload.pixels.get(f.gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL),false);assert.equal(upload.pixels.get(f.gl.UNPACK_COLORSPACE_CONVERSION_WEBGL),f.gl.NONE);}
  const shadow=f.bindings.get(100),p={};f.controller.bind(p);assert.equal(f.snapshot().active,f.before.active);assert.equal(f.bindings.get(100),shadow);assert.equal(f.value(p,'uSurfaceNormal'),1);assert.equal(f.value(p,'uSurfaceRoughness'),2);
  assert(f.owned.has(f.bindings.get(101)));assert(f.owned.has(f.bindings.get(102)));assert(f.parameters.some(p=>p.name===f.gl.TEXTURE_MIN_FILTER&&p.value===f.gl.LINEAR_MIPMAP_LINEAR));
  assert(f.parameters.filter(p=>[f.gl.TEXTURE_WRAP_S,f.gl.TEXTURE_WRAP_T].includes(p.name)).every(p=>p.value===f.gl.REPEAT));f.controller.dispose();assert.equal(f.bindings.get(100),shadow);
});

test('reusing the surface across shader programs allocates no new textures and caches locations per program',()=>{
  const f=fixture(),p={},q={};f.load(0);f.load(1);const allocations=f.calls.filter(call=>call[0]==='create').length;
  for(let i=0;i<20;i++)f.controller.bind(i%2?p:q);
  assert.equal(f.calls.filter(call=>call[0]==='create').length,allocations);assert.equal(f.owned.size,2);assert.equal(f.queries.length,6);assert.equal(f.value(p,'uSurfaceMaps'),1);assert.equal(f.value(q,'uSurfaceMaps'),1);f.controller.dispose();
});

test('allocation, upload and mipmap failures release the complete pair and restore prior GL state',()=>{
  for(const options of [{allocationFailure:1},{allocationFailure:2},{uploadFailure:'placeholder'},{uploadFailure:'image'},{mipmapFailure:true},{glFailure:true}]){
    const f=fixture(options),initialFailure=f.controller.state==='fallback';if(!initialFailure)f.load(0);assert.equal(f.controller.state,'fallback',JSON.stringify(options));assert.equal(f.owned.size,0);assert.equal(f.ready(),initialFailure?0:1);assert.deepEqual(f.snapshot(),f.before);
    assert(f.images.every(image=>image.onload===null&&image.onerror===null));const p={};f.controller.bind(p);assert.equal(f.value(p,'uSurfaceMaps'),0);f.controller.dispose();assert.equal(f.owned.size,0);
  }
});

test('synchronous creation failures do not re-enter the caller while later load failures still request a redraw',()=>{
  for(const options of [{allocationFailure:1},{allocationFailure:2},{uploadFailure:'placeholder'}]){
    const f=fixture(options);assert.equal(f.controller.state,'fallback');assert.equal(f.ready(),0,'The caller is still setting up its render pass');const p={};f.controller.bind(p);assert.equal(f.value(p,'uSurfaceMaps'),0);assert.equal(f.owned.size,0);f.controller.dispose();
  }
  const later=fixture();later.images[0].onerror();assert.equal(later.ready(),1,'An asynchronous failure invalidates earlier placeholder frames');later.controller.dispose();
});

test('network errors or any dimension mismatch discard the pair even after one map succeeds',()=>{
  for(const size of [null,[2048,2048],[1024,512],[1023,1024],[0,0]]){
    const f=fixture();f.load(0);if(size)f.load(1,...size);else f.images[1].onerror();assert.equal(f.controller.state,'fallback');assert.equal(f.owned.size,0);assert.equal(f.ready(),1);
    assert.equal(f.uploads.length,3,'Rejected images never reach texImage2D');f.controller.dispose();
  }
});

test('late load and error callbacks after disposal or context loss never touch GPU resources or notify readiness',()=>{
  for(const contextLost of [false,true]){
    const f=fixture(),callbacks=f.images.map(image=>({load:image.onload,error:image.onerror,image}));if(contextLost)f.lose();f.controller.dispose(contextLost);const count=f.calls.length;
    callbacks.forEach(({load,error,image})=>{Object.assign(image,{naturalWidth:1024,naturalHeight:1024});load();error();});f.controller.bind({});f.controller.dispose(contextLost);
    assert.equal(f.calls.length,count);assert.equal(f.owned.size,0);assert.equal(f.ready(),0);assert.equal(f.controller.state,'disposed');assert.equal(f.deleted.length,contextLost?0:2);
  }
});

test('late callbacks following a failed load cannot revive a half-loaded material',()=>{
  const f=fixture(),late=f.images[1].onload;f.images[0].onerror();Object.assign(f.images[1],{naturalWidth:1024,naturalHeight:1024});const calls=f.calls.length;late();assert.equal(f.controller.state,'fallback');assert.equal(f.calls.length,calls);assert.equal(f.ready(),1);f.controller.dispose();
});

function jpegSize(bytes){
  assert.equal(bytes.readUInt16BE(0),0xffd8);let offset=2;
  while(offset<bytes.length){while(bytes[offset]===0xff)offset++;const marker=bytes[offset++];if(marker===0xd9||marker===0xda)break;const length=bytes.readUInt16BE(offset);assert(length>=2&&offset+length<=bytes.length);if([0xc0,0xc1,0xc2].includes(marker))return {width:bytes.readUInt16BE(offset+5),height:bytes.readUInt16BE(offset+3),channels:bytes[offset+7]};offset+=length;}
  throw new Error('JPEG dimensions missing');
}
test('runtime textures retain the verified original bytes, 1K dimensions and bounded transfer sizes',()=>{
  const folder=path.join(__dirname,'../../spheres-web/ui/military-textures');
  for(const [name,hash,bytes,channels,limit] of [
    ['paint-normal.jpg','970f0273c9e2e3b4fc8338bfd58a28c413b70ac4d1734d9dc8a136724bde56e6',238025,3,1000000],
    ['paint-roughness.jpg','37168cf57144db28dd0743dab47fbebc09147fac919d5e2b5610b069c0b13a46',402450,1,500000]
  ]){const data=fs.readFileSync(path.join(folder,name));assert.equal(data.length,bytes);assert(data.length<limit);assert.equal(crypto.createHash('sha256').update(data).digest('hex'),hash);assert.deepEqual(jpegSize(data),{width:1024,height:1024,channels});}
  const readme=fs.readFileSync(path.join(folder,'README.md'),'utf8');assert(readme.includes('CC0 1.0 Universal'));assert(readme.includes('https://polyhaven.com/a/blue_metal_plate'));assert(readme.includes('OpenGL / +Y tangent-space normal'));
});
