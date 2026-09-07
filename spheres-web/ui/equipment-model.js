/* Original equipment mesh viewer. Visual state never enters a simulation command.
   Uses WebGL triangles, perspective/depth, directional light and projected shadows.
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
  function modelKey(spec){const c=spec?.components||{};return JSON.stringify([spec?.platform,...Object.keys(c).sort().map(k=>[k,c[k]])]);}
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
      // Only olive-painted surfaces change. Rubber, steel, lenses and lamps keep their material.
      if(g>r*1.025&&g>b*1.06&&g>.12){const shade=clamp((r*.25+g*.65+b*.10)/.30,.4,1.5);for(let k=0;k<3;k++)out[i+k]=clamp(tone[k]*shade,0,1);}
    }return out;
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
attribute vec3 aPosition;attribute vec3 aNormal;attribute vec3 aColor;
uniform mat4 uVP;uniform mediump float uMode;
varying mediump vec3 vPosition;varying mediump vec3 vNormal;varying mediump vec3 vColor;
void main(){vec3 p=aPosition;if(uMode>1.5){p=vec3(p.x+p.y*.55,.016,p.z-p.y*.65);}vPosition=p;vNormal=aNormal;vColor=aColor;gl_Position=uVP*vec4(p,1.);}`;
  const FRAGMENT=`precision mediump float;
uniform vec3 uEye;uniform float uMode;uniform float uHighlight;
varying vec3 vPosition;varying vec3 vNormal;varying vec3 vColor;
vec3 floorColor(vec3 p){float radius=length(p.xz*.095);vec3 c=mix(vec3(.135,.172,.177),vec3(.063,.088,.105),smoothstep(.1,1.3,radius));
float grid=pow(abs(cos(p.x*3.14159265)),80.)+pow(abs(cos(p.z*3.14159265)),80.);c+=vec3(.008)*grid*(1.-smoothstep(4.,12.,length(p.xz)));
float contact=exp(-pow(p.x/1.9,4.)-pow(p.z/3.7,4.));return c*(1.-.28*contact);}
void main(){if(uMode>.5){vec3 c=floorColor(vPosition);if(uMode>1.5)c*=.53;gl_FragColor=vec4(c,1.);return;}
vec3 n=normalize(vNormal),view=normalize(uEye-vPosition),light=normalize(vec3(-.55,1.,.65));
float key=max(dot(n,light),0.),fill=max(dot(n,normalize(vec3(.8,.35,-.7))),0.);
vec3 ambient=mix(vec3(.25,.29,.32),vec3(.58,.64,.65),n.y*.5+.5);
float specular=pow(max(dot(n,normalize(light+view)),0.),45.)*.19;
vec3 base=pow(max(vColor,vec3(.001)),vec3(2.2));
vec3 lit=base*(ambient*.6+vec3(1.10,1.05,.91)*key+vec3(.31,.40,.48)*fill)+vec3(specular);
float cavity=mix(.78,1.,smoothstep(.08,1.3,vPosition.y));lit*=cavity;
lit=mix(lit,lit*.65+vec3(.32,.20,.035),uHighlight);
gl_FragColor=vec4(pow(max(lit,vec3(0.)),vec3(1./2.2)),1.);}`;

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
    let gl=null,program=null,buffers=[],floorBuffers=[],uniforms={},attributes={},mesh=null,painted=null,key=null,draft=spec;
    let disposed=false,lost=false,raf=0,turntable=false,visible=true,lastTime=0,ready=false;
    let yaw=VIEWS.hero[0],pitch=VIEWS.hero[1],zoomFactor=1,finish='olive',viewName='hero';
    let width=0,height=0,resizeObserver=null,intersectionObserver=null,selectedPart=null,gesture=null;
    const partSelect=host.querySelector('[data-model-part]');
    const events=[],pointers=new Map(),urls=new Set();
    const on=(node,event,handler,options)=>{node.addEventListener(event,handler,options);events.push([node,event,handler,options]);};
    const say=text=>{status.textContent=text;};
    function setControls(){
      host.querySelectorAll('[data-model-view]').forEach(b=>b.setAttribute('aria-pressed',String(b.dataset.modelView===viewName)));
      host.querySelectorAll('[data-model-turntable]').forEach(b=>{b.setAttribute('aria-pressed',String(turntable));b.textContent=turntable?'Stop rotation':'Auto rotate';});
      host.querySelectorAll('[data-model-finish]').forEach(b=>{if(b.tagName==='SELECT')b.value=finish;else b.setAttribute('aria-pressed',String(b.dataset.modelFinish===finish));});
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
      const aspect=width/height,camera=frame(mesh.bounds,aspect,yaw,pitch,zoomFactor),ray=rayAt(camera,(clientX-rect.left)/rect.width*2-1,1-(clientY-rect.top)/rect.height*2,aspect),hit=raycast(mesh,ray.origin,ray.direction);
      return hit?.part?.slot?selectPart(hit.part.name,true,canvas):null;
    }
    function shader(type,source){const s=gl.createShader(type);gl.shaderSource(s,source);gl.compileShader(s);if(!gl.getShaderParameter(s,gl.COMPILE_STATUS)){const log=gl.getShaderInfoLog(s);gl.deleteShader(s);throw new Error(log||'Shader compilation failed');}return s;}
    function makeBuffer(data){const b=gl.createBuffer();if(!b)throw new Error('Graphics memory unavailable');try{gl.bindBuffer(gl.ARRAY_BUFFER,b);gl.bufferData(gl.ARRAY_BUFFER,data,gl.STATIC_DRAW);return b;}catch(error){gl.deleteBuffer(b);throw error;}}
    function release(){if(!gl||lost)return;buffers.forEach(b=>gl.deleteBuffer(b));floorBuffers.forEach(b=>gl.deleteBuffer(b));buffers=[];floorBuffers=[];if(program)gl.deleteProgram(program);program=null;ready=false;}
    function upload(){if(!ready||!mesh||lost)return;buffers.forEach(b=>gl.deleteBuffer(b));buffers=[];painted=finishColors(mesh.colors,finish);for(const data of [mesh.positions,mesh.normals,painted])buffers.push(makeBuffer(data));}
    function initialize(){
      gl=canvas.getContext('webgl',{alpha:false,antialias:true,depth:true,premultipliedAlpha:false,preserveDrawingBuffer:false});
      if(!gl)throw new Error('WebGL is unavailable');
      const vs=shader(gl.VERTEX_SHADER,VERTEX);let fs=null;
      try{fs=shader(gl.FRAGMENT_SHADER,FRAGMENT);program=gl.createProgram();gl.attachShader(program,vs);gl.attachShader(program,fs);gl.linkProgram(program);}
      finally{gl.deleteShader(vs);if(fs)gl.deleteShader(fs);}
      if(!gl.getProgramParameter(program,gl.LINK_STATUS))throw new Error(gl.getProgramInfoLog(program)||'Shader linking failed');
      for(const name of ['aPosition','aNormal','aColor'])attributes[name]=gl.getAttribLocation(program,name);
      for(const name of ['uVP','uEye','uMode','uHighlight'])uniforms[name]=gl.getUniformLocation(program,name);
      const floor=new Float32Array([-24,0,-24,24,0,-24,24,0,24,-24,0,-24,24,0,24,-24,0,24]);
      floorBuffers=[];for(const data of [floor,new Float32Array(Array.from({length:18},(_,i)=>i%3===1?1:0)),new Float32Array(18).fill(.12)])floorBuffers.push(makeBuffer(data));
      gl.enable(gl.DEPTH_TEST);gl.depthFunc(gl.LEQUAL);gl.clearColor(.063,.088,.105,1);ready=true;upload();
      say('3D model ready. Drag to rotate or use the view controls.');
    }
    function bind(list){['aPosition','aNormal','aColor'].forEach((name,i)=>{gl.bindBuffer(gl.ARRAY_BUFFER,list[i]);gl.enableVertexAttribArray(attributes[name]);gl.vertexAttribPointer(attributes[name],3,gl.FLOAT,false,0,0);});}
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
      const camera=frame(mesh.bounds,width/height,yaw,pitch,zoomFactor);
      gl.viewport(0,0,width,height);gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT);gl.useProgram(program);
      gl.uniformMatrix4fv(uniforms.uVP,false,camera.matrix);gl.uniform3fv(uniforms.uEye,camera.eye);
      gl.uniform1f(uniforms.uHighlight,0);
      bind(floorBuffers);gl.uniform1f(uniforms.uMode,1);gl.drawArrays(gl.TRIANGLES,0,6);
      bind(buffers);gl.uniform1f(uniforms.uMode,2);gl.drawArrays(gl.TRIANGLES,0,mesh.positions.length/3);
      gl.uniform1f(uniforms.uMode,0);gl.drawArrays(gl.TRIANGLES,0,mesh.positions.length/3);
      const selected=mesh.parts?.find(p=>p.name===selectedPart);
      if(selected){gl.uniform1f(uniforms.uHighlight,1);gl.drawArrays(gl.TRIANGLES,selected.first,selected.count);gl.uniform1f(uniforms.uHighlight,0);}
      if(turntable)schedule();
    }
    function stopRotation(){turntable=false;lastTime=0;setControls();}
    function rotate(dy,dp){if(!Number.isFinite(dy)||!Number.isFinite(dp))return;stopRotation();yaw=(yaw+dy)%(Math.PI*2);pitch=clamp(pitch+dp,.055,1.50);viewName='';setControls();schedule();}
    function zoom(delta){if(!Number.isFinite(delta))return;stopRotation();zoomFactor=clamp(zoomFactor*Math.exp(delta*.12),.5,2.25);schedule();}
    function view(name){if(!VIEWS[name])return;stopRotation();[yaw,pitch]=VIEWS[name];viewName=name;setControls();schedule();say(`${name==='hero'?'Three-quarter':name.charAt(0).toUpperCase()+name.slice(1)} view.`);}
    function reset(){zoomFactor=1;view('hero');}
    function toggleRotation(){turntable=!turntable;viewName='';lastTime=0;setControls();schedule();}
    function setFinish(value){
      if(disposed||!Object.prototype.hasOwnProperty.call(FINISHES,value))return;
      finish=value;if(mesh)painted=finishColors(mesh.colors,finish);
      if(mesh&&ready&&!lost){gl.bindBuffer(gl.ARRAY_BUFFER,buffers[2]);gl.bufferData(gl.ARRAY_BUFFER,painted,gl.STATIC_DRAW);}
      setControls();schedule();
    }
    function update(next){
      if(disposed)return;draft=next;const nextKey=modelKey(next);if(nextKey===key){resize();return;}
      try{const candidate=root.EquipmentMesh.build(next);if(!candidate.positions?.length||candidate.positions.length!==candidate.normals.length||candidate.positions.length!==candidate.colors.length)throw new Error('Invalid model geometry');
        const oldSlot=mesh?.parts?.find(p=>p.name===selectedPart)?.slot;
        mesh=candidate;key=nextKey;selectedPart=(mesh.parts?.find(p=>p.name===selectedPart)||mesh.parts?.find(p=>oldSlot&&p.slot===oldSlot))?.name||null;
        painted=finishColors(mesh.colors,finish);upload();refreshParts();setControls();resize();
        if(ready&&!lost)say('3D model ready. Drag to rotate or use the view controls.');
        else if(!lost)say('3D rendering is unavailable on this graphics device. Component choices and cost reviews still work.');
      }catch(error){
        // Never retain a previous configuration under the new draft's name.
        mesh=null;painted=null;key=null;selectedPart=null;refreshParts();
        if(raf)root.cancelAnimationFrame(raf);raf=0;
        if(gl&&!lost){buffers.forEach(b=>gl.deleteBuffer(b));if(ready)gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT);}
        buffers=[];setControls();
        say('This configuration could not be displayed in 3D. Its research and cost controls are still available.');
      }
    }
    function exportGlb(){
      if(!mesh||!root.EquipmentExport)return null;
      return root.EquipmentExport.glb({...mesh,colors:painted||mesh.colors},draft?.name||'Spheres tank');
    }
    function download(){
      try{const data=exportGlb();if(!data)return;const url=root.URL.createObjectURL(new root.Blob([data],{type:'model/gltf-binary'}));urls.add(url);
        const a=doc.createElement('a');a.href=url;a.download=(draft?.name||'spheres-tank').replace(/[^a-z0-9_-]+/gi,'-').slice(0,64)+'.glb';doc.body.appendChild(a);a.click();a.remove();say('3D model downloaded as a GLB file.');
        // The click has consumed this blob URL; defer revocation until the next task.
        root.setTimeout(()=>{root.URL.revokeObjectURL(url);urls.delete(url);},1000);
      }catch(error){say('The 3D file could not be exported. The design is still available.');}
    }
    host.querySelectorAll('[data-model-view]').forEach(b=>on(b,'click',()=>view(b.dataset.modelView)));
    host.querySelectorAll('[data-model-rotate]').forEach(b=>on(b,'click',()=>{const moves={left:[-.18,0],right:[.18,0],up:[0,.12],down:[0,-.12]};const d=moves[b.dataset.modelRotate];if(d)rotate(...d);}));
    host.querySelectorAll('[data-model-zoom]').forEach(b=>on(b,'click',()=>zoom(b.dataset.modelZoom==='in'?-1:1)));
    host.querySelectorAll('[data-model-reset]').forEach(b=>on(b,'click',reset));
    host.querySelectorAll('[data-model-turntable]').forEach(b=>on(b,'click',toggleRotation));
    host.querySelectorAll('[data-model-finish]').forEach(b=>on(b,b.tagName==='SELECT'?'change':'click',()=>setFinish(b.tagName==='SELECT'?b.value:b.dataset.modelFinish)));
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
    on(canvas,'webglcontextlost',e=>{e.preventDefault();lost=true;ready=false;gesture=null;pointers.clear();if(raf)root.cancelAnimationFrame(raf);raf=0;say('The 3D preview is waiting for the graphics device. Your design is preserved.');});
    on(canvas,'webglcontextrestored',()=>{if(disposed)return;lost=false;buffers=[];floorBuffers=[];program=null;try{initialize();resize();}catch(error){release();say('3D rendering is unavailable. You can still edit and review this design.');}});
    on(doc,'visibilitychange',()=>{lastTime=0;if(doc.hidden&&raf){root.cancelAnimationFrame(raf);raf=0;}else schedule();});
    if(root.ResizeObserver){resizeObserver=new root.ResizeObserver(resize);resizeObserver.observe(slot);}else on(root,'resize',resize);
    if(root.IntersectionObserver){intersectionObserver=new root.IntersectionObserver(entries=>{visible=entries[0]?.isIntersecting!==false;lastTime=0;if(!visible&&raf){root.cancelAnimationFrame(raf);raf=0;}else schedule();});intersectionObserver.observe(canvas);}
    function dispose(){if(disposed)return;disposed=true;if(raf)root.cancelAnimationFrame(raf);raf=0;resizeObserver?.disconnect();intersectionObserver?.disconnect();events.forEach(([n,e,f,o])=>n.removeEventListener(e,f,o));pointers.clear();gesture=null;urls.forEach(url=>root.URL.revokeObjectURL(url));urls.clear();release();canvas.remove();mesh=null;painted=null;}
    try{initialize();}catch(error){release();root.console?.warn('Equipment 3D preview:',error.message);say('3D rendering is unavailable on this graphics device. Component choices and cost reviews still work.');}
    update(spec);setControls();resize();
    return {update,resize,dispose,view,rotate,zoom,reset,toggleRotation,setFinish,exportGlb,selectPart,pickAt};
  }
  return {mount,modelKey,frame,finishColors,rayAt,raycast,math:{lookAt,perspective,multiply},views:VIEWS};
});
