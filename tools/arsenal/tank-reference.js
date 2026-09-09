// The licensed reference has its own optional standards-based glTF viewer. It
// is loaded only on request and never registered as a configurable game tank.
const button=document.getElementById('load-reference'),area=document.getElementById('reference-area'),status=document.getElementById('reference-status');
let loading=false,viewer=null;
button.addEventListener('click',async()=>{
 if(loading)return;
 if(viewer){area.hidden=!area.hidden;button.textContent=area.hidden?'Inspect the textured reference ↗':'Hide the reference';return;}
 loading=true;button.disabled=true;area.hidden=false;status.textContent='Loading the textured tank reference…';
 try{
  await import('./vendor/model-viewer/model-viewer.min.js');
  const element=document.createElement('model-viewer');
  for(const [key,value] of Object.entries({src:'../../spheres-web/ui/tank-assets/strv103/strv103.glb',alt:'Textured 3D Stridsvagn 103 tank by canisferus. Drag to orbit and scroll to zoom.','camera-controls':'','touch-action':'pan-y','environment-image':'neutral','shadow-intensity':'1','shadow-softness':'0.8','exposure':'0.9','camera-orbit':'35deg 65deg auto','min-camera-orbit':'auto 10deg 35%','max-camera-orbit':'auto 90deg 200%','interaction-prompt':'none'}))element.setAttribute(key,value);
  element.addEventListener('load',()=>{status.textContent='Strv 103 · textured historical art reference · drag to rotate, scroll to zoom';});
  element.addEventListener('error',()=>{status.textContent='The reference could not load. Your configurable tank remains available above.';element.remove();viewer=null;button.textContent='Retry loading the reference';});
  document.getElementById('reference-viewer').replaceChildren(element);viewer=element;button.textContent='Hide the reference';
 }catch(error){status.textContent='The reference viewer could not load. You can still inspect the configurable tanks above.';button.textContent='Retry loading the reference';}
 finally{loading=false;button.disabled=false;}
});
