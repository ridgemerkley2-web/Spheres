// Run: node --test tools/ui/check_province_economy.cjs
// Actual renderers and async request paths; no economic model is duplicated.
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const {test}=require('node:test');
const root=path.resolve(__dirname,'../..');
const page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
const source=fs.readFileSync(path.join(root,'spheres-web/ui/province-economy-ui.js'),'utf8');
const css=fs.readFileSync(path.join(root,'spheres-web/ui/province-economy.css'),'utf8');
const plain=value=>JSON.parse(JSON.stringify(value));

function pageFunction(name) {
  const found=new RegExp(`^(?:async\\s+)?function ${name}\\(`,'m').exec(page);
  assert(found,`Missing actual page function ${name}`);
  const first=page.slice(found.index,page.indexOf('\n',found.index));
  if(/\}\s*$/.test(first))return first;
  return page.slice(found.index,page.indexOf('\n}',found.index)+2);
}

function element(nation='USA') {
  let html='';
  return {dataset:{nation},attributes:{},details:[],buttons:[],writes:[],scrollTop:0,
    classList:{toggle(){},add(){},remove(){}},
    set innerHTML(value){
      html=value;this.writes.push(value);
      this.details=[...value.matchAll(/<details\b[^>]*data-detail-key="([^"]+)"[^>]*>/g)]
        .map(match=>({dataset:{detailKey:match[1]},open:/\sopen(?:\s|>)/.test(match[0])}));
    },
    get innerHTML(){return html;},
    contains(){return false;},
    setAttribute(name,value){this.attributes[name]=value;},
    querySelectorAll(selector){
      if(selector==='details[data-detail-key]')return this.details;
      if(selector==='[data-economy-province]')return this.buttons;
      return [];
    },
  };
}

function fixture(extra=[]) {
  const box=element(),calls=[];
  const c=vm.createContext({box,calls,
    document:{activeElement:null,querySelector:selector=>selector==='#nationEconomicLedger'?box:null},
    closeSheet:()=>calls.push('close-sheet'),showTab:tab=>calls.push(['tab',tab]),
    selectProvince:(id,move)=>calls.push(['province',id,move]),
    api:async url=>{calls.push(url);return reading();},
  });
  vm.runInContext(`let S={date:'2 Jan 1991',year:1991,month:1,day:2,player:'USA'};
    let selected='USA',nationView='economy',selectedDistrict='US-CA';
    let PROVINCE_POPULATION=null,PROVINCE_POPULATION_REQUEST=0;
    const DINDEX={'US-CA':{name:'California'},'FR-IDF':{name:'Île-de-France'}};
    function renderProvinceDossier(){calls.push(PROVINCE_POPULATION);}
    ${source}
    ${extra.map(pageFunction).join('\n')}`,c);
  c.box=box;c.calls=calls;return c;
}
const run=(c,code)=>vm.runInContext(code,c,{timeout:2000});
function reading(overrides={}) {
  return {total_gdp_bn:101,inherited_gdp_bn:100,project_gdp_bn:1,opening_gdp_bn:90,
    change_since_opening:.12222,receipt_date_label:'Work settled 1 Jan 1991 · current date 2 Jan 1991',
    sectors:[{id:'services',name:'Services',gdp_bn:81,share:81/101},{id:'industry',name:'Industry',gdp_bn:20,share:20/101}],
    projects:[{id:'work-1',name:'Rail project',district:'US-CA',kind:'infrastructure',
      classification:'incremental_value_added',status:'building',reason:'Work completed today adds value.',counted:true,
      annual_gdp_bn:1,daily_value_added_bn:.0027,gross_output_daily_bn:.0037,intermediate_inputs_daily_bn:.001}],
    note:'Game estimates, not historical regional accounts. Real work is annualized over 365 days.',
    province_count:1,unallocated_gdp_bn:0,provinces:[{id:'US-CA',name:'California',total_gdp_bn:101}],...overrides};
}

