// Actual Exchange rendering and request orchestration; no simulation copied into JS.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const root=path.resolve(__dirname,'../..');
const source=fs.readFileSync(path.join(root,'spheres-web/ui/competition-ui.js'),'utf8');
const economySource=fs.readFileSync(path.join(root,'spheres-web/ui/province-economy-ui.js'),'utf8');
const page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
function fixture() {
  const sent=[],stored=new Map();
  const c=vm.createContext({sent,console,Promise,Number,JSON,
    document:{addEventListener(){}},confirm:()=>true,
    sessionStorage:{setItem:(k,v)=>stored.set(k,v),getItem:k=>stored.get(k),removeItem:k=>stored.delete(k)},
    economyMoney:n=>`$${n}bn`,nextAdvanceIdentity:()=>({client_id:'test',request_seq:1}),
    api:async(p,b)=>{sent.push({p,b:JSON.parse(JSON.stringify(b??null))});return {session_id:'one',errors:[]};},
    adopt:async()=>{},
  });
  vm.runInContext(`let S={session_id:'one',player:'USA',nations:[{id:'USA',name:'United States'},{id:'Canada',name:'Canada'}]};
    let advancing=false,pendingAdvance=null;
    ${economySource}
    economyMoney=n=>Number.isFinite(n)?'$'+n+'bn':'—';
    ${source}
    competitionRender=()=>{};
  `,c);
  c.stored=stored;return c;
}
const run=(c,s)=>vm.runInContext(s,c);

test('Every Exchange snapshot and quote read carries the active campaign identity',async()=>{
  const c=fixture();run(c,'COMP.open=true;');
  await run(c,'competitionFetch()');
  await run(c,`competitionFindSuppliers({elements:{good:{value:'capital_goods'},quantity:{value:'2'},delivery_days:{value:'30'}}})`);
  await run(c,`competitionFindModule('TG-1')`);
  await run(c,`competitionFindMaterials({district:'US-1',quantity:2,delivery_days:30})`);
  assert.equal(c.sent[0].p,'/api/competition?session_id=one');
  assert.equal(c.sent[0].b,null);
  assert.deepEqual(c.sent.slice(1).map(request=>[request.p,request.b.session_id]),[
    ['/api/goods-quotes','one'],
    ['/api/industry-module-quotes','one'],
    ['/api/materials-quote','one'],
  ]);
});

function materialsFixture() {
  return {capacity_daily:4.5678,output_daily:.0012345,demand_daily:1.2345,reserved_daily:.00321,
    stock:.56789,storage_capacity:45,imports_daily:.0023,exports_daily:.0034,
    inherited_gdp_annual_bn:.123,new_gdp_annual_bn:.000045,status:'Needs power',reason:'Generation is unavailable.',
    min_delivery_days:7,max_delivery_days:365,note:'Observed production replaces inherited GDP before adding new output.',
    provinces:[{district:'US-1',name:'Home <Province>',capacity_daily:.9,reserved_daily:0,available_daily:.9,
      recommended_quantity:.9876,recommended_days:30,quote:{district:'US-1',eligible:true,can_start:true,
        quantity:.9876,delivery_days:30,reserved_daily:.03292,capacity_daily:.9,political_cost:2,
        conversion_daily_bn:.000071,energy_daily_bn:.000029,conversion_total_bn:.0123,energy_total_bn:.00456,
        available_conversion_bn:.005,available_energy_bn:.002,feasible_today:.00011,
        blockers:['Needs power <not free>'],requirements:[{name:'iron',unit:'t',required:.000123,stock_available:.009}],
        note:'Full-order cost if fulfilled; no upfront payment.'}}],orders:[]};
}

