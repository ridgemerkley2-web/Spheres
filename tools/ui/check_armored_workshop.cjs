'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),{createRequire}=require('node:module');
const workshop=require('../arsenal/armored-workshop.js');
const nativeGround=fs.readFileSync(path.join(__dirname,'../../spheres-sim/src/equipment_ground.rs'),'utf8');
const nativeSpecs=fs.readFileSync(path.join(__dirname,'../../spheres-sim/src/equipment_specs.rs'),'utf8');
const nativeComponents=nativeSpecs+'\n'+nativeGround+'\n'+fs.readFileSync(path.join(__dirname,'../../spheres-sim/src/equipment.rs'),'utf8');
const quoted=text=>[...text.matchAll(/"([^"]+)"/g)].map(m=>m[1]);
const nativeSlots=Object.fromEntries([...nativeGround.matchAll(/pub const (GROUND_[A-Z_]+)_SLOTS:[^=]+= \[([^\]]+)\];/g)].map(m=>[m[1].toLowerCase(),quoted(m[2])]));
const nativeLabels=new Map([...nativeComponents.matchAll(/component!\("([^"]+)","([^"]+)","([^"]+)"/g)].map(m=>[m[1],{name:m[2],slot:m[3]}]));
const pairs=text=>Object.fromEntries([...text.matchAll(/\("([^"]+)","([^"]+)"\)/g)].map(m=>[m[1],m[2]]));
function nativeDefaults(platform){
  const base=nativeSpecs.match(/pub fn tank_spec[\s\S]*?components:\[([\s\S]*?)\]\.into_iter/)[1];
  const common=nativeGround.match(/for \(slot,id\) in \[([^\]]+)\]/)[1];
  const changes=nativeGround.slice(nativeGround.indexOf('let changes:'));
  const row=platform==='ground_air_defense'?changes.match(/_ => &\[([^\]]+)\]/):changes.match(new RegExp('"'+platform+'" => &\\[([^\\]]+)\\]'));
  assert(row,'Native defaults row exists for '+platform);
  const result={...pairs(base),...pairs(common),...pairs(row[1])};
  if(platform==='ground_apc'||platform==='ground_recon')delete result.tracks;
  return result;
}
function validPairing(spec){
  const c=spec.components;
  if(['ground_gun_25','ground_gun_35'].includes(c.armament)){assert.equal(c.turret,'ground_turret_autocannon');assert.equal(c.ammunition,'ground_ammo_autocannon');}
  if(c.armament==='ground_mg_127'){assert.equal(c.turret,'ground_station_mg');assert.equal(c.ammunition,'ground_ammo_ball');}
  if(c.armament==='ground_aa_gun')assert.equal(c.ammunition,'ground_ammo_aa');
  if(c.armament==='ground_aa_missiles'){assert.equal(c.ammunition,'ground_ammo_missiles');assert.equal(c.radar,'ground_radar_tracking');assert.equal(c.fire_control,'fcs_digital');}
  if(c.ammunition==='ground_ammo_guided')assert.equal(c.fire_control,'fcs_digital');
  assert.deepEqual(Object.keys(c).sort(),[...nativeSlots[spec.platform]].sort());
  for(const [slot,id] of Object.entries(c))assert(workshop.allowed(spec.platform,slot).includes(id),spec.platform+' '+slot+' '+id);
}

test('workshop defaults and every displayed component label match the native specialist definitions',()=>{
  assert.deepEqual(Object.keys(workshop.catalog).sort(),Object.keys(nativeSlots).sort());
  for(const platform of Object.keys(nativeSlots)){
    assert.deepEqual(workshop.defaults(platform),nativeDefaults(platform));
    const spec=workshop.create(platform),fields=workshop.fields(spec);
    assert.equal(fields.length,13);validPairing(spec);
    for(const field of fields)for(const choice of field.choices){
      const native=nativeLabels.get(choice.value);assert(native,choice.value);assert.equal(native.slot,field.id);assert.equal(choice.label,native.name);
    }
  }
});

