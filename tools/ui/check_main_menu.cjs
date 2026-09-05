// Execute the shipped menu navigation and save-state presentation functions.
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const {test}=require('node:test');
const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');
function source(name){
  const hit=new RegExp(`^function ${name}\\(`,'m').exec(page);
  assert(hit,`Missing shipped function ${name}`);
  const end=page.indexOf('\n}',hit.index);
  assert(end>hit.index,`Missing closing brace for ${name}`);
  return page.slice(hit.index,end+2);
}
function fixture(){
  const nodes=new Map(),focuses=[],picks=[];
  const node=selector=>{
    if(!nodes.has(selector))nodes.set(selector,{hidden:false,disabled:false,textContent:'',value:'',dataset:{},scrollTop:70,
      focus(options){assert.equal(this.disabled,false,`Focus targeted disabled ${selector}`);focuses.push({selector,options});}});
    return nodes.get(selector);
  };
  const c=vm.createContext({$:node,window:{matchMedia:()=>({matches:true})},
    SESSION:{live:null,saves:[],busy:false},COMMAND_CHANNEL:{pending:null,busy:false},
    S:null,advancing:false,pendingAdvance:null,setupNations:[],pickedNation:null,
    visiblePicks:()=>c.setupNations,
    pickNation:name=>{picks.push(name);c.pickedNation=c.setupNations.find(n=>n.name===name);},
    api:()=>{throw new Error('Menu navigation must not mutate the campaign');}});
  vm.runInContext(source('showMenuView')+'\n'+source('renderMainMenuState'),c);
  return {c,node,nodes,focuses,picks,run:code=>vm.runInContext(code,c)};
}

test('menu navigation displays one view, resets scroll and ignores unknown destinations',()=>{
  const f=fixture(),screens={home:'#campaignHome',nation:'#newCampaignPicker',saves:'#savedCampaigns'};
  for(const view of ['home','nation','saves','home']){
    f.node('#setup').scrollTop=250;f.c.destination=view;f.run('showMenuView(destination,false)');
    for(const [name,selector] of Object.entries(screens))assert.equal(f.node(selector).hidden,name!==view);
    assert.equal(f.node('#setup').dataset.menuView,view);assert.equal(f.node('#setup').scrollTop,0);
  }
  const before=JSON.stringify([...f.nodes]);f.run('showMenuView("unknown")');
  assert.equal(JSON.stringify([...f.nodes]),before);assert.equal(f.focuses.length,0);
});

test('menu focus reaches a usable heading for empty saves and touch nation selection',()=>{
  const f=fixture();f.run('showMenuView("saves")');
  assert.equal(f.node('#saveSlots').disabled,true);assert.equal(f.node('#menuSaveEmpty').hidden,false);
  assert.equal(f.focuses.at(-1).selector,'#savedCampaignTitle');
  f.c.window.matchMedia=()=>({matches:false});f.run('showMenuView("nation")');
  assert.equal(f.focuses.at(-1).selector,'#nationBrowserTitle');
  f.c.window.matchMedia=()=>({matches:true});f.run('showMenuView("nation")');
  assert.equal(f.focuses.at(-1).selector,'#nationSearch');
  f.run('showMenuView("home")');assert.equal(f.focuses.at(-1).selector,'#newCampaignBtn');
  f.c.SESSION.live={player:'France'};f.run('showMenuView("home")');
  assert.equal(f.focuses.at(-1).selector,'#continueBtn');
});

test('leaving and reopening nation choice preserves the selection, search, sorting and seed',()=>{
  const f=fixture();f.c.setupNations=[{name:'France'},{name:'Tonga'}];
  f.c.visiblePicks=()=>[f.c.setupNations[1]];
  f.node('#nationSearch').value='Tonga';f.node('#nationSort').value='name';f.node('#seed').value='42';
  const world={player:'USA',date:'3 Jan 1990'};f.c.S=world;
  f.run('showMenuView("nation",false)');assert.deepEqual(f.picks,['Tonga']);
  const chosen=f.c.pickedNation;
  f.c.visiblePicks=()=>[f.c.setupNations[0]];
  f.run('showMenuView("home",false);showMenuView("saves",false);showMenuView("nation",false)');
  assert.equal(f.c.pickedNation,chosen);assert.deepEqual(f.picks,['Tonga']);
  assert.equal(f.node('#nationSearch').value,'Tonga');assert.equal(f.node('#nationSort').value,'name');
  assert.equal(f.node('#seed').value,'42');assert.equal(f.c.S,world);
});

test('an unavailable roster remains navigable without inventing a selected nation',()=>{
  const f=fixture();f.run('showMenuView("nation",false);showMenuView("home",false)');
  assert.equal(f.c.pickedNation,null);assert.deepEqual(f.picks,[]);
  assert.equal(f.node('#campaignHome').hidden,false);
});

test('save rendering distinguishes unknown, empty and populated inventories',()=>{
  const f=fixture();f.c.SESSION.saves=undefined;f.run('renderMainMenuState()');
  assert.equal(f.node('#menuSaveEmpty').hidden,true,'unknown saves are not reported as empty');
  assert.equal(f.node('#loadBtn').hidden,true);assert.equal(f.node('#loadBtn').disabled,true);
  f.c.SESSION.saves=[];f.run('renderMainMenuState()');
  assert.equal(f.node('#menuSaveEmpty').hidden,false);assert.equal(f.node('#menuSaveCount').textContent,'');
  f.c.SESSION.saves=[{slot:'France',backup:true},{slot:'Tonga',backup:false}];f.node('#saveSlots').value='France';
  f.c.S={player:'USA'};f.run('renderMainMenuState()');
  assert.equal(f.node('#menuSaveEmpty').hidden,true);assert.equal(f.node('#menuSaveCount').textContent,'2');
  assert.equal(f.node('#loadBtn').hidden,false);assert.equal(f.node('#loadBtn').disabled,false);
  assert.equal(f.node('#loadBackupBtn').hidden,false);assert.equal(f.node('#loadBackupBtn').disabled,false);
  assert.equal(f.node('#menuSaveCurrent').hidden,false);
  f.c.S=null;f.node('#saveSlots').value='Tonga';f.run('renderMainMenuState()');
  assert.equal(f.node('#loadBackupBtn').hidden,true);assert.equal(f.node('#loadBackupBtn').disabled,true);
  assert.equal(f.node('#menuSaveCurrent').hidden,true);
});

test('load actions stay locked for active work and unconfirmed receipts without losing the selected slot',()=>{
  for(const blocker of ['SESSION.busy=true','advancing=true','pendingAdvance={}',
    'COMMAND_CHANNEL.pending={}','COMMAND_CHANNEL.busy=true']){
    const f=fixture();f.c.SESSION.saves=[{slot:'France',backup:true}];f.node('#saveSlots').value='France';
    f.run(blocker+';renderMainMenuState()');
    assert.equal(f.node('#loadBtn').disabled,true,blocker);assert.equal(f.node('#loadBackupBtn').disabled,true,blocker);
    assert.equal(f.node('#saveSlots').value,'France');
    assert.equal(f.node('#saveSlots').disabled,blocker==='SESSION.busy=true');
    f.run('SESSION.busy=false;advancing=false;pendingAdvance=null;COMMAND_CHANNEL.pending=null;COMMAND_CHANNEL.busy=false;renderMainMenuState()');
    assert.equal(f.node('#loadBtn').disabled,false);assert.equal(f.node('#loadBackupBtn').disabled,false);
    assert.equal(f.node('#saveSlots').value,'France');
  }
});