test('Materials dashboard renders authoritative capacity, output, demand and GDP boundaries',()=>{
  const c=fixture(),data={materials:materialsFixture()};c.materials=data;
  const before=JSON.stringify(data),html=run(c,'competitionMaterialsHtml(materials)');
  for(const text of ['Materials, made here.','Capacity','Actual output','Demand','4.5678','0.0012345','1.2345',
    'Needs power','Generation is unavailable.','Already included in GDP','$0.123bn','Additional GDP','$0.000045bn',
    'government supplies the raw inputs','not a promise of free goods'])assert(html.includes(text),text);
  assert.equal((html.match(/class="comp-material-stat /g)||[]).length,3);
  assert.match(html,/<details class="comp-material-ledger">/);
  assert.equal(JSON.stringify(data),before);assert.equal(c.sent.length,0);
  assert(run(c,'competitionIndustryHtml(materials)').includes('Materials, made here.'));
});

test('Materials is optional for legacy worlds and does not invent missing figures',()=>{
  const c=fixture();
  for(const value of ['null','undefined','false'])assert.equal(run(c,`competitionMaterialsHtml({materials:${value}})`),'');
  const html=run(c,'competitionMaterialsHtml({materials:{}})');
  assert.doesNotMatch(html,/NaN|Infinity|undefined|>0 packs/);assert(html.includes('—'));
  assert(html.includes('No mapped inherited Materials capacity'));
});

test('Materials quote keeps server totals, fractional inputs, eligibility and blockers distinct',()=>{
  const c=fixture();c.materials=materialsFixture();run(c,'COMP.data={materials};COMP.materialOpen=true;COMP.stale=false;');
  const html=run(c,'competitionMaterialsHtml(COMP.data)');
  for(const text of ['$0.0123bn','$0.00456bn','$0.000071bn','0.000123 t','0.009 t',
    'Needs power &lt;not free&gt;','Order materials','Factories','Energy','2 PC','Home &lt;Province&gt;',
    'Current-supply ceiling · packs','An upper bound, not a delivery promise. Existing funded plants run first'])assert(html.includes(text),text);
  assert.doesNotMatch(html,/<not free>|<Province>/);
  assert.match(html,/data-comp-action="materials-order"[^>]*>/,'inputs may block work without forbidding a finite order');
  run(c,'COMP.data.materials.provinces[0].quote.can_start=false;');
  const refused=run(c,'competitionMaterialsHtml(COMP.data)');
  assert.doesNotMatch(refused,/data-comp-action="materials-order"|You can place this order now/);
  assert(refused.includes('not ready to place'));
});

test('Materials purchase sends exact quote through the existing idempotent receipt barrier',async()=>{
  const c=fixture();c.materials=materialsFixture();run(c,'COMP.data={materials};COMP.stale=false;');
  await run(c,`competitionAction({dataset:{compAction:'materials-order'}})`);
  assert.deepEqual(c.sent[0].b.commands,[{kind:'order_materials',district:'US-1',quantity:.9876,delivery_days:30}]);
  assert.equal(c.sent[0].b.session_id,'one');assert.equal(c.sent[0].b.request_seq,1);
});

test('A Materials draft, stale quote or pending turn cannot send a purchase',async()=>{
  for(const mode of ['draft','stale','loading','pending']){
    const c=fixture();c.materials=materialsFixture();run(c,'COMP.data={materials};COMP.stale=false;');
    if(mode==='draft')run(c,'COMP.materialDraft={district:"US-1",quantity:9,delivery_days:30};');
    if(mode==='stale')run(c,'COMP.stale=true;');
    if(mode==='loading')run(c,'COMP.materialLoading=true;');
    if(mode==='pending')run(c,'pendingAdvance={days:1};');
    await run(c,`competitionAction({dataset:{compAction:'materials-order'}})`);assert.equal(c.sent.length,0,mode);
  }
});

test('Delayed Materials quotes cannot overwrite another draft, turn or campaign',async()=>{
  for(const change of ['draft','turn','campaign']){
    const c=fixture();let release;c.api=()=>new Promise(r=>release=r);
    const pending=run(c,`competitionFindMaterials({district:'US-1',quantity:2,delivery_days:30})`);
    if(change==='draft')run(c,'competitionMaterialsDraft({district:"US-2",quantity:3,delivery_days:60});');
    if(change==='turn')run(c,'competitionInvalidate();');
    if(change==='campaign')run(c,'S.session_id="two";');
    release({district:'US-1',quantity:2,delivery_days:30});await pending;
    assert.equal(run(c,'COMP.materialQuote'),null,change);assert.equal(run(c,'COMP.materialLoading'),false,change);
  }
});

test('Materials cancellation selects only a live displayed order and preserves completed deliveries',async()=>{
  const c=fixture();c.materials=materialsFixture();run(c,`COMP.stale=false;COMP.data={materials};COMP.data.materials.orders=[{id:7,status:'running'},{id:8,status:'completed'}];`);
  await run(c,`competitionAction({dataset:{compAction:'materials-cancel',id:'8'}})`);assert.equal(c.sent.length,0);
  await run(c,`competitionAction({dataset:{compAction:'materials-cancel',id:'7'}})`);
  assert.deepEqual(c.sent[0].b.commands,[{kind:'cancel_materials_order',order:7}]);
});

test('Inherited industry renders five served groups without turning estimated capacity into packs',()=>{
  const c=fixture();
  const names=['Food & textiles','Materials','Chemicals','Machinery & electronics','Other manufacturing'];
  const data={starting_industry:{factory_equivalents:9.8765,capacity_annual_bn:7.6543,
    current_output_annual_bn:1.2345,utilization:1.25,province_count:3,unallocated_factory_equivalents:.00321,
    groups:names.map((name,i)=>({name,key:'g'+i,factory_equivalents:i===0?.000012345:i+.1234,
      current_output_annual_bn:.4321,utilization:.6789})),
    sources:[{origin:'A',share_quality:'reported',mix_quality:'estimated',source:'A 1990 source',notes:'A modeled split.'},
      {origin:'B',share_quality:'estimated',mix_quality:'estimated',source:'B model',notes:'No measured split.'}],
    allocation_basis:'Population-weighted location proxy, not a factory census.',note:'Existing inherited GDP only.'}};
  c.inherited=data;const before=JSON.stringify(data);
  const html=run(c,'competitionIndustryHtml(inherited)');
  for(const value of ['Your inherited industry · 1990 estimates',...names,'9.8765','0.000012345',
    '$1.2345bn','125.0%','67.9%','not literal buildings','GDP already included','not stockpile packs',
    'Population-weighted location proxy','reported','estimated','A 1990 source','B model','0.00321'])assert(html.includes(value.replaceAll('&','&amp;')),value);
  assert.equal((html.match(/class="pe-inherited-group /g)||[]).length,5);
  assert(html.includes('Build what you need'),'the physical pack planner stays separate');
  assert.equal(JSON.stringify(data),before,'the renderer cannot adjust national estimates');
  assert.equal(c.sent.length,0,'viewing estimates cannot place construction or trade orders');
});

test('Inherited Exchange estimate is optional and safely displays incomplete data',()=>{
  const c=fixture();
  for(const value of [null,undefined,false]){
    c.missing=value;assert.equal(run(c,'competitionStartingIndustryHtml({starting_industry:missing})'),'');
  }
  const html=run(c,'competitionStartingIndustryHtml({starting_industry:{groups:[null,{}],sources:[null,{}]}})');
  assert.doesNotMatch(html,/NaN|Infinity|undefined|Books not opened|>0</);
  assert(html.includes('—'));assert(html.includes('not available'));
});

test('Inherited source names and modeled location text are escaped in the actual Exchange view',()=>{
  const c=fixture(),unsafe='\"><img src=x onerror="bad()">&';
  c.unsafe=unsafe;
  const html=run(c,`competitionStartingIndustryHtml({starting_industry:{groups:[{name:unsafe}],
    allocation_basis:unsafe,note:unsafe,sources:[{origin:unsafe,share_quality:unsafe,mix_quality:unsafe,source:unsafe,notes:unsafe}]}})`);
  assert.doesNotMatch(html,/<img\b|onerror="bad/);assert.match(html,/&lt;img/);assert.match(html,/&quot;/);
});

test('Inherited-sector context passes through funded, queued and uncovered annual capacity without pack math',()=>{
  const c=fixture(),data={starting_industry:{groups:[]},capacity_plan:{inherited_sectors:[{
    key:'materials',name:'Materials <estimate>',inherited_capacity_annual_bn:.12345,
    funded_capacity_annual_bn:.02345,committed_capacity_annual_bn:.03456,output_annual_bn:.34567,
    total_capacity_annual_bn:.45678,expansion_annual_bn:.05678,pressure:2,status:'server context',
    reason:'Preserve current pack demand. <Do not invent a consumer>'}]}};
  const html=run(c,`competitionStartingIndustryHtml(${JSON.stringify(data)})`);
  for(const value of ['$0.12345bn','$0.02345bn','$0.03456bn','$0.34567bn','$0.45678bn','$0.05678bn',
    'server context','Materials &lt;estimate&gt;','annual value added, not packs','does not replace actual pack demand'])assert(html.includes(value),value);
  assert.doesNotMatch(html,/<Do not invent/);assert(html.includes('&lt;Do not invent'));
  assert.match(html,/<details class="comp-inherited-plan">/,'optional context is collapsed by default');
  assert.equal(c.sent.length,0);
});
test('Workshop uses served fractional prices and shows data coverage and ongoing paid work honestly',()=>{
  const c=fixture();
  run(c,'COMP.moduleOpen=true;');
  const html=run(c,`competitionModuleHtml({module_board:{provinces:[{id:'TG-1',name:'<unsafe province>'}],selection:{district:'TG-1',quotes:[{label:'Budget fit',district:'TG-1',capacity_micros:5000,scale:.005,cost_bn:.0029,output_daily:.005,lower_bound_days:365,political_cost:12,can_start:true,requirements:[{name:'Iron',required:.00015,stock_available:.00002,unit:'t'}]}]},projects:[],legacy_active:1}})`);
  assert(html.includes('$0.0029bn'));assert(html.includes('0.5%'));assert(html.includes('365 days'));
  assert(html.includes('&lt;unsafe province&gt;'));assert(!html.includes('<unsafe province>'));
  assert(html.includes('existing full-size project(s) keep their original cost'));
  assert(html.includes('Iron · 0.00015 t required · 0.00002 in stock'));
  const missing=run(c,`competitionModuleHtml({module_board:{coverage_reason:'No mapped gateway',provinces:[]}})`);
  assert(missing.includes('No mapped gateway'));assert(!missing.includes('data-comp-action="module-build"'));
});
test('Workshop purchase sends exact quoted frozen size through the receipt barrier',async()=>{
  const c=fixture();
  run(c,`COMP.stale=false;COMP.data={module_board:{selection:{district:'TG-1',quotes:[{district:'TG-1',capacity_micros:5001,scale:.005001,cost_bn:.00290058,political_cost:12,can_start:true}]}}};`);
  await run(c,`competitionAction({dataset:{compAction:'module-build',moduleQuote:'0'}})`);
  assert.deepEqual(c.sent[0].b.commands,[{kind:'start_industry_module',district:'TG-1',capacity_micros:5001}]);
  assert.equal(c.sent[0].b.session_id,'one');
});
test('Stale or forbidden module quotes never place an order',async()=>{
  for(const mode of ['stale','blocked','pending']){
    const c=fixture();run(c,`COMP.stale=false;COMP.data={module_board:{selection:{district:'TG-1',quotes:[{district:'TG-1',capacity_micros:1,scale:.000001,cost_bn:.00000058,political_cost:12,can_start:true}]}}};`);
    if(mode==='stale')run(c,'COMP.stale=true;');
    if(mode==='blocked')run(c,`COMP.data.module_board.selection.quotes[0].reason='Contested';`);
    if(mode==='pending')run(c,`COMP.pending={session_id:'one'};`);
    await run(c,`competitionAction({dataset:{compAction:'module-build',moduleQuote:'0'}})`);
    assert.equal(c.sent.length,0,mode);
  }
});
test('A delayed module quote cannot survive a state change or replace the new campaign',async()=>{
  const c=fixture();let resolve;c.api=()=>new Promise(r=>resolve=r);
  const pending=run(c,`competitionFindModule('TG-1')`);
  run(c,'competitionInvalidate();');resolve({district:'TG-1',quotes:[{capacity_micros:1}]});await pending;
  assert.equal(run(c,'COMP.moduleQuotes'),null);assert.equal(run(c,'COMP.moduleLoading'),false);
});
test('Exchange assets, semantic tabs and modal shortcut barrier are shipped',()=>{
  for(const token of ['src="/competition-ui.js"','href="/competition.css"','id="competitionDockBtn"',
    'id="competitionRoom" role="dialog" aria-modal="true"','id="competitionBody"','room?.id === "competitionRoom"',
    'typeof resetCompetition','typeof competitionInvalidate'])assert(page.includes(token),token);
  for(const tab of ['industry','trade','world','sphere'])assert(page.includes(`data-comp-tab="${tab}"`));
  assert(source.includes('room.hidden=false'));assert(source.includes('room.hidden=true'));
  assert(source.includes('setAttribute("aria-labelledby",`comp-tab-${COMP.tab}`)'));
});
test('All four actual view renderers handle an empty campaign',()=>{
  const c=fixture();
  for(const name of ['Industry','Trade','World','Sphere']){
    const html=run(c,`competition${name}Html({})`);
    assert(html.includes('comp-hero'));assert(!html.includes('NaN'));assert(!html.includes('undefined'));
  }
});
test('Industry distinguishes settled receipts from forecasts and an unsettled start',()=>{
  const c=fixture();
  assert(run(c,'competitionIndustryHtml({})').includes('No industry day has settled yet'));
  const html=run(c,`competitionIndustryHtml({industry_settlement:{label:'2 January 1990'}})`);
  assert(html.includes('Last industry settlement: 2 January 1990'));
  assert(html.includes('settled receipts, not a forecast for today'));
});
test('Capacity planning paints both server-authored balances without inventing demand or output',()=>{
  const c=fixture();
  const data={capacity_plan:{goods:[
    {good:'intermediates',installed_daily:.012345,committed_daily:.006789,domestic_daily:.1,export_daily:.2,demand_daily:.76543,stock:5.4321,incoming:9.8765,expansion_daily:.00321,status:'server says wait',reason:'Keep the paid queue. <No duplicate factory>'},
    {good:'capital_goods',installed_daily:.4321,committed_daily:0,domestic_daily:0,export_daily:0,demand_daily:0,stock:25,incoming:0,expansion_daily:0,status:'covered',reason:'Existing machinery covers planned use.'}
  ]}};
  const html=run(c,`competitionCapacityHtml(${JSON.stringify(data)})`);
  assert(html.includes('Build what you need'));assert(html.includes('Materials'));assert(html.includes('Machinery'));
  for(const value of ['0.012345','0.006789','0.76543','5.4321','9.8765','0.00321','server says wait'])assert(html.includes(value),value);
  assert(html.includes('&lt;No duplicate factory&gt;'));assert(!html.includes('<No duplicate factory>'));
  assert(html.includes('not a 1990 factory census'));assert(html.includes('advisory'));
  assert(html.includes('Installed capacity / day'));assert(html.includes('Queued extra / day'));
  assert(html.includes('Incoming is not usable until delivery'));
  assert.equal((html.match(/class="comp-card comp-capacity-card/g)||[]).length,2);
  assert(!html.includes('disabled'),'advice must not disable manual construction');
  assert(run(c,`competitionIndustryHtml(${JSON.stringify(data)})`).includes('Build what you need'));
});
test('A missing capacity plan stays unknown rather than displaying zero demand',()=>{
  const c=fixture();
  const html=run(c,'competitionCapacityHtml({})');
  assert(html.includes('Planning data is not available'));assert(!html.includes('0 packs'));
  assert(!html.includes('NaN'));assert(!html.includes('undefined'));
});

test('Supply forecast paints two authoritative Need Covered Gap cards without recomputing coverage',()=>{
  const c=fixture(),forecast={as_of_day:44,horizon_days:90,lines:[
    {good:'intermediates',operating_daily:.125,project_remaining:2.5,startup_reserve:15,target:28.75,
      stock:4,imports:3,domestic_contracts:2,recent_domestic_daily:.05,projected_domestic:4.5,
      coverage:13.5,shortage:15.25,storage_capacity:250,storage_headroom:246,status:'replenish',reason:'Materials need a paid refill. <not free>'},
    {good:'capital_goods',operating_daily:0,project_remaining:8,startup_reserve:0,target:8,
      stock:8,imports:0,domestic_contracts:0,recent_domestic_daily:0,projected_domestic:0,
      coverage:8,shortage:0,storage_capacity:250,storage_headroom:242,status:'covered',reason:'Machinery is covered.'}
  ]};
  c.forecast=forecast;const before=JSON.stringify(forecast),html=run(c,'competitionSupplyForecastHtml(forecast)');
  assert.equal((html.match(/class="comp-card comp-supply-card/g)||[]).length,2);
  assert(html.includes('class="comp-status warn">replenish</span>'),'a measured supply gap is an attention state');
  for(const text of ['Keep the production chain supplied','Materials','Machinery','Need','Covered','Gap','28.75 packs','13.5 packs','15.25 packs',
    'Operating use / day','0.125','Unfinished project need','2.5','Startup reserve','15','Paid imports incoming','Domestic contracts remaining',
    'Recent domestic output / day','Projected domestic output','Storage headroom','Snapshot day 44','90-day horizon','Materials need a paid refill. &lt;not free&gt;'])assert(html.includes(text),text);
  assert(!html.includes('<not free>'));assert.match(html,/<details><summary>What makes up this forecast<\/summary>/);
  assert.equal(JSON.stringify(forecast),before);assert.equal(c.sent.length,0);
  assert(run(c,'competitionIndustryHtml({supply_forecast:forecast})').includes('Keep the production chain supplied'));
});

test('Missing or partial supply forecasts stay unknown rather than inventing zero coverage',()=>{
  const c=fixture();
  assert.equal(run(c,'competitionSupplyForecastHtml(null)'),'');
  const html=run(c,`competitionSupplyForecastHtml({as_of_day:1,horizon_days:90,lines:[{good:'intermediates'}]})`);
  assert(html.includes('Forecast data is not available for this good.'));
  assert(html.includes('— packs'));assert(!html.includes('NaN'));assert(!html.includes('undefined'));
});

test('Contracted Materials remain distinct from installed capacity and already delivered stock',()=>{
  const c=fixture(),html=run(c,`competitionCapacityHtml({capacity_plan:{goods:[{
    good:'intermediates',installed_daily:1,committed_daily:2,stock:3,incoming:4,
    contracted_daily:.12345,contracted_remaining:6.789,demand_daily:7
  }]}})`);
  for(const text of ['Domestic contracted / day','Domestic contracts remaining','0.12345','6.789',
    'not installed factories or goods already in stock'])assert(html.includes(text),text);
  assert(html.includes('<dt>Installed capacity / day</dt><dd>1</dd>'),'UI must not add contracts to installed plants');
});
test('Server country names, status reasons and input values are escaped',()=>{
  const c=fixture();
  const html=run(c,`competitionWorldHtml({countries:[{name:'<img src=x onerror="bad()">',gdp_bn:2,tier:'Small',plan:{action:'<unsafe>',reason:'<script>bad()</script>'}}]})`);
  assert(!html.includes('<img'));assert(!html.includes('<script>'));assert(html.includes('&lt;script&gt;'));
  assert.equal(run(c,`competitionText(${JSON.stringify('"\'<&>')})`),'&quot;&#39;&lt;&amp;&gt;');
  run(c,`COMP.trade.quantity='" onfocus="bad()';`);
  assert(!run(c,'competitionTradeHtml({})').includes('value="" onfocus='));
});
test('World decisions expose saved supply snapshots, next target and funding under compact disclosure',()=>{
  const c=fixture(),unsafe='<img src=x onerror="bad()">';c.unsafe=unsafe;
  const html=run(c,`competitionWorldHtml({countries:[{name:'Canada',gdp_bn:2,tier:'Small',production:{active:1,completed:2},plan:{
    action:'goods_trade',reason:'A paid move, not free stock.',district:unsafe,project_kind:'machinery_works',
    funding:{available_authority_bn:.25,remaining_work_cost_bn:.75,earliest_years:3.5,basis:unsafe},
    supply_review:{as_of_day:31,horizon_days:90,lines:[
      {good:'intermediates',target:20,coverage:12,shortage:8,status:'replenish',reason:unsafe},
      {good:'capital_goods',target:5,coverage:5,shortage:0,status:'covered',reason:'Enough paid supply.'}
    ]}}}]})`);
  for(const text of ['goods trade','Next industrial target','machinery works','Supply snapshot · 90 days','Materials','Machinery','Need','Covered','Gap','20','12','8',
    'Funding outlook','$0.25bn','$0.75bn','3.5','Snapshot day 31',"captures supply after the government's recorded review action"])assert(html.includes(text),text);
  assert.match(html,/<details class="comp-world-supply"><summary>/);assert(!html.includes('<img'));assert(html.includes('&lt;img'));
  assert(html.includes('data-label="Country"'));assert(html.includes('data-label="Current decision"'));
});
test('World view filters actual server tiers and names, without claiming guaranteed growth',()=>{
  const c=fixture();
  run(c,`COMP.tier='Micro';COMP.filter='island';`);
  const html=run(c,`competitionWorldHtml({countries:[{name:'Small Island',tier:'Micro',gdp_bn:.4},{name:'Large Island',tier:'Major',gdp_bn:2000}]})`);
  assert(html.includes('Small Island'));assert(!html.includes('Large Island'));
  assert(html.includes('1 countries shown'));assert(html.includes('not a claim that every economy must grow'));
});
test('Purchase uses the exact served affordable quote, never the larger form request',async()=>{
  const c=fixture();
  run(c,`COMP.trade.quantity=500;COMP.quotes={delivery_days:30,quotes:[{seller:'Canada',good:'capital_goods',quantity:2.5,unit_price_bn:.003,total_price_bn:.0075}]};`);
  await run(c,`competitionAction({dataset:{compAction:'buy',quote:'0'}})`);
  assert.equal(c.sent.length,1);
  assert.deepEqual(c.sent[0].b.commands,[{kind:'propose_goods_trade',target:'Canada',good:'capital_goods',quantity:2.5,unit_price_bn:.003,delivery_days:30}]);
  assert.equal(c.sent[0].b.session_id,'one');assert.equal(c.sent[0].b.request_seq,1);
});
test('Lost response keeps the exact receipt for retry, and blocks a new order',async()=>{
  const c=fixture();
  c.api=async(p,b)=>{c.sent.push(JSON.parse(JSON.stringify(b)));throw new Error('Network failed');};
  await run(c,`competitionCommand({kind:'accept_goods_offer',offer:8})`);
  assert(c.stored.has('spheres.pending-economic-order'));
  await run(c,`competitionCommand({kind:'accept_goods_offer',offer:9})`);
  assert.equal(c.sent.length,1);
  c.api=async(p,b)=>{c.sent.push(JSON.parse(JSON.stringify(b)));return {errors:[]};};
  await run(c,'competitionSendPending()');
  assert.deepEqual(c.sent[1],c.sent[0]);assert(!c.stored.has('spheres.pending-economic-order'));
});
test('A pending economic order cannot be rebound to a new campaign',async()=>{
  const c=fixture();
  run(c,`COMP.pending={session_id:'old',client_id:'test',request_seq:1,commands:[]};`);
  await run(c,'competitionSendPending()');assert.equal(c.sent.length,0);
  assert(run(c,'COMP.error').includes('older campaign'));
  const apiStart=page.indexOf('async function api(path, body)');
  const apiEnd=page.indexOf('\n}',apiStart);
  vm.runInContext(page.slice(apiStart,apiEnd+2),c);
  await assert.rejects(run(c,`api('/api/command',{session_id:'old',commands:[]})`),/another campaign/);
});
test('Reopening Exchange preserves a live uncertain receipt when browser storage fails',()=>{
  const c=fixture();
  c.document.getElementById=()=>({setAttribute(){},focus(){},style:{},textContent:''});
  c.sessionStorage={getItem(){throw new Error('Storage denied');}};
  run(c,`let PROD={open:false},LOGI={open:false},stock={open:false},tech={open:false};
    function keysCardIsOpen(){return false;} function dominationIsOpen(){return false;}
    function closeGameDrawers(){} function closeSheet(){} function closeTechMenu(){}
    competitionFetch=()=>{};
    COMP.pending={session_id:'one',client_id:'same',request_seq:7,commands:[]};
    openCompetition();`);
  assert.equal(run(c,'COMP.pending?.request_seq'),7);
  c.sessionStorage={getItem:()=>null};
  run(c,'closeCompetition();openCompetition();');
  assert.equal(run(c,'COMP.pending?.request_seq'),7,'An empty store must not erase a live receipt either');
});
test('State changes invalidate in-flight supplier queries and unstick the search button',async()=>{
  const c=fixture();let resolve;
  c.api=()=>new Promise(r=>resolve=r);
  const pending=run(c,`competitionFindSuppliers({elements:{good:{value:'capital_goods'},quantity:{value:'2'},delivery_days:{value:'30'}}})`);
  run(c,'competitionInvalidate()');resolve({quotes:[{seller:'stale'}]});await pending;
  assert.equal(run(c,'COMP.quotes'),null);assert.equal(run(c,'COMP.searching'),false);
});
test('An invalidated snapshot cannot overwrite a newer server state',async()=>{
  const c=fixture();let resolve;
  c.api=()=>new Promise(r=>resolve=r);run(c,'COMP.open=true');
  const pending=run(c,'competitionFetch()');run(c,`competitionInvalidate();COMP.data={date:'new'};`);
  resolve({date:'old'});await pending;
  assert.equal(run(c,'COMP.data.date'),'new');assert.equal(run(c,'COMP.loading'),false);
});
test('A pending turn blocks an immediate economic purchase',async()=>{
  const c=fixture();run(c,'pendingAdvance={days:1};');
  await run(c,`competitionCommand({kind:'accept_goods_offer',offer:8})`);
  assert.equal(c.sent.length,0);assert(run(c,'COMP.error').includes('pending turn'));
});
test('Quota wait overrides a paid project next-day promise without discarding funded work',()=>{
  const c=fixture();const start=page.indexOf('function etaText('),end=page.indexOf('\n}',start);
  vm.runInContext(page.slice(start,end+2),c);
  assert.match(run(c,`etaText({wait:'funded',days_left:0,acquisition_wait:true})`),/next month/);
  assert.equal(run(c,`etaText({wait:'funded',acquisition_wait:false})`),'funded — lands next day');
  assert.equal(run(c,'competitionNumber(-0)'),'0');
});
test('Counteroffer acceptance is shown only to its buyer, never its seller',()=>{
  const c=fixture();
  const offer={id:4,seller:'USA',buyer:'Canada',good:'capital_goods',quantity:2,unit_price_bn:.01};
  const seller=run(c,`competitionTradeHtml(${JSON.stringify({nation:'USA',commerce:{offers:[offer]}})})`);
  assert(seller.includes('Awaiting buyer acceptance'));assert(!seller.includes('data-comp-action="accept"'));
  const buyer=run(c,`competitionTradeHtml(${JSON.stringify({nation:'Canada',commerce:{offers:[offer]}})})`);
  assert(buyer.includes('data-comp-action="accept"'));
});

function pageFunction(name, optional=false) {
  const hit=new RegExp(`^(?:async\\s+)?function ${name}\\(`,'m').exec(page);
  if(!hit&&optional)return '';
  assert(hit,`Missing actual page function ${name}`);
  return page.slice(hit.index,page.indexOf('\n}',hit.index)+2);
}
function sessionFixture() {
  const c=fixture(),nodes=new Map();
  c.$=id=>{if(!nodes.has(id))nodes.set(id,{disabled:false,textContent:'Start',style:{},focus(){}});return nodes.get(id);};
  c.banner=message=>c.sent.push({banner:message});
  c.window={confirm:()=>true};
  c.renderSessionActions=()=>{};c.syncAdvanceControls=()=>{};c.persistPendingAdvance=()=>{};c.noteQueued=()=>{};
  c.seedFromBox=()=>42;c.enterCampaign=async state=>{c.entered=state;};
  run(c,`let queued=[],CAB={},HIST=null,ui={picked:[]},LOGI={open:false},PROD={open:false},MANU={},STOCKW={},selectedDistrict=null;
    let SESSION={live:{player:'USA'},busy:false},pickedNation='USA';
    function defaultPicks(){return [];} function render(){}
    ${pageFunction('guardEconomicOrder',true)}
    ${['api','adopt','advance','doSave','loadCampaign','continueCampaign'].map(n=>pageFunction(n)).join('\n')}
    openCompetition=()=>{COMP.open=true;};competitionFetch=async()=>{};
  `);
  const start=page.indexOf('$("#startBtn").onclick = async () => {');
  assert(start>=0);run(c,page.slice(start,page.indexOf('\n};',start)+3));
  return c;
}
test('A delayed economic receipt cannot race a newer day into the displayed campaign',async()=>{
  const c=sessionFixture();let release;
  const response=data=>({ok:true,text:async()=>JSON.stringify(data)});
  c.fetch=async path=>{
    c.sent.push({path});
    if(path==='/api/command')return new Promise(resolve=>{release=resolve;});
    if(path==='/api/advance')return response({session_id:'one',player:'USA',date:'2 January 1990'});
    return response({});
  };
  run(c,`S.date='1 January 1990';queued=[{kind:'tax',value:.2}];`);
  const order=run(c,`competitionCommand({kind:'set_goods_sale',good:'intermediates',reserve:0,ask_multiplier:1,enabled:true})`);
  assert.equal(run(c,'COMP.busy'),true);
  const advanced=await run(c,'advance(1)');
  release(response({session_id:'one',player:'USA',date:'1 January 1990',errors:[]}));await order;
  assert.equal(advanced,false,'do not permit a later turn while an older command reply is outstanding');
  assert(!c.sent.some(v=>v.path==='/api/advance'));
  assert.equal(run(c,'S.date'),'1 January 1990');
  assert.equal(run(c,'queued.length'),1,'blocked advance keeps the unsent draft');
  assert.equal(run(c,'COMP.open'),true,'the guard provides the Exchange receipt route');
});
test('Uncertain economic receipts block every mutation route but permit read-only Continue',async()=>{
  for(const busy of [false,true]) {
    const c=sessionFixture();
    c.fetch=async path=>{c.sent.push({path});return {ok:true,text:async()=>JSON.stringify({session_id:'one',player:'USA',date:'1 January 1990'})};};
    run(c,`COMP.pending={session_id:'one',client_id:'test',request_seq:7,commands:[]};COMP.busy=${busy};`);
    const receipt=run(c,'JSON.stringify(COMP.pending)');
    assert.equal(await run(c,'advance(1,true)'),false);
    assert.equal(await run(c,'doSave()'),false);
    await run(c,'loadCampaign()');await c.$('#startBtn').onclick();
    for(const path of ['/api/advance','/api/save','/api/load','/api/new','/api/command']) {
      await assert.rejects(run(c,`api(${JSON.stringify(path)},{commands:[]})`),/Exchange/);
    }
    assert(!c.sent.some(v=>v.path),'none of the refused mutations reaches the server');
    assert.equal(run(c,'JSON.stringify(COMP.pending)'),receipt,'a barrier never discards an unknown financial order');
    await run(c,'continueCampaign()');
    assert.deepEqual(c.sent.filter(v=>v.path).map(v=>v.path),['/api/state']);
    assert.equal(c.entered.session_id,'one');
    assert.equal(run(c,'JSON.stringify(COMP.pending)'),receipt);
  }
});
test('The exact economic receipt may still reach the protected command API',async()=>{
  const c=sessionFixture();
  c.fetch=async(path,options)=>{c.sent.push({path,body:JSON.parse(options.body)});return {ok:true,text:async()=>JSON.stringify({session_id:'one',errors:[],command_replayed:true})};};
  run(c,`COMP.pending={session_id:'one',client_id:'test',request_seq:7,commands:[]};`);
  await run(c,'competitionSendPending()');
  assert.equal(c.sent.filter(v=>v.path).length,1);
  assert.equal(c.sent.find(v=>v.path).body.request_seq,7);
  assert.equal(run(c,'COMP.pending'),null);
});
test('A stored receipt blocks the first post-reload mutation before Exchange opens',async()=>{
  const c=sessionFixture();
  c.stored.set('spheres.pending-economic-order',JSON.stringify({session_id:'one',client_id:'test',request_seq:7,commands:[]}));
  c.fetch=async path=>{c.sent.push({path});return {ok:true,text:async()=>JSON.stringify({session_id:'one',player:'USA'})};};
  assert.equal(await run(c,'advance(1)'),false);
  assert(!c.sent.some(v=>v.path));assert.equal(run(c,'COMP.pending.request_seq'),7);
});
test('An economic receipt retry waits for an already in-flight turn to finish',async()=>{
  const c=sessionFixture();
  c.fetch=async path=>{c.sent.push({path});return {ok:true,text:async()=>JSON.stringify({session_id:'one',errors:[]})};};
  run(c,`COMP.pending={session_id:'one',client_id:'test',request_seq:7,commands:[]};advancing=true;`);
  await run(c,'competitionSendPending()');
  assert(!c.sent.some(v=>v.path));assert.equal(run(c,'COMP.pending.request_seq'),7);
  assert.equal(run(c,'COMP.busy'),false);
});
test('A previous-server receipt requires explicit dismissal before an empty server can Load',async()=>{
  const c=sessionFixture();
  c.fetch=async path=>{c.sent.push({path});return {ok:true,text:async()=>JSON.stringify({session_id:'new',player:'USA'})};};
  run(c,`S=null;SESSION.live={session_id:'new',player:null};COMP.pending={session_id:'old',client_id:'test',request_seq:7,commands:[]};`);
  c.window.confirm=()=>false;await run(c,'loadCampaign()');
  assert.equal(run(c,'COMP.pending.request_seq'),7);assert(!c.sent.some(v=>v.path));
  c.window.confirm=()=>true;await run(c,'loadCampaign()');
  assert.equal(run(c,'COMP.pending'),null);assert(!c.sent.some(v=>v.path),'dismissal does not itself replace a campaign');
  await run(c,'loadCampaign()');assert.deepEqual(c.sent.filter(v=>v.path).map(v=>v.path),['/api/load']);
});
