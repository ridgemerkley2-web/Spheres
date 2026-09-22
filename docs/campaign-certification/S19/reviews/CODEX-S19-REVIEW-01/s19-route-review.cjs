// Integration review: ordinary controls and real native outcomes, no seeded results.
const fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto'),assert=require('node:assert/strict');
const {createRequire}=require('node:module'),{chromium}=require('playwright');
const base=__dirname,repo=path.join(base,'s19-review-01'),binary=path.join(base,'s19-review-target/release/spheres-web.exe');
const label=process.argv[2],expected=process.argv[3]||'achieved',out=path.join(base,'evidence/S19-review-01',label);assert(label&&!fs.existsSync(out));fs.mkdirSync(out);
const pin=cp.execFileSync('git',['rev-parse','HEAD'],{cwd:repo,encoding:'utf8'}).trim();assert.equal(cp.execFileSync('git',['status','--porcelain'],{cwd:repo,encoding:'utf8'}).trim(),'');
// Reuse the already qualified S07 visible-control helpers, not the main S07 run.
const helperPath=path.join(repo,'tools/ui/ci-construction.cjs'),src=fs.readFileSync(helperPath,'utf8');
const h=new Function('require','__dirname',src.slice(0,src.indexOf('async function journey('))+'\nreturn {idle,construction,closePanels,budget,add,day,saveLoad};')(createRequire(helperPath),path.dirname(helperPath));
const proof={passed:false,revision:pin,binary_sha256:crypto.createHash('sha256').update(fs.readFileSync(binary)).digest('hex'),helper_sha256:crypto.createHash('sha256').update(fs.readFileSync(helperPath)).digest('hex'),
 scope:'Fresh France, native funding/construction/pause, save/resume, reading separation and advisory navigation review. This is not the full S19 first-hour qualification.',commands:[],advances:[],errors:[],readings:[]};
