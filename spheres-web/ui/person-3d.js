/* Manual character controls; one shared WebGL context and no idle animation. */
(function(root,factory) {
  const api=factory(root);
  if(typeof module==="object"&&module.exports)module.exports=api;
  else root.Person3D=api;
})(typeof globalThis!=="undefined"?globalThis:this,function(root) {
  "use strict";
  const initial=()=>({yaw:12,pitch:5,zoom:1});
  function change(view,action,dx=0,dy=0) {
    const next={...view};
    if(action==="reset")return initial();
    if(action==="face")return {yaw:12,pitch:2,zoom:3};
    if(action==="left")next.yaw-=25;
    if(action==="right")next.yaw+=25;
    if(action==="in")next.zoom+=.15;
    if(action==="out")next.zoom-=.15;
    if(action==="up")next.pitch+=5;
    if(action==="down")next.pitch-=5;
    if(action==="drag"){next.yaw+=Number.isFinite(dx)?dx*.55:0;next.pitch+=Number.isFinite(dy)?dy*.3:0;}
    next.yaw=((next.yaw%360)+360)%360;
    next.pitch=Math.max(-20,Math.min(35,next.pitch));
    next.zoom=Math.max(.75,Math.min(3,next.zoom));
    return next;
  }
  let dialog=null,disposeDialog=null;
  const sessions=new WeakMap();
  function bind(stage,id,interactive) {
    const canvas=stage.querySelector("canvas"), controls=stage.querySelectorAll("[data-person-turn]");
    const abort=new AbortController(),signal=abort.signal;
    let view=initial(),frame=0,alive=true,pointer=null;
    const paint=()=>{
      frame=0;if(!alive||!canvas.isConnected)return;
      const ok=!!root.Arsenal3D?.draw(canvas,"person:"+id,view);
      stage.classList.toggle("person-3d-ready",ok);
      const detail=stage.querySelector(".person-3d-detail");
      if(detail&&ok)detail.textContent=(root.PersonModels?.meta(id)?.triangle_count||0).toLocaleString()+" triangles · Full detail";
      controls.forEach(b=>{b.disabled=!ok;});
      canvas.setAttribute("aria-label",`${root.PersonModels?.meta(id)?.name||"Character"}, 3D view. ${interactive?"Drag or use arrow keys to rotate; plus and minus to zoom; Home to reset.":"Use the rotation buttons below or open the larger viewer."}`);
    };
    const queue=()=>{if(alive&&!frame)frame=root.requestAnimationFrame(paint);};
    const act=(action,dx,dy)=>{view=change(view,action,dx,dy);queue();};
    controls.forEach(b=>b.addEventListener("click",()=>act(b.dataset.personTurn),{signal}));
    if(interactive) {
      canvas.addEventListener("pointerdown",e=>{
        if(e.isPrimary===false||e.button!==0)return;
        pointer={id:e.pointerId,x:e.clientX,y:e.clientY};canvas.setPointerCapture(e.pointerId);canvas.focus({preventScroll:true});
      },{signal});
      canvas.addEventListener("pointermove",e=>{
        if(!pointer||pointer.id!==e.pointerId)return;
        act("drag",e.clientX-pointer.x,e.clientY-pointer.y);pointer.x=e.clientX;pointer.y=e.clientY;
      },{signal});
      const release=()=>{pointer=null;};
      ["pointerup","pointercancel","lostpointercapture"].forEach(name=>canvas.addEventListener(name,release,{signal}));
      canvas.addEventListener("keydown",e=>{
        const action={ArrowLeft:"left",ArrowRight:"right",ArrowUp:"up",ArrowDown:"down",Home:"reset","+":"in","=":"in","-":"out"}[e.key];
        if(action){e.preventDefault();e.stopPropagation();act(action);}
      },{signal});
      canvas.addEventListener("wheel",e=>{
        if(root.document.activeElement!==canvas)return;
        e.preventDefault();act(e.deltaY<0?"in":"out");
      },{signal,passive:false});
    }
    const resize=typeof root.ResizeObserver==="function"?new root.ResizeObserver(queue):null;
    const visible=typeof root.IntersectionObserver==="function"?new root.IntersectionObserver(entries=>{if(entries.some(e=>e.isIntersecting))queue();},{rootMargin:"150px"}):null;
    resize?.observe(canvas);visible?.observe(canvas);
    if(!visible)queue();
    return ()=>{alive=false;abort.abort();resize?.disconnect();visible?.disconnect();if(frame)root.cancelAnimationFrame(frame);};
  }
  function close() {
    disposeDialog?.();disposeDialog=null;
    if(dialog){if(dialog.open)dialog.close();dialog.remove();dialog=null;}
  }
  function open(id,opener) {
    const meta=root.PersonModels?.meta(id);if(!meta)return;
    close();
    dialog=root.document.createElement("dialog");dialog.className="person-3d-dialog";
    dialog.setAttribute("aria-labelledby","person-3d-title");
    dialog.innerHTML='<header><div><p class="person-3d-kicker">Character studio · 3D likeness study</p><h2 id="person-3d-title"></h2></div><button type="button" data-person-close aria-label="Close character viewer">×</button></header><div class="person-3d-stage"><div class="person-3d-fallback">3D is unavailable in this browser. The character file is available below.</div><canvas tabindex="0" role="img"></canvas><div class="person-3d-toolbar" role="group" aria-label="Character camera"><button type="button" data-person-turn="left" aria-label="Rotate left">↶</button><button type="button" data-person-turn="right" aria-label="Rotate right">↷</button><button type="button" data-person-turn="out" aria-label="Zoom out">−</button><button type="button" data-person-turn="in" aria-label="Zoom in">+</button><button type="button" data-person-turn="face">Face close-up</button><button type="button" data-person-turn="reset">Full figure</button></div><p class="person-3d-detail"></p></div><p class="person-3d-help">Drag to rotate · Arrow keys to turn · + / − to zoom</p><footer><p class="person-3d-credit"></p><a download>Download 3D model (.glb)</a></footer>';
    dialog.querySelector("h2").textContent=meta.name;
    dialog.querySelector(".person-3d-credit").textContent=meta.credit+" Appearance: "+meta.from.slice(0,4)+"–"+(Number(meta.to.slice(0,4))-1)+".";
    dialog.querySelector("a").href="/art/people/"+id+".glb";
    root.document.body.appendChild(dialog);
    // Camera keys and Escape belong to the modal, never to the campaign/map.
    dialog.addEventListener("keydown",e=>e.stopPropagation());
    dialog.querySelector("[data-person-close]").onclick=()=>dialog?.close();
    const current=dialog;
    dialog.addEventListener("close",()=>{if(dialog===current){close();if(opener?.isConnected)opener.focus({preventScroll:true});}},{once:true});
    dialog.showModal();
    disposeDialog=bind(dialog.querySelector(".person-3d-stage"),id,true);
  }
  function scan(host) {
    sessions.get(host)?.();
    close();
    if(!root.PersonModels||!root.Arsenal3D)return;
    root.Arsenal3D.register("person",root.PersonModels.build);
    const disposers=[];
    host.querySelectorAll("[data-person-model]").forEach(stage=>{
      const id=stage.dataset.personModel;if(!root.PersonModels.meta(id))return;
      disposers.push(bind(stage,id,false));
      const button=stage.querySelector("[data-person-open]");
      if(button){const click=()=>open(id,button);button.addEventListener("click",click);disposers.push(()=>button.removeEventListener("click",click));}
    });
    sessions.set(host,()=>disposers.forEach(fn=>fn()));
  }
  function dispose(host){sessions.get(host)?.();sessions.delete(host);close();}
  return {scan,dispose,close,initial,change};
});
