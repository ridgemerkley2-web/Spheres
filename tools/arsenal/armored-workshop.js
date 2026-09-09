/* Art preview choices transcribed from equipment_ground.rs / equipment_specs.rs.
 * These are equipment pairings, not a second simulation: capacity, research,
 * costs and campaign eligibility are still evaluated by the native designer. */
(function(root,factory){const api=factory();if(typeof module==='object'&&module.exports)module.exports=api;else root.ArmoredWorkshop=api;})(typeof globalThis!=='undefined'?globalThis:this,function(){
  'use strict';
  const common={mobility:'engine_diesel_600',transmission:'transmission_manual',suspension:'suspension_torsion',protection:'ground_armor_light',active_protection:'aps_none',sensors:'optics_day',fire_control:'fcs_basic',communications:'comms_radio'};
  const catalog={
    ground_ifv:{name:'Infantry fighting vehicle',tag:'Tracked · Infantry support',description:'An autocannon turret above a protected troop compartment, with a rear ramp for the infantry it carries.',defaults:{tracks:'tracks_standard',turret:'ground_turret_autocannon',armament:'ground_gun_25',ammunition:'ground_ammo_autocannon',troop_compartment:'ground_troops_standard'},weapons:['ground_gun_25','ground_gun_35'],mounts:['ground_turret_autocannon'],loads:['ground_ammo_autocannon']},
    ground_apc:{name:'Armored personnel carrier',tag:'Wheeled · Troop transport',description:'A roomy troop carrier with a compact protected weapon station. Six-wheel and eight-wheel running gear give it a distinct stance.',defaults:{wheels:'ground_wheels_standard',turret:'ground_station_mg',armament:'ground_mg_127',ammunition:'ground_ammo_ball',troop_compartment:'ground_troops_standard'},weapons:['ground_mg_127'],mounts:['ground_station_mg'],loads:['ground_ammo_ball']},
    ground_recon:{name:'Reconnaissance vehicle',tag:'Wheeled · Observation',description:'A compact scout with a choice of weapon stations and an optional elevated sensor mast.',defaults:{wheels:'ground_wheels_standard',turret:'ground_station_mg',armament:'ground_mg_127',ammunition:'ground_ammo_ball',recon_package:'ground_recon_observer'},weapons:['ground_mg_127','ground_gun_25','ground_gun_35'],mounts:['ground_station_mg','ground_turret_autocannon'],loads:['ground_ammo_ball','ground_ammo_autocannon']},
    ground_artillery:{name:'Self-propelled artillery',tag:'Tracked · Fire support',description:'A long howitzer, a substantial rear turret and dedicated loading equipment distinguish this fire-support vehicle.',defaults:{mobility:'engine_diesel_900',tracks:'tracks_standard',turret:'ground_turret_howitzer',armament:'ground_howitzer_122',ammunition:'ground_ammo_he',artillery_loader:'ground_loader_manual'},weapons:['ground_howitzer_122','ground_howitzer_155'],mounts:['ground_turret_howitzer'],loads:['ground_ammo_he','ground_ammo_guided']},
    ground_air_defense:{name:'Mobile air defense',tag:'Tracked · Air defense',description:'A radar-equipped carrier with twin cannon or a missile launcher. Missile equipment uses tracking radar and digital fire control.',defaults:{tracks:'tracks_standard',turret:'ground_turret_aa',armament:'ground_aa_gun',ammunition:'ground_ammo_aa',radar:'ground_radar_search'},weapons:['ground_aa_gun','ground_aa_missiles'],mounts:['ground_turret_aa'],loads:['ground_ammo_aa','ground_ammo_missiles']}
  };
  const labels={engine_diesel_600:'600 hp diesel',engine_diesel_900:'900 hp diesel',engine_diesel_1200:'1,200 hp diesel',ground_engine_750:'Managed 750 hp diesel',transmission_manual:'Manual gearbox',transmission_auto:'Automatic cross-drive',ground_transmission_electric:'Electronic automatic gearbox',tracks_standard:'Standard steel tracks',tracks_wide:'Wide ground-contact tracks',tracks_padded:'Rubber-padded tracks',ground_wheels_standard:'Six-wheel drive',ground_wheels_runflat:'Eight-wheel run-flat drive',suspension_torsion:'Torsion-bar suspension',suspension_hydro:'Hydropneumatic suspension',ground_armor_light:'Light armored hull',ground_armor_modular:'Modular applique armor',aps_none:'No active protection',aps_soft:'Soft-kill countermeasures',aps_hard:'Hard-kill active protection',optics_day:'Daylight observation optics',optics_night:'Night-vision observation optics',optics_thermal:'Thermal panoramic sight',fcs_basic:'Basic gun control',fcs_stabilized:'Stabilized gun control',fcs_digital:'Digital ballistic fire control',comms_radio:'Field radio package',comms_data:'Tactical data integration',ground_comms_secure:'Secure ground radio',ground_comms_network:'Networked ground command',ground_troops_standard:'Standard troop compartment',ground_troops_protected:'Protected troop compartment',ground_recon_observer:'Scout observation package',ground_recon_mast:'Elevated sensor mast',ground_loader_manual:'Manual artillery loading',ground_loader_assisted:'Assisted artillery loading',ground_radar_search:'Search radar',ground_radar_tracking:'Search and tracking radar',ground_turret_autocannon:'Autocannon turret',ground_station_mg:'Protected machine-gun station',ground_turret_howitzer:'Howitzer turret',ground_turret_aa:'Air-defense weapon mount',ground_gun_25:'25 mm autocannon',ground_gun_35:'35 mm autocannon',ground_mg_127:'12.7 mm machine gun',ground_howitzer_122:'122 mm howitzer',ground_howitzer_155:'155 mm howitzer',ground_aa_gun:'Twin air-defense cannon',ground_aa_missiles:'Short-range missile launcher',ground_ammo_autocannon:'Mixed autocannon load',ground_ammo_ball:'Machine-gun ammunition',ground_ammo_he:'High-explosive artillery load',ground_ammo_guided:'Guided artillery load',ground_ammo_aa:'Air-defense cannon load',ground_ammo_missiles:'Short-range missile load'};
  const slots=[['turret','Weapon mount'],['armament','Main weapon'],['ammunition','Ammunition'],['protection','Armor'],['mobility','Powerpack'],['transmission','Transmission'],['tracks','Tracks'],['wheels','Wheels'],['suspension','Suspension'],['active_protection','Protection system'],['sensors','Observation optics'],['fire_control','Fire control'],['communications','Communications'],['troop_compartment','Troop compartment'],['recon_package','Reconnaissance equipment'],['artillery_loader','Loading equipment'],['radar','Radar']];
  const shared={mobility:['engine_diesel_600','engine_diesel_900','ground_engine_750'],transmission:['transmission_manual','transmission_auto','ground_transmission_electric'],tracks:['tracks_standard','tracks_wide','tracks_padded'],wheels:['ground_wheels_standard','ground_wheels_runflat'],suspension:['suspension_torsion','suspension_hydro'],protection:['ground_armor_light','ground_armor_modular'],active_protection:['aps_none','aps_soft','aps_hard'],sensors:['optics_day','optics_night','optics_thermal'],fire_control:['fcs_basic','fcs_stabilized','fcs_digital'],communications:['comms_radio','comms_data','ground_comms_secure','ground_comms_network'],troop_compartment:['ground_troops_standard','ground_troops_protected'],recon_package:['ground_recon_observer','ground_recon_mast'],artillery_loader:['ground_loader_manual','ground_loader_assisted'],radar:['ground_radar_search','ground_radar_tracking']};
  function vehicle(platform){if(!Object.hasOwn(catalog,platform))throw new RangeError('Unknown armored vehicle');return catalog[platform];}
  const record=value=>value!==null&&typeof value==='object'&&!Array.isArray(value);
  // Restored values are data, not executable property accessors. This also
  // keeps direct JavaScript callers from supplying inherited component choices.
  function ownValue(value,key){if(!record(value))return undefined;try{const descriptor=Object.getOwnPropertyDescriptor(value,key);return descriptor&&Object.hasOwn(descriptor,'value')?descriptor.value:undefined;}catch{return undefined;}}
  function defaults(platform){return {...common,...vehicle(platform).defaults};}
  function allowed(platform,slot){const v=vehicle(platform);if(!Object.hasOwn(defaults(platform),slot))return [];if(slot==='turret')return [...v.mounts];if(slot==='armament')return [...v.weapons];if(slot==='ammunition')return [...v.loads];if(slot==='mobility'&&platform==='ground_artillery')return [...shared.mobility,'engine_diesel_1200'];return [...(shared[slot]||[])];}
  function reconcile(platform,components,slot){
    const c={...components};
    if(platform==='ground_recon'){
      if(slot==='turret'||slot==='ammunition'){
        const cannon=slot==='turret'?c.turret==='ground_turret_autocannon':c.ammunition==='ground_ammo_autocannon';
        c.armament=cannon?(['ground_gun_25','ground_gun_35'].includes(c.armament)?c.armament:'ground_gun_25'):'ground_mg_127';
      }
      const cannon=c.armament!=='ground_mg_127';c.turret=cannon?'ground_turret_autocannon':'ground_station_mg';c.ammunition=cannon?'ground_ammo_autocannon':'ground_ammo_ball';
    }
    if(platform==='ground_air_defense'){
      if(slot==='ammunition')c.armament=c.ammunition==='ground_ammo_missiles'?'ground_aa_missiles':'ground_aa_gun';
      const missile=c.armament==='ground_aa_missiles';c.ammunition=missile?'ground_ammo_missiles':'ground_ammo_aa';
      if(missile){c.radar='ground_radar_tracking';c.fire_control='fcs_digital';}
    }
    if(platform==='ground_artillery'&&c.ammunition==='ground_ammo_guided')c.fire_control='fcs_digital';
    return c;
  }
  function create(platform,components={}){const c=defaults(platform);for(const key of Object.keys(c)){const value=ownValue(components,key);if(allowed(platform,key).includes(value))c[key]=value;}return {platform,components:reconcile(platform,c)};}
  function edit(spec,slot,value){if(!allowed(spec.platform,slot).includes(value))throw new RangeError('Component is unavailable for this vehicle');const c=create(spec.platform,spec.components).components;c[slot]=value;return {platform:spec.platform,components:reconcile(spec.platform,c,slot)};}
  function fields(spec){return slots.filter(([id])=>Object.hasOwn(defaults(spec.platform),id)).map(([id,label])=>{
    let ids=allowed(spec.platform,id),reason='';
    if(id==='fire_control'&&(spec.components.armament==='ground_aa_missiles'||spec.components.ammunition==='ground_ammo_guided')){ids=['fcs_digital'];reason='Required by the selected guided weapon.';}
    if(id==='radar'&&spec.components.armament==='ground_aa_missiles'){ids=['ground_radar_tracking'];reason='Required by the missile launcher.';}
    return {id,label,reason,choices:ids.map(value=>({value,label:labels[value]}))};
  });}
  const DRAFT_STORAGE_KEY='spheres.armored-workshop.drafts',DRAFT_SCHEMA_VERSION=1;
  function visualDraft(platform,input={}){
    const finish=ownValue(input,'finish'),wear=ownValue(input,'wear'),lod=ownValue(input,'lod');
    return {components:create(platform,ownValue(input,'components')).components,
      finish:['olive','sand','woodland','winter'].includes(finish)?finish:'olive',
      wear:['factory','service','field'].includes(wear)?wear:'service',
      lod:Number.isInteger(lod)&&lod>=0&&lod<=2?lod:0};
  }
  function sameData(a,b){
    if(a===b)return true;if(!record(a)||!record(b))return false;
    const keys=Object.keys(a);return keys.length===Object.keys(b).length&&keys.every(key=>Object.hasOwn(b,key)&&sameData(a[key],b[key]));
  }
  function readDrafts(raw){
    const empty=()=>({version:DRAFT_SCHEMA_VERSION,activeFamily:'ground_ifv',drafts:{}});
    if(raw===null||raw===undefined)return {status:'empty',state:empty()};
    if(typeof raw!=='string'||raw.length>50000)return {status:'invalid',state:empty()};
    let saved;try{saved=JSON.parse(raw);}catch{return {status:'invalid',state:empty()};}
    if(!record(saved))return {status:'invalid',state:empty()};
    const version=ownValue(saved,'version'),savedDrafts=ownValue(saved,'drafts'),activeFamily=ownValue(saved,'activeFamily');
    if(version!==DRAFT_SCHEMA_VERSION)return {status:Number.isInteger(version)&&version>0?'unsupported':'invalid',state:empty()};
    if(!record(savedDrafts))return {status:'invalid',state:empty()};
    const state=empty();
    if(typeof activeFamily==='string'&&Object.hasOwn(catalog,activeFamily))state.activeFamily=activeFamily;
    // Iterate the catalogue rather than persisted keys: unsupported families,
    // slots and values cannot enter the active preview or a subsequent save.
    for(const platform of Object.keys(catalog)){const savedDraft=ownValue(savedDrafts,platform);if(record(savedDraft))state.drafts[platform]=visualDraft(platform,savedDraft);}
    return {status:sameData(saved,state)?'restored':'repaired',state};
  }
  function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
  return freeze({catalog,labels,defaults,allowed,create,edit,fields,DRAFT_STORAGE_KEY,DRAFT_SCHEMA_VERSION,visualDraft,readDrafts});
});
