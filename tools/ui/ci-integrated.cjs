// Helpers for the disposable binary/browser CI lane. No mocked campaign data.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),cp=require('node:child_process');
const hash=value=>crypto.createHash('sha256').update(value).digest('hex');
const ASSETS=['index.html','campaign-transport.js','campaign-ui.js','fiscal-recovery-ui.js','cash-flow-ui.js','companies-ui.js','companies.css','government-ui.js','government-ui.css','campaign-operations-ui.js','campaign-operations-ui.css','map-controls.js','map-controls.css','guidance-ui.js','guidance-ui.css','globe3d.js','city-layer.js','city-mesh.js','water-detail.js','equipment-mesh.js','equipment-ui.js','equipment-ui.css'];
function git(root,args){const r=cp.spawnSync('git',args,{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});assert.equal(r.status,0,'Cannot verify committed source: '+r.stderr);return r.stdout;}
async function json(page,url,route){const response=await page.request.get(url+route);assert(response.ok(),route+' returned '+response.status());return response.json();}
async function verifyBuild({page,url,root,run,binary}){
  const revision=process.env.SPHERES_EXPECTED_REVISION||git(root,['rev-parse','HEAD']).toString().trim();
  assert.match(revision,/^[a-f0-9]{40}$/,'Expected revision must be an exact commit ID');
  const short=git(root,['rev-parse','--short=12',revision]).toString().trim(),build=await json(page,url,'/api/build');
  assert.equal(build.revision,short,'The running binary must be compiled from the clean requested commit');
  assert.equal(path.resolve(build.save_directory),path.resolve(run),'The server must use the disposable save directory');
  assert(Number.isSafeInteger(build.built_at_unix_seconds)&&build.built_at_unix_seconds>0);
  const assets={};
  for(const name of ASSETS){
    const expected=git(root,['show',revision+':spheres-web/ui/'+name]);
    const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name)),text=checkout.toString('utf8');
    assert(Buffer.from(text,'utf8').equals(checkout),'Checkout '+name+' must be valid UTF-8');
    // Git stores these text assets with LF, while Windows checkout/build input
    // can contain CRLF. Only that pair is normalized for source provenance.
    const canonical=Buffer.from(text.replace(/\r\n/g,'\n'),'utf8');
    assert(canonical.equals(expected),'Checkout '+name+' differs from the exact committed source after CRLF-to-LF normalization');
    const response=await page.request.get(url+(name==='index.html'?'/':'/'+name));assert(response.ok(),'Missing embedded '+name);
    const actual=await response.body();assert(actual.equals(checkout),'Embedded '+name+' differs from the actual checkout/build input bytes');
    assets[name]={served_sha256:hash(actual),served_bytes:actual.length,checkout_sha256:hash(checkout),checkout_bytes:checkout.length,
      committed_sha256:hash(expected),committed_bytes:expected.length,canonical_checkout_sha256:hash(canonical),
      newline_normalization:{rule:'CRLF to LF only for checkout-to-commit comparison; served-to-checkout comparison remains byte-exact',crlf_pairs:(text.match(/\r\n/g)||[]).length}};
  }
  return {revision,build,binary_sha256:hash(fs.readFileSync(binary)),assets};
}
async function capabilities({page,url,player}){
  const state=await json(page,url,'/api/state'),companies=await json(page,url,'/api/companies?session_id='+encodeURIComponent(state.session_id));
  assert.equal(state.player,player);assert.equal(state.simulation_cadence,'daily');
  assert.equal(state.connected_economy?.enabled,true,'Connected economy must be retained');
  assert.equal(state.population_enabled,true);assert.equal(state.fiscal_recovery_enabled,true);
  assert.equal(state.warfare_adoption?.enabled,true,'Operational warfare must be retained');
  assert.equal(companies.enabled,true);assert.equal(companies.operations?.enabled,true,'Supplier operations must be retained');
  assert.equal(companies.nation,player);assert.equal(companies.session_id,state.session_id);
  return {daily:true,population:true,fiscal:true,connected_economy:true,companies:true,supplier_operations:true,operational_warfare:true};
}
async function withinPanel(page,selector,label){
  const box=await page.locator(selector).evaluate(element=>({scroll:element.scrollWidth,width:element.clientWidth}));
  assert(box.width>0&&box.scroll<=box.width+1,label+' overflows horizontally: '+JSON.stringify(box));
  assert(!/\bNaN\b|\bundefined\b/.test(await page.locator(selector).innerText()),label+' contains an invalid numeric or missing-value label');
}
async function archive(page,url,run,slot){
  const response=await page.request.post(url+'/api/save',{data:{slot}});assert(response.ok(),'Could not capture '+slot);
  const value=JSON.parse(fs.readFileSync(path.join(run,'saves',slot+'.json'),'utf8'));
  delete value.saved_unix;return value;
}
async function campaignGuidance(page){
  const nav=page.locator('.decision-nav');
  for(const name of ['Advisors','Tutorial']){
    const button=nav.getByRole('button',{name,exact:true});
    assert(await button.isVisible()&&await button.isEnabled(),name+' must remain an accessible campaign entry point');
  }
  assert(await page.locator('#guidanceLauncher').isHidden(),'Campaign navigation replaces the redundant floating launcher');
  const adviceResponse=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/guidance'&&response.request().method()==='GET');
  await nav.getByRole('button',{name:'Advisors',exact:true}).click();
  await page.locator('#guidanceDialog').getByRole('heading',{name:'What needs your attention?',exact:true}).waitFor();
  assert((await adviceResponse).ok(),'Standard Advisors must load its current native campaign reading');
  await page.locator('#guidanceDialog .guidance-cards[aria-busy="false"]').waitFor();
  await page.getByRole('button',{name:'Close tutorial and advisors',exact:true}).click();
  await page.keyboard.press('F1');
  await page.locator('#guidanceDialog').getByRole('heading',{name:'Your first steps in Spheres',exact:true}).waitFor();
  await page.getByRole('button',{name:'Close tutorial and advisors',exact:true}).click();
  await nav.getByRole('button',{name:'Tutorial',exact:true}).click();
  await page.locator('#guidanceDialog').getByRole('heading',{name:'Your first steps in Spheres',exact:true}).waitFor();
  await page.getByRole('button',{name:'Close tutorial and advisors',exact:true}).click();
  // Reproduce the actual obstructed city action using its ordinary UI path.
  await nav.getByRole('button',{name:'Find',exact:true}).click();
  await page.locator('#worldFindInput').fill('Paris');
  await page.locator('[data-find-kind="city"]').filter({has:page.getByText('Paris',{exact:true})}).click();
  await page.locator('#mapCityCard').waitFor({state:'visible'});
  await page.getByRole('button',{name:'Close city',exact:true}).click();
  assert(await page.locator('#mapCityCard').isHidden(),'Natural Close city click must close the city card');
  assert(await page.locator('#guidanceDialog').isHidden(),'Close city must not open Guidance');
  return {standard_advisors:true,standard_tutorial:true,f1:true,floating_launcher:false,natural_city_close:true};
}
async function panels({page,url,run,out,player}){
  await page.evaluate(()=>clockPause());
  const before=await archive(page,url,run,'s05-preview-before');
  assert.equal(before.world.format,'spheres-integrated-save','A fresh integrated campaign must preserve the combined save envelope');
  const current=await json(page,url,'/api/state');assert.equal(current.player,player);
  const companyPath='/api/companies?session_id='+encodeURIComponent(current.session_id);
  const government=await json(page,url,'/api/government?nation='+encodeURIComponent(player));
  const companies=await json(page,url,companyPath),production=await json(page,url,'/api/production');
  assert.equal(companies.session_id,current.session_id);assert.equal(companies.nation,player);
  assert.equal(government.nation,player);assert.equal(government.mine,true);
  assert.equal(production.nation,player);assert(Array.isArray(production.catalog));
  const expected=companies.directory.map(row=>row.reference).sort();assert(expected.length>0,'Fresh game must contain the actual enrolled service directory');
  const eligible=production.catalog.find(row=>row.kind!=='starter_industry'&&row.actions?.start!==false&&row.eligible_provinces?.length);
  assert(eligible,'The technical USA fixture needs an eligible native construction preview');
  const kind=eligible.kind,province=eligible.eligible_provinces[0];
  const governmentAction=government.actions.findIndex(action=>action.command&&!action.refusal);
  assert(governmentAction>=0,'The native technical fixture must expose a reviewable government decision');
  const sizes=[{width:1440,height:1000,name:'desktop'},{width:390,height:844,name:'mobile'}],views=[];
  for(const size of sizes){
    await page.setViewportSize({width:size.width,height:size.height});
    await page.locator('#govBtn').click();await page.locator('#govScreen .gov-ui').waitFor({state:'visible'});
    assert((await page.locator('#govScreen .gov-ui h1').innerText()).includes(government.nation_name));
    await withinPanel(page,'#govScreen .gov-ui','Government '+size.name);
    await page.screenshot({path:path.join(out,'government-'+size.name+'.png')});
    await page.locator('#gov-tab-decisions').click();
    const governmentResponse=page.waitForResponse(response=>response.url()===url+'/api/government/preview'&&response.request().method()==='POST');
    await page.locator('#govScreen [data-gov-review="'+governmentAction+'"]').click();
    const governmentReading=await governmentResponse;assert(governmentReading.ok());const governmentPreview=await governmentReading.json();
    await page.locator('#govReviewTitle').waitFor({state:'visible'});await page.waitForFunction(()=>!!gov.review&&!gov.review.loading);
    assert.equal(governmentPreview.valid,true);assert.deepEqual(governmentPreview.command,government.actions[governmentAction].command);
    assert.equal(await page.locator('#govReview [data-gov-confirm]').isEnabled(),true);
    await withinPanel(page,'#govReview','Government decision review '+size.name);
    await page.screenshot({path:path.join(out,'government-review-'+size.name+'.png')});
    await page.locator('#govReview [aria-label="Close decision review"]').click();await page.locator('#govReview').waitFor({state:'hidden'});
    await page.locator('#govScreen .tbar .x').click();

    const guidanceEntries=await campaignGuidance(page);
    await page.locator('#productionDockBtn').click();await page.locator('#productionPanel [data-prod-new]').waitFor({state:'visible'});
    await page.locator('#productionPanel [data-prod-new]').click();
    await page.locator('[data-prod-kind="'+kind+'"]').click();
    const previewResponse=page.waitForResponse(r=>r.url()===url+'/api/construction-preview'&&r.request().method()==='POST');
    await page.locator('[data-prod-province="'+province+'"]').click();
    const response=await previewResponse;assert(response.ok());const preview=await response.json();
    await page.locator('#constructionPreviewTitle').waitFor({state:'visible'});
    await page.waitForFunction(()=>PROD.previewLoading===false&&!!PROD.preview);
    assert.equal(preview.project_kind,kind);assert.equal(preview.district,province);assert(Number.isFinite(preview.cost_bn));
    assert.equal(await page.locator('[data-construction-confirm]').isEnabled(),preview.can_start===true);
    assert.equal(await page.locator('.construction-impact-scope').count(),2,'Province and country effects must be visible before purchase');
    await withinPanel(page,'#productionBody','Construction preview '+size.name);
    await page.screenshot({path:path.join(out,'construction-preview-'+size.name+'.png')});
    await page.locator('#productionClose').click();

    await page.locator('[data-drawer="cabinetDrawer"]').click();await page.locator('#cab-tab-companies').click();
    await page.locator('#company-search').waitFor({state:'visible'});await page.waitForFunction(()=>!CDESK.loading&&!!CDESK.data);
    const references=async()=> (await page.locator('[data-company-record]').evaluateAll(rows=>rows.map(row=>row.dataset.companyRecord))).sort();
    assert.deepEqual(await references(),expected,'Directory must show the native qualified company IDs');
    await page.locator('[data-company-role="contractor"]').click();
    assert.deepEqual(await references(),companies.directory.filter(row=>row.kind==='contractor').map(row=>row.reference).sort());
    await page.locator('[data-company-sector="construction"]').click();
    assert.deepEqual(await references(),companies.directory.filter(row=>row.kind==='contractor'&&row.sector==='construction').map(row=>row.reference).sort());
    await page.locator('#company-search').fill('__s05_no_company_exists__');assert.deepEqual(await references(),[]);
    await page.locator('[data-company-reset]').click();assert.deepEqual(await references(),expected);
    await withinPanel(page,'#cabinet-companies','Companies '+size.name);
    if(size.width===390)assert((await page.locator('#company-search').boundingBox()).width>=200,'Mobile search must retain usable width');
    await page.screenshot({path:path.join(out,'companies-'+size.name+'.png')});
    await page.locator('#cabinetDrawer [data-close-drawers]').click();
    views.push({viewport:size.name,width:size.width,height:size.height,guidance_entries:guidanceEntries,government:true,government_review_cancel:{action:governmentAction,price_pc:governmentPreview.price_pc},construction_preview:{kind,province,cost_bn:preview.cost_bn},company_rows:expected.length,filtering:true});
  }
  await page.setViewportSize({width:1440,height:1000});
  const draftsBefore=await page.evaluate(()=>JSON.parse(JSON.stringify({queued,pendingAdvance,command:COMMAND_CHANNEL.pending})));
  await page.getByRole('button',{name:'Tutorial',exact:true}).click();await page.locator('#guidanceDialog').waitFor({state:'visible'});
  await page.locator('[data-guidance-lesson="budget-treasury"]').click();await page.locator('[data-guidance-open-lesson]').click();
  await page.locator('#cabinet-budget').waitFor({state:'visible'});assert(await page.locator('#guidanceDialog').isHidden());
  assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify({queued,pendingAdvance,command:COMMAND_CHANNEL.pending}))),draftsBefore,'Tutorial navigation must not queue orders or start a turn');
  await page.screenshot({path:path.join(out,'tutorial-budget-destination.png')});await page.locator('#cabinetDrawer [data-close-drawers]').click();
  const after=await archive(page,url,run,'s05-preview-after');
  assert.deepEqual(after,before,'Government reads, construction previews and company filtering must preserve the entire saved campaign and history');
  assert.deepEqual(await json(page,url,'/api/production'),production);assert.deepEqual(await json(page,url,companyPath),companies);
  await page.setViewportSize({width:1440,height:1000});
  return {technical_fixture:'USA fresh campaign; a UI/API regression fixture, not a campaign certification focus country',views,tutorial_destination:'budget',tutorial_orders_unchanged:true,archive_purity:true,capabilities:await capabilities({page,url,player})};
}
module.exports={verifyBuild,capabilities,panels};
