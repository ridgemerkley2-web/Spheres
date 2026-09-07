// Actual UI rendering and command adapters. Funding and project work remain Rust-owned.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const root=path.resolve(__dirname,'../..');
const page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
const economy=fs.readFileSync(path.join(root,'spheres-web/ui/province-economy-ui.js'),'utf8');
const plain=x=>JSON.parse(JSON.stringify(x));
function fn(name,source=page){
  const match=new RegExp(`^(?:async )?function ${name}\\(`,'m').exec(source);
  assert(match,`Shipped function ${name} exists`);
  const lineEnd=source.indexOf('\n',match.index),line=source.slice(match.index,lineEnd);
  if(/}\s*$/.test(line))return line;
  return source.slice(match.index,source.indexOf('\n}',match.index)+2);
}
test('map construction sprites resolve coarse geometry separately from close cards',()=>{
  const SiteMesh=require(path.join(root,'spheres-web/ui/site-mesh.js'));
  const providers=new Map(), draws=[];
  const Arsenal3D={available:true,register:(key,build)=>providers.set(key,build),
    sprite(id,size){const split=id.indexOf(':');const mesh=providers.get(id.slice(0,split))(id.slice(split+1));draws.push({id,size,mesh});return mesh;},
    canvasHtml(id){const split=id.indexOf(':');const mesh=providers.get(id.slice(0,split))(id.slice(split+1));draws.push({id,mesh});return id;}};
  const c=vm.createContext({window:{SiteMesh,Arsenal3D},SiteMesh,Arsenal3D,installSurfaceTreatment(){},
    clamp:(v,a,b)=>Math.max(a,Math.min(b,v))});
  vm.runInContext(['registerMeshProviders','siteArtReady','productionKind','productionStatus','productionProgress','productionSiteSprite','productionSite3d'].map(n=>fn(n)).join('\n'),c);
  const project={kind:'arms_plant',progress:0.55,status:'building',level:1};
  c.productionSiteSprite(project,24);c.productionSite3d(project);
  assert.equal(draws[0].mesh.lod,1,'Map pins must actually build the coarse site');
  assert.equal(draws[1].mesh.lod,0,'Cards retain the detailed site');
  assert.notEqual(draws[0].id,draws[1].id,'Renderer caches cannot alias the two detail levels');
  assert(draws[0].mesh.triangleCount<draws[1].mesh.triangleCount/4,'Map geometry must be substantially cheaper');
  c.productionSiteSprite({...project,progress:0.551},30);
  assert.equal(draws[2].id,draws[0].id,'Tiny work updates reuse the stage cache');
  assert.equal(draws[2].size,draws[0].size,'Nearby pin sizes reuse the sprite bucket');
});

const names=['logisticsEscAttr','constructionMoney','productionQueue','productionCatalog','productionProvinces',
  'productionCompleted','productionProject','productionKind','productionStatus','productionTone','productionProvince',
  'productionProgress','productionPriorityChoices','productionCanCancel','productionSummary','productionFundingLabel',
  'productionEligible','productionCardHtml','productionSiteStripHtml','productionSite3d','siteArtReady','productionCatalogHtml','productionProvinceHtml','constructionBudgetHtml',
  'productionCapabilityPairs','productionModuleLabel','constructionSiteContext','constructionProvinceRefusal',
  'constructionProvinceMatches','constructionProvinceChoices','constructionRevealProject','constructionPreviewNoticeHtml',
  'constructionWorkshopProvinces','constructionWorkshopDistrict','constructionChooseWorkshop',
  'constructionSetBudget','constructionWorkshopHtml','constructionQuoteWorkshop','constructionReviewWorkshop',
  'constructionInvalidatePreview','constructionPreviewCurrent','constructionEffectValue','constructionEffectRows',
  'constructionPreviewHtml','constructionPreviewProject','constructionConfirmPreview','constructionReviewMine',
  'constructionSuggestionItems','constructionSuggestionsCurrent','constructionSuggestionsHtml','constructionReviewSuggestion','productionFetch',
  'productionCommand','productionStart','productionSetPriority','productionCancel','openConstruction'];
function fixture(){
  const requests=[],notices=[],routes=[];
  const c=vm.createContext({requests,notices,routes,console,Number,Math,Promise,
    PROD:{open:true,mode:'build',view:'queue',seq:0,loading:false,data:null,stale:false,busy:false,budgetDraft:null,moduleSeq:0,moduleLoading:false,session:'one',previewSeq:0,previewLoading:false,preview:null,previewRequest:null},
    MANU:{},GLOBE:null,PROD_ICON:{civilian_industry:'⚙'},S:{session_id:'one',player:'USA'},advancing:false,pendingAdvance:null,
    clamp:(v,a,b)=>Math.max(a,Math.min(b,v)),fmtQ:String,
    renderProductionPanel(){},openProduction(){routes.push('construction-shell');},
    $:()=>({querySelector:()=>null}),
    campaignConfirm:async message=>{notices.push(message);return true;},
    banner:message=>notices.push(message),
    api:async(url,body)=>{requests.push({url,body:plain(body)});return url==='/api/construction-preview'?impact({district:body.district,project_kind:body.project_kind,capacity_micros:body.capacity_micros,commodity:body.commodity}):{session_id:'one',errors:[]};},
    adopt:async()=>{},
  });
  vm.runInContext(`${page.match(/^const escText\s*=.*;\s*$/m)[0]}\n${fn('economyMoney',economy)}\n${names.map(name=>fn(name)).join('\n')}`,c);
  c.PROD.data={queue:[],mine_queue:[],catalog:[],provinces:[],completed:[],construction_budget:{
    daily_budget_bn:.0125,available_bn:.006,spent_today_bn:.004,planned_daily_bn:.005,spent_ytd_bn:.12,
    authority_bn:.9,explicit:true,can_set:true,reason:''
  }};
  c.PROD.dataState=c.S;
  return c;
}
const project=()=>({id:7,kind:'civilian_industry',name:'Factory',province:{id:'US-CA',name:'California'},priority:'normal',
  status:'building',progress:.37,eta_days:63,finance:{cost_bn:.125,spent_bn:.04,remaining_bn:.085,daily_request_bn:.00125},
  requirements:[{name:'Iron',required:400,shortfall:200}],actions:{cancel:true,set_priority:['high','low']},effect:'Produces through actual operation.'});
