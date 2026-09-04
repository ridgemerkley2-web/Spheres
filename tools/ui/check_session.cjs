// Run: node --test tools/ui/check_session.cjs
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const page = fs.readFileSync(path.join(__dirname, '../../spheres-web/ui/index.html'), 'utf8');
function fn(name) {
  const hit = new RegExp(`^(?:async\\s+)?function ${name}\\(`, 'm').exec(page);
  assert(hit, `Missing real page function ${name}`);
  return page.slice(hit.index, page.indexOf('\n}', hit.index) + 2);
}
function fixture(names = ['advance']) {
  const calls = [], nodes = new Map();
  const node = key => { if (!nodes.has(key)) nodes.set(key, {disabled:false, textContent:'', style:{}, hidden:false, dataset:{}, focus(){}}); return nodes.get(key); };
  const storage=new Map();
  const context = vm.createContext({ calls, console, crypto:{randomUUID:()=> 'turn-123'},
    document:{querySelectorAll:()=>[], querySelector:node}, $:node,
    banner:message=>calls.push(['banner',message]), noteQueued:()=>calls.push(['queue']),
    api:async()=>({date:'2 Jan 1990',session_id:'campaign-1'}),
    adopt:async state=>{context.received=state;},
    sessionStorage:{getItem:key=>storage.get(key)||null,setItem:(key,value)=>storage.set(key,value),removeItem:key=>storage.delete(key)},
    renderSessionActions(){}, syncAdvanceControls(){}, persistPendingAdvance(){},
    nextAdvanceIdentity:()=>({client_id:'test-browser',request_seq:1}),
  });
  vm.runInContext(`let queued=[]; let S={date:'1 Jan 1990',session_id:'campaign-1',player:'USA'};
    let advancing=false; let pendingAdvance=null; let sessionBusy=false;
    const SESSION={live:null,busy:false,client:null,sequence:0}; const CAB={busy:false,error:''};
    ${names.map(fn).join('\n')}`,context);
  context.storage=storage;return context;
}
const run = (c,s)=>vm.runInContext(s,c);
test('edits added and replacements made while an advance is in flight survive its response',async()=>{
  const c=fixture();let release;
  c.api=()=>new Promise(resolve=>{release=resolve;});
  run(c,"queued=[{kind:'tax',value:.2}]");
  const pending=run(c,'advance(1)');
  run(c,"queued=[{kind:'tax',value:.3},{kind:'interest',value:.04}]");
  release({date:'2 Jan 1990',session_id:'campaign-1'});await pending;
  assert.deepEqual(JSON.parse(run(c,'JSON.stringify(queued)')),[{kind:'tax',value:.3},{kind:'interest',value:.04}]);
});
test('network failure is visible, retains the submitted draft, and never rejects unhandled',async()=>{
  const c=fixture();c.api=async()=>{throw new Error('Offline fixture');};
  run(c,"queued=[{kind:'tax',value:.2}]");
  assert.equal(await run(c,'advance(1)'),false);
  assert.equal(run(c,'queued.length'),1);
  assert(c.calls.some(x=>x[0]==='banner'&&x[1].includes('Offline fixture')));
});
test('save failure is surfaced instead of becoming an unhandled rejection',async()=>{
  const c=fixture(['doSave']);c.api=async()=>{throw new Error('Save fixture');};
  await run(c,'doSave()');
  assert(c.calls.some(x=>x[0]==='banner'&&x[1].includes('Save fixture')));
});
test('boot offers explicit continue and load without starting a replacement campaign',()=>{
  assert.match(page,/id="continueBtn"/);assert.match(page,/id="loadBtn"/);
  assert.match(fn('bootSession'),/\/api\/state/);
  assert.doesNotMatch(fn('bootSession'),/\/api\/new/);
  assert.match(page,/bootSession\(\);\s*<\/script>/);
});
test('globe starts with an uninitialized center while real zero stays valid',()=>{
  const cam=/cam:\s*(\{[^}]+\})/.exec(page)[1];
  const value=vm.runInNewContext(`(${cam})`);
  assert.equal(value.cx,undefined);assert.equal(value.cy,undefined);
  assert.match(fn('renderMap'),/ui\.cam\.cx === undefined/);
});

test('only unchanged submitted object references are removed, even if another command has the same value',async()=>{
  const c=fixture();let release;
  c.api=()=>new Promise(resolve=>{release=resolve;});
  run(c,"queued=[{kind:'tax',value:.2},{kind:'interest',value:.03}]");
  const pending=run(c,'advance(1)');
  run(c,"queued[0].value=.25;queued.push({kind:'interest',value:.03})");
  release({date:'2 Jan 1990',session_id:'campaign-1'});await pending;
  assert.deepEqual(JSON.parse(run(c,'JSON.stringify(queued)')),[{kind:'tax',value:.25},{kind:'interest',value:.03}]);
});

