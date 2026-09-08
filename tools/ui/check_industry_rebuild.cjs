// Run the shipped renderers and command adapters; all economics remain server-owned.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const root=path.resolve(__dirname,'../..');
const page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
const industry=fs.readFileSync(path.join(root,'spheres-web/ui/industry-ui.js'),'utf8');
const plain=value=>JSON.parse(JSON.stringify(value));
function fn(name){
  const match=new RegExp(`^(?:async )?function ${name}\\(`,'m').exec(page);assert(match,name);
  const end=page.indexOf('\n',match.index),line=page.slice(match.index,end);
  return /}\s*$/.test(line)?line:page.slice(match.index,page.indexOf('\n}',match.index)+2);
}
function fixture(){
  const orders=[],routes=[];
  const c=vm.createContext({console,Number,Math,Promise,window:{},S:{session_id:'one',player:'USA'},CAB:{tab:'industry'},
    PROD:{open:true,mode:'build',view:'queue',busy:false,stale:false,loading:false,data:{catalog:[],provinces:[],queue:[],mine_queue:[]}},
    PROD_ICON:{},MANU:{},advancing:false,pendingAdvance:null,
    economyMoney:value=>`$${value}bn`,renderProductionPanel(){},
    productionCommand:async(command)=>{orders.push(plain(command));return true;},
    openConstructionCabinet:tab=>routes.push(tab),closeProduction:()=>routes.push('close'),openStock:()=>routes.push('imports'),
    equipmentExternalPending:()=>false,closeEquipmentDrawer:()=>routes.push('close-equipment'),
    cabinetIsOpen:()=>true,industryNavigate:action=>routes.push(plain(action)),
  });
  const names=['productionQueue','productionProject','productionCatalog','productionKind','productionProvinces','productionEligible',
    'productionFundingLabel','constructionProvinceRefusal','logisticsEscAttr','constructionMoney',
    'constructionCapacityHtml','constructionBudgetHtml','constructionFundingDisclosureHtml','constructionAllocationHtml','constructionSetAllocation','constructionRemedyHtml','constructionOpenRemedy',
    'constructionProjectRole','productionCatalogHtml','constructionInvalidatePreview','constructionPreviewCurrent','constructionEffectValue','constructionEffectRows','constructionPreviewHtml','constructionConfirmPreview','equipmentNavigate','manufacturingComponentsHtml'];
  vm.runInContext(page.match(/^const escText\s*=.*;\s*$/m)[0]+'\n'+names.map(fn).join('\n')+'\n'+industry,c);
  c.openIndustry=options=>routes.push({industry:plain(options)});
  c.desk=vm.runInContext('IDESK',c);Object.assign(c.desk,{open:true,state:c.S,stale:false,loading:false});
  c.orders=orders;c.routes=routes;return c;
}
function project(){return {id:7,allocation:{requested:null,assigned:7.5,max:20,mode:'auto'}};}
function operations(extra={}){return {as_of_day:2,inherited_factory_equivalents:8,power_capacity_daily:40,power_required_daily:60,
  workers_available:10000,jobs_required:12000,jobs_filled:10000,facilities:[{
    district:'US-CA',kind:'office_district',name:'Office District',inherited:false,installed_capacity:2,operating_capacity:1,
    utilization:.5,output_daily:.003,output_unit:'$bn / day',jobs_required:6000,jobs_filled:3000,power_required_daily:20,
    power_used_daily:10,worker_fraction:1,power_fraction:.5,input_fraction:.4,funding_fraction:.7,
    reason:'Power shortage limits this facility.',annual_gdp_bn:1.095,annual_tax_bn:.219,cash_spent_daily_bn:.0001,
  }],note:'Jobs are modeled estimates.',...extra};}

