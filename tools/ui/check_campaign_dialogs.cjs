// Actual presentation functions with a minimal dialog/focus surface; no browser.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const transport=require('../../spheres-web/ui/campaign-transport.js');
const source=name=>fs.readFileSync(path.join(__dirname,'../../spheres-web/ui',name),'utf8');

function decisionFixture(){
  const elements=new Map(),listeners=new Map(),routes=[];
  const node=id=>{if(!elements.has(id))elements.set(id,{id,value:'',innerHTML:'',textContent:'',focus(){},addEventListener(kind,fn){this['on'+kind]=fn;},insertAdjacentHTML(where,html){assert.equal(where,'beforeend');this.innerHTML+=html;}});return elements.get(id);};
  const box={open:false,attrs:{},querySelector:node,querySelectorAll:()=>[],
    setAttribute(k,v){this.attrs[k]=v;},addEventListener(k,fn){listeners.set(k,fn);},
    showModal(){this.open=true;},close(){this.open=false;listeners.get('close')?.();},
    set innerHTML(value){node('.decision-body').innerHTML=value;},get innerHTML(){return node('.decision-body').innerHTML;}};
  let attached=false;
  const document={activeElement:{focus(){}},getElementById:()=>attached?box:null,
    createElement:()=>box,body:{append(){attached=true;}}};
  const c=vm.createContext({window:{},document,clockPause(){},S:{session_id:'one',player:'USA',programs:{due:true}},tech:{data:[]},
    constructionMoney:value=>Number.isFinite(value)?`$${value}bn`:'—',
    openConstruction:options=>routes.push(['construction',JSON.parse(JSON.stringify(options||{}))]),
    openConstructionCabinet:tab=>routes.push(['cabinet',tab]),
    openIndustry:()=>routes.push(['industry']),
    constructionPreviewProject:async(kind,district,size)=>{routes.push(['preview',kind,district,size]);return true;},
    openNation:id=>routes.push(['nation',id]),selectNationView:view=>routes.push(['nation-view',view])});
  vm.runInContext(source('decision-tools.js'),c);
  c.box=box;c.body=()=>node('.decision-body');c.routes=routes;return c;
}
test('late Advisor success cannot replace a newly opened Research dialog',async()=>{
  const c=decisionFixture();let finish;
  c.api=()=>new Promise(resolve=>finish=resolve);
  const pending=c.window.openAdvisor();c.box.close();c.window.openResearchList();
  const before=c.body().innerHTML;
  finish({queue:[]});await pending;
  assert.equal(c.box.attrs['aria-label'],'Research decisions');
  assert.equal(c.body().innerHTML,before);assert(!c.body().innerHTML.includes('advisorNext'));
});
test('late Advisor and About failures cannot replace newer dialog content',async()=>{
  for(const open of ['openAdvisor','openBuildInfo']){
    const c=decisionFixture();let reject;
    c.api=()=>new Promise((_,fail)=>reject=fail);
    const pending=c.window[open]();c.box.close();c.window.openResearchList();
    const before=c.body().innerHTML;reject(Error('old request failed'));await pending;
    assert.equal(c.body().innerHTML,before,open);
  }
});
test('late About success cannot replace newer dialog content',async()=>{
  const c=decisionFixture();let finish;c.api=()=>new Promise(resolve=>finish=resolve);
  const pending=c.window.openBuildInfo();c.window.openResearchList();const before=c.body().innerHTML;
  finish({version:'test',built_at_unix_seconds:0});await pending;assert.equal(c.body().innerHTML,before);
});
test('advice does not mix old projects with a newer adopted campaign state',async()=>{
  const c=decisionFixture();let finish;c.api=()=>new Promise(resolve=>finish=resolve);
  const pending=c.window.openAdvisor();c.S={...c.S,date:'2 Jan 1990'};finish({queue:[]});await pending;
  assert(c.body().innerHTML.includes('The campaign changed while advice was loading'));
  assert(!c.body().innerHTML.includes('advisorNext'));
});

