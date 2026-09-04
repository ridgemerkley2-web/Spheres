// Node-only, read-only checks. Run: node tools/ui/check_discovery_arcade.cjs
// No server/browser or external libraries; functions are loaded from the page.
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const assert = require('node:assert/strict');
const { test } = require('node:test');
const root = path.resolve(__dirname, '../..');
const html = fs.readFileSync(path.join(root, 'spheres-web/ui/index.html'), 'utf8');
const css = fs.readFileSync(path.join(root, 'spheres-web/ui/arcade-discovery.css'), 'utf8');
function source(name) {
  const m = new RegExp(`^(?:async\\s+)?function ${name}\\(`, 'm').exec(html);
  assert(m, `real page helper ${name} exists`);
  const first = html.slice(m.index, html.indexOf('\n', m.index));
  if (/\}\s*$/.test(first)) return first;
  return html.slice(m.index, html.indexOf('\n}', m.index) + 2);
}
function context(names, extra = {}) {
  const c = vm.createContext({
    clamp: (v, lo, hi) => Math.max(lo, Math.min(hi, v)),
    escText: t => String(t).replace(/&/g, '&amp;').replace(/</g, '&lt;'),
    stockHue: () => '#a8d4c3', rglyph: () => '<svg></svg>',
    stock: {sel:'copper'}, STATE_WORD:{short:'SHORT',presence:'PRESENCE'},
    ...extra,
  });
  vm.runInContext(names.map(source).join('\n'), c);
  return c;
}
test('all inline JavaScript remains syntactically valid', () => {
  let count = 0;
  for (const m of html.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/g)) {
    if (m[1].trim()) { new vm.Script(m[1]); count++; }
  }
  assert(count > 0);
});
test('resource selection cards retain daily quantities and provenance caveats', () => {
  const c = context(['fmtQ','fmtSigned','dailyRowView','stockRowHtml']);
  c.row = {id:'copper',name:'Copper',status:'short',tracked:true,cover_months:2,cover_days:59,
    produce:25.5,need:30,net:-4.5,unit:'t/day',sentence:'Current builds need more.',
    second:'Production is apportioned; deposits are presence-only.'};
  const card = vm.runInContext('stockRowHtml(row)', c);
  assert.match(card, /^<button type="button"/);
  assert.match(card, /aria-pressed="true" aria-controls="stockDock"/);
  for (const label of ['Production','Stock cover','Demand','Net flow']) assert(card.includes(`data-label="${label}"`));
  for (const text of ['25.5','30','−4.5','59 d','t/day',c.row.second]) assert(card.includes(text));
  c.row.tracked = false; c.row.status = 'presence';
  const presence = vm.runInContext('stockRowHtml(row)', c);
  assert(!presence.includes('25.5'), 'presence-only rows must not invent tonnage');
  assert(presence.includes(c.row.second));
});
test('monthly-compatible server rows render explicit daily forecasts without touching stocks', () => {
  const c = context(['fmtQ','fmtSigned','dailyRowView','stockRowHtml']);
  c.row = {id:'copper',name:'Copper',status:'short',tracked:true,cover_months:2,cover_days:62,
    produce:132,need:0.548167,net:131.452,unit:'kt/mo',
    produce_per_day:4.258065,need_per_day:0.017683,daily_unit:'kt/day',
    sentence:'you make 132 kt/mo, lines need 0.55',second:'Production is apportioned; deposits are presence-only.'};
  const card = vm.runInContext('stockRowHtml(row)', c);
  assert(card.includes('4.26'));
  assert(card.includes('0.017683'));
  assert(card.includes('62 d'));
  assert(card.includes('kt/day'));
  assert(!card.includes('kt/mo'));
  assert.equal(c.row.produce,132,'projection must not mutate server state');
  assert.equal(c.row.cover_days,62);
});
test('native controls keep Enter, Space, text and arrows in both rooms', () => {
  for (const name of ['stockKeys','techKeys']) {
    let prevented = 0;
    const c = context([name], {window:{}, closeStock:()=>assert.fail('native input closed the room')});
    for (const key of ['Enter',' ','ArrowUp','ArrowDown','b','1']) {
      c.event = {key,target:{closest:()=>({})},preventDefault(){prevented++;}};
      vm.runInContext(`${name}(event)`, c);
    }
    assert.equal(prevented, 0, `${name} stole a native control key`);
  }
});
test('discovery layout does not enlarge or recalculate the technology graph', () => {
  assert.match(html, /const TK = \{ W:180, H:56,/);
  assert.match(html, /const ST = \{ COLW:244, ROWGAP:16, TOP:34, LEFT:24 \}/);
  assert.doesNotMatch(css, /(?:zoom\s*:|\.tsvg[^{}]*\{[^}]*font(?:-size)?\s*:)/,
    'graph coordinates and SVG label sizes must remain under the renderer');
  assert.match(css, /#techScreen #techStageWrap, #techScreen #techViewport \{ top: 162px;/);
  assert.match(css, /#techScreen #techTabs \{ top: 72px; height: 90px;/);
  assert.match(source('techTipPlace'), /getBoundingClientRect\(\)\.bottom/);
  assert.match(source('stockTipPlace'), /getBoundingClientRect\(\)\.bottom/);
});
test('expanded quote/contract details survive resource rerenders and offers stay actionable', () => {
  const render = source('renderStock');
  assert(render.includes('details[open][id]'));
  assert(render.includes('if (el) el.open = true'));
  assert(render.includes('id="stockMarketQuotes"'));
  assert(render.includes('id="stockContracts"'));
  assert(render.includes('o.expires_in_days'));
  assert(render.includes('answerOffer(+b.dataset.offer, b.dataset.answer)'));
  assert(render.includes('cancelDeal(+b.dataset.cancel'));
});
test('trade draft shape and server-evaluated negotiation controls stay intact', () => {
  const talks = {to:'Chile',com:'copper',months:36,
    get:{commodity:'copper',rung:2,district:null},
    give:{money_rung:1,commodity:null,district:'province-7'}};
  const c = context(['talksDraft'], {talks});
  assert.deepEqual(JSON.parse(vm.runInContext('JSON.stringify(talksDraft())',c)), talks);
  const render = source('renderTalks');
  for (const text of ['ev.sentence','ev.pluses','ev.minuses','ev.pc','data-get-rung','data-money-rung',
    'data-give-rung','data-get-district','data-give-district','data-months','Daily deliveries']) assert(render.includes(text));
  assert(render.includes('aria-pressed="${on}"'));
  assert.match(source('talksOffer'), /kind: "propose_deal", \.\.\.talksDraft\(\), take_terms: !!takeTerms/);
  assert(source('renderStockDock').includes('live && t.affordable ? "" : "disabled"'), 'last-resort war remains gated');
});
test('technology inspector keeps sim effects, daily research and focus commands', () => {
  const dock = source('techDockHtml');
  assert(dock.includes('n.effects.map((e) =>'));
  assert(dock.includes('rd.rate_daily'));
  assert(dock.includes('switching forfeits half'));
  assert(dock.includes('Discoveries this unlocks'));
  assert(dock.includes('data-act="focus"'));
  assert(dock.includes('data-act="stand"'));
  assert(dock.includes('data-fly="${pi}"'));
  assert(source('techDockDo').includes('window.setFocus(n.domain, n.id)'));
  assert(source('renderTechMenu').includes('${r.daily.toFixed(2)} points a day'));
});
test('discovery rooms enter with focus inside and restore their opener', async () => {
  for (const [name,id,stateName] of [['Stock','stockScreen','stock'],['Tech','techScreen','tech']]) {
    assert(html.includes(`id="${id}" role="dialog" aria-modal="true"`));
    const opener = {isConnected:true,getClientRects:()=>[{}],focus(){focused=this;}};
    let focused = opener;
    const close = {focus(){focused=this;}};
    const room = {style:{},dataset:{}};
    const state = {open:false,data:{},sel:'copper',curDomain:'Energy'};
    const c = context([`open${name}`,`close${name}`], {
      S:{resources:{},research:{domains:[{domain:'Energy'}]}},
      stock: stateName==='stock'?state:{open:false}, tech:stateName==='tech'?state:{open:false},
      PROD:{open:false},LOGI:{open:false},document:{activeElement:opener},window:{},
      $:selector=>selector===`#${id}`?room:close,
      dominationIsOpen:()=>false,closeGameDrawers(){},closeTechMenu(){},closeSheet(){},
      stockRows:()=>[{id:'copper'}],renderStock(){},stockFetchCards(){},stockTipHide(){},
      techWire(){},refreshTech(){},setTechView(){},cancelAnimationFrame(){},techCamAnim:0,
    });
    await vm.runInContext(`open${name}()`,c);
    assert.equal(focused,close,`${name} left focus behind the modal`);
    assert.equal(state.open,true);
    vm.runInContext(`close${name}()`,c);
    assert.equal(focused,opener,`${name} did not restore the opener`);
    assert.equal(state.open,false);
  }
});
test('negotiations retain focus through asynchronous verdicts and bound the partner list', () => {
  const render = source('renderTalks');
  assert(render.includes('id="talksClose" aria-label="Close trade negotiations"'));
  assert(render.includes('replacement.focus({ preventScroll: true })'));
  assert(render.includes('talks.partnerQuery = partnerSearch.value'));
  assert(render.includes('partnerSearch.value = talks.partnerQuery || ""'));
  assert(render.includes('role="group" aria-label="Trading partners"'));
  assert.match(css, /\.trade-partner-list \{ max-height: clamp\(240px,48dvh,560px\); overflow-y: auto/);
  assert.match(css, /\.trade-partner-list \{ max-height: 190px;/);
  assert.match(source('openTalks'), /renderTalks\(\);\s+openSheetEl\(false\)/);
});
test('technology tabs rove, wrap, and remain selected without changing the graph', () => {
  let focused, prevented=0, stopped=0;
  const tabs=['Energy','Computing','all'].map(domain=>({
    dataset:{dom:domain},attributes:{},tabIndex:-1,
    setAttribute(k,v){this.attributes[k]=v;},focus(){focused=this;},scrollIntoView(){},
    closest(){return this;},
  }));
  const dummy={style:{},dataset:{}};
  const c=context(['setTechView','techTabKeys'],{
    tech:{data:null},window:{},techClearHighlight(){},
    document:{querySelectorAll:s=>s==='#techTabs .dtab'?tabs:[]},
    $:s=>s==='#techTabs'?{querySelectorAll:()=>tabs}:dummy,
  });
  vm.runInContext('setTechView("Energy")',c);
  assert.deepEqual(tabs.map(t=>t.tabIndex),[0,-1,-1]);
  focused=tabs[0];
  for(const [key,expected] of [['ArrowLeft',2],['Home',0],['ArrowRight',1],['End',2],['ArrowRight',0]]) {
    c.event={key,target:focused,preventDefault(){prevented++;},stopPropagation(){stopped++;}};
    vm.runInContext('techTabKeys(event)',c);
    assert.equal(focused,tabs[expected]);
    assert.equal(tabs.filter(t=>t.tabIndex===0).length,1);
    assert.equal(focused.attributes['aria-selected'],'true');
  }
  assert.equal(prevented,5);assert.equal(stopped,5);
});
test('research chooser blocks blind time while preserving native activation and Escape', () => {
  const match=/document\.addEventListener\("keydown", \(e\) => \{\r?\n  const room = arcadeTopRoom\(\);/.exec(html);
  assert(match);
  const end=html.indexOf('\n});',match.index);
  let listener,advanced=0,closed=0,opened=0;
  const c=vm.createContext({document:{addEventListener:(type,fn)=>listener=fn},
    arcadeTopRoom:()=>({id:'techMenu'}),cabinetIsOpen:()=>false,dominationIsOpen:()=>false,
    focused:()=>false,typing:()=>false,keysCardIsOpen:()=>false,isKeysCardToggle:()=>false,
    techMenuIsOpen:()=>true,closeTechMenu:()=>closed++,openTech:()=>opened++,
    S:{},tech:{open:false},stock:{open:false},$:()=>({style:{display:'block'}}),advance:()=>advanced++,
  });
  vm.runInContext(html.slice(match.index,end+4),c);
  let prevented=0;
  for(const key of [' ','2','3','4','Enter','Escape','t']) listener({key,target:{tagName:'BUTTON',closest:()=>({})},preventDefault(){prevented++;}});
  assert.equal(advanced,0);
  assert.equal(closed,1);
  assert.equal(opened,1);
  assert.equal(prevented,0,'Space/Enter must still activate the focused chooser button');
  assert(source('toggleTechMenu').includes('.focus({ preventScroll: true })'));
  assert(source('closeTechMenu').includes('target?.focus({ preventScroll: true })'));
  assert(html.includes('id="techMenu" role="dialog" aria-modal="true"'));
});
test('resource tooltips retire on close, reopen and scrolling and ignore touch hover',()=>{
  let removed=false;
  const tip={style:{},classList:{remove(){removed=true;}},setAttribute(k,v){this[k]=v;}};
  const c=context(['stockTipHide'],{$:()=>tip});
  vm.runInContext('stockTipHide()',c);
  assert(removed);assert.equal(tip.style.visibility,'hidden');assert.equal(tip['aria-hidden'],'true');
  assert(source('openStock').includes('stockTipHide();'));
  assert(source('closeStock').includes('stockTipHide();'));
  assert(source('renderStock').includes('e.pointerType !== "touch"'));
  assert(source('renderStock').includes('.onscroll = stockTipHide'));
  assert(source('renderStock').includes('.onpointerleave = stockTipHide'));
});