test('Materials GDP bridge displays the served decomposition without adding observed output twice',()=>{
  const c=fixture(),materials={background_annual_bn:10.1234,observed_annual_bn:3.5678,
    already_included_annual_bn:3.4321,additional_annual_bn:.1357,unobserved_annual_bn:6.6913,total_annual_bn:10.2591};
  const data=reading({materials_accounting:materials}),before=JSON.stringify(data);
  for(const scope of ['nation','province']){
    const html=c.economicCompositionHtml(data,scope);
    for(const text of ['Materials: observed vs inherited','Observed Materials output','Already included in GDP',
      'Additional output','Still represented by the background model','$3.568bn','$3.432bn','$135.7m','$6.691bn',
      'Observed output is not all new GDP'])assert(html.includes(text),text);
    assert.match(html,/<details class="pe-details" data-detail-key="economy-materials">/);
  }
  assert.equal(JSON.stringify(data),before);assert.equal(c.calls.length,0);
  assert(!c.economicCompositionHtml(reading(),'nation').includes('economy-materials'));
});

test('An observed inherited Materials project names the included GDP share without subtracting in JavaScript',()=>{
  const c=fixture();
  const html=c.economyProjectsHtml([{kind:'inherited_materials',name:'Materials conversion',counted:true,
    annual_gdp_bn:.1234,inherited_annual_gdp_bn:.1,status:'running'}]);
  assert(html.includes('$123.4m'));assert(html.includes('$100m'));
  assert(html.includes('already represented in the inherited economy'));
  assert(html.includes('Observed project output / year'));
  assert(!html.includes('$23.4m'),'the renderer does not calculate an additional GDP amount');
});

test('All-country and province inherited estimates stay compact and expose location uncertainty',()=>{
  const c=fixture(),model={factory_equivalents:.004321,current_output_annual_bn:.00034568,utilization:.8,
    province_count:0,unallocated_factory_equivalents:.004321,
    groups:[{key:'food_textiles',name:'Food & textiles',factory_equivalents:.000012345,
      current_output_annual_bn:.0000009876,utilization:1.1}],
    sources:[{origin:'Tonga',share_quality:'estimated',mix_quality:'estimated',source:'Game model',notes:'Not establishments.'}],
    allocation_basis:'Population-weighted proxy; no source locates these factories.',note:'GDP is already included.'};
  const nation=c.economicCompositionHtml(reading({starting_industry:model}),'nation');
  const province=c.provinceEconomyHtml({economy:reading(),starting_industry:{...model,district:'TG-1',origin:'Tonga',source:model.sources[0]}});
  for(const html of [nation,province]){
    assert.match(html,/data-detail-key="economy-inherited-industry"/);
    assert.doesNotMatch(html,/<details[^>]*data-detail-key="economy-inherited-industry"[^>]*\sopen/);
    assert(html.includes('0.000012345'));assert(html.includes('110.0%'),'utilization is not clamped to 100%');
    assert(html.includes('Population-weighted proxy'));assert(html.includes('not literal buildings'));
    assert(html.includes('GDP already included'));assert(html.includes('not stockpile packs'));
  }
  assert(nation.includes('0.004321'),'unmapped national estimates are not silently removed');
  assert(province.includes('Location proxy'),'province estimates must not imply a measured factory location');
  assert(!c.economicCompositionHtml(reading(),'nation').includes('economy-inherited-industry'));
});

test('Inherited counts retain tiny nonzero capacity and unknown amounts never become free zeros',()=>{
  const c=fixture();
  assert.equal(c.economyCapacityNumber(.00000000012345),'0.00000000012345');
  for(const bad of [null,undefined,NaN,Infinity,-Infinity,'1',false])assert.equal(c.economyCapacityNumber(bad),'—');
  const html=c.economyStartingIndustryHtml({groups:[null,{}],sources:[null,{}]},'province');
  assert.doesNotMatch(html,/NaN|Infinity|undefined|<strong>0<\/strong>/);assert(html.includes('not available'));
});

