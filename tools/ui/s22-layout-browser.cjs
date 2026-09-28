'use strict';
const assert=require('node:assert/strict');
const {requestedCameraChange}=require('./s22-browser-contract.cjs');
module.exports=async function layout({page,tap,shot,evidence}){
  const proof=evidence.layout_functional={passed:false,profiles:[],scope:'Layout and native keyboard/touch/focus/scroll checks only; no FPS or input-latency qualification at these viewports.'};
  const settle=()=>page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));
  const camera=()=>page.evaluate(()=>({yaw:GLOBE.yaw,pitch:GLOBE.pitch,zoom:GLOBE.zoom}));
  const focused=selector=>page.locator(selector).evaluate(el=>el===document.activeElement);
  async function keyboardTo(selector){
    const target=page.locator(selector);await target.waitFor({state:'visible'});assert(await target.isEnabled());
    for(let i=0;i<220;i++){if(await target.evaluate(el=>el===document.activeElement))return;await page.keyboard.press('Tab');}
    throw Error('Keyboard cannot reach '+selector);
  }
  async function activate(selector,mode){
    evidence.actions.push({layout:selector,input:mode});
    if(mode==='keyboard'){await keyboardTo(selector);await page.keyboard.press('Enter');}else await page.locator(selector).tap();
  }
  async function panel(selector){
    const data=await page.locator(selector).evaluate(el=>{const r=el.getBoundingClientRect();return {
      text:el.innerText,rect:{left:r.left,right:r.right,top:r.top,bottom:r.bottom,width:r.width,height:r.height},
      client_width:el.clientWidth,scroll_width:el.scrollWidth,client_height:el.clientHeight,scroll_height:el.scrollHeight,scroll_top:el.scrollTop,
      document_width:document.documentElement.scrollWidth,viewport:{width:innerWidth,height:innerHeight},
      headings:[...el.querySelectorAll('h1,h2,h3')].map(n=>n.textContent.trim())};});
    assert(data.text.trim().length&&data.rect.width>0&&data.rect.height>0,selector+' must have visible content');
    assert(data.rect.left>=0&&data.rect.right<=data.viewport.width+1&&data.rect.top>=0&&data.rect.bottom<=data.viewport.height+1,selector+' must fit viewport');
    assert(data.scroll_width<=data.client_width+1,selector+' horizontal overflow');
    assert(data.document_width<=data.viewport.width+1,'Document horizontal overflow');return data;
  }
  async function scrollPanel(selector){
    const before=await panel(selector);
    if(before.scroll_height>before.client_height+1){
      const box=await page.locator(selector).boundingBox();await page.mouse.move(box.x+box.width/2,box.y+box.height/2);
      const delta=before.scroll_top>=before.scroll_height-before.client_height-1?-before.scroll_height:before.scroll_height;
      await page.mouse.wheel(0,delta);await page.waitForFunction(({selector,top})=>document.querySelector(selector).scrollTop!==top,{selector,top:before.scroll_top});
      const after=await panel(selector);assert.notEqual(after.scroll_top,before.scroll_top);return {before,after,scrolled:true,input:'native wheel',delta};
    }
    return {before,after:before,scrolled:false,reason:'Content fits the visible panel'};
  }
  async function closeRooms(){
    for(const [selector,button] of [['#equipmentRoom','[data-equipment-close]'],['#provinceDossier','#provinceDossier .province-close'],
      ['#mapCityCard','[data-map-detail-focus="city-close"]'],['#guidanceDialog','[data-guidance-close]'],['#cabinetDrawer','#cabinetDrawer [data-close-drawers]']])
      if(await page.locator(selector).isVisible())await tap(button);
  }
  for(const viewport of [{width:390,height:844},{width:3440,height:1440}]){
    await closeRooms();await page.setViewportSize(viewport);await settle();
    const row={viewport,dpr:await page.evaluate(()=>devicePixelRatio),checks:[],focus_return:[],screenshots:[]};proof.profiles.push(row);assert.equal(row.dpr,1);
    await page.evaluate(()=>{GLOBE.lookAt(2.3522,48.8566,8);GLOBE.render();});await settle();
    let before=await camera();await activate('[data-map-action="west"]','keyboard');await settle();
    row.keyboard={action:'west',before,after:await camera()};assert(requestedCameraChange('west',before,row.keyboard.after));
    before=await camera();await activate('[data-map-action="east"]','touch');await settle();
    row.touch={action:'east',before,after:await camera()};assert(requestedCameraChange('east',before,row.touch.after));
    row.map=await page.evaluate(()=>({viewport:{width:innerWidth,height:innerHeight},document_width:document.documentElement.scrollWidth,
      controls:[...document.querySelectorAll('#mapControls [data-map-action]')].map(el=>{const r=el.getBoundingClientRect();return {action:el.dataset.mapAction,left:r.left,right:r.right,top:r.top,bottom:r.bottom,width:r.width,height:r.height};})}));
    assert(row.map.document_width<=viewport.width+1);assert(row.map.controls.length>0&&row.map.controls.every(c=>c.left>=0&&c.right<=viewport.width+1&&c.top>=0&&c.bottom<=viewport.height+1&&c.width>0&&c.height>0));
    await activate('#worldFindBtn','keyboard');await page.locator('#worldFindInput').waitFor();assert(await focused('#worldFindInput'));
    await page.keyboard.press('Escape');await page.locator('#decisionDialog').waitFor({state:'hidden'});assert(await focused('#worldFindBtn'));
    row.focus_return.push({route:'keyboard Find → Escape → Find',passed:true});
    await activate('#worldFindBtn','touch');await page.locator('#worldFindInput').fill('Paris');
    const city=page.locator('[data-find-kind="city"]').filter({has:page.getByText('Paris',{exact:true})});await city.tap();
    await page.locator('#mapCityCard').waitFor({state:'visible'});assert.equal(await page.locator('#mapCityTitle').innerText(),'Paris');
    await activate('[data-map-detail-focus="city-province"]','touch');await page.locator('#provinceDossier[aria-hidden="false"]').waitFor();
    await page.waitForFunction(()=>PROVINCE_POPULATION&&!PROVINCE_POPULATION.loading&&!PROVINCE_POPULATION.error&&PROVINCE_POPULATION.id===selectedDistrict);
    assert(await focused('#provinceTitle'));row.province=await scrollPanel('#provinceDossier');
    row.province.reading=await page.evaluate(()=>({district:selectedDistrict,name:PROVINCE_POPULATION.name,date:PROVINCE_POPULATION.read_date,owner:PROVINCE_POPULATION.owner_name}));
    assert.equal(await page.locator('#provinceTitle').innerText(),row.province.reading.name);
    assert(row.province.after.text.includes(row.province.reading.owner)&&row.province.after.text.includes(row.province.reading.date));
    await shot('s22-layout-province-'+viewport.width);row.screenshots.push('s22-layout-province-'+viewport.width+'.png');
    await page.keyboard.press('Escape');await page.locator('#provinceDossier').waitFor({state:'hidden'});assert(await focused('#worldFindBtn'));
    row.focus_return.push({route:'Paris province → Escape → Find',passed:true});
    if(await page.locator('#mapCityCard').isVisible())await tap('[data-map-detail-focus="city-close"]');
    if(await page.locator('#intelDrawer').getAttribute('aria-hidden')!=='false')await activate('[data-drawer="intelDrawer"]','touch');
    await activate('#warsCard [data-ground-equipment-tab="service"]','touch');await page.waitForFunction(()=>equipmentCurrent()&&!equipmentPending());
    assert(await page.evaluate(()=>document.getElementById('equipmentRoom').contains(document.activeElement)));
    row.equipment=await scrollPanel('#equipmentRoom');row.equipment.tab=await page.evaluate(()=>EQUIP.tab);
    await shot('s22-layout-equipment-'+viewport.width);row.screenshots.push('s22-layout-equipment-'+viewport.width+'.png');
    await page.keyboard.press('Escape');await page.locator('#equipmentRoom').waitFor({state:'hidden'});
    const returned=await page.evaluate(()=>({id:document.activeElement?.id,tag:document.activeElement?.tagName,visible:!!document.activeElement?.getClientRects().length,inert:!!document.activeElement?.closest('[inert]')}));
    assert(returned.visible&&returned.tag!=='BODY'&&!returned.inert);row.focus_return.push({route:'Equipment → Escape → visible launcher',...returned,passed:true});
    row.checks.push('Keyboard camera activation','Native touch camera activation','Finder focus and return','Current Paris province content and scroll','Equipment content, scroll and focus return','Viewport/map/room framing');
    row.passed=true;
  }
  await page.setViewportSize({width:1920,height:1080});await settle();proof.passed=true;return proof;
};
