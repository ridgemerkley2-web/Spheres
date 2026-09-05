// Run: node --test tools/ui/check_operations_arcade.cjs
// Executes the actual production/logistics renderers, not copied UI logic.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');

const root = path.resolve(__dirname, '../..');
const page = fs.readFileSync(path.join(root, 'spheres-web/ui/index.html'), 'utf8');
const styles = fs.readFileSync(path.join(root, 'spheres-web/ui/arcade-operations.css'), 'utf8');
function source(name) {
  const found = new RegExp(`^(?:async\\s+)?function ${name}\\(`, 'm').exec(page);
  assert(found, `Missing actual renderer ${name}`);
  const first = page.slice(found.index, page.indexOf('\n', found.index));
  if (/\}\s*$/.test(first)) return first;
  const end = page.indexOf('\n}', found.index);
  assert(end > found.index, `Missing closing brace for ${name}`);
  return page.slice(found.index, end + 2);
}
const helpers = [
  'operationsSceneHtml', 'operationsHeroHtml', 'operationsSetMapView',
  'operationsDetailsState', 'operationsRestoreDetails',
  'productionQueue', 'productionCatalog', 'productionProvinces', 'productionCompleted',
  'productionKind', 'productionStatus', 'productionTone', 'productionProvince',
  'productionProgress', 'productionPriorityChoices', 'productionCanCancel',
  'productionRequirements', 'productionCapabilityPairs', 'productionModuleLabel', 'productionNeedHtml',
  'productionCardHtml', 'productionSummary', 'productionStartAllowed', 'productionBuiltHtml',
  'productionFundingLabel', 'productionCatalogHtml', 'productionEligible', 'productionProvinceHtml',
  'manufacturingLines', 'manufacturingCatalog', 'manufacturingProvinces', 'manufacturingHoldings',
  'manufacturingOrders', 'manufacturingKit', 'manufacturingTone', 'manufacturingClassMark',
  'manufacturingBn', 'manufacturingRequirementHtml', 'manufacturingPriorityChoices',
  'manufacturingCanStop', 'manufacturingLineHtml', 'manufacturingLedgerHtml',
  'manufacturingLinesHtml', 'manufacturingCatalogHtml', 'manufacturingProvinceHtml',
  'logisticsLaneId', 'logisticsEndpoint', 'logisticsEndpointName', 'logisticsCommodity',
  'logisticsCommodityName', 'logisticsState', 'logisticsTone', 'logisticsStateWord',
  'logisticsMonthLabel', 'logisticsNumber', 'logisticsRoute', 'logisticsDisplayNodes',
  'logisticsCalendar', 'logisticsQuantity', 'logisticsActions', 'logisticsAllows',
  'logisticsEscAttr', 'logisticsIsMine', 'logisticsFilteredLanes', 'logisticsSummary',
  'logisticsFindLane', 'logisticsCardHtml', 'logisticsCargoHtml', 'logisticsPolicyHtml',
];
function fixture(extra = []) {
  const body = { innerHTML: '', cards: [], buttons: [], querySelector: () => null,
    querySelectorAll(selector) {
      if (['[data-prod-project]', '[data-manu-line]', '[data-logi-lane]'].includes(selector)) return this.cards;
      if (['[data-prod-map]', '[data-manu-map]'].includes(selector)) return this.buttons;
      return [];
    } };
  const panelClasses = new Set();
  const panel = { attributes: {}, isConnected: true, getClientRects: () => [1],
    setAttribute(name, value) { this.attributes[name] = value; }, focus() {},
    classList: { toggle(name, value) { if (value) panelClasses.add(name); else panelClasses.delete(name); },
      add(...names) { names.forEach(name=>panelClasses.add(name)); }, remove(...names) { names.forEach(name=>panelClasses.delete(name)); } } };
  const c = vm.createContext({
    assert, body, panel, panelClasses, window: {}, calls: [],
    $: key => key.endsWith('Body') ? body : panel,
    escText: value => String(value).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;'),
    clamp: (n, min, max) => Math.min(max, Math.max(min, n)),
    fmtQ: value => String(value),
    rglyph: () => '<svg aria-hidden="true"></svg>',
    stockHue: () => '#abc', stockRows: () => [], nationById: () => null,
    closeSheet: () => c.calls.push('close-sheet'),
    document: { activeElement: null, body: {classList:{add(){},remove(){}}} },
    closeStock:()=>{},closeTech:()=>{},closeGameDrawers:()=>{},closeTechMenu:()=>{},showTab:()=>{},
    renderProductionDock:()=>{},renderLogisticsDock:()=>{},renderLogisticsPanel:()=>{},logisticsFetch:()=>{},setProductionMode:()=>{},
    productionSelect: (id, focus) => c.calls.push(['project', id, focus]),
    manufacturingSelect: (id, focus) => c.calls.push(['line', id, focus]),
    wireProductionPanel: () => {}, renderManufacturingPanel: () => {},
    api: async (url, body) => { c.calls.push({url, body}); return {}; },
    adopt: async () => {}, banner: message => c.calls.push(message),
  });
  vm.runInContext(`
    const PROD={open:true,mode:'build',view:'queue',selected:null,data:{queue:[],catalog:[],provinces:[],capacity:4,actions:{start:true}}};
    const MANU={selected:null,view:'lines',classFilter:'All',showLocked:false,data:{lines:[],catalog:[],provinces:[],stockpile:[],orders:[]}};
    const LOGI={open:true,com:'all',expanded:false,cargoExpanded:false,selected:null,data:{lanes:[],cargo:[],arrivals:[]}};
    const LOGI_BLOCKED=new Set(['blocked','closed']);
    const LOGI_CONSTRAINED=new Set(['supply_short','capacity_limited']);
    const PROD_ICON={infrastructure:'▥',arms_plant:'⛭'};
    const MANU_CLASS_MARK={armour:'▰'};
    const RES_ALL='all',RESOURCES={},MONTHS=['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
    const S={player:'USA',production_summary:{}}; const GLOBE=null; const stock={open:false},tech={open:false};
    function me(){return {id:'USA',name:'United States'};}
    ${[...new Set([...helpers, ...extra])].map(source).join('\n')}
  `, c);
  return c;
}
const run = (context, code) => vm.runInContext(code, context, { timeout: 1000 });
const project = { id: 7, name: 'Rail & roads', kind: 'infrastructure', province: {id:'US-CA',name:'California'},
  progress: .4, eta_days: 60, status: 'building', priority: 'normal',
  requirements: [{name:'Iron',required:20,consumed:8,unit:'kt',shortfall:0}],
  actions: {set_priority:['high','low'],cancel:true} };
