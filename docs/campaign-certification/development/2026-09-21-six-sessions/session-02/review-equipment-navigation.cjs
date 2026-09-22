const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict'),crypto=require('node:crypto');
const {chromium}=require('playwright');
const repo=path.join(__dirname,'integration'),out=path.join(__dirname,'evidence',process.argv[2]||'six-session-02-browser-final');fs.mkdirSync(out,{recursive:true});
const proof={scope:'UI development review against native S19 preview, with current equipment JS/CSS responses substituted. Not an exact packaged build qualification.',passed:false,assets:{},commands:[],pageErrors:[],views:[]};
(async()=>{const browser=await chromium.launch({channel:'chrome',headless:true});const page=await browser.newPage({viewport:{width:390,height:844},reducedMotion:'reduce'});page.on('pageerror',e=>proof.pageErrors.push(e.message));
page.on('request',r=>{if(r.method()==='POST'&&['/api/command','/api/advance'].includes(new URL(r.url()).pathname))proof.commands.push(r.postDataJSON());});
try{
 for(const file of ['equipment-ui.js','equipment-ui.css']){const body=fs.readFileSync(path.join(repo,'spheres-web/ui',file));proof.assets[file]=crypto.createHash('sha256').update(body).digest('hex');await page.route('**/'+file,r=>r.fulfill({body,contentType:file.endsWith('.js')?'text/javascript':'text/css'}));}
 await page.goto('http://127.0.0.1:7863/');await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await page.waitForFunction(()=>!SESSION.busy);await page.keyboard.press('F1');await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready');await page.locator('[data-guidance-route-step="air_force"]').click();await page.locator('#equipmentRoom').waitFor({state:'visible'});await page.waitForFunction(()=>!EQUIP.loading);
 const draft=await page.evaluate(()=>JSON.stringify(EQUIP.draft));
 for(const section of ['flight','companies','designer','research','library','development','production','ammunition','service']){
  const select=page.locator('[data-equipment-section]');await select.selectOption(section);await page.waitForFunction(()=>!EQUIP.loading);
  assert.equal(await page.locator('[data-equipment-section]').inputValue(),section);assert.equal(await page.locator('[data-equipment-section]').evaluate(e=>e===document.activeElement),true);
  await page.locator('[data-equipment-jump]').click();const target=page.locator(section==='flight'?'[data-flight-page]':'[data-equipment-content]');assert.equal(await target.evaluate(e=>e===document.activeElement),true);
  const rect=await target.boundingBox();assert(rect.y>=0&&rect.y<422,'The section heading must be visible in the upper half; short pages can reach maximum scroll before aligning at the top.');
  assert(await page.locator('#equipmentRoom').evaluate(e=>e.scrollWidth<=e.clientWidth+1));
  assert.equal(await page.evaluate(()=>JSON.stringify(EQUIP.draft)),draft);proof.views.push({section,focused:true,content_y:rect.y});
  if(['flight','companies'].includes(section))await page.screenshot({path:path.join(out,section+'-390.png')});
 }
 await page.setViewportSize({width:1440,height:1000});await page.locator('[data-equipment-tab="flight"]').click();await page.locator('[data-equipment-flight-page="bases"]').click();await page.locator('[data-equipment-jump]').click();await page.screenshot({path:path.join(out,'bases-desktop.png')});
 assert.deepEqual(proof.commands,[]);assert.deepEqual(proof.pageErrors,[]);proof.passed=true;
}catch(e){proof.failure=String(e.stack);await page.screenshot({path:path.join(out,'failure.png')});throw e;}finally{fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(proof,null,2)+'\n');await browser.close();}})().catch(e=>{console.error(e);process.exitCode=1;});
