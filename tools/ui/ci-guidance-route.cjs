// S19: outcome-aware first-hour guidance against a real, fresh France campaign.
// Requires an already-built release binary compiled from the exact committed
// source. Every game action below uses an ordinary visible control (or F1/Enter/
// Escape on the focused control) and each one is logged with the selector that
// was actually clicked or focused. page.request is used only for GET reads and
// /api/save archive snapshots (observation): it bypasses the page's api(), so it
// can never create a guidance save receipt, and every such call is logged.
// Real /api/guidance traffic is only ever delayed (response held, or request
// held), never altered. No fixture saves, endowments or synthetic bodies.
'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const net=require('node:net'),cp=require('node:child_process'),crypto=require('node:crypto');
const {chromium}=require('playwright');
const integrated=require('./ci-integrated.cjs');
const root=path.resolve(__dirname,'../..');
const hash=value=>crypto.createHash('sha256').update(value).digest('hex');
const fileHash=file=>new Promise((resolve,reject)=>{const h=crypto.createHash('sha256');
  fs.createReadStream(file).on('error',reject).on('data',chunk=>h.update(chunk)).on('end',()=>resolve(h.digest('hex')));});
const q=value=>JSON.stringify(String(value));
const isRoute=(response,route,method='POST')=>new URL(response.url()).pathname===route&&response.request().method()===method;
const iso=day=>new Date(Date.UTC(1990,0,1)+day*86400000).toISOString().slice(0,10);
const STEP_IDS=['finances','construction','research_design','procurement','air_force','save_resume'];
const STATUS_LABEL={achieved:'Achieved in this campaign',progress:'In progress',not_yet:'Not yet',unknown:'Unknown'};
const MILESTONE_LABEL={done:'Done',pending:'Not yet',unknown:'Unknown'};
const GUIDANCE_ASSETS=['tutorial-model.js','advisor-model.js','guidance-ui.js','guidance-ui.css'];
const RECEIPTS_KEY='spheres.guidance.receipts.v1',PROGRESS_KEY='spheres.guidance.tutorial.v1';
const BUDGET_KINDS=['program_budget','annual_budget','construction_budget'];
// money_view.rs decision labels for the budget kinds (annual_budget falls to the generic label).
const DECISION_LABEL={program_budget:'Department funding reviewed',construction_budget:'Construction funding reviewed',annual_budget:'Government funding reviewed'};
const RAW_DISTRICT=/\b[A-Z]{3}_[A-Za-z0-9-]+/,PROVINCE='Île-de-France';
const IDENTITY_NOTICE='The live campaign has moved ahead or changed. Open Campaigns and Continue to read its current state.';
const STALE_LINE='These results are from an earlier reading of this campaign. Refresh before following a step.';
const SLOT='s19-first-hour';
const frames=p=>p.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));

async function freePort(){
  for(;;){
    const socket=net.createServer();await new Promise((resolve,reject)=>{socket.once('error',reject);socket.listen(0,'127.0.0.1',resolve);});
    const port=socket.address().port;await new Promise(resolve=>socket.close(resolve));
    if(port!==7777)return port; // 7777 is another tool's customary server port.
  }
}
async function idle(p){
  await p.waitForFunction(()=>!advancing&&!pendingAdvance&&!SESSION.busy&&!CAB.busy&&!PROD.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!EQUIP.busy);
}
async function verifyGuidanceAssets(page,url,revision){
  const out={};
  for(const name of GUIDANCE_ASSETS){
    const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name));
    const committed=cp.spawnSync('git',['show',revision+':spheres-web/ui/'+name],{cwd:root,windowsHide:true,maxBuffer:16*1024*1024});
    assert.equal(committed.status,0,'Cannot read committed '+name);
    const text=checkout.toString('utf8');assert(Buffer.from(text,'utf8').equals(checkout),name+' checkout must be UTF-8');
    const canonical=Buffer.from(text.replace(/\r\n/g,'\n'),'utf8');
    assert(canonical.equals(committed.stdout),name+' checkout differs from the committed source after CRLF-to-LF normalization');
    const response=await page.request.get(url+'/'+name);
    let served;try{assert(response.ok(),'Missing served '+name);served=await response.body();}finally{await response.dispose();}
    assert(served.equals(checkout),'Served '+name+' differs from the compiled checkout bytes');
    out[name]={served_sha256:hash(served),served_bytes:served.length,checkout_sha256:hash(checkout),checkout_bytes:checkout.length,
      committed_sha256:hash(committed.stdout),committed_bytes:committed.stdout.length,crlf_pairs:(text.match(/\r\n/g)||[]).length,
      newline_normalization:'CRLF to LF only for checkout-to-commit comparison; served-to-checkout comparison is byte-exact'};
  }
  return out;
}
// Every text file the server embeds with include_str!("../ui/..."): the checkout must equal the commit, and
// wherever the server answers the same root-relative path the served bytes must equal the checkout.
async function verifyEmbeddedText(page,url,revision,index){
  const names=new Set();
  for(const file of fs.readdirSync(path.join(root,'spheres-web/src')).filter(f=>f.endsWith('.rs'))){
    for(const m of fs.readFileSync(path.join(root,'spheres-web/src',file),'utf8').matchAll(/include_str!\("\.\.\/ui\/([^"]+)"\)/g))names.add(m[1]);
  }
  const served=[],other=[];
  for(const name of [...names].sort()){
    const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name));
    const committed=cp.spawnSync('git',['show',revision+':spheres-web/ui/'+name],{cwd:root,windowsHide:true,maxBuffer:64*1024*1024});
    assert.equal(committed.status,0,'Cannot read committed '+name);
    assert(Buffer.from(checkout.toString('utf8').replace(/\r\n/g,'\n'),'utf8').equals(committed.stdout),name+' checkout differs from the commit after CRLF-to-LF normalization');
    const response=await page.request.get(url+(name==='index.html'?'/':'/'+name));
    try{
      const body=response.ok()?await response.body():null;
      if(body&&body.equals(checkout))served.push({name,sha256:hash(body),bytes:body.length});
      else{assert(!body||name!=='index.html'&&body.equals(index),'Served '+name+' differs from the checkout bytes');other.push({name,status:response.status(),index_fallback:!!body});}
    }finally{await response.dispose();}
  }
  return {embedded_text_files:names.size,served_byte_equal:served.length,not_served_at_root_path:other,files:served};
}

