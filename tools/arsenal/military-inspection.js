'use strict';
const platforms=[['tank_standard','Main battle tank'],['tank_heavy','Heavy tank'],['tank_light','Light tank'],['tank_destroyer','Tank destroyer'],['ground_ifv','Infantry fighting vehicle'],['ground_apc','Armored personnel carrier'],['ground_recon','Reconnaissance vehicle'],['ground_artillery','Self-propelled artillery'],['ground_air_defense','Air-defense vehicle'],['air_light_attack','Light attack aircraft'],['air_tactical_strike','Tactical strike aircraft']];
const vehicle=document.getElementById('vehicle'),variant=document.getElementById('variant'),lod=document.getElementById('lod'),part=document.getElementById('part');
vehicle.replaceChildren(...platforms.map(([id,name])=>new Option(name,id)));
const tankBase={mobility:'engine_diesel_900',transmission:'transmission_manual',tracks:'tracks_standard',suspension:'suspension_torsion',turret:'turret_standard',armament:'gun_105',ammunition:'ammo_mixed',protection:'protection_standard',active_protection:'aps_none',sensors:'optics_day',fire_control:'fcs_basic',communications:'comms_radio'};
// Use the same distinct platform presets as the shipped GLB generator.
const tankPresets={tank_heavy:{mobility:'engine_diesel_1200',protection:'protection_heavy',turret:'turret_heavy',armament:'gun_120'},tank_light:{mobility:'engine_diesel_600',turret:'turret_compact',armament:'gun_90'},tank_destroyer:{turret:'turret_casemate',armament:'gun_120',ammunition:'ammo_penetrator'}};
function spec(){
  const platform=vehicle.value,tank=platform.startsWith('tank_'),air=platform.startsWith('air_');
  const components=tank?{...tankBase,...tankPresets[platform]}:{};
  if(variant.value==='upgraded')Object.assign(components,air?{air_engine:platform==='air_light_attack'?'air_engine_efficient':'air_engine_twin',air_avionics:'air_avionics_digital',air_payload:'air_payload_guided',air_fuel:'air_fuel_extended'}:tank?{mobility:'engine_diesel_1200',suspension:'suspension_hydro',active_protection:'aps_hard',sensors:'optics_thermal',fire_control:'fcs_digital',communications:'comms_data',tracks:'tracks_padded'}:{protection:'ground_armor_modular',communications:'ground_comms_secure',sensors:'optics_thermal'});
  return {platform,lod:Number(lod.value),components};
}
const controller=EquipmentModel.mount(document.getElementById('modelHost'),spec());
function update(){lod.disabled=vehicle.value.startsWith('air_');if(lod.disabled)lod.value='0';lod.title=lod.disabled?'These two aircraft currently have inspection geometry only.':'';const model=EquipmentMesh.build(spec());controller.update(spec());controller.selectPart('');part.replaceChildren(new Option('Whole vehicle',''),...model.parts.map(p=>new Option(p.label||p.name,p.name)));const stats=document.getElementById('stats');stats.replaceChildren();for(const text of [`${model.triangleCount.toLocaleString()} triangles`,`${model.parts.length} selectable parts`,`${(model.positions.byteLength+model.normals.byteLength+model.colors.byteLength)/1048576>=1?((model.positions.byteLength+model.normals.byteLength+model.colors.byteLength)/1048576).toFixed(1):'<1'} MiB geometry buffers`]){const p=document.createElement('p');p.textContent=text;stats.append(p);}}
vehicle.onchange=update;variant.onchange=update;lod.onchange=update;part.onchange=()=>controller.selectPart(part.value);
document.getElementById('finish').onchange=e=>controller.setFinish(e.target.value);
document.getElementById('modelHost').addEventListener('equipment-part-select',event=>{part.value=event.detail.part;});
document.getElementById('export').onclick=()=>{const bytes=controller.exportGlb();if(!bytes)return;const url=URL.createObjectURL(new Blob([bytes],{type:'model/gltf-binary'}));const a=document.createElement('a');a.href=url;a.download=`spheres-${vehicle.value}.glb`;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);};
update();
// Build a balanced set on demand; the complete catalogue remains available below.
const chosen=['arm_gen2','arm_gen3','inf_mech','f15e','f22','f35a','b2','ea18g','predator','nav_patrol','nav_escort','nav_blue','la_ssn','patriot','tomahawk','kh11'].map(id=>ArsenalModels.build(id));
document.getElementById('catalogue').replaceChildren(...chosen.map(g=>{const card=document.createElement('article');card.className='has-kit3d';const canvas=document.createElement('canvas');canvas.dataset.kit3d=g.id;canvas.setAttribute('aria-label',g.name+' 3D model');const title=document.createElement('h3');title.textContent=g.name;const detail=document.createElement('small');detail.textContent=`${g.cls} · ${(g.count/3).toLocaleString()} triangles`;card.append(canvas,title,detail);return card;}));
Arsenal3D.scan(document.getElementById('catalogue'));