test('weapon choice lists match each native platform armament, mount and ammunition branch',()=>{
  for(const slot of ['turret','armament','ammunition']){
    const branch=nativeGround.slice(nativeGround.indexOf('"'+slot+'" => match platform'));
    const body=branch.slice(0,branch.indexOf('},'));
    for(const platform of Object.keys(nativeSlots)){
      const key=platform==='ground_air_defense'?'_':'"'+platform+'"';
      const expression=body.match(new RegExp(key+' => (?:matches!\\(c.id,([^)]*)\\)|c.id == "([^"]+)")'));
      assert(expression,platform+' '+slot);
      const expected=expression[2]?[expression[2]]:quoted(expression[1]);
      assert.deepEqual([...workshop.allowed(platform,slot)].sort(),expected.sort());
    }
  }
});

test('all common choices preserve native family isolation and cannot introduce tank components',()=>{
  const compatibility=nativeGround.slice(nativeGround.indexOf('pub fn component_compatible'));
  for(const platform of Object.keys(nativeSlots)){
    const original=workshop.create(platform),snapshot=JSON.stringify(original);
    for(const slot of nativeSlots[platform].filter(id=>!['turret','armament','ammunition'].includes(id))){
      const branch=compatibility.match(new RegExp('"'+slot+'" => matches!\\(c.id,([^)]*)\\)'));
      assert(branch,'Native shared compatibility branch '+slot);
      const expected=quoted(branch[1]);
      if(slot==='mobility'&&platform==='ground_artillery')expected.push('engine_diesel_1200');
      assert.deepEqual([...workshop.allowed(platform,slot)].sort(),expected.sort());
    }
    for(const slot of nativeSlots[platform])for(const choice of workshop.allowed(platform,slot)){
      const next=workshop.edit(original,slot,choice);validPairing(next);
      assert.equal(JSON.stringify(original),snapshot,'Editing must not mutate the cached family draft');
      assert.deepEqual(workshop.create(platform,next.components),next,'A canonical edit is stable on recreation');
    }
    assert.throws(()=>workshop.edit(original,'armament','gun_120'),RangeError);
    assert.throws(()=>workshop.edit(original,'protection','protection_heavy'),RangeError);
    assert.throws(()=>workshop.edit(original,'mobility','engine_turbine_1500'),RangeError);
    assert.equal(workshop.allowed(platform,'mobility').includes('engine_diesel_1200'),platform==='ground_artillery');
    const ignored=workshop.create(platform,{...original.components,radar:'ground_radar_tracking',troop_compartment:'ground_troops_protected',turret:'turret_casemate',unexpected:'value'});
    validPairing(ignored);assert.equal(ignored.components.unexpected,undefined);
  }
  assert.throws(()=>workshop.create('__proto__'),RangeError);
  assert.equal(Object.isFrozen(workshop.catalog.ground_recon.defaults),true);
});

test('recon edits through each of its three paired selectors produce matching mounts and loads',()=>{
  let spec=workshop.edit(workshop.create('ground_recon'),'sensors','optics_night');
  spec=workshop.edit(spec,'armament','ground_gun_35');validPairing(spec);
  spec=workshop.edit(spec,'ammunition','ground_ammo_ball');assert.equal(spec.components.armament,'ground_mg_127');validPairing(spec);
  spec=workshop.edit(spec,'turret','ground_turret_autocannon');assert.equal(spec.components.armament,'ground_gun_25');validPairing(spec);
  spec=workshop.edit(spec,'armament','ground_gun_35');
  spec=workshop.edit(spec,'ammunition','ground_ammo_autocannon');assert.equal(spec.components.armament,'ground_gun_35','Same ammunition preserves the chosen cannon caliber');
  spec=workshop.edit(spec,'turret','ground_station_mg');validPairing(spec);
  assert.equal(spec.components.sensors,'optics_night','Pairing changes preserve independent observation choices');
  assert.equal(spec.components.armament,'ground_mg_127');
});

