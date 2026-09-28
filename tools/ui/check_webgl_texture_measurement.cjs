// Run: node --test tools/ui/check_webgl_texture_measurement.cjs
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const measurement=require('./webgl-texture-measurement.js');
function fixture({webgl1=false}={}) {
  const names=['TEXTURE_2D','TEXTURE_CUBE_MAP','TEXTURE_BINDING_2D','TEXTURE_BINDING_CUBE_MAP',
    'TEXTURE_CUBE_MAP_POSITIVE_X','TEXTURE_CUBE_MAP_NEGATIVE_X','TEXTURE_CUBE_MAP_POSITIVE_Y',
    'TEXTURE_CUBE_MAP_NEGATIVE_Y','TEXTURE_CUBE_MAP_POSITIVE_Z','TEXTURE_CUBE_MAP_NEGATIVE_Z',
    'TEXTURE_BASE_LEVEL','TEXTURE_MAX_LEVEL','RGBA','RGB','RED','RG','DEPTH_COMPONENT','DEPTH_STENCIL',
    'ALPHA','LUMINANCE','LUMINANCE_ALPHA','RGBA8','RGB8','R8','R16F','RGBA16F','RGBA32F','RGB565',
    'DEPTH_COMPONENT16','DEPTH_COMPONENT24','DEPTH24_STENCIL8','DEPTH32F_STENCIL8',
    'UNSIGNED_BYTE','BYTE','UNSIGNED_SHORT','SHORT','UNSIGNED_INT','INT','FLOAT','HALF_FLOAT',
    'UNSIGNED_SHORT_5_6_5','UNSIGNED_SHORT_4_4_4_4','UNSIGNED_SHORT_5_5_5_1','UNSIGNED_INT_24_8','FLOAT_32_UNSIGNED_INT_24_8_REV'];
  let serial=0,unit=0,lost=false,errorReads=0,throwUpload=false;
  const bindings=new Map(),listeners=new Map(),parameters=new Map(),calls=[];
  const gl=Object.fromEntries(names.map((name,index)=>[name,index+1]));
  if(webgl1){delete gl.TEXTURE_BASE_LEVEL;delete gl.TEXTURE_MAX_LEVEL;delete gl.HALF_FLOAT;}
  const targetFamily=target=>target===gl.TEXTURE_2D?gl.TEXTURE_2D:gl.TEXTURE_CUBE_MAP;
  Object.assign(gl,{
    canvas:{addEventListener:(name,fn)=>listeners.set(name,fn),removeEventListener:(name,fn)=>{if(listeners.get(name)===fn)listeners.delete(name);}},
    isContextLost:()=>lost,
    createTexture(){return lost?null:{id:++serial};},
    deleteTexture(texture){for(const [key,value]of bindings)if(value===texture)bindings.set(key,null);},
    activeTexture(value){unit=value;},
    bindTexture(target,texture){bindings.set(unit+':'+targetFamily(target),texture);},
    getParameter(parameter){calls.push(['getParameter',parameter]);return bindings.get(unit+':'+(parameter===gl.TEXTURE_BINDING_2D?gl.TEXTURE_2D:gl.TEXTURE_CUBE_MAP))||null;},
    getTexParameter(target,parameter){calls.push(['getTexParameter',target,parameter]);return parameters.get(parameter)??(parameter===gl.TEXTURE_BASE_LEVEL?0:1000);},
    texImage2D(...args){calls.push(['texImage2D',...args]);if(throwUpload)throw Error('original upload failed');return 'original-upload-result';},
    texStorage2D(...args){calls.push(['texStorage2D',...args]);},
    generateMipmap(...args){calls.push(['generateMipmap',...args]);},
    getError(){errorReads++;return 1282;},
  });
  const originals=Object.fromEntries(['createTexture','deleteTexture','bindTexture','texImage2D','texStorage2D','generateMipmap'].map(name=>[name,gl[name]]));
  const probe=measurement.attach(gl);
  return {gl,probe,originals,calls,parameters,bindings,
    texture(target=gl.TEXTURE_2D){const texture=gl.createTexture();gl.bindTexture(target,texture);return texture;},
    upload(width,height,{internal=gl.RGBA,format=gl.RGBA,type=gl.UNSIGNED_BYTE,level=0,target=gl.TEXTURE_2D}={}) {
      return gl.texImage2D(target,level,internal,width,height,0,format,type,null);
    },
    lose(){lost=true;bindings.clear();listeners.get('webglcontextlost')?.();},restore(){lost=false;},
    setThrow(value){throwUpload=value;},errorReads:()=>errorReads,
  };
}
const bytes=f=>f.probe.snapshot().declared_texture_texel_payload_bytes;

