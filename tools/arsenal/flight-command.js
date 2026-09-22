'use strict';
(() => {
  const $=id=>document.getElementById(id);
  let controller=null,mesh=null;
  const host=$('model-host');
  const fields=[
    ['air_engine','Engines',[['air_engine_economical','Economical'],['air_engine_efficient','Efficient'],['air_engine_twin','Twin engine']]],
    ['air_wing','Wings',[['air_wing_straight','Straight'],['air_wing_stable','High stability'],['air_wing_swept','Swept']]],
    ['air_radar','Radar',[['air_radar_basic','Basic'],['air_radar_mapping','Terrain mapping']]],
    ['air_avionics','Avionics',[['air_avionics_analog','Analog'],['air_avionics_digital','Digital']]],
    ['air_countermeasures','Protection',[['air_countermeasures_basic','Chaff and flares'],['air_countermeasures_ecm','Electronic countermeasures']]],
    ['air_hardpoints','Weapon mounts',[['air_hardpoints_light','Light'],['air_hardpoints_heavy','Heavy']]],
    ['air_payload','Payload',[['air_payload_unguided','Unguided'],['air_payload_guided','Guided']]],
    ['air_fuel','Fuel',[['air_fuel_standard','Standard'],['air_fuel_extended','Extended']]]
  ];
  let components={};
  function build(){
    const spec={platform:$('platform').value,lod:Number($('detail').value),components};
    mesh=EquipmentMesh.build(spec);
    const previous=$('part').value;
    if(!controller){controller=EquipmentModel.mount(host,spec);}else controller.update(spec);
    $('part').replaceChildren(new Option('Whole aircraft',''),...mesh.parts.map(p=>new Option(p.label||p.name,p.name)));
    if(mesh.parts.some(p=>p.name===previous))$('part').value=previous;
    controller.selectPart($('part').value);
    $('mesh-count').textContent=`${mesh.triangleCount.toLocaleString()} triangles · ${mesh.parts.length} selectable assemblies · ${(mesh.positions.byteLength*3/1048576).toFixed(1)} MiB`;
  }
  function reset(){
    const defaults=EquipmentMesh.build({platform:$('platform').value,lod:2}).specification.components;components={...defaults};$('components').replaceChildren();
    const fighterFields=[
      ['air_engine','Engines',[['air_engine_interceptor','Interceptor'],['air_engine_interceptor_efficient','Efficient interceptor']]],
      ['air_wing','Wings',[['air_wing_interceptor','Interceptor wing']]],
      ['air_radar','Radar',[['air_radar_interceptor','Air search'],['air_radar_interceptor_tracking','Tracking radar']]],
      ['air_avionics','Avionics',[['air_avionics_interceptor','Interception suite'],['air_avionics_interceptor_digital','Digital interception']]],
      fields[4],['air_hardpoints','Weapon mounts',[['air_hardpoints_interceptor','Interceptor mounts']]],
      ['air_payload','Payload',[['air_payload_short_range','Short-range missiles'],['air_payload_medium_range','Medium-range missiles']]],fields[7]
    ];
    for(const [slot,name,choices] of $('platform').value==='air_fighter'?fighterFields:fields){const label=document.createElement('label'),select=document.createElement('select');label.textContent=name;select.dataset.slot=slot;
      const restricted=$('platform').value==='air_light_attack'?['air_engine_twin','air_wing_swept','air_hardpoints_heavy']:[];
      select.replaceChildren(...choices.filter(([id])=>!restricted.includes(id)).map(([id,text])=>new Option(text,id)));select.value=components[slot];select.addEventListener('change',()=>{components[slot]=select.value;build();});label.append(select);$('components').append(label);}
    if(controller)build();
  }
  $('platform').addEventListener('change',reset);$('detail').addEventListener('change',build);
  $('part').addEventListener('change',()=>controller?.selectPart($('part').value));
  host.addEventListener('equipment-part-select',e=>{if(mesh?.parts.some(p=>p.name===e.detail?.part))$('part').value=e.detail.part;});
  $('download').addEventListener('click',()=>{
    let url=null,a=null;
    try{
      const bytes=controller?.exportGlb();if(!bytes)throw new Error('No model');
      url=URL.createObjectURL(new Blob([bytes],{type:'model/gltf-binary'}));a=document.createElement('a');a.href=url;a.download=`spheres-${$('platform').value}.glb`;document.body.append(a);a.click();
      host.querySelector('[data-model-status]').textContent='Aircraft mesh download started.';
    }catch(error){host.querySelector('[data-model-status]').textContent='The mesh could not be downloaded. Your design is still available.';}
    finally{a?.remove();if(url)setTimeout(()=>URL.revokeObjectURL(url),1000);}
  });
  window.addEventListener('pagehide',()=>{controller?.dispose();controller=null;mesh=null;});
  window.addEventListener('pageshow',()=>{if(!$('aircraft').hidden&&!controller)build();});
  reset();build();
})();