test('the real page loads the assets and refreshes selected economy readings from server state',()=>{
  new vm.Script(source);
  for(const block of page.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/g))if(block[1].trim())new vm.Script(block[1]);
  for(const asset of ['/province-economy-ui.js','/province-economy.css'])assert(page.includes(asset));
  assert.match(pageFunction('adopt'),/invalidateEconomicLedger\(\)/);
  assert.match(pageFunction('adopt'),/if \(selectedDistrict\) loadProvincePopulation\(selectedDistrict, true\)/);
  assert.match(pageFunction('selectNationView'),/view === "economy"[\s\S]*fillNationEconomy\(selected\)/);
  assert.match(pageFunction('provinceDossierHtml'),/provinceEconomyHtml\(pop\)/);
  assert.match(pageFunction('openNation'),/rememberNationEconomy\(\)/);
  assert.match(pageFunction('openNation'),/nationEconomyHtml\(n.id\)/);
  assert.match(pageFunction('renderMap'),/rememberProvinceDossier\(\)/);
  assert.match(pageFunction('renderProvinceDossier'),/provinceDossierView\(selectedDistrict\)/);
  assert.match(pageFunction('renderProvinceDossier'),/economicRestoreDetails\(box, view.details\)/);
  assert.match(page,/resetProvinceDossierState\(\);\s*tech.data = null/);
  assert.doesNotMatch(source,/PROD\.data|MANU\.data|population\s*\*|\.gdp\s*\*/,'the dossier must not invent geographic output from player caches or population');
});

test('money and percentages keep missing amounts unknown, zero exact and small output visible',()=>{
  const c=fixture();
  for(const bad of [null,undefined,NaN,Infinity,-Infinity,'10',false,{}]){
    assert.equal(c.economyMoney(bad),'—');assert.equal(c.economyPercent(bad),'—');
  }
  assert.equal(c.economyMoney(0),'$0');assert.equal(c.economyMoney(-0),'$0');assert.equal(c.economyMoney(1234),'$1.234tn');
  assert.equal(c.economyMoney(.0027),'$2.7m');assert.equal(c.economyMoney(.0000002),'$200');
  assert.equal(c.economyMoney(-.000002),'$-2k');
  assert.equal(c.economyPercent(.125),'12.5%');assert.equal(c.economyPercent(.125,true),'+12.5%');
  assert.equal(c.economyPercent(-.125,true),'-12.5%');
});

test('province output, sectors and work amounts are served values, never added again in the browser',()=>{
  const c=fixture(),data=reading(),before=plain(data);
  // Intentionally non-reconciling fixture values prove the renderer does not
  // repair or substitute the authoritative total with its own calculation.
  data.total_gdp_bn=777;
  const html=c.provinceEconomyHtml({id:'US-CA',economy:data});
  for(const text of ['$777bn','$100bn','$1bn','$81bn','$20bn','80.2%','19.8%',
    'Inside the economy','Sector totals combine modeled inherited activity and counted project output',
    'not measured historical state accounts','GDP, not government cash',data.receipt_date_label,
    'Construction contributes while work is performed','Orders are not delivered military production'])assert(html.includes(text),`Missing ${text}`);
  assert(html.includes('data-detail-key="economy-sectors"'));
  assert(html.includes('data-detail-key="economy-projects"'));
  assert.deepEqual({...data,total_gdp_bn:before.total_gdp_bn},before,'rendering must not alter the ledger');
});

