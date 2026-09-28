'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const crypto=require('node:crypto'),zlib=require('node:zlib');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const attr=(key,value)=>`[${key}=${JSON.stringify(String(value))}]`;

// Exact-native S20 route. The caller owns build/save identity and a hasTouch
// browser context. Only visible controls write saves, replace the campaign, or
// request previews. Fault tests delay/abort real GETs; no data is fabricated.
module.exports=async function navigationCloseout({page,tap,state,read,shot,evidence}){
  const proof=evidence.navigation={passed:false,checks:[],observations:[],faults:[],saves:[],focus:[]};
  const requestStart=evidence.requests.length;
  const idle=()=>page.waitForFunction(()=>!SESSION.busy&&!advancing&&!pendingAdvance&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending);
  const active=()=>page.evaluate(()=>({id:document.activeElement?.id||'',key:document.activeElement?.dataset.mapDetailFocus||'',
    text:document.activeElement?.textContent?.trim().slice(0,120)||'',tag:document.activeElement?.tagName||'',
    visible:!!document.activeElement?.getClientRects().length,insideProvince:!!document.activeElement?.closest('#provinceDossier')}));
  async function expectFocus(locator,label){
    const observation={label,...await active()};proof.focus.push(observation);
    assert(await locator.evaluate(el=>el===document.activeElement),label+'; active='+JSON.stringify(observation));
  }
  async function keyboardTo(locator,label){
    await locator.waitFor({state:'visible'});
    assert(await locator.isEnabled(),label+' must be enabled');
    for(let i=0;i<220;i++){
      if(await locator.evaluate(el=>el===document.activeElement))return;
      await page.keyboard.press('Tab');
    }
    throw Error('Keyboard could not reach '+label+'; active='+JSON.stringify(await active()));
  }
  async function activate(locator,mode,label){
    evidence.actions.push({navigation:label,input:mode});
    if(mode==='keyboard'){await keyboardTo(locator,label);await page.keyboard.press('Enter');}
    else await locator.tap();
  }
  async function escape(label){evidence.actions.push({key:'Escape',label});await page.keyboard.press('Escape');}
  async function closeRooms(keepMapDetails=false){
    for(const [panel,button] of [['#guidanceDialog','#guidanceDialog [data-guidance-close]'],
      ['#equipmentRoom','[data-equipment-close]'],['#productionPanel','#productionClose'],
      ['#cabinetDrawer','#cabinetDrawer [data-close-drawers]'],...(keepMapDetails?[]:[['#provinceDossier','#provinceDossier .province-close'],
      ['#mapCityCard','#mapCityCard [data-map-detail-focus="city-close"]']])])
      if(await page.locator(panel).isVisible())await tap(button);
  }
  function record(name,data){
    const file=name+'.json',bytes=Buffer.from(JSON.stringify(data,null,2)+'\n');
    fs.writeFileSync(path.join(evidence.out,file),bytes);
    proof.observations.push({file,bytes:bytes.length,sha256:hash(bytes)});return data;
  }
  async function openSaveScreen(keepMapDetails=false){
    await closeRooms(keepMapDetails);
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');
    await tap('#campaignsBtn');await tap('#openSavesBtn');
    await page.locator('#saveName').waitFor({state:'visible'});
  }
  async function resume(){
    await tap('#savedCampaigns [data-menu-back]');await tap('#continueBtn');await page.locator('#app').waitFor({state:'visible'});await idle();
  }
  async function save(slot){
    await openSaveScreen();await page.locator('#saveName').fill(slot);evidence.actions.push({fill:'#saveName',value:slot});
    const response=page.waitForResponse(r=>r.request().method()==='POST'&&new URL(r.url()).pathname==='/api/save');
    await tap('#saveNamedBtn');const saved=await response;assert(saved.ok(),await saved.text());
    await page.locator(`#saveSlots option[value="${slot}"]`).waitFor({state:'attached'});
    const raw=fs.readFileSync(path.join(evidence.run,'saves',slot+'.json')),data=JSON.parse(raw);
    assert.equal(data.format,'spheres-campaign');assert.equal(data.version,1);
    assert(Number.isSafeInteger(data.saved_unix)&&data.saved_unix>0&&data.saved_unix<=Math.floor(Date.now()/1000)+5);
    assert(data.world&&Array.isArray(data.history)&&Array.isArray(data.log)&&data.journey);
    const file=slot+'.json.gz',compressed=zlib.gzipSync(raw);fs.writeFileSync(path.join(evidence.out,file),compressed);
    proof.saves.push({slot,file,raw_bytes:raw.length,raw_sha256:hash(raw),gzip_sha256:hash(compressed),saved_unix:data.saved_unix});
    await resume();return data;
  }
  async function load(slot){
    const previous=(await state()).session_id;await openSaveScreen(true);
    await page.locator('#saveSlots').selectOption(slot);evidence.actions.push({select:'#saveSlots',value:slot});
    const response=page.waitForResponse(r=>r.request().method()==='POST'&&new URL(r.url()).pathname==='/api/load');
    await tap('#loadBtn');await tap('#campaignConfirmAccept');const loaded=await response;assert(loaded.ok(),await loaded.text());
    await page.locator('#app').waitFor({state:'visible'});await idle();
    await page.waitForFunction(old=>S?.session_id&&S.session_id!==old,previous);
    return {before:previous,after:(await state()).session_id};
  }
  async function finder(query,kind,id,mode){
    await activate(page.locator('#worldFindBtn'),mode,'Open Find');await page.locator('#worldFindInput').waitFor();
    await expectFocus(page.locator('#worldFindInput'),'Find enters its search input');
    await page.keyboard.type(query);evidence.actions.push({type:'#worldFindInput',value:query});
    const result=kind==='city'?page.locator('[data-find-kind="city"]').filter({has:page.getByText(query,{exact:true})}):
      page.locator('[data-find-kind="province"]'+attr('data-find-id',id));
    await result.waitFor({state:'visible'});await activate(result,mode,'Find '+kind+' '+query);
  }
  async function parisCity(mode){
    await finder('Paris','city',null,mode);await page.locator('#mapCityCard').waitFor({state:'visible'});
    assert.equal(await page.locator('#mapCityTitle').innerText(),'Paris');
    await expectFocus(page.locator('#mapCityTitle'),'City enters its heading');
    return await page.locator('#mapCityCard').innerText();
  }
  async function cityProvince(mode){
    await activate(page.locator('[data-map-detail-focus="city-province"]'),mode,'City → Province');
    await page.locator('#provinceDossier[aria-hidden="false"]').waitFor();
    await expectFocus(page.locator('#provinceTitle'),'Province enters its heading');
    return await page.locator('#provinceDossier').getAttribute('data-province');
  }
  async function readyProvince(id,label){
    await page.waitForFunction(did=>selectedDistrict===did&&PROVINCE_POPULATION?.id===did&&
      !PROVINCE_POPULATION.loading&&!PROVINCE_POPULATION.error, id);
    assert.equal(await page.locator('#provinceDossier').getAttribute('aria-busy'),'false');
    const native=record(label+'-native',await read('district-population/'+encodeURIComponent(id)));
    const adopted=await adoptedProvince();
    assert.deepEqual(adopted,native,'Province must display the exact native response');
    assert.equal(await page.locator('#provinceTitle').innerText(),native.name);
    const owner=await page.locator('#provinceDossier .province-owner').innerText();
    assert(owner.includes(native.owner_name)&&owner.includes(id),'Current native owner and district');
    const rows=(native.economy?.projects||[]).filter(p=>p&&typeof p==='object'&&!Array.isArray(p));
    const construction=p=>p.sector==='construction'||String(p.id||'').startsWith('construction:')||p.status==='building';
    const producing=p=>!construction(p)&&p.classification!=='pending_order'&&
      ((Number.isFinite(p.output_quantity_daily)&&p.output_quantity_daily>0)||(Number.isFinite(p.gross_output_daily_bn)&&p.gross_output_daily_bn>0));
    const counts=[rows.filter(construction).length,rows.filter(producing).length,
      rows.filter(p=>['blocked','paused','slowed','stalled','inactive'].includes(p.status)).length];
    assert.deepEqual(await page.locator('#provinceDossier .pe-activity-counts dd').allTextContents(),counts.map(String));
    proof.checks.push({label,district:id,owner:native.owner,activity:counts});return native;
  }
  async function adoptedProvince(){
    const {reading,date}=await page.evaluate(()=>({reading:JSON.parse(JSON.stringify(PROVINCE_POPULATION)),date:S.date}));
    assert.equal(reading.read_date,date,'Province labels the actual campaign reading date');
    assert((await page.locator('#provinceDossier [role="status"]').allTextContents()).includes('Reading for '+date));
    // The sole UI annotation is validated separately; all native fields must
    // still match the independent endpoint response without normalization.
    delete reading.read_date;return reading;
  }
  async function narrowShot(name){
    assert(await page.locator('#provinceDossier').evaluate(el=>el.scrollWidth<=el.clientWidth+1),'Province horizontal overflow');
    await shot(name);
  }
  async function closeProvince(label){
    await escape(label);await page.locator('#provinceDossier').waitFor({state:'hidden'});
    await expectFocus(page.locator('#worldFindBtn'),'Province closes to surviving Find opener');
  }
  async function buildReview(id,mode,label){
    await activate(page.locator('[data-map-detail-focus="province-build"]'),mode,'Build here');
    await page.waitForFunction(()=>PROD.open&&PROD.data&&!PROD.loading&&!PROD.stale);
    assert.equal(await page.evaluate(()=>PROD.provinceFilter),id);
    const native=record(label+'-production',await read('production'));
    const kind=native.catalog.find(c=>c.kind!=='starter_industry'&&c.actions?.start!==false&&c.eligible_provinces?.includes(id))?.kind;
    assert(kind,'A native eligible project exists for review');
    await activate(page.locator(attr('data-prod-kind',kind)),mode,'Choose '+kind);
    await activate(page.locator(attr('data-prod-province',id)),mode,'Review local project effects');
    await page.waitForFunction(()=>constructionPreviewCurrent());
    const preview=record(label+'-construction-review',await page.evaluate(()=>JSON.parse(JSON.stringify(PROD.preview))));
    assert.equal(preview.district,id);assert.equal(preview.project_kind,kind);
    assert.equal(await page.locator('.construction-impact-scope').count(),2);
    await shot(label+'-review');await escape('Cancel unconfirmed project review');
    await page.locator('#productionPanel').waitFor({state:'hidden'});
    await expectFocus(page.locator('[data-map-detail-focus="province-build"]'),'Cancelled construction returns to Build here');
  }
  const populationPattern='**/api/district-population/**';
  async function holdPopulation(id,label){
    let release,captured,finished,used=false;
    const gate=new Promise(r=>{release=r;}),seen=new Promise(r=>{captured=r;}),done=new Promise(r=>{finished=r;});
    const fault={label,kind:'delay_actual_get',district:id};proof.faults.push(fault);
    const handler=async route=>{
      if(used||decodeURIComponent(new URL(route.request().url()).pathname.split('/').at(-1))!==id)return route.continue();
      used=true;
      try{
        const response=await route.fetch();assert(response.ok());
        fault.native=record(label+'-held-response',await response.json());fault.url=route.request().url();captured();
        await gate;await route.fulfill({response});finished();
      }catch(error){fault.error=String(error);captured();finished();throw error;}
    };
    await page.route(populationPattern,handler);
    return {seen:async()=>{await seen;assert(!fault.error,fault.error);},release:async()=>{
      const completion=page.waitForEvent('requestfinished',{predicate:r=>r.url()===fault.url});
      release();await done;await completion;await page.unroute(populationPattern,handler);
      await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
      assert(!fault.error,fault.error);
    }};
  }
  async function nativeAction(root,attribute,predicate){
    const buttons=page.locator(root).locator('['+attribute+']'),values=await buttons.evaluateAll((els,key)=>els.map(e=>e.getAttribute(key)),attribute);
    const index=values.findIndex(raw=>{try{return predicate(JSON.parse(raw));}catch{return false;}});
    assert(index>=0,'Native navigation action '+attribute);return buttons.nth(index);
  }
  async function equipmentTab(name,mode){
    if(mode==='touch'){await activate(page.locator('#equipmentRoom nav [data-equipment-tab="'+name+'"]'),mode,'Equipment '+name);return;}
    await keyboardTo(page.locator('#equipmentRoom .eq-tabs [aria-selected="true"]'),'Equipment workflow tabs');
    for(let i=0;i<10;i++){
      if(await page.locator('#equipmentRoom nav [data-equipment-tab="'+name+'"]').getAttribute('aria-selected')==='true')return;
      evidence.actions.push({key:'ArrowRight',label:'Equipment workflow'});await page.keyboard.press('ArrowRight');
    }
    throw Error('Equipment workflow cannot reach '+name);
  }

  await idle();await closeRooms();await page.setViewportSize({width:1440,height:1000});
  assert.equal((await state()).player,'France','This qualification consumes the recorded France campaign');
  const before=await save('s20-navigation-before');
  const city=await parisCity('keyboard');
  // The current city selection is a read-only geometry observation, not a
  // test-written district ID or a direct navigation call.
  const paris=await page.evaluate(()=>{const c=ui.selectedCity,w=Globe3D.project(c.lon,c.lat),n=countryAt(w[0],w[1]);return n&&districtAt(n,w[0],w[1])?.id;});
  assert(paris,'Paris maps to a real province');
  const delayed=await holdPopulation(paris,'desktop-async-focus');await cityProvince('keyboard');await delayed.seen();
  assert.equal(await page.locator('#provinceDossier').getAttribute('aria-busy'),'true');
  assert((await page.locator('#provinceDossier [role="status"]').allTextContents()).some(t=>/Reading/i.test(t)));
  // Targeted focus setup is used only to check asynchronous replacement. All
  // navigation into and out of the page uses keyboard/touch activation.
  await page.locator('[data-map-detail-focus="province-center"]').focus();
  evidence.actions.push({focus_for_assertion:'province-center'});await delayed.release();
  const nativeParis=await readyProvince(paris,'desktop-paris');assert(city.includes(nativeParis.owner_name),'City identifies current owner');
  await expectFocus(page.locator('[data-map-detail-focus="province-center"]'),'Async reading preserves focused control');
  await shot('s20-province-desktop');await buildReview(paris,'keyboard','s20-desktop');await closeProvince('Close desktop province');

  await page.setViewportSize({width:390,height:844});await parisCity('touch');await cityProvince('touch');
  await readyProvince(paris,'touch-paris');await narrowShot('s20-province-390');
  await buildReview(paris,'touch','s20-touch');await closeProvince('Keyboard Escape after touch navigation');

  let failed=false;
  const fail=async route=>{if(!failed&&decodeURIComponent(new URL(route.request().url()).pathname.split('/').at(-1))===paris){failed=true;return route.abort('failed');}return route.continue();};
  await page.route(populationPattern,fail);await parisCity('touch');await cityProvince('touch');
  await page.locator('[data-province-retry]').waitFor({state:'visible'});
  assert(failed);assert.equal(await page.locator('#provinceDossier').getAttribute('aria-busy'),'false');
  assert.equal(await page.locator('#provinceDossier .pe-activity-counts').count(),0,'Failed reading cannot show old activity');
  proof.faults.push({kind:'abort_actual_get',district:paris,retry_visible:true});await shot('s20-province-read-error');
  await page.unroute(populationPattern,fail);await activate(page.locator('[data-province-retry]'),'touch','Retry province reading');
  await readyProvince(paris,'retried-paris');await expectFocus(page.locator('#provinceTitle'),'Retry replaces removed control with heading focus');
  await closeProvince('Close recovered province');

  const production=record('navigation-production',await read('production'));
  const facility=production.completed.find(r=>r.outcome_actions?.some(a=>a.kind==='arms_plant'));
  assert(facility?.province?.id&&facility.province.id!==paris,'Earned Arms Plant in a second province');
  const other=facility.province.id,otherName=facility.province.name||other;
  await parisCity('touch');const cross=await holdPopulation(paris,'cross-province');await cityProvince('touch');await cross.seen();
  await closeProvince('Leave delayed province');await finder(otherName,'province',other,'touch');
  const otherNative=await readyProvince(other,'different-province');await cross.release();
  assert.deepEqual(await adoptedProvince(),otherNative,'Late Paris reading cannot overwrite another province');
  assert.equal(await page.locator('#provinceDossier').getAttribute('data-province'),other);await narrowShot('s20-cross-province-390');
  await closeProvince('Close second province');

  await parisCity('touch');const replacement=await holdPopulation(paris,'cross-campaign');await cityProvince('touch');await replacement.seen();
  proof.campaign_replacement=await load('s20-navigation-before');await replacement.release();
  assert(await page.locator('#provinceDossier').isHidden());
  assert.deepEqual(await page.evaluate(()=>({district:selectedDistrict,reading:PROVINCE_POPULATION})),{district:null,reading:null});
  proof.checks.push('A delayed prior-session reading cannot repopulate the replaced campaign');await shot('s20-campaign-replaced');

  await parisCity('touch');
  assert.equal(await page.evaluate(()=>ui.selectedCity?.name),'Paris');
  proof.selected_city_replacement=await load('s20-navigation-before');
  assert.equal(await page.evaluate(()=>ui.selectedCity),null,'Campaign replacement clears the selected city');
  assert(await page.locator('#mapCityCard').isHidden(),'Campaign replacement removes the old city card');
  proof.checks.push('Loading through ordinary campaign controls clears an open selected city');await shot('s20-city-campaign-replaced');

  await page.setViewportSize({width:1440,height:1000});await finder(otherName,'province',other,'keyboard');
  await readyProvince(other,'completed-facility-province');await shot('s20-operating-province');
  await activate(page.locator('#productionDockBtn'),'keyboard','Open Construction');
  await page.waitForFunction(()=>PROD.open&&PROD.data&&!PROD.loading&&!PROD.stale);
  if(await page.locator('[data-prod-built]').getAttribute('aria-expanded')!=='true')await activate(page.locator('[data-prod-built]'),'keyboard','Completed province upgrades');
  const completedRoot=attr('data-construction-completed',other);
  const outcome=await nativeAction(completedRoot,'data-construction-outcome',a=>a.district===other&&a.kind==='arms_plant');
  await activate(outcome,'keyboard','Inspect completed Arms Plant');await page.waitForFunction(()=>industryCurrent());
  const industry=record('navigation-industry',await read('industry'));assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(IDESK.data))),industry);
  const site='[data-industry-site='+JSON.stringify('site:'+other+':arms_plant')+']';await page.locator(site).waitFor({state:'visible'});await shot('s20-completed-arms-plant');
  const manage=await nativeAction(site,'data-industry-action',a=>a.action==='manufacture');
  await activate(manage,'keyboard','Manage equipment at completed plant');
  await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='companies');
  const equipment=record('navigation-equipment',await read('equipment'));
  assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),equipment);await shot('s20-company-destination');
  await equipmentTab('flight','keyboard');await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='flight');
  assert(equipment.flight.squadrons.length>0,'S19 supplied real squadrons');
  for(const squadron of equipment.flight.squadrons)assert((await page.locator('#equipmentRoom').innerText()).includes(squadron.name));
  await shot('s20-air-command-desktop');await page.setViewportSize({width:390,height:844});
  await activate(page.locator('[data-equipment-flight-page="reports"]'),'touch','Air command reports');await shot('s20-air-reports-390');
  assert(await page.locator('#equipmentRoom').evaluate(el=>el.scrollWidth<=el.clientWidth+1));
  await escape('Return from Air command');await page.locator('#equipmentRoom').waitFor({state:'hidden'});
  const returned=await active();assert(returned.visible&&returned.tag!=='BODY','Equipment close restores a visible control');proof.focus.push({label:'Air command return',...returned});

  const after=await save('s20-navigation-after');assert(after.saved_unix>=before.saved_unix);
  const expected=structuredClone(before),actual=structuredClone(after);delete expected.saved_unix;delete actual.saved_unix;
  assert.deepEqual(actual,expected,'Read/navigation/review/cancel must preserve the complete campaign envelope');
  const writes=evidence.requests.slice(requestStart);
  assert(writes.every(r=>['/api/save','/api/load','/api/construction-preview'].includes(r.route)),'Only visible saves, campaign load and read-only construction previews are permitted');
  assert(!writes.some(r=>r.route==='/api/advance'||r.route==='/api/command'));
  proof.permitted_requests=writes;proof.checks.push('Before/after saves match completely except validated storage timestamp; world, history, log and journey are unchanged');
  proof.passed=true;console.log('PASS: native map/province/facility/equipment navigation, keyboard/touch, delayed/error recovery and full saved-campaign purity');
};
