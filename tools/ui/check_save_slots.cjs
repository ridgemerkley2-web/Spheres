// Execute actual refresh/load functions; deferred reads model network ordering.
// The separate real-browser CI checks the native select and confirmation DOM.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const {test}=require('node:test');
const root=path.resolve(__dirname,'../../spheres-web/ui');
const source=fs.readFileSync(path.join(root,'campaign-ui.js'),'utf8').replace(/\r\n/g,'\n');
const page=fs.readFileSync(path.join(root,'index.html'),'utf8').replace(/\r\n/g,'\n');
function fn(text,name){const start=new RegExp('^async function '+name+'\\(','m').exec(text);assert(start);const end=text.indexOf('\n}',start.index);assert(end>start.index);return text.slice(start.index,end+2);}
const declaration=/^let saveSlotsRead=0;$/m.exec(source)?.[0];assert(declaration);
const entry=slot=>({slot,player:'France',date:'1 Feb 1990',readable:true,backup:false});
const packet=(...slots)=>({slots:slots.map(entry),autosave:'Three rotating autosaves'});
function deferred(){let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b;});return {promise,resolve,reject};}
function fixture(){
  const reads=[],loads=[],confirmations=[],adoptions=[];let renders=0,selected='old';
  const select={options:[{value:'old'},{value:'ci-smoke'}],
    get value(){return selected;},set value(value){selected=this.options.some(o=>o.value===value)?value:'';},
    replaceChildren(){this.options=[];selected='';},
    append(option){this.options.push(option);if(this.options.length===1)selected=option.value;}};
  const status={textContent:'Previous status'};
  const state={session_id:'original',player:'France',date:'1 Feb 1990',cash:12};
  const c=vm.createContext({document:{getElementById:id=>id==='saveSlots'?select:status,createElement:()=>({value:'',textContent:'',dataset:{}})},
    SESSION:{slot:'old',saves:packet('old','ci-smoke').slots,live:state,busy:false},S:state,advancing:false,pendingAdvance:null,
    $:id=>{assert.equal(id,'#saveSlots');return select;},clockPause(){},renderSessionActions(){},syncCommandControls(){},banner(){},
    renderMainMenuState(){renders++;},
    campaignConfirm(message){const task=deferred();confirmations.push({message,...task});return task.promise;},
    async enterCampaign(value,replacing){adoptions.push({value,replacing});c.S=value;},
    api(url,body){if(url==='/api/saves'){const task=deferred();reads.push(task);return task.promise;}
      assert.equal(url,'/api/load');loads.push(JSON.parse(JSON.stringify(body)));return Promise.resolve({...state,session_id:'loaded'});}});
  vm.runInContext(declaration+'\n'+fn(source,'refreshSaveSlots')+'\n'+fn(page,'loadCampaign'),c);
  return {c,select,status,state,reads,loads,confirmations,adoptions,get renders(){return renders;},
    refresh:()=>vm.runInContext('refreshSaveSlots()',c),load:()=>vm.runInContext('loadCampaign(false)',c)};
}

test('a delayed save-list response keeps the choice made after its request began',async()=>{
  const f=fixture(),pending=f.refresh();f.select.value='ci-smoke';
  f.reads[0].resolve(packet('old','ci-smoke'));await pending;
  assert.equal(f.select.value,'ci-smoke');assert.equal(f.c.S,f.state);
  assert.deepEqual(f.loads,[]);assert.equal(f.renders,1);
});

test('an older save-list success cannot replace newer inventory or user selection',async()=>{
  const f=fixture(),old=f.refresh(),latest=f.refresh();
  f.select.value='ci-smoke';f.reads[1].resolve(packet('new-slot','ci-smoke'));await latest;
  f.select.value='new-slot';f.reads[0].resolve(packet('old','ci-smoke'));await old;
  assert.equal(f.select.value,'new-slot');assert.deepEqual(f.select.options.map(o=>o.value),['new-slot','ci-smoke']);
  assert.deepEqual(JSON.parse(JSON.stringify(f.c.SESSION.saves)),packet('new-slot','ci-smoke').slots);
  assert.equal(f.renders,1);assert.deepEqual(f.loads,[]);
});

test('stale read errors cannot overwrite successful current status and current errors retain selection',async()=>{
  const f=fixture(),old=f.refresh(),latest=f.refresh();
  f.select.value='ci-smoke';f.reads[1].resolve(packet('ci-smoke','old'));await latest;
  f.reads[0].reject(Error('obsolete failure'));await old;
  assert.equal(f.status.textContent,'Three rotating autosaves');assert.equal(f.select.value,'ci-smoke');
  const failed=f.refresh();f.reads[2].reject(Error('current failure'));await failed;
  assert.match(f.status.textContent,/current failure/);assert.equal(f.select.value,'ci-smoke');
  assert.equal(f.c.S,f.state);assert.deepEqual(f.loads,[]);
});

test('cancel then confirm retains the reviewed save even when a list response arrives inside the first review',async()=>{
  const f=fixture(),pending=f.refresh();f.select.value='ci-smoke';const first=f.load();
  assert.match(f.confirmations[0].message,/Load ci-smoke /);
  f.reads[0].resolve(packet('old','ci-smoke'));await pending;
  f.confirmations[0].resolve(false);await first;
  assert.equal(f.select.value,'ci-smoke');assert.equal(f.c.S,f.state);
  assert.deepEqual(f.loads,[]);assert.deepEqual(f.adoptions,[]);
  const second=f.load();assert.match(f.confirmations[1].message,/Load ci-smoke /);
  f.confirmations[1].resolve(true);await second;
  assert.deepEqual(f.loads,[{slot:'ci-smoke',backup:false}]);assert.equal(f.adoptions.length,1);
});

test('empty inventories keep their honest placeholder and removed choices fall back only to an available entry',async()=>{
  const f=fixture(),empty=f.refresh();f.reads[0].resolve(packet());await empty;
  assert.equal(f.select.value,'default');assert.equal(f.select.options[0].textContent,'No saved campaign yet');
  assert.deepEqual(JSON.parse(JSON.stringify(f.c.SESSION.saves)),[]);
  const fresh=f.refresh();f.reads[1].resolve(packet('remaining'));await fresh;
  assert.equal(f.select.value,'remaining');assert.deepEqual(f.loads,[]);
});
