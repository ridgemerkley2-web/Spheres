// Disposable real-browser CI smoke, including a committed response lost in
// transit. Local agent UI verification uses the computer-use browser instead.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),net=require('node:net'),cp=require('node:child_process');
const {chromium}=require('playwright');
const root=path.resolve(__dirname,'../..'),out=path.join(root,'artifacts/browser-ci');
async function port(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const p=s.address().port;await new Promise(r=>s.close(r));return p;}
(async()=>{
  fs.mkdirSync(out,{recursive:true});const run=fs.mkdtempSync(path.join(out,'campaign-')),p=await port(),url=`http://127.0.0.1:${p}`;
  const binary=process.env.SPHERES_BINARY||path.join(root,'target/release/spheres-web'+(process.platform==='win32'?'.exe':''));
  const server=cp.spawn(binary,['--port',String(p),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(run,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser;
  try{
    for(let n=0;;n++){try{if((await fetch(url+'/api/state')).ok)break;}catch(_){}if(n>=200)throw Error('Disposable server failed to start');await new Promise(r=>setTimeout(r,100));}
    browser=await chromium.launch({headless:true});const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);const errors=[];page.on('pageerror',e=>errors.push(e.message));
    await page.goto(url);await page.locator('#nationPick [aria-label^="United States;"]').click();await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});
    const state=async()=> (await page.request.get(url+'/api/state')).json();
    const initial=await state();assert.equal(initial.player,'USA');assert.equal(initial.simulation_cadence,'daily');
    await page.getByRole('button',{name:'Advisor',exact:true}).click();await page.getByRole('heading',{name:'Fund your plan',exact:true}).waitFor();await page.getByRole('button',{name:'Close Your development advisor',exact:true}).click();
    await page.getByRole('button',{name:'Find',exact:true}).click();await page.locator('#worldFindInput').fill('California');await page.locator('[data-find-id="US-CA"]').click();await page.locator('#provinceDossier').waitFor({state:'visible'});
    await page.evaluate(()=>closeProvince());
    // Let the normal command channel create the receipt. Lose only the first
    // already-committed response, then recover via the visible receipt button.
    let lost=false;await page.route('**/api/command',async route=>{if(lost)return route.continue();lost=true;await route.fetch();await route.abort('failed');});
    await page.evaluate(()=>api('/api/command',{commands:[{kind:'tax',value:0.29}]}));
    await page.locator('#retryCommandBtn').waitFor({state:'visible'});const committed=await state();
    await page.locator('#retryCommandBtn').click();await page.locator('#pendingCommand').waitFor({state:'hidden'});
    const recovered=await state();assert.equal(recovered.nations.find(n=>n.id==='USA').political_capital,committed.nations.find(n=>n.id==='USA').political_capital);
    assert.equal(recovered.nations.find(n=>n.id==='USA').tax,0.29);
    await page.unroute('**/api/command');
    const saved=await page.request.post(url+'/api/save',{data:{slot:'ci-smoke'}});assert(saved.ok());
    const history=await (await page.request.get(url+'/api/history?nation=USA')).json();
    const loaded=await page.request.post(url+'/api/load',{data:{slot:'ci-smoke'}});assert(loaded.ok());
    assert.deepEqual(await(await page.request.get(url+'/api/history?nation=USA')).json(),history);
    await page.reload();await page.locator('#continueBtn').click();
    await page.locator('#techBtn').click();await page.locator('#techMenu .dfoot').click();await page.getByRole('button',{name:'Research list',exact:true}).click();await page.locator('#researchListQuery').fill('');await page.locator('[data-research-id]').first().waitFor();
    await page.screenshot({path:path.join(out,'research-desktop.png')});await page.setViewportSize({width:414,height:896});
    assert(await page.evaluate(()=>document.querySelector('#decisionDialog').scrollWidth<=document.querySelector('#decisionDialog').clientWidth+1));
    await page.screenshot({path:path.join(out,'research-mobile.png')});assert.deepEqual(errors,[]);
    fs.writeFileSync(path.join(out,'result.json'),JSON.stringify({passed:true,build:initial.build,lost_committed_response_recovered:lost,save_history_roundtrip:true},null,2));
  }finally{if(browser)await browser.close();server.kill();log.end();}
})().catch(e=>{console.error(e);process.exitCode=1;});