test('classification labels distinguish inherited activity, new value, enablers and undelivered orders',()=>{
  const c=fixture();
  const labels={incremental_value_added:'Project value added',inherited_activity:'Inherited activity',
    legacy_unmodeled:'Inherited activity',unpriced_output:'Output · not priced into GDP',
    enabling_asset:'Enabling capacity',inactive_capacity:'Capacity · not operating',pending_order:'Order · not delivered output'};
  for(const [classification,label] of Object.entries(labels)){
    assert.equal(c.economyProjectClass(classification),label);
    const html=c.economyProjectsHtml([{classification,name:'Project',annual_gdp_bn:0,counted:false,
      reason:'A served explanation.',valuation_basis:'no_additional_value',payments_daily_bn:4,
      output_quantity_daily:2.5,output_unit:'packs',daily_value_added_bn:0,
      gross_output_daily_bn:0,intermediate_inputs_daily_bn:0}]);
    for(const text of [label,'$0','A served explanation.','Not an added GDP contribution.',
      'Valuation: no additional value','Payments / day · not GDP','$4bn','2.5 packs'])assert(html.includes(text));
    if(['inherited_activity','legacy_unmodeled'].includes(classification)){
      assert(html.includes('Its separate inherited value is not estimated here.'));
      assert(!html.includes('already inside inherited GDP / year'));
    }
  }
  const inherited=c.economyProjectsHtml([{classification:'inherited_value_added',counted:true,annual_gdp_bn:5}]);
  assert.match(inherited,/Existing project output/);assert.match(inherited,/\$5bn/);
  assert.match(inherited,/GDP attributed to this project \/ year/);
  assert.match(inherited,/not a new GDP bonus/);
  const ongoing=c.economyProjectsHtml([{classification:'incremental_value_added',counted:true,annual_gdp_bn:5}]);
  assert.match(ongoing,/GDP attributed to this project \/ year/);
  assert.doesNotMatch(ongoing,/added GDP \/ year/,'a stable run-rate is not a new daily increase');
});

test('hostile server labels cannot create HTML or break quoted province attributes',()=>{
  const c=fixture(),hostile='\"><img src=x onerror="bad()"><script>bad</script>&';
  const data=reading({note:hostile,receipt_date_label:hostile,
    sectors:[{name:hostile,id:hostile,gdp_bn:1,share:.2}],
    projects:[{name:hostile,district:hostile,classification:hostile,status:hostile,reason:hostile,
      valuation_basis:hostile,output_quantity_daily:1,output_unit:hostile}],
    provinces:[{id:hostile,name:hostile,total_gdp_bn:2}]});
  const html=c.economicCompositionHtml(data,'nation');
  assert.doesNotMatch(html,/<img\b|<script\b|onerror="bad/);
  assert.match(html,/&lt;img/);assert.match(html,/&quot;/);assert.match(html,/&amp;/);
  const button=/<button\b[^>]*data-economy-province="([^"]*)"[^>]*>/.exec(html);
  assert(button);assert(button[1].includes('&quot;'));
  assert(!button[0].replace(/"[^"]*"/g,'""').includes(' onerror='),'hostile text must remain inside its quoted value');
  assert.doesNotMatch(c.provinceEconomyHtml({error:hostile}),/<img\b|<script\b/);
});

test('missing or legacy economic data gets an explicit fallback, not fabricated zeros',()=>{
  const c=fixture();
  assert.match(c.provinceEconomyHtml(null),/Reading the province/);
  assert.match(c.provinceEconomyHtml({id:'US-CA'}),/not available on this server yet/);
  assert.match(c.provinceEconomyHtml({error:'Network offline'}),/Economic reading unavailable.*Network offline/);
  const html=c.economicCompositionHtml({},'province');
  assert.match(html,/<div class="pe-total">—<\/div>/);
  assert(!html.includes('<strong>$0'),'absent amounts are unknown rather than zero');
  assert.match(html,/No sector composition/);assert.match(html,/No project contribution/);
  assert.doesNotMatch(html,/NaN|undefined|Infinity/);
});

test('the national ledger retains unallocated GDP and makes mapped provinces selectable for all countries',()=>{
  const c=fixture();run(c,"selected='France'; S.player='USA';");c.box.dataset.nation='France';
  const html=c.economicCompositionHtml(reading({unallocated_gdp_bn:12,province_count:2,
    provinces:[{id:'FR-IDF',name:'Île-de-France',total_gdp_bn:25},{id:'unmapped',name:'Unmapped',total_gdp_bn:12}]}),'nation');
  assert.match(html,/\$12bn of annual output is not assigned/);
  assert.match(html,/2 mapped/);assert.match(html,/data-economy-province="FR-IDF"/);
  assert.match(html,/<button[^>]*data-economy-province="unmapped"[^>]*disabled/);
  c.box.buttons=[{dataset:{economyProvince:'FR-IDF'}}];
  c.renderNationEconomy('France',{data:reading()});c.box.buttons[0].onclick();
  assert.deepEqual(c.calls,['close-sheet',['tab','map'],['province','FR-IDF',true]]);
  assert.match(c.economyProvincesHtml({unallocated_gdp_bn:101,province_count:0}),/No mapped province breakdown/);
});

