// Ordinary browser controls over a native campaign. No state or command injection.
const assert=require('node:assert/strict'),path=require('node:path'),crypto=require('node:crypto');
exports.review=async({page,data,out})=>{
  const q=JSON.stringify,copy=x=>JSON.parse(JSON.stringify(x)),screenshots=[],inspections=[];
  const draft=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.draft)));
  const sections={command:['Your squadrons','Mission orders'],aircraft:['Owned aircraft revisions'],bases:['Airbases'],reports:['Dated mission results']};
  for(const key of Object.keys(sections)){
    await page.locator('[data-equipment-flight-page='+q(key)+']').click();
    assert.equal(await page.locator('[data-flight-page]').getAttribute('data-flight-page'),key);
    const panel=page.locator('[data-flight-page]'),text=await panel.innerText();assert(!/Falcon Squadron|Northfield|\bNaN\b|\bundefined\b/.test(text));
    for(const title of sections[key])assert(await panel.locator('section[aria-label='+q(title)+']').count());
    const rows=key==='command'?data.flight.squadrons:key==='aircraft'?data.flight.aircraft:key==='bases'?data.flight.bases:data.flight.missions.results;
    for(const row of rows)assert(text.includes(row.name),'Native record missing: '+row.name);
    assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data.flight))),copy(data.flight));
    for(const width of [1440,390]){
      await page.setViewportSize({width,height:width===390?844:1000});await panel.evaluate(el=>el.scrollIntoView({block:'start'}));
      const size=await page.locator('#equipmentRoot').evaluate(el=>({width:el.clientWidth,scroll:el.scrollWidth}));assert(size.width>0&&size.scroll<=size.width+1,key+' overflow');
      const file=`flight-${key}-${width}.png`;await page.screenshot({path:path.join(out,file)});screenshots.push(file);
    }
  }
  await page.setViewportSize({width:1440,height:1000});await page.locator('[data-equipment-flight-page=aircraft]').click();
  for(const row of data.flight.aircraft.filter(x=>x.spec)){
    await page.locator('[data-equipment-flight-aircraft='+q(row.id)+']').click();
    const host=page.locator('[data-equipment-model]');await host.locator('canvas').waitFor();
    assert.equal(await page.locator('#flightInspectionTitle').innerText(),row.name);
    for(const slot of ['air_avionics','air_engine']){await host.locator('[data-model-focus='+q(slot)+']:not([data-model-focus-region])').click();assert.equal(await host.locator('[data-model-focus='+q(slot)+']:not([data-model-focus-region])').getAttribute('aria-pressed'),'true');}
    await host.locator('[data-model-focus-region=front]').click();await host.locator('[data-model-reset]').click();
    const downloadPromise=page.waitForEvent('download');await host.locator('[data-model-export]').click();const download=await downloadPromise;assert.equal(await download.failure(),null);
    const chunks=[];for await(const chunk of await download.createReadStream())chunks.push(chunk);const bytes=Buffer.concat(chunks),json=JSON.parse(bytes.subarray(20,20+bytes.readUInt32LE(12)).toString()),extras=json.meshes[0].extras;
    assert.deepEqual(extras.specification,copy(row.spec),'Downloaded frozen revision must match the native specification');assert(extras.triangleCount>=100000);
    const file=`owned-${inspections.length}.png`;await host.screenshot({path:path.join(out,file)});screenshots.push(file);inspections.push({id:row.id,name:row.name,spec:row.spec,triangles:extras.triangleCount,glb_sha256:crypto.createHash('sha256').update(bytes).digest('hex')});
  }
  assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.draft))),draft,'Inspection preserves the local designer draft');
  await page.locator('[data-equipment-flight-page=command]').click();
  return {pages:Object.keys(sections),screenshots,inspections,draft_preserved:true};
};