const line = { id: 9, name: 'Armour programme', kit:'arm_gen3', class:'armour',
  province:{id:'US-CA',name:'California'}, status:'producing', priority:'normal',
  allocation_bn_day:.03,allocation_bn_actual_day:.03,throughput_ratio:1,
  lead_days:450,units_planned_day:4.5,units_ordered_day:4.5,ordered_bn:8,
  requirements:[{commodity:'iron',name:'Iron',draw:2,required:2,unit:'kt/day',stock_unit:'kt',stock_available:4,priority_available:2,shortfall:0}],
  actions:{set_priority:['high','low'],stop:true} };
const lane = {id:'shipment-1',from:'Canada',to:'USA',from_name:'Canada',to_name:'United States',
  commodity:'iron',commodity_name:'Iron',unit:'kt',requested:4,dispatched:3,unshipped:1,state:'supply_short',
  reason:'Seller has 3 kt available.',capacity_period:'day',settled_day:'2 Jan 1990',source:'contract',contract_id:5,
  actions:['focus','open_contract'],route:{estimated_days:12,capacity_tonnes:8000,bottleneck:'Windsor gateway',mode:'land',nodes:[{id:'a',name:'Ontario'},{id:'b',name:'Michigan'}]} };

test('the entire shipped inline JavaScript remains syntactically valid', () => {
  const scripts=[...page.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/g)].map(m=>m[1]).filter(s=>s.trim());
  assert(scripts.length);
  scripts.forEach(script=>new vm.Script(script));
});
test('operations styling stays scoped, readable and touch-sized', () => {
  assert.match(styles, /:is\(#productionPanel, #logisticsPanel\)\s*\{[\s\S]*?position: fixed;[\s\S]*?inset: 16px;/);
  assert.match(styles, /min-height: 46px/);
  for(const match of styles.matchAll(/font-size:\s*(\d+)px/g)) assert(+match[1]>=14, 'No miniature body labels');
  assert.doesNotMatch(styles, /font:[^;\n]+\s+inherit;/, 'font shorthand cannot combine inherit with a size');
  const selectors=styles.split('\n').filter(line=>/^[^\s@/*].*\{/.test(line));
  selectors.forEach(line=>assert(/^(:is\(#productionPanel|#productionPanel|#logisticsPanel)/.test(line), `Unscoped style: ${line}`));
  assert.match(styles, /prefers-reduced-motion/);
  assert.match(styles, /\.map-view/);
});
test('production overview keeps one clear start action and exact server progress', () => {
  const c=fixture(['renderProductionPanel']); c.project=project;
  run(c,'PROD.data.queue=[project];renderProductionPanel();');
  assert.match(c.body.innerHTML,/Build what comes next/);
  assert.equal((c.body.innerHTML.match(/data-prod-new/g)||[]).length,1);
  assert.match(c.body.innerHTML,/40%/); assert.match(c.body.innerHTML,/60 days left/);
  assert.match(c.body.innerHTML,/1 of 4 national project slots/);
  assert.match(c.body.innerHTML,/<details class="operations-details"><summary>Materials &amp; project priority/);
  assert.match(c.body.innerHTML,/Iron · 8\/20 kt/);
  assert.match(c.body.innerHTML,/data-prod-priority="high" data-prod-id="7"/);
  assert.match(c.body.innerHTML,/data-prod-map="7"/);
  run(c,'PROD.selected="7";renderProductionPanel();');
  assert.match(c.body.innerHTML,/<details class="operations-details" open>/);
});
test('project catalogue and province choice retain eligibility and authored costs', () => {
  const c=fixture();
  run(c,`PROD.data.catalog=[{kind:'infrastructure',name:'Infrastructure',base_days:180,pc_cost:8,effect:'More corridor capacity',eligible_provinces:['US-CA']}];
    PROD.data.provinces=[{id:'US-CA',name:'California'},{id:'US-NY',name:'New York'}];PROD.pickKind='infrastructure';`);
  const catalog=run(c,'productionCatalogHtml()');
  assert.match(catalog,/data-prod-kind="infrastructure"/);assert.match(catalog,/180 work-days · 8 PC/);
  const provinces=run(c,'productionProvinceHtml()');
  assert.match(provinces,/data-prod-province="US-CA"/);assert.doesNotMatch(provinces,/US-NY/);
});

test('completed fractional workshops stay visible without becoming full-site levels', () => {
  const c=fixture();
  run(c,`PROD.showBuilt=true;PROD.data.completed=[{province:{id:'US-CA',name:'California'},
    capabilities:{civilian_industry:0,power_grid:0},module_capacity:0.012345}];`);
  const built=run(c,'productionBuiltHtml()');
  assert.match(built,/Starter workshop · 1\.2345% standard capacity/);
  assert.match(built,/0 full-site levels · 1 workshop province/);
  assert.doesNotMatch(built,/civilian industry 1|power grid 1/);
  run(c,`PROD.data.catalog=[{kind:'infrastructure',eligible_provinces:['US-CA']}];
    PROD.data.provinces=[{id:'US-CA',name:'California',module_capacity:0.012345,
    capabilities:{civilian_industry:0,power_grid:0}}];PROD.pickKind='infrastructure';`);
  const pick=run(c,'productionProvinceHtml()');
  assert.match(pick,/Starter workshop · 1\.2345% standard capacity/);
  assert.doesNotMatch(pick,/available ground/);
});
test('manufacturing displays actual daily draw without changing it to a monthly recipe', () => {
  const c=fixture();c.line=line;
  const html=run(c,'manufacturingLineHtml(line)');
  assert.match(html,/\$0\.03bn/);assert.match(html,/450 days/);
  assert.match(html,/4\.5 units last day · 4\.5 planned/);assert.match(html,/Iron · 2 kt\/day/);
  assert.match(html,/national warehouse 4 kt/);assert.doesNotMatch(html,/class="work-need short"/);
  assert.match(html,/<details class="operations-details"><summary>Daily materials &amp; line priority/);
  assert.match(html,/data-manu-priority="high" data-manu-id="9"/);
  assert.match(html,/data-manu-stop="9"/);assert.match(html,/data-manu-map="9"/);
  run(c,'line.status="blocked";line.reason="Needs more iron";line.units_planned_day=0;line.units_ordered_day=0;line.allocation_bn_actual_day=0;line.throughput_ratio=0;');
  assert.match(run(c,'manufacturingLineHtml(line)'),/0 units last day · 0 planned/);
});
test('empty manufacturing gives a useful next step and does not invent free plant capacity', () => {
  const c=fixture();
  run(c,'MANU.data.summary={capacity:0,free_slots:0};MANU.data.finance={procurement_budget_bn_day:.25,procurement_share:.2};');
  const html=run(c,'manufacturingLinesHtml()');
  assert.match(html,/data-manu-build-plant/);assert.doesNotMatch(html,/data-manu-new/);
  assert.match(html,/\$0\.25bn/);assert.match(html,/20% of the defense budget/);
});
test('freight cards preserve delivered quantities, date, holds and daily corridor units', () => {
  const c=fixture();c.lane=lane;
  const html=run(c,'logisticsCardHtml(lane)');
  assert.match(html,/4 kt/);assert.match(html,/3 kt/);assert.match(html,/1 kt/);
  assert.match(html,/2 Jan 1990/);assert.match(html,/8 kt\/day/);
  assert.match(html,/12 days · before any hold/);assert.match(html,/Windsor gateway/);
  assert.match(html,/<details class="operations-details"><summary>Route, capacity &amp; waypoints/);
  assert.match(html,/data-logi-action="focus"/);assert.match(html,/data-logi-action="open_contract"/);
  c.cargo={...lane,id:3,quantity:2,due_day:'14 Jan 1990',hold_reason:'Gateway closed'};
  const cargo=run(c,'logisticsCargoHtml(cargo,false)');
  assert.match(cargo,/HELD/);assert.match(cargo,/Gateway closed/);assert.match(cargo,/14 Jan 1990/);
});
test('freight room shows cargo first and tucks the dispatch ledger away', () => {
  const c=fixture(['renderLogisticsPanel']);c.lane=lane;
  run(c,`LOGI.data.lanes=[lane];LOGI.data.cargo=[{...lane,id:'cargo-1',quantity:3,due_day:'14 Jan 1990'}];
    LOGI.data.policy={selected:'fastest',options:[{id:'fastest',label:'Fastest',description:'Use the shortest modeled journey.'}]};renderLogisticsPanel();`);
  assert.match(c.body.innerHTML,/Keep your nation moving/);
  assert.match(c.body.innerHTML,/Cargo on the move/);
  assert.match(c.body.innerHTML,/<details class="operations-details"><summary>Latest dispatches/);
  assert.match(c.body.innerHTML,/data-logi-policy="fastest" class="on" aria-pressed="true"/);
  assert.match(c.body.innerHTML,/Use the shortest modeled journey/);
});
test('room selection never moves the globe until the explicit map button is pressed', () => {
  for(const [wire,kind,selectKey,mapKey] of [['wireProductionPanel','build','prodProject','prodMap'],['wireManufacturingPanel','manufacture','manuLine','manuMap']]) {
    const c=fixture([wire]);const card={dataset:{[selectKey]:'7'}};const button={dataset:{[mapKey]:'7'}};
    c.body.cards=[card];c.body.buttons=[button];
    run(c,`PROD.mode=${JSON.stringify(kind)};${wire}();`);
    card.onclick({target:{closest:()=>null}});
    assert.equal(c.calls.at(-1)[2],false);
    const count=c.calls.length;
    card.onkeydown({target:button,key:'Enter',preventDefault(){throw new Error('Native button key was intercepted');}});
    assert.equal(c.calls.length,count);
    card.onclick({target:{closest:()=>({tagName:'SUMMARY'})}});
    assert.equal(c.calls.length,count,'Native details must not trigger card navigation');
    button.onclick();assert(c.panelClasses.has('map-view'));assert(c.calls.includes('close-sheet'));
    assert.equal(c.calls.at(-1)[2],true);
  }
});
test('map mode tells the shared modal coordinator and back-to-room restores it', () => {
  const c=fixture();c.window.arcadeRoomMapViewChanged=(panel,on)=>c.calls.push(['room',on]);
  run(c,'operationsSetMapView("productionPanel",true);');
  assert(c.panelClasses.has('map-view'));assert.deepEqual(c.calls.at(-1),['room',true]);
  run(c,'operationsSetMapView("productionPanel",false);');
  assert(!c.panelClasses.has('map-view'));assert.deepEqual(c.calls.at(-1),['room',false]);
});
test('expanded details survive daily repaint while selecting a new card can reveal it', () => {
  const c=fixture();let selected=false;
  const card={dataset:{prodProject:'7'},classList:{contains:()=>selected}};
  const detail={open:true,closest:()=>card,querySelector:()=>({textContent:'Materials & project priority'})};
  c.body.querySelectorAll=selector=>selector==='details'?[detail]:[];
  run(c,'const remembered=operationsDetailsState(body);');
  detail.open=false;
  run(c,'operationsRestoreDetails(body,remembered);');
  assert.equal(detail.open,true,'open resource details survive the next day');
  detail.open=false;
  run(c,'const closed=operationsDetailsState(body);');
  selected=true;detail.open=true;
  run(c,'operationsRestoreDetails(body,closed);');
  assert.equal(detail.open,true,'a newly selected card may open its own inspector');
});
test('routing policy still posts only its existing command and no simulation arithmetic', async () => {
  const c=fixture(['logisticsSetPolicy']);
  await run(c,'logisticsSetPolicy("avoid_chokepoints")');
  const call=c.calls.find(call=>call && call.url);
  assert.equal(call.url,'/api/command');
  assert.deepEqual(JSON.parse(JSON.stringify(call.body)),{commands:[{kind:'set_logistics_policy',policy:'avoid_chokepoints'}]});
});
test('opening and closing operations resets map mode, modality and launch focus', () => {
  const c=fixture(['openProduction','closeProduction','openLogistics','closeLogistics']);
  let restored=0;
  const launcher={isConnected:true,getClientRects:()=>[1],focus(){restored++;}};
  for(const [open,close,state] of [['openProduction','closeProduction','PROD'],['openLogistics','closeLogistics','LOGI']]) {
    run(c,'PROD.open=false;LOGI.open=false;');
    c.document.activeElement=launcher;c.panelClasses.add('map-view');
    run(c,`${open}();`);
    assert(!c.panelClasses.has('map-view'));
    assert.equal(c.panel.attributes['aria-modal'],'true');
    assert.equal(c.panel.attributes['aria-hidden'],'false');
    assert.equal(run(c,`${state}.open`),true);
    c.panelClasses.add('map-view');const before=restored;
    run(c,`${close}();`);
    assert(!c.panelClasses.has('map-view'));
    assert.equal(c.panel.attributes['aria-modal'],'false');
    assert.equal(c.panel.attributes['aria-hidden'],'true');
    assert.equal(run(c,`${state}.open`),false);
    assert.equal(restored,before+1);
  }
});