test('missile and guided-artillery changes constrain required sensors and release choices when reversed',()=>{
  for(const selector of ['armament','ammunition']){
    let spec=workshop.edit(workshop.create('ground_air_defense'),selector,selector==='armament'?'ground_aa_missiles':'ground_ammo_missiles');validPairing(spec);
    for(const slot of ['radar','fire_control']){const field=workshop.fields(spec).find(f=>f.id===slot);assert.equal(field.choices.length,1);assert.match(field.reason,/Required/);}
    spec=workshop.edit(spec,'radar','ground_radar_search');validPairing(spec);
    spec=workshop.edit(spec,'fire_control','fcs_basic');validPairing(spec);
    spec=workshop.edit(spec,selector,selector==='armament'?'ground_aa_gun':'ground_ammo_aa');validPairing(spec);
    assert.equal(workshop.fields(spec).find(f=>f.id==='radar').choices.length,2);
    assert.equal(workshop.fields(spec).find(f=>f.id==='fire_control').choices.length,3);
    spec=workshop.edit(workshop.edit(spec,'radar','ground_radar_search'),'fire_control','fcs_basic');validPairing(spec);
    assert.deepEqual(spec,workshop.create('ground_air_defense'));
  }
  let artillery=workshop.edit(workshop.create('ground_artillery'),'ammunition','ground_ammo_guided');validPairing(artillery);
  artillery=workshop.edit(artillery,'armament','ground_howitzer_155');validPairing(artillery);
  artillery=workshop.edit(artillery,'ammunition','ground_ammo_he');
  artillery=workshop.edit(artillery,'fire_control','fcs_basic');validPairing(artillery);
  assert.equal(artillery.components.armament,'ground_howitzer_155','Unguided ammunition does not discard independent caliber selection');
});

function harness(options={}){
  // Execute the real workshop with the existing real-controller no-WebGL DOM
  // fixture, real specialist geometry, and actual GLB serialization.
  const file=path.join(__dirname,'check_equipment_model.cjs'),tests=fs.readFileSync(file,'utf8');
  const scope={require:createRequire(file),assert,vm,source:fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/equipment-model.js'),'utf8'),bounds:{min:[-2,0,-4],max:[2,4,6]}};
  vm.runInNewContext(tests.slice(tests.indexOf('function fixture('),tests.indexOf("test('material readiness"))+';this.make=fixture;',scope);
  const f=scope.make({noGraphics:true}),nodes={},timers=[],created=[],revoked=[],anchors=[],mounts=[];let clickFails=false;
  const create=f.doc.createElement;
  f.doc.createElement=tag=>{const n=create(tag);n.tagName=tag.toUpperCase();n.append=(...children)=>children.forEach(c=>n.appendChild(c));
    if(tag==='a'){anchors.push(n);n.click=()=>{assert.equal(n.parentNode,f.doc.body);if(clickFails)throw Error('Blocked download');};}return n;};
  f.doc.createTextNode=text=>({textContent:text});
  for(const id of ['vehicle','lod','part','finish','wear','components','stats','reset-spec','export','component-notice','draft-status','role-tag','role-description','fleet'])nodes[id]=f.doc.createElement(['vehicle','lod','part','finish','wear'].includes(id)?'select':'div');
  nodes.vehicle.value='ground_ifv';nodes.lod.value='2';nodes.part.value='';nodes.finish.value='olive';nodes.wear.value='service';nodes.modelHost=f.host;
  f.doc.getElementById=id=>{assert(nodes[id],id);return nodes[id];};
  const components=()=>nodes.components.children.flatMap(label=>label.children).filter(n=>n.tagName==='SELECT');
  nodes.components.querySelector=selector=>components().find(n=>selector===`[data-component="${n.dataset.component}"]`)||null;
  f.doc.querySelectorAll=selector=>{assert.equal(selector,'[data-vehicle]');return nodes.fleet.children;};
  f.host.scrollIntoView=()=>{};
  const emit=f.host.dispatchEvent.bind(f.host);f.host.dispatchEvent=e=>{emit(e);f.host.listeners.get(e.type)?.(e);};
  f.ctx.EquipmentMesh=require('../../spheres-web/ui/equipment-mesh.js');f.ctx.EquipmentExport=require('../../spheres-web/ui/equipment-export.js');
  f.ctx.ArmoredWorkshop=workshop;f.ctx.EquipmentModel={mount:(host,spec)=>{assert.equal(host,f.host);mounts.push(JSON.parse(JSON.stringify(spec)));f.controller.update(spec);return f.controller;}};
  if(options.storageAccessError)Object.defineProperty(f.ctx,'localStorage',{get(){throw Error('Storage access denied');}});
  else if(Object.hasOwn(options,'storage'))f.ctx.localStorage=options.storage;
  f.ctx.Option=function(text,value){const o=f.doc.createElement('option');o.textContent=text;o.value=value;return o;};
  f.ctx.URL={createObjectURL:blob=>{assert(blob instanceof Blob);const url='blob:test-'+created.length;created.push({url,blob});return url;},revokeObjectURL:url=>revoked.push(url)};
  f.ctx.Blob=Blob;f.ctx.setTimeout=fn=>{timers.push(fn);return timers.length;};
  vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../arsenal/armored-inspection.js'),'utf8'),f.ctx,{filename:'armored-inspection.js'});
  const change=(node,value)=>{assert(node);assert(!node.disabled,'A real user cannot edit a disabled selector');node.value=value;node.listeners.get('change')({type:'change'});};
  const component=id=>components().find(n=>n.dataset.component===id);
  return {...f,nodes,timers,created,revoked,anchors,mounts,change,component,click:id=>nodes[id].listeners.get('click')(),failClick:value=>{clickFails=value;}};
}
function jsonOf(bytes){const length=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength).getUint32(12,true);return JSON.parse(new TextDecoder().decode(bytes.subarray(20,20+length)));}
function colorsOf(bytes){
  const jsonLength=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength).getUint32(12,true),json=jsonOf(bytes);
  const acc=json.accessors[json.meshes[0].primitives[0].attributes.COLOR_0],view=json.bufferViews[acc.bufferView];
  assert.equal(acc.componentType,5126);assert.equal(acc.type,'VEC3');
  return new Float32Array(bytes.buffer,bytes.byteOffset+28+jsonLength+view.byteOffset+(acc.byteOffset||0),acc.count*3);
}