function impact(extra={}) { return {district:'US-CA',district_name:'California',project_kind:'civilian_industry',name:'Factory',nation_name:'United States',
  cost_bn:.2,minimum_days:180,eta_days:245,can_start:true,
  province_effects:[{label:'Industrial sites',before:2,after:3,unit:'sites',detail:'A completed local facility.'}],
  national_effects:[{label:'Potential operating output',before:10,after:12,unit:'packs/day',detail:'Requires an operating supply chain.'}],
  operating_requirements:[{label:'Running budget',value:.00003,unit:'$bn/day',detail:'Applies after completion.'}],
  notes:['Actual output depends on operation; it is not a guaranteed GDP or tax increase.'],...extra}; }
function reviewed(c,request={district:'US-CA',project_kind:'civilian_industry'},quote=impact(request)) {
  c.PROD.previewRequest=request;c.PROD.preview=quote;c.PROD.previewState=c.S;return quote;
}
function suggested(c,extra={}) {
  const item={id:'grid-ca',project_kind:'power_grid',district:'US-CA',name:'Power grid',priority:'Bottleneck',
    district_name:'California',reason:'Local grid limits existing factory output.',evidence:['Two operating plants share this grid.'],
    caution:'Generation and inputs are still required.',cost_bn:.025,minimum_days:180,eta_days:250,...extra};
  c.PROD.data.suggestions={as_of_day:1,items:[item],note:'Suggestions compare current bottlenecks and existing projects.'};
  return item;
}

test('suggestions show at most three reason-first cards with served costs, lead times and evidence',()=>{
  const c=fixture(),item=suggested(c);
  c.PROD.data.suggestions.items=Array.from({length:4},(_,i)=>({...item,id:`grid-${i}`,name:`Grid option ${i}`}));
  const before=plain(c.PROD.data),html=c.constructionSuggestionsHtml();
  for(const text of ['Suggested construction','Across your country','Bottleneck','California','Local grid limits existing factory output.',
    'Why this is suggested','Two operating plants share this grid.','Generation and inputs are still required.','$25m total','250 days at current funding','At least 180 days','Review suggestion'])assert(html.includes(text),text);
  assert(html.indexOf(item.reason)<html.indexOf('<details>'),'The reason is visible before optional supporting evidence');
  assert.equal((html.match(/data-construction-suggestion=/g)||[]).length,3);assert.doesNotMatch(html,/Grid option 3/);
  assert.doesNotMatch(html,/data-construction-suggestion="[^"]+" disabled/);
  assert.deepEqual(plain(c.PROD.data),before);
});
test('suggestions respect province scope and useful empty states without inventing a recommendation',()=>{
  const c=fixture(),item=suggested(c);c.PROD.data.provinces=[{id:'US-NV',name:'Nevada'}];
  c.PROD.provinceFilter='US-NV';let html=c.constructionSuggestionsHtml();
  assert.match(html,/For Nevada/);assert.match(html,/No current suggestion for this province/);assert.doesNotMatch(html,/data-construction-suggestion=/);
  c.PROD.data.suggestions.items.push({...item,id:'grid-nv',district:'US-NV',district_name:'Nevada'});
  html=c.constructionSuggestionsHtml();assert.match(html,/data-construction-suggestion="grid-nv"/);assert.doesNotMatch(html,/California|grid-ca/);
  c.PROD.provinceFilter=null;c.PROD.data.suggestions.items=[];html=c.constructionSuggestionsHtml();
  assert.match(html,/Suggestions compare current bottlenecks/);assert.doesNotMatch(html,/Review suggestion/);
  delete c.PROD.data.suggestions;assert.match(c.constructionSuggestionsHtml(),/Use Add project to compare/);
  c.PROD.loading=true;assert.match(c.constructionSuggestionsHtml(),/Updating suggestions/);
});
test('suggestion fields are escaped and unavailable timing and costs remain unavailable',()=>{
  const c=fixture();suggested(c,{id:'" onclick="bad()<img>',name:'<img>',district_name:'<svg>',priority:'<iframe>',
    reason:'<script>reason</script>',evidence:['<img src=x>'],caution:'<svg>care',cost_bn:null,eta_days:null,minimum_days:null});
  c.PROD.data.suggestions.note='<script>note</script>';
  const html=c.constructionSuggestionsHtml();assert.doesNotMatch(html,/<img|<script|<iframe|<svg/);
  assert.match(html,/data-construction-suggestion="&quot; onclick=&quot;bad\(\)&lt;img>"/);
  assert.match(html,/— total/);assert.match(html,/Completion awaits available funding/);assert.doesNotMatch(html,/At least 0|\$0/);
});
test('suggestion selection is disabled while stale, loading, busy or tied to another world or scope',async()=>{
  for(const change of [c=>c.PROD.stale=true,c=>c.PROD.loading=true,c=>c.PROD.busy=true,c=>c.PROD.previewLoading=true,
    c=>c.advancing=true,c=>c.pendingAdvance={},c=>c.S={session_id:'one',player:'USA'},c=>c.PROD.open=false,
    c=>c.PROD.mode='manufacture',c=>c.PROD.view='catalog']){
    const c=fixture();suggested(c);change(c);
    assert.match(c.constructionSuggestionsHtml(),/data-construction-suggestion="grid-ca" disabled/);
    assert.equal(await c.constructionReviewSuggestion('grid-ca'),false);assert.equal(c.requests.length,0);
  }
  const c=fixture();suggested(c);c.PROD.provinceFilter='US-NV';
  assert.equal(await c.constructionReviewSuggestion('grid-ca'),false);assert.equal(c.requests.length,0);
});
test('reviewing a suggestion sends its exact project, province and frozen size only to the read-only preview',async()=>{
  for(const extra of [{},{id:'small-ca',project_kind:'starter_industry',capacity_micros:5001}]){
    const c=fixture(),item=suggested(c,extra);
    assert.equal(await c.constructionReviewSuggestion(item.id),true);
    const body={district:'US-CA',project_kind:item.project_kind,session_id:'one'};
    if(item.capacity_micros!=null)body.capacity_micros=item.capacity_micros;
    assert.deepEqual(c.requests,[{url:'/api/construction-preview',body}]);
    assert.equal(c.PROD.view,'preview');assert.equal(c.constructionPreviewCurrent(),true);
    assert.equal(c.notices.length,0,'Reviewing never places an order or applies funding');
  }
});
test('a suggestion button captures the current ledger and cannot select a replacement payload',async()=>{
  const c=fixture();suggested(c);const button={dataset:{constructionSuggestion:'grid-ca'}};
  c.$=()=>({querySelector:()=>null,querySelectorAll:selector=>selector==='[data-construction-suggestion]'?[button]:[]});
  vm.runInContext(fn('wireProductionPanel'),c);c.wireProductionPanel();
  const previous=c.PROD.data;c.PROD.data=plain(previous);c.PROD.dataState=c.S;
  assert.equal(await button.onclick(),false);assert.equal(c.requests.length,0);
  c.wireProductionPanel();assert.equal(await button.onclick(),true);
  assert.equal(c.requests[0].url,'/api/construction-preview');
});
test('the production fetch binds suggestions to the state used for its request',async()=>{
  const c=fixture();suggested(c);const data=plain(c.PROD.data);c.PROD.stale=true;
  c.api=async()=>data;await c.productionFetch();
  assert.equal(c.PROD.dataState,c.S);assert.equal(c.constructionSuggestionsCurrent(),true);
  c.PROD.stale=true;let resolve;c.api=()=>new Promise(r=>resolve=r);
  const read=c.productionFetch();c.S={session_id:'one',player:'USA'};resolve(plain(data));await read;
  assert.equal(c.constructionSuggestionsCurrent(),false,'A response for the previous state cannot authorize a suggestion');
});
test('suggestions appear only on the queue between funding and the project list',()=>{
  const c=fixture();suggested(c);const body={scrollTop:0,innerHTML:'',querySelector:()=>null};
  c.$=()=>body;c.document={activeElement:null};c.operationsDetailsState=()=>({});c.operationsRestoreDetails=()=>{};
  c.wireProductionPanel=()=>{};c.productionBuiltHtml=()=>'';
  vm.runInContext(fn('renderProductionPanel'),c);c.renderProductionPanel();
  assert(body.innerHTML.indexOf('Daily construction budget')<body.innerHTML.indexOf('Suggested construction'));
  assert(body.innerHTML.indexOf('Suggested construction')<body.innerHTML.indexOf('Your construction queue'));
  c.PROD.view='catalog';c.renderProductionPanel();assert.doesNotMatch(body.innerHTML,/Suggested construction/);
});

