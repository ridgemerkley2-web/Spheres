// Exercise the shipped theatre renderer and form handlers without a browser.
const assert = require('node:assert/strict');
const {test} = require('node:test');
const ui = require('../../spheres-web/ui/campaign-operations-ui.js');
const fixture = () => ({
  enabled:true, conflict:7, theatre_name:'Persian Gulf',
  order:{conflict:7,nation:'Iraq',target:'IQ-BA',approach:'advance',reserve_bp:1234,air:'reconnaissance',naval:'none'},
  targets:[{id:'IQ-BA',name:'Basra',held:false},{id:'IQ-NI',name:'Nineveh',held:true}],
  fielded:12,reserve:3,garrison:.8,in_transit:1.25,readiness:.63,preparation:.42,
  supply:{coverage:.51,status:'Strained',reason:'The road through Basra is contested.',eta_days:3},
  enemy:{low:4,high:8,confidence:.6,observed_day:10,observed_label:'10 Jan 1990',actual:987654.321},
  forecast:{advance_low:.003,advance_high:.008,loss_low:.02,loss_high:.04,reason:'Readiness is limiting the advance.',period:'per day'},
  last_report:{day:11,day_label:'11 Jan 1990',advanced:.006,retreated:0,losses:.025,readiness:.63,summary:'A supplied reserve held the crossing.'}
});
const peaceFixture = () => ({
  aim:{kind:'recover',districts:['IQ-BA']},
  aim_options:[{kind:'expel',label:'Expel an invader',description:'Restore the legal border.'},{kind:'recover',label:'Recover territory',description:'Recover Basra.'},
    {kind:'concession',label:'Compel a concession',description:'Seek limited terms.'},{kind:'government_change',label:'Government change',description:'Seek a transition.'}],
  targets:[{id:'IQ-BA',name:'Basra'}],can_propose:true,prices:{aim:3,proposal:0,response:0},limits:{reparations_min_bp:1,reparations_max_bp:200,garrison_max_bp:5000},
  proposals:[{id:2,from_name:'Kuwait',incoming:true,summary:'Ceasefire: restore legal borders.',expires_day:24,expires_label:'24 Jan 1990',can_respond:true},
    {id:3,from_name:'Iraq',incoming:false,summary:'Awaiting coalition consent.',can_respond:false}],
  garrison:{share_bp:1000,policy:'restraint'},occupation:[{district:'IQ-BA',name:'Basra',resistance:.31,coverage:.48,days:8}]
});
function bound(kind='operation', values={}, send=async()=>{}, dataset={}) {
  const fields=Object.fromEntries(Object.entries({approach:'advance',target:'IQ-BA',reserve_percent:'12.34',air:'reconnaissance',naval:'none',...values}).map(([key,value])=>[key,{value}]));
  const button={disabled:false},status={textContent:''},sections={};
  for(const selector of ['[data-campaign-recover]','[data-campaign-cede]','[data-campaign-reparations]'])sections[selector]={hidden:true};
  const form={dataset:{campaignCommand:kind,conflict:'7',offer:'2',reparationsMin:'1',reparationsMax:'200',garrisonMax:'5000',...dataset},querySelector(selector){
    if(selector==='button[type="submit"]')return button;
    if(selector==='[role="status"]')return status;
    if(sections[selector])return sections[selector];
    const name=selector.match(/^\[name="([^"]+)"\](?::checked)?$/)?.[1];return fields[name]||null;
  }};
  ui.bind({querySelectorAll:selector=>selector==='form[data-campaign-command]'?[form]:[]},send);
  return {form,fields,button,status,sections,submit:submitter=>form.onsubmit({preventDefault(){},submitter})};
}

test('theatre report shows supplied values, meaningful units and readable dates',()=>{
  const data=fixture(),html=ui.html(data);
  for(const text of ['Persian Gulf','63%','51%','12.00','1.25 in transit','0.80 on garrison duty','National reserve','4.00–8.00','60% confidence','10 Jan 1990',
    '0.30–0.80','Control points per day','Force points per day','Readiness is limiting the advance.',
    'The road through Basra is contested.','Next delivery: 3 days','A supplied reserve held the crossing.','11 Jan 1990',
    'value="12.34"','value="IQ-BA" selected','value="advance" checked'])assert(html.includes(text),text);
  assert(!html.includes('987654.321'),'No fallback to an exact hidden enemy value');
  assert(html.includes('Changing the draft below does not recalculate this report.'));
  assert.equal(ui.html({...data,enabled:false}),'');
  assert.equal(ui.html(null),'');
});

