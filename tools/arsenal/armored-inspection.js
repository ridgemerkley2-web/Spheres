'use strict';
const workshop=ArmoredWorkshop,byId=id=>document.getElementById(id),vehicle=byId('vehicle'),lod=byId('lod'),part=byId('part'),finish=byId('finish'),wear=byId('wear'),host=byId('modelHost');
let storage=null,restored;
try{storage=globalThis.localStorage;if(!storage)throw new Error('Storage unavailable');restored=workshop.readDrafts(storage.getItem(workshop.DRAFT_STORAGE_KEY));}
catch{storage=null;restored={...workshop.readDrafts(null),status:'unavailable'};}
const drafts=new Map(Object.entries(restored.state.drafts));
const storageLocked=restored.status==='unsupported';
vehicle.value=restored.state.activeFamily;
const initial=drafts.get(vehicle.value)||workshop.visualDraft(vehicle.value);
finish.value=initial.finish;wear.value=initial.wear;lod.value=String(initial.lod);
let draft=workshop.create(vehicle.value,initial.components),mesh=null;
function draftStatus(message){const status=byId('draft-status');if(status)status.textContent=message;}
const restoreMessages={empty:'Browser-only visual draft · not saved yet.',restored:'Restored from this browser · visual draft only.',repaired:'Restored browser draft · invalid selections reset.',invalid:'Saved draft could not be restored · vehicle preset loaded.',unsupported:'Saved draft format is unsupported · changes last for this visit.',unavailable:'Browser storage unavailable · draft lasts for this visit.'};
draftStatus(restoreMessages[restored.status]);
function rememberCurrent(){drafts.set(draft.platform,workshop.visualDraft(draft.platform,{components:draft.components,finish:finish.value,wear:wear.value,lod:Number(lod.value)}));}
function saveCurrent(){
 rememberCurrent();
 if(storageLocked){draftStatus(restoreMessages.unsupported);return;}
 if(!storage){draftStatus(restoreMessages.unavailable);return;}
 try{storage.setItem(workshop.DRAFT_STORAGE_KEY,JSON.stringify({version:workshop.DRAFT_SCHEMA_VERSION,activeFamily:draft.platform,drafts:Object.fromEntries(drafts)}));draftStatus('Saved in this browser · visual draft only.');}
 catch{draftStatus('Browser save unavailable · current changes last for this visit.');}
}
const specification=()=>({...draft,lod:Number(lod.value),components:{...draft.components}});
const controller=EquipmentModel.mount(host,specification());
function showComponents(){
 const container=byId('components');container.replaceChildren();
 for(const field of workshop.fields(draft)){
  const label=document.createElement('label'),select=document.createElement('select');label.append(document.createTextNode(field.label));
  select.replaceChildren(...field.choices.map(c=>new Option(c.label,c.value)));select.value=draft.components[field.id];select.disabled=field.choices.length===1;select.title=field.reason;
  select.addEventListener('change',()=>{
   const before=draft.components;draft=workshop.edit(draft,field.id,select.value);
   const also=workshop.fields(draft).filter(f=>f.id!==field.id&&draft.components[f.id]!==before[f.id]);
   byId('component-notice').textContent=also.length?'Also updated: '+also.map(f=>f.label.toLowerCase()).join(', ')+'.':'';
   showComponents();update();saveCurrent();
   byId('components').querySelector(`[data-component="${field.id}"]`)?.focus();
  });select.dataset.component=field.id;label.append(select);container.append(label);
 }
}
function update(resetSelection=false){
 const previous=resetSelection?null:mesh?.parts.find(p=>p.name===part.value);
 mesh=EquipmentMesh.build(specification());controller.update(specification());
 const selected=previous&&(mesh.parts.find(p=>p.name===previous.name)||mesh.parts.find(p=>p.slot===previous.slot));
 part.replaceChildren(new Option('Whole vehicle',''),...mesh.parts.map(p=>new Option(p.label||p.name,p.name)));part.value=selected?.name||'';controller.selectPart(part.value);
 controller.setFinish(finish.value);controller.setWear(wear.value);
 const role=workshop.catalog[draft.platform];byId('role-tag').textContent=role.tag;byId('role-description').textContent=role.description;
 const stats=byId('stats');stats.replaceChildren();
 for(const [label,value] of [['Triangles',mesh.triangleCount.toLocaleString()],['Selectable parts',String(mesh.parts.length)]]){const dt=document.createElement('dt'),dd=document.createElement('dd');dt.textContent=label;dd.textContent=value;stats.append(dt,dd);}
 document.querySelectorAll('[data-vehicle]').forEach(button=>button.setAttribute('aria-pressed',String(button.dataset.vehicle===draft.platform)));
}
function changeVehicle(){
 if(!Object.hasOwn(workshop.catalog,vehicle.value)){vehicle.value=draft.platform;return;}
 rememberCurrent();const restoredDraft=drafts.get(vehicle.value)||workshop.visualDraft(vehicle.value);
 draft=workshop.create(vehicle.value,restoredDraft.components);finish.value=restoredDraft.finish;wear.value=restoredDraft.wear;lod.value=String(restoredDraft.lod);
 byId('component-notice').textContent='';showComponents();update(true);saveCurrent();
}
vehicle.addEventListener('change',changeVehicle);lod.addEventListener('change',()=>{update();saveCurrent();});finish.addEventListener('change',()=>{controller.setFinish(finish.value);saveCurrent();});wear.addEventListener('change',()=>{controller.setWear(wear.value);saveCurrent();});part.addEventListener('change',()=>controller.selectPart(part.value));
byId('reset-spec').addEventListener('click',()=>{draft=workshop.create(draft.platform);byId('component-notice').textContent='';showComponents();update(true);saveCurrent();});
host.addEventListener('equipment-part-select',event=>{const name=event.detail?.part;if(typeof name==='string'&&mesh?.parts.some(p=>p.name===name))part.value=name;});
byId('export').addEventListener('click',()=>{
 const status=host.querySelector('[data-model-status]');let url=null,anchor=null;
 try{const bytes=controller.exportGlb();if(!bytes){status.textContent='No vehicle model is available to download.';return;}
  url=URL.createObjectURL(new Blob([bytes],{type:'model/gltf-binary'}));anchor=document.createElement('a');anchor.href=url;anchor.download=`spheres-${draft.platform}-${finish.value}-${wear.value}.glb`;document.body.appendChild(anchor);anchor.click();status.textContent='Vehicle model download started.';
 }catch(error){status.textContent='The model could not be exported. Your design is still available.';}
 finally{anchor?.remove();if(url)setTimeout(()=>URL.revokeObjectURL(url),1000);}
});
for(const [id,role] of Object.entries(workshop.catalog)){
 const button=document.createElement('button'),tag=document.createElement('span'),name=document.createElement('strong'),action=document.createElement('span');button.className='fleet-card';button.dataset.vehicle=id;tag.className='role-tag';tag.textContent=role.tag;name.textContent=role.name;action.textContent='Inspect vehicle →';button.append(tag,name,action);
 button.addEventListener('click',()=>{vehicle.value=id;changeVehicle();host.scrollIntoView({block:'start'});});byId('fleet').append(button);
}
showComponents();update();controller.zoom(-1);
