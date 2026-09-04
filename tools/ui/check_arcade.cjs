// Behavior checks for the real shared room and dossier helpers.
// Run: node --test tools/ui/check_arcade.cjs
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');

const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');
function source(name) {
  const found=new RegExp(`^function ${name}\\(`,'m').exec(page);
  assert(found,`Missing actual helper ${name}`);
  const end=page.indexOf('\n}',found.index);
  assert(end>found.index);
  return page.slice(found.index,end+2);
}
function fixture() {
  const elements=new Map();
  const doc={activeElement:null};
  function element(id) {
    if(elements.has(id)) return elements.get(id);
    const classes=new Set();
    const el={id,style:{display:'none'},attributes:{},dataset:{},children:[],tabIndex:0,
      hidden:false,inertAncestor:false,disabled:false,rects:true,clicks:0,
      classList:{add:n=>classes.add(n),remove:n=>classes.delete(n),contains:n=>classes.has(n)},
      setAttribute(name,value){this.attributes[name]=value;},
      getClientRects(){return this.rects && !this.hidden?[1]:[];},
      closest(){return this.hidden || this.inertAncestor?{}:null;},
      focus(){doc.activeElement=this;},click(){this.clicks++;},
      contains(other){return other===this || this.children.includes(other);},
      querySelectorAll(selector){
        if(selector==='[data-nation-page]') return this.children.filter(child=>child.dataset.nationPage);
        if(selector==='[data-nation-view]') return this.children.filter(child=>child.dataset.nationView);
        if(selector.includes('button:not(:disabled)')) return this.children.filter(child=>!child.disabled);
        return this.children;
      },
    };
    elements.set(id,el);return el;
  }
  const flags={keys:false,domination:false,cabinet:false,techMenu:false};
  const c=vm.createContext({document:doc,element,flags,$:selector=>element(selector.slice(1)),
    keysCardIsOpen:()=>flags.keys,dominationIsOpen:()=>flags.domination,cabinetIsOpen:()=>flags.cabinet,
    techMenuIsOpen:()=>flags.techMenu});
  vm.runInContext(`const stock={open:false},tech={open:false},PROD={open:false},LOGI={open:false};
    let nationView='overview';let queued=[{kind:'tax',value:.25}];const S={date:'1 Jan 1990'};
    ${['arcadeTopRoom','arcadeTrapTab','selectNationView','arcadeAccessibleActions'].map(source).join('\n')}`,c);
  c.elements=elements;c.element=element;return c;
}
const run=(c,code)=>vm.runInContext(code,c,{timeout:1000});
function tab(c,room,shift=false) {
  let prevented=0;
  c.event={key:'Tab',shiftKey:shift,preventDefault(){prevented++;}};c.room=room;
  run(c,'arcadeTrapTab(event,room);');return prevented;
}
test('top-room selection follows visual priority without changing any open state',()=>{
  const c=fixture();
  assert.equal(run(c,'arcadeTopRoom()'),null);
  c.element('intelDrawer').classList.add('open');
  assert.equal(run(c,'arcadeTopRoom().id'),'intelDrawer');
  for(const [state,id] of [['LOGI','logisticsPanel'],['PROD','productionPanel'],['tech','techScreen'],['stock','stockScreen']]) {
    run(c,`${state}.open=true;`);assert.equal(run(c,'arcadeTopRoom().id'),id);
  }
  c.flags.techMenu=true;assert.equal(run(c,'arcadeTopRoom().id'),'techMenu');
  c.flags.cabinet=true;assert.equal(run(c,'arcadeTopRoom().id'),'cabinetDrawer');
  c.flags.domination=true;assert.equal(run(c,'arcadeTopRoom().id'),'dominationScreen');
  c.element('sheet').style.display='block';assert.equal(run(c,'arcadeTopRoom().id'),'sheet');
  c.flags.keys=true;assert.equal(run(c,'arcadeTopRoom().id'),'keys');
  const before=run(c,'JSON.stringify({stock,tech,PROD,LOGI,queued,S})');
  for(let i=0;i<5;i++) run(c,'arcadeTopRoom();');
  assert.equal(run(c,'JSON.stringify({stock,tech,PROD,LOGI,queued,S})'),before);
});
test('explicit map-view operations remain non-modal while other rooms still win',()=>{
  const c=fixture();run(c,'PROD.open=true;LOGI.open=true;');
  assert.equal(run(c,'arcadeTopRoom().id'),'productionPanel');
  c.element('productionPanel').classList.add('map-view');
  assert.equal(run(c,'arcadeTopRoom().id'),'logisticsPanel');
  c.element('logisticsPanel').classList.add('map-view');
  assert.equal(run(c,'arcadeTopRoom()'),null);
  run(c,'tech.open=true;');assert.equal(run(c,'arcadeTopRoom().id'),'techScreen');
  run(c,'tech.open=false;');c.element('productionPanel').classList.remove('map-view');
  assert.equal(run(c,'arcadeTopRoom().id'),'productionPanel');
});
test('Tab wraps at room boundaries, enters from outside, and leaves middle controls native',()=>{
  const c=fixture();const room=c.element('room');
  const first=c.element('first'),middle=c.element('middle'),last=c.element('last');
  room.children=[first,middle,last];
  c.element('outside').focus();assert.equal(tab(c,room),1);assert.equal(c.document.activeElement,first);
  first.focus();assert.equal(tab(c,room,true),1);assert.equal(c.document.activeElement,last);
  last.focus();assert.equal(tab(c,room),1);assert.equal(c.document.activeElement,first);
  middle.focus();assert.equal(tab(c,room),0);assert.equal(c.document.activeElement,middle);
  assert.equal(tab(c,room,true),0,'middle Shift+Tab remains a native transition');
});
test('Tab ignores hidden, inert, disabled and negative-tabindex descendants',()=>{
  const c=fixture();const room=c.element('room');
  const hidden=c.element('hidden');hidden.hidden=true;
  const rectless=c.element('rectless');rectless.rects=false;
  const inert=c.element('inert');inert.inertAncestor=true;
  const disabled=c.element('disabled');disabled.disabled=true;
  const negative=c.element('negative');negative.tabIndex=-1;
  const first=c.element('first'),last=c.element('last');
  room.children=[hidden,rectless,inert,disabled,negative,first,last];
  c.element('outside').focus();tab(c,room);assert.equal(c.document.activeElement,first);
  tab(c,room,true);assert.equal(c.document.activeElement,last);
  tab(c,room);assert.equal(c.document.activeElement,first);
});
test('an empty or loading room contains keyboard focus on itself',()=>{
  const c=fixture();const room=c.element('room');
  c.element('outside').focus();assert.equal(tab(c,room),1);assert.equal(c.document.activeElement,room);
  assert.equal(tab(c,room,true),1);assert.equal(c.document.activeElement,room);
});
test('nation tabs reveal exactly one page and preserve drafts and game state',()=>{
  const c=fixture();const sheet=c.element('sheet');const names=['overview','economy','resources','world'];
  names.forEach(name=>{
    const page=c.element(`page-${name}`);page.dataset.nationPage=name;sheet.children.push(page);
    const button=c.element(`tab-${name}`);button.dataset.nationView=name;sheet.children.push(button);
  });
  const saved=run(c,'JSON.stringify({queued,S})');
  for(const name of names) {
    run(c,`selectNationView(${JSON.stringify(name)});`);assert.equal(run(c,'nationView'),name);
    names.forEach(other=>{
      assert.equal(c.element(`page-${other}`).hidden,other!==name);
      assert.equal(c.element(`tab-${other}`).attributes['aria-selected'],String(other===name));
    });
  }
  const visible=sheet.children.filter(el=>el.dataset.nationPage && !el.hidden);
  assert.equal(visible.length,1);
  run(c,'selectNationView("unknown-tab");');assert.equal(run(c,'nationView'),'world');
  assert.equal(run(c,'JSON.stringify({queued,S})'),saved,'navigation must never post or mutate orders');
});
test('non-native action rows gain Enter and Space activation without swallowing other keys',()=>{
  const c=fixture();const root=c.element('action-root'),action=c.element('action');root.children=[action];
  c.root=root;run(c,'arcadeAccessibleActions(root);');
  assert.equal(action.attributes.role,'button');assert.equal(action.tabIndex,0);
  let prevented=0,stopped=0;
  const press=key=>action.onkeydown({key,preventDefault(){prevented++;},stopPropagation(){stopped++;}});
  press('Enter');press(' ');assert.equal(action.clicks,2);assert.equal(prevented,2);assert.equal(stopped,2);
  for(const key of ['Tab','Escape','ArrowDown','a'])press(key);
  assert.equal(action.clicks,2);assert.equal(prevented,2);
});
test('nation tabs maintain exactly one tab stop when selection changes',()=>{
  const c=fixture();const sheet=c.element('sheet');
  const names=['overview','economy','resources','world'];
  const tabs=names.map(name=>{
    const tab=c.element(`tab-${name}`);tab.dataset.nationView=name;sheet.children.push(tab);return tab;
  });
  tabs[0].focus();
  for(const name of ['economy','world','overview','resources']) {
    run(c,`selectNationView(${JSON.stringify(name)});`);
    assert.equal(tabs.filter(tab=>tab.tabIndex===0).length,1,'only the selected tab belongs in the Tab sequence');
    tabs.forEach(tab=>assert.equal(tab.tabIndex,tab.dataset.nationView===name?0:-1));
    assert.equal(c.document.activeElement,tabs[0],'selection alone must not unexpectedly steal focus');
  }
  const indices=tabs.map(tab=>tab.tabIndex);
  run(c,'selectNationView("invalid");');
  assert.deepEqual(tabs.map(tab=>tab.tabIndex),indices,'invalid page names cannot corrupt roving tabindex');
});
test('all three arcade stylesheets are local, embedded and served as CSS',()=>{
  const server=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/src/main.rs'),'utf8');
  for(const name of ['arcade','arcade-operations','arcade-discovery']) {
    assert(page.includes(`<link rel="stylesheet" href="/${name}.css">`));
    assert(server.includes(`include_str!("../ui/${name}.css")`));
    assert(server.includes(`"/${name}.css"`));
    assert(fs.statSync(path.resolve(__dirname,`../../spheres-web/ui/${name}.css`)).size>100);
  }
  const route=server.slice(server.indexOf('path @ ("/arcade.css"'),server.indexOf('path @ ("/arcade.css"')+800);
  assert(route.includes('text/css'),'stylesheet route must use CSS MIME type');
});