test('all shipped inline scripts and companion UI scripts parse',()=>{
  for(const match of page.matchAll(/<script([^>]*)>([\s\S]*?)<\/script>/g)){
    if(!match[1].includes('src=')&&!match[1].includes('application/json'))new vm.Script(match[2]);
  }
  for(const name of ['programs-ui.js','competition-ui.js','decision-tools.js','province-economy-ui.js','industry-ui.js','cash-flow-ui.js'])new vm.Script(fs.readFileSync(path.join(root,'spheres-web/ui',name),'utf8'));
});
test('funding form formats the served daily target in millions and preserves a zero draft',()=>{
  const c=fixture(),before=plain(c.PROD.data);
  let html=c.constructionBudgetHtml();
  assert.match(html,/value="12.5"/);assert.match(html,/\$ millions \/ day/);
  for(const value of ['$6m','$5m','$4m','$120m','$900m'])assert(html.includes(value),value);
  assert.match(html,/Last recorded spending/);assert.doesNotMatch(html,/>Spent today</);
  assert.match(html,/data-construction-cash-flow>Review national cash flow/);
  c.PROD.budgetDraft='0';html=c.constructionBudgetHtml();assert.match(html,/value="0"/);
  assert.deepEqual(plain(c.PROD.data),before,'Rendering cannot mutate budget readings');
});
test('unknown funding is not rendered as zero and blocked funding keeps the server reason',()=>{
  const c=fixture();c.PROD.data.construction_budget={can_set:false,reason:'Annual authority < unavailable'};
  const html=c.constructionBudgetHtml();assert.match(html,/value="" disabled/);
  assert.match(html,/Annual authority &lt; unavailable/);assert.match(html,/<dd>—<\/dd>/);
});
test('setting a daily budget converts display units only and zero sends an explicit pause',async()=>{
  const c=fixture();
  assert.equal(await c.constructionSetBudget('2.75'),true);
  assert.deepEqual(c.requests[0],{url:'/api/command',body:{commands:[{kind:'construction_budget',daily_budget_bn:.00275}]}});
  assert.equal(await c.constructionSetBudget('0'),true);
  assert.equal(c.requests[1].body.commands[0].daily_budget_bn,0);
  assert.equal(c.PROD.budgetDraft,null);
});
test('empty, negative and nonfinite budgets cannot place a financial order',async()=>{
  for(const amount of ['', ' ', '-1','Infinity','NaN','1e999']){
    const c=fixture();assert.equal(await c.constructionSetBudget(amount),false);assert.equal(c.requests.length,0,amount);
    assert.match(c.PROD.actionError,/zero or more/);
  }
});
test('failed and uncertain commands preserve a budget draft and do not report success',async()=>{
  for(const response of [{errors:['Insufficient appropriation']},{command_pending:true}]){
    const c=fixture();c.PROD.budgetDraft='5';c.api=async()=>response;
    assert.equal(await c.constructionSetBudget('5'),false);assert.equal(c.PROD.budgetDraft,'5');
    assert(c.PROD.actionError);assert.equal(c.notices.length,0);assert.equal(c.PROD.busy,false);
  }
});
test('a pending turn or in-flight project command blocks duplicate financial orders',async()=>{
  const c=fixture();c.advancing=true;
  assert.equal(await c.constructionSetBudget('1'),false);assert.equal(c.requests.length,0);
  c.advancing=false;let resolve;c.api=()=>new Promise(r=>resolve=r);
  const pending=c.constructionSetBudget('1');assert.equal(c.PROD.busy,true);
  assert.equal(await c.constructionSetBudget('2'),false);
  resolve({errors:[]});assert.equal(await pending,true);assert.equal(c.PROD.busy,false);
});
test('project rows display served money, progress and ETA without construction inputs or slots',()=>{
  const c=fixture(),p=project(),before=plain(p),html=c.productionCardHtml(p);
  for(const text of ['$125m','$40m','$85m','$1.25m','37%','63 days','Total cost','Spent','Remaining','Planned / day','After completion'])assert(html.includes(text),text);
  assert.match(html,/role="progressbar"/);assert.match(html,/aria-valuenow="37"/);
  assert.match(html,/data-prod-priority="high"/);assert.match(html,/data-prod-cancel="7"/);
  assert.doesNotMatch(html,/Iron|Materials|work-days|slots|PC|political capital/);
  assert.deepEqual(plain(p),before);
});
test('project names, reasons and quoted attributes cannot inject HTML',()=>{
  const c=fixture(),p=project();p.name='Factory" onclick="bad()<img>';p.province.name='<script>bad</script>';p.reason='<iframe>';
  const html=c.productionCardHtml(p);assert.doesNotMatch(html,/<img>|<script>|<iframe>/);
  assert.match(html,/aria-label="Factory&quot; onclick=&quot;bad\(\)&lt;img> progress"/);
});
test('the first paid work remains visible below one percent without inventing progress on unstarted projects',()=>{
  const c=fixture(),p=project();
  p.progress=.00125;
  assert.match(c.productionCardHtml(p),/<b>0\.1%<\/b> complete/,'250k of a 200m project has visible first-day progress');
  p.progress=.00001;
  assert.match(c.productionCardHtml(p),/<b>&lt;0\.1%<\/b> complete/,'A positive fraction below display precision is not called zero');
  p.progress=0;
  assert.match(c.productionCardHtml(p),/<b>0%<\/b> complete/);
  p.progress=.4567;
  assert.match(c.productionCardHtml(p),/<b>45\.7%<\/b> complete/);
});
test('mines share the construction queue with map access and no unsupported management actions',()=>{
  const c=fixture(),p=project();c.PROD.data.queue=[p];
  const mine={...project(),id:'mine:US-NV:iron',name:'Iron mine',actions:undefined,priority:undefined,legacy_prepaid:true};
  c.PROD.data.mine_queue=[mine];assert.equal(c.productionQueue().length,2);
  assert.equal(c.productionProject(mine.id),mine);
  const html=c.productionCardHtml(mine);assert.match(html,/data-prod-map="mine:US-NV:iron"/);
  assert.doesNotMatch(html,/data-prod-priority|data-prod-cancel/);assert.match(html,/without a second construction charge/);
});
test('project start requires a matching current preview and retains served eligibility',async()=>{
  const c=fixture();c.PROD.data.catalog=[{kind:'civilian_industry',name:'Factory',eligible_provinces:['US-CA']}];
  c.PROD.data.provinces=[{id:'US-CA'},{id:'US-NV'}];
  await c.productionStart('civilian_industry','US-NV');assert.equal(c.requests.length,0);
  await c.productionStart('civilian_industry','US-CA');assert.equal(c.requests.length,0,'An eligible province alone cannot bypass review');
  reviewed(c);
  await c.productionStart('civilian_industry','US-CA');
  assert.deepEqual(c.requests[0].body.commands,[{kind:'start_project',project_kind:'civilian_industry',district:'US-CA'}]);
  assert.equal(c.PROD.view,'queue');
});
test('catalog removes construction recipes and political cost while preserving project effects',()=>{
  const c=fixture();c.PROD.data.catalog=[{kind:'civilian_industry',name:'Factory',total_days:180,pc_cost:25,
    funding:{work_cost_bn:.15},description:'Build industry',effect:'Produces usable output',requirements:[{name:'Steel'}],actions:{start:true},eligible_provinces:['US-CA']}];
  c.PROD.data.provinces=[{id:'US-CA',name:'California'}];
  const html=c.productionCatalogHtml();assert.match(html,/\$150m total project cost/);assert.match(html,/180 days/);
  assert.match(html,/Produces usable output/);assert.doesNotMatch(html,/Steel|political capital|PC|slots/);
  c.PROD.pickKind='civilian_industry';assert.doesNotMatch(c.productionProvinceHtml(),/Materials|Steel/);
});

