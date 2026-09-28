// Resume the unedited S19 France checkpoint in a disposable server. All mutations
// use visible controls; GETs observe native state. This qualifies recovery UI,
// not produced stock, equipment delivery, or the complete S19 route.
'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),crypto=require('node:crypto'),zlib=require('node:zlib'),net=require('node:net');
const {chromium}=require('playwright');
const root=path.resolve(__dirname,'../..'),sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const git=args=>cp.execFileSync('git',args,{cwd:root,encoding:'utf8',windowsHide:true});
(async()=>{
  const expected=process.env.SPHERES_EXPECTED_REVISION;
  assert.match(expected||'',/^[a-f0-9]{40}$/);
  assert.equal(git(['status','--porcelain','--','spheres-web','spheres-sim']).trim(),'');
  assert.equal(git(['diff','--name-only',expected,'HEAD','--','spheres-web','spheres-sim','Cargo.toml','Cargo.lock']).trim(),'');
  const binary=path.resolve(process.env.SPHERES_BINARY||path.join(root,'target/release/spheres-web.exe'));
  const checkpoint=process.env.SPHERES_SUPPLIER_CHECKPOINT||path.join(root,'docs/campaign-certification/S19/integration/procurement-supply-evidence/recorded-france-supplier-checkpoint.json.gz');
  const bytes=fs.readFileSync(checkpoint),raw=bytes[0]===0x1f&&bytes[1]===0x8b?zlib.gunzipSync(bytes):bytes;
  const expectedSave=process.env.SPHERES_SUPPLIER_CHECKPOINT?process.env.SPHERES_SUPPLIER_CHECKPOINT_SHA256:'fbe56cdfec5d83b5d9c259f80acb7bb0888dab37d4f35c70beb8686b5eeaf91e';
  assert.match(expectedSave||'',/^[a-f0-9]{64}$/,'Custom checkpoints require a recorded hash');assert.equal(sha(raw),expectedSave);
  const output=path.resolve(process.env.SPHERES_SUPPLY_OUTPUT||path.join(root,'artifacts/supplier-input-recovery'));
  fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'france-')),run=path.join(out,'server');
  fs.mkdirSync(path.join(run,'saves'),{recursive:true});fs.writeFileSync(path.join(run,'saves/s19-input.json'),raw);
  const socket=net.createServer();await new Promise(resolve=>socket.listen(0,'127.0.0.1',resolve));
  const port=socket.address().port;await new Promise(resolve=>socket.close(resolve));const url=`http://127.0.0.1:${port}`;
  const evidence={passed:false,scope:'Recorded-campaign supplier input recovery UI; not stock/delivery qualification',out,run,url,
    started_utc:new Date().toISOString(),runtime_revision:expected,head:git(['rev-parse','HEAD']).trim(),
    driver_sha256:sha(fs.readFileSync(__filename)),binary_sha256:sha(fs.readFileSync(binary)),checkpoint_sha256:sha(raw),
    actions:[],requests:[],errors:[],screenshots:[],assets:[]};
  fs.copyFileSync(__filename,path.join(out,'driver.cjs'));
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,launchError;server.on('error',error=>{launchError=error;});console.log(out);
  try{
    let ready=false;
    for(let i=0;i<300;i++){
      if(launchError)throw launchError;if(server.exitCode!==null)throw Error('Server exited');
      try{if((await fetch(url+'/api/state')).ok){ready=true;break;}}catch{}
      await new Promise(resolve=>setTimeout(resolve,100));
    }
    assert(ready,'Server startup');
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});
    page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.on('pageerror',e=>evidence.errors.push(e.message));
    page.on('request',r=>{if(r.method()==='POST')evidence.requests.push({route:new URL(r.url()).pathname,payload:r.postDataJSON()});});
    const tap=async selector=>{evidence.actions.push({click:selector});await page.locator(selector).click();};
    const get=async route=>{const r=await page.request.get(url+route);assert(r.ok(),route);return r.json();};
    const state=()=>get('/api/state');
    const read=async route=>get('/api/'+route+'?session_id='+encodeURIComponent((await state()).session_id));
    const shot=async name=>{const file=name+'.png';await page.screenshot({path:path.join(out,file)});evidence.screenshots.push(file);};
    async function companies(){
      for(const [panel,close] of [['#guidanceDialog','#guidanceDialog [data-guidance-close]'],['#equipmentRoom','[data-equipment-close]'],['#competitionRoom','#competitionClose'],['#productionPanel','#productionClose'],['#cabinetDrawer','#cabinetDrawer [data-close-drawers]']])
        if(await page.locator(panel).isVisible())await tap(close);
      evidence.actions.push({key:'F1'});await page.keyboard.press('F1');
      await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready'&&!GUIDANCE.session.state().routeStale);
      await tap('[data-guidance-route-step="procurement"]');
      await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='companies');
      const administration=page.locator('[data-equipment-detail="company-administration"]');
      if(await administration.count() && await administration.getAttribute('open')===null)
        await tap('[data-equipment-detail="company-administration"] > summary');
      await page.locator('[data-equipment-record="1:3"]').waitFor({state:'visible'});
      await tap('[data-equipment-record="1:3"] h3');
    }
    await page.goto(url);
    for(const name of ['index.html','competition-ui.js','equipment-ui.js','equipment-ui.css']){
      const response=await page.request.get(url+(name==='index.html'?'/':'/'+name)),served=await response.body();
      assert(served.equals(fs.readFileSync(path.join(root,'spheres-web/ui',name))),'Served asset '+name);
      evidence.assets.push({name,sha256:sha(served)});
    }
    await tap('#openSavesBtn');await page.locator('#saveSlots option[value="s19-input"]').waitFor({state:'attached'});
    evidence.actions.push({select:'#saveSlots',value:'s19-input'});await page.locator('#saveSlots').selectOption('s19-input');
    await tap('#loadBtn');await page.waitForFunction(()=>S?.player&&!SESSION.busy);
    evidence.before={state:await state(),industry:await read('industry'),companies:await read('companies')};
    if(process.env.SPHERES_SUPPLIER_GRID_RECOVERY==='1'){
      evidence.scope='Ordinary grid recovery and equipment purchase/delivery from a recorded component-producing campaign';
      const helper=path.join(__dirname,'supplier-production-route.cjs');
      evidence.production_driver_sha256=sha(fs.readFileSync(helper));fs.copyFileSync(helper,path.join(out,'production-driver.cjs'));
      await require(helper)({page,tap,state,read,shot,companies,evidence});
      assert.deepEqual(evidence.errors,[]);assert.equal(sha(fs.readFileSync(binary)),evidence.binary_sha256);
      evidence.passed=true;return;
    }
    if(process.env.SPHERES_SUPPLIER_VERIFY_DELIVERY==='1'){
      evidence.scope='Native procurement milestones, save/load and Continue from a recorded delivered campaign';
      const helper=path.join(__dirname,'supplier-delivery-proof.cjs');
      evidence.proof_driver_sha256=sha(fs.readFileSync(helper));fs.copyFileSync(helper,path.join(out,'proof-driver.cjs'));
      await require(helper)({page,tap,read,state,shot,evidence});
      assert.deepEqual(evidence.errors,[]);assert.equal(sha(fs.readFileSync(binary)),evidence.binary_sha256);
      evidence.passed=true;return;
    }
    await companies();
    const card=page.locator('[data-equipment-record="1:3"]');evidence.card_text=await card.innerText();
    const shortage=card.locator('.eq-supplier-input').filter({has:page.getByRole('heading',{name:'Advanced components components',exact:true})});
    assert.deepEqual(await shortage.locator('dd').allTextContents(),['0.4441','0','0.4441']);
    await card.locator('.eq-supplier-input--short h5').click();await shot('supplier-inputs-desktop');
    await page.setViewportSize({width:390,height:844});await card.locator('.eq-supplier-input--short h5').click();await shot('supplier-inputs-narrow');
    assert(await page.locator('#equipmentRoom').evaluate(e=>e.scrollWidth<=e.clientWidth+1),'Narrow equipment overflow');
    await page.setViewportSize({width:1440,height:1000});
    await card.getByRole('button',{name:'Inspect operating industry',exact:true}).click();evidence.actions.push({click:'Inspect operating industry'});
    await page.waitForFunction(()=>industryCurrent());evidence.industry_text=await page.locator('#cabinet-industry').innerText();await shot('operating-industry');
    await companies();await card.getByRole('button',{name:'Review advanced-components plant',exact:true}).click();evidence.actions.push({click:'Review advanced-components plant'});
    await page.locator('[data-prod-province]').first().waitFor();
    const district=await page.locator('[data-prod-province]').first().getAttribute('data-prod-province');
    await tap(`[data-prod-province="${district}"]`);await page.waitForFunction(()=>constructionPreviewCurrent());
    evidence.construction_preview=await page.evaluate(()=>JSON.parse(JSON.stringify(PROD.preview)));
    assert.equal(await page.evaluate(()=>PROD.pickKind),'advanced_industry');await shot('advanced-industry-review');
    if(process.env.SPHERES_SUPPLIER_PRODUCTION==='1'){
      evidence.scope='Ordinary component production and equipment purchase/delivery from the recorded France campaign';
      const helper=path.join(__dirname,'supplier-production-route.cjs');
      evidence.production_driver_sha256=sha(fs.readFileSync(helper));fs.copyFileSync(helper,path.join(out,'production-driver.cjs'));
      await require(helper)({page,tap,state,read,shot,companies,evidence});
    }else{
    await companies();await card.getByRole('button',{name:'Review component imports',exact:true}).click();evidence.actions.push({click:'Review component imports'});
    await page.waitForFunction(()=>COMP.open&&!COMP.loading&&!COMP.stale);
    if(await page.locator('[data-comp-action="enable"]').isVisible()){
      await tap('[data-comp-action="enable"]');
      await page.waitForFunction(()=>COMP.data?.enabled&&!COMP.busy&&!COMP.pending&&!COMP.loading);
    }
    const form=page.locator('#competitionQuoteForm');await form.waitFor();
    assert.equal(await form.locator('[name="good"]').inputValue(),'advanced_components');
    assert(Math.abs(Number(await form.locator('[name="quantity"]').inputValue())-0.4441)<1e-12);
    await tap('#competitionQuoteForm button[type="submit"]');await page.waitForFunction(()=>!COMP.searching&&COMP.quotes!==null);
    evidence.goods_quotes=await page.evaluate(()=>JSON.parse(JSON.stringify(COMP.quotes)));await shot('component-quotes');
    evidence.after={state:await state(),industry:await read('industry'),companies:await read('companies')};
    assert.equal(evidence.before.state.date,evidence.after.state.date);
    assert.equal(evidence.after.industry.goods.find(g=>g.good==='advanced_components').stock,0);
    assert(!evidence.requests.some(r=>r.route==='/api/advance'));
    assert(evidence.requests.filter(r=>r.route==='/api/command').every(r=>(r.payload.commands||[]).every(c=>c.kind==='enable_economic_competition')));
    }
    assert.deepEqual(evidence.errors,[]);assert.equal(sha(fs.readFileSync(binary)),evidence.binary_sha256);
    evidence.passed=true;console.log(evidence.production?'PASS: component production and equipment delivery route':'PASS: exact shortage, industry/build navigation and real component quotes; no stock grant or purchase');
  }catch(error){evidence.error=error.stack;if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.txt'),await page.locator('body').innerText());}catch{}throw error;}
  finally{evidence.finished_utc=new Date().toISOString();fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(evidence,null,2));if(browser)await browser.close();if(server.exitCode===null)server.kill();log.end();}
})().catch(error=>{console.error(error);process.exitCode=1;});