test('help contains time shortcuts but leaves its native close button usable',()=>{
  const match=/document\.addEventListener\("keydown", \(e\) => \{\r?\n  const room = arcadeTopRoom\(\);/.exec(page);
  assert(match);
  const end=page.indexOf('\n});',match.index);
  let listener,advanced=0,closed=0,prevented=0;
  const c=vm.createContext({document:{addEventListener:(type,fn)=>listener=fn},
    arcadeTopRoom:()=>({id:'keys'}),cabinetIsOpen:()=>false,dominationIsOpen:()=>false,
    focused:()=>false,typing:()=>false,keysCardIsOpen:()=>true,isKeysCardToggle:()=>false,
    setKeysCard:()=>closed++,advance:()=>advanced++,
  });
  vm.runInContext(page.slice(match.index,end+4),c);
  const event=(key,button)=>({key,target:{closest:()=>button?{}:null},preventDefault(){prevented++;}});
  listener(event(' ',false));assert.equal(prevented,1,'Space on the dialog surface cannot scroll behind it');
  for(const key of [' ','Enter','1','2','3','4'])listener(event(key,true));
  assert.equal(prevented,1,'native button activation must remain untouched');
  assert.equal(advanced,0,'help never advances the world');
  listener(event('Escape',true));assert.equal(closed,1);
});
