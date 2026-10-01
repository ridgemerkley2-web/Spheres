// Real embedded selector check on a disposable native server; never starts a campaign.
// NODE_PATH=<Playwright modules> SPHERES_BROWSER_CHANNEL=chrome node this-file BINARY NEW_OUTPUT
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const net=require('node:net'),cp=require('node:child_process'),crypto=require('node:crypto');
const {chromium}=require('playwright');
const root=path.resolve(__dirname,'../..');
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
async function freePort(){
  const server=net.createServer();await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const port=server.address().port;await new Promise(resolve=>server.close(resolve));return port;
}
async function main(){
  const [binaryArg,outputArg]=process.argv.slice(2);assert(binaryArg&&outputArg,'Supply binary and a new evidence directory');
  const binary=path.resolve(binaryArg),out=path.resolve(outputArg);
  assert(!fs.existsSync(out),'Keep previous evidence: output must be new');
  const run=path.join(out,'disposable-campaign');fs.mkdirSync(run,{recursive:true});
  const port=await freePort();assert.notEqual(port,7777);const origin='http://127.0.0.1:'+port;
  const proof={format:'spheres-campaign-leader-selector-browser/v1',passed:false,
    scope:'Working-tree UI and native API integration; opening selector only. Disposable server, no campaign creation, orders, saves or time advancement. Not campaign qualification.',
    started_utc:new Date().toISOString(),binary_sha256:hash(fs.readFileSync(binary)),origin,cases:[],screenshots:[],errors:[],requests:[]};
  const server=cp.spawn(binary,['--no-open','--port',String(port)],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page;
  async function get(route){const response=await fetch(origin+route,{signal:AbortSignal.timeout(10000)});assert(response.ok,route+' '+response.status);return Buffer.from(await response.arrayBuffer());}
  async function shot(name){
    const filename=name+'.jpg';await page.screenshot({path:path.join(out,filename),type:'jpeg',quality:85});
    proof.screenshots.push({file:filename,sha256:hash(fs.readFileSync(path.join(out,filename))),viewport:page.viewportSize()});
  }
  try{
    const deadline=Date.now()+30000;
    for(;;){
      try{proof.build=JSON.parse(await get('/api/build'));break;}
      catch(error){if(Date.now()>=deadline||server.exitCode!=null)throw error;await new Promise(resolve=>setTimeout(resolve,100));}
    }
    assert.equal(path.resolve(proof.build.save_directory),run,'Only the new disposable save directory may be used');
    const index=fs.readFileSync(path.join(root,'spheres-web/ui/index.html')),served=await get('/');
    assert(index.equals(served),'Embedded index must be the exact current UI bytes');
    proof.index={path:'spheres-web/ui/index.html',bytes:index.length,sha256:hash(index)};
    const rosterBytes=await get('/api/roster'),roster=JSON.parse(rosterBytes),beforeState=await get('/api/state');
    assert.equal(roster.nations.length,137,'Only nations seated in the opening 1990 world belong in the roster');proof.roster_sha256=hash(rosterBytes);
    proof.date={year:roster.year,month:roster.month,day:roster.day};
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});
    page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(15000);
    page.on('pageerror',error=>proof.errors.push(error.message));
    page.on('request',request=>proof.requests.push({method:request.method(),url:request.url()}));
    page.on('response',response=>{if(response.status()>=400)proof.errors.push(response.status()+' '+response.url());});
    await page.goto(origin,{waitUntil:'domcontentloaded'});
    await page.locator('#newCampaignBtn').click();
    await page.waitForFunction(count=>setupNations.length===count,roster.nations.length);
    const people=[['France','francois_mitterrand'],['Brazil','jose_sarney'],['India','v_p_singh'],['SaudiArabia','fahd_bin_abdulaziz_al_saud'],['USSR','mikhail_gorbachev']];
    for(const [nationId,personId] of people){
      const nation=roster.nations.find(n=>n.id===nationId),leader=nation.campaign_leader;
      assert.equal(leader.person_id,personId);assert.equal(leader.date,'1990-01-01');
      assert.equal(leader.identity_status,'linked_person');assert.match(leader.portrait.url,/^\/art\/people\//);
      await page.locator('#nationSearch').fill(nation.name);
      const card=page.locator('#nationPick .opt').filter({has:page.locator('.nm',{hasText:nation.name})});
      assert.equal(await card.count(),1);await card.click();
      await page.waitForFunction(url=>{const image=document.querySelector('#showcasePortrait');return image.getAttribute('src')===url&&image.complete&&image.naturalWidth>0&&image.classList.contains('ready');},leader.portrait.url);
      assert.equal(await page.locator('#showcaseFigure strong').textContent(),leader.name);
      assert.match(await page.locator('#showcaseLeaderDate').textContent(),/1990-01-01/);
      assert.equal(await card.locator('.figure-name').textContent(),leader.name);
      const asset=path.join(root,'spheres-web/ui/person-portraits',path.basename(leader.portrait.url));
      const image=await get(leader.portrait.url);assert(image.equals(fs.readFileSync(asset)),'Served art bytes must match the reviewed local portrait');
      const size=await page.locator('#showcasePortrait').evaluate(img=>[img.naturalWidth,img.naturalHeight]);
      assert(size[0]>0&&size[1]>0);await shot('selector-'+nationId.toLowerCase());
      proof.cases.push({nation:nationId,person_id:personId,name:leader.name,date:leader.date,portrait:leader.portrait.url,asset_sha256:hash(image),native_size:size});
    }
    const fallback=roster.nations.find(n=>n.id==='Afghanistan'&&n.campaign_leader.name&&!n.campaign_leader.portrait)
      ||roster.nations.find(n=>n.campaign_leader.name&&!n.campaign_leader.portrait);
    assert(fallback,'Exercise an actual named officeholder without reviewed art');
    await page.locator('#nationSearch').fill(fallback.name);
    await page.locator('#nationPick .opt').filter({has:page.locator('.nm',{hasText:fallback.name})}).click();
    assert.equal(await page.locator('#showcaseFigure strong').textContent(),fallback.campaign_leader.name);
    assert.equal(await page.locator('#showcasePortrait').getAttribute('src'),null,'Previous nation art is cleared');
    assert(!await page.locator('#showcasePortrait').evaluate(img=>img.classList.contains('ready')));
    assert.match(await page.locator('#showcaseLeaderDate').textContent(),/Portrait pending/);
    assert((await page.locator('#showcaseInitials').textContent()).trim());
    await shot('selector-missing-art');
    proof.cases.push({nation:fallback.id,name:fallback.campaign_leader.name,date:fallback.campaign_leader.date,portrait:null,missing_art_fallback:true});
    assert.deepEqual(proof.requests.filter(r=>r.method!=='GET'),[],'Selector browsing sends no orders or save requests');
    assert.deepEqual(proof.requests.filter(r=>/nation-figures|\/art\/portraits\//.test(r.url)),[],'No legacy selector metadata or art is requested');
    assert.deepEqual(proof.errors,[]);
    assert(beforeState.equals(await get('/api/state')),'Selector browsing leaves native state bytes unchanged');
    assert.deepEqual(fs.readdirSync(run),[],'No campaign files were created');
    assert.equal(hash(fs.readFileSync(binary)),proof.binary_sha256);
    proof.state_unchanged_sha256=hash(beforeState);proof.passed=true;
  }catch(error){
    proof.failure=String(error.stack||error);if(page)await shot('failure').catch(()=>{});throw error;
  }finally{
    proof.finished_utc=new Date().toISOString();fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(proof,null,2)+'\n');
    if(browser)await browser.close();server.kill();log.end();
  }
  console.log(JSON.stringify({passed:true,cases:proof.cases.length,result:path.join(out,'result.json')}));
}
main().catch(error=>{console.error(error);process.exitCode=1;});