test('actual workshop preserves each family draft, announces paired changes and restores focus',()=>{
  const h=harness();try{
    h.change(h.nodes.vehicle,'ground_recon');h.change(h.component('armament'),'ground_gun_35');
    assert.equal(h.component('turret').value,'ground_turret_autocannon');assert.equal(h.component('ammunition').value,'ground_ammo_autocannon');
    assert.match(h.nodes['component-notice'].textContent,/weapon mount.*ammunition/);assert.equal(h.doc.activeElement,h.component('armament'));
    h.change(h.nodes.vehicle,'ground_apc');assert.equal(h.component('armament').value,'ground_mg_127');assert.equal(h.component('armament').disabled,true);assert.equal(h.component('recon_package'),undefined);
    h.change(h.component('wheels'),'ground_wheels_runflat');h.change(h.nodes.vehicle,'ground_recon');assert.equal(h.component('armament').value,'ground_gun_35');assert.equal(h.component('wheels').value,'ground_wheels_standard');
    h.click('reset-spec');assert.equal(h.component('armament').value,'ground_mg_127');
    h.change(h.nodes.vehicle,'ground_apc');assert.equal(h.component('wheels').value,'ground_wheels_runflat','Reset applies only to the current family');
    assert.equal(h.nodes.fleet.children.filter(n=>n.attrs['aria-pressed']==='true').length,1);
  }finally{h.controller.dispose();}
});