const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(proof,null,2));
(async()=>{
 const sock=net.createServer();await new Promise(r=>sock.listen(0,'127.0.0.1',r));const port=sock.address().port;await new Promise(r=>sock.close(r));const url='http://127.0.0.1:'+port;
 const run=path.join(out,'server');fs.mkdirSync(run);const log=fs.createWriteStream(path.join(out,'server.log'));
 const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});server.stdout.pipe(log);server.stderr.pipe(log);let browser,page;
 try{
  const until=Date.now()+30000;for(;;){try{const r=await fetch(url+'/api/build');if(r.ok){proof.build=await r.json();break;}await r.arrayBuffer();}catch{}assert(Date.now()<until);await new Promise(r=>setTimeout(r,100));}
  assert.equal(proof.build.revision,pin.slice(0,12));browser=await chromium.launch({channel:'chrome',headless:true});page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
  page.on('pageerror',e=>proof.errors.push(e.message));page.on('request',r=>{if(r.method()==='POST'){const p=new URL(r.url()).pathname;if(p==='/api/command')proof.commands.push(r.postDataJSON());if(p==='/api/advance')proof.advances.push(r.postDataJSON());}});
  const read=async(route)=>{const r=await page.request.get(url+route);assert(r.ok(),route);const d=await r.json();await r.dispose();return d;};
  async function guidance(name){
   await h.closePanels(page);await page.keyboard.press('F1');await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready');
   const model=await page.evaluate(()=>GUIDANCE.session.state()),state=await read('/api/state'),native=await read('/api/guidance?session_id='+encodeURIComponent(state.session_id));
   const reading={name,date:state.date,session_id:state.session_id,outcomes:native.outcomes,route:model.route};proof.readings.push(reading);
   assert.equal(model.route.status,'ready');assert.equal(model.route.as_of.player,'France');assert.equal(model.route.as_of.date,state.date);
   await page.locator('#guidanceRouteTitle').scrollIntoViewIfNeeded();await page.screenshot({path:path.join(out,name+'.png')});
   assert(await page.locator('#guidanceDialog').evaluate(e=>e.scrollWidth<=e.clientWidth+1));await page.keyboard.press('Escape');return reading;
  }
  const status=(r,id)=>r.route.steps.find(s=>s.id===id).status;
  await page.goto(url,{waitUntil:'domcontentloaded'});await page.waitForFunction(()=>!!SESSION.live?.session_id);
  await page.locator('#newCampaignBtn').click();await page.locator('#nationPick [aria-label^="France;"]').click();await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});await h.idle(page);
  const fresh=await guidance('fresh');assert.equal(status(fresh,'construction'),'not_yet');
  await h.construction(page);const initial=await read('/api/production'),district=initial.provinces.find(p=>/le-de-france/i.test(p.id)).id;
  await h.budget(page,initial.construction_budget.daily_budget_bn*4000);await h.add(page,url,'starter_industry',district,true);await h.day(page,url);
  const paid=await guidance('paid');assert.equal(status(paid,'finances'),'achieved');assert.equal(status(paid,'construction'),'achieved');assert(paid.outcomes.construction.projects[0].spent_bn>0);
  await h.budget(page,0);await h.day(page,url);await page.setViewportSize({width:390,height:844});const paused=await guidance('paused-390');
  assert.equal(paused.outcomes.construction.projects[0].spent_bn,paid.outcomes.construction.projects[0].spent_bn);
  assert.equal(paused.outcomes.construction.projects[0].last_spent_bn,paid.outcomes.construction.projects[0].last_spent_bn);
  assert.equal(paused.outcomes.construction.projects[0].last_day,paid.outcomes.construction.projects[0].last_day);assert.equal(status(paused,'construction'),expected);
  proof.pause={paid_status:status(paid,'construction'),paused_status:status(paused,'construction'),retained_spent_bn:paused.outcomes.construction.projects[0].spent_bn};
  proof.save_resume=await h.saveLoad(page,url,'s19-route-review');const resumed=await guidance('resumed-390');assert.equal(status(resumed,'save_resume'),'achieved');assert.equal(status(resumed,'construction'),expected);
  await page.reload();await page.locator('#continueBtn').click();await h.idle(page);const continued=await guidance('continued-390');assert.equal(status(continued,'save_resume'),'achieved');
  const commandsBefore=proof.commands.length,advancesBefore=proof.advances.length;
  await page.keyboard.press('F1');await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready');
  const beforeReading=await page.evaluate(()=>GUIDANCE.session.state().route);
  await page.locator('[data-guidance-complete]').click();await page.locator('[data-guidance-skip]').click();
  assert.deepEqual(await page.evaluate(()=>GUIDANCE.session.state().route),beforeReading);
  proof.reading_separation={complete_and_skip_keep_route:true};await page.keyboard.press('Escape');
  proof.navigation=[];
  for(const [id,tab,subpage] of [['research_design','designer',null],['procurement','companies',null],['air_force','flight','bases']]){
   await page.keyboard.press('F1');await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready');
   await page.locator(`[data-guidance-route-step="${id}"]`).click();await page.locator('#guidanceDialog').waitFor({state:'hidden'});
   await page.locator(`#equipmentRoot[data-equipment-tab-view="${tab}"]`).waitFor({state:'visible'});
   await page.waitForFunction(()=>!EQUIP.loading&&!EQUIP.busy);if(subpage)await page.locator(`[data-flight-page="${subpage}"]`).waitFor({state:'visible'});
   await page.screenshot({path:path.join(out,'navigation-'+id+'-390.png')});
   proof.navigation.push({step:id,tab,page:subpage,visible:true});await page.keyboard.press('Escape');await page.locator('#equipmentRoom').waitFor({state:'hidden'});
  }
  assert.equal(proof.commands.length,commandsBefore);assert.equal(proof.advances.length,advancesBefore);proof.navigation_read_only=true;
  await page.keyboard.press('F1');await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready');
  await page.locator('[data-guidance-route-card="construction"]').scrollIntoViewIfNeeded();await page.screenshot({path:path.join(out,'construction-result-390.png')});
  await page.setViewportSize({width:1440,height:1000});await page.locator('#guidanceRouteTitle').scrollIntoViewIfNeeded();await page.screenshot({path:path.join(out,'route-desktop.png')});await page.keyboard.press('Escape');
  assert.deepEqual(proof.errors,[]);proof.passed=true;
 }catch(error){proof.failure=String(error.stack||error);if(page)await page.screenshot({path:path.join(out,'failure.png')}).catch(()=>{});throw error;}
 finally{write();if(browser)await browser.close();server.kill();await new Promise(r=>server.once('exit',r));log.end();console.log(path.join(out,'result.json'));}
})().catch(e=>{console.error(e);process.exitCode=1;});
