// Actual manufacturing renderers and click wiring; no copied pricing or slot arithmetic.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const page=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/index.html'),'utf8');
const economy=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/province-economy-ui.js'),'utf8');
function source(name,text=page){
  const found=new RegExp(`^function ${name}\\(`,'m').exec(text);assert(found,name);
  const line=text.slice(found.index,text.indexOf('\n',found.index));
  return /}\s*$/.test(line)?line:text.slice(found.index,text.indexOf('\n}',found.index)+2);
}
function fixture(){
  const routes=[],button={};
  const body={querySelector:()=>null,querySelectorAll:selector=>selector==='[data-manu-component-supply]'?[button]:[]};
  const c=vm.createContext({Number,window:{},S:{player:'France',economic_competition:false},PROD:{mode:'manufacture'},
    MANU:{data:{catalog:[],provinces:[]}},MANU_CLASS_MARK:{naval:'⚓'},
    $:()=>body,escText:value=>String(value).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;'),
    fmtQ:String,clamp:(value,min,max)=>Math.min(max,Math.max(min,value)),operationsHeroHtml:()=>'',closeProduction:()=>routes.push('close'),
    openConstruction:action=>routes.push(JSON.parse(JSON.stringify(action))),
    industryNavigate:action=>routes.push(JSON.parse(JSON.stringify(action)))});
  vm.runInContext(source('economyMoney',economy),c);
  vm.runInContext(['manufacturingCatalog','manufacturingKit','manufacturingProvinces','manufacturingClassMark',
    'manufacturingBn','logisticsEscAttr','manufacturingComponentsHtml','manufacturingProvinceHtml','wireManufacturingPanel',
    'manufacturingTone','manufacturingKit3d','manufacturingPriorityChoices','manufacturingCanStop','manufacturingRequirementHtml','manufacturingLineHtml',
    'manufacturingLines','manufacturingHoldings','manufacturingOrders','manufacturingLedgerHtml','manufacturingLinesHtml'].map(name=>source(name)).join('\n'),c);
  return {c,routes,button};
}
test('component remedy offers domestic construction while the optional trade mode is off',()=>{
  const {c,routes,button}=fixture(),line={advanced_components:{required:2,stock:0,unit:'packs',cadence:'daily'}};
  const html=c.manufacturingComponentsHtml(line);
  assert.match(html,/Review Advanced Industry/);assert.doesNotMatch(html,/Review component suppliers/);
  c.wireManufacturingPanel();button.onclick();
  assert.deepEqual(routes,['close',{kind:'advanced_industry'}]);
  assert.equal(c.S.economic_competition,false,'Navigation does not enable a different game mode');
});
test('component remedy retains the supplier route when manufactured trade is enabled',()=>{
  const {c,routes,button}=fixture();c.S.economic_competition=true;
  assert.match(c.manufacturingComponentsHtml({advanced_components:{required:2,stock:0}}),/Review component suppliers/);
  c.wireManufacturingPanel();button.onclick();
  assert.deepEqual(routes,['close',{action:'trade',good:'advanced_components'}]);
});
test('legacy equipment needs no component remedy while unfunded advanced equipment retains one',()=>{
  const {c}=fixture();
  const components={required:0,stock:0,unit:'packs',cadence:'daily'};
  assert.equal(c.manufacturingComponentsHtml({tech:null,advanced_components:components}),'');
  assert.match(c.manufacturingComponentsHtml({tech:{id:'aero_stealth_multirole'},advanced_components:components}),/Review Advanced Industry/);
});
test('naval selection quotes only shipyard capacity and shows the opening political charge',()=>{
  const {c}=fixture();c.MANU.pickKit='nav_escort';
  c.MANU.data.catalog=[{id:'nav_escort',name:'Escort',class:'naval',facility_kind:'shipyard',pc_cost:8,unit_cost_bn:.18,lead_days:2192,eligible_provinces:['FRA-port']}];
  c.MANU.data.provinces=[{id:'FRA-port',name:'Port',arms_plants:7,free_slots:5,military_factory_slots:5,military_factory_free_slots:4,shipyard_slots:2,shipyard_free_slots:1}];
  const before=JSON.stringify(c.MANU.data),html=c.manufacturingProvinceHtml();
  assert.match(html,/1 free · Shipyard 2/);assert.doesNotMatch(html,/5 free|arms plant 7/);
  assert.match(html,/8 PC to open this line/);
  assert.equal(JSON.stringify(c.MANU.data),before);
});
test('land selection excludes spare shipyards from the displayed free count',()=>{
  const {c}=fixture();c.MANU.pickKit='inf_light';
  c.MANU.data.catalog=[{id:'inf_light',name:'Infantry',class:'infantry',facility_kind:'arms_plant',pc_cost:8,unit_cost_bn:.01,lead_days:730,eligible_provinces:['FRA-port']}];
  c.MANU.data.provinces=[{id:'FRA-port',name:'Port',arms_plants:7,free_slots:5,military_factory_slots:2,military_factory_free_slots:1,shipyard_slots:5,shipyard_free_slots:4}];
  const html=c.manufacturingProvinceHtml();
  assert.match(html,/1 free · Military Factory 2/);assert.doesNotMatch(html,/5 free|Shipyard 5/);
  assert.match(html,/8 PC to open this line/);
});