test('actual Advisor renders stability and opens only explicitly chosen policy rooms',async()=>{
  for(const [action,target] of [['budget','budget'],['world','intelDrawer'],['decisions','agency']]){
    const c=decisionFixture(),button={dataset:{stabilityAction:action}},calls=[];
    c.api=async()=>({queue:[]});c.openConstructionCabinet=tab=>calls.push(tab);c.toggleGameDrawer=id=>calls.push(id);c.openAgency=()=>calls.push('agency');
    c.S.policy={stability:{monthly_points_before_bounds:-.1,month_fraction:1,terms:[{label:'Current pressure',monthly_points:-.1,action}]}};
    c.box.querySelectorAll=selector=>selector==='[data-stability-action]'?[button]:[];
    await c.window.openAdvisor();
    assert.match(c.body().innerHTML,/Current economic stability pressure/);assert.match(c.body().innerHTML,/-0\.1000 stability points per month/);
    assert.deepEqual(calls,[]);assert.equal(c.box.open,true);
    button.onclick();assert.deepEqual(calls,[target]);assert.equal(c.box.open,false);
  }
});

test('Advisor leads with exact server suggestion effects and never starts work when following it',async()=>{
  for(const [kind,size] of [['power_grid',undefined],['starter_industry',5001]]){
    const c=decisionFixture();c.S.programs={enabled:true};const reads=[];
    c.api=async url=>{reads.push(url);return {queue:[{id:9,status:'building'}],completed:[{}],suggestions:{items:[{
      id:'top',project_kind:kind,district:'US-CA',district_name:'California',capacity_micros:size,name:'Suggested project',priority:'Bottleneck',
      reason:'Grid capacity is limiting existing output.',cost_bn:.025,minimum_days:90,eta_days:125,evidence:['An exact served observation.'],caution:'Operating funds are still needed.'}]}};};
    await c.window.openAdvisor();const html=c.body().innerHTML;
    for(const text of ['Your next step','Suggested project','California','Grid capacity is limiting existing output.','$0.025bn total project cost','at least 90 days','125 days at current funding','An exact served observation.','Operating funds are still needed.','Review suggested effects'])assert(html.includes(text),text);
    assert.deepEqual(c.routes,[]);await c.box.querySelector('#advisorNext').onclick();
    assert.deepEqual(c.routes,[['construction',{}],['preview',kind,'US-CA',size]]);
    assert.deepEqual(reads,['/api/production'],'Following advice only opens the existing read-only effects review');
  }
});
test('Advisor budget and growth actions reach explicit tabs regardless of the previous Cabinet view',async()=>{
  const c=decisionFixture();c.S.policy={war:.02};c.api=async()=>({queue:[]});
  await c.window.openAdvisor();assert.match(c.body().innerHTML,/Renew your yearly budget/);
  c.box.querySelector('#advisorNext').onclick();assert.deepEqual(c.routes,[['cabinet','budget']]);
  await c.window.openAdvisor();c.box.querySelector('#advisorCauses').onclick();
  assert.deepEqual(c.routes,[['cabinet','budget'],['cabinet','policy']]);
});
test('Advisor keeps a suggestion open while the same world has a pending turn or construction order',async()=>{
  for(const hold of [c=>c.advancing=true,c=>c.pendingAdvance={payload:{}},c=>c.PROD={busy:true}]){
    const c=decisionFixture();c.S.programs={enabled:true};
    c.api=async()=>({suggestions:{items:[{id:'grid',name:'Grid',project_kind:'power_grid',district:'US-CA'}]}});
    await c.window.openAdvisor();const state=c.S;hold(c);
    assert.equal(await c.box.querySelector('#advisorNext').onclick(),false);
    assert.equal(c.S,state);assert.equal(c.box.open,true);assert.deepEqual(c.routes,[]);
    assert.match(c.box.querySelector('#advisorStatus').textContent,/current turn or order to finish/);
    c.advancing=false;c.pendingAdvance=null;c.PROD={busy:false};
    assert.equal(await c.box.querySelector('#advisorNext').onclick(),true);
    assert.deepEqual(c.routes,[['construction',{}],['preview','power_grid','US-CA',undefined]]);
    assert.equal(c.box.open,false);
  }
});
test('Advisor diagnostics remain optional and paid project progress keeps its first decimal',async()=>{
  const c=decisionFixture();c.S.programs={enabled:true};c.S.policy={war:.01,stability:{monthly_points_before_bounds:0,terms:[]}};
  c.api=async()=>({queue:[{id:7,name:'Grid',province:{name:'California'},status:'building',progress:.00125,eta_days:180,finance:{}}]});
  await c.window.openAdvisor();const html=c.body().innerHTML;
  assert.match(html,/0\.1% complete/);assert.match(html,/<details class="advisor-diagnostics"><summary>Economic pressures and stability/);
  assert.doesNotMatch(html,/<details class="advisor-diagnostics"[^>]*\bopen/);
  assert(html.indexOf('advisorNext')<html.indexOf('Economic pressures and stability'));
  c.box.querySelector('#advisorNext').onclick();assert.deepEqual(c.routes,[['construction',{project:7}]]);
});
test('completed-industry advice opens the shared Industry desk and rejects stale navigation',async()=>{
  const c=decisionFixture();c.S.programs={enabled:true};c.api=async()=>({completed:[{province:{id:'US-CA'}}]});
  await c.window.openAdvisor();assert.match(c.body().innerHTML,/Manage industry in Economy/);
  assert.deepEqual(c.routes,[]);c.box.querySelector('#advisorNext').onclick();
  assert.deepEqual(c.routes,[['industry']]);assert.equal(c.box.open,false);
  await c.window.openAdvisor();const action=c.box.querySelector('#advisorNext').onclick;
  c.S={session_id:'two',player:'Japan'};assert.equal(action(),false);
  assert.deepEqual(c.routes,[['industry']]);assert.equal(c.box.open,true);
});
test('loaded Advisor actions refuse a changed world, replaced campaign or superseding dialog',async()=>{
  for(const target of ['#advisorNext','#advisorExchange','#advisorCauses'])for(const replacement of [{session_id:'one',player:'USA'},{session_id:'two',player:'Japan'}]){
    const c=decisionFixture();c.S.policy={war:.02};c.api=async()=>({queue:[]});await c.window.openAdvisor();
    const click=c.box.querySelector(target).onclick;c.S=replacement;
    assert.equal(click(),false);assert.deepEqual(c.routes,[]);assert.equal(c.box.open,true);
    assert.match(c.box.querySelector('#advisorStatus').textContent,/Refresh advice/);
  }
  const c=decisionFixture();c.api=async()=>({queue:[]});await c.window.openAdvisor();
  const oldAction=c.box.querySelector('#advisorNext').onclick;c.box.close();c.window.openResearchList();
  assert.equal(oldAction(),false);assert.deepEqual(c.routes,[]);assert.equal(c.box.open,true);
});
test('Advisor escapes suggestion names, reasons, evidence and cautions',async()=>{
  const c=decisionFixture();c.S.programs={enabled:true};c.api=async()=>({suggestions:{items:[{id:'x',project_kind:'power_grid',district:'US-CA',
    name:'<img>',district_name:'<iframe>',reason:'<script>reason</script>',evidence:['<svg>'],caution:'<script>caution</script>'}]}});
  await c.window.openAdvisor();const html=c.body().innerHTML;assert.doesNotMatch(html,/<img|<iframe|<script|<svg/);
  assert.match(html,/&lt;script&gt;reason/);assert.match(html,/&lt;svg&gt;/);assert.match(html,/— total project cost/);
});