test('expanded sections survive insertion, reorder and async rerender by stable keys',()=>{
  const c=fixture(),old=element();
  old.details=[{dataset:{detailKey:'geography'},open:true},{dataset:{detailKey:'borders'},open:false}];
  const state=c.economicDetailsState(old),fresh=element();
  fresh.details=[{dataset:{detailKey:'economy-projects'},open:false},{dataset:{detailKey:'borders'},open:true},
    {dataset:{detailKey:'geography'},open:false}];
  c.economicRestoreDetails(fresh,state);
  assert.deepEqual(fresh.details.map(el=>el.open),[false,false,true]);
  c.renderNationEconomy('USA',{data:reading()});
  c.box.details.find(el=>el.dataset.detailKey==='economy-projects').open=true;
  c.renderNationEconomy('USA',{loading:true});
  c.renderNationEconomy('USA',{data:reading()});
  assert.equal(c.box.details.find(el=>el.dataset.detailKey==='economy-projects').open,true);
});

function provinceRenderFixture() {
  const c=fixture(['renderMap','renderProvinceDossier','provinceDossierHtml','loadProvincePopulation','adopt']);
  let province=element();province.dataset={};
  const stage={appendChild(){}}, mapModes=element();
  const pane={set innerHTML(value){
    // Replacing the real map ancestor destroys the old drawer, just as a
    // browser does. Reusing one fake drawer would hide this regression.
    assert(value.includes('id="provinceDossier"'));
    province=element();province.dataset={};
  }};
  Object.defineProperty(c,'provinceBox',{get:()=>province});
  c.$=selector=>selector==='#provinceDossier'?province:selector==='#pane-map'?pane:
    selector==='#pane-map .globe-stage'?stage:selector==='#mapModes'?mapModes:null;
  c.document.querySelector=selector=>selector==='#provinceDossier'?province:
    selector==='#nationEconomicLedger'?c.box:null;
  c.window={Globe3D:{}};
  for(const name of ['globeBoot','applyCam','glShimmerKick','camWork','invalidateProgramPreview'])c[name]=()=>{};
  c.globeReadout=()=>'';c.rglyph=()=>'';c.resHue=()=>'';
  c.nationOfDistrict=id=>id==='US-CA'?'USA':'France';
  c.nationById=id=>({id,name:id});c.escText=c.economyText;c.provinceDepositHtml=()=>'';
  c.fmt={pop:String,signed:String,pct:String};
  vm.runInContext(`const MAP_MODES={political:{label:'Political',legend:[]}};
    const ui={mapMode:'political',resOverlay:false,cam:{cx:0,cy:0,k:1},picked:['USA']};
    const POL={},SEL={},GLCV={},GLOVL={},STOCKW={},LOGI={open:false},PROD={open:false},MANU={};
    const DISTRICT_SPECS={},TERRAIN={byId:{}};let WARRING,HIST;
    S.wars=[];
    function render(){renderMap();}
  `,c);
  c.data=reading();run(c,"PROVINCE_POPULATION={id:'US-CA',population:30,economy:data};");
  return c;
}
const detail=(c,key)=>c.provinceBox.details.find(el=>el.dataset.detailKey===key);

test('actual adopt → full map rebuild → async province refresh keeps open sections and scroll',async()=>{
  const c=provinceRenderFixture(),requests=[];
  c.api=url=>new Promise(resolve=>requests.push({url,resolve}));
  c.renderMap();
  for(const key of ['economy-sectors','economy-projects','geography']){
    detail(c,key).open=true;detail(c,key).ontoggle();
  }
  c.provinceBox.scrollTop=240;c.provinceBox.onscroll();
  const oldNode=c.provinceBox;
  await c.adopt(run(c,"({...S,date:'3 Jan 1991',day:3})"),false);
  assert.notEqual(c.provinceBox,oldNode,'the actual renderMap path must replace the drawer element');
  for(const key of ['economy-sectors','economy-projects','geography'])assert.equal(detail(c,key).open,true,key);
  assert.equal(c.provinceBox.scrollTop,240);
  assert.equal(requests.length,1);assert.equal(requests[0].url,'/api/district-population/US-CA');
  requests[0].resolve({id:'US-CA',population:30,economy:reading({total_gdp_bn:123})});
  await new Promise(setImmediate); // drain the VM's cross-realm async continuation
  assert(c.provinceBox.innerHTML.includes('$123bn'));
  for(const key of ['economy-sectors','economy-projects','geography'])assert.equal(detail(c,key).open,true,key);
  assert.equal(c.provinceBox.scrollTop,240);
});