test('missing intelligence, force and delivery data stay unknown rather than becoming zero',()=>{
  const data=fixture();data.enemy={actual:99999};data.fielded=null;data.readiness=null;data.supply={};data.forecast={};data.last_report=null;
  const html=ui.html(data);
  for(const text of ['Unknown','Not yet assessed','No current observation','Delivery time unknown','No field report yet'])assert(html.includes(text),text);
  assert(!html.includes('99999'));assert(!html.includes('0% confidence'));assert(!html.includes('NaN'));
});

test('remote names, summaries, identifiers and saved stale targets are escaped',()=>{
  const attack='"<img src=x onerror=alert(1)>&';const data=fixture();
  data.theatre_name=attack;data.targets[0].name=attack;data.targets[0].id=attack;data.order.target=attack;
  data.forecast.reason=attack;data.supply.reason=attack;data.last_report.summary=attack;
  const peace=peaceFixture();peace.proposals[0].summary=attack;peace.occupation[0].name=attack;
  const html=ui.html(data,peace);assert(!html.includes('<img'));assert(html.includes('&quot;&lt;img'));
  data.targets=[];assert(ui.html(data).includes('review target'));
});

test('support mission constraints show the served explanation and escape remote text',()=>{
  const data=fixture();data.mission_constraints=['No serviceable aircraft are available.','<img src=x onerror="alert(1)">'];
  const html=ui.html(data);
  assert(html.includes('aria-label="Support mission constraints"'));
  assert(html.includes('No serviceable aircraft are available.'));
  assert(html.includes('&lt;img src=x onerror=&quot;alert(1)&quot;&gt;'));
  assert(!html.includes('<img'));
  assert(!ui.html(fixture()).includes('campaign-mission-constraints'));
});

test('operation submit preserves exact basis points and sends authenticated intent only',async()=>{
  const calls=[],f=bound('operation',{},async c=>calls.push(c));await f.submit();
  assert.deepEqual(calls,[{kind:'operation',conflict:7,target:'IQ-BA',approach:'advance',reserve_bp:1234,air:'reconnaissance',naval:'none'}]);
  assert(!('nation' in calls[0]));assert.equal(f.button.disabled,false);assert.equal(f.status.textContent,'Operation order applied.');
});

test('automatic target, withdrawal, all mission enums and reserve endpoints dispatch correctly',async()=>{
  for(const [value,bp] of [['0',0],['100',10000],['.5',50],['0.01',1]]) {
    const calls=[],f=bound('operation',{target:'',approach:'fighting_withdrawal',reserve_percent:value,air:'none',naval:'sea_denial'},async c=>calls.push(c));
    await f.submit();assert.equal(calls.length,1);assert.equal(calls[0].reserve_bp,bp);assert.equal(calls[0].target,null);assert.equal(calls[0].approach,'fighting_withdrawal');
  }
  for(const air of ['none','reconnaissance','interception','ground_support','interdiction'])for(const naval of ['none','transport','escort','sea_denial']) {
    const calls=[],f=bound('operation',{air,naval},async c=>calls.push(c));await f.submit();assert.equal(calls[0].air,air);assert.equal(calls[0].naval,naval);
  }
});

test('malformed operations are refused before sending anything',async()=>{
  for(const values of [{reserve_percent:'-1'},{reserve_percent:'100.01'},{reserve_percent:'NaN'},{reserve_percent:'1e1'},{reserve_percent:''},{reserve_percent:'0.001'},
    {air:'fighter'},{naval:'fleet'},{approach:'withdraw'}]) {
    let calls=0;const f=bound('operation',values,async()=>calls++);await f.submit();assert.equal(calls,0,JSON.stringify(values));assert(f.status.textContent);
  }
  for(const conflict of ['-1','7.5','4294967296','no']) {
    let calls=0;const f=bound('operation',{},async()=>calls++,{conflict});await f.submit();assert.equal(calls,0);
  }
});

test('one in-flight decision is allowed across operation and peace forms; failure recovers',async()=>{
  let fail,calls=0;const f=bound('operation',{},()=>{calls++;return new Promise((_,reject)=>{fail=reject;});});
  const other=bound('propose',{terms:'ceasefire'},async()=>calls++),pending=f.submit();
  assert(f.button.disabled);await f.submit();await other.submit();assert.equal(calls,1);assert.match(other.status.textContent,/Wait for the current order/);
  fail(Error('The route closed.'));await pending;assert.equal(f.button.disabled,false);assert.equal(f.status.textContent,'The route closed.');
  await other.submit();assert.equal(calls,2);
});

test('a returned server refusal never claims an applied order',async()=>{
  const f=bound('operation',{},async()=>({errors:['The conflict ended.']}));await f.submit();assert.equal(f.status.textContent,'The conflict ended.');assert.equal(f.button.disabled,false);
});