test('uncertain retry resends the identical frozen turn, never newly added draft orders',async()=>{
  const c=fixture();const sent=[];let count=0;
  c.api=async(url,body)=>{sent.push(JSON.parse(JSON.stringify(body)));if(++count===1)throw new Error('Lost response');return {date:'2 Jan 1990',session_id:'campaign-1'};};
  run(c,"queued=[{kind:'tax',value:.2}]");
  assert.equal(await run(c,'advance(1)'),false);
  run(c,"queued.push({kind:'interest',value:.04})");
  assert.equal(await run(c,'advance(30)'),false,'a fresh advance is blocked while outcome is uncertain');
  assert.equal(sent.length,1);
  assert.equal(await run(c,'advance(1,true)'),true);
  assert.deepEqual(sent[1],sent[0]);
  assert.deepEqual(JSON.parse(run(c,'JSON.stringify(queued)')),[{kind:'interest',value:.04}]);
  assert.equal(run(c,'pendingAdvance'),null);
});

test('an explicit non-advancing server refusal unlocks the retained draft for correction',async()=>{
  const c=fixture();c.api=async()=>{const error=new Error('Invalid order 2');error.notAdvanced=true;throw error;};
  run(c,"queued=[{kind:'unknown'}]");
  assert.equal(await run(c,'advance(1)'),false);
  assert.equal(run(c,'queued.length'),1);assert.equal(run(c,'pendingAdvance'),null);
});

test('a display failure after acknowledgement cannot cause a committed turn to be retried',async()=>{
  const c=fixture();c.adopt=async()=>{throw new Error('Drawing failed');};
  run(c,"queued=[{kind:'tax',value:.2}]");
  assert.equal(await run(c,'advance(1)'),false);
  assert.equal(run(c,'queued.length'),0);assert.equal(run(c,'pendingAdvance'),null);
  assert(c.calls.some(x=>x[0]==='banner'&&x[1].includes('turn was recorded')));
});

test('reload restores the same pending receipt and only its still-matching draft references',async()=>{
  const names=['advance','persistPendingAdvance','restorePendingAdvance','nextAdvanceIdentity'];
  const c=fixture(names);c.api=async()=>{throw new Error('Disconnected');};
  run(c,"queued=[{kind:'tax',value:.2},{kind:'interest',value:.04}]");
  await run(c,'advance(1)');
  run(c,"queued=queued.filter(c=>c.kind!=='tax');queued.push({kind:'tax',value:.3});persistPendingAdvance()");
  const stored=c.storage.get('spheres.pending-turn');
  const reopened=fixture(names);
  reopened.storage.set('spheres.pending-turn',stored);
  reopened.storage.set('spheres.turn-client',c.storage.get('spheres.turn-client'));
  run(reopened,'restorePendingAdvance(S,false)');
  assert.equal(run(reopened,'pendingAdvance.payload.request_seq'),1);
  // Re-persisting a restored turn must preserve the right surviving source,
  // even when it no longer occupies the original command's array position.
  run(reopened,'persistPendingAdvance();queued=[];pendingAdvance=null;restorePendingAdvance(S,false)');
  const sent=[];reopened.api=async(url,body)=>{sent.push(body);return {date:'2 Jan 1990',session_id:'campaign-1'};};
  assert.equal(await run(reopened,'advance(1,true)'),true);
  assert.equal(sent[0].request_seq,1);
  assert.deepEqual(JSON.parse(run(reopened,'JSON.stringify(queued)')),[{kind:'tax',value:.3}]);
  assert.equal(run(reopened,'nextAdvanceIdentity().request_seq'),2);
});

test('pending orders from a replaced campaign never enter the current queue or retry path',()=>{
  const c=fixture(['restorePendingAdvance']);
  c.storage.set('spheres.pending-turn',JSON.stringify({payload:{session_id:'old-campaign',commands:[{kind:'tax',value:.2}]},queued:[{kind:'tax',value:.2}],submitted:[0]}));
  run(c,'restorePendingAdvance(S,false)');
  assert.equal(run(c,'pendingAdvance'),null);assert.equal(run(c,'queued.length'),0);
  assert(c.calls.some(x=>x[0]==='banner'&&x[1].includes('cannot be retried')));
});