test('province choices remain isolated and survive a loading-only drawer and another full rebuild',()=>{
  const c=provinceRenderFixture();c.renderMap();
  detail(c,'economy-projects').open=true;detail(c,'economy-projects').ontoggle();
  run(c,"selectedDistrict='FR-IDF'; PROVINCE_POPULATION={id:'FR-IDF',economy:data};");c.renderProvinceDossier();
  assert.equal(detail(c,'economy-projects').open,false,'French province must not inherit California choices');
  detail(c,'economy-sectors').open=true;detail(c,'economy-sectors').ontoggle();
  run(c,"selectedDistrict='US-CA'; PROVINCE_POPULATION={id:'US-CA',loading:true};");c.renderProvinceDossier();
  assert.equal(detail(c,'economy-projects'),undefined);
  c.renderMap();
  run(c,"PROVINCE_POPULATION={id:'US-CA',economy:data};");c.renderProvinceDossier();
  assert.equal(detail(c,'economy-projects').open,true,'missing temporary sections must not erase their remembered state');
  assert.equal(detail(c,'economy-sectors').open,false,'California must not inherit the French choice');
  detail(c,'economy-projects').open=false;detail(c,'economy-projects').ontoggle();c.renderMap();
  assert.equal(detail(c,'economy-projects').open,false,'explicit closing is also persistent');
});

test('a new campaign clears province choices and rejects previous DOM and pending readings',async()=>{
  const c=provinceRenderFixture();c.renderMap();
  detail(c,'economy-projects').open=true;detail(c,'economy-projects').ontoggle();
  let resolve;c.api=()=>new Promise(done=>{resolve=done;});
  const pending=c.loadProvincePopulation('US-CA',true);
  c.resetProvinceDossierState();
  assert.equal(run(c,'selectedDistrict'),null);assert.equal(run(c,'PROVINCE_POPULATION'),null);
  c.rememberProvinceDossier();
  assert.equal(run(c,'PROVINCE_DOSSIER_UI.views.size'),0,'old live DOM cannot reseed the new campaign cache');
  resolve({id:'US-CA',economy:reading({total_gdp_bn:999})});await pending;
  assert.equal(run(c,'PROVINCE_POPULATION'),null,'old-world response must remain invalid');
  run(c,"selectedDistrict='US-CA'; PROVINCE_POPULATION={id:'US-CA',economy:data};");c.renderMap();
  assert.equal(detail(c,'economy-projects').open,false);
  assert.equal(detail(c,'economy-sectors').open,false);
});

test('national fetching is lazy, caches the exact reading and does not repaint another nation',async()=>{
  const c=fixture(),requests=[];
  c.api=url=>new Promise(resolve=>requests.push({url,resolve}));
  run(c,"nationView='overview';");await c.fillNationEconomy('USA');assert.equal(requests.length,0);
  run(c,"nationView='economy';");const first=c.fillNationEconomy('USA');
  assert.equal(requests.length,1);assert.equal(requests[0].url,'/api/economic-ledger/USA');
  await c.fillNationEconomy('USA');assert.equal(requests.length,1,'an in-flight request is not duplicated');
  run(c,"selected='France';");c.box.dataset.nation='France';
  const second=c.fillNationEconomy('France');const writes=c.box.writes.length;
  requests[0].resolve(reading({total_gdp_bn:999}));await first;
  assert.equal(c.box.writes.length,writes,'a late USA response cannot overwrite the French dossier');
  requests[1].resolve(reading({total_gdp_bn:222}));await second;
  assert(c.box.innerHTML.includes('$222bn'));assert(!c.box.innerHTML.includes('$999bn'));
  await c.fillNationEconomy('France');assert.equal(requests.length,2);
});

