/* Original equipment mesh viewer. Visual state never enters a simulation command.
   Uses WebGL triangles, perspective/depth and cached directional mesh shadows.
   No CDN, remote assets, timers while hidden, or shared state with the globe. */
(function(root,factory){const api=factory(root);if(typeof module==='object'&&module.exports)module.exports=api;else root.EquipmentModel=api;})(typeof globalThis!=='undefined'?globalThis:this,function(root){
  'use strict';
  const clamp=(v,a,b)=>Math.max(a,Math.min(b,v));
  const VIEWS={hero:[0.65,0.36],front:[0,0.13],side:[Math.PI/2,0.16],rear:[Math.PI,0.22],top:[0,1.49]};
  const FINISHES={olive:null,sand:[0.58,0.48,0.30],winter:[0.63,0.68,0.65]};
  function normalize(a){const l=Math.hypot(...a)||1;return a.map(v=>v/l);}
  function cross(a,b){return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];}
  function dot(a,b){return a.reduce((s,v,i)=>s+v*b[i],0);}
  function lookAt(eye,target){
    const z=normalize(eye.map((v,i)=>v-target[i])),x=normalize(cross([0,1,0],z)),y=cross(z,x);
    return new Float32Array([x[0],y[0],z[0],0,x[1],y[1],z[1],0,x[2],y[2],z[2],0,-dot(x,eye),-dot(y,eye),-dot(z,eye),1]);
  }
  function perspective(fov,aspect,near,far){const f=1/Math.tan(fov/2),d=1/(near-far);return new Float32Array([f/aspect,0,0,0,0,f,0,0,0,0,(far+near)*d,-1,0,0,2*far*near*d,0]);}
  function multiply(a,b){const out=new Float32Array(16);for(let c=0;c<4;c++)for(let r=0;r<4;r++)for(let k=0;k<4;k++)out[c*4+r]+=a[k*4+r]*b[c*4+k];return out;}
  const KEY_LIGHT=normalize([-.55,.85,.65]);
  function shadowFrame(bounds,resolution){
    // Include the ground projection of every corner, so tall radar, wings and
    // long barrels retain their shadows without spending texels on empty sky.
    const points=[];
    for(const x of [bounds.min[0],bounds.max[0]])for(const y of [bounds.min[1],bounds.max[1]])for(const z of [bounds.min[2],bounds.max[2]]){
      points.push([x,y,z],[x-KEY_LIGHT[0]*y/KEY_LIGHT[1],0,z-KEY_LIGHT[2]*y/KEY_LIGHT[1]]);
    }
    const center=[0,1,2].map(i=>(Math.min(...points.map(p=>p[i]))+Math.max(...points.map(p=>p[i])))/2);
    const radius=Math.max(.1,...points.map(p=>Math.hypot(...p.map((v,i)=>v-center[i]))));
    const view=lookAt(center.map((v,i)=>v+KEY_LIGHT[i]*(radius*3+1)),center);
    const lightPoints=points.map(p=>[0,1,2].map(r=>view[r]*p[0]+view[4+r]*p[1]+view[8+r]*p[2]+view[12+r]));
    const pad=Math.max(.05,radius*.045),lo=[0,1,2].map(i=>Math.min(...lightPoints.map(p=>p[i]))-pad),hi=[0,1,2].map(i=>Math.max(...lightPoints.map(p=>p[i]))+pad);
    const span=hi.map((v,i)=>Math.max(.1,v-lo[i]));
    const projection=new Float32Array([2/span[0],0,0,0,0,2/span[1],0,0,0,0,-2/span[2],0,-(hi[0]+lo[0])/span[0],-(hi[1]+lo[1])/span[1],(hi[2]+lo[2])/span[2],1]);
    const texelWorld=Math.max(span[0],span[1])/resolution;
    return {matrix:multiply(projection,view),normalBias:texelWorld*.75,depthBias:Math.max(.000015,texelWorld*.12/span[2]),floorExtent:Math.max(24,...points.map(p=>Math.max(Math.abs(p[0]),Math.abs(p[2]))+radius*.5))};
  }
  function modelKey(spec){const c=spec?.components||{},asked=Math.round(Number(spec?.lod)),lod=Number.isFinite(asked)?Math.min(2,Math.max(0,asked)):0;return JSON.stringify([spec?.platform,lod,...Object.keys(c).sort().map(k=>[k,c[k]])]);}
  function frame(bounds,aspect,yaw,pitch,zoom=1){
    const size=bounds.max.map((v,i)=>v-bounds.min[i]);
    const center=bounds.min.map((v,i)=>(v+bounds.max[i])/2);
    const radius=Math.hypot(...size)/2;
    const direction=[Math.sin(yaw)*Math.cos(pitch),Math.sin(pitch),Math.cos(yaw)*Math.cos(pitch)];
    const right=normalize(cross([0,1,0],direction)),up=cross(direction,right),tangent=Math.tan(38*Math.PI/360);
    // Fit the projected corners, so a long barrel does not leave the whole tank tiny.
    let fit=radius;
    for(const x of [-1,1])for(const y of [-1,1])for(const z of [-1,1]){
      const corner=[x*size[0]/2,y*size[1]/2,z*size[2]/2];
      fit=Math.max(fit,dot(corner,direction)+Math.max(Math.abs(dot(corner,right))/(tangent*Math.max(.25,aspect)),Math.abs(dot(corner,up))/tangent));
    }
    const distance=fit*1.10*zoom;
    const eye=center.map((v,i)=>v+direction[i]*distance);
    return {eye,center,distance,radius,matrix:multiply(perspective(38*Math.PI/180,Math.max(.25,aspect),.08,Math.max(100,distance+radius*4)),lookAt(eye,center))};
  }
  function finishColors(colors,finish){
    const tone=FINISHES[finish];if(!tone)return colors;
    const out=new Float32Array(colors);
    for(let i=0;i<out.length;i+=3){const r=colors[i],g=colors[i+1],b=colors[i+2];
      // Legacy non-armour palette conversion. Armoured vehicles use TankSurface's explicit
      // material mask, since green-biased steel must never be treated as paint.
      if(g>r*1.025&&g>b*1.06&&g>.12){const shade=clamp((r*.25+g*.65+b*.10)/.30,.4,1.5);for(let k=0;k<3;k++)out[i+k]=clamp(tone[k]*shade,0,1);}
    }return out;
  }
  // Surface metadata is authored in vertex units, like selectable part ranges.
  // Reject ambiguous ownership rather than render stale or opaque canopy data.
  function surfaceRanges(mesh){
    const count=mesh.positions.length/3;
    if(mesh.assetKind!=='aircraft')return {opaque:[{first:0,count}],glass:[]};
    if(mesh.surfaces!==undefined&&!Array.isArray(mesh.surfaces))throw new TypeError('Invalid aircraft surface ranges.');
    const glass=(mesh.surfaces||[]).map(surface=>{
      if(!surface||surface.material!=='glass'||!Number.isSafeInteger(surface.first)||!Number.isSafeInteger(surface.count)||surface.first<0||surface.count<=0||surface.first%3||surface.count%3||surface.first+surface.count>count||!Number.isFinite(surface.opacity)||surface.opacity<=0||surface.opacity>=1)throw new TypeError('Invalid aircraft surface ranges.');
      const owners=(mesh.parts||[]).filter(p=>Number.isSafeInteger(p.first)&&Number.isSafeInteger(p.count)&&p.first>=0&&p.count>0&&p.first%3===0&&p.count%3===0&&p.first+p.count<=count&&surface.first<p.first+p.count&&surface.first+surface.count>p.first);
      if(owners.length!==1||surface.first<owners[0].first||surface.first+surface.count>owners[0].first+owners[0].count)throw new TypeError('Aircraft glass must belong to exactly one model part.');
      const center=[0,0,0];for(let i=surface.first*3;i<(surface.first+surface.count)*3;i+=3)for(let axis=0;axis<3;axis++)center[axis]+=mesh.positions[i+axis]/surface.count;
      return {first:surface.first,count:surface.count,material:'glass',opacity:surface.opacity,part:owners[0].name,center};
    }).sort((a,b)=>a.first-b.first);
    const opaque=[];let first=0;
    for(const surface of glass){if(surface.first<first)throw new TypeError('Aircraft surface ranges overlap.');if(surface.first>first)opaque.push({first,count:surface.first-first});first=surface.first+surface.count;}
    if(first<count)opaque.push({first,count:count-first});
    return {opaque,glass};
  }
  function partBounds(mesh,value,region){
    if(!mesh?.positions||!Array.isArray(mesh.parts)||typeof value!=='string'||!value)return null;
    const exact=mesh.parts.find(p=>p.name===value);let parts=exact?[exact]:mesh.parts.filter(p=>p.slot===value);
    if(!parts.length)return null;
    if(!exact&&parts[0].slot==='air_engine'&&region==='front'&&parts.length>1){
      // Looking between both intakes puts the eye inside the nose. Inspect the
      // outboard positive-X nacelle, leaving the complete airframe rendered.
      const candidates=parts.map(part=>({part,bounds:partBounds(mesh,part.name)?.bounds})).filter(p=>p.bounds).sort((a,b)=>(b.bounds.min[0]+b.bounds.max[0])-(a.bounds.min[0]+a.bounds.max[0]));
      if(!candidates.length)return null;parts=[candidates[0].part];
    }
    let ranges=parts;
    // The avionics assembly includes radio aerials elsewhere on the airframe.
    // Authored canopy triangles identify the cockpit itself at every mesh LOD.
    if(parts[0].slot==='air_avionics'&&mesh.assetKind==='aircraft'){
      const glass=surfaceRanges(mesh).glass.filter(s=>parts.some(p=>p.name===s.part));if(glass.length)ranges=glass;
    }
    const min=[Infinity,Infinity,Infinity],max=[-Infinity,-Infinity,-Infinity],count=mesh.positions.length/3;
    for(const part of ranges){
      if(!Number.isSafeInteger(part.first)||!Number.isSafeInteger(part.count)||part.first<0||part.count<=0||part.first%3||part.count%3||part.first+part.count>count)return null;
      for(let i=part.first*3;i<(part.first+part.count)*3;i++){const v=mesh.positions[i];if(!Number.isFinite(v))return null;min[i%3]=Math.min(min[i%3],v);max[i%3]=Math.max(max[i%3],v);}
    }
    if(parts[0].slot==='air_engine'&&(region==='rear'||region==='front')){
      const depth=Math.min(1.6,(max[2]-min[2])*.22),edge=region==='rear'?min[2]+depth:max[2]-depth;
      min.fill(Infinity);max.fill(-Infinity);
      for(const part of ranges)for(let i=part.first*3;i<(part.first+part.count)*3;i+=3){
        const z=mesh.positions[i+2];if(region==='rear'?z>edge:z<edge)continue;
        for(let axis=0;axis<3;axis++){min[axis]=Math.min(min[axis],mesh.positions[i+axis]);max[axis]=Math.max(max[axis],mesh.positions[i+axis]);}
      }
    }
    if(![...min,...max].every(Number.isFinite)||Math.hypot(...max.map((v,i)=>v-min[i]))<.001)return null;
    return {bounds:{min,max},part:parts[0],parts};
  }
  function rayAt(camera,x,y,aspect){
    const forward=normalize(camera.center.map((v,i)=>v-camera.eye[i])),right=normalize(cross(forward,[0,1,0])),up=cross(right,forward),tangent=Math.tan(38*Math.PI/360);
    return {origin:camera.eye,direction:normalize(forward.map((v,i)=>v+right[i]*x*tangent*Math.max(.25,aspect)+up[i]*y*tangent))};
  }
  // Moller-Trumbore against the rendered triangles; the nearest surface wins,
  // including non-selectable surfaces, so hidden parts cannot be picked through.
  function raycast(mesh,origin,direction){
    if(!mesh?.positions||![...origin,...direction].every(Number.isFinite))return null;
    const p=mesh.positions;let nearest=Infinity,vertex=-1;
    for(let i=0;i<p.length;i+=9){
      const ax=p[i],ay=p[i+1],az=p[i+2],e1x=p[i+3]-ax,e1y=p[i+4]-ay,e1z=p[i+5]-az,e2x=p[i+6]-ax,e2y=p[i+7]-ay,e2z=p[i+8]-az;
      const hx=direction[1]*e2z-direction[2]*e2y,hy=direction[2]*e2x-direction[0]*e2z,hz=direction[0]*e2y-direction[1]*e2x,det=e1x*hx+e1y*hy+e1z*hz;
      if(Math.abs(det)<1e-10)continue;
      const inv=1/det,sx=origin[0]-ax,sy=origin[1]-ay,sz=origin[2]-az,u=(sx*hx+sy*hy+sz*hz)*inv;
      if(u<0||u>1)continue;
      const qx=sy*e1z-sz*e1y,qy=sz*e1x-sx*e1z,qz=sx*e1y-sy*e1x,v=(direction[0]*qx+direction[1]*qy+direction[2]*qz)*inv;
      if(v<0||u+v>1)continue;
      const t=(e2x*qx+e2y*qy+e2z*qz)*inv;
      if(t>1e-6&&t<nearest){nearest=t;vertex=i/3;}
    }
    if(vertex<0)return null;
    return {part:mesh.parts?.find(part=>vertex>=part.first&&vertex<part.first+part.count)||null,vertex,distance:nearest,point:origin.map((v,i)=>v+direction[i]*nearest)};
  }
  const VERTEX=`precision highp float;
attribute vec3 aPosition;attribute vec3 aNormal;attribute vec3 aColor;attribute vec4 aSurface;
uniform mat4 uVP;uniform mediump float uMode;
varying highp vec3 vPosition;varying mediump vec3 vNormal;varying mediump vec3 vColor;varying mediump vec4 vSurface;
void main(){vec3 p=aPosition;if(uMode>1.5){p=vec3(p.x+p.y*.55,.016,p.z-p.y*.65);}vPosition=p;vNormal=aNormal;vColor=aColor;vSurface=aSurface;gl_Position=uVP*vec4(p,1.);}`;
  const SHADOW_VERTEX=`precision highp float;
attribute vec3 aPosition;uniform mat4 uLightVP;
void main(){gl_Position=uLightVP*vec4(aPosition,1.);}`;
  const SHADOW_FRAGMENT=`precision highp float;
void main(){vec4 depth=fract(min(gl_FragCoord.z,.9999999)*vec4(16777216.,65536.,256.,1.));depth-=depth.xxyz*vec4(0.,.00390625,.00390625,.00390625);gl_FragColor=depth*(256./255.);}`;
  const SHADOW_SAMPLE=`uniform sampler2D uShadowMap;uniform mat4 uLightVP;
uniform float uShadowTexel;uniform float uShadowNormalBias;uniform float uShadowDepthBias;
float shadowVisibility(vec3 position,vec3 normal){
  vec3 n=normalize(normal),light=normalize(vec3(-.55,.85,.65));
  float slope=1.-abs(dot(n,light));
  vec4 clip=uLightVP*vec4(position+n*uShadowNormalBias*(.35+slope),1.);
  vec3 p=clip.xyz/clip.w*.5+.5;
  if(p.x<=uShadowTexel||p.x>=1.-uShadowTexel||p.y<=uShadowTexel||p.y>=1.-uShadowTexel||p.z<=0.||p.z>=1.)return 1.;
  float visibility=0.,depth=p.z-uShadowDepthBias*(1.+2.*slope);
  for(int y=-1;y<=1;y++)for(int x=-1;x<=1;x++){
    vec4 packed=texture2D(uShadowMap,p.xy+vec2(float(x),float(y))*uShadowTexel);
    float stored=dot(packed,vec4(.000000059604644775390625,.0000152587890625,.00390625,1.))*(255./256.);
    visibility+=step(depth,stored);
  }
  return visibility/9.;
}`;
  function fragmentSource(shadows){return `precision highp float;
uniform vec3 uEye;uniform mediump float uMode;uniform float uHighlight;uniform float uHeight;uniform float uTankEnabled;uniform float uAircraftEnabled;uniform float uAircraftGlass;uniform float uSurfaceOpacity;
varying highp vec3 vPosition;varying mediump vec3 vNormal;varying mediump vec3 vColor;varying mediump vec4 vSurface;
${shadows?SHADOW_SAMPLE:'float shadowVisibility(vec3 position,vec3 normal){return 1.;}'}
${root.MilitarySurface?.glsl||''}
${root.TankSurface?.glsl||''}
vec3 floorColor(vec3 p){float radius=length(p.xz*.095);vec3 c=uTankEnabled>.5?mix(vec3(.205,.220,.218),vec3(.105,.125,.132),smoothstep(.1,1.3,radius)):mix(vec3(.135,.172,.177),vec3(.063,.088,.105),smoothstep(.1,1.3,radius));
float grid=pow(abs(cos(p.x*3.14159265)),80.)+pow(abs(cos(p.z*3.14159265)),80.);c+=vec3(.008)*grid*(1.-smoothstep(4.,12.,length(p.xz)));
float contact=exp(-pow(p.x/1.9,4.)-pow(p.z/3.7,4.));return c*(1.-.28*contact);}
void main(){if(uMode>.5){vec3 c=floorColor(vPosition);if(uMode>1.5)c*=.53;else c*=mix(.48,1.,shadowVisibility(vPosition,vec3(0.,1.,0.)));gl_FragColor=vec4(c,1.);return;}
float visibility=shadowVisibility(vPosition,vNormal);
if(uAircraftEnabled>.5){
  vec3 n=normalize(vNormal),view=normalize(uEye-vPosition),light=normalize(vec3(-.55,.85,.65));
  if(uAircraftGlass>.5){
    if(!gl_FrontFacing)n=-n;
    float fresnel=pow(1.-max(dot(n,view),0.),4.);
    float reflection=pow(max(dot(n,normalize(light+view)),0.),110.);
    vec3 sky=mix(vec3(.12,.19,.23),vec3(.66,.78,.82),n.y*.5+.5);
    vec3 glass=mix(vColor*.48,sky,.42+fresnel*.40)+vec3(.55,.62,.64)*reflection;
    glass=mix(glass,vec3(.62,.79,.80),uHighlight*.22);
    gl_FragColor=vec4(glass,clamp(uSurfaceOpacity+fresnel*.30+reflection*.22,0.,.82));return;
  }
  // Deliberately clean airframe finish: broad hangar reflections, matte paint,
  // and restrained neutral-metal highlights; no tank dirt or camouflage mask.
  float key=max(dot(n,light),0.),fill=max(dot(n,normalize(vec3(.8,.35,-.7))),0.);
  float tone=max(vColor.r,max(vColor.g,vColor.b));
  float chroma=tone-min(vColor.r,min(vColor.g,vColor.b));
  float metal=(1.-smoothstep(.025,.12,chroma))*(1.-smoothstep(.16,.30,tone));
  float highlight=pow(max(dot(n,normalize(light+view)),0.),mix(34.,78.,metal))*mix(.065,.19,metal);
  vec3 base=pow(max(vColor,vec3(.001)),vec3(2.2));
  vec3 ambient=mix(vec3(.29,.34,.39),vec3(.69,.76,.79),n.y*.5+.5);
  vec3 lit=base*(ambient*.65+vec3(1.12,1.08,.98)*key*mix(.36,1.,visibility)+vec3(.30,.38,.46)*fill)+vec3(highlight*visibility);
  vec3 painted=pow(max(lit,vec3(0.)),vec3(1./2.2));
  float rim=pow(1.-abs(dot(n,view)),2.);
  painted=mix(painted,mix(painted,vec3(.68,.80,.82),.10+rim*.35),uHighlight);
  gl_FragColor=vec4(painted,1.);return;
}
${root.TankSurface?.glsl?'if(uTankEnabled>.5){vec3 painted=tankLighting(vColor,vNormal,vPosition,uEye-vPosition,visibility,vSurface);float rim=pow(1.-abs(dot(normalize(vNormal),normalize(uEye-vPosition))),2.);painted=mix(painted,mix(painted,vec3(.65,.73,.72),.10+rim*.38),uHighlight);gl_FragColor=vec4(painted,1.);return;}':''}
${root.MilitarySurface?'vec3 painted=militaryLighting(vColor,vNormal,vPosition,uEye-vPosition,visibility,uHeight);gl_FragColor=vec4(mix(painted,painted*.70+vec3(.20,.13,.015),uHighlight),1.);return;':''}
vec3 n=normalize(vNormal),view=normalize(uEye-vPosition),light=normalize(vec3(-.55,.85,.65));
float key=max(dot(n,light),0.),fill=max(dot(n,normalize(vec3(.8,.35,-.7))),0.);
vec3 ambient=mix(vec3(.25,.29,.32),vec3(.58,.64,.65),n.y*.5+.5);
float specular=pow(max(dot(n,normalize(light+view)),0.),45.)*.19;
vec3 base=pow(max(vColor,vec3(.001)),vec3(2.2));
vec3 lit=base*(ambient*.6+vec3(1.10,1.05,.91)*key*visibility+vec3(.31,.40,.48)*fill)+vec3(specular*visibility);
float cavity=mix(.78,1.,smoothstep(.08,1.3,vPosition.y));lit*=cavity;
lit=mix(lit,lit*.65+vec3(.32,.20,.035),uHighlight);
gl_FragColor=vec4(pow(max(lit,vec3(0.)),vec3(1./2.2)),1.);}`;}

  function mount(host,spec){
    if(!host||!host.querySelector)throw new Error('A model preview host is required.');
    const doc=host.ownerDocument||root.document,slot=host.querySelector('[data-model-canvas]')||host;
    const canvas=doc.createElement('canvas');canvas.className='eq-model-canvas';canvas.tabIndex=0;
    canvas.setAttribute('data-equipment-focus','model-canvas');
    canvas.setAttribute('role','img');canvas.setAttribute('aria-label','Interactive 3D vehicle. Click a visible part to inspect its specifications. Drag to orbit; arrow keys rotate, plus and minus zoom, and Home resets. The part selector provides keyboard access.');
    canvas.style.cssText='display:block;width:100%;height:100%;touch-action:none;outline-offset:-4px;';
    slot.appendChild(canvas);
    const status=host.querySelector('[data-model-status]')||doc.createElement('p');
    if(!status.parentNode){status.setAttribute('data-model-status','');status.setAttribute('role','status');slot.appendChild(status);}
    let gl=null,program=null,buffers=[],floorBuffers=[],uniforms={},attributes={},mesh=null,painted=null,key=null,draft=spec,shadow=null,material=null,tankData=null,ranges=null;
    let disposed=false,lost=false,raf=0,turntable=false,visible=true,lastTime=0,ready=false;
    let yaw=VIEWS.hero[0],pitch=VIEWS.hero[1],zoomFactor=1,finish='olive',wear='service',viewName='hero';
    let width=0,height=0,resizeObserver=null,intersectionObserver=null,selectedPart=null,gesture=null,focus=null;
    const partSelect=host.querySelector('[data-model-part]');
    const events=[],pointers=new Map(),urls=new Set(),focusTargets=new Map();
    const on=(node,event,handler,options)=>{node.addEventListener(event,handler,options);events.push([node,event,handler,options]);};
    const say=text=>{status.textContent=text;};
    function setControls(){
      host.querySelectorAll('[data-model-view]').forEach(b=>b.setAttribute('aria-pressed',String(b.dataset.modelView===viewName)));
      host.querySelectorAll('[data-model-focus]').forEach(b=>{const region=b.dataset.modelFocusRegion||'rear';b.disabled=!ready||lost||!mesh||!focusTarget(b.dataset.modelFocus,region);b.setAttribute('aria-pressed',String(focus?.value===b.dataset.modelFocus&&focus?.region===region));b.setAttribute('title',region==='front'&&mesh?.assetKind!=='aircraft'?'Intake interior close-up is available on the tactical aircraft.':'Inspect the modeled component.');});
      host.querySelectorAll('[data-model-turntable]').forEach(b=>{b.setAttribute('aria-pressed',String(turntable));b.textContent=turntable?'Stop rotation':'Auto rotate';});
      host.querySelectorAll('[data-model-finish]').forEach(b=>{if(b.tagName==='SELECT')b.value=finish;else b.setAttribute('aria-pressed',String(b.dataset.modelFinish===finish));});
      host.querySelectorAll('[data-model-wear]').forEach(b=>{b.disabled=!tankData;if(b.tagName==='SELECT')b.value=wear;else b.setAttribute('aria-pressed',String(b.dataset.modelWear===wear));});
      // Keep the existing attribute/API names; support now includes the five
      // specialist armoured platforms with authored material metadata.
      host.querySelectorAll('[data-model-tank-only]').forEach(b=>{b.hidden=!tankData;});
      host.querySelectorAll('[data-model-export]').forEach(b=>b.disabled=!mesh||!root.EquipmentExport);
      if(partSelect){partSelect.disabled=!mesh;partSelect.value=selectedPart||'';}
    }
    function refreshParts(){
      if(!partSelect)return;
      const placeholder=doc.createElement('option');placeholder.value='';placeholder.textContent='Choose a visible part';
      const options=[placeholder,...(mesh?.parts||[]).filter(p=>p.slot).map(p=>{const option=doc.createElement('option');option.value=p.name;option.textContent=p.label||p.name;return option;})];
      partSelect.replaceChildren(...options);partSelect.value=selectedPart||'';
    }
    function selectPart(value,emit=false,target=host){
      if(disposed||!mesh)return null;
      const part=mesh.parts?.find(p=>p.name===value)||mesh.parts?.find(p=>p.slot===value);
      selectedPart=part?.name||null;setControls();schedule();
      if(part){say(`${part.label||part.name} selected. Review ${part.slot.replace(/_/g,' ')} specifications.`);
        if(emit&&part.slot&&typeof root.CustomEvent==='function')target.dispatchEvent(new root.CustomEvent('equipment-part-select',{bubbles:true,detail:{slot:part.slot,part:part.name,label:part.label||part.name}}));}
      return part||null;
    }
    function pickAt(clientX,clientY){
      if(disposed||lost||!ready||!mesh)return null;
      const rect=canvas.getBoundingClientRect();if(!rect.width||!rect.height||clientX<rect.left||clientX>rect.left+rect.width||clientY<rect.top||clientY>rect.top+rect.height)return null;
      const aspect=width/height,camera=currentCamera(),ray=rayAt(camera,(clientX-rect.left)/rect.width*2-1,1-(clientY-rect.top)/rect.height*2,aspect),hit=raycast(mesh,ray.origin,ray.direction);
      return hit?.part?.slot?selectPart(hit.part.name,true,canvas):null;
    }
    function shader(type,source){
      const s=gl.createShader(type);if(!s)throw new Error('Graphics shader memory unavailable');
      try{gl.shaderSource(s,source);gl.compileShader(s);if(!gl.getShaderParameter(s,gl.COMPILE_STATUS))throw new Error(gl.getShaderInfoLog(s)||'Shader compilation failed');return s;}
      catch(error){gl.deleteShader(s);throw error;}
    }
    function makeProgram(vertex,fragment){
      let vs=null,fs=null,p=null;
      try{vs=shader(gl.VERTEX_SHADER,vertex);fs=shader(gl.FRAGMENT_SHADER,fragment);p=gl.createProgram();if(!p)throw new Error('Graphics program memory unavailable');gl.attachShader(p,vs);gl.attachShader(p,fs);gl.linkProgram(p);if(!gl.getProgramParameter(p,gl.LINK_STATUS))throw new Error(gl.getProgramInfoLog(p)||'Shader linking failed');return p;}
      catch(error){if(p)gl.deleteProgram(p);throw error;}
      finally{if(vs)gl.deleteShader(vs);if(fs)gl.deleteShader(fs);}
    }
    function makeBuffer(data){const b=gl.createBuffer();if(!b)throw new Error('Graphics memory unavailable');try{gl.bindBuffer(gl.ARRAY_BUFFER,b);gl.bufferData(gl.ARRAY_BUFFER,data,gl.STATIC_DRAW);return b;}catch(error){gl.deleteBuffer(b);throw error;}}
    function releaseShadow(){
      if(shadow&&gl&&!lost){if(shadow.framebuffer)gl.deleteFramebuffer(shadow.framebuffer);if(shadow.depth)gl.deleteRenderbuffer(shadow.depth);if(shadow.texture)gl.deleteTexture(shadow.texture);if(shadow.program)gl.deleteProgram(shadow.program);}
      shadow=null;
    }
    function createShadow(){
      // WebGL 1 core RGBA + DEPTH_COMPONENT16 needs no depth-texture extension.
      // Minimal mocks and devices lacking these methods retain projected shadows.
      const methods=['createFramebuffer','deleteFramebuffer','bindFramebuffer','checkFramebufferStatus','framebufferTexture2D','createTexture','deleteTexture','bindTexture','texImage2D','texParameteri','activeTexture','createRenderbuffer','deleteRenderbuffer','bindRenderbuffer','renderbufferStorage','framebufferRenderbuffer','getParameter','disable','disableVertexAttribArray','uniform1i'];
      if(methods.some(name=>typeof gl[name]!=='function'))return;
      let limit;
      try{limit=Math.min(gl.getParameter(gl.MAX_TEXTURE_SIZE),gl.getParameter(gl.MAX_RENDERBUFFER_SIZE));
        if(!Number.isFinite(limit)||limit<512||gl.getParameter(gl.MAX_TEXTURE_IMAGE_UNITS)<1)return;
        if(gl.getShaderPrecisionFormat&&!gl.getShaderPrecisionFormat(gl.FRAGMENT_SHADER,gl.HIGH_FLOAT)?.precision)return;
      }catch(error){return;}
      shadow={size:limit>=1024?1024:512,dirty:true,valid:false,frame:null};
      try{
        shadow.texture=gl.createTexture();if(!shadow.texture)throw new Error('Shadow texture unavailable');
        gl.activeTexture(gl.TEXTURE0);gl.bindTexture(gl.TEXTURE_2D,shadow.texture);
        gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MIN_FILTER,gl.NEAREST);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MAG_FILTER,gl.NEAREST);
        gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_WRAP_S,gl.CLAMP_TO_EDGE);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_WRAP_T,gl.CLAMP_TO_EDGE);
        gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,shadow.size,shadow.size,0,gl.RGBA,gl.UNSIGNED_BYTE,null);
        shadow.depth=gl.createRenderbuffer();if(!shadow.depth)throw new Error('Shadow depth unavailable');
        gl.bindRenderbuffer(gl.RENDERBUFFER,shadow.depth);gl.renderbufferStorage(gl.RENDERBUFFER,gl.DEPTH_COMPONENT16,shadow.size,shadow.size);
        shadow.framebuffer=gl.createFramebuffer();if(!shadow.framebuffer)throw new Error('Shadow framebuffer unavailable');
        gl.bindFramebuffer(gl.FRAMEBUFFER,shadow.framebuffer);gl.framebufferTexture2D(gl.FRAMEBUFFER,gl.COLOR_ATTACHMENT0,gl.TEXTURE_2D,shadow.texture,0);gl.framebufferRenderbuffer(gl.FRAMEBUFFER,gl.DEPTH_ATTACHMENT,gl.RENDERBUFFER,shadow.depth);
        if(gl.checkFramebufferStatus(gl.FRAMEBUFFER)!==gl.FRAMEBUFFER_COMPLETE)throw new Error('Shadow framebuffer incomplete');
        shadow.program=makeProgram(SHADOW_VERTEX,SHADOW_FRAGMENT);
        shadow.position=gl.getAttribLocation(shadow.program,'aPosition');shadow.matrix=gl.getUniformLocation(shadow.program,'uLightVP');
        if(shadow.position<0)throw new Error('Shadow position attribute unavailable');
      }catch(error){releaseShadow();}
      finally{gl.bindFramebuffer(gl.FRAMEBUFFER,null);gl.bindRenderbuffer(gl.RENDERBUFFER,null);gl.bindTexture(gl.TEXTURE_2D,null);}
    }
    function locateProgram(){
      for(const name of ['aPosition','aNormal','aColor'])attributes[name]=gl.getAttribLocation(program,name);
      attributes.aSurface=root.TankSurface?.glsl?gl.getAttribLocation(program,'aSurface'):-1;
      for(const name of ['uVP','uEye','uMode','uHighlight','uHeight','uTankEnabled','uAircraftEnabled','uAircraftGlass','uSurfaceOpacity','uTankCamo','uTankWear','uTankHeight','uShadowMap','uLightVP','uShadowTexel','uShadowNormalBias','uShadowDepthBias'])uniforms[name]=gl.getUniformLocation(program,name);
    }
    function release(){
      if(gl&&!lost){buffers.forEach(b=>gl.deleteBuffer(b));floorBuffers.forEach(b=>gl.deleteBuffer(b));if(program)gl.deleteProgram(program);}
      buffers=[];floorBuffers=[];program=null;releaseShadow();material?.dispose(lost);material=null;ready=false;
    }
    function upload(){
      if(!ready||!mesh||lost)return;buffers.forEach(b=>gl.deleteBuffer(b));buffers=[];
      if(shadow){shadow.dirty=true;shadow.valid=false;shadow.frame=shadowFrame(mesh.bounds,shadow.size);}
      const extent=shadow?.frame.floorExtent||shadowFrame(mesh.bounds,512).floorExtent;
      gl.bindBuffer(gl.ARRAY_BUFFER,floorBuffers[0]);gl.bufferData(gl.ARRAY_BUFFER,new Float32Array([-extent,0,-extent,extent,0,-extent,extent,0,extent,-extent,0,-extent,extent,0,extent,-extent,0,extent]),gl.STATIC_DRAW);
      if(!painted)painted=previewColors();for(const data of [mesh.positions,mesh.normals,painted])buffers.push(makeBuffer(data));
      if(tankData)buffers.push(makeBuffer(tankData.parameters));
    }
    function initialize(){
      gl=canvas.getContext('webgl',{alpha:false,antialias:true,depth:true,premultipliedAlpha:false,preserveDrawingBuffer:false});
      if(!gl)throw new Error('WebGL is unavailable');
      createShadow();
      try{program=makeProgram(VERTEX,fragmentSource(Boolean(shadow)));}
      catch(error){if(!shadow)throw error;releaseShadow();program=makeProgram(VERTEX,fragmentSource(false));}
      locateProgram();material=root.MilitarySurface?.create?.(gl,schedule)||null;
      const floor=new Float32Array([-24,0,-24,24,0,-24,24,0,24,-24,0,-24,24,0,24,-24,0,24]);
      floorBuffers=[];for(const data of [floor,new Float32Array(Array.from({length:18},(_,i)=>i%3===1?1:0)),new Float32Array(18).fill(.12)])floorBuffers.push(makeBuffer(data));
      gl.enable(gl.DEPTH_TEST);gl.depthFunc(gl.LEQUAL);gl.clearColor(.063,.088,.105,1);ready=true;upload();
      say('3D model ready. Drag to rotate or use the view controls.');
    }
    function bind(list){['aPosition','aNormal','aColor'].forEach((name,i)=>{gl.bindBuffer(gl.ARRAY_BUFFER,list[i]);gl.enableVertexAttribArray(attributes[name]);gl.vertexAttribPointer(attributes[name],3,gl.FLOAT,false,0,0);});
      if(attributes.aSurface>=0){if(list[3]){gl.bindBuffer(gl.ARRAY_BUFFER,list[3]);gl.enableVertexAttribArray(attributes.aSurface);gl.vertexAttribPointer(attributes.aSurface,4,gl.FLOAT,false,0,0);}else{gl.disableVertexAttribArray?.(attributes.aSurface);gl.vertexAttrib4f?.(attributes.aSurface,.85,0,255,1);}}
    }
    function drawShadow(){
      if(!shadow||!shadow.dirty)return true;
      let failed=false;
      try{
        gl.activeTexture(gl.TEXTURE0);gl.bindTexture(gl.TEXTURE_2D,null);
        gl.bindFramebuffer(gl.FRAMEBUFFER,shadow.framebuffer);gl.viewport(0,0,shadow.size,shadow.size);
        // Dithering would corrupt the packed depth bytes. The glass pass restores
        // blending/depth writes, and this target has no multisampling.
        gl.disable(gl.DITHER);gl.clearColor(1,1,1,1);gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT);
        gl.useProgram(shadow.program);gl.uniformMatrix4fv(shadow.matrix,false,shadow.frame.matrix);
        for(const index of Object.values(attributes))if(index>=0&&index!==shadow.position)gl.disableVertexAttribArray(index);
        gl.bindBuffer(gl.ARRAY_BUFFER,buffers[0]);gl.enableVertexAttribArray(shadow.position);gl.vertexAttribPointer(shadow.position,3,gl.FLOAT,false,0,0);
        for(const range of ranges.opaque)gl.drawArrays(gl.TRIANGLES,range.first,range.count);
        if(gl.getError&&gl.getError()!==gl.NO_ERROR)throw new Error('Shadow pass failed');
        shadow.dirty=false;shadow.valid=true;
      }catch(error){failed=true;}
      finally{gl.bindFramebuffer(gl.FRAMEBUFFER,null);gl.enable(gl.DITHER);gl.clearColor(.063,.088,.105,1);}
      if(failed){
        // An optional pass must never strand the renderer on an offscreen FBO
        // or leave a sampler pointing at a deleted/incomplete texture.
        releaseShadow();const old=program;program=null;
        try{program=makeProgram(VERTEX,fragmentSource(false));locateProgram();}
        catch(error){release();say('3D rendering is unavailable. You can still edit and review this design.');return false;}
        finally{if(old)gl.deleteProgram(old);}
      }
      return true;
    }
    function schedule(){if(!disposed&&!lost&&ready&&!raf&&visible&&!doc.hidden)raf=root.requestAnimationFrame(draw);}
    function resize(){
      if(disposed)return;const rect=slot.getBoundingClientRect();if(rect.width<1||rect.height<1)return;
      const ratio=Math.min(root.devicePixelRatio||1,2,Math.sqrt(2250000/(rect.width*rect.height)));
      const w=Math.max(1,Math.min(2048,Math.round(rect.width*ratio))),h=Math.max(1,Math.min(2048,Math.round(rect.height*ratio)));
      if(width!==w||height!==h){width=canvas.width=w;height=canvas.height=h;}
      schedule();
    }
    function draw(time){
      raf=0;if(disposed||lost||!ready||!mesh||!visible||doc.hidden||!canvas.isConnected)return;
      if(turntable&&lastTime)yaw+=(Math.min(64,time-lastTime)/1000)*.22;lastTime=time;
      if(!width||!height){resize();return;}
      const camera=currentCamera();
      if(!drawShadow())return;
      if(tankData)gl.clearColor(.105,.125,.132,1);else gl.clearColor(.063,.088,.105,1);
      gl.viewport(0,0,width,height);gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT);gl.useProgram(program);
      gl.uniformMatrix4fv(uniforms.uVP,false,camera.matrix);gl.uniform3fv(uniforms.uEye,camera.eye);
      gl.uniform1f(uniforms.uHeight,mesh.bounds.max[1]-mesh.bounds.min[1]);
      gl.uniform1f(uniforms.uAircraftEnabled,mesh.assetKind==='aircraft'?1:0);gl.uniform1f(uniforms.uAircraftGlass,0);gl.uniform1f(uniforms.uSurfaceOpacity,1);
      gl.uniform1f(uniforms.uTankEnabled,tankData?1:0);gl.uniform1f(uniforms.uTankCamo,finish==='woodland'?1:0);gl.uniform1f(uniforms.uTankWear,root.TankSurface?.wearLevels?.[wear]||0);gl.uniform1f(uniforms.uTankHeight,mesh.bounds.max[1]-mesh.bounds.min[1]);
      gl.uniform1f(uniforms.uHighlight,0);
      material?.bind(program);
      if(shadow?.valid){
        gl.activeTexture(gl.TEXTURE0);gl.bindTexture(gl.TEXTURE_2D,shadow.texture);gl.uniform1i(uniforms.uShadowMap,0);
        gl.uniformMatrix4fv(uniforms.uLightVP,false,shadow.frame.matrix);gl.uniform1f(uniforms.uShadowTexel,1/shadow.size);
        gl.uniform1f(uniforms.uShadowNormalBias,shadow.frame.normalBias);gl.uniform1f(uniforms.uShadowDepthBias,shadow.frame.depthBias);
      }
      bind(floorBuffers);gl.uniform1f(uniforms.uMode,1);gl.drawArrays(gl.TRIANGLES,0,6);
      bind(buffers);if(!shadow?.valid){gl.uniform1f(uniforms.uMode,2);for(const range of ranges.opaque)gl.drawArrays(gl.TRIANGLES,range.first,range.count);}
      gl.uniform1f(uniforms.uMode,0);for(const range of ranges.opaque)gl.drawArrays(gl.TRIANGLES,range.first,range.count);
      const selected=mesh.parts?.find(p=>p.name===selectedPart);
      if(selected){gl.uniform1f(uniforms.uHighlight,1);for(const range of ranges.opaque){const first=Math.max(range.first,selected.first),end=Math.min(range.first+range.count,selected.first+selected.count);if(end>first)gl.drawArrays(gl.TRIANGLES,first,end-first);}gl.uniform1f(uniforms.uHighlight,0);}
      if(ranges.glass.length){
        const blending=['blendFunc','depthMask','disable'].every(name=>typeof gl[name]==='function');
        // Transparent ranges are far-to-near; back faces precede front faces on
        // each convex canopy. Opaque cockpit geometry already owns the depth.
        const forward=normalize(camera.center.map((v,i)=>v-camera.eye[i]));
        const glass=[...ranges.glass].sort((a,b)=>dot(b.center,forward)-dot(a.center,forward));
        try{
          if(blending){gl.enable(gl.BLEND);gl.blendFunc(gl.SRC_ALPHA,gl.ONE_MINUS_SRC_ALPHA);gl.depthMask(false);gl.uniform1f(uniforms.uAircraftGlass,1);}
          for(const range of glass){
            gl.uniform1f(uniforms.uSurfaceOpacity,range.opacity);gl.uniform1f(uniforms.uHighlight,range.part===selectedPart?1:0);
            if(blending&&typeof gl.cullFace==='function'){gl.enable(gl.CULL_FACE);gl.cullFace(gl.FRONT);gl.drawArrays(gl.TRIANGLES,range.first,range.count);gl.cullFace(gl.BACK);gl.drawArrays(gl.TRIANGLES,range.first,range.count);gl.disable(gl.CULL_FACE);}
            else gl.drawArrays(gl.TRIANGLES,range.first,range.count);
          }
        }finally{if(blending){gl.depthMask(true);gl.disable(gl.BLEND);if(typeof gl.cullFace==='function')gl.disable(gl.CULL_FACE);}gl.uniform1f(uniforms.uAircraftGlass,0);gl.uniform1f(uniforms.uHighlight,0);gl.uniform1f(uniforms.uSurfaceOpacity,1);}
      }
      if(turntable)schedule();
    }
    function stopRotation(){turntable=false;lastTime=0;setControls();}
    function currentCamera(){return frame(focus?.bounds||mesh.bounds,width/height,yaw,pitch,zoomFactor);}
    function focusTarget(value,region){const key=JSON.stringify([value,region]);if(!focusTargets.has(key)){const target=partBounds(mesh,value,region);focusTargets.set(key,region==='front'&&target?.part.slot==='air_engine'&&mesh.assetKind!=='aircraft'?null:target);}return focusTargets.get(key);}
    function clearFocus(restoreAngle=false){if(!focus)return;zoomFactor=focus.overviewZoom;if(restoreAngle){yaw=focus.overviewYaw;pitch=focus.overviewPitch;viewName=focus.overviewView;}focus=null;}
    function focusPart(value,region='rear'){
      if(disposed||lost||!ready||!mesh)return null;
      if(region!=='rear'&&region!=='front')return null;const target=focusTarget(value,region);if(!target)return null;
      const overview=focus||{overviewZoom:zoomFactor,overviewYaw:yaw,overviewPitch:pitch,overviewView:viewName};
      stopRotation();focus={value,region,bounds:target.bounds,overviewZoom:overview.overviewZoom,overviewYaw:overview.overviewYaw,overviewPitch:overview.overviewPitch,overviewView:overview.overviewView};
      [yaw,pitch]=target.part.slot==='air_engine'?[region==='front'?(target.bounds.min[0]+target.bounds.max[0]<0?-.2:.2):Math.PI,.12]:target.part.slot==='air_avionics'?[2.85,.9]:VIEWS.hero;zoomFactor=1;viewName='';
      selectPart(target.part.name,true);setControls();schedule();
      say(`${target.part.slot==='air_avionics'?'Cockpit':target.part.slot==='air_engine'?(region==='front'?'Intakes':'Engines'):target.part.label||target.part.name} close-up. Drag to orbit; choose a normal view or Reset to see the whole aircraft.`);
      return target.part;
    }
    function rotate(dy,dp){if(!Number.isFinite(dy)||!Number.isFinite(dp))return;stopRotation();yaw=(yaw+dy)%(Math.PI*2);pitch=clamp(pitch+dp,.055,1.50);viewName='';setControls();schedule();}
    function zoom(delta){if(!Number.isFinite(delta))return;stopRotation();zoomFactor=clamp(zoomFactor*Math.exp(delta*.12),.5,2.25);schedule();}
    function view(name){if(!VIEWS[name])return;stopRotation();clearFocus();[yaw,pitch]=VIEWS[name];viewName=name;setControls();schedule();say(`${name==='hero'?'Three-quarter':name.charAt(0).toUpperCase()+name.slice(1)} view.`);}
    function reset(){clearFocus();zoomFactor=1;view('hero');}
    function toggleRotation(){turntable=!turntable;viewName='';lastTime=0;setControls();schedule();}
    function previewColors(){return tankData?root.TankSurface.bake(mesh,{finish,wear,baseOnly:true}):finishColors(mesh.colors,finish);}
    function repaint(){if(mesh)painted=previewColors();if(mesh&&ready&&!lost){gl.bindBuffer(gl.ARRAY_BUFFER,buffers[2]);gl.bufferData(gl.ARRAY_BUFFER,painted,gl.STATIC_DRAW);}setControls();schedule();}
    function setFinish(value){
      if(disposed||!Object.prototype.hasOwnProperty.call(FINISHES,value)&&!(tankData&&Object.prototype.hasOwnProperty.call(root.TankSurface.finishes,value)))return;
      finish=value;repaint();
    }
    function setWear(value){if(disposed||!tankData||!Object.prototype.hasOwnProperty.call(root.TankSurface.wearLevels,value))return;wear=value;repaint();}
    function update(next){
      if(disposed)return;draft=next;const nextKey=modelKey(next);if(nextKey===key){resize();return;}
      try{const candidate=root.EquipmentMesh.build(next);if(!candidate.positions?.length||candidate.positions.length!==candidate.normals.length||candidate.positions.length!==candidate.colors.length)throw new Error('Invalid model geometry');
        const oldSlot=mesh?.parts?.find(p=>p.name===selectedPart)?.slot;
        const nextRanges=surfaceRanges(candidate);clearFocus(true);mesh=candidate;ranges=nextRanges;focusTargets.clear();tankData=root.TankSurface?.supports?.(mesh,draft?.platform)?root.TankSurface.prepare(mesh):null;if(!tankData&&finish==='woodland')finish='olive';
        key=nextKey;selectedPart=(mesh.parts?.find(p=>p.name===selectedPart)||mesh.parts?.find(p=>oldSlot&&p.slot===oldSlot))?.name||null;
        painted=previewColors();upload();refreshParts();setControls();resize();
        if(ready&&!lost)say('3D model ready. Drag to rotate or use the view controls.');
        else if(!lost)say('3D rendering is unavailable on this graphics device. Component choices and cost reviews still work.');
      }catch(error){
        // Never retain a previous configuration under the new draft's name.
        clearFocus(true);mesh=null;ranges=null;focusTargets.clear();tankData=null;painted=null;key=null;selectedPart=null;if(shadow){shadow.dirty=true;shadow.valid=false;shadow.frame=null;}refreshParts();
        if(raf)root.cancelAnimationFrame(raf);raf=0;
        if(gl&&!lost){buffers.forEach(b=>gl.deleteBuffer(b));if(ready)gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT);}
        buffers=[];setControls();
        say('This configuration could not be displayed in 3D. Its research and cost controls are still available.');
      }
    }
    function exportGlb(){
      if(!mesh||!root.EquipmentExport)return null;
      // Fragment camouflage/wear is sampled at vertices for the simple GLB
      // format. Large faces therefore export an approximation of the preview.
      const colors=tankData?root.TankSurface.bake(mesh,{finish,wear}):painted||mesh.colors;
      return root.EquipmentExport.glb({...mesh,colors},draft?.name||'Spheres equipment');
    }
    function download(){
      try{const data=exportGlb();if(!data)return;const url=root.URL.createObjectURL(new root.Blob([data],{type:'model/gltf-binary'}));urls.add(url);
        const a=doc.createElement('a');a.href=url;a.download=(draft?.name||'spheres-equipment').replace(/[^a-z0-9_-]+/gi,'-').slice(0,64)+'.glb';doc.body.appendChild(a);a.click();a.remove();say('3D model downloaded as a GLB file.');
        // The click has consumed this blob URL; defer revocation until the next task.
        root.setTimeout(()=>{root.URL.revokeObjectURL(url);urls.delete(url);},1000);
      }catch(error){say('The 3D file could not be exported. The design is still available.');}
    }
    host.querySelectorAll('[data-model-view]').forEach(b=>on(b,'click',()=>view(b.dataset.modelView)));
    host.querySelectorAll('[data-model-focus]').forEach(b=>on(b,'click',()=>focusPart(b.dataset.modelFocus,b.dataset.modelFocusRegion||'rear')));
    host.querySelectorAll('[data-model-rotate]').forEach(b=>on(b,'click',()=>{const moves={left:[-.18,0],right:[.18,0],up:[0,.12],down:[0,-.12]};const d=moves[b.dataset.modelRotate];if(d)rotate(...d);}));
    host.querySelectorAll('[data-model-zoom]').forEach(b=>on(b,'click',()=>zoom(b.dataset.modelZoom==='in'?-1:1)));
    host.querySelectorAll('[data-model-reset]').forEach(b=>on(b,'click',reset));
    host.querySelectorAll('[data-model-turntable]').forEach(b=>on(b,'click',toggleRotation));
    host.querySelectorAll('[data-model-finish]').forEach(b=>on(b,b.tagName==='SELECT'?'change':'click',()=>setFinish(b.tagName==='SELECT'?b.value:b.dataset.modelFinish)));
    host.querySelectorAll('[data-model-wear]').forEach(b=>on(b,b.tagName==='SELECT'?'change':'click',()=>setWear(b.tagName==='SELECT'?b.value:b.dataset.modelWear)));
    host.querySelectorAll('[data-model-export]').forEach(b=>on(b,'click',download));
    if(partSelect)on(partSelect,'change',()=>selectPart(partSelect.value,true,host));
    on(canvas,'pointerdown',e=>{if(e.pointerType==='mouse'&&e.button!==0)return;e.preventDefault();canvas.focus({preventScroll:true});stopRotation();pointers.set(e.pointerId,[e.clientX,e.clientY]);gesture=pointers.size===1?{id:e.pointerId,start:[e.clientX,e.clientY],dragged:false}:null;canvas.setPointerCapture?.(e.pointerId);});
    on(canvas,'pointermove',e=>{if(!pointers.has(e.pointerId))return;const old=pointers.get(e.pointerId),before=[...pointers.values()];pointers.set(e.pointerId,[e.clientX,e.clientY]);
      if(pointers.size===1&&gesture){
        if(!gesture.dragged){if(Math.hypot(e.clientX-gesture.start[0],e.clientY-gesture.start[1])<=5)return;gesture.dragged=true;old[0]=gesture.start[0];old[1]=gesture.start[1];}
        rotate(-(e.clientX-old[0])*.009,(e.clientY-old[1])*.007);
      }
      else if(pointers.size===2){const after=[...pointers.values()],a=Math.hypot(before[0][0]-before[1][0],before[0][1]-before[1][1]),b=Math.hypot(after[0][0]-after[1][0],after[0][1]-after[1][1]);if(a>0&&b>0)zoom(Math.log(a/b)/.12);}
    });
    on(canvas,'pointerup',e=>{const click=gesture&&gesture.id===e.pointerId&&!gesture.dragged&&pointers.size===1&&Math.hypot(e.clientX-gesture.start[0],e.clientY-gesture.start[1])<=5;pointers.delete(e.pointerId);gesture=null;if(click)pickAt(e.clientX,e.clientY);});
    for(const event of ['pointercancel','lostpointercapture'])on(canvas,event,e=>{pointers.delete(e.pointerId);gesture=null;});
    on(canvas,'wheel',e=>{e.preventDefault();zoom(clamp(e.deltaY*(e.deltaMode===1?16:e.deltaMode===2?height:1)/120,-4,4));},{passive:false});
    on(canvas,'keydown',e=>{const keys={ArrowLeft:[-.15,0],ArrowRight:[.15,0],ArrowUp:[0,.1],ArrowDown:[0,-.1]};if(keys[e.key])rotate(...keys[e.key]);else if(e.key==='+'||e.key==='=')zoom(-1);else if(e.key==='-')zoom(1);else if(e.key==='Home'||e.key==='0')reset();else return;e.preventDefault();e.stopPropagation();});
    on(canvas,'webglcontextlost',e=>{e.preventDefault();lost=true;release();gesture=null;pointers.clear();if(raf)root.cancelAnimationFrame(raf);raf=0;setControls();say('The 3D preview is waiting for the graphics device. Your design is preserved.');});
    on(canvas,'webglcontextrestored',()=>{if(disposed)return;lost=false;buffers=[];floorBuffers=[];program=null;try{initialize();resize();}catch(error){release();say('3D rendering is unavailable. You can still edit and review this design.');}setControls();});
    on(doc,'visibilitychange',()=>{lastTime=0;if(doc.hidden&&raf){root.cancelAnimationFrame(raf);raf=0;}else schedule();});
    if(root.ResizeObserver){resizeObserver=new root.ResizeObserver(resize);resizeObserver.observe(slot);}else on(root,'resize',resize);
    if(root.IntersectionObserver){intersectionObserver=new root.IntersectionObserver(entries=>{visible=entries[0]?.isIntersecting!==false;lastTime=0;if(!visible&&raf){root.cancelAnimationFrame(raf);raf=0;}else schedule();});intersectionObserver.observe(canvas);}
    function dispose(){if(disposed)return;disposed=true;if(raf)root.cancelAnimationFrame(raf);raf=0;resizeObserver?.disconnect();intersectionObserver?.disconnect();events.forEach(([n,e,f,o])=>n.removeEventListener(e,f,o));pointers.clear();gesture=null;urls.forEach(url=>root.URL.revokeObjectURL(url));urls.clear();release();canvas.remove();mesh=null;painted=null;tankData=null;}
    try{initialize();}catch(error){release();root.console?.warn('Equipment 3D preview:',error.message);say('3D rendering is unavailable on this graphics device. Component choices and cost reviews still work.');}
    update(spec);setControls();resize();
    return {update,resize,dispose,view,rotate,zoom,reset,toggleRotation,setFinish,setWear,exportGlb,selectPart,focusPart,pickAt};
  }
  return {mount,modelKey,frame,finishColors,surfaceRanges,partBounds,rayAt,raycast,shadowFrame,math:{lookAt,perspective,multiply},views:VIEWS};
});