test('real part selection survives detail and same-slot edits, then clears on family or preset changes',()=>{
  const h=harness();try{
    h.change(h.nodes.vehicle,'ground_recon');const original=h.controller.selectPart('armament',true);assert(original);assert.equal(h.nodes.part.value,original.name);
    h.change(h.nodes.lod,'1');assert.equal(h.nodes.part.value,original.name);
    h.change(h.component('armament'),'ground_gun_35');const selected=h.controller.selectPart(h.nodes.part.value);assert.equal(selected.slot,'armament');assert.notEqual(selected.name,original.name);
    h.host.dispatchEvent({type:'equipment-part-select',detail:{part:'no longer exists'}});assert.equal(h.nodes.part.value,selected.name);
    h.click('reset-spec');assert.equal(h.nodes.part.value,'');assert.equal(h.nodes.lod.value,'1');
    h.controller.selectPart('armament',true);h.change(h.nodes.vehicle,'ground_artillery');assert.equal(h.nodes.part.value,'');
  }finally{h.controller.dispose();}
});

test('downloaded GLB contains the current paired vehicle, current LOD and actual painted vertex colors',async()=>{
  const h=harness();try{
    h.change(h.nodes.vehicle,'ground_air_defense');h.change(h.component('armament'),'ground_aa_missiles');h.change(h.nodes.finish,'sand');h.change(h.nodes.wear,'field');
    h.change(h.nodes.lod,'2');
    const before=jsonOf(h.controller.exportGlb());assert.equal(before.meshes[0].extras.specification.platform,'ground_air_defense');validPairing(before.meshes[0].extras.specification);
    h.click('export');assert.equal(h.anchors.at(-1).download,'spheres-ground_air_defense-sand-field.glb');assert.equal(h.anchors.at(-1).isConnected,false);
    const bytes=new Uint8Array(await h.created[0].blob.arrayBuffer()),download=jsonOf(bytes);assert.deepEqual(download,before);
    const rawMesh=require('../../spheres-web/ui/equipment-mesh.js').build({...download.meshes[0].extras.specification,lod:2});
    const expectedPaint=require('../../spheres-web/ui/equipment-model.js').finishColors(rawMesh.colors,'sand');
    assert.deepEqual(colorsOf(bytes),expectedPaint);assert.notDeepEqual(colorsOf(bytes),rawMesh.colors,'The downloaded model carries the selected paint');
    const farCount=download.meshes[0].extras.triangleCount;h.change(h.nodes.lod,'1');assert(jsonOf(h.controller.exportGlb()).meshes[0].extras.triangleCount>farCount);
    h.timers.shift()();assert.deepEqual(h.revoked,[h.created[0].url]);
    assert.equal(h.component('radar').disabled,true);assert.equal(h.component('fire_control').disabled,true);
    h.change(h.component('ammunition'),'ground_ammo_aa');assert.equal(h.component('radar').disabled,false);assert.equal(h.component('fire_control').disabled,false);
  }finally{h.controller.dispose();}
});

test('download failures preserve the specialist draft and clean up all allocated browser handles',()=>{
  const h=harness(),original=h.controller.exportGlb;try{
    h.change(h.nodes.vehicle,'ground_artillery');h.change(h.component('ammunition'),'ground_ammo_guided');
    h.controller.exportGlb=()=>{throw Error('Synthetic export failure');};assert.doesNotThrow(()=>h.click('export'));assert.match(h.status.textContent,/could not be exported/);assert.equal(h.created.length,0);
    h.controller.exportGlb=()=>null;h.click('export');assert.match(h.status.textContent,/No vehicle model/);
    h.controller.exportGlb=original;h.failClick(true);assert.doesNotThrow(()=>h.click('export'));assert.equal(h.anchors.at(-1).isConnected,false);h.timers.shift()();assert.equal(h.revoked.length,1);
    assert.equal(h.component('ammunition').value,'ground_ammo_guided');assert.equal(h.component('fire_control').value,'fcs_digital');
  }finally{h.controller.dispose();}
});

function memoryStorage(initial=null){
  let value=initial,failRead=false,failWrite=false;const reads=[],writes=[];
  return {reads,writes,get value(){return value;},set failRead(v){failRead=v;},set failWrite(v){failWrite=v;},getItem(key){reads.push(key);if(failRead)throw Error('Storage read denied');return value;},setItem(key,next){writes.push({key,value:next});if(failWrite)throw Error('Quota exceeded');value=next;}};
}
const stored=(activeFamily,drafts)=>JSON.stringify({version:workshop.DRAFT_SCHEMA_VERSION,activeFamily,drafts});
const currentRecord=h=>({components:Object.fromEntries(workshop.fields(workshop.create(h.nodes.vehicle.value)).map(field=>[field.id,h.component(field.id).value])),finish:h.nodes.finish.value,wear:h.nodes.wear.value,lod:Number(h.nodes.lod.value)});