test('province search matches names and codes, preserves eligible scope and reports useful no-results text',()=>{
  const c=fixture();c.PROD.pickKind='power_grid';
  c.PROD.data.catalog=[{kind:'power_grid',name:'Power Grid',eligible_provinces:['BR-SP','US-NY']}];
  c.PROD.data.provinces=[{id:'BR-SP',name:'São Paulo'},{id:'US-NY',name:'New York'},{id:'US-CA',name:'California'}];
  const before=plain(c.PROD.data);
  let html=c.productionProvinceHtml();assert.match(html,/Find a province/);assert.match(html,/2 of 2 eligible provinces/);
  assert.match(html,/data-construction-search-clear disabled/);assert.doesNotMatch(html,/data-prod-province="US-CA"/);
  c.PROD.provinceQuery='SAO';html=c.productionProvinceHtml();assert.match(html,/1 of 2 eligible provinces/);
  assert.match(html,/data-prod-province="BR-SP"/);assert.doesNotMatch(html,/data-prod-province="US-NY"/);
  c.PROD.provinceQuery='new us-ny';html=c.productionProvinceHtml();assert.match(html,/data-prod-province="US-NY"/);
  c.PROD.provinceQuery='absent';html=c.productionProvinceHtml();assert.match(html,/0 of 2 eligible provinces/);
  assert.match(html,/No provinces match your search/);assert.doesNotMatch(html,/data-prod-province=/);
  c.PROD.provinceQuery='';c.PROD.provinceFilter='BR-SP';html=c.productionProvinceHtml();
  assert.match(html,/1 of 1 eligible province/);assert.match(html,/Selected province only/);assert.doesNotMatch(html,/US-NY/);
  assert.deepEqual(plain(c.PROD.data),before);assert.equal(c.requests.length,0);
});

