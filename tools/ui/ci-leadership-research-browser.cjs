/* Read-only browser qualification. Serves repository references, never a game API. */
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),http=require('node:http'),crypto=require('node:crypto');
const {execFileSync}=require('node:child_process'),{chromium}=require('playwright');
const root=path.resolve(__dirname,'../..'),hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const git=(...args)=>execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,encoding:'utf8'}).trim();
const read=p=>JSON.parse(fs.readFileSync(path.join(root,p),'utf8'));
const index=read('docs/campaign-certification/C01/research-index.json');
const output=path.resolve(process.env.SPHERES_RESEARCH_REVIEW_OUTPUT||path.join(root,'../evidence/S10h-research-browser-development'));
assert(!fs.existsSync(output),'Use a new evidence directory');fs.mkdirSync(output,{recursive:true});
const revision=git('rev-parse','HEAD'),dirty=git('status','--porcelain');
if(process.env.SPHERES_EXPECTED_REVISION){assert.equal(revision,process.env.SPHERES_EXPECTED_REVISION);assert.equal(dirty,'');}
const proof={format:'spheres-s10h-research-browser/v1',revision,clean_before:!dirty,started_utc:new Date().toISOString(),scope:'Static research references only. No game server, campaign save, command or elapsed campaign time.',requests:[],errors:[],countries:[],screenshots:[],authored_error_cases:[],source_files:{}};
for(const p of ['tools/ui/leadership-research-review.html','tools/ui/leadership-research-review.css','tools/ui/leadership-research-review.js','tools/ui/ci-leadership-research-browser.cjs','docs/campaign-certification/C01/research-index.json',...index.source_files.map(s=>s.path)])proof.source_files[p]=hash(fs.readFileSync(path.join(root,p)));
const server=http.createServer((req,res)=>{
  try{const url=new URL(req.url,'http://localhost'),name=decodeURIComponent(url.pathname).replace(/^\/+/,''),file=path.resolve(root,name);
    const allowed=['tools/ui/','docs/campaign-certification/C01/','spheres-web/ui/','spheres-web/data/'].some(prefix=>name.startsWith(prefix));
    if(req.method!=='GET'||!allowed||!file.startsWith(root+path.sep)||!fs.existsSync(file)||!fs.statSync(file).isFile()){res.writeHead(404);return res.end();}
    const ext=path.extname(file),mime={'.html':'text/html; charset=utf-8','.js':'text/javascript; charset=utf-8','.css':'text/css; charset=utf-8','.json':'application/json; charset=utf-8','.png':'image/png'}[ext]||'application/octet-stream';
    res.writeHead(200,{'Content-Type':mime,'Cache-Control':'no-store'});res.end(fs.readFileSync(file));
  }catch{res.writeHead(400);res.end();}
});
let browser;
async function ready(page,nation){await page.waitForFunction(id=>document.querySelector('#atlas-country').value===id&&document.querySelector('#atlas-country-content').getAttribute('aria-busy')==='false',nation);assert.equal(await page.locator('[role="alert"]').count(),0);}
async function shot(page,name){await page.screenshot({path:path.join(output,name)});proof.screenshots.push(name);}
async function layout(page){
  const reading=await page.evaluate(()=>{const visible=n=>n.getClientRects().length&&getComputedStyle(n).visibility!=='hidden';const viewport=document.documentElement.clientWidth;return {width:innerWidth,scroll_width:document.documentElement.scrollWidth,viewport,overflow:Array.from(document.querySelectorAll('main *,header *,footer *')).filter(visible).filter(n=>{const r=n.getBoundingClientRect();return r.left< -1||r.right>viewport+1;}).map(n=>({tag:n.tagName,id:n.id,text:n.textContent.slice(0,120)}))};});
  assert(reading.scroll_width<=reading.viewport+1,JSON.stringify(reading));assert.deepEqual(reading.overflow,[]);return reading;
}
async function main(){
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const origin='http://127.0.0.1:'+server.address().port,url=origin+'/tools/ui/leadership-research-review.html';
  proof.origin=origin;browser=await chromium.launch({channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome',headless:true});
  const context=await browser.newContext({viewport:{width:1440,height:1000}}),page=await context.newPage();
  context.on('request',req=>{proof.requests.push({method:req.method(),url:req.url()});assert.equal(req.method(),'GET');assert(!new URL(req.url()).pathname.startsWith('/api/'));});
  page.on('pageerror',error=>proof.errors.push(error.message));
  await page.goto(url+'?country=France');await ready(page,'France');
  assert.equal(await page.locator('#atlas-country option').count(),160);assert.match(await page.locator('#atlas-status').textContent(),/0 exhaustive censuses/);
  assert.equal(await page.locator('.entry').count(),25);await page.locator('.entry>summary').first().click();await page.locator('.entry-body').first().waitFor();
  const firstId=await page.locator('.entry').first().getAttribute('data-research-id');await page.getByRole('button',{name:'Show 25 more observations'}).click();
  assert.equal(await page.locator('.entry').count(),50);assert.equal(await page.locator('.entry[open]').first().getAttribute('data-research-id'),firstId);
  proof.pagination={first_page:25,next_page:50,opened_evidence_preserved:true};
  for(const row of index.countries){
    await page.locator('#atlas-country').selectOption(row.nation);await ready(page,row.nation);
    assert.equal(await page.locator('.entry').count(),Math.min(25,row.organization_observations+row.institution_observations));
    const packet=read(row.packet),first=packet.organizations[0]||packet.institutions[0];
    await page.locator('.entry>summary').first().click();await page.locator('.entry-body').first().waitFor();
    assert.equal(await page.locator('.entry').first().getAttribute('data-research-id'),first.id);
    assert.match(await page.locator('.entry-body').first().textContent(),/Game identity mapping: unresolved|Reviewed catalogue references/);
    await page.locator('.entry .source>summary').first().click();
    const cid=first.claim_ids[0],claim=packet.sources.flatMap(s=>s.claims).find(c=>c.id===cid);
    assert((await page.locator('.entry-body').first().textContent()).includes(claim.text));
    await page.locator('#atlas-kind').selectOption('institutions');assert.equal(await page.locator('.entry').count(),Math.min(25,row.institution_observations));
    await page.locator('#atlas-kind').selectOption('roles');const roleEntries=[...packet.organizations,...packet.institutions].filter(e=>e.roles.length);
    assert.equal(await page.locator('.entry').count(),Math.min(25,roleEntries.length));
    if(roleEntries.length){await page.locator('.entry>summary').first().click();await page.locator('.entry-body').first().waitFor();assert((await page.locator('.entry-body').first().textContent()).includes(roleEntries[0].roles[0].title));}
    proof.countries.push({nation:row.nation,organization_observations:row.organization_observations,institution_observations:row.institution_observations,roles_filter:roleEntries.length,first_claim_exact:true});
  }
  await page.locator('#atlas-country').selectOption('Tonga');await ready(page,'Tonga');
  await page.locator('#atlas-search').fill("Tu'i'onetoa");assert((await page.locator('.entry').count())>0);
  const society=page.locator('.entry[data-research-id="to_peoples_party"]');await society.locator(':scope > summary').click();
  await society.locator('.role').first().waitFor();
  assert.equal(await society.locator('.role').count(),3);
  for(const name of ['President','Secretary']){
    const role=society.locator('.role').filter({has:page.locator('h4',{hasText:name+' ('})});
    assert.match(await role.textContent(),/Observed between 2022-04-19 and 2022-04-21; office term not established/);
  }
  assert.match(await society.textContent(),/Whether Leader and society President are one office is unresolved/);
  proof.society_observations={separate_offices:3,observation_period:['2022-04-19','2022-04-21'],terms_not_inferred:true,uncertainty_visible:true};
  for(const width of [1440,390,320]){
    await page.setViewportSize({width,height:1000});await society.locator('.role').nth(1).scrollIntoViewIfNeeded();await layout(page);await shot(page,`tonga-society-${width}.png`);
  }
  await page.setViewportSize({width:1440,height:1000});
  await page.locator('#atlas-search').fill('No Such Observation 123');assert.equal(await page.locator('.entry').count(),0);assert.match(await page.locator('#atlas-entries').textContent(),/No observations match/);
  await page.locator('#atlas-country').selectOption('UK');await ready(page,'UK');assert.equal(await page.locator('.entry').count(),0);assert.match(await page.locator('#atlas-results').textContent(),/No discovery packet yet/);assert(await page.locator('#atlas-search').isDisabled());
  const ukRows=read('docs/campaign-certification/C01/countries.json').find(c=>c.id==='UK').party_rows;
  assert(ukRows>0);assert.equal(await page.locator('#atlas-summary .fact strong').first().textContent(),String(ukRows));
  assert.equal(await page.locator('#atlas-summary .fact strong').nth(1).textContent(),'Unknown');
  proof.empty_country={nation:'UK',existing_game_party_rows:ukRows,party_count_matches_inventory:true,undiscovered_organization_count:'Unknown',no_stale_observations:true};
  await page.locator('#atlas-country').selectOption('Tonga');await ready(page,'Tonga');
  await page.locator('#atlas-kind').focus();await page.keyboard.press('Tab');await page.keyboard.press('Tab');
  const focus=await page.locator('.entry>summary').first().evaluate(n=>({focused:document.activeElement===n,offset:getComputedStyle(n).outlineOffset,width:getComputedStyle(n).outlineWidth}));
  assert.equal(focus.focused,true);assert.equal(focus.offset,'-4px');assert.equal(focus.width,'3px');
  await page.keyboard.press('Enter');await page.locator('.entry-body').first().waitFor();await shot(page,'atlas-keyboard-focus.png');
  proof.keyboard={summary_reached_with_tab:true,opened_with_enter:true,focus_outline:focus};await page.keyboard.press('Enter');
  proof.layouts=[];
  for(const width of [1440,390,320]){
    await page.setViewportSize({width,height:1000});await page.evaluate(()=>scrollTo(0,0));proof.layouts.push(await layout(page));await shot(page,`atlas-${width}.png`);
    await page.locator('#atlas-kind').selectOption('roles');const entry=page.locator('.entry').first();await entry.locator(':scope > summary').click();await entry.locator('.entry-body').waitFor();
    await entry.locator('.source>summary').first().click();await entry.locator('.role').first().scrollIntoViewIfNeeded();proof.layouts.push(await layout(page));await shot(page,`atlas-role-${width}.png`);
    await page.locator('#atlas-kind').selectOption('all');
  }
  // Deliberately tampered response: source-integrity refusal is expected and
  // labeled separately from the real-content journey.
  const corrupt=await context.newPage();corrupt.on('pageerror',error=>proof.errors.push(error.message));
  await corrupt.route('**/research/tonga.json',async route=>{const res=await route.fetch();await route.fulfill({response:res,body:(await res.text())+' '});});
  await corrupt.goto(url+'?country=Tonga');await corrupt.getByRole('alert').waitFor();assert.match(await corrupt.getByRole('alert').textContent(),/Source files changed/);assert.equal(await corrupt.locator('.entry').count(),0);assert(await corrupt.locator('#atlas-search').isDisabled());
  proof.authored_error_cases.push({kind:'tampered_packet_bytes',rejected:true,old_data_absent:true});await corrupt.close();
  const race=await context.newPage();race.on('pageerror',error=>proof.errors.push(error.message));let delayed;
  await race.goto(url+'?country=Tonga');await ready(race,'Tonga');
  const gate=new Promise(resolve=>delayed=resolve);await race.route('**/research/france.json',async route=>{await gate;try{await route.continue();}catch{}});
  await race.locator('#atlas-country').selectOption('France');await race.locator('#atlas-country').selectOption('Japan');await ready(race,'Japan');delayed();
  await race.locator('#atlas-kind').selectOption('institutions');assert.match(await race.locator('#atlas-summary h2').textContent(),/^Japan$/);assert.equal(await race.locator('.entry').count(),index.countries.find(c=>c.nation==='Japan').institution_observations);
  proof.authored_error_cases.push({kind:'superseded_country_request',latest_country:'Japan',old_data_absent:true});await race.close();
  await page.reload();await ready(page,'Tonga');proof.deep_link_reload={country:'Tonga',preserved:true};
  assert.deepEqual(proof.errors,[]);await context.close();
  proof.revision_after=git('rev-parse','HEAD');proof.clean_after=!git('status','--porcelain');assert.equal(proof.revision_after,revision);
  for(const [p,expected] of Object.entries(proof.source_files))assert.equal(hash(fs.readFileSync(path.join(root,p))),expected,'Source changed during review: '+p);
  if(process.env.SPHERES_EXPECTED_REVISION)assert(proof.clean_after);
  proof.source_files_unchanged=true;proof.passed=true;
}
main().catch(error=>{proof.passed=false;proof.failure=error.stack;process.exitCode=1;}).finally(async()=>{
  if(browser)await browser.close();await new Promise(resolve=>server.close(resolve));proof.finished_utc=new Date().toISOString();
  fs.writeFileSync(path.join(output,'result.json'),JSON.stringify(proof,null,2)+'\n');console.log(JSON.stringify({passed:proof.passed,output,countries:proof.countries.length,failure:proof.failure}));
});