test('peace reports expose only actionable offers and authoritative prices',()=>{
  const html=ui.html(fixture(),peaceFixture());
  for(const text of ['Current aim:','Recover territory','Set war aim · 3 PC','Propose terms · free','Accept terms · free','24 Jan 1990','31%','48%','political transition'])assert(html.includes(text),text);
  assert(html.includes('data-offer="2"'));assert(!html.includes('data-offer="3"'));
  const data=peaceFixture();data.can_propose=false;data.reason='Only the principal can propose terms.';
  const restricted=ui.peaceHtml(data,7);assert(!restricted.includes('data-campaign-command="propose"'));assert(restricted.includes(data.reason));
});

test('aim commands preserve a district only for territorial recovery',async()=>{
  for(const kind of ['expel','recover','concession','government_change']) {
    const calls=[],f=bound('set_aim',{aim:kind,aim_district:'IQ-BA'},async c=>calls.push(c));await f.submit();
    assert.deepEqual(calls,[{kind:'war_diplomacy',order:{kind:'set_aim',conflict:7,aim:kind==='recover'?{kind,districts:['IQ-BA']}:{kind}}}]);
  }
  let sent=false;const f=bound('set_aim',{aim:'recover',aim_district:''},async()=>{sent=true;});await f.submit();assert.equal(sent,false);assert.match(f.status.textContent,/Choose the territory/);
  f.fields.aim.value='recover';f.fields.aim.onchange();assert.equal(f.sections['[data-campaign-recover]'].hidden,false);
  f.fields.aim.value='expel';f.fields.aim.onchange();assert.equal(f.sections['[data-campaign-recover]'].hidden,true);
});

test('peace terms dispatch exact demand payloads without stale hidden fields',async()=>{
  for(const kind of ['ceasefire','cede','reparations','transition']) {
    const calls=[],f=bound('propose',{terms:kind,cede_district:'IQ-BA',reparation_percent:'1.25'},async c=>calls.push(c));await f.submit();
    const terms=kind==='cede'?{kind,districts:['IQ-BA']}:kind==='reparations'?{kind,share_bp:125}:{kind};
    assert.deepEqual(calls,[{kind:'war_diplomacy',order:{kind:'propose',conflict:7,terms}}]);
    f.fields.terms.onchange();assert.equal(f.sections['[data-campaign-cede]'].hidden,kind!=='cede');assert.equal(f.sections['[data-campaign-reparations]'].hidden,kind!=='reparations');
  }
  let calls=0;const f=bound('propose',{terms:'cede',cede_district:''},async()=>calls++);await f.submit();assert.equal(calls,0);assert.match(f.status.textContent,/Choose the district/);
});

test('reparations honor served bounds and an absent quote cannot authorize a default',async()=>{
  for(const value of ['0','2.01','5']) {let sent=false;const f=bound('propose',{terms:'reparations',reparation_percent:value},async()=>{sent=true;});await f.submit();assert.equal(sent,false);}
  let sent=false;const missing=bound('propose',{terms:'reparations',reparation_percent:'1'},async()=>{sent=true;},{reparationsMax:''});await missing.submit();assert.equal(sent,false);
  const calls=[],custom=bound('propose',{terms:'reparations',reparation_percent:'0.5'},async cmd=>calls.push(cmd),{reparationsMin:'10',reparationsMax:'50'});await custom.submit();assert.equal(calls[0].order.terms.share_bp,50);
});

test('accept and reject require the explicitly chosen response',async()=>{
  const calls=[],f=bound('respond',{},async c=>calls.push(c));await f.submit();assert.equal(calls.length,0);
  for(const value of ['false','true']) {const submitter={value,disabled:false};await f.submit(submitter);assert.equal(submitter.disabled,false);}
  assert.deepEqual(calls,[{kind:'war_diplomacy',order:{kind:'respond',offer:2,accept:false}},{kind:'war_diplomacy',order:{kind:'respond',offer:2,accept:true}}]);
});

test('garrisons have their own bounded allocation and policy',async()=>{
  const calls=[],f=bound('garrison',{garrison_percent:'12.34',policy:'reconstruction'},async c=>calls.push(c));await f.submit();
  assert.deepEqual(calls,[{kind:'war_diplomacy',order:{kind:'garrison',conflict:7,share_bp:1234,policy:'reconstruction'}}]);
  for(const value of ['50.01','100','-1','0.001']) {
    let sent=false;const bad=bound('garrison',{garrison_percent:value,policy:'security'},async()=>{sent=true;});await bad.submit();assert.equal(sent,false);
  }
});

test('editing can pause the host clock without storing drafts across campaigns',()=>{
  let edits=0;const f=bound();ui.bind({querySelectorAll:()=>[f.form]},async()=>{},()=>{edits++;});
  assert.equal(edits,0);f.form.onfocusin();assert.equal(edits,1);
});