test('province context uses real upgrades, fractional workshops and active projects without predicting output',()=>{
  const c=fixture();c.PROD.pickKind='power_grid';
  const province={id:'US-CA',capabilities:{civilian_industry:1,power_grid:2,generation:1,warehouse:1},module_capacity:.012345,active_projects:[7,9]};
  const before=plain(province),text=c.constructionSiteContext(province);
  for(const value of ['Grid 2','Estate 1','Generation 1','+1 other upgrade types','Starter workshop · 1.2345% standard capacity','2 active projects'])assert(text.includes(value),value);
  assert.doesNotMatch(text,/packs\/day|GDP|construction capacity/);assert.deepEqual(plain(province),before);
  assert.equal(c.constructionSiteContext({capabilities:{power_grid:0}}),'No built upgrades recorded');
  assert.equal(c.constructionSiteContext({module_capacity:.000001,active_projects:[1]}),'Starter workshop · 0.0001% standard capacity · 1 active project');
});

test('unavailable provinces and scoped catalog cards show exact served prerequisites without a start action',()=>{
  const c=fixture();c.PROD.pickKind='machinery_works';
  const project={kind:'machinery_works',name:'Machinery Works',eligible_provinces:['US-CA'],reason:'National fallback'};
  c.PROD.data.catalog=[project];c.PROD.data.provinces=[{id:'US-CA',name:'California'},
    {id:'US-NV',name:'Nevada <script>',start_refusals:{machinery_works:'Build an Industrial Estate <first>.'}}];
  assert.doesNotMatch(c.productionProvinceHtml(),/Nevada/);
  c.PROD.showUnavailable=true;let html=c.productionProvinceHtml();
  assert.match(html,/2 of 2 owned provinces/);assert.match(html,/Build an Industrial Estate &lt;first>/);
  assert.match(html,/Nevada &lt;script>/);assert.doesNotMatch(html,/<script>|data-prod-province="US-NV"/);
  assert.match(html,/data-prod-province="US-CA"/);
  c.PROD.provinceFilter='US-NV';html=c.productionCatalogHtml();
  assert.match(html,/Build an Industrial Estate &lt;first>/);assert.doesNotMatch(html,/data-prod-kind="machinery_works"/);
  c.PROD.provinceFilter=null;project.actions={start:false};project.eligible_provinces=[];
  assert.match(c.productionCatalogHtml(),/National fallback/);
  c.PROD.provinceQuery='" autofocus onfocus="bad()<img>';assert.doesNotMatch(c.productionProvinceHtml(),/<img>/);
});

test('search clear and unavailable controls update only display state',()=>{
  const c=fixture(),search={value:'York',focus:()=>{}},toggle={checked:true},clear={addEventListener:(type,handler)=>clear.click=handler};
  const controls={'#constructionProvinceSearch':search,'#constructionProvinceUnavailable':toggle,'[data-construction-search-clear]':clear};
  c.$=()=>({querySelector:selector=>controls[selector]||null,querySelectorAll:()=>[]});
  vm.runInContext(fn('wireProductionPanel'),c);c.wireProductionPanel();
  search.oninput();assert.equal(c.PROD.provinceQuery,'York');toggle.onchange();assert.equal(c.PROD.showUnavailable,true);
  clear.click();assert.equal(c.PROD.provinceQuery,'');assert.equal(c.requests.length,0);
});

test('workshop placement never narrows the national queue and explicit Build here scope stays fixed',async()=>{
  const c=fixture();c.PROD.provinceFilter=null;c.PROD.data.provinces=[{id:'US-CA'},{id:'US-NV'}];
  reviewed(c);assert.equal(c.constructionChooseWorkshop('US-NV'),true);
  assert.equal(c.PROD.workshopDistrict,'US-NV');assert.equal(c.PROD.provinceFilter,null);assert.equal(c.PROD.preview,null);
  c.api=async(url,body)=>{c.requests.push({url,body:plain(body)});return {district:body.district,quotes:[]};};
  await c.constructionQuoteWorkshop('US-NV');assert.equal(c.PROD.provinceFilter,null);
  assert.equal(c.PROD.moduleQuotes.district,'US-NV');assert.equal(c.requests[0].url,'/api/industry-module-quotes');
  c.openConstruction({province:'US-CA',kind:'starter_industry'});
  assert.deepEqual(plain(c.constructionWorkshopProvinces()).map(p=>p.id),['US-CA']);
  assert.equal(c.constructionWorkshopDistrict(),'US-CA');assert.equal(c.constructionChooseWorkshop('US-NV'),false);
  assert.equal(c.PROD.provinceFilter,'US-CA');assert.equal(c.PROD.workshopDistrict,'US-CA');
  assert.doesNotMatch(c.constructionWorkshopHtml(),/<option value="US-NV"/);
});

