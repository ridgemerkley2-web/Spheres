// Actual presentation functions with a minimal dialog/focus surface; no browser.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const transport=require('../../spheres-web/ui/campaign-transport.js');
const source=name=>fs.readFileSync(path.join(__dirname,'../../spheres-web/ui',name),'utf8');

function decisionFixture(){
  const elements=new Map(),listeners=new Map();
  const node=id=>{if(!elements.has(id))elements.set(id,{id,value:'',innerHTML:'',textContent:'',focus(){}});return elements.get(id);};
  const box={open:false,attrs:{},querySelector:node,querySelectorAll:()=>[],
    setAttribute(k,v){this.attrs[k]=v;},addEventListener(k,fn){listeners.set(k,fn);},
    showModal(){this.open=true;},close(){this.open=false;listeners.get('close')?.();},
    set innerHTML(value){node('.decision-body').innerHTML=value;},get innerHTML(){return node('.decision-body').innerHTML;}};
  let attached=false;
  const document={activeElement:{focus(){}},getElementById:()=>attached?box:null,
    createElement:()=>box,body:{append(){attached=true;}}};
  const c=vm.createContext({window:{},document,clockPause(){},S:{session_id:'one',player:'USA',programs:{due:true}},tech:{data:[]}});
  vm.runInContext(source('decision-tools.js'),c);
  c.box=box;c.body=()=>node('.decision-body');return c;
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
test('shipped game actions have no blocking browser confirm calls',()=>{
  for(const file of ['index.html','campaign-ui.js','competition-ui.js'])assert.doesNotMatch(source(file),/(?:window\.)?\bconfirm\(/,file);
});