(async()=>{
  assert(process.env.SPHERES_BINARY,'Set SPHERES_BINARY to the already-built release executable');
  const expected=process.env.SPHERES_EXPECTED_REVISION||'';
  assert.match(expected,/^[a-f0-9]{40}$/,'Set SPHERES_EXPECTED_REVISION to its exact committed source');
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,encoding:'utf8',windowsHide:true,maxBuffer:32*1024*1024});
  const head=git(['rev-parse','HEAD']).trim();
  assert.equal(git(['status','--porcelain','--','spheres-web','spheres-sim','spheres-cli','Cargo.toml','Cargo.lock']).trim(),'','Runtime source must be the clean committed checkout');
  assert.equal(git(['diff','--name-only',expected,head,'--','spheres-web','spheres-sim','spheres-cli','Cargo.toml','Cargo.lock']).trim(),'','HEAD must carry the exact runtime source of the expected revision');
  const binary=path.resolve(process.env.SPHERES_BINARY);assert(fs.statSync(binary).isFile());
  const binaryBefore=await fileHash(binary);
  const output=path.resolve(process.env.SPHERES_GUIDANCE_OUTPUT||path.join(root,'artifacts/browser-guidance-ci'));
  fs.mkdirSync(output,{recursive:true});
  const out=fs.mkdtempSync(path.join(output,'france-')),run=fs.mkdtempSync(path.join(out,'server-'));
  const port=await freePort(),url=`http://127.0.0.1:${port}`;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let launchError,browser,page,second=null,stage='launch',guidanceWindow=null;server.on('error',error=>{launchError=error;});
  const evidence={passed:false,out,run,url,port,started_utc:new Date().toISOString(),
    fixture:'Fresh France chosen through #newCampaignBtn, the nation picker and #startBtn in a fresh browser profile and a fresh server save directory. '
      +'No fixture saves, seeded money, granted research, synthetic responses or hidden state mutation. page.request is used only for GET reads and '
      +'/api/save archive snapshots (observation outside the page api(), so no guidance receipt can come from them); every such call is listed in observer_requests. '
      +'page.route delays real /api/guidance traffic in the stale stages (a held response in 8 and 8b, a held request in 8c) and never changes a body. '
      +'Stages 8 and 8b install a read-only PerformanceObserver and MutationObserver in the page (window.__s19Race, removed afterwards) to see the held response arrive and prove no repaint followed.',
    source:{head,expected_revision:expected,driver:path.relative(root,__filename).replace(/\\/g,'/'),driver_sha256:hash(fs.readFileSync(__filename)),
      driver_committed:false,runtime_source_clean:true},
    browser_channel:process.env.SPHERES_BROWSER_CHANNEL||'playwright-default',
    stages:[],actions:[],requests:[],guidance_requests:[],guidance_reads:[],observer_requests:[],archive_saves:[],screenshots:[],
    errors:[],console_errors:[],http_errors:[],checks:[]};
  const check=text=>{evidence.checks.push(text);progress('check',{text});};
  const write=(name,value)=>fs.writeFileSync(path.join(out,name),JSON.stringify(value,null,2)+'\n');
  const progress=(event,details={})=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');
  const mark=value=>{stage=value;progress('stage');console.log('[s19] '+value);};
  // Guidance scrolls inside its dialog, so a capture aligns its subject to the top of the visible scroller.
  const shot=async(name,locator,block='start',p=page)=>{
    for(let attempt=0;locator;attempt++){
      try{await locator.scrollIntoViewIfNeeded({timeout:5000});await locator.evaluate((e,b)=>{e.scrollIntoView({block:b});
        // A sticky dialog header would otherwise cover the subject's first lines.
        const header=e.closest('dialog')?.querySelector('.guidance-header');if(b!=='start'||!header)return;
        let s=e.parentElement;while(s&&!(s.scrollHeight>s.clientHeight&&/(auto|scroll)/.test(getComputedStyle(s).overflowY)))s=s.parentElement;
        if(s)s.scrollTop-=header.getBoundingClientRect().height+12;},block,{timeout:5000});break;}
      catch(error){if(attempt>=4||!/not attached|detached|Timeout/i.test(String(error)))throw error;} // a re-render replaced the subject; locate it again
      await frames(p);
    }
    if(locator)await frames(p);
    await p.screenshot({path:path.join(out,name+'.png')});evidence.screenshots.push(name+'.png');};
  const orders=()=>evidence.requests.filter(r=>r.route==='/api/command'||r.route==='/api/advance');
  const stop=label=>{if(process.env.SPHERES_GUIDANCE_STOP_AFTER===label)throw Object.assign(new Error('Stopped after '+label+' by SPHERES_GUIDANCE_STOP_AFTER'),{stopped:true});};

  // ---------- observed actions: every click/key/select is logged with the selector actually used ----------
  const last={main:null,second:null};
  const describe=p=>p.evaluate(()=>{const e=document.activeElement;if(!e||e===document.body)return 'body';
    const data=[...e.attributes].find(a=>a.name.startsWith('data-guidance-')&&a.name!=='data-guidance-focus');
    return e.id?'#'+e.id:e.tagName.toLowerCase()+(data?`[${data.name}="${data.value}"]`:'');});
  async function tap(selector,{locator=null,p=page,who='main'}={}){
    const loc=locator||p.locator(selector);let text=null;
    try{text=((await loc.textContent({timeout:5000}))||'').replace(/\s+/g,' ').trim().slice(0,90);}catch(_){}
    const row={seq:evidence.actions.length+1,stage,page:who,how:'click',selector,text};evidence.actions.push(row);last[who]=row;
    await loc.click();return row;
  }
  async function press(key,{p=page,who='main'}={}){
    const row={seq:evidence.actions.length+1,stage,page:who,how:'key',key,selector:await describe(p),text:null};evidence.actions.push(row);last[who]=row;
    await p.keyboard.press(key);return row;
  }
  async function choose(selector,value,{p=page,who='main'}={}){
    const row={seq:evidence.actions.length+1,stage,page:who,how:'select',selector,value:String(value),text:null};evidence.actions.push(row);last[who]=row;
    await p.locator(selector).selectOption(String(value));return row;
  }
  async function type(selector,value,{p=page,who='main'}={}){
    const row={seq:evidence.actions.length+1,stage,page:who,how:'fill',selector,value:String(value),text:null};evidence.actions.push(row);last[who]=row;
    await p.locator(selector).fill(String(value));return row;
  }
  // ---------- observation outside the page api() ----------
  async function observe(route,p=page){
    evidence.observer_requests.push({stage,method:'GET',route:route.split('?')[0]});
    const response=await p.request.get(url+route);
    try{assert(response.ok(),route+' returned '+response.status());return await response.json();}finally{await response.dispose();}
  }
  async function archive(slot){
    evidence.observer_requests.push({stage,method:'POST',route:'/api/save',slot});evidence.archive_saves.push({stage,slot});
    const response=await page.request.post(url+'/api/save',{data:{slot}});
    try{assert(response.ok(),'Could not capture '+slot);await response.body();}finally{await response.dispose();}
    const value=JSON.parse(fs.readFileSync(path.join(run,'saves',slot+'.json'),'utf8'));delete value.saved_unix;return value;
  }
  async function nativeGuidance(){
    const state=await observe('/api/state');
    const native=await observe('/api/guidance?session_id='+encodeURIComponent(state.session_id));
    assert.equal(native.state.session_id,state.session_id,'Native guidance must read the live session');
    assert.equal(native.state.date,state.date,'Native guidance must read the live date');
    return {state,native};
  }
  function watch(p,who){
    p.on('pageerror',error=>evidence.errors.push({page:who,stage,message:error.message}));
    p.on('console',message=>{if(message.type()==='error')evidence.console_errors.push({page:who,stage,text:message.text(),location:message.location()?.url||null});});
    p.on('response',response=>{if(response.status()>=400)evidence.http_errors.push({page:who,stage,status:response.status(),method:response.request().method(),url:response.url()});});
    p.on('request',request=>{
      const route=new URL(request.url()).pathname,window=who==='main'?guidanceWindow:null;
      if(request.method()==='GET'&&route==='/api/guidance')evidence.guidance_requests.push({stage,page:who,guidance_window:window,session_id:new URL(request.url()).searchParams.get('session_id')});
      if(request.method()==='POST'&&/^\/api\/(advance|command|new|save|load)$/.test(route)){
        let payload=null;try{payload=request.postDataJSON();}catch(_){payload=request.postData();}
        const control=last[who]?{action_seq:last[who].seq,how:last[who].how,selector:last[who].selector,key:last[who].key??null,text:last[who].text}:null;
        evidence.requests.push({seq:evidence.requests.length+1,stage,page:who,guidance_window:window,route,payload,
          kinds:Array.isArray(payload?.commands)?payload.commands.map(c=>c.kind):[],control});
      }
    });
  }

  // ---------- page helpers (closures over page/evidence) ----------
  async function closePanels(){
    const dialog=page.locator('#guidanceDialog');
    if(await dialog.count()&&await dialog.isVisible())await tap('#guidanceDialog [data-guidance-close]');
    if(await page.locator('#equipmentRoom').isVisible())await tap('[data-equipment-close]');
    if(await page.locator('#productionPanel').isVisible())await tap('#productionClose');
    if(await page.locator('#sheet').isVisible())await tap('#sheet > .head .close');
    if(await page.locator('#intelDrawer').count()&&await page.locator('#intelDrawer').getAttribute('aria-hidden')==='false')await tap('#intelDrawer [data-close-drawers]');
    if(await page.locator('#cabinetDrawer').isVisible())await tap('#cabinetDrawer [data-close-drawers]');
  }
  async function routeReady(){
    await page.waitForFunction(()=>{const m=GUIDANCE.session.state();
      return m.view==='tutorial'&&(m.status==='ready'||m.status==='error')&&document.querySelector('#guidanceDialog .guidance-route')?.getAttribute('aria-busy')==='false';});
    const model=await page.evaluate(()=>{const m=GUIDANCE.session.state();return {status:m.status,notice:m.notice,stale:m.routeStale,
      route:m.route?.status??null,reason:m.route?.reason??null,snapshot:m.snapshot?{session_id:m.snapshot.session_id,date:m.snapshot.date}:null,
      live:{session_id:S.session_id,date:S.date}};});
    assert.equal(model.status,'ready','Guidance reading failed: '+JSON.stringify(model));
    assert.equal(model.route,'ready','The accepted route must be ready: '+JSON.stringify(model));
    assert.equal(model.stale,false);assert.deepEqual(model.snapshot,model.live,'The accepted reading must be the live campaign');
  }
  async function openGuide(label,via='f1'){
    await closePanels();guidanceWindow='open:'+label;
    const before=orders().length;
    const response=page.waitForResponse(r=>isRoute(r,'/api/guidance','GET'));
    if(via==='f1')await press('F1');
    else await tap('.decision-nav button "Tutorial"',{locator:page.locator('.decision-nav').getByRole('button',{name:'Tutorial',exact:true})});
    await page.locator('#guidanceDialog').getByRole('heading',{name:'Your first steps in Spheres',exact:true}).waitFor();
    assert((await response).ok(),'Guidance must load a native reading');
    await routeReady();
    assert.equal(orders().length,before,'Opening guidance issued an order or advanced time');
  }
  async function readRendered(){
    return page.evaluate(()=>{
      const dialog=document.querySelector('#guidanceDialog'),panel=dialog.querySelector('.guidance-route');
      const text=el=>el?el.textContent.replace(/\s+/g,' ').trim():null;
      const heading=card=>{const h=card.querySelector('h4').cloneNode(true);h.querySelector('.guidance-route-number')?.remove();return text(h);};
      return {results_line:text(dialog.querySelector('.guidance-results-line')),route_line:text(panel.querySelector('.guidance-route-line')),
        refresh_label:text(panel.querySelector('[data-guidance-refresh]')),refresh_disabled:!!panel.querySelector('[data-guidance-refresh]')?.disabled,panel_text:text(panel),
        steps:[...panel.querySelectorAll('[data-guidance-route-card]')].map(card=>{
          const chips=card.querySelectorAll('.guidance-chips > .guidance-chip'),o=card.querySelector('.guidance-obstacle'),b=o?.querySelector('[data-guidance-route-step]');
          return {id:card.dataset.guidanceRouteCard,status:([...card.classList].find(c=>c.startsWith('guidance-route-step--'))||'').slice('guidance-route-step--'.length)||null,
            title:heading(card),reading:text(chips[0]),campaign:text(chips[1]),summary:text(card.querySelector('.guidance-route-summary')),
            milestones:[...card.querySelectorAll('.guidance-milestones > li')].map(li=>({state:text(li.querySelector('.guidance-milestone-state')),
              cls:li.className,date:li.querySelector('time')?.getAttribute('datetime')||null,detail:text(li.querySelector('small')),text:text(li)})),
            obstacle:o?{title:text(o.querySelector('strong')),reason:text(o.querySelector('p:not(.guidance-kicker)')),
              button:b?{tag:b.tagName,type:b.type,label:b.getAttribute('aria-label'),text:text(b),disabled:b.disabled,tabIndex:b.tabIndex}:null}:null};
        })};
    });
  }
  async function accepted(){
    return page.evaluate(()=>{const m=GUIDANCE.session.state();return JSON.parse(JSON.stringify({status:m.status,notice:m.notice,route:m.route,routeChecked:m.routeChecked,
      routeStale:m.routeStale,snapshot:m.snapshot?{session_id:m.snapshot.session_id,player:m.snapshot.player,date:m.snapshot.date,t:m.snapshot.t}:null,progress:m.progress}));});
  }
  async function storedReceipts(){
    const raw=await page.evaluate(key=>localStorage.getItem(key),RECEIPTS_KEY);
    if(raw===null)return {raw:null,receipts:{saves:[],loads:[]}};
    const parsed=JSON.parse(raw);assert.equal(parsed.version,1);return {raw:parsed,receipts:{saves:parsed.saves,loads:parsed.loads}};
  }
  const storedProgress=async()=>JSON.parse(await page.evaluate(k=>localStorage.getItem(k),PROGRESS_KEY));
  // The rendered panel must be the page's own accepted route, and that route must
  // equal AdvisorModel.recognize over a separate native /api/guidance reading.
  async function readRoute(label){
    guidanceWindow='read:'+label;const before=orders().length;
    const rendered=await readRendered(),model=await accepted(),{state,native}=await nativeGuidance(),{receipts}=await storedReceipts();
    const recomputed=await page.evaluate(([data,held])=>JSON.parse(JSON.stringify(AdvisorModel.recognize(data,held))),[native,receipts]);
    assert.equal(model.status,'ready');assert(model.route&&model.route.status==='ready','Accepted route must be ready');
    assert.equal(model.routeStale,false);assert.equal(model.snapshot.session_id,state.session_id);assert.equal(model.snapshot.date,state.date);
    assert.equal(model.routeChecked,state.date);
    assert.deepEqual(recomputed,model.route,'Accepted route must equal AdvisorModel.recognize over the native /api/guidance reading');
    assert.deepEqual(model.route.as_of,{session_id:state.session_id,player:'France',date:state.date,day:native.outcomes.as_of_day});
    assert.equal(native.outcomes.nation,'France');assert.equal(native.outcomes.date,state.date);
    assert.deepEqual(rendered.steps.map(s=>s.id),STEP_IDS,'Six route steps render in route order');
    const raw=RAW_DISTRICT.exec(rendered.panel_text);assert.equal(raw,null,'The route panel shows a raw district id: '+(raw&&raw[0]));
    model.route.steps.forEach((step,i)=>{
      const r=rendered.steps[i];assert.equal(r.id,step.id);assert.equal(r.status,step.status,step.id+' rendered status');
      assert(r.campaign.includes('Campaign: '+STATUS_LABEL[step.status]),step.id+' campaign chip: '+r.campaign);
      if(step.status==='achieved')assert(r.campaign.includes('as of '+state.date),step.id+' achieved chip must carry the reading date');
      assert(/^Reading: (Read|Not read|Skipped)/.test(r.reading),step.id+' reading chip: '+r.reading);
      assert.equal(r.milestones.length,step.milestones.length,step.id+' milestone count');
      step.milestones.forEach((m,j)=>{assert.equal(r.milestones[j].state,MILESTONE_LABEL[m.status],step.id+'/'+m.id+' state');
        assert.equal(r.milestones[j].date,m.date,step.id+'/'+m.id+' date');assert.equal(r.milestones[j].detail,m.detail||null,step.id+'/'+m.id+' detail');
        if(m.status==='done'&&m.day!==null){assert(m.day<=native.outcomes.as_of_day,step.id+'/'+m.id+' is dated after the reading');assert.equal(m.date,iso(m.day));}});
      assert.equal(!!r.obstacle,!!step.obstacle,step.id+' obstacle presence');
      if(step.obstacle){assert.equal(r.obstacle.title,step.obstacle.title);assert.equal(r.obstacle.reason,step.obstacle.reason,step.id+' obstacle explanation');
        assert(r.obstacle.button,step.id+' obstacle needs a review button');
        assert.equal(r.obstacle.button.tag,'BUTTON');assert.equal(r.obstacle.button.type,'button');assert.equal(r.obstacle.button.disabled,false);
        assert.equal(r.obstacle.button.label,`${step.obstacle.actionLabel}: ${step.title}`);}
    });
    const achieved=model.route.steps.filter(s=>s.status==='achieved').length;
    assert.equal(rendered.results_line,`Campaign results: ${achieved} of 6 achieved · see Your first hour below`);
    assert.equal(rendered.route_line,`Checked ${state.date}. Only dated records from this campaign count.`);
    assert.equal(orders().length,before,'Reading guidance issued an order');guidanceWindow=null;
    const row={label,date:state.date,session_id:state.session_id,as_of_day:native.outcomes.as_of_day,results_line:rendered.results_line,
      steps:model.route.steps.map((step,i)=>({id:step.id,status:step.status,rendered_status:rendered.steps[i].status,reading:rendered.steps[i].reading,campaign:rendered.steps[i].campaign,summary:step.summary,
        milestones:step.milestones.map(m=>({id:m.id,status:m.status,date:m.date,detail:m.detail})),
        rendered_milestones:rendered.steps[i].milestones.map(m=>({state:m.state,date:m.date})),
        obstacle:step.obstacle?{title:step.obstacle.title,reason:step.obstacle.reason,action:step.obstacle.action,actionLabel:step.obstacle.actionLabel}:null})),
      recognize_check:{method:'AdvisorModel.recognize(GET /api/guidance?session_id=..., stored receipts) evaluated in the page; deep-equal to the accepted route, which the panel renders. '
          +'This re-runs S19 code over the S19 payload; it is not independent of S19 (see independent_cross_check on the stages that have one).',
        native_date:native.state.date,native_session_id:native.state.session_id,native_as_of_day:native.outcomes.as_of_day,
        statuses:Object.fromEntries(recomputed.steps.map(s=>[s.id,s.status])),recomputed_equals_accepted:true,rendered_equals_accepted:true,
        receipts:{saves:receipts.saves.length,loads:receipts.loads.length}}};
    evidence.guidance_reads.push({label,date:state.date,session_id:state.session_id});
    write('guidance-'+label+'.json',{label,rendered,accepted:model,native_outcomes:native.outcomes,native_state_identity:{session_id:state.session_id,player:state.player,date:state.date,t:state.t},receipts});
    return {row,route:model.route,native,state,rendered,step:id=>model.route.steps.find(s=>s.id===id),renderedStep:id=>rendered.steps.find(s=>s.id===id)};
  }
  async function follow(step,via='click'){
    guidanceWindow='follow:'+step;const before=orders().length;
    const selector=`#guidanceDialog [data-guidance-route-step=${q(step)}]`,button=page.locator(selector);
    assert.equal(await button.count(),1,'One obstacle button for '+step);
    assert.equal(await button.evaluate(b=>b.tagName),'BUTTON');assert(await button.isEnabled());
    const label=await button.getAttribute('aria-label');let control;
    if(via==='keyboard'){
      await button.focus();assert.equal(await button.evaluate(b=>document.activeElement===b),true,'The obstacle button must take keyboard focus');
      control=await press('Enter');assert.equal(control.selector,`button[data-guidance-route-step="${step}"]`,'Enter must be pressed on the focused obstacle button');
    }else control=await tap(selector);
    await page.locator('#guidanceDialog').waitFor({state:'hidden'});
    return {label,control,done:()=>{assert.equal(orders().length,before,'Following the '+step+' obstacle issued an order');guidanceWindow=null;}};
  }
  async function stepDay(p=page,who='main'){
    if(who==='main')await closePanels();const before=await observe('/api/state',p);
    const wait=p.waitForResponse(r=>isRoute(r,'/api/advance'));
    await tap('#stepBtn',{p,who});const response=await wait;assert(response.ok(),'+1 DAY failed');
    const payload=response.request().postDataJSON();assert.equal(payload.days,1);
    const result=await response.json();assert(!result.advance_pending);assert.deepEqual(result.errors||[],[]);await idle(p);
    const after=await observe('/api/state',p);assert.equal(after.session_id,before.session_id);assert.notEqual(after.date,before.date);
    return after;
  }
  async function command(action,kind){
    const wait=page.waitForResponse(r=>isRoute(r,'/api/command'));await action();const response=await wait;
    assert(response.ok(),kind+' command refused');const result=await response.json();assert.deepEqual(result.errors||[],[]);assert(!result.command_pending);
    const payload=response.request().postDataJSON();assert(payload.commands.some(c=>c.kind===kind),'Expected a '+kind+' command');await idle(page);return payload;
  }
  async function productionOpen(){
    await page.waitForFunction(()=>PROD.open&&PROD.mode==='build'&&PROD.data&&!PROD.loading&&!PROD.stale&&!PROD.busy&&!PROD.error);
  }
  async function designerReady(){await page.waitForFunction(()=>equipmentCurrent()&&equipmentPreviewCurrent()&&!EQUIP.review&&!EQUIP.replacement);}
  async function reviewQuote(){await page.waitForFunction(()=>equipmentReviewQuoteCurrent());return page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.review.quote)));}
  async function campaignHome(p=page,who='main'){
    if(who==='main')await closePanels();
    if(await p.locator('#app').isVisible()){
      if(await p.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary',{p,who});
      await tap('#campaignsBtn',{p,who});
    }
    await p.locator('#campaignHome').waitFor({state:'visible'});
  }
  async function continueCampaign(p=page,who='main'){
    await tap('#continueBtn',{p,who});await p.locator('#app').waitFor({state:'visible'});await idle(p);
  }
  async function overflow(selectors){
    const boxes={};
    for(const selector of selectors){const locator=page.locator(selector).first();if(!await locator.count())continue;
      const box=await locator.evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth}));
      assert(box.width>0&&box.scroll<=box.width+1,selector+' overflows horizontally: '+JSON.stringify(box));boxes[selector]=box;}
    const documentBox=await page.evaluate(()=>({width:document.documentElement.clientWidth,scroll:document.documentElement.scrollWidth}));
    assert(documentBox.scroll<=documentBox.width+1,'The document overflows horizontally: '+JSON.stringify(documentBox));
    return {...boxes,document:documentBox};
  }
  // Measures the open tutorial at 390x844: containers, every route card, obstacle and its button.
  async function narrowMeasure(){
    await page.setViewportSize({width:390,height:844});await frames(page);
    const dims=await overflow(['#guidanceDialog','#guidanceDialog .guidance-content','#guidanceDialog .guidance-route','#guidanceDialog .guidance-lesson']);
    const cards=await page.evaluate(()=>{const dialog=document.querySelector('#guidanceDialog').getBoundingClientRect();
      return [...document.querySelectorAll('#guidanceDialog [data-guidance-route-card]')].map(card=>{const o=card.querySelector('.guidance-obstacle'),b=card.querySelector('[data-guidance-route-step]'),r=b?.getBoundingClientRect();
        return {id:card.dataset.guidanceRouteCard,width:card.clientWidth,scroll:card.scrollWidth,obstacle:o?{width:o.clientWidth,scroll:o.scrollWidth}:null,
          button:b?{left:Math.round(r.left),right:Math.round(r.right),width:Math.round(r.width),inside:r.left>=dialog.left-1&&r.right<=dialog.right+1}:null};});});
    for(const c of cards){assert(c.width>0&&c.scroll<=c.width+1,c.id+' card overflows: '+JSON.stringify(c));
      if(c.obstacle)assert(c.obstacle.scroll<=c.obstacle.width+1,c.id+' obstacle overflows: '+JSON.stringify(c));
      if(c.button)assert(c.button.inside&&c.button.width>0,c.id+' obstacle button leaves the dialog: '+JSON.stringify(c));}
    return {viewport:{width:390,height:844},overflow:dims,cards,obstacles:cards.filter(c=>c.obstacle).length};
  }
  // "Your advisors" from the same accepted reading; the route-next rule is the first unachieved step with an
  // obstacle, dropped only when another card already opens exactly that screen.
  async function advisors(label){
    guidanceWindow='advisors:'+label;const before=orders().length;
    const tabSelector='#guidanceDialog [data-guidance-view="advisors"]';
    assert.equal((await page.locator(tabSelector).innerText()).trim(),'Your advisors');
    const control=await tap(tabSelector);
    await page.waitForFunction(()=>{const m=GUIDANCE.session.state();return m.view==='advisors'&&m.status==='ready'&&document.querySelector('#guidanceDialog .guidance-cards')?.getAttribute('aria-busy')==='false';});
    const model=await page.evaluate(()=>{const m=GUIDANCE.session.state();return JSON.parse(JSON.stringify({cards:m.cards,route:m.route,snapshot:{session_id:m.snapshot.session_id,date:m.snapshot.date}}));});
    const dom=await page.evaluate(()=>[...document.querySelectorAll('#guidanceDialog [data-guidance-card]')].map(a=>({id:a.dataset.guidanceCard,title:a.querySelector('h3')?.textContent.trim(),
      reason:a.querySelector('.guidance-advice-copy > p:not(.guidance-kicker):not(.guidance-caution)')?.textContent.trim(),
      evidence:[...a.querySelectorAll('details li')].map(li=>li.textContent.trim()),follow:a.querySelector('[data-guidance-follow]')?.textContent.replace(/\s+/g,' ').trim()})));
    const next=model.route.steps.find(s=>s.status!=='achieved'&&s.obstacle)||null;
    const card=model.cards.find(c=>c.id==='route-next')||null,domCard=dom.find(c=>c.id==='route-next')||null;
    const duplicate=next?model.cards.find(c=>c.id!=='route-next'&&JSON.stringify(c.action)===JSON.stringify(next.obstacle.action))||null:null;
    assert.deepEqual(dom.map(c=>c.id),model.cards.map(c=>c.id),'Rendered advisor cards must be the accepted cards');
    const cardsText=await page.locator('#guidanceDialog .guidance-cards').textContent(),rawCard=RAW_DISTRICT.exec(cardsText);
    assert.equal(rawCard,null,'An advisor card shows a raw district id: '+(rawCard&&rawCard[0]));
    if(next&&!duplicate){
      assert(card&&domCard,'A route-next card must name the first unachieved step '+next.id);
      assert.equal(card.title,next.obstacle.title);assert.equal(card.reason,next.obstacle.reason);assert.equal(card.actionLabel,next.obstacle.actionLabel);
      assert.deepEqual(card.action,next.obstacle.action);assert(card.evidence.includes(`First-hour step: ${next.title}.`),JSON.stringify(card.evidence));
      assert.equal(domCard.title,next.obstacle.title);assert.equal(domCard.reason,next.obstacle.reason);
      assert(domCard.follow.startsWith(next.obstacle.actionLabel),domCard.follow);assert(domCard.evidence.includes(`First-hour step: ${next.title}.`));
    }else assert.equal(card,null,'route-next must be dropped when another card already opens '+JSON.stringify(next?.obstacle?.action));
    assert.equal(orders().length,before,'Opening Your advisors issued an order');guidanceWindow=null;
    return {label,control:control.selector,snapshot:model.snapshot,next:next?{id:next.id,title:next.title,obstacle:next.obstacle}:null,
      route_next:card?{title:card.title,reason:card.reason,action:card.action,actionLabel:card.actionLabel,evidence:card.evidence}:null,rendered_route_next:domCard,
      duplicate:duplicate?{id:duplicate.id,title:duplicate.title,action:duplicate.action}:null,cards:model.cards.map(c=>({id:c.id,priority:c.priority,title:c.title,action:c.action}))};
  }
  async function backToTutorial(){
    await tap('#guidanceDialog [data-guidance-view="tutorial"]');
    await page.waitForFunction(()=>GUIDANCE.session.state().view==='tutorial'&&document.querySelector('#guidanceDialog .guidance-route')?.getAttribute('aria-busy')==='false');
    await routeReady();
  }
  // Read-only page observers: a PerformanceObserver sees a held /api/guidance response reach the page, and a
  // MutationObserver on the dialog content proves no repaint followed; the accepted route object is compared by identity.
  async function armDiscardWatch(){
    await page.evaluate(()=>{const race=window.__s19Race={entries:[],mutations:0,route:GUIDANCE.session.state().route};
      race.po=new PerformanceObserver(list=>{for(const e of list.getEntries())if(new URL(e.name).pathname==='/api/guidance')race.entries.push({responseEnd:Math.round(e.responseEnd)});});
      race.po.observe({type:'resource'});
      race.mo=new MutationObserver(records=>{race.mutations+=records.length;});
      race.mo.observe(document.querySelector('#guidanceDialog .guidance-content'),{childList:true,subtree:true,characterData:true,attributes:true});});
  }
  async function discardVerdict(){
    await page.waitForFunction(()=>window.__s19Race.entries.length>=1,null,{timeout:10000});
    await page.evaluate(()=>new Promise(resolve=>setTimeout(()=>setTimeout(resolve,50),0)));await frames(page);
    const verdict=await page.evaluate(()=>{const race=window.__s19Race,m=GUIDANCE.session.state();race.po.disconnect();race.mo.disconnect();
      const out={same_route_object:m.route===race.route,mutations:race.mutations,resource_entries:race.entries.length,status:m.status,routeStale:m.routeStale,routeChecked:m.routeChecked};delete window.__s19Race;return out;});
    assert.equal(verdict.same_route_object,true,'The held (older) reading replaced the accepted route');
    assert.equal(verdict.mutations,0,'The held reading repainted the dialog');assert.equal(verdict.status,'ready');assert.equal(verdict.routeStale,false);
    return verdict;
  }
  // Hold exactly one real /api/guidance response; later requests pass untouched.
  async function holdGuidance(){
    let used=false,release,reportReady,reportDone,request=null;
    const gate=new Promise(resolve=>{release=resolve;}),ready=new Promise(resolve=>{reportReady=resolve;}),done=new Promise(resolve=>{reportDone=resolve;});
    const pattern='**/api/guidance?*';
    const handler=async route=>{
      if(used)return route.continue();used=true;request=route.request();
      try{const response=await route.fetch();const body=await response.json();reportReady({ok:response.ok(),date:body.state?.date,session_id:body.state?.session_id,as_of_day:body.outcomes?.as_of_day});
        await gate;await route.fulfill({response});}
      finally{reportDone();}
    };
    await page.route(pattern,handler);
    return {ready,release,request:()=>request,async finish(){release();await done;await page.unroute(pattern,handler);}};
  }
  // Hold exactly one real /api/guidance request before it reaches the server; it is then sent unchanged.
  async function holdGuidanceRequest(){
    let used=false,release,reportRequested,reportContinued,request=null;
    const gate=new Promise(resolve=>{release=resolve;}),requested=new Promise(resolve=>{reportRequested=resolve;}),continued=new Promise(resolve=>{reportContinued=resolve;});
    const pattern='**/api/guidance?*';
    const handler=async route=>{
      if(used)return route.continue();used=true;request=route.request();reportRequested(request);
      try{await gate;await route.continue();}finally{reportContinued();}
    };
    await page.route(pattern,handler);
    return {requested,release:()=>release(),request:()=>request,async finish(){release();await continued;await page.unroute(pattern,handler);}};
  }

  try{
    const startupDeadline=Date.now()+30000;
    for(let attempt=0;;attempt++){
      if(launchError)throw launchError;if(server.exitCode!==null)throw Error('Disposable server exited with '+server.exitCode);
      try{const response=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(Math.max(1,Math.min(2000,startupDeadline-Date.now())))});await response.arrayBuffer();if(response.ok)break;}catch(_){}
      if(attempt>=300||Date.now()>=startupDeadline)throw Error('Disposable server failed to start');await new Promise(resolve=>setTimeout(resolve,100));
    }
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});
    evidence.browser_version=browser.version();
    const context=await browser.newContext({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page=await context.newPage();page.setDefaultTimeout(45000);watch(page,'main');

    mark('build provenance');
    evidence.build=await integrated.verifyBuild({page,url,root,run,binary});
    assert.equal(evidence.build.binary_sha256,binaryBefore);
    evidence.guidance_assets=await verifyGuidanceAssets(page,url,evidence.build.revision);
    evidence.embedded_text=await verifyEmbeddedText(page,url,evidence.build.revision,fs.readFileSync(path.join(root,'spheres-web/ui/index.html')));
    check(`Build: /api/build revision equals the expected commit; verifyBuild assets plus the four guidance files are served byte-identical to the checkout and equal the commit after CRLF normalization; `
      +`${evidence.embedded_text.served_byte_equal} of ${evidence.embedded_text.embedded_text_files} include_str! UI text files are served byte-identical at their root path (all ${evidence.embedded_text.embedded_text_files} equal the commit)`);

    mark('fresh France');
    await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor({state:'visible'});
    await page.waitForFunction(()=>!!SESSION.live?.session_id);
    await tap('#newCampaignBtn');await page.locator('#newCampaignPicker').waitFor({state:'visible'});
    await tap('#nationPick [aria-label^="France;"]');await tap('#startBtn');
    await page.locator('#app').waitFor({state:'visible'});await idle(page);
    const opening=await observe('/api/state');assert.equal(opening.player,'France');assert.equal(opening.simulation_cadence,'daily');
    assert.equal(await page.evaluate(()=>clock.running),false,'A fresh campaign must start paused');
    assert.equal(await page.evaluate(k=>localStorage.getItem(k),RECEIPTS_KEY),null,'Fresh browser profile holds no receipts');
    evidence.opening={date:opening.date,session_id:opening.session_id,player:opening.player};
    stop('fresh');

    // 1. Initial route: a pure read, the fresh Advisors view and the fresh narrow layout.
    mark('1 initial route');
    const pureBefore=await archive('s19-read-before');const ordersBefore=orders().length;
    await openGuide('initial','f1');
    const initial=await readRoute('initial');
    for(const step of initial.route.steps){assert.notEqual(step.status,'achieved',step.id+' cannot be achieved in a fresh campaign');
      assert(step.obstacle,step.id+' needs an obstacle in a fresh campaign');}
    assert(['not_yet','unknown'].includes(initial.step('finances').status));
    const procurementObstacle=initial.step('procurement').obstacle;
    assert(/manufacturer/i.test(procurementObstacle.title+' '+procurementObstacle.reason),'Procurement obstacle must name the manufacturer: '+JSON.stringify(procurementObstacle));
    await shot('route-initial-desktop',page.locator('#guidanceDialog .guidance-route'));
    await shot('lesson-initial-desktop',page.locator('#guidanceDialog .guidance-lesson'));
    const freshAdvisors=await advisors('fresh');
    assert.equal(freshAdvisors.next.id,'finances');
    await shot('advisors-fresh-desktop',page.locator('#guidanceDialog .guidance-cards'));
    await backToTutorial();
    // H. Narrow layout at the fresh state, with all six obstacles.
    const freshNarrow=await narrowMeasure();assert.equal(freshNarrow.obstacles,6,'The fresh route shows six obstacles');
    await shot('narrow-initial-route',page.locator('#guidanceDialog .guidance-route'));
    await shot('narrow-initial-obstacles',page.locator('#guidanceDialog [data-guidance-route-card="procurement"]'));
    await page.setViewportSize({width:1440,height:1000});await frames(page);
    assert.equal(orders().length,ordersBefore,'Opening, reading, Your advisors and resizing issued no order');
    assert.deepEqual(await archive('s19-read-after'),pureBefore,'Opening and reading guidance must leave the saved campaign unchanged');
    check(`Stage 1: F1 opened guidance on a fresh France campaign; all six steps read not_yet with an obstacle and rendered explanation; procurement names the manufacturer; `
      +`Your advisors opened from the same reading (${freshAdvisors.route_next?'route-next present':'route-next dropped because '+freshAdvisors.duplicate.id+' already opens '+JSON.stringify(freshAdvisors.duplicate.action)}); `
      +'at 390x844 the dialog, content, lesson, route panel, all six route cards, obstacles and their buttons fit without horizontal overflow; no order and the saved archive minus saved_unix is unchanged');
    evidence.stages.push({stage:'1 initial route',...initial.row,orders_issued:0,archive_unchanged:true,via:'F1',
      initial_statuses:Object.fromEntries(initial.route.steps.map(s=>[s.id,s.status])),advisors:freshAdvisors,narrow_fresh:freshNarrow});
    stop('1');

    // 2. Reading is not a result.
    mark('2 reading vs result');
    await tap('[data-guidance-lesson="budget-treasury"]');
    await page.locator('#guidanceDialog [data-guidance-heading]').filter({hasText:'Give your money a purpose'}).waitFor();
    await tap('#guidanceDialog [data-guidance-complete]');
    const reading=await readRoute('after-budget-lesson');
    const financeChip=reading.renderedStep('finances');
    assert.equal(financeChip.reading,'Reading: Read','Budget lesson must read Read');
    assert(financeChip.campaign.includes('Campaign: Not yet'),'Reading the lesson must not complete finances: '+financeChip.campaign);
    assert.deepEqual(reading.route,initial.route,'Reading a lesson must not change the campaign route');
    const progressRead=await storedProgress();
    assert(progressRead.done.includes('budget-treasury'));assert.equal(orders().length,ordersBefore);
    check("Stage 2: 'I understand' on the budget lesson shows finances Reading: Read while Campaign: Not yet; the route is unchanged and no order was issued");
    await shot('reading-vs-result-desktop',page.locator('#guidanceDialog [data-guidance-route-card="finances"]'));
    evidence.stages.push({stage:'2 reading vs result',...reading.row,finances_reading:financeChip.reading,finances_campaign:financeChip.campaign,orders_issued:0,
      stored_progress:progressRead});

    // 2b. D: skipping a lesson is reading progress only.
    mark('2b lesson skip');
    const skippedLesson=await page.evaluate(()=>GUIDANCE.session.state().progress.current);
    assert.equal(skippedLesson,'construction-effects','After the budget lesson the current lesson is the construction lesson');
    assert.equal((await page.locator('#guidanceDialog [data-guidance-skip]').innerText()).trim(),'Skip for now');
    await tap('#guidanceDialog [data-guidance-skip]');
    await page.waitForFunction(id=>GUIDANCE.session.state().progress.skipped.includes(id),skippedLesson);
    const afterSkip=await readRoute('after-skip');
    const constructionChip=afterSkip.renderedStep('construction');
    assert.equal(constructionChip.reading,'Reading: Skipped','Skipping the construction lesson shows Reading: Skipped');
    assert(constructionChip.campaign.includes('Campaign: Not yet'),'Skipping must not change the campaign status: '+constructionChip.campaign);
    assert.deepEqual(afterSkip.route,initial.route,'Skipping a lesson must not change the campaign route');
    const railChip=(await page.locator(`#guidanceDialog [data-guidance-lesson="${skippedLesson}"] small`).innerText()).trim();assert.equal(railChip,'Skipped');
    const progressSkip=await storedProgress();assert(progressSkip.skipped.includes(skippedLesson));assert(!progressSkip.done.includes(skippedLesson));
    assert.equal(orders().length,ordersBefore,'Skipping a lesson issued an order');
    check("Stage 2b: 'Skip for now' on the construction lesson shows construction Reading: Skipped (lesson rail: Skipped) while Campaign: Not yet; the route is unchanged and no order was issued");
    await shot('lesson-skipped-desktop',page.locator('#guidanceDialog [data-guidance-route-card="construction"]'));
    evidence.stages.push({stage:'2b lesson skip',...afterSkip.row,skipped:skippedLesson,construction_reading:constructionChip.reading,construction_campaign:constructionChip.campaign,
      lesson_rail:railChip,stored_progress:progressSkip,orders_issued:0,route_unchanged:true});
    stop('2');

    // 3. Finances through the obstacle (keyboard: focus + Enter) and the visible Cabinet enact.
    mark('3 finances');
    const toBudget=await follow('finances','keyboard');
    await page.locator('#cabinet-budget').waitFor({state:'visible'});toBudget.done();
    const enactBefore=await observe('/api/state');
    const enactWait=page.waitForResponse(r=>isRoute(r,'/api/advance'));
    const enactLabel=(await page.locator('#cabinetEnact').innerText()).trim();assert.match(enactLabel,/^Enact & advance 1 day/,'The visible Cabinet control must enact the budget');
    await tap('#cabinetEnact');const enacted=await enactWait;assert(enacted.ok(),'Cabinet enact failed');
    const enactPayload=enacted.request().postDataJSON();assert.equal(enactPayload.days,1);assert.equal(enactPayload.session_id,enactBefore.session_id);
    await page.waitForFunction(()=>!advancing&&!CAB.busy&&!pendingAdvance);await idle(page);
    const enactAfter=await observe('/api/state');assert.notEqual(enactAfter.date,enactBefore.date);
    assert.deepEqual((enactAfter.log||[]).filter(row=>row.date===enactBefore.date&&row.text.startsWith('[rejected]')),[],'An enacted order was refused');
    await openGuide('finances','tutorial');
    const fin=await readRoute('finances');
    assert.equal(fin.step('finances').status,'achieved','Finances must be achieved after the enacted budget');
    const budgetMilestone=fin.step('finances').milestones.find(m=>m.id==='budget_enacted');
    const yearStart=Math.round((Date.UTC(fin.state.year,0,1)-Date.UTC(1990,0,1))/86400000);
    const decisions=fin.native.outcomes.money.decisions.filter(d=>BUDGET_KINDS.includes(d.kind)&&d.day>=yearStart&&d.day<=fin.native.outcomes.as_of_day);
    assert(decisions.length>0,'Native money journal must hold the budget decision');
    const firstDecision=decisions.reduce((a,b)=>b.day<a.day?b:a);
    assert.equal(budgetMilestone.status,'done');assert.equal(budgetMilestone.day,firstDecision.day);assert.equal(budgetMilestone.date,iso(firstDecision.day));
    assert.equal(fin.renderedStep('finances').milestones[0].date,iso(firstDecision.day));
    // B. Independent: the pre-S19 cash-flow money view, selected by its own labels and dates.
    const cash=await observe('/api/cash-flow?session_id='+encodeURIComponent(fin.state.session_id));
    assert.equal(cash.money?.available,true,'The cash-flow money account must be available');
    const budgetRows=cash.money.decisions.filter(d=>Object.values(DECISION_LABEL).includes(d.label)&&d.date>=`${fin.state.year}-01-01`&&d.date<=iso(fin.native.outcomes.as_of_day));
    assert(budgetRows.length>0,'GET /api/cash-flow money.decisions must list this year\'s budget decision');
    const earliestRow=budgetRows.reduce((a,b)=>b.date<a.date||b.date===a.date&&b.id<a.id?b:a);
    assert.equal(earliestRow.date,budgetMilestone.date,'The route budget date must equal the cash-flow journal date');
    assert.equal(earliestRow.id,firstDecision.id);assert.equal(earliestRow.label,DECISION_LABEL[firstDecision.kind]);
    const financeCross={endpoint:'GET /api/cash-flow?session_id=... money.decisions (money_view.rs, pre-S19)',selected_by:'budget labels, date within this year up to the reading',
      row:{id:earliestRow.id,label:earliestRow.label,date:earliestRow.date},route_milestone_date:budgetMilestone.date,equal:true};
    check('Stage 3: the finances obstacle was activated with focus + Enter and opened the Cabinet budget; the visible #cabinetEnact enacted it; finances is Achieved dated '+budgetMilestone.date
      +', which equals the pre-S19 GET /api/cash-flow money.decisions row "'+earliestRow.label+'" dated '+earliestRow.date);
    await shot('route-finances-desktop',page.locator('#guidanceDialog [data-guidance-route-card="finances"]'));
    evidence.stages.push({stage:'3 finances',...fin.row,obstacle_control:toBudget.control,control:'#cabinetEnact ('+enactLabel+')',enacted:{before:enactBefore.date,after:enactAfter.date},
      native_decision:firstDecision,milestone_date:budgetMilestone.date,independent_cross_check:financeCross});

    // 3b. F: Your advisors carries the route-next card, and following it opens the real screen.
    mark('3b advisors route-next');
    const earlyAdvisors=await advisors('after-finances');
    assert.equal(earlyAdvisors.next.id,'construction');assert(earlyAdvisors.route_next,'Your advisors must carry the route-next card after the budget');
    await shot('advisors-route-next-desktop',page.locator('#guidanceDialog [data-guidance-card="route-next"]'));
    guidanceWindow='follow:route-next';const advisorOrders=orders().length;
    const advisorControl=await tap('#guidanceDialog [data-guidance-follow="route-next"]');
    await page.locator('#guidanceDialog').waitFor({state:'hidden'});await productionOpen();
    assert.equal(orders().length,advisorOrders,'Following route-next issued an order');guidanceWindow=null;
    const advisorLanding=await page.evaluate(()=>({open:PROD.open,mode:PROD.mode,panel:!!document.querySelector('#productionPanel')&&getComputedStyle(document.querySelector('#productionPanel')).display!=='none'}));
    assert.deepEqual(advisorLanding,{open:true,mode:'build',panel:true},'route-next must open construction');
    await shot('advisors-route-next-landing',page.locator('#productionBody'));
    check(`Stage 3b: the visible "Your advisors" tab showed a route-next card titled "${earlyAdvisors.route_next.title}" naming the first unachieved step (${earlyAdvisors.next.title}) with its obstacle explanation; `
      +`following "${earlyAdvisors.route_next.actionLabel}" opened construction and issued no order`);
    evidence.stages.push({stage:'3b advisors route-next',advisors:earlyAdvisors,follow_control:advisorControl.selector,landing:advisorLanding,orders_issued:0});
    stop('3');

    // 4. Construction: fund the daily budget, preview without confirming, start a workshop, pay work with +1 DAY.
    mark('4 construction');
    await openGuide('before-construction','f1');
    const toConstruction=await follow('construction');
    await productionOpen();toConstruction.done();
    const board=await observe('/api/production');
    const district=(board.provinces.find(p=>/le-de-france/i.test(p.id))||{}).id;assert(district,'Île-de-France must be a French construction province');
    const funded=board.construction_budget.daily_budget_bn*4;assert(Number.isFinite(funded)&&funded>0,'Fresh France must have a positive daily construction budget');
    await type('#constructionDailyBudget',funded*1000);
    const budgetPayload=await command(()=>tap('#constructionBudgetForm button[type="submit"]'),'construction_budget');
    await productionOpen();
    const openWorkshopPreview=async()=>{
      await tap('[data-prod-new]');await tap('[data-construction-small]');
      await choose('#constructionWorkshopProvince',district);
      await tap('#constructionWorkshopForm button');
      await page.locator('[data-construction-workshop="0"]').waitFor({state:'visible'});await tap('[data-construction-workshop="0"]');
      await page.waitForFunction(()=>constructionPreviewCurrent()&&PROD.preview.can_start===true);
      return page.evaluate(()=>JSON.parse(JSON.stringify(PROD.preview)));
    };

    // 4a. A: a construction preview that is not confirmed changes nothing.
    mark('4a preview without confirm');
    await openGuide('before-preview','f1');const beforePreview=await readRoute('before-preview');
    const previewArchiveBefore=await archive('s19-preview-before');const queueBeforePreview=(await observe('/api/production')).queue.map(p=>p.id);
    const previewOrders=orders().length;
    const toPreview=await follow('construction');await productionOpen();toPreview.done();
    const unconfirmed=await openWorkshopPreview();
    await shot('construction-preview-unconfirmed',page.locator('#productionBody'));
    await closePanels(); // leave the preview without its confirm button
    const previewArchiveAfter=await archive('s19-preview-after');
    assert.deepEqual(previewArchiveAfter,previewArchiveBefore,'An unconfirmed construction preview must leave the saved campaign unchanged');
    assert.deepEqual((await observe('/api/production')).queue.map(p=>p.id),queueBeforePreview,'An unconfirmed preview must not queue a project');
    await openGuide('after-preview','f1');const afterPreview=await readRoute('after-preview');
    assert.deepEqual(afterPreview.route,beforePreview.route,'An unconfirmed preview must not change the route');
    assert.equal(afterPreview.step('construction').status,'not_yet');
    assert.equal(orders().length,previewOrders,'An unconfirmed preview issued an order');
    check('Stage 4a: a Starter workshop preview (can_start) was closed without confirming; the saved archive minus saved_unix, the production queue and the route are unchanged and no order was issued');
    evidence.stages.push({stage:'4a preview without confirm',...afterPreview.row,preview:{can_start:unconfirmed.can_start,cost_bn:unconfirmed.cost_bn,district},
      archive_unchanged:true,queue_unchanged:queueBeforePreview,route_unchanged:true,orders_issued:0});

    // 4. Confirm the workshop this time.
    mark('4 construction');
    const toWorkshop=await follow('construction');await productionOpen();toWorkshop.done();
    const beforeQueue=new Set((await observe('/api/production')).queue.map(p=>p.id));
    const quote=await openWorkshopPreview();
    await shot('construction-preview-desktop',page.locator('#productionBody'));
    const workshopPayload=await command(()=>tap('[data-construction-confirm]'),'start_industry_module');
    const queuedBoard=await observe('/api/production');const queued=queuedBoard.queue.filter(p=>!beforeQueue.has(p.id));
    assert.equal(queued.length,1);assert.equal(queued[0].kind,'starter_industry');assert.equal(queued[0].province.id,district);
    const workshopId=queued[0].id;

    // 4b. A: the command before its result is not a result.
    mark('4b command before result');
    await openGuide('workshop-queued','f1');const queuedRead=await readRoute('workshop-queued');
    const queuedStep=queuedRead.step('construction'),queuedNative=queuedRead.native.outcomes.construction.projects.find(p=>p.id===workshopId);
    assert(queuedNative,'Native outcomes list the queued workshop');assert(!(queuedNative.last_spent_bn>0),'No work may be paid before a day closes');
    assert.equal(queuedStep.status,'not_yet','A queued, unpaid workshop is Not yet');
    assert.equal(queuedStep.milestones.find(m=>m.id==='work_paid').status,'pending');
    assert.equal(queuedStep.obstacle.title,'Your project awaits paid work');assert.deepEqual(queuedStep.obstacle.action,{kind:'project',id:workshopId});
    assert.equal(queuedStep.obstacle.actionLabel,'Review this project');
    const queuedFinance=(await observe('/api/production')).queue.find(p=>p.id===workshopId)?.finance;
    assert(queuedFinance&&queuedFinance.spent_bn===0,'GET /api/production must show nothing spent before a day closes: '+JSON.stringify(queuedFinance));
    check('Stage 4b: right after the workshop confirm (no day advanced) construction is Not yet with work_paid pending and the obstacle "Your project awaits paid work" opening project '+workshopId
      +'; pre-S19 GET /api/production shows finance.spent_bn 0 for it');
    await shot('route-workshop-queued',page.locator('#guidanceDialog [data-guidance-route-card="construction"]'));
    evidence.stages.push({stage:'4b command before result',...queuedRead.row,workshop_id:workshopId,native_project:queuedNative,
      independent_cross_check:{endpoint:'GET /api/production queue[].finance (pre-S19)',project_id:workshopId,spent_bn:queuedFinance.spent_bn,expected:'0 before any paid day'}});

    mark('4 construction');
    const paidDays=[];let paid=null;
    for(let i=0;i<8&&!paid;i++){
      const after=await stepDay();const {native}=await nativeGuidance();
      const project=native.outcomes.construction.projects.find(p=>p.id===workshopId);
      paidDays.push({date:after.date,as_of_day:native.outcomes.as_of_day,project});
      if(project&&project.last_spent_bn>0)paid=project;
    }
    assert(paid,'The funded workshop must record paid work within a few days: '+JSON.stringify(paidDays));
    await openGuide('construction','f1');
    const con=await readRoute('construction');
    assert.equal(con.step('construction').status,'achieved','Construction must be achieved by paid work');
    const nativeProject=con.native.outcomes.construction.projects.find(p=>p.id===workshopId);
    assert(nativeProject.last_spent_bn>0&&nativeProject.last_day<=con.native.outcomes.as_of_day);
    const workMilestone=con.step('construction').milestones.find(m=>m.id==='work_paid');
    assert.equal(workMilestone.status,'done');assert.equal(workMilestone.day,nativeProject.last_day);assert(workMilestone.day<=con.native.outcomes.as_of_day);
    // Province name, never the raw id.
    const paidBoard=await observe('/api/production'),paidRow=paidBoard.queue.find(p=>p.id===workshopId);
    assert(paidRow,'GET /api/production must still list the workshop');
    assert.equal(paidRow.province.name,PROVINCE,'The pre-S19 production view names the province');
    assert.equal(nativeProject.district_name,PROVINCE,'Native outcomes serve the province name beside the id');
    const workRendered=con.renderedStep('construction').milestones.find(m=>m.cls.includes('done'));
    assert(workRendered.text.includes(PROVINCE),'The paid-work detail must name '+PROVINCE+': '+workRendered.text);
    assert(!workRendered.text.includes(district),'The paid-work detail must not show '+district);
    // B. Independent: pre-S19 production finance for the same project id.
    assert(paidRow.finance&&paidRow.finance.spent_bn>0,'GET /api/production finance.spent_bn must be positive for the paid workshop: '+JSON.stringify(paidRow.finance));
    assert(paidRow.finance.spent_bn+1e-12>=nativeProject.last_spent_bn,'Total spent must cover the latest payment');
    const constructionCross={endpoint:'GET /api/production queue[].finance (industry::project_finance, pre-S19)',project_id:workshopId,province:paidRow.province,
      spent_bn:paidRow.finance.spent_bn,cost_bn:paidRow.finance.cost_bn,remaining_bn:paidRow.finance.remaining_bn,guidance_last_spent_bn:nativeProject.last_spent_bn,guidance_spent_bn:nativeProject.spent_bn};
    check(`Stage 4: after visible +1 DAY construction is Achieved dated by the native project last_day; pre-S19 GET /api/production shows finance.spent_bn ${paidRow.finance.spent_bn} > 0 for project ${workshopId}; `
      +`the rendered detail reads "${workRendered.detail}" (province name, no raw id)`);
    await shot('route-construction-desktop',page.locator('#guidanceDialog [data-guidance-route-card="construction"]'));
    evidence.stages.push({stage:'4 construction',...con.row,district,daily_budget_bn:funded,workshop:{id:workshopId,quote_cost_bn:quote.cost_bn,capacity_micros:quote.capacity_micros??null},
      days:paidDays.map(d=>({date:d.date,last_day:d.project?.last_day??null,last_spent_bn:d.project?.last_spent_bn??null,spent_bn:d.project?.spent_bn??null})),
      native_project:nativeProject,milestone_date:workMilestone.date,rendered_detail:workRendered.detail,commands:[budgetPayload,workshopPayload],independent_cross_check:constructionCross});
    stop('4');

    // 5. Research/design: one valid saved draft through the designer.
    mark('5 research design');
    const draftsBefore=(await observe('/api/equipment?session_id='+encodeURIComponent(con.state.session_id))).designs.filter(d=>String(d.id).startsWith('draft:'));
    const toDesigner=await follow('research_design');
    await page.locator('#equipmentRoom').waitFor({state:'visible'});await designerReady();toDesigner.done();
    assert.equal(await page.evaluate(()=>EQUIP.tab),'designer','The research obstacle must open the designer');
    const recommendations=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.preview.guidance?.recommendations||[])));
    assert(recommendations.length>0,'The designer must offer a legal recommendation');
    const suggestion=recommendations.find(r=>r.id==='advanced')||recommendations[0];
    await tap('[data-equipment-recommendation-use='+q(suggestion.id)+']');
    if(await page.locator('[data-equipment-replacement]').isVisible())await tap('[data-equipment-replacement-accept]');
    await designerReady();
    const designName='S19 First Hour';await type('#equipmentName',designName);await designerReady();
    const chosen=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.draft)));
    assert.equal(chosen.platform,suggestion.spec.platform);assert.deepEqual(chosen.components,suggestion.spec.components);
    assert.equal(await page.evaluate(()=>EQUIP.preview.valid),true,'The recommendation must be a valid design');
    await tap('[data-equipment-action][data-equipment-scope="preview"] "Save design draft"',{locator:page.locator('[data-equipment-action][data-equipment-scope="preview"]').filter({hasText:'Save design draft'})});
    await page.locator('#equipmentActionReviewTitle').waitFor();
    const savePayload=await command(()=>tap('[data-equipment-confirm]'),'equipment_save');
    await designerReady();await page.locator('[data-equipment-draft-status="saved"]').waitFor();
    await shot('designer-saved-desktop',page.locator('[data-equipment-draft-status]'));
    await openGuide('research-design','f1');
    const res=await readRoute('research-design');
    assert.equal(res.step('research_design').status,'achieved','Research/design must be achieved by the saved valid draft');
    const draft=res.native.outcomes.research.drafts.find(d=>d.name===designName);
    assert(draft,'Native outcomes must list the saved draft');assert.equal(draft.valid,true);
    const saveMilestone=res.step('research_design').milestones.find(m=>m.id==='design_saved');
    assert.equal(saveMilestone.status,'done');assert.equal(saveMilestone.day,draft.updated_day);
    // B. Independent: the pre-S19 equipment designer list.
    const draftsAfter=(await observe('/api/equipment?session_id='+encodeURIComponent(res.state.session_id))).designs.filter(d=>String(d.id).startsWith('draft:'));
    const listed=draftsAfter.filter(d=>d.name===designName);
    assert.equal(listed.length,1,'GET /api/equipment designs must list the saved draft once');assert.equal(listed[0].status,'Draft');
    assert.equal(draftsAfter.length,draftsBefore.length+1,'Exactly one new draft');
    const designCross={endpoint:'GET /api/equipment?session_id=... designs (equipment_view.rs, pre-S19)',drafts_before:draftsBefore.length,drafts_after:draftsAfter.length,
      row:{id:listed[0].id,name:listed[0].name,status:listed[0].status,detail:listed[0].detail}};
    check(`Stage 5: the research obstacle opened the designer; a legal recommendation was applied and saved through the visible review; research_design is Achieved dated by the native valid draft updated_day; `
      +`pre-S19 GET /api/equipment designs lists ${listed[0].id} "${designName}" with status Draft (drafts ${draftsBefore.length} -> ${draftsAfter.length})`);
    await shot('route-research-desktop',page.locator('#guidanceDialog [data-guidance-route-card="research_design"]'));
    evidence.stages.push({stage:'5 research design',...res.row,recommendation:suggestion.id,design:{name:designName,platform:chosen.platform,components:chosen.components},
      native_draft:draft,native_research:{active:res.native.outcomes.research.active,learned:res.native.outcomes.research.learned,last_completed_day:res.native.outcomes.research.last_completed_day},
      milestone_date:saveMilestone.date,commands:[savePayload],independent_cross_check:designCross});
    stop('5');

    // 6. Air force: airbase foundation reviewed on the real Bases page.
    mark('6 air force');
    const basesBefore=(await observe('/api/equipment?session_id='+encodeURIComponent(res.state.session_id))).flight.bases.map(b=>({id:b.id,status:b.status}));
    const airObstacle=res.step('air_force').obstacle;assert.equal(airObstacle.actionLabel,'Review airbases');assert.deepEqual(airObstacle.action,{kind:'air',page:'bases'});
    const toAir=await follow('air_force');
    await page.locator('#equipmentRoom').waitFor({state:'visible'});
    await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='flight');toAir.done();
    const landed=await page.evaluate(()=>({tab:EQUIP.tab,page:EQUIP.flightPage,pressed:document.querySelector('[data-equipment-flight-page="bases"]')?.getAttribute('aria-pressed'),
      rendered:document.querySelector('.eq-flight-page')?.dataset.flightPage,selectedTab:document.querySelector('[role="tab"][data-equipment-tab="flight"]')?.getAttribute('aria-selected')}));
    assert.deepEqual(landed,{tab:'flight',page:'bases',pressed:'true',rendered:'bases',selectedTab:'true'},'Review airbases must land on Air command > Bases');
    assert(await page.locator('section[aria-label="Airbases"]').isVisible());
    await shot('air-bases-landing-desktop',page.locator('section[aria-label="Airbases"]'));
    const establish=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data.flight.base_actions||[])));
    const establishIndex=establish.findIndex(a=>a.command?.kind==='air_base'&&a.enabled!==false);assert(establishIndex>=0,'An airbase foundation action must be offered');
    const districtField=establish[establishIndex].inputs.find(f=>f.key==='district');
    const french=districtField.options.filter(o=>o.enabled!==false&&/ · France$/.test(o.label));assert(french.length>0,'A mapped French province must be offered');
    const baseDistrict=(french.find(o=>/le-de-france/i.test(String(o.value)))||french[0]).value;
    await tap('[data-equipment-action='+q('flight.base_actions.'+establishIndex)+']');
    await page.locator('#equipmentActionReviewTitle').waitFor();await reviewQuote();
    const districtInput=page.locator('[data-equipment-order-input="district"]');
    if(await districtInput.inputValue()!==String(baseDistrict)){await choose('[data-equipment-order-input="district"]',baseDistrict);await reviewQuote();}
    // The served default name follows the first listed province, not the chosen one, so name the base in the visible field.
    const defaultBaseName=await page.locator('[data-equipment-order-input="name"]').inputValue();
    const baseName=french.find(o=>o.value===baseDistrict).label.replace(/ · France$/,'')+' airbase';
    if(defaultBaseName!==baseName)await type('[data-equipment-order-input="name"]',baseName);
    const baseQuote=await reviewQuote();
    assert.equal(await page.locator('[data-equipment-order-input="name"]').inputValue(),baseName);
    assert.equal(baseQuote.valid,true,'Airbase review must be valid: '+JSON.stringify(baseQuote.blockers));
    const intent=baseQuote.actions.findIndex(a=>a.command?.kind==='air_base'&&a.enabled!==false);assert(intent>=0);
    await shot('air-foundation-review-desktop',page.locator('.eq-review'));
    const airPayload=await command(()=>tap('[data-equipment-intent='+q(intent)+']'),'air_base');

    // 6a. A: the airbase order before any paid day is not a result.
    mark('6a command before result');
    await openGuide('airbase-ordered','f1');const ordered=await readRoute('airbase-ordered');
    const orderedStep=ordered.step('air_force'),orderedMile=id=>orderedStep.milestones.find(m=>m.id===id);
    assert.notEqual(orderedStep.status,'achieved','An ordered, unpaid airbase cannot be achieved');assert.equal(orderedStep.status,'not_yet');
    assert.equal(orderedMile('base_funded').status,'pending');assert.equal(orderedMile('base_completed').status,'pending');
    assert.equal(orderedStep.obstacle.title,'Fund your airbase work');
    const orderedBase=ordered.native.outcomes.aviation.bases.find(b=>b.id===String(baseDistrict));
    assert(orderedBase&&orderedBase.project&&orderedBase.project.paid_bn===0,'The native base project exists unpaid: '+JSON.stringify(orderedBase));
    check('Stage 6a: right after the airbase confirm (no day advanced) air_force is Not yet with base_funded and base_completed pending and the obstacle "Fund your airbase work"');
    await shot('route-airbase-ordered',page.locator('#guidanceDialog [data-guidance-route-card="air_force"]'));
    evidence.stages.push({stage:'6a command before result',...ordered.row,base:{id:orderedBase.id,project:orderedBase.project}});

    mark('6 air force');
    const airDays=[];let completedBase=null,fundedRead=null;
    for(let i=0;i<40&&!completedBase;i++){
      const after=await stepDay();const {native}=await nativeGuidance();
      const base=native.outcomes.aviation.bases.find(b=>b.id===String(baseDistrict));
      airDays.push({date:after.date,as_of_day:native.outcomes.as_of_day,base:base?{id:base.id,project:base.project,history:base.history}:null});
      const done=base&&(base.history||[]).find(h=>Number.isInteger(h.completed_day))||(base?.project&&Number.isInteger(base.project.completed_day)?base.project:null);
      if(done){completedBase={base_id:base.id,name:base.name,completion:done};break;}
      // 6b. A: during the funded days the step is In progress, never achieved.
      if(!fundedRead&&base?.project?.paid_bn>0){
        await openGuide('airbase-funded','f1');const mid=await readRoute('airbase-funded');
        const s=mid.step('air_force'),m=id=>s.milestones.find(x=>x.id===id),project=mid.native.outcomes.aviation.bases.find(b=>b.id===String(baseDistrict)).project;
        assert.equal(s.status,'progress','Paid but unfinished airbase work is In progress');assert.notEqual(s.status,'achieved');
        assert.equal(m('base_funded').status,'done');assert.equal(m('base_funded').date,iso(project.last_paid_day));
        assert.equal(m('base_completed').status,'pending');assert.equal(s.obstacle.title,'Keep airbase work funded');
        assert(mid.renderedStep('air_force').campaign.includes('Campaign: In progress'));
        fundedRead={...mid.row,paid_bn:project.paid_bn,total_cost_bn:project.total_cost_bn,last_paid_day:project.last_paid_day,progress_days:project.progress_days,total_days:project.total_days};
        await shot('route-airbase-funded',page.locator('#guidanceDialog [data-guidance-route-card="air_force"]'));
      }
    }
    assert(fundedRead,'A funded-but-unfinished airbase day must have been read');
    assert(completedBase,'The airbase foundation must complete within 40 days: '+JSON.stringify(airDays.slice(-3)));
    evidence.stages.push({stage:'6b funded days',...fundedRead});
    check(`Stage 6b: on ${fundedRead.date}, after paid airbase work and before completion, air_force is In progress with base_funded Done (${fundedRead.steps.find(s=>s.id==='air_force').milestones[0].date}) and base_completed Not yet`);
    await openGuide('air-force','f1');
    const air=await readRoute('air-force');
    assert.equal(air.step('air_force').status,'achieved','Air force must be achieved by the completed airbase foundation');
    const baseMilestone=air.step('air_force').milestones.find(m=>m.id==='base_completed');
    assert.equal(baseMilestone.status,'done');assert.equal(baseMilestone.day,completedBase.completion.completed_day);
    const fundedMilestone=air.step('air_force').milestones.find(m=>m.id==='base_funded');
    assert.equal(fundedMilestone.status,'done');assert(fundedMilestone.date,'A completed airbase keeps a dated funding milestone');
    assert(fundedMilestone.day<=baseMilestone.day);
    assert.equal(air.renderedStep('air_force').milestones[0].date,fundedMilestone.date,'The funding milestone renders its date');
    // B. Independent: the pre-S19 Air command bases board.
    const basesAfter=(await observe('/api/equipment?session_id='+encodeURIComponent(air.state.session_id))).flight.bases;
    const board6=basesAfter.find(b=>b.id===String(baseDistrict));assert(board6,'GET /api/equipment flight.bases must list the new base');
    const spaces=board6.metrics.find(m=>m.label==='Aircraft spaces')?.value;
    assert(Number.isFinite(spaces)&&spaces>0,'The completed base must have capacity (capacity_level >= 1): '+JSON.stringify(board6.metrics));
    assert.notEqual(board6.status,'Improvement in progress');
    assert(!basesBefore.some(b=>b.id===String(baseDistrict)),'The base did not exist before the order');
    const airCross={endpoint:'GET /api/equipment?session_id=... flight.bases (equipment_flight_view.rs, pre-S19)',before:basesBefore,
      row:{id:board6.id,name:board6.name,status:board6.status,detail:board6.detail,aircraft_spaces:spaces},capacity_level_at_least_1:true};
    check(`Stage 6: Review airbases landed on Air command > Bases; the foundation was reviewed and confirmed; after visible +1 DAY air_force is Achieved dated by the native completed_day, `
      +`base_funded is dated ${fundedMilestone.date}; pre-S19 GET /api/equipment flight.bases lists ${board6.id} "${board6.name}" with ${spaces} aircraft spaces (status ${board6.status})`);
    await shot('route-air-desktop',page.locator('#guidanceDialog [data-guidance-route-card="air_force"]'));
    evidence.stages.push({stage:'6 air force',...air.row,landing:landed,base_district:baseDistrict,base_name:{served_default:defaultBaseName,confirmed:baseName},review:{valid:baseQuote.valid,metrics:baseQuote.metrics,costs:baseQuote.costs},
      days:airDays.map(d=>({date:d.date,as_of_day:d.as_of_day,paid_bn:d.base?.project?.paid_bn??null,last_paid_day:d.base?.project?.last_paid_day??null,completed:(d.base?.history||[]).map(h=>h.completed_day)})),
      completed:completedBase,milestone_date:baseMilestone.date,funded_milestone:{date:fundedMilestone.date,detail:fundedMilestone.detail},commands:[airPayload],independent_cross_check:airCross});
    stop('6');

    // 7. Procurement honesty: named obstacle, followed once, no purchase.
    mark('7 procurement');
    const procReading=await readRoute('procurement');const proc=procReading.step('procurement');assert.notEqual(proc.status,'achieved');assert(proc.obstacle);
    assert.deepEqual(procReading.route,air.route,'Re-reading the same day must give the same route');
    assert(/manufacturer/i.test(proc.obstacle.title+' '+proc.obstacle.reason),'Procurement obstacle must still name the manufacturer');
    const procOrders=orders().length;
    const toCompanies=await follow('procurement');
    await page.locator('#equipmentRoom').waitFor({state:'visible'});await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='companies');toCompanies.done();
    assert.equal(await page.locator('[role="tab"][data-equipment-tab="companies"]').first().getAttribute('aria-selected'),'true');
    assert.equal((await page.locator('[role="tab"][data-equipment-tab="companies"]').first().innerText()).trim(),'Companies & Procurement');
    await shot('procurement-companies-desktop',page.locator('#equipmentRoot'));
    assert.equal(orders().length,procOrders,'No purchase action may be taken');
    check('Stage 7: procurement stays Not yet with the manufacturer obstacle; following it opened Companies & Procurement; no purchase was made');
    evidence.stages.push({stage:'7 procurement',...procReading.row,procurement:{status:proc.status,obstacle:proc.obstacle,native:{companies:procReading.native.outcomes.procurement.companies,
      certified_products:procReading.native.outcomes.procurement.certified_products,deliveries:procReading.native.outcomes.procurement.deliveries.length,imports:procReading.native.outcomes.procurement.imports.length}},
      landed:'Companies & Procurement',purchase_actions:0});
    stop('7');

    // 8. A held real response is overtaken by a newer real day (close / advance / reopen path).
    mark('8 stale response');
    await closePanels();const held=await holdGuidance();
    try{
      guidanceWindow='open:held';await press('F1');
      await page.locator('#guidanceDialog').getByRole('heading',{name:'Your first steps in Spheres',exact:true}).waitFor();
      const old=await held.ready;assert(old.ok);
      const during=await page.evaluate(()=>({status:GUIDANCE.session.state().status,stale:GUIDANCE.session.state().routeStale,
        busy:document.querySelector('#guidanceDialog .guidance-route')?.getAttribute('aria-busy'),
        refresh:document.querySelector('#guidanceDialog [data-guidance-refresh]')?.textContent.trim()}));
      assert.equal(during.status,'loading');assert.equal(during.busy,'true');
      await tap('#guidanceDialog [data-guidance-close]');await page.locator('#guidanceDialog').waitFor({state:'hidden'});guidanceWindow=null;
      const advanced=await stepDay();assert.notEqual(advanced.date,old.date,'The newer reading must come from a later real day');
      await openGuide('after-held','f1');
      const acceptedFirst=await accepted();assert.equal(acceptedFirst.snapshot.date,advanced.date);
      await armDiscardWatch();
      held.release();await held.finish();
      // The held response really reaches the page, after the newer one was accepted.
      const heldResponse=await held.request().response();assert(heldResponse);await heldResponse.finished();
      const staleVerdict=await discardVerdict();
      const newer=await readRoute('after-held');
      assert.deepEqual(newer.route,acceptedFirst.route,'The late old response replaced the newer accepted route');
      assert.equal(newer.route.as_of.date,advanced.date);assert.notEqual(newer.route.as_of.date,old.date);
      assert.equal(newer.rendered.route_line,`Checked ${advanced.date}. Only dated records from this campaign count.`);
      check('Stage 8: one real /api/guidance response was held, the dialog was closed (cancelling that reading), a real day advanced through +1 DAY, the newer reading was accepted, and the held response was delivered afterwards (seen by a PerformanceObserver in the page) and discarded: the accepted route object was unchanged and the dialog was not repainted');
      await shot('route-after-stale-desktop',page.locator('#guidanceDialog .guidance-route'));
      evidence.stages.push({stage:'8 stale response',...newer.row,stale:{held:{date:old.date,session_id:old.session_id,as_of_day:old.as_of_day,delivered_to_page_after_newer:true,status:heldResponse.status(),
          resource_timing_seen_in_page:staleVerdict.resource_entries,route_object_unchanged:staleVerdict.same_route_object,dialog_mutations_after_release:staleVerdict.mutations},during_hold:during,
        accepted:{date:newer.route.as_of.date,as_of_day:newer.route.as_of.day},old_response_discarded:true,path:'dialog close cancels the ticket (guidance-ui.js close() -> session.cancel()); this covers only the ticket check',
        method:'F1 read held by page.route; dialog closed; one visible +1 DAY; F1 read accepted the newer day; the held real response was then released and delivered, and the accepted route stayed on the newer day'}});
    }finally{held.release();}

    // 8b. C: two readings inside the open dialog; the older one is held and must not replace the newer.
    mark('8b in-dialog race');
    const raceBase=await accepted();assert.equal(raceBase.status,'ready');
    const held2=await holdGuidance();let race;
    try{
      guidanceWindow='refresh:in-dialog-race';
      const refreshLabel=(await page.locator('#guidanceDialog [data-guidance-refresh]').innerText()).trim();assert.equal(refreshLabel,'Refresh results');
      const first=await tap('#guidanceDialog [data-guidance-refresh]');
      const firstBody=await held2.ready;assert(firstBody.ok);
      const during=await page.evaluate(()=>{const b=document.querySelector('#guidanceDialog [data-guidance-refresh]');
        return {status:GUIDANCE.session.state().status,routeStale:GUIDANCE.session.state().routeStale,refresh_disabled:!!b?.disabled,refresh_text:b?.textContent.trim(),
          busy:document.querySelector('#guidanceDialog .guidance-route')?.getAttribute('aria-busy')};});
      assert.equal(during.status,'loading');assert.equal(during.refresh_disabled,true,'The visible refresh is disabled while a reading is in flight');
      assert.equal(during.refresh_text,'Reading your campaign…');
      // The visible refresh cannot be clicked again while it is disabled; Playwright refuses the action.
      let refreshRefused=false;
      try{await page.locator('#guidanceDialog [data-guidance-refresh]').click({trial:true,timeout:1500});}catch(error){refreshRefused=/not enabled|disabled|Timeout/i.test(String(error));}
      assert.equal(refreshRefused,true,'A second click on the disabled refresh must be impossible');
      // F1 (the documented guidance shortcut) inside the open dialog starts a second real reading.
      const secondKey=await press('F1');
      await routeReady();
      const secondAccepted=await accepted();
      assert.equal(secondAccepted.snapshot.date,firstBody.date,'Both readings are of the same live day');
      await armDiscardWatch();
      const requestsBeforeRelease=evidence.guidance_requests.filter(r=>r.stage===stage).length;
      held2.release();await held2.finish();
      const heldResponse=await held2.request().response();assert(heldResponse);await heldResponse.finished();
      const after=await discardVerdict();
      assert.equal(requestsBeforeRelease,2,'Exactly two real guidance requests in the race');
      const raced=await readRoute('after-dialog-race');
      assert.deepEqual(raced.route,secondAccepted.route);
      race={first_control:first.selector,refresh_label_before:refreshLabel,during_hold:during,second_refresh_click_refused:true,second_control:{how:'key',key:'F1',focused:secondKey.selector},
        held:{date:firstBody.date,as_of_day:firstBody.as_of_day,status:heldResponse.status(),delivered_after_second_accepted:true,resource_timing_seen_in_page:after.resource_entries},
        accepted_is_second:{route_object_unchanged_after_release:true,dialog_mutations_after_release:0,as_of:secondAccepted.route.as_of},
        note:'Both readings are of the same day, so the accepted route is identified as the second by object identity: it was accepted while the first was still held, '
          +'and after the held response reached the page (PerformanceObserver) the accepted object was unchanged and the dialog was not repainted. guidance-ui.js drops it at the ticket check (line 137).'};
      check('Stage 8b: inside the open dialog one real reading from the visible "Refresh results" was held; that button then read "Reading your campaign…" and was disabled, so a second click was refused; '
        +'F1 inside the dialog started a second real reading which was accepted; the held response was then delivered to the page and did not replace the accepted route or repaint the dialog');
      evidence.stages.push({stage:'8b in-dialog race',...raced.row,race});
    }finally{held2.release();}
    stop('8');

    // 8c. C: a real later-day identity race. A second page in the same browser context advances a day with its own
    // visible +1 DAY while this page's reading request is held; the reading then arrives dated a day this page has not adopted.
    mark('8c later-day identity race');
    const identityBase=await accepted();assert.equal(identityBase.status,'ready');
    const dayBefore=identityBase.snapshot.date;
    const heldRequest=await holdGuidanceRequest();let identity;
    try{
      guidanceWindow='refresh:identity-race';
      await tap('#guidanceDialog [data-guidance-refresh]');await heldRequest.requested;
      assert.equal(await page.evaluate(()=>GUIDANCE.session.state().status),'loading');
      second=await context.newPage();second.setDefaultTimeout(45000);watch(second,'second');
      await second.goto(url,{waitUntil:'domcontentloaded'});await second.locator('#campaignHome').waitFor({state:'visible'});
      await second.waitForFunction(()=>!!SESSION.live?.session_id);
      await continueCampaign(second,'second');
      const secondBefore=await second.evaluate(()=>({session_id:S.session_id,date:S.date}));
      assert.deepEqual(secondBefore,{session_id:identityBase.snapshot.session_id,date:dayBefore},'The second page continues the same live campaign');
      const secondAfter=await stepDay(second,'second');assert.notEqual(secondAfter.date,dayBefore);
      await shot('identity-race-second-page',second.locator('#hdrDate'),'start',second);
      await heldRequest.finish();
      const lateResponse=await heldRequest.request().response();assert(lateResponse&&lateResponse.ok());
      const lateBody=await lateResponse.json();
      assert.equal(lateBody.state.date,secondAfter.date,'The held request was answered with the later real day');
      assert.equal(lateBody.state.session_id,identityBase.snapshot.session_id);
      await page.waitForFunction(()=>GUIDANCE.session.state().status!=='loading');
      const refused=await accepted(),rendered=await readRendered(),liveDate=await page.evaluate(()=>S.date);
      assert.equal(refused.status,'error','A reading dated after this page\'s campaign day must be refused');
      assert.equal(refused.notice,IDENTITY_NOTICE);
      assert.equal(refused.routeStale,true);assert.equal(refused.route.as_of.date,dayBefore,'The earlier accepted route is kept, labelled stale');
      assert.deepEqual(refused.route,identityBase.route,'The earlier route object is kept unchanged');
      assert.equal(refused.routeChecked,dayBefore);assert.equal(liveDate,dayBefore,'This page has not adopted the later day');
      assert.equal(rendered.route_line,STALE_LINE);assert.equal(rendered.refresh_label,`Last checked ${dayBefore} — refresh`);
      assert(rendered.steps.every(s=>!s.obstacle||s.obstacle.button.disabled),'Stale obstacle buttons are disabled');
      assert(rendered.steps.every(s=>s.campaign.startsWith(`Campaign: Last checked ${dayBefore}`)||s.campaign.includes(`Last checked ${dayBefore}`)),'Campaign chips carry the earlier date');
      const notice=(await page.locator('#guidanceDialog .guidance-notice').innerText()).trim();assert.equal(notice,IDENTITY_NOTICE);
      await shot('identity-race-refused',page.locator('#guidanceDialog .guidance-body'));
      await shot('identity-race-stale-route',page.locator('#guidanceDialog .guidance-route'));
      guidanceWindow=null;
      await second.close();second=null;
      // Follow the notice: Campaigns, Continue, then read again.
      await campaignHome();await continueCampaign();
      assert.equal(await page.evaluate(()=>S.date),secondAfter.date,'Continue adopts the later day');
      await openGuide('after-identity-race','f1');const resynced=await readRoute('after-identity-race');
      assert.equal(resynced.route.as_of.date,secondAfter.date);
      identity={day_before:dayBefore,second_page_day:secondAfter.date,held_request_answered:{date:lateBody.state.date,session_id:lateBody.state.session_id,status:lateResponse.status()},
        refused:{status:refused.status,notice:refused.notice,routeStale:refused.routeStale,kept_route_as_of:refused.route.as_of,route_line:rendered.route_line,refresh_label:rendered.refresh_label,
          obstacle_buttons_disabled:true,page_date:liveDate},resync:{control:'#campaignsBtn then #continueBtn',accepted_date:resynced.route.as_of.date},
        note:'guidance-ui.js identity check (lines 139-142) refused a real reading dated after the page\'s own state; the held request was forwarded unchanged after the second page\'s visible +1 DAY.'};
      check(`Stage 8c: a second page in the same browser context continued the campaign and advanced ${dayBefore} -> ${secondAfter.date} with its own visible #stepBtn while this page's /api/guidance request was held; `
        +'the real answer (dated '+lateBody.state.date+') was refused with "'+IDENTITY_NOTICE+'", the earlier route stayed labelled stale with disabled obstacles, and Campaigns + Continue then read the later day');
      evidence.stages.push({stage:'8c later-day identity race',...resynced.row,identity});
    }finally{heldRequest.release();if(second){await second.close().catch(()=>{});second=null;}}

    // 9. Named save and confirmed load, both reached through the save_resume obstacle; reload + Continue.
    mark('9 save and resume');
    const beforeSaveRead=await readRoute('before-save');
    const achievedBeforeSave=Object.fromEntries(beforeSaveRead.route.steps.map(s=>[s.id,{status:s.status,milestones:s.milestones}]));
    assert.equal(beforeSaveRead.step('save_resume').obstacle.actionLabel,'Open campaigns');
    const beforeSave=await observe('/api/state');
    const toSave=await follow('save_resume');await page.locator('#campaignHome').waitFor({state:'visible'});toSave.done();
    const saveLanding=await page.evaluate(()=>({campaign_home:getComputedStyle(document.querySelector('#campaignHome')).display!=='none',app_hidden:getComputedStyle(document.querySelector('#app')).display==='none'}));
    assert.deepEqual(saveLanding,{campaign_home:true,app_hidden:true},'"Open campaigns" must land on the campaigns screen');
    await shot('campaigns-from-obstacle',page.locator('#campaignHome'));
    await tap('#openSavesBtn');await page.locator('#savedCampaigns').waitFor({state:'visible'});
    await type('#saveName',SLOT);
    const saveWait=page.waitForResponse(r=>isRoute(r,'/api/save'));await tap('#saveNamedBtn');
    const saved=await saveWait;assert(saved.ok());assert.equal(saved.request().postDataJSON().slot,SLOT);
    await page.locator('#saveSlots option[value='+q(SLOT)+']').waitFor({state:'attached'});
    const saveReceipt=(await storedReceipts()).raw;
    assert(saveReceipt&&saveReceipt.saves.some(r=>r.slot===SLOT&&r.session_id===beforeSave.session_id&&r.date===beforeSave.date&&r.player==='France'),'The native save must be recorded as a guidance receipt');
    await tap('#savedCampaigns [data-menu-back]',{locator:page.locator('#savedCampaigns [data-menu-back]').first()});await continueCampaign();
    assert.equal((await observe('/api/state')).session_id,beforeSave.session_id,'Continue keeps the live session');
    await openGuide('saved','f1');
    const savedRoute=await readRoute('saved');
    const sr=savedRoute.step('save_resume');assert.equal(sr.status,'progress','A save alone is In progress');
    assert.equal(sr.milestones.find(m=>m.id==='campaign_saved').status,'done');assert.equal(sr.milestones.find(m=>m.id==='campaign_resumed').status,'pending');
    assert.equal(sr.obstacle.title,'Resume from your save');
    check('Stage 9a: the save_resume obstacle "Open campaigns" landed on the campaigns screen; the visible named save stored a receipt; save_resume is In progress with the saved milestone done');
    await shot('route-saved-desktop',page.locator('#guidanceDialog [data-guidance-route-card="save_resume"]'));
    evidence.stages.push({stage:'9a named save',...savedRoute.row,slot:SLOT,obstacle_control:toSave.control.selector,landing:saveLanding,save_receipt:saveReceipt.saves.find(r=>r.slot===SLOT)});

    const toLoad=await follow('save_resume');await page.locator('#campaignHome').waitFor({state:'visible'});toLoad.done();
    await tap('#openSavesBtn');await page.locator('#savedCampaigns').waitFor({state:'visible'});
    await choose('#saveSlots',SLOT);
    await tap('#loadBtn');await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
    assert((await page.locator('#campaignConfirmMessage').innerText()).includes(SLOT));
    const loadWait=page.waitForResponse(r=>isRoute(r,'/api/load'));await tap('#campaignConfirmAccept');
    const loadedResponse=await loadWait;assert(loadedResponse.ok());assert.equal(loadedResponse.request().postDataJSON().slot,SLOT);
    await page.locator('#app').waitFor({state:'visible'});await idle(page);
    const loaded=await observe('/api/state');assert.notEqual(loaded.session_id,beforeSave.session_id,'Load starts a new session');assert.equal(loaded.date,beforeSave.date);
    const loadReceipt=(await storedReceipts()).raw;
    assert(loadReceipt.loads.some(r=>r.session_id===loaded.session_id&&r.slot===SLOT&&r.date===loaded.date&&r.backup===false),'The native load must be recorded as a guidance receipt');
    await openGuide('loaded','f1');
    const loadedRoute=await readRoute('loaded');
    const lr=loadedRoute.step('save_resume');assert.equal(lr.status,'achieved','Loading the save achieves save_resume');
    assert.equal(lr.milestones.find(m=>m.id==='campaign_resumed').date,iso(loadedRoute.native.outcomes.as_of_day));
    for(const id of ['finances','construction','research_design','air_force']){
      assert.equal(loadedRoute.step(id).status,'achieved',id+' must be re-derived as achieved under the loaded session');
      assert.deepEqual(loadedRoute.step(id).milestones,achievedBeforeSave[id].milestones,id+' milestones must re-derive identically from native records');
    }
    check('Stage 9b: the save_resume obstacle "Open campaigns" led to Load + confirm, which started a new session on the same date; save_resume is Achieved and finances, construction, research_design and air_force re-derive identically from native records');
    await shot('route-loaded-desktop',page.locator('#guidanceDialog .guidance-route'));
    evidence.stages.push({stage:'9b confirmed load',...loadedRoute.row,obstacle_control:toLoad.control.selector,previous_session:beforeSave.session_id,load_receipt:loadReceipt.loads.find(r=>r.session_id===loaded.session_id),
      rederived:['finances','construction','research_design','air_force']});

    await closePanels();await page.reload({waitUntil:'domcontentloaded'});
    await page.locator('#continueBtn').waitFor({state:'visible'});await continueCampaign();
    assert.equal((await observe('/api/state')).session_id,loaded.session_id,'Continue after reload keeps the loaded session');
    await openGuide('reload-continue','tutorial');
    const continued=await readRoute('reload-continue');
    assert.equal(continued.step('save_resume').status,'achieved','save_resume stays achieved after reload + Continue');
    assert.deepEqual(continued.route.steps,loadedRoute.route.steps,'Reload + Continue re-derives the same route');
    check('Stage 9c: page reload + Continue kept the loaded session and save_resume stays Achieved');
    evidence.stages.push({stage:'9c reload continue',...continued.row});
    stop('9');

    // 10a. E: Escape closes the dialog and focus returns to the control that had it.
    mark('10a keyboard escape');
    await closePanels();const escapeOrders=orders().length;
    await page.locator('#stepBtn').focus();assert.equal(await describe(page),'#stepBtn');
    guidanceWindow='open:escape';const f1=await press('F1');
    await page.locator('#guidanceDialog').getByRole('heading',{name:'Your first steps in Spheres',exact:true}).waitFor();await routeReady();
    const focusInside=await describe(page);
    const escape=await press('Escape');await page.locator('#guidanceDialog').waitFor({state:'hidden'});guidanceWindow=null;
    const focusAfter=await describe(page);
    assert.equal(focusAfter,'#stepBtn','Escape must return focus to the control that opened guidance');
    assert.equal(orders().length,escapeOrders,'F1 and Escape issued no order');
    check('Stage 10a: with focus on #stepBtn, F1 opened guidance (focus moved to '+focusInside+'), Escape closed it and focus returned to #stepBtn; no order');
    evidence.stages.push({stage:'10a keyboard escape',opened_with:{key:'F1',focused:f1.selector},focus_in_dialog:focusInside,closed_with:{key:'Escape',focused:escape.selector},focus_after:focusAfter,orders_issued:0});

    // 10. Narrow layout and keyboard-reachable native buttons (final state).
    mark('10 narrow');
    await openGuide('narrow','f1');
    const narrow=await readRoute('narrow');
    const narrowFinal=await narrowMeasure();
    const order=await page.evaluate(()=>{const lesson=document.querySelector('#guidanceDialog .guidance-lesson'),routePanel=document.querySelector('#guidanceDialog .guidance-route');
      return {following:!!(lesson.compareDocumentPosition(routePanel)&Node.DOCUMENT_POSITION_FOLLOWING),lessonTop:lesson.getBoundingClientRect().top,routeTop:routePanel.getBoundingClientRect().top,
        completeTop:document.querySelector('#guidanceDialog [data-guidance-complete]').getBoundingClientRect().top};});
    assert(order.following&&order.lessonTop<order.routeTop&&order.completeTop<order.routeTop,'The lesson and its controls must render before the route panel: '+JSON.stringify(order));
    const buttons=await page.evaluate(()=>[...document.querySelectorAll('#guidanceDialog [data-guidance-route-step],#guidanceDialog [data-guidance-refresh]')].map(b=>{b.focus();
      return {tag:b.tagName,type:b.type,step:b.dataset.guidanceRouteStep||'refresh',tabIndex:b.tabIndex,disabled:b.disabled,focused:document.activeElement===b,width:b.getBoundingClientRect().width};}));
    assert(buttons.length>=2);for(const b of buttons){assert.equal(b.tag,'BUTTON');assert.equal(b.type,'button');assert(b.tabIndex>=0);assert.equal(b.disabled,false);assert(b.focused,'Route button must take focus: '+b.step);}
    await page.locator('#guidanceDialog [data-guidance-close]').focus();
    await shot('narrow-lesson-top',page.locator('#guidanceDialog .guidance-intro'));
    await shot('narrow-lesson-controls',page.locator('#guidanceDialog .guidance-lesson-copy'));
    await shot('narrow-lesson-actions',page.locator('#guidanceDialog .guidance-lesson .guidance-actions'),'center');
    await shot('narrow-route-panel',page.locator('#guidanceDialog .guidance-route'));
    await shot('narrow-route-air-save',page.locator('#guidanceDialog [data-guidance-route-card="air_force"]'));
    await page.locator('#guidanceDialog [data-guidance-close]').focus();let tabbed=null,presses=0;
    for(;presses<120&&!tabbed;presses++){await page.keyboard.press('Tab');tabbed=await page.evaluate(()=>document.activeElement?.dataset?.guidanceRouteStep||null);}
    assert(tabbed,'Keyboard Tab must reach a route obstacle button');
    check('Stage 10: at 390x844 (final state, '+narrowFinal.obstacles+' obstacle) the dialog, content, lesson, route panel and cards have no horizontal overflow; the lesson and its controls render before the route panel; route buttons are native focusable buttons reached by Tab');
    evidence.stages.push({stage:'10 narrow',...narrow.row,narrow_final:narrowFinal,order,buttons,keyboard_tab_reached:{step:tabbed,presses}});
    await closePanels();await page.setViewportSize({width:1440,height:1000});
    await openGuide('final-desktop','f1');
    const final=await readRoute('final-desktop');
    await shot('route-final-desktop',page.locator('#guidanceDialog .guidance-route'));
    await shot('route-final-desktop-steps',page.locator('#guidanceDialog [data-guidance-route-card="air_force"]'));
    evidence.stages.push({stage:'10 final desktop',...final.row});

    // 10c. D: Restart lessons resets reading progress only.
    mark('10c restart lessons');
    const restartOrders=orders().length,progressBefore=await storedProgress();
    assert(progressBefore.done.includes('budget-treasury')&&progressBefore.skipped.includes('construction-effects'));
    assert.equal(final.renderedStep('finances').reading,'Reading: Read');assert.equal(final.renderedStep('construction').reading,'Reading: Skipped');
    assert.equal((await page.locator('#guidanceDialog [data-guidance-restart]').innerText()).trim(),'Restart lessons');
    await tap('#guidanceDialog [data-guidance-restart]');
    await page.waitForFunction(()=>{const p=GUIDANCE.session.state().progress;return p.done.length===0&&p.skipped.length===0;});
    const restarted=await readRoute('after-restart');
    assert(restarted.rendered.steps.every(s=>s.reading==='Reading: Not read'),'Every reading chip resets: '+JSON.stringify(restarted.rendered.steps.map(s=>s.reading)));
    assert.deepEqual(restarted.route,final.route,'Restarting lessons must not change campaign results');
    assert.equal(restarted.rendered.results_line,final.rendered.results_line);
    const progressAfter=await storedProgress();assert.deepEqual([progressAfter.done,progressAfter.skipped],[[],[]]);
    assert.equal(orders().length,restartOrders,'Restart lessons issued an order');
    check('Stage 10c: "Restart lessons" reset every reading chip to Not read (finances was Read, construction Skipped) while the campaign route and "'+final.rendered.results_line+'" stayed unchanged; no order');
    await shot('route-after-restart',page.locator('#guidanceDialog .guidance-route'));
    evidence.stages.push({stage:'10c restart lessons',...restarted.row,readings_before:final.rendered.steps.map(s=>[s.id,s.reading]),readings_after:restarted.rendered.steps.map(s=>[s.id,s.reading]),
      progress_before:progressBefore,progress_after:progressAfter,route_unchanged:true,orders_issued:0});
    stop('10');

    // 11. G: a new campaign inherits nothing.
    mark('11 new campaign');
    const oldReceipts=(await storedReceipts()).receipts;assert(oldReceipts.saves.length>0&&oldReceipts.loads.length>0);
    await campaignHome();await tap('#newCampaignBtn');await page.locator('#newCampaignPicker').waitFor({state:'visible'});
    await tap('#nationPick [aria-label^="France;"]');await tap('#startBtn');
    await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
    const newMessage=(await page.locator('#campaignConfirmMessage').innerText()).trim();
    const newWait=page.waitForResponse(r=>isRoute(r,'/api/new'));await tap('#campaignConfirmAccept');
    assert((await newWait).ok(),'New campaign failed');
    await page.locator('#app').waitFor({state:'visible'});await idle(page);
    const renewed=await observe('/api/state');assert.equal(renewed.player,'France');assert.equal(renewed.date,opening.date);
    const earlierSessions=[opening.session_id,beforeSave.session_id,loaded.session_id];
    assert(!earlierSessions.includes(renewed.session_id),'A new campaign has a new session');
    assert(![...oldReceipts.saves,...oldReceipts.loads].some(r=>r.session_id===renewed.session_id));
    assert.equal(await page.evaluate(()=>clock.running),false);
    await openGuide('new-campaign','f1');const fresh=await readRoute('new-campaign');
    for(const step of fresh.route.steps){assert.equal(step.status,'not_yet',step.id+' must not be inherited by a new campaign');assert(step.obstacle);
      assert(step.milestones.every(m=>m.status!=='done'),step.id+' has no done milestone in a new campaign');}
    const newSave=fresh.step('save_resume');
    assert.deepEqual(newSave.milestones.map(m=>[m.id,m.status]),[['campaign_saved','pending'],['campaign_resumed','pending']]);
    assert.equal(fresh.rendered.results_line,'Campaign results: 0 of 6 achieved · see Your first hour below');
    const heldReceipts=(await storedReceipts()).receipts;assert.deepEqual(heldReceipts,oldReceipts,'The browser still holds the earlier receipts');
    check(`Stage 11: Campaigns > New campaign > France > Start > confirm started session ${renewed.session_id} on ${renewed.date}; all six steps are Not yet with no done milestone and save_resume is Not yet `
      +`although the browser still holds ${oldReceipts.saves.length} save and ${oldReceipts.loads.length} load receipts from earlier sessions`);
    await shot('route-new-campaign',page.locator('#guidanceDialog .guidance-route'));
    evidence.stages.push({stage:'11 new campaign',...fresh.row,confirm_message:newMessage,earlier_sessions:earlierSessions,receipts_held:{saves:oldReceipts.saves.length,loads:oldReceipts.loads.length}});
    stop('11');

    // 12. Every order came from a logged visible control; guidance issued none.
    mark('12 requests');
    assert.deepEqual(evidence.errors,[],'No page errors');
    const guided=evidence.requests.filter(r=>r.guidance_window!==null&&r.route!=='/api/save'&&r.route!=='/api/load');
    assert.deepEqual(guided,[],'Guidance itself issued no request');
    for(const r of evidence.requests)assert(r.control,'Every order request follows a logged action: '+JSON.stringify(r));
    const commands=evidence.requests.filter(r=>r.route==='/api/command');
    assert.deepEqual(commands.flatMap(r=>r.kinds),['construction_budget','start_industry_module','equipment_save','air_base'],'Only the four visible review confirmations sent commands');
    const commandControl={construction_budget:s=>s==='#constructionBudgetForm button[type="submit"]',start_industry_module:s=>s==='[data-construction-confirm]',
      equipment_save:s=>s==='[data-equipment-confirm]',air_base:s=>s.startsWith('[data-equipment-intent=')};
    for(const r of commands)assert(r.control.how==='click'&&commandControl[r.kinds[0]](r.control.selector),'Unexpected control for '+r.kinds[0]+': '+JSON.stringify(r.control));
    const advances=evidence.requests.filter(r=>r.route==='/api/advance');
    for(const r of advances){assert.equal(r.payload.days,1);assert(r.control.how==='click'&&['#cabinetEnact','#stepBtn'].includes(r.control.selector),'Unexpected advance control: '+JSON.stringify(r.control));}
    assert.equal(advances.filter(r=>r.control.selector==='#cabinetEnact').length,1);
    assert.equal(advances.filter(r=>r.page==='second').length,1,'One advance from the second page');
    const saves=evidence.requests.filter(r=>r.route==='/api/save'),loads=evidence.requests.filter(r=>r.route==='/api/load'),news=evidence.requests.filter(r=>r.route==='/api/new');
    assert.deepEqual(saves.map(r=>r.control.selector),['#saveNamedBtn'],'One visible named save');
    assert.deepEqual(loads.map(r=>r.control.selector),['#campaignConfirmAccept'],'One visible confirmed load');
    assert.deepEqual(news.map(r=>r.control.selector),['#startBtn','#campaignConfirmAccept'],'Two visible new campaigns');
    assert.equal(evidence.archive_saves.length,4,'Four archive snapshots');
    check(`Stage 12: no page errors; the only commands were construction_budget, start_industry_module, equipment_save and air_base, each sent right after its logged review-confirm click; `
      +`${advances.length} one-day advances, each right after a logged #cabinetEnact (1) or #stepBtn click (${advances.filter(r=>r.page==='second').length} on the second page); `
      +`POST /api/save ${saves.length+evidence.archive_saves.length} in total (1 visible #saveNamedBtn + ${evidence.archive_saves.length} page.request archive snapshots); guidance issued no request`);
    const controlCount=rows=>rows.reduce((m,r)=>{const k=`${r.page}:${r.control.how}:${r.control.selector}`;m[k]=(m[k]||0)+1;return m;},{});
    evidence.request_summary={
      commands:commands.map(r=>({seq:r.seq,stage:r.stage,kinds:r.kinds,control:r.control})),
      advances:{total:advances.length,by_observed_control:controlCount(advances),list:advances.map(r=>({seq:r.seq,stage:r.stage,page:r.page,days:r.payload.days,kinds:r.kinds,control:r.control.selector}))},
      post_api_save:{total:saves.length+evidence.archive_saves.length,visible:saves.map(r=>({stage:r.stage,control:r.control.selector,slot:r.payload?.slot})),
        archive_snapshots:evidence.archive_saves,archive_note:'page.request POST /api/save snapshots bypass the page api(), so they are not guidance receipts; the save handler reads the world under a read lock'},
      post_api_load:loads.map(r=>({stage:r.stage,control:r.control.selector,slot:r.payload?.slot})),
      post_api_new:news.map(r=>({stage:r.stage,control:r.control.selector,nation:r.payload?.nation})),
      observer_requests:{total:evidence.observer_requests.length,gets:evidence.observer_requests.filter(r=>r.method==='GET').length,posts:evidence.observer_requests.filter(r=>r.method==='POST').length},
      guidance_requests:{total:evidence.guidance_requests.length,by_window:evidence.guidance_requests.reduce((m,r)=>{m[r.guidance_window||'none']=(m[r.guidance_window||'none']||0)+1;return m;},{})},
      guidance_issued:0};

    mark('13 result');
    const binaryAfter=await fileHash(binary);assert.equal(binaryAfter,binaryBefore,'The binary changed during the run');
    assert.equal(git(['rev-parse','HEAD']).trim(),head);
    evidence.binary={path:binary,sha256_before:binaryBefore,sha256_after:binaryAfter};
    evidence.scope_notes=[
      'Procurement is expected to stay Not yet in the first hour: companies sell only certified designs (at least 180 days of ground or 240 days of air development) and no foreign import was attempted.',
      'Construction is achieved by paid work only; project completion and site output (about 182 days for this workshop) were not reached.',
      'The work_paid milestone is dated by the native last_day of the latest payment, so its displayed date moves forward while work continues.',
      'Air force is achieved by the completed airbase foundation; squadron formation, readiness and missions need delivered aircraft and were not exercised.',
      'Inside the open dialog the visible refresh is disabled while a reading is in flight, so the second in-dialog reading (8b) was started with F1, the documented guidance shortcut, which re-runs open("tutorial") and refresh().',
      'The fresh Advisors view drops route-next by design because the annual-budget card already opens the same budget screen; route-next was asserted and followed after the budget (3b).',
      'Independent cross-checks read pre-S19 endpoints (cash-flow money view, production finance, equipment designs, flight bases). They come from the same world state, so they prove agreement between S19 and older readers, not a second simulation.'];
    evidence.passed=true;evidence.finished_utc=new Date().toISOString();write('result.json',evidence);
    console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){
    evidence.failed_stage=stage;evidence.failure=error.stack||String(error);evidence.stopped=!!error.stopped;
    try{evidence.binary={path:binary,sha256_before:binaryBefore,sha256_after:await fileHash(binary)};}catch(_){}
    write('result.json',evidence);
    if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch(_){}
    console.error('S19 guidance evidence: '+path.join(out,'result.json'));if(!error.stopped)throw error;
  }finally{
    if(browser)await browser.close().catch(()=>{});
    // Kill only the server this driver started.
    if(server.exitCode===null)server.kill();log.end();
  }
})().catch(error=>{console.error(error);process.exitCode=1;});