test('shipped image, canvas and shadow declarations are counted and replacement is not accumulation',()=>{
  const f=fixture(),{gl}=f;const texture=f.texture();
  gl.texImage2D(gl.TEXTURE_2D,0,gl.RGB8,gl.RGB,gl.UNSIGNED_BYTE,{width:2400,height:1018});
  assert.equal(bytes(f),2400*1018*3);
  f.upload(1024,1024);assert.equal(bytes(f),1024*1024*4);
  gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,gl.RGBA,gl.UNSIGNED_BYTE,{naturalWidth:512,naturalHeight:256,width:30,height:20});
  assert.equal(bytes(f),512*256*4,'intrinsic image dimensions win over CSS dimensions');
  gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,gl.RGBA,gl.UNSIGNED_BYTE,{width:640,height:400});
  assert.equal(bytes(f),640*400*4);assert.equal(f.probe.snapshot().peak_declared_texture_texel_payload_bytes,2400*1018*3);
  assert.equal(f.probe.snapshot().live_textures,1);assert.equal(f.probe.snapshot().allocated_texture_levels,1);
  gl.deleteTexture(texture);assert.equal(bytes(f),0);assert.equal(f.probe.snapshot().live_textures,0);
});

test('actual bindings on the active unit determine ownership, including unobserved external rebinding',()=>{
  const f=fixture(),{gl}=f,a=f.texture();f.upload(2,3);
  gl.activeTexture(7);const b=f.texture();f.upload(3,4);assert.equal(bytes(f),24+48);
  // Bypass the observed bind call: the real getParameter binding still wins.
  f.bindings.set('7:'+gl.TEXTURE_2D,a);f.upload(5,6);assert.equal(bytes(f),120+48);
  gl.deleteTexture(a);assert.equal(bytes(f),48);gl.deleteTexture(b);assert.equal(bytes(f),0);
  assert(f.calls.some(call=>call[0]==='getParameter'));
});

test('shipped immutable RGBA16F/R16F chains allocate all declared levels exactly once',()=>{
  const f=fixture(),{gl}=f;f.texture();gl.texStorage2D(gl.TEXTURE_2D,4,gl.RGBA16F,8,4);
  assert.equal(bytes(f),(8*4+4*2+2*1+1*1)*8);
  gl.generateMipmap(gl.TEXTURE_2D);assert.equal(bytes(f),344,'generating immutable contents adds no storage');
  f.upload(100,100);assert.equal(bytes(f),344,'immutable image redefinition cannot allocate storage');
  gl.texStorage2D(gl.TEXTURE_2D,1,gl.RGBA8,100,100);assert.equal(bytes(f),344,'immutable storage cannot be redefined');
  f.texture();gl.texStorage2D(gl.TEXTURE_2D,3,gl.R16F,7,5);
  assert.equal(bytes(f),344+(35+6+1)*2);assert.equal(f.probe.snapshot().allocated_texture_levels,7);
});

test('generated mutable mips replace their own levels and retain separately allocated higher levels',()=>{
  const f=fixture(),{gl}=f;f.texture();f.upload(8,8);gl.generateMipmap(gl.TEXTURE_2D);
  assert.equal(bytes(f),(64+16+4+1)*4);gl.generateMipmap(gl.TEXTURE_2D);assert.equal(bytes(f),340);
  f.upload(4,4);assert.equal(bytes(f),(16+16+4+1)*4,'base redefinition alone leaves other levels allocated');
  gl.generateMipmap(gl.TEXTURE_2D);assert.equal(bytes(f),(16+4+1+1)*4,'generation redefines levels1–2 only');
});

test('mipmap ranges query actual base and maximum texture levels',()=>{
  const f=fixture(),{gl}=f;f.texture();f.upload(16,8,{internal:gl.R8,format:gl.RED,level:2});
  f.parameters.set(gl.TEXTURE_BASE_LEVEL,2);f.parameters.set(gl.TEXTURE_MAX_LEVEL,4);gl.generateMipmap(gl.TEXTURE_2D);
  assert.equal(bytes(f),128+32+8);assert.equal(f.probe.snapshot().allocated_texture_levels,3);
  assert(f.calls.some(call=>call[0]==='getTexParameter'&&call[2]===gl.TEXTURE_BASE_LEVEL));
});

test('WebGL1 default mip range and half-float extension payload do not require WebGL2 parameters',()=>{
  const f=fixture({webgl1:true}),{gl}=f;f.texture();f.upload(4,2,{type:0x8D61});gl.generateMipmap(gl.TEXTURE_2D);
  assert.equal(bytes(f),(8+2+1)*8);assert(!f.calls.some(call=>call[0]==='getTexParameter'));
});

test('cube faces have independent levels and immutable cube storage includes all six faces',()=>{
  const f=fixture(),{gl}=f;const texture=f.texture(gl.TEXTURE_CUBE_MAP);
  const faces=Object.entries(gl).filter(([name])=>/^TEXTURE_CUBE_MAP_(POSITIVE|NEGATIVE)_/.test(name)).map(([,value])=>value);
  for(const target of faces)f.upload(4,4,{target});gl.generateMipmap(gl.TEXTURE_CUBE_MAP);
  assert.equal(bytes(f),6*(16+4+1)*4);assert.equal(f.probe.snapshot().allocated_texture_levels,18);
  gl.deleteTexture(texture);f.texture(gl.TEXTURE_CUBE_MAP);gl.texStorage2D(gl.TEXTURE_CUBE_MAP,2,gl.RGBA16F,4,4);
  assert.equal(bytes(f),6*(16+4)*8);assert.equal(f.probe.snapshot().allocated_texture_levels,12);
});

