/* Test instrumentation only; never loaded by the game.
 * Records declared 2D/cube texture texel payload from API allocation requests.
 * This is NOT queried GPU residency or driver VRAM. Sized internal formats use
 * their declared texel layout; unsized formats use format/type payload width.
 * Row unpack alignment, source padding, driver expansion/alignment, compression,
 * renderbuffers, framebuffers, multisampling and CPU image storage are excluded.
 * No getError call is made: driver acceptance/allocation failure is not verified
 * and pending errors are never consumed. Install before texture creation. Any
 * unknown layout or pre-existing texture makes total payload explicitly null;
 * known payload and unknown counts remain available separately.
 */
(function(root){
  'use strict';
  const attached=new WeakMap();
  const method='Declared API texture texel payload, including allocated mip levels; not driver VRAM or verified successful GPU allocation. Excludes source row padding/unpack alignment, driver layout/expansion/overhead, renderbuffers, framebuffer/default surface storage and CPU images. Unsupported layouts are unmeasured, not zero. No GL errors are read or cleared.';
  function attach(gl){
    if(attached.has(gl))return attached.get(gl);
    const textures=new Map(),originals=new Map(),wrappers=new Map(),reasons=new Set();
    let active=true,nextId=1,known=0,peakKnown=0,peakUnknown=false,unattributed=0;
    const calls={tex_image_2d:0,tex_storage_2d:0,generate_mipmap:0,unmeasured:0,context_losses:0,probe_errors:0};
    const is=(value,name)=>gl[name]!==undefined&&value===gl[name];
    const sized=new Map();
    for(const [bytes,names]of [
      [1,'R8 R8_SNORM R8I R8UI'],
      [2,'R16F R16I R16UI RG8 RG8_SNORM RG8I RG8UI RGB565 RGBA4 RGB5_A1 DEPTH_COMPONENT16'],
      [3,'RGB8 RGB8_SNORM RGB8I RGB8UI SRGB8 DEPTH_COMPONENT24'],
      [4,'R32F R32I R32UI RG16F RG16I RG16UI RGBA8 RGBA8_SNORM RGBA8I RGBA8UI SRGB8_ALPHA8 RGB10_A2 RGB10_A2UI R11F_G11F_B10F RGB9_E5 DEPTH_COMPONENT32F DEPTH24_STENCIL8'],
      [6,'RGB16F RGB16I RGB16UI'],
      [8,'RG32F RG32I RG32UI RGBA16F RGBA16I RGBA16UI DEPTH32F_STENCIL8'],
      [12,'RGB32F RGB32I RGB32UI'],[16,'RGBA32F RGBA32I RGBA32UI'],
    ])for(const name of names.split(' '))if(gl[name]!==undefined)sized.set(gl[name],bytes);
    const channels=new Map();
    for(const [count,names]of [[1,'RED RED_INTEGER ALPHA LUMINANCE DEPTH_COMPONENT'],[2,'RG RG_INTEGER LUMINANCE_ALPHA'],[3,'RGB RGB_INTEGER'],[4,'RGBA RGBA_INTEGER']])
      for(const name of names.split(' '))if(gl[name]!==undefined)channels.set(gl[name],count);
    const scalar=new Map();
    for(const [bytes,names]of [[1,'UNSIGNED_BYTE BYTE'],[2,'UNSIGNED_SHORT SHORT HALF_FLOAT'],[4,'UNSIGNED_INT INT FLOAT']])
      for(const name of names.split(' '))if(gl[name]!==undefined)scalar.set(gl[name],bytes);
    scalar.set(0x8D61,2); // OES_texture_half_float uses this fixed extension enum.
    const packed=new Map();
    for(const [bytes,names]of [[2,'UNSIGNED_SHORT_5_6_5 UNSIGNED_SHORT_4_4_4_4 UNSIGNED_SHORT_5_5_5_1'],
      [4,'UNSIGNED_INT_2_10_10_10_REV UNSIGNED_INT_10F_11F_11F_REV UNSIGNED_INT_5_9_9_9_REV UNSIGNED_INT_24_8'],
      [8,'FLOAT_32_UNSIGNED_INT_24_8_REV']])for(const name of names.split(' '))if(gl[name]!==undefined)packed.set(gl[name],bytes);
    const cubeFaces=['TEXTURE_CUBE_MAP_POSITIVE_X','TEXTURE_CUBE_MAP_NEGATIVE_X','TEXTURE_CUBE_MAP_POSITIVE_Y',
      'TEXTURE_CUBE_MAP_NEGATIVE_Y','TEXTURE_CUBE_MAP_POSITIVE_Z','TEXTURE_CUBE_MAP_NEGATIVE_Z'].map(name=>gl[name]).filter(value=>value!==undefined);
    function family(target){
      if(is(target,'TEXTURE_2D'))return {target:gl.TEXTURE_2D,binding:gl.TEXTURE_BINDING_2D,faces:[gl.TEXTURE_2D]};
      if(is(target,'TEXTURE_CUBE_MAP')||cubeFaces.includes(target))return {target:gl.TEXTURE_CUBE_MAP,binding:gl.TEXTURE_BINDING_CUBE_MAP,faces:cubeFaces};
      return null;
    }
    function note(reason){reasons.add(reason);calls.unmeasured++;peakUnknown=true;}
    function textureInfo(texture,preexisting=false){
      if(!texture)return null;
      if(!textures.has(texture)){textures.set(texture,{id:nextId++,levels:new Map(),immutable:false,preexisting});
        if(preexisting)note('Texture existed before probe creation or was not observed being created');}
      return textures.get(texture);
    }
    function bound(target){
      const f=family(target);
      if(!f||f.binding===undefined){unattributed++;note('Unsupported texture allocation target');return null;}
      // Query the binding on the real currently active texture unit. Do not
      // mirror activeTexture/bindTexture calls into a second state machine.
      return textureInfo(gl.getParameter(f.binding),true);
    }
    function recount(){
      known=0;for(const texture of textures.values())for(const level of texture.levels.values())if(level.bytes!==null)known+=level.bytes;
      peakKnown=Math.max(peakKnown,known);
    }
    function bytesPerTexel(internal,format,type,storage=false){
      if(storage)return sized.get(internal)??null;
      if(!channels.has(format)&&!is(format,'DEPTH_STENCIL'))return null;
      if(!scalar.has(type)&&!packed.has(type))return null;
      if(sized.has(internal))return sized.get(internal);
      if(internal!==format)return null;
      if(packed.has(type)){
        const valid=is(type,'UNSIGNED_SHORT_5_6_5')||is(type,'UNSIGNED_INT_10F_11F_11F_REV')||is(type,'UNSIGNED_INT_5_9_9_9_REV')
          ? is(format,'RGB') : is(type,'UNSIGNED_INT_24_8')||is(type,'FLOAT_32_UNSIGNED_INT_24_8_REV')
            ? is(format,'DEPTH_STENCIL') : is(format,'RGBA')||is(format,'RGBA_INTEGER');
        return valid?packed.get(type):null;
      }
      return channels.has(format)?channels.get(format)*scalar.get(type):null;
    }
    function put(texture,face,level,width,height,bpp,internal){
      const valid=[level,width,height].every(value=>Number.isSafeInteger(value)&&value>=0);
      const bytes=valid&&bpp!==null&&Number.isSafeInteger(width*height*bpp)?width*height*bpp:null;
      if(bytes===null)note('Unmeasured texture format, dimensions or level');
      texture.levels.set(face+':'+level,{face,level,width,height,bytes_per_texel:bpp,internal_format:internal,bytes});
    }
    function imageDimensions(source){
      if(!source||typeof source!=='object')return [null,null];
      for(const [w,h]of [['videoWidth','videoHeight'],['naturalWidth','naturalHeight'],['displayWidth','displayHeight'],['width','height']])
        if(w in source&&h in source)return [source[w],source[h]];
      return [null,null];
    }
    const patch=(name,after)=>{
      if(typeof gl[name]!=='function')return;
      const original=gl[name];originals.set(name,original);
      const wrapped=function(...args){
        const result=original.apply(this,args); // preserve returns and throws
        if(active&&!gl.isContextLost())try{after(args,result);recount();}catch(error){
          calls.probe_errors++;unattributed++;note('Probe could not observe allocation: '+String(error.message||error));
        }
        return result;
      };
      wrappers.set(name,wrapped);gl[name]=wrapped;
    };
    patch('createTexture',(_,texture)=>textureInfo(texture));
    patch('deleteTexture',([texture])=>textures.delete(texture));
    patch('bindTexture',([target])=>{const f=family(target);if(f&&f.binding!==undefined)textureInfo(gl.getParameter(f.binding),true);});
    patch('texImage2D',args=>{
      calls.tex_image_2d++;const [target,level,internal]=args,texture=bound(target);if(!texture)return;
      const six=args.length===6,[width,height]=six?imageDimensions(args[5]):[args[3],args[4]],format=args[six?3:6],type=args[six?4:7];
      // Immutable storage cannot be redefined by texImage2D. The attempted call
      // is still counted; its driver error remains untouched for the caller.
      if(texture.immutable)return;
      put(texture,target,level,width,height,bytesPerTexel(internal,format,type),internal);
    });
    patch('texStorage2D',([target,levels,internal,width,height])=>{
      calls.tex_storage_2d++;const texture=bound(target),f=family(target);if(!texture||!f)return;
      if(texture.immutable)return;
      if(!Number.isInteger(levels)||levels<1||levels>32||![width,height].every(value=>Number.isSafeInteger(value)&&value>0)
        ||levels>Math.floor(Math.log2(Math.max(width,height)))+1){
        texture.preexisting=true;note('Unmeasured immutable texture dimensions or level count');return;
      }
      texture.levels.clear();texture.preexisting=false;texture.immutable=true;
      const bpp=bytesPerTexel(internal,null,null,true);
      for(const face of f.faces)for(let level=0;level<levels;level++)put(texture,face,level,Math.max(1,Math.floor(width/2**level)),Math.max(1,Math.floor(height/2**level)),bpp,internal);
    });
    patch('generateMipmap',([target])=>{
      calls.generate_mipmap++;const texture=bound(target),f=family(target);if(!texture||!f||texture.immutable)return;
      const parameter=(name,fallback)=>gl[name]!==undefined&&typeof gl.getTexParameter==='function'?gl.getTexParameter(f.target,gl[name]):fallback;
      const base=parameter('TEXTURE_BASE_LEVEL',0),max=parameter('TEXTURE_MAX_LEVEL',1000);
      for(const face of f.faces){
        const source=texture.levels.get(face+':'+base);
        if(!source||!Number.isInteger(base)||!Number.isInteger(max)||max<base){texture.preexisting=true;note('Mipmap generation has an unobserved base level or range');continue;}
        if(![source.width,source.height].every(value=>Number.isSafeInteger(value)&&value>0)){
          texture.preexisting=true;note('Mipmap generation has unmeasured base dimensions');continue;
        }
        const last=Math.min(max,base+Math.floor(Math.log2(Math.max(source.width,source.height))));
        for(let level=base+1;level<=last;level++)put(texture,face,level,Math.max(1,Math.floor(source.width/2**(level-base))),
          Math.max(1,Math.floor(source.height/2**(level-base))),source.bytes_per_texel,source.internal_format);
      }
    });
    const lost=()=>{textures.clear();known=0;unattributed=0;calls.context_losses++;};
    gl.canvas?.addEventListener('webglcontextlost',lost);
    const api={
      snapshot(){
        let levels=0,unknown=unattributed;
        for(const texture of textures.values()){if(texture.preexisting)unknown++;for(const level of texture.levels.values()){levels++;if(level.bytes===null)unknown++;}}
        return {live_textures:textures.size,allocated_texture_levels:levels,known_texture_texel_payload_bytes:known,
          declared_texture_texel_payload_bytes:unknown?null:known,peak_known_texture_texel_payload_bytes:peakKnown,
          peak_declared_texture_texel_payload_bytes:peakUnknown?null:peakKnown,unmeasured_texture_allocations:unknown,
          unmeasured_allocation_events:calls.unmeasured,tex_image_2d_calls:calls.tex_image_2d,tex_storage_2d_calls:calls.tex_storage_2d,
          generate_mipmap_calls:calls.generate_mipmap,context_losses:calls.context_losses,probe_errors:calls.probe_errors,method};
      },
      diagnostics(){return [...reasons];},
      stop(){if(!active)return;active=false;for(const [name,original]of originals)if(gl[name]===wrappers.get(name))gl[name]=original;
        gl.canvas?.removeEventListener('webglcontextlost',lost);attached.delete(gl);},
    };
    attached.set(gl,api);return api;
  }
  const api={attach,method};
  if(typeof module==='object'&&module.exports)module.exports=api;else root.WebGLTextureMeasurement=api;
})(typeof globalThis==='undefined'?this:globalThis);
