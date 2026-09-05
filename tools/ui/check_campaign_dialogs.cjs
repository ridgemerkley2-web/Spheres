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