test('small equipment prices and paid runs remain visible in normal currency units',()=>{
  const {c}=fixture();c.MANU.pickKit='jdam';
  c.MANU.data.catalog=[{id:'jdam',name:'JDAM',class:'air',facility_kind:'arms_plant',pc_cost:8,unit_cost_bn:.00003,lead_days:365,eligible_provinces:['FRA-port']}];
  c.MANU.data.provinces=[{id:'FRA-port',name:'Port',military_factory_slots:1,military_factory_free_slots:1}];
  const html=c.manufacturingProvinceHtml();
  assert.match(html,/\$30k per unit/);assert.doesNotMatch(html,/\$0\.000bn/);
  assert.equal(c.manufacturingBn(.000001),'$1k');
  assert.equal(c.manufacturingBn(.00000001),'$10');
  assert.equal(c.manufacturingBn(0),'$0');
});

test('manufacturing separates current unfunded plans from dated slowed runs and preserves the receipt',()=>{
  const {c}=fixture(),line={id:3,kit:'inf_light',name:'Infantry',status:'slowed',reason:'SLOWED: workers limit this line to 97% throughput today.',
    has_current_funding:false,current_blocker:null,units_planned_day:0,allocation_bn_actual_day:.03,throughput_ratio:.97,units_ordered_day:3,
    last_run:{day:4382,label:'31 Dec 2001',status:'slowed',reason:'SLOWED: workers limit this line to 97% throughput today.',allocation_bn_day:.03,units_day:3,throughput_ratio:.97}};
  const before=JSON.stringify(line),html=c.manufacturingLineHtml(line);
  assert.match(html,/>NO CURRENT FUNDING<\/span>/);
  assert.match(html,/Recorded · 31 Dec 2001 · SLOWED/);assert.match(html,/97% throughput on the recorded day/);
  assert.doesNotMatch(html,/throughput today/);assert.match(html,/0 units planned for the next day/);
  assert.match(html,/3 units recorded/);assert.match(html,/\$30m/);
  assert.equal(JSON.stringify(line),before);
  line.current_blocker='Province is occupied';line.status='blocked';line.allocation_bn_actual_day=0;line.units_ordered_day=0;line.throughput_ratio=0;
  const blocked=c.manufacturingLineHtml(line);
  assert.match(blocked,/>BLOCKED<\/span>/);assert.match(blocked,/Current constraint · Province is occupied/);
  assert.match(blocked,/3 units recorded/);assert.match(blocked,/\$30m/);
});
test('new manufacturing lines await a first run and never claim production before funding',()=>{
  const {c}=fixture(),line={id:4,name:'Escort',status:'producing',last_run:null,has_current_funding:false,units_planned_day:0};
  let html=c.manufacturingLineHtml(line);
  assert.match(html,/>NO CURRENT FUNDING<\/span>/);assert.match(html,/Awaiting first run/);
  assert.doesNotMatch(html,/>PRODUCING<\/span>|Recorded ·/);
  line.has_current_funding=true;line.units_planned_day=.01;html=c.manufacturingLineHtml(line);
  assert.match(html,/>AWAITING FIRST RUN<\/span>/);assert.doesNotMatch(html,/>PRODUCING<\/span>/);
});

test('loaded manufacturing history without a dated receipt does not claim a first run is pending',()=>{
  const {c}=fixture(),line={id:3,name:'F15',ordered_bn:16.735472,has_order_history:true,
    last_run:null,has_current_funding:false,units_planned_day:0};
  const before=JSON.stringify(line);let html=c.manufacturingLineHtml(line);
  assert.match(html,/>NO CURRENT FUNDING<\/span>/);
  assert.match(html,/No dated receipt available/);assert.doesNotMatch(html,/Awaiting first run|Recorded ·/);
  assert.match(html,/ordered to date/);assert.equal(JSON.stringify(line),before);
  line.has_current_funding=true;html=c.manufacturingLineHtml(line);
  assert.match(html,/>FUNDED PLAN<\/span>/);assert.doesNotMatch(html,/AWAITING FIRST RUN|Awaiting first run/);
  // Exact history flag survives even when a tiny cumulative value rounds to zero.
  line.ordered_bn=0;assert.match(c.manufacturingLineHtml(line),/No dated receipt available/);
  delete line.has_order_history;line.ordered_bn=16.735472;
  assert.match(c.manufacturingLineHtml(line),/No dated receipt available/);
});

test('a small paid manufacturing run remains visibly nonzero in its speed and dated reason',()=>{
  const {c}=fixture(),line={id:5,name:'Submarine',has_current_funding:true,units_planned_day:1,
    last_run:{day:4382,label:'31 Dec 2001',status:'slowed',reason:'SLOWED: inputs limit this line to 0% throughput today.',allocation_bn_day:.000001,units_day:.000001,throughput_ratio:.001}};
  const before=JSON.stringify(line),html=c.manufacturingLineHtml(line);
  assert.match(html,/Last run<b>&lt;1%<\/b>/);assert.match(html,/&lt;1% throughput on the recorded day/);
  assert.doesNotMatch(html,/to 0% throughput/);assert.match(html,/0.000001 units recorded/);
  assert.equal(JSON.stringify(line),before);
  line.last_run.throughput_ratio=0;
  assert.match(c.manufacturingLineHtml(line),/Last run<b>0%<\/b>/);
});

test('the recurring procurement estimate does not claim available funds or a paid treasury charge',()=>{
  const {c}=fixture();c.MANU.data.finance={procurement_budget_bn_day:.03,procurement_share:.2};
  const html=c.manufacturingLinesHtml();
  assert.match(html,/Budgeted daily procurement/);assert.match(html,/\$30m/);
  assert.match(html,/Current funding and dated spending are shown on each line/);
  assert.doesNotMatch(html,/treasury already carries that spending|Daily defense procurement/);
});
