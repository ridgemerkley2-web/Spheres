// Execute the shipped clock against controllable requests and timers.
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const {test}=require('node:test');
const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');
const clockSource=page.slice(page.indexOf('const SPEED_DELAY_MS ='),page.indexOf('function setSpeed(n) {'));
const campaignSource=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/campaign-ui.js'),'utf8');
function fixture(advance){
  const timers=new Map(),messages=[];let nextId=0;
  const context=vm.createContext({advance,gameIsUp:()=>true,banner:text=>messages.push(text),
    $:()=>null,document:{querySelectorAll:()=>[]},
    setTimeout(fn,ms){const id=++nextId;timers.set(id,{fn,ms});return id;},
    clearTimeout(id){timers.delete(id);}});
  vm.runInContext(clockSource+'\nthis.clockState=clock;',context);
  return {context,timers,messages,run:code=>vm.runInContext(code,context),
    fire(){assert.equal(timers.size,1);const [id,timer]=timers.entries().next().value;timers.delete(id);return timer.fn();}};
}
const flush=()=>new Promise(resolve=>setImmediate(resolve));

function controlFixture(){
  const f=fixture(()=>f.context.advancing?null:new Promise(()=>{}));
  const nodes=new Map();
  const node=id=>{
    if(!nodes.has(id))nodes.set(id,{id,disabled:false,hidden:false,innerHTML:'',textContent:'',title:'',
      classList:{toggle(){}},click(){if(!this.disabled)this.onclick?.();}});
    return nodes.get(id);
  };
  Object.assign(f.context,{COMMAND_CHANNEL:{pending:null,busy:false},SESSION:{busy:false,slot:'default'},
    CAB:{busy:false},advancing:false,pendingAdvance:null,$:selector=>node(selector.slice(1)),
    document:{activeElement:null,getElementById:node,querySelectorAll:selector=>selector==='[data-step]'?[node('stepBtn')]:[]}});
  vm.runInContext(campaignSource,f.context);
  f.run(page.match(/^\$\("#playPauseBtn"\)\.onclick = clockToggle;$/m)[0]);
  return {...f,node};
}

test('rendered PAUSE stays clickable during a day while mutation controls remain locked',async()=>{
  const f=controlFixture();f.context.advancing=true;f.context.pendingAdvance={};
  f.run('clock.running=true;clockRenderState();syncCommandControls()');
  assert.match(f.node('playPauseBtn').innerHTML,/PAUSE/);
  assert.equal(f.node('playPauseBtn').disabled,false,'the running clock must always offer Pause');
  for(const id of ['saveBtn','stepBtn','loadBtn','newCampaignBtn','saveNamedBtn','loadBackupBtn','cabinetEnact']){
    assert.equal(f.node(id).disabled,true,id+' must remain protected');
  }
  f.node('playPauseBtn').click();
  assert.equal(f.context.clockState.running,false);assert.match(f.node('playPauseBtn').innerHTML,/PLAY/);
  assert.equal(f.node('playPauseBtn').disabled,false,'an ordinary in-flight day permits local resume');
  f.node('playPauseBtn').click();await flush();
  assert.equal(f.context.clockState.running,true);assert.match(f.node('playPauseBtn').innerHTML,/PAUSE/);
  assert.equal(f.timers.size,1);f.node('playPauseBtn').click();assert.equal(f.timers.size,0);
});

test('rendered PLAY remains locked for unresolved receipts, immediate commands and campaign transitions',()=>{
  for(const setup of [
    'pendingAdvance={};advancing=false;',
    'COMMAND_CHANNEL.pending={};',
    'COMMAND_CHANNEL.pending={};COMMAND_CHANNEL.busy=true;',
    'COMMAND_CHANNEL.busy=true;',
    'SESSION.busy=true;'
  ]){
    const f=controlFixture();f.run(setup+'clockRenderState();syncCommandControls();clockRenderState();');
    assert.match(f.node('playPauseBtn').innerHTML,/PLAY/);
    assert.equal(f.node('playPauseBtn').disabled,true,setup);
    f.node('playPauseBtn').click();assert.equal(f.context.clockState.running,false,setup);
  }
});

test('clicking Pause immediately locks the resulting PLAY when a receipt needs review',()=>{
  const f=controlFixture();f.context.pendingAdvance={};
  f.run('clock.running=true;clockRenderState();');
  assert.equal(f.node('playPauseBtn').disabled,false);
  f.node('playPauseBtn').click();assert.equal(f.context.clockState.running,false);
  assert.equal(f.node('playPauseBtn').disabled,true,'clock rendering must refresh the Play lock without another transport event');
  f.context.pendingAdvance=null;f.run('syncCommandControls();');
  assert.equal(f.node('playPauseBtn').disabled,false,'confirmed receipt restores Play');
});

test('pause and play during a held day leave one timer and one request chain',async()=>{
  let held,calls=0,inFlight=0,maxInFlight=0;
  const f=fixture(()=>{
    if(inFlight)return null;
    calls++;inFlight++;maxInFlight=Math.max(maxInFlight,inFlight);
    return new Promise(resolve=>{held=state=>{inFlight--;resolve(state);};});
  });
  f.run('clockPlay()');assert.equal(calls,1);
  for(let i=0;i<6;i++){f.run('clockPause();clockPlay()');await flush();assert.equal(f.timers.size,1);}
  held({});await flush();
  assert.equal(f.timers.size,1,'the retired request cannot arm another timer');
  f.fire();assert.equal(calls,2);assert.equal(maxInFlight,1);
  f.run('clockPause()');held({});await flush();
  assert.equal(f.timers.size,0,'a paused chain stays retired after its day lands');
});

test('a busy refusal waits before retrying while a failed turn stops the clock',async()=>{
  let calls=0;const f=fixture(async()=>++calls===1?null:false);
  f.run('clock.speed=5;clockPlay()');await flush();
  assert.equal(f.timers.size,1);assert(f.timers.values().next().value.ms>0,'busy retry cannot spin at speed five');
  await f.fire();assert.equal(calls,2);
  assert.equal(f.context.clockState.running,false);assert.equal(f.timers.size,0);
});

test('an interrupt pauses a completed day and keeps its reason visible',async()=>{
  const f=fixture(async()=>({interrupt:'A treaty needs your response.'}));
  f.run('clockPlay()');await flush();
  assert.equal(f.context.clockState.running,false);assert.equal(f.timers.size,0);
  assert.match(f.messages[0],/A treaty needs your response/);
});

test('a retired request error does not pause a newly started chain',async()=>{
  let rejectOld,calls=0;
  const f=fixture(()=>++calls===1?new Promise((resolve,reject)=>{rejectOld=reject;}):Promise.resolve({}));
  f.run('clockPlay();clockPause();clockPlay()');await flush();
  rejectOld(new Error('retired request'));await flush();
  assert.equal(f.context.clockState.running,true);assert.equal(f.timers.size,1);assert.equal(f.messages.length,0);
});

test('the clock cannot start while the app is hidden by its stylesheet',()=>{
  const gameSource=page.match(/^function gameIsUp\(\).*$/m)[0];
  let calls=0;const f=fixture(()=>{calls++;return {};});
  f.context.$=selector=>selector==='#app'?{style:{display:''}}:null;
  f.context.S={};f.context.getComputedStyle=()=>({display:'none'});
  vm.runInContext(gameSource,f.context);f.run('clockPlay()');
  assert.equal(calls,0,'a hidden game must not advance');
  assert.equal(f.context.clockState.running,false);assert.equal(f.timers.size,0);
});
