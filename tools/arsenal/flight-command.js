'use strict';
(() => {
  const $=id=>document.getElementById(id);
  const missions=[
    ['defend','Defend skies','Requires fighters',false],
    ['army','Support army','Tactical aircraft',true],
    ['strike','Strike target','Tactical aircraft',true],
    ['scout','Scout area','Requires recon aircraft',false],
    ['transport','Transport','Requires transports',false],
    ['fleet','Support fleet','Requires maritime aircraft',false]
  ];
  let selected=null,pending=null,controller=null,mesh=null;
  const host=$('model-host');
  function page(id){
    document.querySelectorAll('.page').forEach(el=>el.hidden=el.id!==id);
    document.querySelectorAll('[data-page]').forEach(el=>el.setAttribute('aria-pressed',String(el.dataset.page===id)));
    if(id==='aircraft') {if(!controller)build();else controller.resize();}
  }
  document.querySelectorAll('[data-page]').forEach(b=>b.addEventListener('click',()=>page(b.dataset.page)));
  $('inspect-aircraft').addEventListener('click',()=>page('aircraft'));
  for(const [id,label,hint,compatible] of missions){
    const b=document.createElement('button');b.dataset.mission=id;b.setAttribute('aria-pressed','false');
    const text=document.createElement('span'),small=document.createElement('small');text.textContent=label;small.textContent=hint;b.append(text,small);
    b.addEventListener('click',()=>{selected=id;document.querySelectorAll('[data-mission]').forEach(el=>el.setAttribute('aria-pressed',String(el===b)));$('order-hint').textContent=compatible?'Choose an area, then review the assignment.':hint+'. This tactical squadron cannot fly that mission.';$('review-mission').disabled=!compatible;});
    $('missions').append(b);
  }
  $('review-mission').disabled=true;
  $('review-mission').addEventListener('click',()=>{
    const mission=missions.find(m=>m[0]===selected);if(!mission?.[3])return;
    pending={mission:mission[1],area:$('area').selectedOptions[0].textContent,intensity:$('intensity').value,reachable:$('area').value!=='distant'};
    $('review-title').textContent=pending.mission;
    const dl=document.createElement('dl');dl.className='review-facts';
    for(const [key,value] of [['Aircraft','8 ready'],['Area',pending.area],['Commitment',pending.intensity],['Reach',pending.reachable?'Within demo range':'Outside demo range'],['Support',pending.intensity==='Intense'?'Higher demand':'Routine demand']]){const dt=document.createElement('dt'),dd=document.createElement('dd');dt.textContent=key;dd.textContent=value;dl.append(dt,dd);}
    $('review-body').replaceChildren(dl);
    if(!pending.reachable){const p=document.createElement('p');p.textContent='Choose a closer area. This squadron cannot reach the distant region from Northfield.';$('review-body').append(p);}
    $('confirm-mission').disabled=!pending.reachable;$('mission-review').showModal();
  });
  $('confirm-mission').addEventListener('click',()=>{
    if(!pending?.reachable)return;
    $('assignment').textContent=pending.mission;$('map-caption').textContent=`${pending.mission} · ${pending.area}`;
    $('flight-path').setAttribute('d',pending.area==='Home region'?'M156 205C75 85 250 80 235 200S120 290 156 205':'M156 205Q220 72 335 177');
    $('flight-path').setAttribute('opacity','1');
    const entry=document.createElement('div');entry.className='report';entry.textContent=`Preview assignment: ${pending.mission} · ${pending.area} · ${pending.intensity}. 8 aircraft assigned; no campaign spending or combat simulated.`;
    if(!$('report-list').querySelector('.report'))$('report-list').replaceChildren();$('report-list').prepend(entry);
    while($('report-list').children.length>12)$('report-list').lastElementChild.remove();
    $('mission-review').close();$('order-hint').textContent='Preview order assigned. You can review it in Reports.';pending=null;
  });
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
    for(const [slot,name,choices] of fields){const label=document.createElement('label'),select=document.createElement('select');label.textContent=name;select.dataset.slot=slot;
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
  reset();
})();
