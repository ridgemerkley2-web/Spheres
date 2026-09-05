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
    stock: {sel:'copper',horizon:90}, STATE_WORD:{short:'SHORT',presence:'PRESENCE',secure:'SECURE',watch:'WATCH',action:'ACTION NEEDED',stalled:'STALLED',market:'MARKET'},
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
  const c = context(['fmtQ','fmtSigned','dailyRowView','stockStrategicWindow','stockStrategicStatus','stockRowHtml']);
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
  const c = context(['fmtQ','fmtSigned','dailyRowView','stockStrategicWindow','stockStrategicStatus','stockRowHtml']);
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
test('strategic supply cards trust actual-block status from the served window, not a legacy prospective red', () => {
  const c = context(['fmtQ','fmtSigned','dailyRowView','stockStrategicWindow','stockStrategicStatus','stockRowHtml']);
  c.row={id:'copper',name:'Copper',status:'stalled',tracked:true,unit:'kt/day',sentence:'legacy prospective status',
    strategic:{horizons:[
      {days:30,status:'secure',headline:'Thirty is covered.',demand:4,covered:4,gap:0},
      {days:90,status:'watch',headline:'Server says plan for a refill.',demand:15,covered:12,gap:3},
      {days:365,status:'action',headline:'One-year structural gap.',demand:80,covered:30,gap:50},
    ]}};
  const before=JSON.stringify(c.row),card=vm.runInContext('stockRowHtml(row)',c);
  assert.match(card,/^<button type="button"/);
  assert.match(card,/supply-command-resource st-watch/);
  assert.match(card,/aria-pressed="true" aria-controls="stockDock"/);
  assert(card.includes('Server says plan for a refill.'));
  assert(!card.includes('Thirty is covered.'));assert(!card.includes('One-year structural gap.'));
  assert(!card.includes('15')&&!card.includes('12')&&!card.includes('3'),'collapsed card stays a plain-language brief');
  assert.equal(JSON.stringify(c.row),before,'rendering must not mutate or calculate the served forecast');
});
test('strategic supply summary leads with one server-owned mission and one primary action',()=>{
  const c=context(['stockStrategicMissionHtml','stockStrategicSummaryHtml']);
  c.resources={rows:[{id:'iron'},{id:'copper'},{id:'coal'},{id:'gas'}],strategic_summary:{headline:'Four lines need attention.',attention_count:4,
    mission:{id:'cover_active_supply',horizon_days:90,title:'Protect the next 90 days',objective:'Keep every active production and construction line supplied.',
      state:'at_risk',status_label:'NEEDS A DECISION',active_lines:4,secured_lines:1,attention_lines:3,blocked_lines:0,
      progress_percent:25,progress_label:'1 of 4 active materials secured',complete:false,focus_resource_id:'iron'},
    primary_action:{kind:'review_resource',resource_id:'iron',name:'Iron',severity:'run_gap',label:'Secure Iron now',detail:'Iron first.'},attention:[
    {id:'iron',name:'Iron',headline:'Iron first.'},{id:'copper',name:'Copper',headline:'Copper second.'},
    {id:'coal',name:'Coal',headline:'Coal third.'},{id:'gas',name:'Gas',headline:'Gas fourth.'},
  ]}};
  const html=vm.runInContext('stockStrategicSummaryHtml(resources)',c);
  assert(html.includes('Strategic Supply Command'));
  assert.equal((html.match(/aria-labelledby="supplyMissionTitle"/g)||[]).length,1);
  assert.equal((html.match(/id="supplyMissionTitle"/g)||[]).length,1);
  assert.match(html,/Protect the next 90 days/);
  assert.match(html,/1<span> \/ 4 secure<\/span>/);
  assert.match(html,/role="progressbar"[^>]*aria-valuemax="4"[^>]*aria-valuenow="1"/);
  assert.match(html,/style="width:25%"/,'the bar uses the served percent');
  assert.equal((html.match(/data-supply-primary=/g)||[]).length,1);
  assert.match(html,/data-supply-primary="review_resource"[^>]*data-resource-id="iron"/);
  assert.match(html,/Secure Iron now/);
  assert.match(html,/role="group" aria-label="Resource planning horizon"/);
  assert.equal((html.match(/data-stock-horizon=/g)||[]).length,3);
  assert.match(html,/data-stock-horizon="90" aria-pressed="true"/);
  assert.match(html,/mission stays fixed at 90 days/i,'the exploration horizon must not rerank the mission');
  assert.equal((html.match(/data-supply-alert=/g)||[]).length,2,'the primary risk is not duplicated in the folded queue');
  assert(html.includes('Iron first.')&&html.includes('Coal third.'));
  assert(!html.includes('Gas fourth.'));
  assert.doesNotMatch(source('stockStrategicMissionHtml'),/\.demand|\.covered|\.gap/,'the browser must not recalculate mission coverage from unlike quantities');
  assert.match(source('stockVisibleRows'),/return stockRows\(\)/,'all twelve resource buttons remain in the base view');
});
test('the mission action only navigates to a served resource or returns to command',()=>{
  let selected=[],closed=0;
  const c=context(['stockFollowObjective'],{
    stockRows:()=>[{id:'iron'},{id:'copper'}],stockSelect:id=>selected.push(id),closeStock:()=>closed++,
  });
  c.stock.horizon=30;
  vm.runInContext(`stockFollowObjective('review_resource','iron')`,c);
  vm.runInContext(`stockFollowObjective('review_resource','oil')`,c);
  vm.runInContext(`stockFollowObjective('return_to_map',null)`,c);
  assert.deepEqual(selected,['iron']);assert.equal(closed,1);assert.equal(c.stock.horizon,90,'mission navigation restores its fixed horizon');
  assert.doesNotMatch(source('stockFollowObjective'),/api\(|\/api\/command|mine|take|talks/i,'the mission CTA never executes a strategic command');
});
test('resources use separate overview and resource-action screens',()=>{
  assert.match(source('stockSelect'),/stock\.view = "resource"/);
  assert.match(source('stockShowOverview'),/stock\.view = "overview"/);
  assert.match(source('renderStock'),/classList\.toggle\("view-resource"/);
  const dock=source('renderStockDock');
  for(const label of ['Resource action screen','Choose a function','stock-action-grid','Supply overview']) assert(dock.includes(label),label);
  assert(dock.indexOf('${objectiveHtml}')<dock.indexOf('Choose a function'),'the selected resource objective precedes its choices');
  assert.match(source('stockFetchCards'),/focusResourceOnLoad[\s\S]*data-act=back/,'resource selection restores focus to a visible action-screen control');
  for(const rule of ['#stockScreen.view-overview #stockRows { right: 0; }','#stockScreen.view-overview #stockDock { display: none; }',
    '#stockScreen.view-resource #stockRows { display: none; }','#stockScreen.view-resource #stockDock { left: 0; width: auto;']) assert(css.includes(rule),rule);
});
test('horizon selection is presentation state and restores focus after rerender',()=>{
  let rendered=0,focused=0;
  const c=context(['stockSetHorizon'],{renderStock:()=>rendered++,document:{querySelector:selector=>{
    assert.equal(selector,'#stockRows [data-stock-horizon="365"]');return {focus:()=>focused++};
  }}});
  vm.runInContext('stockSetHorizon(365)',c);
  assert.equal(c.stock.horizon,365);assert.equal(rendered,1);assert.equal(focused,1);
  vm.runInContext('stockSetHorizon(365)',c);vm.runInContext('stockSetHorizon(31)',c);
  assert.equal(rendered,1,'same and unsupported horizons do not rerender or synthesize a window');
});
test('selected supply brief shows served Need Covered Gap, reason, sources and action',()=>{
  const c=context(['fmtQ','stockStrategicWindow','stockStrategicStatus','stockStrategicDetailHtml','stockActionObjectiveHtml']);
  c.row={id:'iron',name:'Iron',status:'short',stock:{unit:'kt'},strategic:{quantity_unit:'kt',storable:true,
    drivers:[{label:'Civilian factories',value:2.5,unit:'kt/day'}],horizons:[{
    days:90,demand:18.25,covered:12,gap:6.25,status:'action',headline:'Iron needs attention.',reason:'A funded mill needs more iron.',
    recommended_action:{kind:'review',label:'Secure supply',detail:'Compare trade and domestic extraction.'},
    sources:{stock:4,warehouse_stock:6,prior_claims:2,domestic_output:5,contracted_inbound:2,paid_inbound:1},
  }]}};
  const before=JSON.stringify(c.row),html=vm.runInContext('stockStrategicDetailHtml(row)',c);
  for(const text of ['90 day brief','Need','18.3 kt','Covered','12 kt','Gap','6.25 kt','A funded mill needs more iron.',
    'Iron needs attention.','How this forecast works','Secure supply','Compare trade and domestic extraction.','What needs this resource?','Civilian factories','2.5 kt/day',
    'Stock free after prior claims','Domestic output','Contracted inbound','Paid inbound · due',
    'Warehouse on hand','6 kt','Prior outbound claims','2 kt'])assert(html.includes(text),text);
  assert.equal(JSON.stringify(c.row),before);
  const cue=vm.runInContext('stockActionObjectiveHtml(row)',c);
  for(const text of ['Your 90-day objective','Iron needs attention.','Recommended move:','Secure supply','Need','Covered','Gap','18.3 kt','12 kt','6.25 kt'])assert(cue.includes(text),text);
  assert.equal(JSON.stringify(c.row),before,'the action cue preserves the server-owned forecast');
  c.row.id='oil';c.row.status='market';c.row.strategic.storable=false;c.row.strategic.storage_note='Priced flow; no physical oil pile.';
  const oil=vm.runInContext('stockStrategicDetailHtml(row)',c);
  assert(oil.includes('Priced flow; no physical oil pile.'));
  assert(!oil.includes('Warehouse on hand'),'oil must not acquire a warehouse breakdown');
});
test('supply command CSS keeps large controls, responsive cards and red exclusive to stalled',()=>{
  for(const token of ['.supply-command-horizons button { min-height: 44px',
    '.supply-command-alerts button { display: grid', '.supply-command-primary {', 'min-height: 56px',
    '.supply-command-risk-queue {', '.stockrow.supply-command-resource { min-height: 164px',
    '@media (max-width: 800px)', '@media (max-width: 480px)'])assert(css.includes(token),token);
  assert.match(css,/\.stockrow\.st-stalled \{ border-left-color: var\(--red\); \}/);
  assert.doesNotMatch(css,/\.stockrow\.st-(?:watch|action)[^{]*\{[^}]*var\(--red\)/);
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
  assert(source('renderStockDock').includes('c.row.strategic ? ""'),
    'strategic detail must not be contradicted by a legacy prospective status sentence');
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
  assert(source('stockTipShow').includes('stockStrategicWindow(r)'),'strategic hover must use the authoritative forecast window');
  assert(source('stockTipShow').includes('forecast ? forecast.headline'),'strategic hover must not repeat legacy prospective-red prose');
});
