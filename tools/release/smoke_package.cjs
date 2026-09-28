'use strict';
// Only test tooling comes from this checkout. The game, artwork and meshes must
// come from a hash-verified extraction of the supplied portable archive.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const net = require('node:net');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const zlib = require('node:zlib');
const sha = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const fileHash = file => sha(fs.readFileSync(file));

function inside(parent, file) {
  const rel = path.relative(path.resolve(parent), path.resolve(file));
  assert(rel && rel !== '..' && !rel.startsWith('..' + path.sep) && !path.isAbsolute(rel), 'Path escaped owned output');
  return path.resolve(file);
}
function regular(file) {
  assert(fs.lstatSync(file).isFile() && !fs.lstatSync(file).isSymbolicLink(), 'Expected ordinary file: ' + file);
  assert.equal(fs.realpathSync(file), path.resolve(file), 'File resolves through a link');
  return file;
}
function allowedRequest(value, origin) {
  try { const url = new URL(value); return url.protocol === 'http:' && url.origin === origin && url.hostname === '127.0.0.1' && !url.username && !url.password; }
  catch { return false; }
}
function archive(file) {
  const bytes = fs.readFileSync(regular(file)), text = bytes.toString('utf8');
  assert(text.startsWith('{"format":"spheres-campaign","version":1,'));
  const match = /^(.*),"saved_unix":(0|[1-9][0-9]*)}$/s.exec(text);
  assert(match && BigInt(match[2]) <= 18446744073709551615n, 'Missing terminal native timestamp');
  const normalized = match[1] + '}';
  const parsed = JSON.parse(text);
  return {bytes, normalized, record:{bytes:bytes.length,sha256:sha(bytes),campaign_sha256:sha(normalized),
    date:parsed.saved_date,player:parsed.player,history_rows:parsed.history.length,log_rows:parsed.log.length}};
}
function extractionIdentity(report, revision, out) {
  assert.equal(report.format, 'spheres-package-verification/v1'); assert.equal(report.passed, true);
  const root = inside(path.join(out,'extracted'), report.extracted_root);
  assert.equal(fs.realpathSync(root), root);
  const manifest = report.manifest;
  assert.equal(manifest.format, 'spheres-release/v1'); assert.equal(manifest.revision, revision);
  assert.equal(manifest.short_revision, revision.slice(0,12));
  const expectedPlatform = process.platform === 'win32' ? 'windows' : process.platform;
  assert.equal(manifest.platform, expectedPlatform, 'Package must match this operating system');
  assert.equal(manifest.native_build_info.full_revision, revision);
  assert.equal(manifest.native_build_info.revision, revision.slice(0,12));
  assert.equal(manifest.executable.path, expectedPlatform === 'windows' ? 'spheres-web.exe' : 'spheres-web');
  const binary = regular(inside(root, path.join(root,manifest.executable.path)));
  assert.equal(fileHash(binary), manifest.executable.sha256);
  assert.equal(fs.statSync(binary).size, manifest.executable.bytes);
  return {root,manifest,binary};
}
function imageKind(bytes, kind) {
  assert(bytes.length > 1024, 'Representative image is empty or a placeholder');
  if(kind === 'png') assert(bytes.subarray(0,8).equals(Buffer.from([137,80,78,71,13,10,26,10])));
  else { assert.equal(bytes.toString('ascii',0,4),'RIFF'); assert.equal(bytes.toString('ascii',8,12),'WEBP'); }
}
async function port() {
  const s = net.createServer(); await new Promise((resolve,reject) => {s.once('error',reject);s.listen(0,'127.0.0.1',resolve);});
  const p = s.address().port; await new Promise(resolve => s.close(resolve)); return p;
}
const post = (response, route) => response.request().method() === 'POST' && new URL(response.url()).pathname === route;
async function idle(page) { await page.waitForFunction(() => !SESSION.busy && !advancing && !pendingAdvance && !COMMAND_CHANNEL.busy && !COMMAND_CHANNEL.pending); }
async function campaigns(page) {
  if(await page.locator('#app').isVisible()) {
    if(!await page.locator('#campaignsBtn').isVisible()) await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if(!await page.locator('#campaignHome').isVisible()) await page.locator('#savedCampaigns [data-menu-back]').click();
  await page.locator('#openSavesBtn').click(); await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function save(page, slot) {
  await page.locator('#saveName').fill(slot);
  const received=page.waitForResponse(response => post(response,'/api/save'));
  await page.locator('#saveNamedBtn').click(); const response=await received, body=await response.json();
  assert(response.ok()); assert.equal(body.ok,true);
  await page.waitForFunction(() => !document.getElementById('saveNamedBtn').disabled);
  return {payload:response.request().postDataJSON(),response:body};
}
async function resume(page) {
  await page.locator('#savedCampaigns [data-menu-back]').click(); await page.locator('#continueBtn').click();
  await page.locator('#app').waitFor({state:'visible'}); await idle(page);
}
async function load(page, slot, backup) {
  await page.locator('#saveSlots').selectOption(slot);
  await page.locator(backup ? '#loadBackupBtn' : '#loadBtn').click();
  await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
  const received=page.waitForResponse(response => post(response,'/api/load'));
  await page.locator('#campaignConfirmAccept').click(); const response=await received;
  assert(response.ok()); assert.equal(response.request().postDataJSON().slot,slot);
  assert.equal(!!response.request().postDataJSON().backup,backup);
  await page.locator('#app').waitFor({state:'visible'}); await idle(page);
  return {payload:response.request().postDataJSON(),response:await response.json()};
}
async function stop(child) {
  if(!child || child.exitCode !== null || child.signalCode !== null)return;
  const exited = new Promise(resolve => child.once('exit',resolve)); child.kill();
  let timer; await Promise.race([exited,new Promise((_,reject) => {timer=setTimeout(() => reject(Error('Owned server failed to stop')),10000);})]).finally(() => clearTimeout(timer));
}

async function main(argv=process.argv.slice(2)) {
  const [archiveArg,revision,outputArg] = argv;
  assert(archiveArg && outputArg && /^[a-f0-9]{40}$/.test(revision), 'ZIP FULL_REVISION NEW_OUTPUT required');
  const zip=regular(path.resolve(archiveArg)), out=path.resolve(outputArg);
  assert(!fs.existsSync(out),'Evidence output must be new'); fs.mkdirSync(out,{recursive:true});
  assert.equal(fs.realpathSync(out),out);
  const proof={format:'spheres-package-smoke/v1',scope:'Extracted portable package functional/offline smoke; not full campaign certification or performance qualification',
    revision,archive:{path:zip,sha256:fileHash(zip)},driver_sha256:fileHash(__filename),started_utc:new Date().toISOString(),
    passed:false,errors:[],console_errors:[],blocked_requests:[],http_errors:[],requests:[],screenshots:[],assets:[]};
  let stage='verify and extract',server,browser,page,serverLog;
  const write=(name,value) => fs.writeFileSync(inside(out,path.join(out,name)),JSON.stringify(value,null,2)+'\n');
  const mark=name => {stage=name;fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({stage,utc:new Date().toISOString()})+'\n');};
  try {
    const verifier=path.join(__dirname,'verify_package.py');
    proof.extractor_sha256=fileHash(verifier);
    const verifierImplementation=path.join(__dirname,'release_package.py');
    proof.extractor_implementation_sha256=fileHash(verifierImplementation);
    const extracted=cp.spawnSync(process.env.SPHERES_PYTHON||'python',['-B',verifier,zip,path.join(out,'extracted'),'--report',path.join(out,'extraction.json')],
      {windowsHide:true,encoding:'utf8',maxBuffer:4*1024*1024,timeout:120000});
    fs.writeFileSync(path.join(out,'extraction.log'),(extracted.stdout||'')+(extracted.stderr||''));
    if(extracted.error)throw extracted.error; assert.equal(extracted.status,0,'Package extraction verification failed');
    const verified=JSON.parse(fs.readFileSync(path.join(out,'extraction.json'),'utf8'));
    const {root,manifest,binary}=extractionIdentity(verified,revision,out);
    proof.package_root=root; proof.binary_sha256=fileHash(binary); proof.extraction=verified;
    const packagePins=Object.fromEntries(['release.json','SHA256SUMS.txt',...Object.keys(manifest.files)].map(name => [name,fileHash(regular(inside(root,path.join(root,name))))]));
    const url='http://127.0.0.1:'+await port(); proof.origin=url;
    server=cp.spawn(binary,['--port',url.split(':').at(-1),'--no-open'],{cwd:root,windowsHide:true,stdio:['ignore','pipe','pipe']});
    let launchError; server.on('error',error => {launchError=error;});
    serverLog=fs.createWriteStream(path.join(out,'server.log')); server.stdout.pipe(serverLog,{end:false}); server.stderr.pipe(serverLog,{end:false});
    const deadline=Date.now()+45000;
    for(;;) {
      if(launchError)throw launchError; assert.equal(server.exitCode,null,'Extracted server exited');
      try {const r=await fetch(url+'/api/build',{redirect:'error',signal:AbortSignal.timeout(2000)});if(r.ok){proof.build=await r.json();break;}await r.arrayBuffer();}
      catch(error){if(Date.now()>=deadline)throw error;}
      assert(Date.now()<deadline,'Extracted server startup timed out'); await new Promise(resolve=>setTimeout(resolve,100));
    }
    assert.equal(proof.build.full_revision,revision); assert.equal(proof.build.revision,revision.slice(0,12));
    assert.equal(fs.realpathSync(proof.build.save_directory),root,'The extracted application must use its own save directory');
    for(const key of ['version','target_os','target_arch','built_at_unix_seconds','branch'])assert.deepEqual(proof.build[key],manifest.native_build_info[key],key);
    const {chromium}=require(require.resolve('playwright',{paths:[path.resolve(__dirname,'../ui'),...module.paths]}));
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL ? {channel:process.env.SPHERES_BROWSER_CHANNEL} : {})});
    proof.browser_version=browser.version();
    const context=await browser.newContext({viewport:{width:1440,height:1000},deviceScaleFactor:1,reducedMotion:'reduce',serviceWorkers:'block'});
    await context.route('**/*',route => {
      if(allowedRequest(route.request().url(),url))return route.continue();
      proof.blocked_requests.push({stage,url:route.request().url()}); return route.abort('blockedbyclient');
    });
    await context.addInitScript(() => {
      const original=HTMLCanvasElement.prototype.getContext;
      window.__packageDraws=[];
      HTMLCanvasElement.prototype.getContext=function(type,...args){
        const gl=original.call(this,type,...args);
        if(!gl || !['webgl','webgl2'].includes(type) || __packageDraws.some(row=>row.gl===gl))return gl;
        const row={canvas:this,gl,type,draws:0}; __packageDraws.push(row);
        for(const name of ['drawArrays','drawElements']) {const fn=gl[name];gl[name]=function(...values){const result=fn.apply(this,values);if(!gl.isContextLost())row.draws++;return result;};}
        return gl;
      };
    });
    page=await context.newPage(); page.setDefaultTimeout(45000);
    page.on('pageerror',e=>proof.errors.push(e.message));
    page.on('console',message=>{if(message.type()==='error')proof.console_errors.push({stage,text:message.text()});});
    page.on('response',r=>{if(r.status()>=400)proof.http_errors.push({stage,url:r.url(),status:r.status()});});
    page.on('request',r=>{if(r.method()==='POST')proof.requests.push({stage,path:new URL(r.url()).pathname,payload:r.postDataJSON()});});
    async function state(){const r=await page.request.get(url+'/api/state',{maxRedirects:0});try{assert(r.ok());return await r.json();}finally{await r.dispose();}}
    async function shot(name,selector) {
      const layout=await page.locator(selector).evaluate(el=>({width:el.clientWidth,scroll:el.scrollWidth}));
      assert(layout.width>0&&layout.scroll<=layout.width+1,'Visible panel has horizontal overflow: '+JSON.stringify(layout));
      assert(!/\bNaN\b|\bundefined\b/.test(await page.locator(selector).innerText()));
      const namePng=name+'.png'; await page.screenshot({path:path.join(out,namePng)});
      proof.screenshots.push({path:namePng,sha256:fileHash(path.join(out,namePng)),viewport:page.viewportSize(),layout});
    }
    function retain(name,data) {
      const packed=zlib.gzipSync(data.bytes,{level:9}),file=name+'.json.gz';
      assert(zlib.gunzipSync(packed).equals(data.bytes),'Campaign gzip round trip failed');
      fs.writeFileSync(path.join(out,file),packed,{flag:'wx'});
      return {...data.record,retained:{path:file,bytes:packed.length,sha256:sha(packed),decoded_sha256:sha(data.bytes),decoded_bytes:data.bytes.length}};
    }
    mark('ordinary offline France startup'); await page.goto(url,{waitUntil:'domcontentloaded'});
    await page.waitForFunction(()=>!!SESSION.live?.session_id); assert.equal((await state()).player,null);
    await page.locator('#newCampaignBtn').click(); await page.locator('#nationPick [aria-label^="France;"]').click();
    await shot('nation-selection-desktop','#newCampaignPicker');
    const created=page.waitForResponse(r=>post(r,'/api/new')); await page.locator('#startBtn').click(); assert((await created).ok());
    await page.locator('#app').waitFor({state:'visible'});await idle(page);
    const opening=await state(); assert.equal(opening.player,'France');assert.equal(opening.simulation_cadence,'daily');
    assert.deepEqual([opening.year,opening.month,opening.day],[1990,1,1],'Ordinary fresh campaign must start on January1,1990');
    await page.waitForFunction(()=>typeof GL!=='undefined'&&GL.ready&&GL.ok);
    for(const width of [1440,390]) {await page.setViewportSize({width,height:width===390?844:1000});await shot('map-'+width,'#app');}
    await page.setViewportSize({width:1440,height:1000});
    mark('representative embedded assets');
    async function resource(route) {assert(allowedRequest(url+route,url));const expected=manifest.runtime_assets?.[route];assert(expected,'Package lacks expected asset pin: '+route);
      const r=await page.request.get(url+route,{maxRedirects:0});try{assert(r.ok(),route+' '+r.status());const bytes=await r.body();assert.equal(bytes.length,expected.bytes,route+' size');assert.equal(sha(bytes),expected.sha256,route+' hash');
        proof.assets.push({path:route,bytes:bytes.length,sha256:sha(bytes),expected,content_type:r.headers()['content-type']});return bytes;}finally{await r.dispose();}}
    for(const route of ['/art/pages/nation-selection-v1.webp','/art/pages/saved-campaigns-v1.webp','/art/pages/tank-designer-v1.webp'])imageKind(await resource(route),'webp');
    for(const route of ['/height-detail.png','/relief.png'])imageKind(await resource(route),'png');
    const terrain=JSON.parse((await resource('/terrain-tiles/manifest.json')).toString('utf8'));
    const tile=Object.values(terrain.tiles).find(row=>typeof row.file==='string'&&row.bytes>1024);assert(tile);
    assert(/^[a-z0-9_]+\.png$/.test(tile.file));const tileBytes=await resource('/terrain-tiles/'+tile.file);imageKind(tileBytes,'png');assert.equal(tileBytes.length,tile.bytes);assert.equal(sha(tileBytes),tile.sha256);
    for(const route of ['/arsenal-models.js','/equipment-mesh.js','/equipment-model.js','/equipment-export.js'])assert((await resource(route)).length>1024);
    mark('released mesh in native designer');
    if(await page.locator('#intelDrawer').getAttribute('aria-hidden')!=='false')await page.locator('[data-drawer="intelDrawer"]').click();
    await page.locator('#warsCard [data-ground-equipment-tab="service"]').click();await page.waitForFunction(()=>equipmentCurrent());
    await page.locator('[role="tab"][data-equipment-tab="designer"]').click();await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='designer'&&!equipmentPending());
    const family=await page.evaluate(()=>equipmentFamily(equipmentRows('platforms').find(row=>row.id==='air_fighter')));assert(family);
    await page.locator('[data-equipment-family='+JSON.stringify(family)+']').click();await page.waitForFunction(()=>!equipmentPending());
    if(await page.locator('#equipmentPlatform').inputValue()!=='air_fighter')await page.locator('#equipmentPlatform').selectOption('air_fighter');
    await page.locator('[data-model-canvas]').scrollIntoViewIfNeeded();
    await page.waitForFunction(()=>document.querySelector('[data-model-status]')?.textContent.includes('3D model ready'));
    await page.waitForFunction(()=>__packageDraws.some(row=>row.canvas.isConnected&&row.canvas.closest('[data-model-canvas]')&&row.draws>0));
    proof.model=await page.evaluate(()=>({platform:EQUIP.draft.platform,triangles:EquipmentMesh.build(EQUIP.draft).triangleCount,
      draws:__packageDraws.filter(row=>row.canvas.isConnected&&row.canvas.closest('[data-model-canvas]')).map(row=>({type:row.type,draws:row.draws,width:row.canvas.width,height:row.canvas.height}))}));
    assert.equal(proof.model.platform,'air_fighter');assert(proof.model.triangles>=100000);assert(proof.model.draws.some(row=>row.draws>0&&row.width>0&&row.height>0));
    await shot('embedded-aircraft-designer','#equipmentRoom');await page.locator('[data-equipment-close]').click();
    mark('save and ordinary day');await campaigns(page);proof.save_a=await save(page,'package-smoke');
    const primary=inside(root,path.join(root,'saves/package-smoke.json')),backup=primary+'.bak',a=archive(primary);proof.archive_a=retain('archive-a',a);
    await resume(page);const advanced=page.waitForResponse(r=>post(r,'/api/advance'));await page.locator('#stepBtn').click();const advancedResponse=await advanced;
    assert(advancedResponse.ok());assert.equal(advancedResponse.request().postDataJSON().days,1);await idle(page);
    const after=await state();assert.deepEqual([after.year,after.month,after.day],[1990,1,2],'The single ordinary step must advance exactly one calendar day');assert.equal(after.session_id,opening.session_id);
    proof.advance={before:opening.date,after:after.date,before_calendar:[opening.year,opening.month,opening.day],after_calendar:[after.year,after.month,after.day],requested_days:1};
    await campaigns(page);proof.save_b=await save(page,'package-smoke');const b=archive(primary);assert(a.normalized!==b.normalized,'Ordinary day did not change full campaign');assert.equal(fileHash(backup),a.record.sha256);proof.archive_b=retain('archive-b',b);
    mark('ordinary named reload');proof.normal_load=await load(page,'package-smoke',false);const normal=await state();assert.equal(normal.date,after.date);assert.notEqual(normal.session_id,after.session_id);
    await campaigns(page);await save(page,'package-loaded');const loaded=archive(path.join(root,'saves/package-loaded.json'));assert(loaded.normalized===b.normalized,'Normal reload changed full campaign');proof.loaded_archive=retain('archive-loaded',loaded);
    mark('owned missing-primary backup fixture');const displaced=inside(out,path.join(out,'removed-primary.json'));regular(primary);fs.renameSync(primary,displaced);assert(!fs.existsSync(primary));proof.primary_displacement={path:displaced,sha256:fileHash(displaced)};
    await campaigns(page);await page.waitForFunction(()=>SESSION.saves?.some(row=>row.slot==='package-smoke'&&row.current_exists===false&&row.metadata_from_backup));await page.locator('#saveSlots').selectOption('package-smoke');
    for(const width of [1440,390]) {
      await page.setViewportSize({width,height:width===390?844:1000});assert(await page.locator('#loadBtn').isDisabled());assert(await page.locator('#loadBackupBtn').isEnabled());
      assert((await page.locator('#saveRecoveryStatus').innerText()).includes('main save is missing'));await page.locator('#loadBackupBtn').scrollIntoViewIfNeeded();await shot('backup-recovery-'+width,'#savedCampaigns');
    }
    mark('ordinary backup load');proof.backup_load=await load(page,'package-smoke',true);const restored=await state();assert.equal(restored.date,opening.date);assert.notEqual(restored.session_id,normal.session_id);assert(!fs.existsSync(primary));assert.equal(fileHash(backup),a.record.sha256);
    await campaigns(page);proof.recovered_save=await save(page,'package-recovered');const recovered=archive(path.join(root,'saves/package-recovered.json'));assert(recovered.normalized===a.normalized,'Backup recovery changed full campaign');proof.recovered_archive={...retain('archive-recovered',recovered),exact_except_saved_unix:true};
    assert.deepEqual(proof.blocked_requests,[],'Package attempted an external network dependency');assert.deepEqual(proof.errors,[]);assert.deepEqual(proof.http_errors,[]);assert.deepEqual(proof.console_errors,[]);
    assert.equal(fileHash(binary),proof.binary_sha256);assert.equal(fileHash(zip),proof.archive.sha256);assert.equal(fileHash(__filename),proof.driver_sha256);assert.equal(fileHash(verifier),proof.extractor_sha256);
    assert.equal(fileHash(verifierImplementation),proof.extractor_implementation_sha256);
    for(const [name,hash] of Object.entries(packagePins))assert.equal(fileHash(path.join(root,name)),hash,'Package payload changed: '+name);
    proof.package_payload_unchanged=true;proof.passed=true;
  } catch(error) {
    proof.failed_stage=stage;proof.failure=String(error.stack||error);
    if(page){await page.screenshot({path:path.join(out,'failure.png')}).catch(()=>{});await page.content().then(html=>fs.writeFileSync(path.join(out,'failure.html'),html)).catch(()=>{});}
  } finally {
    try {if(browser)await browser.close();await stop(server);proof.server_stopped=true;}catch(error){proof.cleanup_error=String(error);proof.passed=false;}
    if(serverLog)await new Promise(resolve=>serverLog.end(resolve));
    proof.finished_utc=new Date().toISOString();write('result.json',proof);
  }
  console.log(JSON.stringify({passed:proof.passed,result:path.join(out,'result.json')}));
  return proof;
}
module.exports={allowedRequest,archive,extractionIdentity,imageKind,inside,main};
if(require.main===module)main().then(result=>{if(!result.passed)process.exitCode=1;}).catch(error=>{console.error(error);process.exitCode=1;});