test('same-day adoption invalidation and daily changes cannot revive old national responses',async()=>{
  const c=fixture(),requests=[];
  c.api=url=>new Promise(resolve=>requests.push({url,resolve}));
  const first=c.fillNationEconomy('USA'),oldKey=run(c,"economicLedgerKey('USA')");
  c.invalidateEconomicLedger();
  assert.notEqual(run(c,"economicLedgerKey('USA')"),oldKey,'same-day state changes have a distinct generation');
  const second=c.fillNationEconomy('USA');
  requests[0].resolve(reading({total_gdp_bn:999}));await first;
  assert(!c.box.innerHTML.includes('$999bn'));
  requests[1].resolve(reading({total_gdp_bn:222}));await second;
  assert(c.box.innerHTML.includes('$222bn'));
  run(c,"S.date='3 Jan 1991'; S.day=3;");
  const third=c.fillNationEconomy('USA');assert.equal(requests.length,3);
  requests[2].resolve(reading({total_gdp_bn:333}));await third;
  assert(c.box.innerHTML.includes('$333bn'));
});

test('request failures are visible, cached without a retry loop and explicitly retryable',async()=>{
  const c=fixture();let count=0;
  c.api=async()=>{count++;throw new Error('<script>offline</script>');};
  await c.fillNationEconomy('USA');assert.equal(count,1);
  assert.match(c.box.innerHTML,/Economic ledger unavailable/);assert.match(c.box.innerHTML,/&lt;script&gt;offline/);
  assert(!c.box.innerHTML.includes('<script>'));
  await c.fillNationEconomy('USA');assert.equal(count,1);
  c.api=async()=>{count++;return reading();};await c.fillNationEconomy('USA',true);
  assert.equal(count,2);assert(c.box.innerHTML.includes('$101bn'));
});

test('the existing province loader carries economy and rejects stale province responses',async()=>{
  const c=fixture(['loadProvincePopulation']),requests=[];
  c.api=url=>new Promise(resolve=>requests.push({url,resolve}));
  const first=c.loadProvincePopulation('US-CA',false);
  run(c,"selectedDistrict='FR-IDF';");const second=c.loadProvincePopulation('FR-IDF',false);
  requests[1].resolve({id:'FR-IDF',economy:reading({total_gdp_bn:50})});await second;
  requests[0].resolve({id:'US-CA',economy:reading({total_gdp_bn:100})});await first;
  assert.equal(run(c,'PROVINCE_POPULATION.id'),'FR-IDF');
  assert.equal(run(c,'PROVINCE_POPULATION.economy.total_gdp_bn'),50);
  const third=c.loadProvincePopulation('FR-IDF',true);
  assert.equal(run(c,'PROVINCE_POPULATION.economy.total_gdp_bn'),50,'keepOld preserves a reading while the next arrives');
  requests[2].resolve({id:'FR-IDF',economy:reading({total_gdp_bn:55})});await third;
  assert.equal(run(c,'PROVINCE_POPULATION.economy.total_gdp_bn'),55);
});

test('styles keep narrow dossiers contained, large touch controls and reduced-motion support',()=>{
  assert.match(css,/grid-template-columns:repeat\(2,minmax\(0,1fr\)\)/);
  assert.match(css,/@media\(max-width:700px\)/);assert.match(css,/@media\(max-width:370px\)/);
  assert.match(css,/min-height:48px/);assert.match(css,/min-height:56px/);
  assert.match(css,/focus-visible/);assert.match(css,/prefers-reduced-motion:reduce/);
  assert.match(css,/overflow-wrap:anywhere/);assert.match(css,/\.pe-province \.pe-sectors,\.pe-province \.pe-projects \{ grid-template-columns:minmax\(0,1fr\)/);
  assert.equal((css.match(/\{/g)||[]).length,(css.match(/\}/g)||[]).length);
});