test('API rejects a delayed old-campaign response and transmits identity on immediate mutations',async()=>{
  const c=fixture(['api']);let release,ready;const sent=[];
  const reading=new Promise(resolve=>{ready=resolve;});
  c.fetch=async(url,options)=>{sent.push(JSON.parse(options.body));return {ok:true,text:()=>new Promise(resolve=>{release=resolve;ready();})};};
  const result=run(c,"api('/api/command',{commands:[{kind:'tax',value:.2}]})");
  await reading;
  assert.equal(sent[0].session_id,'campaign-1');
  run(c,"S={session_id:'replacement'}");release('{}');
  await assert.rejects(result,/old response was not adopted/);
});

test('server refusal metadata distinguishes certain rejection from uncertain transport failure',async()=>{
  const c=fixture(['api']);c.fetch=async()=>({ok:false,status:400,text:async()=>JSON.stringify({error:'Malformed batch',not_advanced:true})});
  await assert.rejects(run(c,"api('/api/advance',{})"),error=>error.notAdvanced===true&&error.status===400);
});

test('boot reads state and roster without mutating the campaign and reveals Continue',async()=>{
  const c=fixture(['bootSession','renderSessionActions']);const paths=[];
  c.buildSetup=async()=>paths.push('roster');c.restorePendingAdvance=()=>{};
  c.api=async path=>{paths.push(path);return {player:'France',date:'31 Jan 1990',session_id:'existing'};};
  await run(c,'bootSession()');
  assert.deepEqual(paths,['roster','/api/state']);
  assert.equal(c.$('#continueBtn').hidden,false);assert.equal(c.$('#newCampaignPicker').hidden,true);
  assert.match(c.$('#sessionStatus').textContent,/France.*31 Jan 1990/);
});

test('older-sequence conflicts retain orders for explicit review instead of marking them safe to resend',async()=>{
  const c=fixture();c.api=async()=>{const error=new Error('A newer turn was already processed');error.requiresReview=true;throw error;};
  run(c,"queued=[{kind:'tax',value:.2}]");
  assert.equal(await run(c,'advance(1)'),false);
  assert.equal(run(c,'pendingAdvance.reviewRequired'),true);
  assert.equal(run(c,'queued.length'),1);
  assert(c.calls.some(x=>x[0]==='banner'&&x[1].includes('Review current state')));
});

test('review requires confirmation and never resubmits old orders, retaining newer draft edits',async()=>{
  const c=fixture(['advance','reviewPendingTurn']);
  c.api=async()=>{const error=new Error('Older request');error.requiresReview=true;throw error;};
  run(c,"queued=[{kind:'tax',value:.2}]");await run(c,'advance(1)');
  run(c,"queued.push({kind:'interest',value:.04})");
  const paths=[];c.api=async(path,body)=>{paths.push([path,body]);return {date:'3 Jan 1990',session_id:'campaign-1'};};
  c.window={confirm:()=>false};await run(c,'reviewPendingTurn()');
  assert.equal(run(c,'queued.length'),2);assert.notEqual(run(c,'pendingAdvance'),null);
  c.window.confirm=()=>true;await run(c,'reviewPendingTurn()');
  assert.equal(run(c,'pendingAdvance'),null);
  assert.deepEqual(JSON.parse(run(c,'JSON.stringify(queued)')),[{kind:'interest',value:.04}]);
  assert(paths.every(([path,body])=>path==='/api/state'&&body===undefined),'review is read-only, not another advance');
});

test('campaign changes and modal entry close only global More and Map menus',()=>{
  const c=fixture(['closeGlobalMenus']);
  const time={open:true},map={open:true},project={open:true},economy={open:true};let selector;
  c.document.querySelectorAll=value=>{selector=value;return [time,map];};
  run(c,'closeGlobalMenus()');
  assert.equal(selector,'#app .arc-time-menu[open], #app .arc-map-tools[open]');
  assert.equal(time.open,false);assert.equal(map.open,false);
  assert.equal(project.open,true);assert.equal(economy.open,true);
  for(const name of ['showCampaigns','resetCampaignUi','enterCampaign'])assert.match(fn(name),/closeGlobalMenus\(\)/);
  assert.ok(page.includes('document.addEventListener("focusin", () => {'));
  assert.ok(page.includes('if (arcadeTopRoom()) closeGlobalMenus();'));
});

test('campaign actions occupy a separate row and short screens can scroll without clipping Start',()=>{
  assert.ok(page.includes('#setup .setup-shell { display:block; height:auto; min-height:100%; align-self:flex-start; }'));
  assert.ok(page.includes('#setup { overflow-y:auto; overflow-x:hidden; }'));
  assert.ok(page.includes('#newCampaignPicker #nationShowcase { display:block; min-height:0; }'));
  assert.ok(page.includes('#newCampaignPicker .showcase-copy,#newCampaignPicker #setupFoot { flex-shrink:0; }'));
});