function recoveryFixture(){
  const elements=new Map(),timers=[];let customOpen=true;
  const doc={activeElement:null};
  const node=id=>{if(!elements.has(id))elements.set(id,{id,tagName:'BUTTON',isConnected:true,disabled:false,hidden:false,style:{},
    getClientRects:()=>[{}],focus(){doc.activeElement=this;}});return elements.get(id);};
  const opener=node('agencyBtn'),submit=node('acceptOffer');doc.activeElement=submit;
  const dialog={open:true,close(){this.open=false;opener.focus();}};
  doc.getElementById=node;doc.querySelector=selector=>selector==='dialog[open]'&&dialog.open?dialog:null;
  doc.querySelectorAll=selector=>selector==='dialog[open]'&&dialog.open?[dialog]:[];
  const c=vm.createContext({document:doc,SESSION:{slot:'France-1990',busy:false},CAB:{busy:false},clock:{running:false},
    S:{session_id:'one',player:'USA'},advancing:false,pendingAdvance:null,queued:[{kind:'tax',value:.3}],
    arcadeTopRoom:()=>customOpen?{id:'sheet'}:null,
    closeGameDrawers(){customOpen=false;opener.focus();},banner(){},setTimeout:fn=>timers.push(fn),
    adopt:async state=>{c.S=state;}});
  vm.runInContext(source('campaign-ui.js'),c);
  let request,seq=0;
  c.COMMAND_CHANNEL=transport.create({session:()=>c.S.session_id,identity:()=>({client_id:'test',request_seq:++seq}),
    request:p=>request(p),changed:()=>c.syncCommandControls()});
  return {c,doc,node,dialog,submit,opener,timers,customOpen:()=>customOpen,setRequest:fn=>request=fn};
}
test('normal sending leaves rooms open; uncertain response exposes reachable recovery without losing intent',async()=>{
  const f=recoveryFixture();let finish;
  f.setRequest(()=>new Promise(resolve=>finish=resolve));
  const sending=f.c.COMMAND_CHANNEL.send({commands:[{kind:'respond_diplomacy',offer:8,accept:true}]});
  assert(f.dialog.open);assert(f.customOpen());assert(f.submit.disabled,'the submitted button is protected while awaiting a response');
  finish({session_id:'one',player:'USA'});await sending;
  assert(f.dialog.open);assert(f.customOpen());assert.equal(f.submit.disabled,false);
  assert.equal(f.node('saveBtn').textContent,'Save · France-1990');
  const queued=JSON.stringify(f.c.queued);
  f.setRequest(async()=>{throw Error('lost committed response');});
  await assert.rejects(f.c.COMMAND_CHANNEL.send({commands:[{kind:'respond_diplomacy',offer:9,accept:false}]}));
  assert.equal(f.dialog.open,false);assert.equal(f.customOpen(),false);
  assert.equal(f.node('pendingCommand').hidden,false);assert.equal(f.node('retryCommandBtn').disabled,false);
  assert.equal(f.doc.activeElement,f.node('retryCommandBtn'));
  assert.equal(f.c.COMMAND_CHANNEL.pending.commands[0].offer,9);
  assert.equal(JSON.stringify(f.c.queued),queued,'closing presentation never drops a queued draft');
  f.setRequest(async p=>({session_id:p.session_id,player:'USA',errors:[]}));
  await f.c.retryCampaignCommand();
  assert.equal(f.c.COMMAND_CHANNEL.pending,null);assert.equal(f.node('pendingCommand').hidden,true);
  assert.equal(f.doc.activeElement,f.opener,'receipt confirmation restores the room launcher');
});
test('an explicit non-applied refusal leaves its form open for correction',async()=>{
  const f=recoveryFixture();f.setRequest(async()=>{const e=Error('malformed');e.notApplied=true;throw e;});
  await assert.rejects(f.c.COMMAND_CHANNEL.send({commands:[]}));
  assert(f.dialog.open);assert(f.customOpen());assert.equal(f.c.COMMAND_CHANNEL.pending,null);
});