test('changing workshop location invalidates an earlier pending quote without issuing an order',async()=>{
  const c=fixture();c.PROD.data.provinces=[{id:'US-CA'},{id:'US-NV'}];c.PROD.provinceFilter=null;
  let resolve;c.api=()=>new Promise(r=>resolve=r);
  const pending=c.constructionQuoteWorkshop('US-CA');c.constructionChooseWorkshop('US-NV');
  resolve({district:'US-CA',quotes:[{district:'US-CA',capacity_micros:10}]});await pending;
  assert.equal(c.PROD.moduleQuotes,null);assert.equal(c.PROD.moduleLoading,false);
  assert.equal(c.PROD.workshopDistrict,'US-NV');assert.equal(c.PROD.provinceFilter,null);
});

test('monthly transition notice explains all chooser screens and disables effect review without changing eligibility',async()=>{
  const c=fixture(),notice='Construction effects reviews need daily play. This older campaign will switch after the current month finishes.';
  c.PROD.data.preview_notice=notice;c.PROD.pickKind='power_grid';
  const project={kind:'power_grid',name:'Power Grid',eligible_provinces:['US-CA'],actions:{start:true}};
  const province={id:'US-CA',name:'California'};
  c.PROD.data.catalog=[project];c.PROD.data.provinces=[province];
  c.PROD.moduleQuotes={district:'US-CA',quotes:[{district:'US-CA',capacity_micros:100,label:'Small workshop'}]};
  const body={scrollTop:0,innerHTML:'',querySelector:()=>null};
  c.$=()=>body;c.document={activeElement:null};c.operationsDetailsState=()=>({});c.operationsRestoreDetails=()=>{};
  c.wireProductionPanel=()=>{};vm.runInContext(fn('renderProductionPanel'),c);
  for(const view of ['catalog','provinces','workshop']){
    c.PROD.view=view;c.renderProductionPanel();assert(body.innerHTML.includes(notice),view);
    assert.match(body.innerHTML,/class="construction-preview-status" role="status"/);
    if(view==='provinces')assert.match(body.innerHTML,/data-prod-province="US-CA" disabled/);
    if(view==='workshop')assert.match(body.innerHTML,/data-construction-workshop="0" disabled/);
  }
  assert.equal(c.productionEligible(project,province),true,'Monthly command eligibility remains server-owned and unchanged');
  assert.equal(await c.constructionPreviewProject('power_grid','US-CA'),false);assert.equal(c.requests.length,0);
  c.PROD.data.preview_notice=null;assert.equal(c.constructionPreviewNoticeHtml(),'');
  assert.doesNotMatch(c.productionProvinceHtml(),/data-prod-province="US-CA" disabled/);
  assert.doesNotMatch(c.constructionWorkshopHtml(),/data-construction-workshop="0" disabled/);
  c.PROD.data.preview_notice='<script>notice</script>';assert.match(c.constructionPreviewNoticeHtml(),/&lt;script>/);
});

