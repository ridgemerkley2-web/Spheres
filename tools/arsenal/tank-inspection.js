'use strict';
const baseline={mobility:'engine_diesel_900',transmission:'transmission_manual',tracks:'tracks_standard',suspension:'suspension_torsion',turret:'turret_standard',armament:'gun_105',ammunition:'ammo_mixed',protection:'protection_standard',active_protection:'aps_none',sensors:'optics_day',fire_control:'fcs_basic',communications:'comms_radio'};
const presets={tank_standard:{},tank_heavy:{mobility:'engine_diesel_1200',protection:'protection_heavy',turret:'turret_heavy',armament:'gun_120'},tank_light:{mobility:'engine_diesel_600',turret:'turret_compact',armament:'gun_90'},tank_destroyer:{turret:'turret_casemate',armament:'gun_120',ammunition:'ammo_penetrator'}};
const fields=[
 ['turret','Turret',[['turret_compact','Compact'],['turret_standard','Standard'],['turret_heavy','Heavy'],['turret_autoload','Autoloading'],['turret_casemate','Fixed casemate']]],
 ['armament','Main gun',[['gun_90','90 mm'],['gun_105','105 mm'],['gun_120','120 mm'],['gun_125','125 mm']]],
 ['protection','Armor',[['protection_standard','Standard'],['protection_heavy','Reinforced modular'],['protection_active','Active protection package']]],
 ['mobility','Powerpack',[['engine_diesel_600','600 hp diesel'],['engine_diesel_900','900 hp diesel'],['engine_diesel_1200','1,200 hp diesel'],['engine_turbine_1500','1,500 hp turbine']]],
 ['transmission','Transmission',[['transmission_manual','Manual'],['transmission_auto','Automatic']]],
 ['tracks','Tracks',[['tracks_standard','Standard'],['tracks_wide','Wide'],['tracks_padded','Rubber padded']]],
 ['suspension','Suspension',[['suspension_torsion','Torsion bar'],['suspension_hydro','Hydropneumatic']]],
 ['sensors','Optics',[['optics_day','Day sight'],['optics_night','Night sight'],['optics_thermal','Thermal sight']]],
 ['fire_control','Fire control',[['fcs_basic','Basic'],['fcs_stabilized','Stabilized'],['fcs_digital','Digital']]],
 ['active_protection','Protection system',[['aps_none','None'],['aps_soft','Soft kill'],['aps_hard','Hard kill']]],
 ['communications','Communications',[['comms_radio','Field radio'],['comms_data','Data link']]],
 ['ammunition','Ammunition stowage',[['ammo_mixed','Mixed purpose'],['ammo_penetrator','Penetrator'],['ammo_support','Support']]]
];
const vehicle=document.getElementById('vehicle'),lod=document.getElementById('lod'),part=document.getElementById('part'),finish=document.getElementById('finish'),wear=document.getElementById('wear');
let components={...baseline,...presets[vehicle.value]},currentMesh=null;
const spec=()=>({platform:vehicle.value,lod:Number(lod.value),components:{...components}});
const modelHost=document.getElementById('modelHost'),controller=EquipmentModel.mount(modelHost,spec());
function update({resetSelection=false}={}){
 const previous=resetSelection?null:currentMesh?.parts.find(p=>p.name===part.value);
 const mesh=EquipmentMesh.build(spec());controller.update(spec());currentMesh=mesh;
 const selected=previous&&(mesh.parts.find(p=>p.name===previous.name)||mesh.parts.find(p=>p.slot===previous.slot));
 part.replaceChildren(new Option('Whole vehicle',''),...mesh.parts.map(p=>new Option(p.label||p.name,p.name)));
 part.value=selected?.name||'';controller.selectPart(part.value);
 const stats=document.getElementById('stats');stats.replaceChildren();
 for(const [label,value] of [['Triangles',mesh.triangleCount.toLocaleString()],['Selectable parts',String(mesh.parts.length)],['Geometry',( (mesh.positions.byteLength+mesh.normals.byteLength+mesh.colors.byteLength)/1048576).toFixed(1)+' MiB']]){const dt=document.createElement('dt'),dd=document.createElement('dd');dt.textContent=label;dd.textContent=value;stats.append(dt,dd);}
 controller.setFinish(finish.value);controller.setWear?.(wear.value);
}
function resetComponents(){components={...baseline,...presets[vehicle.value]};const host=document.getElementById('components');host.replaceChildren();for(const [id,name,choices] of fields){const label=document.createElement('label'),select=document.createElement('select');label.append(document.createTextNode(name));select.replaceChildren(...choices.map(([value,text])=>new Option(text,value)));select.value=components[id];select.addEventListener('change',()=>{components[id]=select.value;update();});label.append(select);host.append(label);}update({resetSelection:true});}
vehicle.addEventListener('change',resetComponents);lod.addEventListener('change',update);finish.addEventListener('change',()=>controller.setFinish(finish.value));wear.addEventListener('change',()=>controller.setWear?.(wear.value));part.addEventListener('change',()=>controller.selectPart(part.value));document.getElementById('reset-spec').addEventListener('click',resetComponents);
modelHost.addEventListener('equipment-part-select',event=>{const name=event.detail?.part;if(typeof name==='string'&&currentMesh?.parts.some(p=>p.name===name))part.value=name;});
document.getElementById('export').addEventListener('click',()=>{
 const status=modelHost.querySelector('[data-model-status]');let url=null,anchor=null;
 try{
  const bytes=controller.exportGlb();if(!bytes){status.textContent='No tank model is available to download.';return;}
  url=URL.createObjectURL(new Blob([bytes],{type:'model/gltf-binary'}));anchor=document.createElement('a');
  anchor.href=url;anchor.download=`spheres-${vehicle.value}-${finish.value}.glb`;
  document.body.appendChild(anchor);anchor.click();status.textContent='Tank model download started.';
 }catch(error){status.textContent='The tank model could not be exported. Your design is still available.';}
 finally{anchor?.remove();if(url)setTimeout(()=>URL.revokeObjectURL(url),1000);}
});
resetComponents();
controller.zoom(-2);
