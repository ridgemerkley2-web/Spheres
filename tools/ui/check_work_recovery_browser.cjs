// NODE_PATH=<Playwright modules> SPHERES_BROWSER_CHANNEL=msedge node this-file BINARY FULL_REVISION NEW_OUTPUT
// Loads five pinned authored fixtures; only visible Load/advance/retry controls
// change gameplay. Direct POSTs save diagnostic archives or test stale refusal.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),zlib=require('node:zlib');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),{verifyBuild}=require('./ci-integrated.cjs');
const root=path.resolve(__dirname,'../..'),configPath=path.join(__dirname,'recovery-boundaries.json');
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex'),fileHash=file=>sha(fs.readFileSync(file));
const isPost=(response,route)=>response.request().method()==='POST'&&new URL(response.url()).pathname===route;
const INPUT='recovery-input',OBSERVE='recovery-observe';
function inside(parent,file){const relative=path.relative(parent,file);assert(relative&&relative!=='..'&&!relative.startsWith('..'+path.sep)&&!path.isAbsolute(relative),'Path escaped its root');return file;}
function regular(file){assert(fs.lstatSync(file).isFile()&&!fs.lstatSync(file).isSymbolicLink());assert.equal(fs.realpathSync(file),file);return file;}
function world(archive){assert.equal(archive.format,'spheres-campaign');return archive.world.world||archive.world;}
function dayNumber(w){return (Date.UTC(w.year,w.month-1,w.day)-Date.UTC(1990,0,1))/86400000;}
function marker(archive,test){
  const w=world(archive),n=w.nations.find(row=>row.id==='France'),m=test.marker,base={day:dayNumber(w),date:archive.saved_date};assert(n);
  if(test.id==='construction'){const b=w.airbases?.bases.find(row=>row.id===m.base);assert(b);const p=b.project?.id===m.project?b.project:b.history.find(row=>row.id===m.project);assert(p);return {...base,project:p};}
  if(test.id==='delivery'){const d=w.companies.deliveries.find(row=>row.id===m.delivery);assert(d);return {...base,delivery:d,held:(n.arsenal.held||[]).filter(row=>row.design_id===m.revision).reduce((sum,row)=>sum+row.units,0)};}
  if(test.id==='refit'){const f=w.companies.firms.find(row=>row.id===m.company),r=f?.refits.find(row=>row.id===m.refit);assert(r);return {...base,refit:r};}
  if(test.id==='rebasing'){const s=n.aviation?.squadrons.find(row=>row.id===m.squadron);assert(s);return {...base,squadron:s};}
  assert.equal(test.id,'mission');const orders=test.marker.missions.map(id=>w.air_missions?.orders.find(row=>row.id===id));assert(orders.every(Boolean));return {...base,orders};
}
function ready(m,test){
  if(test.id==='construction'){assert.equal(m.project.completed_day,null);assert.equal(m.project.cancelled_day,null);assert.equal(m.project.paused,false);return m.project.paid_bn>0&&m.project.progress_days>0&&m.project.progress_days<m.project.total_days;}
  if(test.id==='delivery'){assert.equal(m.delivery.delivered_day,null);assert.equal(m.delivery.quantity,test.marker.quantity);assert(m.delivery.total_price_bn>0);return m.delivery.settled_day!=null&&m.delivery.due_day!=null&&m.day>=m.delivery.due_day;}
  if(test.id==='refit'){assert.equal(m.refit.status,'refitting');assert(m.refit.settled_day!=null&&m.refit.escrow_bn>0&&m.refit.completed_units<m.refit.quantity);return m.refit.unit_work_days>0&&m.refit.unit_work_days<m.refit.unit_days;}
  if(test.id==='rebasing'){assert(m.squadron.transit);assert.equal(m.squadron.transit.to,test.marker.destination);return m.day>=m.squadron.transit.arrival_day;}
  assert(m.orders.every(row=>row.status==='queued'&&row.report==null));return m.orders.every(row=>row.launch_day<=m.day);
}
function progressed(before,after,test){
  assert.equal(after.day,before.day+1,'The committed request must close exactly one day');
  if(test.id==='construction'){assert(after.project.paid_bn>before.project.paid_bn);assert(after.project.progress_days>before.project.progress_days);}
  else if(test.id==='delivery'){assert(after.delivery.delivered_day!=null);assert.equal(after.delivery.status,'delivered');assert.equal(after.delivery.quantity,before.delivery.quantity);assert.equal(after.held-before.held,test.marker.quantity);}
  else if(test.id==='refit')assert(after.refit.unit_work_days>before.refit.unit_work_days||after.refit.completed_units>before.refit.completed_units,'Paid refit work must actually advance');
  else if(test.id==='rebasing'){assert.equal(after.squadron.transit,null);assert.equal(after.squadron.base,test.marker.destination);assert.equal(after.squadron.assigned,before.squadron.assigned);}
  else assert(after.orders.every(row=>row.status==='flown'&&row.report?.stores_used>0&&row.report.day===before.day),'The tested day must actually fly both queued missions');
}
async function get(page,url,route){const response=await page.request.get(url+route);try{assert(response.ok());return await response.json();}finally{await response.dispose();}}
async function idle(page){await page.waitForFunction(()=>!advancing&&!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending);}
async function menus(page){
  if(await page.locator('#app').isVisible()){if(!await page.locator('#campaignsBtn').isVisible())await page.locator('.arc-time-menu > summary').click();await page.locator('#campaignsBtn').click();}
  if(!await page.locator('#campaignHome').isVisible())await page.locator('#savedCampaigns [data-menu-back]').click();
  await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function load(page,slot){
  await menus(page);await page.locator('#saveSlots option[value="'+slot+'"]').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
  const replacing=await page.evaluate(()=>!!SESSION.live?.player),response=page.waitForResponse(r=>isPost(r,'/api/load'));
  await page.locator('#loadBtn').click();if(replacing)await page.locator('#campaignConfirmAccept').click();
  const r=await response;assert(r.ok());assert.equal(r.request().postDataJSON().slot,slot);assert.equal(r.request().postDataJSON().backup,false);
  await page.locator('#app').waitFor({state:'visible'});await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);
}
async function advance(page){const pending=page.waitForResponse(r=>isPost(r,'/api/advance'));await page.locator('#stepBtn').click();const r=await pending,v=await r.json();assert(r.ok());assert.deepEqual(v.errors||[],[]);assert.equal(r.request().postDataJSON().days,1);await idle(page);return {payload:r.request().postDataJSON(),date:v.date};}
async function port(){const listener=net.createServer();await new Promise(resolve=>listener.listen(0,'127.0.0.1',resolve));const value=listener.address().port;await new Promise(resolve=>listener.close(resolve));return value;}
async function main(){
  const [binaryArg,revision,outArg]=process.argv.slice(2);assert(binaryArg&&outArg&&/^[a-f0-9]{40}$/.test(revision));
  const binary=regular(path.resolve(binaryArg)),out=path.resolve(outArg);assert(!fs.existsSync(out),'Output must be new; preserve prior attempts');
  const config=JSON.parse(fs.readFileSync(configPath,'utf8')),required=['construction','delivery','refit','rebasing','mission'];
  assert.deepEqual(config.cases.map(row=>row.id),required,'All five boundaries are mandatory, with no filtering');
  const run=inside(out,path.join(out,'disposable-campaign'));fs.mkdirSync(path.join(run,'saves'),{recursive:true});assert.equal(fs.realpathSync(out),out);
  const url='http://127.0.0.1:'+await port(),proof={passed:false,scope:config.scope,revision,driver_sha256:fileHash(__filename),config_sha256:fileHash(configPath),
    build_verifier_sha256:fileHash(path.join(__dirname,'ci-integrated.cjs')),binary_sha256:fileHash(binary),started_utc:new Date().toISOString(),run,url,cases:[],errors:[]};
  let browser,page,launchError,stage='launch';
  const server=cp.spawn(binary,['--port',new URL(url).port,'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  server.on('error',error=>{launchError=error;});const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(proof,null,2)+'\n');
  const mark=value=>{stage=value;fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage})+'\n');};
  async function snapshot(label,test,e){
    const current=await get(page,url,'/api/state'),saved=await page.request.post(url+'/api/save',{data:{slot:OBSERVE,session_id:current.session_id}});
    try{assert(saved.ok());assert.equal((await saved.json()).ok,true);}finally{await saved.dispose();}
    const raw=fs.readFileSync(regular(path.join(run,'saves',OBSERVE+'.json'))),text=raw.toString('utf8'),value=JSON.parse(text);
    const expression=/("saved_unix":)\d+(?=\s*}\s*$)/;assert(expression.test(text));
    for(const key of ['world','history','log','journey','history_epoch'])assert(Object.hasOwn(value,key));
    const normalized=text.replace(expression,'$1<TIMESTAMP>'),compressed=zlib.gzipSync(raw),name=test.id+'-'+label+'.json.gz';
    const target=inside(out,path.join(out,name));assert(!fs.existsSync(target));fs.writeFileSync(target,compressed);
    const fact=marker(value,test),record={label,path:name,raw_sha256:sha(raw),gzip_sha256:sha(compressed),raw_bytes:raw.length,gzip_bytes:compressed.length,campaign_bytes_sha256:sha(normalized),marker:fact};
    e.archives.push(record);return {normalized,marker:fact,state:current,record};
  }
  try{
    const deadline=Date.now()+30000;
    for(;;){if(launchError)throw launchError;assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=deadline)throw error;}assert(Date.now()<deadline);await new Promise(resolve=>setTimeout(resolve,100));}
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'msedge'});proof.browser_version=browser.version();
    page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);page.on('pageerror',error=>proof.errors.push({stage,message:error.message}));
    process.env.SPHERES_EXPECTED_REVISION=revision;proof.build=await verifyBuild({page,url,root,run,binary});
    await page.goto(url,{waitUntil:'domcontentloaded'});await page.waitForFunction(()=>!!SESSION.live?.session_id);
    assert.equal((await get(page,url,'/api/state')).player,null,'Disposable startup must have no player campaign');
    for(const test of config.cases){
      const e={id:test.id,passed:false,source:test,archives:[],preparation:[],screenshots:[]};proof.cases.push(e);mark(test.id+': import pinned authored fixture');
      const source=regular(inside(root,path.resolve(root,test.source))),manifestPath=regular(inside(root,path.resolve(root,test.manifest)));
      assert.equal(fileHash(source),test.gzip_sha256);assert.equal(fileHash(manifestPath),test.manifest_sha256);
      const manifest=JSON.parse(fs.readFileSync(manifestPath,'utf8')),raw=zlib.gunzipSync(fs.readFileSync(source));assert.equal(sha(raw),test.raw_sha256);
      assert.equal(manifest.compiled_revision,test.compiled_revision);assert.equal(manifest.expected_stages[test.stage],path.basename(test.source,'.gz'));
      e.provenance={scope:manifest.scope,authored_preconditions:manifest.authored_preconditions,raw_bytes:raw.length};
      const input=inside(run,path.join(run,'saves',INPUT+'.json'));if(fs.existsSync(input))regular(input);fs.writeFileSync(input,raw);
      await load(page,INPUT);let current=await snapshot('loaded',test,e);assert.equal(current.state.player,'France');
      for(let count=0;!ready(current.marker,test);count++){
        assert(count<test.max_preparation_days,'Required real pending-work boundary was not reached within its declared ordinary-day bound');
        mark(test.id+': prepare ordinary day '+(count+1));e.preparation.push(await advance(page));current=await snapshot('preparation-'+(count+1),test,e);
      }
      const before=current;e.pending_marker=before.marker;mark(test.id+': commit day and lose response');
      let lost,interceptError;
      const handler=async route=>{try{const r=await route.fetch();lost={payload:route.request().postDataJSON(),status:r.status(),response:await r.json()};await r.dispose();await route.abort('failed');}catch(error){interceptError=error;await route.abort('failed').catch(()=>{});}};
      await page.route('**/api/advance',handler);await page.locator('#stepBtn').click();await page.locator('#pendingTurn').waitFor({state:'visible'});await idle(page);
      await page.unroute('**/api/advance',handler);if(interceptError)throw interceptError;
      assert(lost);assert.equal(lost.status,200);assert.deepEqual(lost.response.errors||[],[]);assert.equal(lost.payload.days,1);assert.deepEqual(lost.payload.commands,[]);
      assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(pendingAdvance.payload))),lost.payload);
      const committed=await snapshot('committed-response-lost',test,e);progressed(before.marker,committed.marker,test);e.lost={payload:lost.payload,status:lost.status,date:lost.response.date,actual_native_commit:true,browser_banner:await page.locator('#banner').innerText()};
      mark(test.id+': reload Continue frozen retry');await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').waitFor({state:'visible'});
      assert.equal((await get(page,url,'/api/state')).date,committed.state.date);await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await idle(page);
      assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(pendingAdvance.payload))),lost.payload);
      const reloaded=await snapshot('browser-reloaded-pending',test,e);assert(reloaded.normalized===committed.normalized,'Browser reload/Continue changed committed campaign');
      for(const width of [1440,390]){await page.setViewportSize({width,height:width===390?844:1000});await page.locator('#retryAdvanceBtn').scrollIntoViewIfNeeded();const name=test.id+'-pending-'+width+'.png';await page.screenshot({path:path.join(out,name)});e.screenshots.push({path:name,sha256:fileHash(path.join(out,name)),width});}
      const replay=page.waitForResponse(r=>isPost(r,'/api/advance'));await page.locator('#retryAdvanceBtn').click();const replayResponse=await replay,replayValue=await replayResponse.json();assert(replayResponse.ok());assert.deepEqual(replayValue.errors||[],[]);assert.deepEqual(replayResponse.request().postDataJSON(),lost.payload);
      await page.locator('#pendingTurn').waitFor({state:'hidden'});await idle(page);assert.equal(await page.evaluate(()=>pendingAdvance),null);
      const retried=await snapshot('receipt-retried',test,e);assert(retried.normalized===committed.normalized,'Lost-day retry changed complete campaign bytes');e.retry={identical_frozen_payload:true,exact_world_history_log_journey:true,day_advanced_again:false};
      mark(test.id+': named Load replaces session');await load(page,OBSERVE);const loaded=await snapshot('named-loaded',test,e);assert.notEqual(loaded.state.session_id,committed.state.session_id);assert(loaded.normalized===committed.normalized,'Named Load changed complete committed campaign');
      mark(test.id+': refuse old-session day');const stale=await page.request.post(url+'/api/advance',{data:lost.payload});e.stale={status:stale.status(),response:await stale.json(),payload:lost.payload};await stale.dispose();assert.equal(e.stale.status,400);assert.equal(e.stale.response.requires_review,true);
      const refused=await snapshot('stale-refused',test,e);assert(refused.normalized===loaded.normalized,'Old-session day changed restored campaign');
      mark(test.id+': ordinary continuation');e.continuation=await advance(page);const continued=await snapshot('continued',test,e);assert.equal(continued.marker.day,loaded.marker.day+1);assert.equal(continued.state.session_id,loaded.state.session_id);
      assert.equal(fileHash(source),test.gzip_sha256);assert.equal(fileHash(manifestPath),test.manifest_sha256);e.passed=true;write();await page.setViewportSize({width:1440,height:1000});
    }
    assert.equal(proof.cases.length,5);assert(proof.cases.every(row=>row.passed));assert.deepEqual(proof.errors,[]);
    assert.equal(fileHash(binary),proof.binary_sha256);assert.equal(fileHash(__filename),proof.driver_sha256);assert.equal(fileHash(configPath),proof.config_sha256);assert.equal(fileHash(path.join(__dirname,'ci-integrated.cjs')),proof.build_verifier_sha256);
    proof.passed=true;
  }catch(error){proof.failed_stage=stage;proof.failure=String(error.stack||error);if(page){await page.screenshot({path:path.join(out,'failure.png')}).catch(()=>{});await page.content().then(html=>fs.writeFileSync(path.join(out,'failure.html'),html)).catch(()=>{});}throw error;}
  finally{proof.finished_utc=new Date().toISOString();write();if(browser)await browser.close();if(server.exitCode===null){await new Promise(resolve=>{const timer=setTimeout(resolve,5000);server.once('exit',()=>{clearTimeout(timer);resolve();});server.kill();});}log.end();}
  console.log(JSON.stringify({passed:true,cases:5,result:path.join(out,'result.json')}));
}
main().catch(error=>{console.error(error);process.exitCode=1;});