test('an explicit project link waits for its card, then reveals it once without stealing focus on refresh',()=>{
  const c=fixture(),calls=[];c.openConstruction({project:7});c.PROD.loading=true;
  const body={querySelectorAll:()=>[]};c.constructionRevealProject(body);assert.equal(c.PROD.revealProject,'7');
  const card={dataset:{prodProject:'7'},focus:value=>calls.push(['focus',plain(value)]),scrollIntoView:value=>calls.push(['scroll',plain(value)])};
  body.querySelectorAll=()=>[card];c.constructionRevealProject(body);
  assert.equal(c.PROD.revealProject,null);assert.equal(calls.length,2);
  c.constructionRevealProject(body);assert.equal(calls.length,2);
  c.PROD.revealProject='gone';c.PROD.loading=false;c.constructionRevealProject(body);
  assert.equal(c.PROD.revealProject,null);assert.equal(c.requests.length,0);
});
test('workshop quotes send campaign identity and ignore a delayed previous campaign',async()=>{
  const c=fixture();let resolve;c.api=(url,body)=>{c.requests.push({url,body:plain(body)});return new Promise(r=>resolve=r);};
  c.PROD.data.provinces=[{id:'US-CA',name:'California'}];
  const pending=c.constructionQuoteWorkshop('US-CA');
  assert.deepEqual(c.requests[0],{url:'/api/industry-module-quotes',body:{district:'US-CA',session_id:'one'}});
  c.S={session_id:'two'};resolve({district:'US-CA',quotes:[{cost_bn:1}]});await pending;
  assert.equal(c.PROD.moduleQuotes,null);assert.equal(c.PROD.moduleLoading,false);
});
test('workshop selection reviews impacts first and confirms the exact frozen size through the command channel',async()=>{
  const c=fixture(),q={district:'US-CA',capacity_micros:5001,cost_bn:.00290058,lower_bound_days:92.4,output_daily:.03,
    label:'Budget fit',can_start:true,requirements:[{name:'Iron'}],political_cost:12};
  c.PROD.provinceFilter='US-CA';c.PROD.data.provinces=[{id:'US-CA',name:'California'}];
  c.PROD.moduleQuotes={district:'US-CA',quotes:[q]};
  const html=c.constructionWorkshopHtml();assert.match(html,/\$2.901m/);assert.match(html,/At least 93 days/);
  assert.doesNotMatch(html,/Iron|political|capacity|stock/);
  assert.match(html,/Review effects/);assert.doesNotMatch(html,/Add to construction queue/);
  await c.constructionReviewWorkshop(0);
  assert.equal(c.requests.length,1);assert.equal(c.requests[0].url,'/api/construction-preview');
  assert.equal(c.requests[0].body.capacity_micros,5001);
  assert.equal(await c.constructionConfirmPreview(),true);
  assert.deepEqual(c.requests[1].body.commands,[{kind:'start_industry_module',district:'US-CA',capacity_micros:5001}]);
});
test('stale and wrong-province workshop quotes cannot place a project or select a different impact review',async()=>{
  for(const mode of ['stale','wrong']){
    const c=fixture();c.PROD.workshopDistrict='US-CA';c.PROD.data.provinces=[{id:'US-CA'},{id:'US-NV'}];
    c.PROD.moduleQuotes={quotes:[{district:'US-CA',can_start:true,capacity_micros:7}]};
    if(mode==='stale')c.PROD.stale=true;if(mode==='wrong')c.PROD.workshopDistrict='US-NV';
    await c.constructionReviewWorkshop(0);assert.equal(c.requests.length,0,mode);
  }
});
test('cancel explains sunk spending and respects server permission; priorities require allowed transitions',async()=>{
  const c=fixture();c.PROD.data.queue=[project()];
  await c.productionSetPriority(7,'normal');assert.equal(c.requests.length,0);
  await c.productionSetPriority(7,'high');assert.equal(c.requests[0].body.commands[0].priority,'high');
  c.PROD.stale=false;await c.productionCancel(7);assert.match(c.notices.find(x=>x.includes('Cancel Factory')),/Money already spent/);
  assert.equal(c.requests[1].body.commands[0].kind,'cancel_project');
  c.PROD.stale=false;c.PROD.data.queue[0].actions={};await c.productionCancel(7);assert.equal(c.requests.length,2);
});
test('shared construction route carries province/project intent and resets previous-campaign drafts',()=>{
  const c=fixture();c.openConstruction({province:'US-CA'});assert.equal(c.PROD.view,'catalog');assert.equal(c.PROD.provinceFilter,'US-CA');
  c.openConstruction({kind:'starter_industry'});assert.equal(c.PROD.view,'workshop');
  c.openConstruction({project:7});assert.equal(c.PROD.selected,'7');assert.equal(c.PROD.view,'queue');
  c.openConstruction({project:7,province:'US-CA',kind:'starter_industry'});
  assert.equal(c.PROD.view,'queue','An existing project takes priority over province and new-build context');
  assert.equal(c.PROD.selected,'7');assert.equal(c.PROD.revealProject,'7');assert.equal(c.PROD.provinceFilter,'US-CA');
  c.openConstruction({project:'mine:US-CA:iron',province:'US-CA'});
  assert.equal(c.PROD.view,'queue');assert.equal(c.PROD.revealProject,'mine:US-CA:iron');
  c.PROD.budgetDraft='900';c.S={session_id:'two'};c.openConstruction();
  assert.equal(c.PROD.data,null);assert.equal(c.PROD.budgetDraft,null);assert.equal(c.PROD.stale,true);
});
test('impact review shows separate province and national before/after effects, timing and operating needs',()=>{
  const c=fixture(),q=reviewed(c),before=plain(q),html=c.constructionPreviewHtml();
  for(const text of ['For California','For United States','Industrial sites','Potential operating output','Now','After completion',
    '$200m','180 days','About 245 days','What it needs to operate','Running budget','$30k<span>/day</span>','not a guaranteed GDP or tax increase'])assert(html.includes(text),text);
  assert.match(html,/data-construction-confirm >Add to construction queue/);
  assert.deepEqual(plain(q),before,'The UI displays the effect model without recalculating it');
  assert.equal(c.constructionEffectValue(.0325,'%'),'0.0325 <span>%</span>','Units arrive ready to display; no inferred percentage multiplication');
  assert.equal(c.constructionEffectValue(null,'sites'),'—');
  assert.equal(c.constructionEffectValue(0,'sites'),'0 <span>sites</span>');
});
test('impact money uses the shared dollar scale and preserves periods, signs, zero and unknown values',()=>{
  const c=fixture();
  for(const [value,unit,expected] of [
    [.00003,'$bn/day','$30k<span>/day</span>'],
    [.000000025,'$bn/day','$25<span>/day</span>'],
    [.125,'$bn','$125m'],
    [2.5,'$bn / year','$2.5bn<span>/year</span>'],
    [-.00003,'$bn / day','$-30k<span>/day</span>'],
    [0,'$bn/day','$0<span>/day</span>'],
    [0,'$bn','$0'],
    [-0,'$bn/day','$0<span>/day</span>'],[-0,'packs/day','0 <span>packs/day</span>'],
    [-0,'','0'],[-.00000001,'units','-0.00000001 <span>units</span>'],
    [null,'$bn/day','—'],[NaN,'$bn','—'],[Infinity,'$bn/year','—'],
    [.00003,'packs/day','0.00003 <span>packs/day</span>'],
    [.0325,'%','0.0325 <span>%</span>'],
    ['Unspecified','$bn/day','Unspecified <span>$bn/day</span>'],
  ])assert.equal(c.constructionEffectValue(value,unit),expected,`${value} ${unit}`);
});
test('effect comparisons use the server timing label and keep completion as the default',()=>{
  const c=fixture();
  const rows=[
    {label:'Unpaid commitments',before:.2,after:.4,unit:'$bn',after_label:'If queued'},
    {label:'Completed facilities',before:1,after:2,unit:'sites'},
    {label:'Escaped timing',before:0,after:1,after_label:'<img src=x onerror=bad()>'},
  ];
  const before=plain(rows),html=c.constructionEffectRows(rows);
  assert.match(html,/Unpaid commitments[\s\S]*?<small>If queued<\/small>/);
  assert.match(html,/Completed facilities[\s\S]*?<small>After completion<\/small>/);
  assert.match(html,/&lt;img src=x onerror=bad\(\)>/);assert.doesNotMatch(html,/<img/);
  assert.deepEqual(rows,before);
});
test('effects, model notes and requirements are escaped and missing estimates do not become invented gains',()=>{
  const c=fixture();reviewed(c,undefined,impact({name:'<img src=x>',district_name:'<script>province</script>',nation_name:'<iframe>',
    province_effects:[{label:'<svg>',before:null,after:null,unit:'<img>',detail:'<script>detail</script>'}],national_effects:[],
    operating_requirements:[{label:'<img>',value:'<iframe>',unit:'<script>',detail:'<svg>'}],notes:['<script>note</script>']}));
  const html=c.constructionPreviewHtml();assert.doesNotMatch(html,/<img|<script|<iframe|<svg/);
  assert.match(html,/&lt;script>note/);assert.match(html,/No direct change is estimated for this scope/);
  assert.match(html,/<strong>—<\/strong>/);
});
test('province selection makes one read-only impact request; only confirmation starts the reviewed project',async()=>{
  const c=fixture();c.PROD.data.catalog=[{kind:'civilian_industry',name:'Factory',eligible_provinces:['US-CA']}];c.PROD.data.provinces=[{id:'US-CA'}];
  assert.equal(await c.constructionPreviewProject('civilian_industry','US-CA'),true);
  assert.deepEqual(c.requests,[{url:'/api/construction-preview',body:{district:'US-CA',project_kind:'civilian_industry',session_id:'one'}}]);
  assert.equal(c.PROD.view,'preview');assert.equal(c.constructionPreviewCurrent(),true);
  assert.equal(await c.constructionConfirmPreview(),true);
  assert.deepEqual(c.requests[1].body.commands,[{kind:'start_project',project_kind:'civilian_industry',district:'US-CA'}]);
  assert.equal(c.PROD.view,'queue');assert.equal(c.PROD.preview,null);
  assert.match(fn('wireProductionPanel'),/data-prod-province[\s\S]*?constructionPreviewProject\(PROD.pickKind, b.dataset.prodProvince\)/);
  assert.doesNotMatch(fn('productionProvinceHtml'),/Add to queue/);
});
test('a blocked preview remains informative but cannot enqueue work',async()=>{
  const c=fixture();reviewed(c,undefined,impact({can_start:false,reason:'A power grid is required.'}));
  const html=c.constructionPreviewHtml();assert.match(html,/A power grid is required/);assert.match(html,/Industrial sites/);
  assert.match(html,/data-construction-confirm disabled/);
  assert.equal(await c.constructionConfirmPreview(),false);assert.equal(c.requests.length,0);
});
test('state adoption or campaign replacement invalidates preview permission even on the same calendar date',async()=>{
  for(const next of [{session_id:'one',date:'1 Jan 1990'},{session_id:'two',date:'1 Jan 1990'}]){
    const c=fixture();c.S.date='1 Jan 1990';reviewed(c);c.S=next;
    assert.equal(c.constructionPreviewCurrent(),false);assert.equal(await c.constructionConfirmPreview(),false);
    assert.equal(c.requests.length,0);
  }
  const c=fixture();reviewed(c);c.constructionInvalidatePreview();
  assert.equal(c.PROD.preview,null);assert(c.PROD.previewRequest);assert.match(c.PROD.previewError,/Refresh/);
  assert.match(c.constructionPreviewHtml(),/data-construction-preview-refresh/);
  assert.match(fn('adopt'),/constructionInvalidatePreview\(\)/,'Actual adoption invalidates the displayed model reading');
});
test('a delayed effect response cannot overwrite a later project or an adopted state',async()=>{
  const c=fixture(),pending=[];c.api=(url,body)=>new Promise(resolve=>pending.push({body,resolve}));
  const first=c.constructionPreviewProject('civilian_industry','US-CA');
  const second=c.constructionPreviewProject('power_grid','US-NV');
  pending[0].resolve(impact());assert.equal(await first,false);assert.equal(c.PROD.preview,null);
  pending[1].resolve(impact({project_kind:'power_grid',district:'US-NV'}));assert.equal(await second,true);
  assert.equal(c.PROD.preview.project_kind,'power_grid');
  const third=c.constructionPreviewProject('civilian_industry','US-CA');c.S={session_id:'one',date:'next day'};
  pending[2].resolve(impact());assert.equal(await third,false);assert.equal(c.PROD.preview,null);
});
test('a mismatched province, project, frozen size or commodity never creates a usable preview',async()=>{
  for(const bad of [impact({district:'US-NV'}),impact({project_kind:'power_grid'}),impact({capacity_micros:5}),impact({commodity:'iron'})]){
    const c=fixture();c.api=async()=>bad;
    assert.equal(await c.constructionPreviewProject('civilian_industry','US-CA'),false);
    assert.equal(c.constructionPreviewCurrent(),false);assert.match(c.PROD.previewError,/did not match/);
  }
  const c=fixture();c.api=async()=>impact({project_kind:'starter_industry'});
  assert.equal(await c.constructionPreviewProject('starter_industry','US-CA',5001),false,'A workshop preview must echo its exact reviewed size');
});
test('mine effects are reviewed without a resource command, then commit the exact commodity and district',async()=>{
  const c=fixture();
  assert.equal(await c.constructionReviewMine('iron','US-CA'),true);
  assert.equal(c.requests.length,1);
  assert.deepEqual(c.requests[0],{url:'/api/construction-preview',body:{district:'US-CA',project_kind:'resource_mine',commodity:'iron',session_id:'one'}});
  assert.match(c.constructionPreviewHtml(),/Choose a deposit/);
  assert.equal(await c.constructionConfirmPreview(),true);
  assert.deepEqual(c.requests[1].body.commands,[{kind:'develop_resource',commodity:'iron',district:'US-CA'}]);
  assert.match(fn('renderStockDock'),/await constructionReviewMine\(com,target.district\)/);
  assert.doesNotMatch(fn('renderStockDock'),/kind: "develop_resource"/);
});
test('loading, pending commands and read failures keep the final action unavailable and recoverable',async()=>{
  const c=fixture();reviewed(c);c.PROD.previewLoading=true;
  assert.equal(await c.constructionConfirmPreview(),false);assert.match(c.constructionPreviewHtml(),/data-construction-confirm disabled/);
  c.PROD.previewLoading=false;c.pendingAdvance={};assert.equal(await c.constructionConfirmPreview(),false);
  c.pendingAdvance=null;c.api=async()=>{throw new Error('Preview service unavailable');};
  assert.equal(await c.constructionPreviewProject('civilian_industry','US-CA'),false);
  const html=c.constructionPreviewHtml();assert.match(html,/Preview service unavailable/);assert.match(html,/Refresh impact preview/);
  assert.match(html,/data-construction-confirm disabled/);
});