test('persisted visual records normalize unknown and obsolete values and restore required component pairings',()=>{
  assert.equal(workshop.DRAFT_STORAGE_KEY,'spheres.armored-workshop.drafts');assert.equal(workshop.DRAFT_SCHEMA_VERSION,1);
  for(const [platform,components] of [
    ['ground_recon',{armament:'ground_gun_35',turret:'ground_station_mg',ammunition:'ground_ammo_ball',wheels:'ground_wheels_runflat'}],
    ['ground_air_defense',{armament:'ground_aa_missiles',ammunition:'ground_ammo_aa',radar:'ground_radar_search',fire_control:'fcs_basic'}],
    ['ground_artillery',{armament:'ground_howitzer_155',ammunition:'ground_ammo_guided',fire_control:'fcs_basic'}]
  ]){
    const input={components:{...components,protection:'retired_armor_id',engine:'engine_turbine_1500'},finish:'polished_gold',wear:'destroyed',lod:42},before=JSON.stringify(input),result=workshop.visualDraft(platform,input);
    validPairing({platform,components:result.components});assert.equal(result.components.protection,'ground_armor_light');assert.equal(result.components.engine,undefined);assert.equal(result.finish,'olive');assert.equal(result.wear,'service');assert.equal(result.lod,0);assert.equal(JSON.stringify(input),before);
  }
  for(const lod of [-1,1.5,'2',NaN,Infinity,null,true])assert.equal(workshop.visualDraft('ground_ifv',{lod}).lod,0);
});
test('visual draft validation ignores inherited keys and accessors without invoking their code',()=>{
  let reads=0;const inherited=Object.create({components:{armament:'ground_gun_35'},finish:'sand',wear:'field',lod:2});assert.deepEqual(workshop.visualDraft('ground_recon',inherited),workshop.visualDraft('ground_recon'));
  const input={},components=Object.create({wheels:'ground_wheels_runflat'});Object.defineProperty(components,'armament',{enumerable:true,get(){reads++;throw Error('Getter must not execute');}});Object.defineProperty(input,'finish',{enumerable:true,get(){reads++;throw Error('Getter must not execute');}});input.components=components;
  assert.doesNotThrow(()=>workshop.visualDraft('ground_recon',input));assert.equal(reads,0);assert.equal(workshop.visualDraft('ground_recon',input).components.armament,'ground_mg_127');assert.equal(workshop.visualDraft('ground_recon',input).components.wheels,'ground_wheels_standard');
  assert.throws(()=>workshop.visualDraft('__proto__'),RangeError);
});
test('stored schema validation rejects corrupt or future payloads and removes unknown families without prototype effects',()=>{
  assert.equal(workshop.readDrafts(null).status,'empty');
  for(const raw of ['{broken','null','[]','"text"','x'.repeat(50001),JSON.stringify({version:1,drafts:[]}),JSON.stringify({version:0,drafts:{}})]){const result=workshop.readDrafts(raw);assert.equal(result.status,'invalid');assert.equal(result.state.activeFamily,'ground_ifv');assert.deepEqual(result.state.drafts,{});}
  const future=JSON.stringify({version:2,activeFamily:'ground_apc',drafts:{ground_apc:{components:{wheels:'ground_wheels_runflat'}}}});assert.equal(workshop.readDrafts(future).status,'unsupported');assert.deepEqual(workshop.readDrafts(future).state.drafts,{});
  const raw='{"version":1,"activeFamily":"__proto__","drafts":{"__proto__":{"polluted":true},"ground_future":{"components":{"armament":"unknown"}},"ground_recon":{"components":{"__proto__":{"polluted":true},"armament":"ground_gun_35"},"finish":"sand","wear":"field","lod":1}}}';
  const result=workshop.readDrafts(raw);assert.equal(result.status,'repaired');assert.equal(result.state.activeFamily,'ground_ifv');assert.deepEqual(Object.keys(result.state.drafts),['ground_recon']);validPairing({platform:'ground_recon',components:result.state.drafts.ground_recon.components});assert.equal({}.polluted,undefined);assert.equal(Object.hasOwn(result.state.drafts,'__proto__'),false);
  const canonical=JSON.stringify(result.state);assert.equal(workshop.readDrafts(canonical).status,'restored');assert.equal(JSON.stringify(workshop.readDrafts(canonical).state),canonical);
});
test('actual page reload restores separate components, finish, condition and detail for each family before mounting',()=>{
  const storage=memoryStorage(),a=harness({storage});let recon,apc;
  try{
    assert.equal(storage.writes.length,0,'Opening an empty workshop must not save an unsolicited draft');
    a.change(a.nodes.vehicle,'ground_recon');a.change(a.component('armament'),'ground_gun_35');a.change(a.component('recon_package'),'ground_recon_mast');a.change(a.nodes.finish,'woodland');a.change(a.nodes.wear,'field');a.change(a.nodes.lod,'1');recon=currentRecord(a);
    a.change(a.nodes.vehicle,'ground_apc');a.change(a.component('wheels'),'ground_wheels_runflat');a.change(a.nodes.finish,'sand');a.change(a.nodes.wear,'factory');a.change(a.nodes.lod,'2');apc=currentRecord(a);a.change(a.nodes.vehicle,'ground_recon');
    assert(storage.writes.length>0);assert(storage.writes.every(row=>row.key===workshop.DRAFT_STORAGE_KEY));
  }finally{a.controller.dispose();}
  const writes=storage.writes.length,b=harness({storage});try{
    assert.equal(storage.writes.length,writes,'Reading a saved draft does not rewrite it');assert.equal(b.nodes.vehicle.value,'ground_recon');assert.equal(b.mounts[0].platform,'ground_recon');assert.equal(b.mounts[0].components.armament,'ground_gun_35');assert.equal(b.mounts[0].components.turret,'ground_turret_autocannon');assert.equal(b.mounts[0].lod,1);assert.deepEqual(currentRecord(b),recon);validPairing(b.mounts[0]);
    b.change(b.nodes.vehicle,'ground_apc');assert.deepEqual(currentRecord(b),apc);b.change(b.nodes.vehicle,'ground_recon');assert.deepEqual(currentRecord(b),recon);
  }finally{b.controller.dispose();}
});
test('startup repairs incompatible saved weapons before the first model and never writes while repairing',()=>{
  for(const [platform,components] of [
    ['ground_recon',{armament:'ground_gun_35',turret:'ground_station_mg',ammunition:'ground_ammo_ball',tracks:'tracks_wide'}],
    ['ground_air_defense',{armament:'ground_aa_missiles',radar:'ground_radar_search',fire_control:'fcs_basic',ammunition:'ground_ammo_aa'}],
    ['ground_artillery',{ammunition:'ground_ammo_guided',fire_control:'fcs_basic',turret:'turret_heavy'}]
  ]){const raw=stored(platform,{[platform]:{components,finish:'sand',wear:'field',lod:2}}),storage=memoryStorage(raw),h=harness({storage});try{
    assert.equal(storage.writes.length,0);assert.equal(storage.value,raw);assert.equal(h.mounts[0].platform,platform);validPairing(h.mounts[0]);assert.equal(h.mounts[0].lod,2);assert.equal(h.nodes.finish.value,'sand');assert.equal(h.nodes.wear.value,'field');
    assert.match(h.nodes['draft-status'].textContent,/invalid selections reset/i);
  }finally{h.controller.dispose();}}
});
test('missing, denied or corrupt storage keeps a functional session without startup writes',()=>{
  const denied=memoryStorage();denied.failRead=true;const corrupt=memoryStorage('{not-json');
  for(const options of [{},{storageAccessError:true},{storage:denied},{storage:corrupt}]){const h=harness(options);try{
    if(options.storage)assert.equal(options.storage.writes.length,0);validPairing(h.mounts[0]);h.change(h.nodes.vehicle,'ground_recon');h.change(h.component('armament'),'ground_gun_35');h.change(h.nodes.finish,'winter');h.change(h.nodes.vehicle,'ground_apc');h.change(h.nodes.vehicle,'ground_recon');assert.equal(h.component('armament').value,'ground_gun_35');assert.equal(h.nodes.finish.value,'winter');assert(h.controller.exportGlb());
  }finally{h.controller.dispose();}}
});
test('quota failure preserves live per-family drafts, reports session-only status and does not overwrite the last saved bytes',()=>{
  const existing=stored('ground_recon',{ground_recon:workshop.visualDraft('ground_recon',{finish:'sand',lod:2})}),storage=memoryStorage(existing);storage.failWrite=true;const h=harness({storage});try{
    h.change(h.component('armament'),'ground_gun_35');h.change(h.nodes.finish,'woodland');h.change(h.nodes.wear,'field');h.change(h.nodes.vehicle,'ground_apc');h.change(h.nodes.vehicle,'ground_recon');assert.equal(h.component('armament').value,'ground_gun_35');assert.equal(h.nodes.finish.value,'woodland');assert.equal(h.nodes.wear.value,'field');assert.equal(storage.value,existing);assert(storage.writes.length>0);assert.match(h.nodes['draft-status'].textContent,/session|not.*saved|unable|unavailable/i);
  }finally{h.controller.dispose();}
  storage.failWrite=false;const reloaded=harness({storage});try{assert.equal(reloaded.component('armament').value,'ground_mg_127');assert.equal(reloaded.nodes.finish.value,'sand');}finally{reloaded.controller.dispose();}
});
test('an unsupported future schema is preserved byte-for-byte while edits and preset reset remain session-only',()=>{
  const raw=JSON.stringify({version:99,activeFamily:'ground_artillery',drafts:{ground_artillery:{components:{armament:'future_howitzer'}}},futureFields:{keep:true}}),storage=memoryStorage(raw),h=harness({storage});try{
    assert.equal(storage.writes.length,0);assert.equal(h.mounts[0].platform,'ground_ifv');validPairing(h.mounts[0]);h.change(h.nodes.vehicle,'ground_recon');h.change(h.component('armament'),'ground_gun_35');h.change(h.nodes.finish,'sand');h.click('reset-spec');assert.equal(h.component('armament').value,'ground_mg_127');assert.equal(storage.writes.length,0);assert.equal(storage.value,raw);assert.match(h.nodes['draft-status'].textContent,/newer|version|unsupported/i);
  }finally{h.controller.dispose();}
});
test('resetting one family persists its default components while retaining its finish/detail and every other family draft',()=>{
  const recon=workshop.visualDraft('ground_recon',{components:{armament:'ground_gun_35',recon_package:'ground_recon_mast'},finish:'woodland',wear:'field',lod:1}),apc=workshop.visualDraft('ground_apc',{components:{wheels:'ground_wheels_runflat'},finish:'sand',wear:'factory',lod:2}),storage=memoryStorage(stored('ground_recon',{ground_recon:recon,ground_apc:apc})),h=harness({storage});try{
    h.click('reset-spec');assert.equal(h.nodes.vehicle.value,'ground_recon');assert.deepEqual(currentRecord(h),{components:workshop.defaults('ground_recon'),finish:'woodland',wear:'field',lod:1});const saved=JSON.parse(storage.value);assert.deepEqual(saved.drafts.ground_apc,apc);assert.deepEqual(saved.drafts.ground_recon.components,workshop.defaults('ground_recon'));
  }finally{h.controller.dispose();}
  const reload=harness({storage});try{assert.equal(reload.component('armament').value,'ground_mg_127');assert.equal(reload.nodes.finish.value,'woodland');reload.change(reload.nodes.vehicle,'ground_apc');assert.deepEqual(currentRecord(reload),apc);}finally{reload.controller.dispose();}
});
