// Disposable native/browser backup recovery probe. No injected campaign state.
// NODE_PATH=<Playwright modules> SPHERES_BROWSER_CHANNEL=msedge node this-file BINARY FULL_REVISION NEW_OUTPUT
// The sole direct POST is an intentionally stale save, which must be refused.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const net = require('node:net');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {verifyBuild} = require('./ci-integrated.cjs');
const root = path.resolve(__dirname, '../..');
const sha = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const fileHash = file => sha(fs.readFileSync(file));
const post = (response, route) => response.request().method() === 'POST' && new URL(response.url()).pathname === route;
const SLOT = 'backup-recovery', RECOVERED = 'backup-recovered-check';
async function freePort() {
  const listener = net.createServer();
  await new Promise(resolve => listener.listen(0, '127.0.0.1', resolve));
  const port = listener.address().port;
  await new Promise(resolve => listener.close(resolve));
  return port;
}
function inside(parent, file) {
  const relative = path.relative(parent, file);
  assert(relative && relative !== '..' && !relative.startsWith('..'+path.sep) && !path.isAbsolute(relative), 'Path escaped owned output');
  return file;
}
function regular(file) {
  assert(fs.lstatSync(file).isFile() && !fs.lstatSync(file).isSymbolicLink(), 'Expected ordinary file: '+file);
  assert.equal(fs.realpathSync(file), path.resolve(file));
  return file;
}
function archive(file) {
  const bytes = fs.readFileSync(regular(file)), text = bytes.toString('utf8'), value = JSON.parse(text);
  assert.equal(value.format, 'spheres-campaign');
  for (const key of ['world','history','log','journey','history_epoch','saved_date','player','saved_unix']) {
    assert(Object.hasOwn(value,key), 'Native archive is missing '+key);
  }
  assert(Number.isSafeInteger(value.saved_unix) && value.saved_unix > 0);
  // Compare all native serialized bytes, including floats/order/history, while
  // excluding only the final envelope timestamp emitted by storage::encode.
  const timestamp = /("saved_unix":)\d+(?=\s*}\s*$)/;
  assert(timestamp.test(text), 'Expected the native terminal save timestamp');
  const normalized = text.replace(timestamp, '$1<TIMESTAMP>');
  return {bytes, normalized, record:{path:file,bytes:bytes.length,sha256:sha(bytes),campaign_bytes_sha256:sha(normalized),
    player:value.player,date:value.saved_date,saved_unix:value.saved_unix,history_rows:value.history.length,log_rows:value.log.length}};
}
async function get(page,url,route) {
  const response = await page.request.get(url+route);
  try { assert(response.ok(), route+': '+response.status()); return await response.json(); }
  finally { await response.dispose(); }
}
async function idle(page) {
  await page.waitForFunction(() => !SESSION.busy && !advancing && !pendingAdvance && !COMMAND_CHANNEL.busy && !COMMAND_CHANNEL.pending);
}
async function campaigns(page) {
  if (await page.locator('#app').isVisible()) {
    if (!await page.locator('#campaignsBtn').isVisible()) await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if (!await page.locator('#campaignHome').isVisible()) await page.locator('#savedCampaigns [data-menu-back]').click();
  await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function save(page,slot) {
  await page.locator('#saveName').fill(slot);
  const received = page.waitForResponse(response => post(response,'/api/save'));
  await page.locator('#saveNamedBtn').click();
  const response = await received;
  const result = await response.json();
  assert(response.ok()); assert.equal(result.ok,true);
  await page.waitForFunction(() => !document.getElementById('saveNamedBtn').disabled);
  return {payload:response.request().postDataJSON(),response:result};
}
async function main() {
  const [binaryArg, revision, outputArg] = process.argv.slice(2);
  assert(binaryArg && outputArg && /^[a-f0-9]{40}$/.test(revision), 'Supply binary, full revision and new output directory');
  const binary = regular(path.resolve(binaryArg)), out = path.resolve(outputArg);
  assert(!fs.existsSync(out), 'Preserve prior evidence: output directory must be new');
  const run = inside(out,path.join(out,'disposable-campaign'));
  fs.mkdirSync(run,{recursive:true});
  assert.equal(fs.realpathSync(out),out, 'Output must not resolve through a link');
  const port = await freePort(), url = 'http://127.0.0.1:'+port;
  const proof = {scope:'Ordinary fresh France save-response loss and missing-primary backup recovery; bounded S24 preparation, not worldwide recovery qualification',
    revision,source_revision:cp.execFileSync('git',['rev-parse','HEAD'],{cwd:root,windowsHide:true,encoding:'utf8'}).trim(),
    driver_sha256:fileHash(__filename),build_verifier_sha256:fileHash(path.join(__dirname,'ci-integrated.cjs')),
    binary_sha256:fileHash(binary),started_utc:new Date().toISOString(),url,run,requests:[],request_failures:[],screenshots:[],errors:[],passed:false};
  let stage = 'launch', browser, page, launchError;
  const write = (name,value) => fs.writeFileSync(inside(out,path.join(out,name)),JSON.stringify(value,null,2)+'\n');
  const mark = name => { stage=name; fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({stage,utc:new Date().toISOString()})+'\n'); };
  const server = cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  server.on('error',error => { launchError=error; });
  const log = fs.createWriteStream(path.join(out,'server.log'));
  server.stdout.pipe(log); server.stderr.pipe(log);
  try {
    const deadline = Date.now()+30000;
    for (;;) {
      if (launchError) throw launchError;
      assert.equal(server.exitCode,null,'Disposable server exited');
      try { const response=await fetch(url+'/api/build',{signal:AbortSignal.timeout(2000)}); await response.arrayBuffer(); if(response.ok)break; }
      catch(error) { if(Date.now()>=deadline)throw error; }
      assert(Date.now()<deadline,'Disposable server startup timed out');
      await new Promise(resolve => setTimeout(resolve,100));
    }
    browser = await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'msedge'});
    proof.browser_version=browser.version();
    page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);
    page.on('pageerror',error => proof.errors.push(error.message));
    page.on('request',request => { if(request.method()==='POST')proof.requests.push({stage,path:new URL(request.url()).pathname,payload:request.postDataJSON()}); });
    page.on('requestfailed',request => proof.request_failures.push({stage,path:new URL(request.url()).pathname,error:request.failure()?.errorText}));
    process.env.SPHERES_EXPECTED_REVISION=revision;
    proof.build=await verifyBuild({page,url,root,run,binary});
    mark('fresh ordinary France');
    await page.goto(url,{waitUntil:'domcontentloaded'});
    await page.waitForFunction(() => !!SESSION.live?.session_id);
    const opening=await get(page,url,'/api/state');
    assert.equal(opening.player,null,'A new disposable server must have no player campaign');
    await page.locator('#newCampaignBtn').click();
    await page.locator('#nationPick [aria-label^="France;"]').click();
    const started=page.waitForResponse(response => post(response,'/api/new'));
    await page.locator('#startBtn').click();
    assert((await started).ok());
    await page.locator('#app').waitFor({state:'visible'}); await idle(page);
    const aState=await get(page,url,'/api/state');
    assert.equal(aState.player,'France');
    assert.equal(await page.evaluate(() => clock.running),false);
    mark('save A'); await campaigns(page);
    proof.save_a=await save(page,SLOT);
    const primary=inside(run,path.join(run,'saves',SLOT+'.json')), backup=inside(run,primary+'.bak');
    const a=archive(primary);
    fs.writeFileSync(path.join(out,'archive-a.json'),a.bytes);
    proof.archive_a={...a.record,retained_path:path.join(out,'archive-a.json')};
    assert(!fs.existsSync(backup),'The first save has no older backup');
    await page.locator('#savedCampaigns [data-menu-back]').click();
    await page.locator('#continueBtn').click(); await page.locator('#app').waitFor({state:'visible'}); await idle(page);
    mark('advance one ordinary day');
    const advanced=page.waitForResponse(response => post(response,'/api/advance'));
    await page.locator('#stepBtn').click();
    const dayResponse=await advanced, day=await dayResponse.json();
    assert(dayResponse.ok()); assert.deepEqual(day.errors||[],[]);
    assert.equal(dayResponse.request().postDataJSON().days,1);
    await idle(page);
    const bState=await get(page,url,'/api/state');
    assert.notEqual(bState.date,aState.date); assert.equal(bState.session_id,aState.session_id);
    proof.advance={payload:dayResponse.request().postDataJSON(),before:aState.date,after:bState.date};
    mark('commit B and lose HTTP response'); await campaigns(page); await page.locator('#saveName').fill(SLOT);
    let lost, interceptError;
    const lose = async route => {
      try {
        const response=await route.fetch();
        lost={payload:route.request().postDataJSON(),status:response.status(),response:await response.json()};
        await response.dispose();
        await route.abort('failed');
      } catch(error) { interceptError=error; await route.abort('failed').catch(()=>{}); }
    };
    await page.route('**/api/save',lose);
    await page.locator('#saveNamedBtn').click();
    await page.waitForFunction(() => !document.getElementById('saveNamedBtn').disabled && /Save failed:/.test(document.getElementById('banner').textContent));
    await page.unroute('**/api/save',lose);
    if(interceptError)throw interceptError;
    assert(lost); assert.equal(lost.status,200); assert.equal(lost.response.ok,true);
    proof.lost_response={...lost,browser_banner:await page.locator('#banner').innerText(),method:'Actual native route.fetch commit followed by route.abort; no replacement success body'};
    const b=archive(primary);
    fs.writeFileSync(path.join(out,'archive-b.json'),b.bytes); proof.archive_b={...b.record,retained_path:path.join(out,'archive-b.json')};
    assert.notEqual(b.normalized,a.normalized,'B must contain the actual advanced day');
    assert.equal(fileHash(backup),a.record.sha256,'B must retain exact A bytes as its backup');
    // Cross a timestamp second so a repeated same-state save genuinely exercises
    // storage's timestamp-only no-op, not accidental identical wall-clock time.
    await page.waitForFunction(seconds => Date.now()>=(seconds+1)*1000+25,b.record.saved_unix);
    mark('retry visible Save without advancing');
    proof.save_retry=await save(page,SLOT);
    assert.deepEqual(proof.save_retry.payload,lost.payload);
    assert(archive(primary).normalized===b.normalized,'Retry changed campaign data');
    assert.equal(fileHash(backup),a.record.sha256,'Retry must not replace older A with another copy of B');
    assert.deepEqual(await get(page,url,'/api/state'),bState,'Saving/retrying changed the live campaign');
    proof.save_retry_note='Repeated save uses native same-campaign comparison; this is not command-receipt idempotency.';
    mark('remove primary by retained rename');
    const retained=inside(out,path.join(out,'removed-primary.json'));
    assert(!fs.existsSync(retained)); regular(primary);
    assert.equal(fs.realpathSync(path.dirname(primary)),path.dirname(primary));
    fs.renameSync(primary,retained);
    assert(!fs.existsSync(primary)); assert.equal(archive(retained).normalized,b.normalized);
    proof.primary_displacement={from:primary,to:retained,sha256:fileHash(retained),reason:'Owned disposable filesystem failure fixture; original primary bytes retained'};
    const listing=await get(page,url,'/api/saves'); write('missing-primary-saves.json',listing);
    const entry=listing.slots.find(row => row.slot===SLOT); assert(entry,'Backup-only slot must remain discoverable');
    assert.equal(entry.current_exists,false); assert.equal(entry.readable,false); assert.equal(entry.backup,true); assert.equal(entry.metadata_from_backup,true);
    assert.equal(entry.date,a.record.date); assert.equal(entry.player,'France'); proof.missing_primary=entry;
    await campaigns(page);
    await page.waitForFunction(slot => SESSION.saves?.some(row => row.slot===slot && row.current_exists===false && row.metadata_from_backup===true),SLOT);
    await page.locator('#saveSlots').selectOption(SLOT);
    for(const width of [1440,390]) {
      await page.setViewportSize({width,height:width===390?844:1000});
      assert(await page.locator('#loadBtn').isDisabled()); assert(await page.locator('#loadBackupBtn').isEnabled());
      const label=await page.locator('#saveSlots option:checked').textContent();
      assert.match(label,/main save missing/); assert.match(label,/backup available/); assert(label.includes(a.record.date));
      assert(await page.locator('#saveRecoveryStatus').isVisible());
      const recoveryMessage=await page.locator('#saveRecoveryStatus').innerText();
      assert(recoveryMessage.includes(a.record.date) && recoveryMessage.includes('main save is missing'));
      const layout=await page.locator('#savedCampaigns').evaluate(element => ({client:element.clientWidth,scroll:element.scrollWidth}));
      assert(layout.client>0 && layout.scroll<=layout.client+1,'Saved campaigns panel overflows: '+JSON.stringify(layout));
      assert(!/\bNaN\b|\bundefined\b/.test(await page.locator('#savedCampaigns').innerText()));
      await page.locator('#loadBackupBtn').scrollIntoViewIfNeeded();
      const shot='missing-primary-'+width+'.png'; await page.screenshot({path:path.join(out,shot)});
      proof.screenshots.push({path:shot,sha256:fileHash(path.join(out,shot)),width,label,layout,normal_load_disabled:true,backup_load_enabled:true});
    }
    mark('ordinary backup confirmation and load');
    await page.locator('#loadBackupBtn').click();
    assert.match(await page.locator('#campaignConfirmMessage').innerText(),/backup/);
    const loaded=page.waitForResponse(response => post(response,'/api/load'));
    await page.locator('#campaignConfirmAccept').click();
    const loadResponse=await loaded; assert(loadResponse.ok());
    assert.equal(loadResponse.request().postDataJSON().backup,true); assert.equal(loadResponse.request().postDataJSON().slot,SLOT);
    await page.locator('#app').waitFor({state:'visible'}); await idle(page);
    const restored=await get(page,url,'/api/state');
    assert.notEqual(restored.session_id,bState.session_id); assert.equal(restored.player,'France'); assert.equal(restored.date,aState.date);
    assert(!fs.existsSync(primary),'Loading a backup must not silently overwrite the missing primary');
    assert.equal(fileHash(backup),a.record.sha256);
    mark('archive complete recovered campaign'); await campaigns(page); proof.recovered_save=await save(page,RECOVERED);
    const recoveredPath=inside(run,path.join(run,'saves',RECOVERED+'.json')), recovered=archive(recoveredPath);
    assert(recovered.normalized===a.normalized,'Backup recovery changed world/history/log/journey or other campaign fields');
    fs.writeFileSync(path.join(out,'archive-recovered.json'),recovered.bytes);
    proof.recovered_archive={...recovered.record,retained_path:path.join(out,'archive-recovered.json'),exact_except_saved_unix:true};
    mark('reject stale save from pre-load session');
    const beforeStale=await get(page,url,'/api/state'), recoveredHash=fileHash(recoveredPath);
    assert.equal(lost.payload.session_id,bState.session_id);
    const stale=await page.request.post(url+'/api/save',{data:lost.payload});
    proof.stale_save={payload:lost.payload,status:stale.status(),response:await stale.json()}; await stale.dispose();
    assert.equal(proof.stale_save.status,400); assert.equal(proof.stale_save.response.requires_review,true);
    assert(!fs.existsSync(primary),'Stale save recreated/replaced the missing primary');
    assert.equal(fileHash(backup),a.record.sha256); assert.equal(fileHash(recoveredPath),recoveredHash);
    assert.deepEqual(await get(page,url,'/api/state'),beforeStale,'Stale save changed live state');
    assert.deepEqual(proof.errors,[]); assert.equal(fileHash(binary),proof.binary_sha256);
    assert.equal(fileHash(__filename),proof.driver_sha256);
    assert.equal(fileHash(path.join(__dirname,'ci-integrated.cjs')),proof.build_verifier_sha256);
    proof.passed=true;
  } catch(error) {
    proof.failed_stage=stage; proof.failure=String(error.stack||error);
    if(page) {
      await page.screenshot({path:path.join(out,'failure.png')}).catch(()=>{});
      await page.content().then(html => fs.writeFileSync(path.join(out,'failure.html'),html)).catch(()=>{});
    }
    throw error;
  } finally {
    proof.finished_utc=new Date().toISOString(); write('result.json',proof);
    if(browser)await browser.close(); server.kill(); log.end();
  }
  console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
}
main().catch(error => {console.error(error);process.exitCode=1;});
