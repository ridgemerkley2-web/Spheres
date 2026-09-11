// Run against the existing static workshop: node tools/ui/check_flight_command_browser.cjs
const {chromium}=require('playwright');
const assert=require('node:assert/strict');
const path=require('node:path');
(async()=>{
  const browser=await chromium.launch({headless:true,...(process.env.PLAYWRIGHT_EXECUTABLE_PATH?{executablePath:process.env.PLAYWRIGHT_EXECUTABLE_PATH}:{})});
  try{
    const page=await browser.newPage({viewport:{width:1440,height:1000}}),errors=[];
    page.on('pageerror',e=>errors.push(e.message));
    await page.goto((process.env.SPHERES_WORKSHOP_URL||'http://127.0.0.1:7841')+'/tools/arsenal/flight-command.html');
    assert.equal(await page.locator('[data-mission]').count(),6);
    await page.locator('[data-mission="defend"]').click();
    assert(await page.locator('#review-mission').isDisabled(),'tactical aircraft cannot defend skies');
    await page.locator('[data-mission="army"]').click();
    await page.selectOption('#area','distant');await page.locator('#review-mission').click();
    assert(await page.locator('#confirm-mission').isDisabled(),'out-of-range assignment rejected');
    await page.keyboard.press('Escape');
    assert.equal(await page.locator('#assignment').textContent(),'Standby','cancel does not assign');
    await page.selectOption('#area','front');await page.locator('#review-mission').click();await page.locator('#confirm-mission').click();
    assert.equal(await page.locator('#assignment').textContent(),'Support army');
    await page.locator('[data-page="reports"]').click();assert.equal(await page.locator('.report').count(),1);
    await page.locator('[data-page="command"]').click();
    await page.screenshot({path:path.resolve(__dirname,'../../../flight-command-desktop.png'),fullPage:true});
    await page.locator('[data-page="aircraft"]').click();
    await page.waitForFunction(()=>document.querySelector('#mesh-count').textContent.includes('triangles'));
    assert(await page.locator('#model-host canvas').count(),'viewer mounts');
    for(const platform of ['air_tactical_strike','air_light_attack']){
      await page.selectOption('#platform',platform);
      const stats=await page.locator('#mesh-count').textContent();assert(Number(stats.split(' triangles')[0].replaceAll(',',''))>=100000,stats);
      await page.selectOption('[data-slot="air_fuel"]','air_fuel_extended');
      await page.selectOption('#detail','2');
      const count=Number((await page.locator('#mesh-count').textContent()).split(' triangles')[0].replaceAll(',',''));assert(count<2500);
      await page.selectOption('#detail','0');
    }
    await page.selectOption('#platform','air_tactical_strike');
    await page.screenshot({path:path.resolve(__dirname,'../../../flight-hangar-desktop.png'),fullPage:true});
    const downloadPromise=page.waitForEvent('download');await page.locator('#download').click();
    const download=await downloadPromise;assert.equal(await download.failure(),null);
    await page.setViewportSize({width:390,height:844});
    assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),'mobile hangar does not overflow');
    await page.locator('[data-page="command"]').click();
    assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),'mobile command does not overflow');
    await page.screenshot({path:path.resolve(__dirname,'../../../flight-command-mobile.png'),fullPage:true});
    assert.deepEqual(errors,[]);console.log('PASS flight command: mission compatibility, range/cancel/assign, reports, both 100k+ meshes, LOD, GLB download, mobile layout, no browser errors');
  }finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