test('capacity ledger renders the served pool, assignments and idle capacity without changing state',()=>{
  const c=fixture();c.PROD.data.industry_rebuild={capacity:{total_capacity:30,assigned_capacity:21.5,idle_capacity:8.5,
    inherited_capacity:10,civilian_capacity:20,starter_capacity:0,max_per_project:20}};
  const before=plain(c.PROD.data),html=c.constructionCapacityHtml();
  for(const text of ['Available capacity','30','21.5','8.5','Maximum per project','20','guaranteed starting capacity'])assert(html.includes(text),text);
  assert.deepEqual(plain(c.PROD.data),before);assert.equal(c.orders.length,0);
  c.PROD.data.industry_rebuild.capacity.total_capacity=null;assert.match(c.constructionCapacityHtml(),/Available capacity<\/dt><dd>—/);
});
test('chooser funding disclosure keeps the current limits visible and retains the real funding form',()=>{
  const c=fixture();c.PROD.data.industry_rebuild={capacity:{total_capacity:30,assigned_capacity:20}};
  c.PROD.data.construction_budget={daily_budget_bn:.02,available_bn:.01,authority_bn:1,can_set:true};
  const html=c.constructionFundingDisclosureHtml();
  assert.match(html,/^<details class="construction-funding-disclosure"><summary>/);
  assert.match(html,/30 capacity · 20 assigned · \$0.02bn daily funding limit/);
  assert.match(html,/id="constructionBudgetForm"/);assert.match(html,/id="constructionDailyBudget"/);
  assert.equal(c.orders.length,0);
});
test('allocation distinguishes requested and assigned capacity and preserves a zero draft',()=>{
  const c=fixture(),p=project();p.allocation={...p.allocation,requested:20,mode:'manual'};
  c.PROD.allocationDrafts={'7':'0'};let html=c.constructionAllocationHtml(p);
  assert.match(html,/7.5 assigned of 20 requested/);assert.match(html,/value="0"/);assert.match(html,/max="20"/);
  assert.match(html,/data-construction-allocation-auto="7" aria-pressed="false"/);
  c.PROD.stale=true;assert.match(c.constructionAllocationHtml(p),/Assign capacity<\/button>/);assert.match(c.constructionAllocationHtml(p),/type="submit" disabled/);
});
test('review distinguishes the fastest supplied completion from the current allocated estimate',()=>{
  const c=fixture();c.PROD.previewRequest={district:'US-CA',project_kind:'generation'};
  c.PROD.preview={...c.PROD.previewRequest,name:'Power Plant',minimum_days:240,eta_days:600,cost_bn:.16,can_start:true};
  c.PROD.previewState=c.S;c.PROD.data.industry_rebuild={enabled:true};
  const before=plain(c.PROD.preview),html=c.constructionPreviewHtml();
  assert.match(html,/<dt>Earliest completion<\/dt><dd>240 days<\/dd><small>At maximum capacity, with full funding and supplies<\/small>/);
  assert.match(html,/<dt>At current capacity &amp; funding<\/dt><dd>About 600 days<\/dd>/);
  assert.doesNotMatch(html,/Base work duration|At standard capacity/);
  c.PROD.data.industry_rebuild=null;
  const legacy=c.constructionPreviewHtml();assert.match(legacy,/Project lead time/);assert.match(legacy,/<dt>At current funding<\/dt>/);
  assert.deepEqual(plain(c.PROD.preview),before);assert.equal(c.orders.length,0);
});
test('manual allocation and Auto send exactly one server command, including a real zero pause',async()=>{
  const c=fixture();c.PROD.data.queue=[project()];
  for(const value of ['12.5','0',null])assert.equal(await c.constructionSetAllocation('7',value),true);
  assert.deepEqual(c.orders,[{kind:'set_project_allocation',project:7,capacity:12.5},{kind:'set_project_allocation',project:7,capacity:0},{kind:'set_project_allocation',project:7,capacity:null}]);
});
test('mine capacity uses the resource command and keeps district and commodity identity',async()=>{
  const c=fixture(),mine={...project(),id:'mine:US-CA:iron',kind:'resource_mine',commodity:'iron',province:{id:'US-CA'}};
  c.PROD.data.mine_queue=[mine];
  assert.equal(await c.constructionSetAllocation(mine.id,'4'),true);
  assert.equal(await c.constructionSetAllocation(mine.id,null),true);
  assert.deepEqual(c.orders,[{kind:'set_mine_allocation',district:'US-CA',commodity:'iron',capacity:4},{kind:'set_mine_allocation',district:'US-CA',commodity:'iron',capacity:null}]);
  const html=c.constructionAllocationHtml(mine);
  assert.match(html,/data-construction-allocation="mine:US-CA:iron"/);
  assert.match(html,/id="constructionAllocation-[0-9a-f_]+"/,'Focus target remains a valid CSS selector for string mine IDs');
  delete mine.commodity;assert.equal(await c.constructionSetAllocation(mine.id,3),false);assert.equal(c.orders.length,2);
});
test('invalid, unknown and stale allocation requests cannot place orders',async()=>{
  for(const value of ['', ' ', -1, 20.1, 'NaN', Infinity, '1e309', undefined]){
    const c=fixture();c.PROD.data.queue=[project()];assert.equal(await c.constructionSetAllocation(7,value),false);assert.equal(c.orders.length,0);
  }
  for(const field of ['stale','loading','busy']){const c=fixture();c.PROD.data.queue=[project()];c.PROD[field]=true;assert.equal(await c.constructionSetAllocation(7,10),false);assert.equal(c.orders.length,0);}
  const c=fixture();assert.equal(await c.constructionSetAllocation(8,null),false);assert.equal(c.orders.length,0);
  c.PROD.data.queue=[project()];c.pendingAdvance={};assert.equal(await c.constructionSetAllocation(7,10),false);
});
test('catalog groups projects by their decision and scopes upgrades to a selected province',()=>{
  const c=fixture();c.PROD.data.provinces=[{id:'US-CA',name:'California'}];
  c.PROD.data.catalog=['civilian_industry','office_district','arms_plant','generation','automation','industry_preset'].map(kind=>({kind,name:kind,eligible_provinces:['US-CA'],work_cost_bn:.3}));
  let html=c.productionCatalogHtml();
  for(const text of ['Build faster','Earn income','Equip forces','Support economy','Facility upgrades','Standard industrial starter preset','Needs / tradeoff'])assert(html.includes(text),text);
  assert.doesNotMatch(html,/data-prod-kind="automation"/);assert.match(html,/id="constructionUpgradeProvince"/);
  c.PROD.provinceFilter='US-CA';html=c.productionCatalogHtml();assert.match(html,/Facility upgrades · California/);assert.match(html,/data-prod-kind="automation"/);
  c.PROD.data.catalog[0].purpose='<script>bad</script>';assert.doesNotMatch(c.productionCatalogHtml(),/<script>/);
});
test('a reviewed standard preset uses the atomic preset command and never starts a phantom building',async()=>{
  const c=fixture();c.PROD.previewRequest={project_kind:'industry_preset',district:'US-CA'};
  c.PROD.preview={...c.PROD.previewRequest,can_start:true,name:'Standard industrial starter preset'};c.PROD.previewState=c.S;
  assert.equal(await c.constructionConfirmPreview(),true);assert.deepEqual(c.orders,[{kind:'start_industry_preset',district:'US-CA'}]);
  assert.equal(c.PROD.view,'queue');assert.equal(c.PROD.preview,null);
});
test('shortage routes choose actual catalog entries, imports and funding without placing orders',()=>{
  const c=fixture();c.PROD.data.catalog=[{kind:'generation'}];
  const html=c.constructionRemedyHtml('Power shortage and missing materials; funding limited.');
  for(const target of ['generation','imports','budget'])assert(html.includes(`data-construction-remedy="${target}"`));
  assert.equal(c.constructionOpenRemedy('generation'),true);assert.equal(c.PROD.pickKind,'generation');assert.equal(c.PROD.view,'provinces');
  assert.equal(c.constructionOpenRemedy('unsupported'),false);c.constructionOpenRemedy('imports');c.constructionOpenRemedy('budget');
  assert.deepEqual(c.routes,['close','imports','budget']);assert.equal(c.orders.length,0);
});
test('equipment component-supply navigation opens Industry while pending orders keep it disabled',async()=>{
  const c=fixture();
  assert.equal(await c.equipmentNavigate({action:'industry',district:'US-CA'}),true);
  assert.deepEqual(c.routes,['close-equipment',{industry:{district:'US-CA'}}]);
  c.equipmentExternalPending=()=>true;assert.equal(await c.equipmentNavigate({action:'industry'}),false);
  assert.equal(c.routes.length,2);assert.equal(c.orders.length,0);
});
test('manufacturing shows the served component plan beside shared stock without inventing a reservation',()=>{
  const c=fixture(),line={advanced_components:{required:3.25,stock:2,unit:'packs',cadence:'daily',note:'Inventory is shared; none is reserved.'}};
  const before=plain(line),html=c.manufacturingComponentsHtml(line);
  assert.match(html,/3.25/);assert.match(html,/2<\/strong> in national stock/);assert.match(html,/planned this day/);
  assert.match(html,/Inventory is shared; none is reserved/);assert.match(html,/data-manu-component-supply/);
  assert.deepEqual(plain(line),before);assert.equal(c.manufacturingComponentsHtml({}),'');
});
test('unified industry separates installed capacity, latest output, GDP and tax and shows operating reasons',()=>{
  const c=fixture(),data={name:'United States',sites:[],industry_rebuild:{operations:operations()}};c.desk.data=data;
  const before=plain(data),html=c.industryUnifiedHtml(data);
  for(const text of ['Installed capacity','Operable capacity now','Latest output','0.003','Annual GDP contribution','$1.095bn','Annual tax revenue','$0.219bn','Power shortage limits this facility.','Jobs are modeled estimates.'])assert(html.includes(text),text);
  assert.match(html,/Review a Power Plant/);assert.match(html,/Find imports/);assert.match(html,/Fund operations/);assert.match(html,/Review facility upgrades/);
  assert.deepEqual(plain(data),before);assert.equal(c.orders.length,0);
});
test('no settled operation is rendered as awaiting output; inherited output stays explicitly already counted',()=>{
  const c=fixture(),ops=operations({as_of_day:null});ops.facilities[0].output_daily=999;
  let html=c.industryUnifiedHtml({industry_rebuild:{operations:ops}});assert.match(html,/Awaiting first operation/);assert.doesNotMatch(html,/999/);
  ops.facilities[0].inherited=true;ops.facilities[0].recorded_day=0;html=c.industryUnifiedHtml({industry_rebuild:{operations:ops}});
  assert.match(html,/Inherited national industry/);assert.match(html,/already included in national GDP/);assert.match(html,/999/);
});
test('unified operations escape names and reasons and support province search without mutating readings',()=>{
  const c=fixture(),ops=operations();ops.facilities[0].name='<img src=x>';ops.facilities[0].reason='<script>bad</script>';
  const data={industry_rebuild:{operations:ops}},html=c.industryUnifiedHtml(data);assert.doesNotMatch(html,/<img|<script/);assert.match(html,/&lt;img/);
  c.desk.query='US-CA';assert.match(c.industryUnifiedHtml(data),/Installed capacity/);
  c.desk.query='ZZ-NO';assert.match(c.industryUnifiedHtml(data),/No operating facilities match/);
});
