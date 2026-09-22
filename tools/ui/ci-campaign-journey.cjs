// S21 browser qualification on explicitly authored boundary scenarios. All
// gameplay actions use visible controls; saves capture exact read-only oracles.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright');
const root=path.resolve(__dirname,'../..'),hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const git=(...a)=>cp.execFileSync('git',a,{cwd:root,windowsHide:true}).toString().trim();
const post=(r,p)=>new URL(r.url()).pathname===p&&r.request().method()==='POST';
async function main(){
 const pin=process.env.SPHERES_EXPECTED_REVISION,binary=path.resolve(process.env.SPHERES_BINARY),fixtures=path.resolve(process.env.SPHERES_S21_FIXTURE_DIR),out=path.resolve(process.env.SPHERES_S21_BROWSER_OUTPUT);
 assert.equal(git('rev-parse','HEAD'),pin);assert.equal(git('status','--porcelain'),'');assert(!fs.existsSync(out));fs.mkdirSync(out,{recursive:true});
 const run=path.join(out,'server');fs.mkdirSync(path.join(run,'saves'),{recursive:true});
 for(const file of ['active','endpoint','succession'])fs.copyFileSync(path.join(fixtures,file+'.json'),path.join(run,'saves',file+'.json'));
 const sock=net.createServer();await new Promise(r=>sock.listen(0,'127.0.0.1',r));const port=sock.address().port;await new Promise(r=>sock.close(r));const url='http://127.0.0.1:'+port;
 const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']}),log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
 const proof={passed:false,revision:pin,binary_sha256:hash(binary),scope:'Authored France 1990/2035 and USSR dissolution scenarios; not a full 1990–2035 campaign.',commands:[],advances:[],errors:[],screenshots:[],checks:[]};
 let browser,page;
 const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(proof,null,2));
 try{
  const until=Date.now()+30000;for(;;){try{const r=await fetch(url+'/api/build');if(r.ok){proof.build=await r.json();break;}}catch{}assert(Date.now()<until);await new Promise(r=>setTimeout(r,100));}
  assert.equal(proof.build.revision,pin.slice(0,12));
  browser=await chromium.launch({channel:'chrome',headless:true});page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
  page.on('pageerror',e=>proof.errors.push(e.message));page.on('request',r=>{if(r.method()==='POST'&&new URL(r.url()).pathname==='/api/command')proof.commands.push(r.postDataJSON());if(r.method()==='POST'&&new URL(r.url()).pathname==='/api/advance')proof.advances.push(r.postDataJSON());});
  const idle=()=>page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance);
  async function state(){const r=await page.request.get(url+'/api/state');assert(r.ok());const s=await r.json();await r.dispose();return s;}
  async function capture(name){const s=await state();const r=await page.request.post(url+'/api/save',{data:{slot:name,session_id:s.session_id}});assert(r.ok());await r.dispose();return JSON.parse(fs.readFileSync(path.join(run,'saves',name+'.json'),'utf8'));}
  async function load(slot){
   if(await page.locator('#campaignJourney').isVisible())await page.locator('#journeyClose').click();
   if(await page.locator('#app').isVisible()){if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();await page.locator('#campaignsBtn').click();}
   if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();
   await page.locator('#saveSlots').selectOption(slot);const replacing=await page.evaluate(()=>!!SESSION.live?.player),response=page.waitForResponse(r=>post(r,'/api/load'));
   await page.locator('#loadBtn').click();if(replacing)await page.locator('#campaignConfirmAccept').click();const loaded=await response;assert(loaded.ok(),await loaded.text());await page.locator('#app').waitFor({state:'visible'});await idle();
  }
  async function open(){await page.locator('#campaignJourneyBtn').click();await page.waitForFunction(()=>document.querySelector('#journeyStatus').textContent===''&&document.querySelector('#journeyBody').childElementCount>0);}
  async function tab(name){await page.locator('#campaignJourney nav [data-journey-tab='+name+']').click();}
  async function shot(name){await page.screenshot({path:path.join(out,name+'.png')});proof.screenshots.push(name+'.png');assert(await page.locator('#campaignJourney').evaluate(e=>e.scrollWidth<=e.clientWidth+1),'no horizontal room overflow');}
  async function continuation(label){const response=page.waitForResponse(r=>post(r,'/api/command'));await page.getByRole('button',{name:label,exact:true}).click();await page.locator('#campaignConfirmAccept').click();const r=await response;assert(r.ok());const data=await r.json();assert.deepEqual(data.errors,[]);await idle();await page.waitForFunction(()=>document.querySelector('#journeyStatus').textContent==='');return data;}
  await page.goto(url,{waitUntil:'domcontentloaded'});await load('active');const before=await capture('before-review');await open();await shot('overview-desktop');
  await tab('goals');assert((await page.locator('#journeyBody').innerText()).includes('Shared prosperity'));await shot('goals-desktop');
  await page.setViewportSize({width:390,height:844});await shot('goals-390');await tab('history');await page.locator('#journeyCategory').selectOption('politics');await page.waitForFunction(()=>document.querySelector('#journeyStatus').textContent==='');await shot('history-390');
  await page.keyboard.press('Escape');await page.locator('#campaignJourney').waitFor({state:'hidden'});assert.equal(await page.evaluate(()=>document.activeElement.id),'campaignJourneyBtn');
  const after=await capture('after-review');for(const key of ['world','log','history','journey'])assert.deepEqual(after[key],before[key]);proof.checks.push('Overview, goals and filtered history inspection preserve world, archive and continuation metadata; Escape restores focus.');
  await page.setViewportSize({width:1440,height:1000});await load('endpoint');const advance=page.waitForResponse(r=>post(r,'/api/advance'));await page.locator('#stepBtn').click();assert((await advance).ok());await idle();let s=await state();assert.equal(s.year,2036);assert.equal(s.day,1);assert.equal(s.campaign_journey.status,'horizon');
  await open();await tab('overview');await shot('endpoint-desktop');const terminal=await capture('endpoint-before-choice');await continuation('Continue beyond 2035 · sandbox');const sandbox=await capture('sandbox-result');assert.deepEqual(terminal.world,sandbox.world);assert.equal(sandbox.journey.beyond_2035,true);proof.checks.push('31 December settles; sandbox continuation changes no simulation fields.');
  await load('sandbox-result');assert.equal((await state()).campaign_journey.beyond_2035,true);await page.locator('#stepBtn').click();await idle();assert.equal((await state()).day,2);
  await load('succession');const old=await capture('succession-before');await open();await tab('overview');await page.setViewportSize({width:390,height:844});await shot('succession-390');await continuation('Continue as Russia');assert.equal((await state()).player,'Russia');
  const next=await capture('successor-result');let expected=structuredClone(old.world),actual=next.world;while(expected.world){expected=expected.world;actual=actual.world;}expected.player='Russia';expected.player_set_rate=false;const endedGoal=expected.campaign_aims.active;expected.campaign_aims.active=null;expected.campaign_aims.history.push({goal:endedGoal,ended_day:(Date.UTC(expected.year,expected.month-1,expected.day)-Date.UTC(1990,0,1))/86400000,outcome:"government ended"});assert.deepEqual(actual,expected);assert.equal(next.journey.transitions.length,1);assert.equal(actual.campaign_aims.active,null);assert.equal(actual.campaign_aims.history.at(-1).goal.nation,'USSR');assert.equal(actual.campaign_aims.history.at(-1).outcome,'government ended');proof.checks.push('Russia retains exact existing simulation state except player control/manual-rate ownership and the explicitly verified former-country aim record.');
  await load('successor-result');assert.equal((await state()).player,'Russia');assert.equal((await state()).campaign_journey.transitions.length,1);await open();await tab('overview');await shot('successor-resumed-390');
  // Force a genuine network failure for the read endpoint; never replace data.
  await page.route('**/api/campaign-journey?*',r=>r.abort());await page.locator('#journeyClose').click();await page.locator('#campaignJourneyBtn').click();await page.getByRole('button',{name:'Retry loading record'}).waitFor();await page.unroute('**/api/campaign-journey?*');await page.getByRole('button',{name:'Retry loading record'}).click();await page.locator('.journey-hero h3').waitFor();proof.checks.push('Read failure has a working retry; no stale continuation control is exposed.');
  assert.deepEqual(proof.errors,[]);assert.equal(proof.commands.length,2);assert.equal(hash(binary),proof.binary_sha256);assert.equal(git('rev-parse','HEAD'),pin);assert.equal(git('status','--porcelain'),'');proof.passed=true;write();console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
 }catch(e){proof.failure=String(e.stack||e);write();if(page)await page.screenshot({path:path.join(out,'failure.png')}).catch(()=>{});throw e;}
 finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(e=>{console.error(e);process.exitCode=1;});