test('known packed and sized depth payloads retain their declared width, excluding driver padding',()=>{
  const f=fixture(),{gl}=f;f.texture();
  for(const [internal,format,type,bpp]of [[gl.RGB,gl.RGB,gl.UNSIGNED_SHORT_5_6_5,2],
    [gl.RGBA,gl.RGBA,gl.UNSIGNED_SHORT_4_4_4_4,2],[gl.DEPTH_STENCIL,gl.DEPTH_STENCIL,gl.UNSIGNED_INT_24_8,4],
    [gl.DEPTH_COMPONENT24,gl.DEPTH_COMPONENT,gl.UNSIGNED_INT,3],[gl.RGBA32F,gl.RGBA,gl.FLOAT,16]]){
    f.upload(5,3,{internal,format,type});assert.equal(bytes(f),15*bpp);
  }
});

test('unknown allocation formats are explicit and never silently counted as zero',()=>{
  const f=fixture(),{gl}=f;const known=f.texture();f.upload(2,2);const unknown=f.texture();f.upload(10,10,{internal:0xabcdef});
  let s=f.probe.snapshot();assert.equal(s.declared_texture_texel_payload_bytes,null);assert.equal(s.known_texture_texel_payload_bytes,16);
  assert.equal(s.unmeasured_texture_allocations,1);assert.equal(s.peak_declared_texture_texel_payload_bytes,null);assert(f.probe.diagnostics().length);
  gl.generateMipmap(gl.TEXTURE_2D);s=f.probe.snapshot();assert.equal(s.unmeasured_texture_allocations,4,'unknown generated levels stay unmeasured');
  gl.deleteTexture(unknown);assert.equal(bytes(f),16);assert.equal(f.probe.snapshot().peak_declared_texture_texel_payload_bytes,null,'historical peak remains incomplete');
  gl.deleteTexture(known);assert.equal(bytes(f),0);
});

test('unknown image dimensions and formats/types retain an unmeasured level',()=>{
  const f=fixture(),{gl}=f;f.texture();gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,gl.RGBA,gl.UNSIGNED_BYTE,{});
  assert.equal(bytes(f),null);f.upload(1,1,{type:0xabcdef});assert.equal(bytes(f),null);
  f.upload(1,1);assert.equal(bytes(f),4,'known replacement repairs current accounting without erasing the incomplete peak');
});

test('pre-existing textures and missing mip bases are disclosed until deletion',()=>{
  const f=fixture(),{gl}=f;const external={id:'before-probe'};gl.bindTexture(gl.TEXTURE_2D,external);f.upload(4,4);
  assert.equal(bytes(f),null);assert.equal(f.probe.snapshot().known_texture_texel_payload_bytes,64);
  gl.deleteTexture(external);assert.equal(bytes(f),0);const empty=f.texture();gl.generateMipmap(gl.TEXTURE_2D);
  assert.equal(bytes(f),null);gl.deleteTexture(empty);assert.equal(bytes(f),0);
});

test('context loss clears resident declarations and restoration accepts new handles without stale bytes',()=>{
  const f=fixture(),{gl}=f;f.texture();f.upload(8,8);f.lose();
  assert.equal(bytes(f),0);assert.equal(f.probe.snapshot().live_textures,0);assert.equal(f.probe.snapshot().context_losses,1);
  assert.equal(gl.createTexture(),null);f.upload(99,99);assert.equal(bytes(f),0);
  f.restore();f.texture();f.upload(2,2);assert.equal(bytes(f),16);assert.equal(f.probe.snapshot().peak_known_texture_texel_payload_bytes,256);
});

test('probe preserves original results and exceptions, does not consume errors, and can detach',()=>{
  const f=fixture(),{gl}=f;assert.equal(measurement.attach(gl),f.probe,'double installation cannot double-count');
  f.texture();assert.equal(f.upload(2,2),'original-upload-result');f.setThrow(true);assert.throws(()=>f.upload(30,30),/original upload failed/);
  assert.equal(bytes(f),16);assert.equal(f.errorReads(),0);
  f.probe.stop();for(const [name,original]of Object.entries(f.originals))assert.equal(gl[name],original,name);
  f.probe.stop();assert.equal(f.errorReads(),0);
});

test('standalone browser export exposes the same instrumentation and its measurement limits',()=>{
  const c=vm.createContext({});vm.runInContext(fs.readFileSync(path.join(__dirname,'webgl-texture-measurement.js'),'utf8'),c);
  assert.equal(typeof c.WebGLTextureMeasurement.attach,'function');assert.match(c.WebGLTextureMeasurement.method,/not driver VRAM/);
  assert.match(c.WebGLTextureMeasurement.method,/renderbuffers/);assert.match(c.WebGLTextureMeasurement.method,/No GL errors/);
});