function confirmationFixture(){
  const mounted=new Map(),messages=[],calls=[];const doc={activeElement:null};
  const make=id=>({id,isConnected:true,disabled:false,textContent:'',value:'',focus(){doc.activeElement=this;}});
  const opener=make('loadBtn');doc.activeElement=opener;mounted.set('saveSlots',{value:'audit-check'});
  let newest;
  doc.createElement=()=>{
    const nodes=new Map(),listeners=new Map();
    const dialog={id:'',open:false,attrs:{},innerHTML:'',
      querySelector(selector){if(!nodes.has(selector))nodes.set(selector,make(selector.slice(1)));return nodes.get(selector);},
      setAttribute(k,v){this.attrs[k]=v;},addEventListener(k,fn){listeners.set(k,fn);},
      showModal(){this.open=true;},close(){this.open=false;listeners.get('close')?.();},
      remove(){mounted.delete(this.id);},cancel(){let prevented=false;listeners.get('cancel')?.({preventDefault(){prevented=true;}});return prevented;}};
    newest=dialog;return dialog;
  };
  doc.body={append(node){mounted.set(node.id,node);}};doc.getElementById=id=>mounted.get(id)||null;
  const c=vm.createContext({document:doc,clockPause(){},banner:m=>messages.push(m),
    S:{session_id:'live',player:'USA'},SESSION:{live:{session_id:'live',player:'USA'},busy:false,slot:'default'},advancing:false,pendingAdvance:null,
    $:selector=>doc.getElementById(selector.slice(1)),renderSessionActions(){},
    window:{confirm(){assert.fail('blocking browser confirm must not be called');}},
    api:async(url,body)=>{calls.push({url,body});return {session_id:'loaded',player:'USA',storage_notice:'Restored'};},
    enterCampaign:async state=>{c.S=state;}});
  vm.runInContext(source('campaign-ui.js'),c);c.syncCommandControls=()=>{};
  return {c,doc,opener,messages,calls,dialog:()=>newest};
}
test('HTML confirmation stays pending, defaults to Cancel and safely renders its description',async()=>{
  const f=confirmationFixture();let settled=false;
  const result=f.c.campaignConfirm('<img src=x onerror=unsafe()>',{title:'Load saved campaign',confirmLabel:'Load campaign'}).then(value=>{settled=true;return value;});
  await Promise.resolve();assert.equal(settled,false);
  const d=f.dialog();assert(d.open);assert.equal(d.attrs['aria-labelledby'],'campaignConfirmTitle');assert.equal(d.attrs['aria-describedby'],'campaignConfirmMessage');
  assert.equal(f.doc.activeElement.id,'campaignConfirmCancel');
  assert.equal(d.querySelector('#campaignConfirmMessage').textContent,'<img src=x onerror=unsafe()>');
  assert(!d.innerHTML.includes('<img'));assert.equal(d.querySelector('#campaignConfirmAccept').textContent,'Load campaign');
  d.querySelector('#campaignConfirmCancel').onclick();assert.equal(await result,false);assert.equal(f.doc.activeElement,f.opener);
});
test('Escape and external close cancel; only the explicit confirmation button grants consent',async()=>{
  for(const close of [d=>assert(d.cancel()),d=>d.close(),d=>d.querySelector('#campaignConfirmCancel').onclick()]){
    const f=confirmationFixture(),result=f.c.campaignConfirm('Replace campaign?');close(f.dialog());assert.equal(await result,false);
  }
  const f=confirmationFixture(),result=f.c.campaignConfirm('Replace campaign?');
  assert.equal(await f.c.campaignConfirm('A second unrelated action?'),false,'a second action never shares the first action\'s approval');
  f.dialog().querySelector('#campaignConfirmAccept').onclick();assert.equal(await result,true);
});
test('confirmation cannot authorize an action against a newly adopted campaign',async()=>{
  const f=confirmationFixture(),result=f.c.campaignConfirm('Replace this campaign?');
  f.c.S={...f.c.S,session_id:'different'};f.dialog().querySelector('#campaignConfirmAccept').onclick();
  assert.equal(await result,false);assert.match(f.messages[0],/campaign changed/);
});
test('the actual load action preserves the live campaign until explicit HTML consent',async()=>{
  const page=source('index.html'),start=page.indexOf('async function loadCampaign('),end=page.indexOf('\n}',start)+2;
  assert(start>=0);
  for(const accept of [false,true]){
    const f=confirmationFixture();vm.runInContext(page.slice(start,end),f.c);
    const result=f.c.loadCampaign(true);await Promise.resolve();assert.equal(f.calls.length,0);
    f.dialog().querySelector(accept?'#campaignConfirmAccept':'#campaignConfirmCancel').onclick();await result;
    assert.equal(f.calls.length,accept?1:0);
    if(accept){assert.equal(f.calls[0].url,'/api/load');assert.deepEqual(JSON.parse(JSON.stringify(f.calls[0].body)),{slot:'audit-check',backup:true});}
    else assert.equal(f.c.S.session_id,'live');
  }
});
function governmentFunction(name){
  const page=source('index.html'),match=new RegExp('(?:async )?function '+name+'\\(').exec(page);
  assert(match,name);return page.slice(match.index,page.indexOf('\n}',match.index)+2);
}
function governmentFixture(){
  const f=confirmationFixture(),nodes=new Map(),c=f.c;
  const node=id=>{if(!nodes.has(id))nodes.set(id,{id,style:{},isConnected:true,tabIndex:0,
    classList:{contains:()=>false,remove(){}},getClientRects:()=>[1],closest:()=>null,
    focus(){f.doc.activeElement=this;},querySelectorAll:()=>[],contains:()=>false});return nodes.get(id);};
  const lookup=c.$;c.$=selector=>lookup(selector)||node(selector.slice(1));
  c.gov={open:true,nation:'USA',data:null,seq:0,lastFocus:null,busy:false,action:null};c.COVERT_ACTION={pending:null};
  c.COMMAND_CHANNEL={busy:false,pending:null};c.selected='France';c.adopted=[];c.renders=0;c.reads=0;
  c.renderGovernment=()=>c.renders++;c.govFetch=()=>c.reads++;c.fillCovertDash=()=>c.reads++;
  c.adopt=async value=>{c.adopted.push(value);};
  vm.runInContext(['governmentActionBlocked','govAct','covertAct'].map(governmentFunction).join('\n'),c);
  f.action={label:'Reviewed action',price:5,command:{kind:'covert',target:'France'}};c.gov.data={mine:true,actions:[f.action]};
  f.run=kind=>kind==='government'?c.govAct(f.action):c.covertAct(f.action,'France');f.node=node;return f;
}
for(const kind of ['government','covert']){
  test(kind+' waits for explicit consent, blocks duplicate clicks and cancels without sending',async()=>{
    const f=governmentFixture(),pending=f.run(kind);assert.equal(f.calls.length,0);assert(f.dialog().open);
    await f.run(kind);assert.equal(f.calls.length,0);
    f.dialog().querySelector('#campaignConfirmCancel').onclick();await pending;
    assert.equal(f.calls.length,0);assert.equal(f.c.gov.busy,false);assert.equal(f.c.COVERT_ACTION.pending,null);
  });
  test(kind+' sends only the captured reviewed command after confirmation',async()=>{
    const f=governmentFixture(),pending=f.run(kind);f.action.command.target='Japan';f.action.label='Changed action';
    f.dialog().querySelector('#campaignConfirmAccept').onclick();await pending;
    assert.equal(f.calls.length,1);assert.equal(f.calls[0].url,'/api/command');
    assert.deepEqual(JSON.parse(JSON.stringify(f.calls[0].body)),{commands:[{kind:'covert',target:'France'}]});
    assert.equal(f.c.adopted.length,1);assert.match(f.messages[0],/^Reviewed action/);
  });
  test(kind+' refuses a changed campaign, target or pending order after confirmation',async()=>{
    for(const change of [f=>f.c.S={...f.c.S},f=>kind==='government'?f.c.gov.nation='Japan':f.c.selected='Japan',f=>f.c.COMMAND_CHANNEL.pending={}]){
      const f=governmentFixture(),pending=f.run(kind);change(f);
      f.dialog().querySelector('#campaignConfirmAccept').onclick();await pending;assert.equal(f.calls.length,0);
    }
  });
  test(kind+' cannot adopt an old response into a replacement campaign',async()=>{
    const f=governmentFixture();let finish;f.c.api=()=>new Promise(resolve=>finish=resolve);
    const pending=f.run(kind);f.dialog().querySelector('#campaignConfirmAccept').onclick();await Promise.resolve();
    assert.equal(typeof finish,'function');f.c.S={session_id:'replacement',player:'Japan'};
    finish({session_id:'live',player:'USA'});await pending;assert.equal(f.c.adopted.length,0);assert.equal(f.messages.length,0);
  });
}
test('Government reset clears campaign readers and late reads cannot repopulate the closed room',async()=>{
  for(const reject of [false,true]){
    const f=governmentFixture(),c=f.c;vm.runInContext(['closeGovernment','resetGovernment','govFetch'].map(governmentFunction).join('\n'),c);
    let finish;c.api=()=>new Promise((resolve,fail)=>finish=reject?fail:resolve);
    const pending=c.govFetch();c.gov.busy=true;c.gov.action={};c.COVERT_ACTION.pending={};const seq=c.gov.seq;
    c.resetGovernment();assert.equal(c.gov.open,false);assert.equal(c.gov.nation,null);assert.equal(c.gov.data,null);assert(c.gov.seq>seq);
    assert.equal(c.gov.busy,false);assert.equal(c.gov.action,null);assert.equal(c.COVERT_ACTION.pending,null);
    finish(reject?Error('old reader'):{mine:true,actions:[]});await pending;
    assert.equal(c.gov.data,null);assert.equal(c.renders,0);assert(governmentFunction('resetCampaignUi').includes('resetGovernment();'));
  }
});
test('Government shares room navigation and Tab trapping while reopening retains its launcher',()=>{
  const f=governmentFixture(),c=f.c;c.stock={open:false};c.tech={open:false};c.PROD={open:false};c.LOGI={open:false};
  for(const name of ['keysCardIsOpen','dominationIsOpen','cabinetIsOpen','techMenuIsOpen'])c[name]=()=>false;
  for(const name of ['closeTechMenu','closeSheet'])c[name]=()=>{};
  f.doc.querySelectorAll=()=>[];c.ARCADE_ROOMS={worldFocus:null};
  vm.runInContext(['arcadeTopRoom','arcadeTrapTab','closeGameDrawers','closeGovernment','openGovernment'].map(governmentFunction).join('\n'),c);
  const room=f.node('govScreen'),first=f.node('first'),last=f.node('last'),launcher=f.node('govBtn');
  room.querySelectorAll=()=>[first,last];room.contains=el=>el===first||el===last;
  assert.equal(c.arcadeTopRoom(),room);f.doc.activeElement=last;let trapped=0;
  c.arcadeTrapTab({shiftKey:false,preventDefault(){trapped++;}},room);assert.equal(f.doc.activeElement,first);assert.equal(trapped,1);
  c.gov.lastFocus=launcher;c.openGovernment('USA');assert.equal(c.gov.lastFocus,launcher);assert.equal(c.gov.open,true);
  c.closeGameDrawers();assert.equal(c.gov.open,false);assert.equal(f.doc.activeElement,launcher);assert.equal(c.arcadeTopRoom(),null);
});
test('covert readers cannot attach old actions after campaign replacement',async()=>{
  for(const reject of [false,true]){
    const f=governmentFixture(),c=f.c;vm.runInContext(governmentFunction('fillCovertDash'),c);
    let finish;c.api=()=>new Promise((resolve,fail)=>finish=reject?fail:resolve);
    const pending=c.fillCovertDash('France');c.S={session_id:'replacement',player:'Japan'};
    finish(reject?Error('old reader'):{});await pending;assert.equal(f.node('covertDash').textContent,undefined);
  }
});
test('shipped game actions have no blocking browser confirm calls',()=>{
  for(const file of ['index.html','campaign-ui.js','competition-ui.js'])assert.doesNotMatch(source(file),/(?:window\.)?\bconfirm\(/,file);
});
