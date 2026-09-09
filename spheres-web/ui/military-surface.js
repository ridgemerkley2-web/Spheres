/* Local military presentation shader. No external requests, game state or simulation effects.
 * Per-part material weights are estimated from RGB. Shared CC0 normal/roughness
 * maps add finish detail without model UVs. Skin, buildings and terrain keep
 * their own shading paths; geometry/export colours stay unchanged by lighting. */
(function(root,factory){const api=factory(root);if(typeof module==='object'&&module.exports)module.exports=api;else root.MilitarySurface=api;})(typeof globalThis!=='undefined'?globalThis:this,function(root){
  'use strict';
  // Resolve once from this module, so embedded game routes and the static art
  // workshop both request their own local copies. Never fetch provider/CDN URLs.
  const moduleUrl=root.document?.currentScript?.src;
  const files=['military-textures/paint-normal.jpg','military-textures/paint-roughness.jpg'];
  const glsl=`
uniform float uSurfaceMaps;
uniform sampler2D uSurfaceNormal;
uniform sampler2D uSurfaceRoughness;
#if __VERSION__ >= 300
#define MIL_TEXTURE texture
#else
#define MIL_TEXTURE texture2D
#endif
vec4 milTexture(vec3 p,vec3 n){
 vec3 w=pow(abs(n),vec3(6.));w/=max(w.x+w.y+w.z,.0001);
 // The source material covers 2.5 metres; avoid shrinking its seams into a grid.
 vec3 s=sign(n),delta=vec3(0.);float rough=0.;p*=.4;
 if(w.x>.001){vec2 uv=vec2(-p.z*s.x,p.y);vec3 t=MIL_TEXTURE(uSurfaceNormal,uv).xyz*2.-1.;delta+=vec3(0.,t.y,-t.x*s.x)*w.x;rough+=MIL_TEXTURE(uSurfaceRoughness,uv).r*w.x;}
 if(w.y>.001){vec2 uv=vec2(p.x,-p.z*s.y);vec3 t=MIL_TEXTURE(uSurfaceNormal,uv).xyz*2.-1.;delta+=vec3(t.x,0.,-t.y*s.y)*w.y;rough+=MIL_TEXTURE(uSurfaceRoughness,uv).r*w.y;}
 if(w.z>.001){vec2 uv=vec2(p.x*s.z,p.y);vec3 t=MIL_TEXTURE(uSurfaceNormal,uv).xyz*2.-1.;delta+=vec3(t.x*s.z,t.y,0.)*w.z;rough+=MIL_TEXTURE(uSurfaceRoughness,uv).r*w.z;}
 return vec4(delta-n*dot(delta,n),rough);
}
float milHash(vec3 p){p=fract(p*.1031);p+=dot(p,p.yzx+33.33);return fract((p.x+p.y)*p.z);}
float milNoise(vec3 p){vec3 i=floor(p),f=fract(p);f=f*f*(3.-2.*f);
 return mix(mix(mix(milHash(i),milHash(i+vec3(1,0,0)),f.x),mix(milHash(i+vec3(0,1,0)),milHash(i+vec3(1,1,0)),f.x),f.y),mix(mix(milHash(i+vec3(0,0,1)),milHash(i+vec3(1,0,1)),f.x),mix(milHash(i+vec3(0,1,1)),milHash(i+vec3(1,1,1)),f.x),f.y),f.z);}
// x: rubber, y: glazing, z: bare-metal response. Paint remains dielectric.
vec3 milWeights(vec3 c){float lo=min(c.r,min(c.g,c.b)),hi=max(c.r,max(c.g,c.b)),lum=dot(c,vec3(.25,.65,.10));
 float rubber=1.-smoothstep(.13,.205,lum);
 float glass=smoothstep(.06,.14,max(c.g,c.b)-c.r)*(1.-smoothstep(.28,.38,c.r))*smoothstep(.17,.26,max(c.g,c.b));
 float neutral=1.-smoothstep(.022,.085,hi-lo);
 float darkSteel=smoothstep(.17,.23,lum)*(1.-smoothstep(.34,.42,lum));
 float brightSteel=smoothstep(.60,.67,lum)*(1.-smoothstep(.74,.82,lum));
 float metal=neutral*max(darkSteel,brightSteel)*(1.-rubber)*(1.-glass);
 return vec3(rubber*(1.-glass),glass,metal);}
vec3 milFresnel(float cosine,vec3 f0){return f0+(1.-f0)*pow(1.-clamp(cosine,0.,1.),5.);}
vec3 milBRDF(vec3 base,vec3 n,vec3 v,vec3 l,float rough,float metal,vec3 radiance){
 vec3 h=normalize(v+l);float nv=max(dot(n,v),.001),nl=max(dot(n,l),0.),nh=max(dot(n,h),0.);
 float a=rough*rough,a2=a*a,d=nh*nh*(a2-1.)+1.;float distribution=a2/max(3.14159265*d*d,.00001);
 float k=(rough+1.)*(rough+1.)*.125;float geometry=nv/(nv*(1.-k)+k)*nl/max(nl*(1.-k)+k,.001);
 vec3 f=milFresnel(max(dot(h,v),0.),mix(vec3(.04),base,metal));
 return ((1.-f)*(1.-metal)*base/3.14159265+distribution*geometry*f/max(4.*nv*nl,.001))*radiance*nl;
}
vec3 milTone(vec3 c){return clamp((c*(2.51*c+.03))/(c*(2.43*c+.59)+.14),0.,1.);}
vec3 militaryLighting(vec3 rgb,vec3 normal,vec3 p,vec3 view,float visibility,float height){
 vec3 n=normalize(normal),v=normalize(view);if(dot(n,v)<0.)n=-n;
 vec3 weights=milWeights(rgb);float rubber=weights.x,glass=weights.y,metal=weights.z*.82;
 // Broad deposits follow physical model-space scale, never triangle boundaries.
 float broad=milNoise(p*2.7),grain=milNoise(p*77.);
 float paint=(1.-rubber)*(1.-glass)*(1.-metal);
 float low=1.-smoothstep(.02,max(.24,height*.24),p.y);
 float dust=low*(.045+.07*broad)*(1.-glass);
 vec3 albedo=mix(rgb*(.94+.07*broad),vec3(.31,.275,.205),dust);
 albedo*=1.+paint*(grain-.5)*.035;
 vec3 base=pow(max(albedo,vec3(.002)),vec3(2.2));
 float rough=clamp(.63+paint*(broad-.5)*.10+rubber*.26-metal*.24-glass*.43,.17,.94);
 // Original CC0 material maps are projected in model space without changing
 // geometry or exported vertex colours. Glazing and tyres keep smooth surfaces.
 if(uSurfaceMaps>.5&&paint>.10){vec4 detail=milTexture(p,n);n=normalize(n+detail.xyz*.24*paint);rough=clamp(rough+(detail.w-.5)*.26*paint,.17,.94);}
 vec3 key=normalize(vec3(-.55,.85,.65)),fill=normalize(vec3(.7,.35,-.55));
 vec3 lit=milBRDF(base,n,v,key,rough,metal,vec3(3.9,3.65,3.25))*visibility;
 lit+=milBRDF(base,n,v,fill,rough,metal,vec3(.68,.83,1.05));
 float sky=smoothstep(-.55,.85,n.y),ao=mix(.70,1.,smoothstep(.035,max(.25,height*.35),p.y));
 lit+=base*mix(vec3(.055,.060,.07),vec3(.29,.35,.43),sky)*ao*(1.-metal*.55);
 // Soft studio strips give curved optics and exposed metal an environment to reflect.
 vec3 r=reflect(-v,n);float strip=pow(max(dot(r,normalize(vec3(.15,.70,.70))),0.),mix(7.,70.,1.-rough));
 vec3 fresnel=milFresnel(max(dot(n,v),0.),mix(vec3(.04),base,metal));
 lit+=fresnel*(.045+strip*1.4)*(1.-rough*.5)*ao;
 lit+=glass*vec3(.035,.065,.075)*sky;
 return pow(milTone(max(lit,vec3(0.))),vec3(1./2.2));
}`;
  function create(gl,onReady){
    const methods=['createTexture','deleteTexture','bindTexture','texImage2D','texParameteri','generateMipmap','activeTexture','getParameter','pixelStorei','getUniformLocation','uniform1i','uniform1f'];
    if(!moduleUrl||typeof root.Image!=='function'||typeof root.URL!=='function'||methods.some(name=>typeof gl[name]!=='function'))return null;
    let sources;
    try{const page=root.location?.href||root.document?.baseURI;if(!page)return null;const base=new root.URL(moduleUrl);sources=files.map(file=>new root.URL(file,base));const units=gl.getParameter(gl.MAX_TEXTURE_IMAGE_UNITS),limit=gl.getParameter(gl.MAX_TEXTURE_SIZE);if(!['http:','https:'].includes(base.protocol)||base.origin!==new root.URL(page).origin||sources.some(url=>url.origin!==base.origin)||!Number.isFinite(units)||units<3||!Number.isFinite(limit)||limit<1024)return null;}
    catch(error){return null;}
    let closed=false,failed=false,loaded=0,initializing=true;
    const textures=[],images=[],locations=new WeakMap();
    function release(contextLost=false){
      images.forEach(img=>{if(img){img.onload=null;img.onerror=null;}});images.length=0;
      if(!contextLost)textures.forEach(texture=>gl.deleteTexture(texture));
      textures.length=0;
    }
    function fail(){
      if(closed||failed)return;failed=true;release();
      // Creation can happen inside a shared renderer's in-progress draw. An
      // immediate allocation failure must not re-enter it and overwrite its
      // viewport/uniforms. There is no earlier textured frame to invalidate yet.
      if(!initializing)onReady?.();
    }
    function upload(index,image){
      const active=gl.getParameter(gl.ACTIVE_TEXTURE);
      gl.activeTexture(gl.TEXTURE0+1+index);
      const previous=gl.getParameter(gl.TEXTURE_BINDING_2D),flip=gl.getParameter(gl.UNPACK_FLIP_Y_WEBGL),premultiply=gl.getParameter(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL),conversion=gl.getParameter(gl.UNPACK_COLORSPACE_CONVERSION_WEBGL);
      try{
        gl.bindTexture(gl.TEXTURE_2D,textures[index]);
        gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL,true);gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL,false);gl.pixelStorei(gl.UNPACK_COLORSPACE_CONVERSION_WEBGL,gl.NONE);
        if(image){gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,gl.RGBA,gl.UNSIGNED_BYTE,image);gl.generateMipmap(gl.TEXTURE_2D);}
        else gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,1,1,0,gl.RGBA,gl.UNSIGNED_BYTE,new Uint8Array(index?[128,128,128,255]:[128,128,255,255]));
        gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MIN_FILTER,image?gl.LINEAR_MIPMAP_LINEAR:gl.LINEAR);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MAG_FILTER,gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_WRAP_S,gl.REPEAT);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_WRAP_T,gl.REPEAT);
        if(gl.getError&&gl.getError()!==gl.NO_ERROR)throw new Error('Material texture upload failed');
      }finally{
        gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL,flip);gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL,premultiply);gl.pixelStorei(gl.UNPACK_COLORSPACE_CONVERSION_WEBGL,conversion);
        gl.bindTexture(gl.TEXTURE_2D,previous);gl.activeTexture(active);
      }
    }
    try{
      for(let i=0;i<2;i++){const texture=gl.createTexture();if(!texture)throw new Error('Material texture allocation failed');textures.push(texture);upload(i,null);}
      sources.forEach((url,index)=>{
        if(failed||closed)return;
        const image=new root.Image();images.push(image);
        image.onload=()=>{if(closed||failed)return;try{if(image.naturalWidth!==1024||image.naturalHeight!==1024)throw new Error('Material dimensions exceed the 1K budget');upload(index,image);image.onload=null;image.onerror=null;images[index]=null;loaded++;if(loaded===2)onReady?.();}catch(error){fail();}};
        image.onerror=fail;image.src=url.href;
      });
    }catch(error){fail();}
    initializing=false;
    return {
      bind(program){
        if(closed)return;let u=locations.get(program);
        if(!u){u={enabled:gl.getUniformLocation(program,'uSurfaceMaps'),normal:gl.getUniformLocation(program,'uSurfaceNormal'),rough:gl.getUniformLocation(program,'uSurfaceRoughness')};locations.set(program,u);}
        gl.uniform1f(u.enabled,!failed&&loaded===2?1:0);
        if(failed)return;
        const active=gl.getParameter(gl.ACTIVE_TEXTURE);
        textures.forEach((texture,index)=>{gl.activeTexture(gl.TEXTURE0+1+index);gl.bindTexture(gl.TEXTURE_2D,texture);});gl.activeTexture(active);
        gl.uniform1i(u.normal,1);gl.uniform1i(u.rough,2);
      },
      dispose(contextLost=false){if(closed)return;closed=true;release(contextLost);},
      get state(){return closed?'disposed':failed?'fallback':loaded===2?'ready':'loading';}
    };
  }
  return Object.freeze({glsl,create,version:2});
});
