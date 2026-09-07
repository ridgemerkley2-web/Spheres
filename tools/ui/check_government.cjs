// Contract tests for the pure Government presentation. The host owns all orders.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const sourcePath=path.resolve(__dirname,'../../spheres-web/ui/government-ui.js');
const {render}=require(sourcePath);
const copy=x=>JSON.parse(JSON.stringify(x));
const deepFreeze=x=>{if(x&&typeof x==='object'){Object.freeze(x);Object.values(x).forEach(deepFreeze);}return x;};
const buttons=(html,attribute)=>[...html.matchAll(new RegExp(`<button\\b[^>]*${attribute}(?=[\\s=>])(?:="[^"]*")?[^>]*>[^]*?</button>`,'g'))].map(m=>m[0]);
const confirm=html=>buttons(html,'data-gov-confirm');
const reviewHtml=(data,extra={},state={})=>render(data,{tab:'decisions',review:{index:2,data:quote(),...extra},...state});
function fixture(extra={}) {
  return {nation:'Poland',nation_name:'Poland',mine:true,on:true,electoral:true,system:'Parliamentary republic',ruling_institution:'Cabinet',political_capital:31.5,ruling_bloc:'Western',discontent:.273,
    leader:{name:'Tadeusz Mazowiecki',office:'Prime Minister of Poland',since:'1989-08-24',native:'Tadeusz Mazowiecki'},government_of_the_day:'pl_solidarity',government_of_the_day_name:'Solidarity',
    government_seats:.514,term_months:48,months_in_office:7,next_election:'1993-10',strain:4.25,upkeep:1.2,coup_pressure:6.5,
    briefing:{date_label:'11 Feb 1990',summary:'The cabinet holds 51.4% of the chamber.',stats:[{key:'political_capital',label:'Political capital',value:'31.5 / 100',detail:'Recovery depends on current conditions.'},{key:'stability',label:'National stability',value:'63.2 / 100',detail:'Current order.'}],
      attention:[{id:'majority',tone:'good',title:'Your cabinet holds a majority',detail:'Its partners still have a political cost.'},{id:'standing',tone:'attention',title:'Consider the coalition cost',detail:'Review the actual government change.',action_index:2,route:'budget'},{id:'watch',tone:'info',title:'Takeover routes are closed',detail:'Ordinary government survival remains active.'}],
      drivers:[{id:'prices',label:'Prices',value:'12.7%',detail:'Inflation affects political standing.',route:'budget'}],links:[{id:'cash',label:'Public finances',detail:'Review the treasury.',route:'cash_flow'},{id:'build',label:'Construction',detail:'Review investments.',route:'construction'}]},
    bar:[{bloc:'Western',share:.643,backing:.08,governing:true,ruling:true,abroad:[{kind:'covert',weight:.03,exposed:false,sponsor:'SECRET SPONSOR'},{kind:'covert',weight:.02,exposed:true,sponsor:'Visible sponsor'},{kind:'patronage',weight:.03,exposed:true}]},{bloc:'Communist',share:.357,backing:0,banned:true,abroad:[]}],
    groups:[{bloc:'Western',label:'Western',share:.643,parties:[{id:'pl_solidarity',name:'Solidarity',native:'Solidarność',family:'Christian democratic',support:.438,seats:.514,in_government:true,leads:true},{id:'pl_small',name:'Small partner',family:'Liberal',support:.031,seats:.075,in_government:true}]},{bloc:'Communist',label:'Communist',share:.357,banned:true,parties:[{id:'pl_sld',name:'Social Democrats',family:'Socialist',support:.357,seats:.411,pariah:true,banned:true}]}],
    pillars:[{pillar:'Army',key:'army',name:'Armed forces',loyalty:.612,bloc:'Nationalist'},{pillar:'Party',key:'party',name:'Party institutions',loyalty:.833,bloc:'Communist'}],
    takeover:{coup:{open:false,reason:'The ideological takeover route is closed for calibration.',armed:false,half_armed:true,gauges:[{name:'Armed institution support',value:.91,trigger:.7,sense:'Above',progress:.2,met:false}]},uprising:{open:false,reason:'No uprising route is enabled.',armed:false,half_armed:false,gauges:[{name:'Discontent band',value:.27,trigger:.2,upper:.6,sense:'Inside',progress:.8,met:true}]},round_table:{open:true,armed:true,gauges:[{name:'Recorded pressure',value:6,trigger:5,sense:'Above',progress:1,met:true}]},collapse:{open:false,reason:'Rule disabled.',gauges:[]}},
    actions:[{kind:'call_election',category:'coalition',key:'call_election',label:'Call an early election',detail:'Use current public support.',price:15,command:{kind:'call_election'},refusal:'The government is too young to call an election.',effects:['A new seat allocation.']},{kind:'secure_pillar',category:'institutions',key:'secure_pillar:army',label:'Secure Armed forces',price:14,command:{kind:'secure_pillar',pillar:'army'},detail:'Make a one-time institutional payment.',effects:['Loyalty responds to policy afterward.']},{kind:'expel',category:'coalition',key:'expel:pl_small',label:'Expel Small partner',detail:'Remove this partner from the cabinet.',price:8,command:{kind:'expel_from_government',party:'pl_small'},effects:['A smaller coalition.']},{kind:'reform',category:'reform',key:'reform:closed',label:'Closed reform',price:30,command:{kind:'stratagem',id:'closed'},refusal:'This reform is unavailable under the current rules.'}],...extra};
}
function quote(extra={}) {return {valid:true,title:'Expel Small partner',kind:'expel',command:{kind:'expel_from_government',party:'pl_small'},price_pc:3,money_cost_bn:0,description:'Remove this partner from the cabinet.',changes:[{key:'political_capital',label:'Political capital',before:'3.0',after:'0.0',detail:'Actual immediate deduction.'},{key:'seats',label:'Government seats',before:'51.4%',after:'43.9%',detail:'Seats differ from public support.'},{key:'upkeep',label:'Coalition upkeep',before:'1.2 PC / month',after:'0.8 PC / month',detail:'Ongoing cost.'}],effects:['The coalition loses this party.'],warnings:['Only 3 PC is deducted from the current stock.'],...extra};}

test('UMD exposes a frozen pure renderer in Node and a browser global',()=>{
  assert.equal(typeof render,'function');assert.equal(Object.isFrozen(require(sourcePath)),true);
  const context=vm.createContext({});vm.runInContext(fs.readFileSync(sourcePath,'utf8'),context);
  assert.equal(typeof context.GovernmentUI.render,'function');assert.equal(Object.isFrozen(context.GovernmentUI),true);
});
test('rendering every section preserves frozen server data and caller state',()=>{
  const data=deepFreeze(fixture()),saved=JSON.stringify(data);
  for(const tab of ['overview','decisions','politics','watch'])render(data,deepFreeze({tab,query:'',review:{index:2,data:quote()}}));
  assert.equal(JSON.stringify(data),saved);
});
test('hero uses the council artwork and the actual served country and leader',()=>{
  const html=render(fixture());assert.match(html,/src="\/art\/government\/council-v1\.png" alt="" aria-hidden="true"/);
  assert.match(html,/<h1>Poland<\/h1>/);assert.match(html,/Tadeusz Mazowiecki/);assert.match(html,/Prime Minister of Poland/);assert.match(html,/11 Feb 1990/);
  assert.equal((html.match(/<img /g)||[]).length,1);assert.doesNotMatch(html,/leader_art|setupFigures|portrait/i);
});
test('unnamed office holders stay unnamed and preserve their actual described office',()=>{
  const html=render(fixture({leader:{name:null,described:'the office-holder',office:'Head of state',since:'1974-12-17'}}));
  assert.match(html,/the office-holder/);assert.match(html,/Head of state/);assert.doesNotMatch(html,/Mazowiecki|undefined|null/);
});
test('leadership retains succession, native names and other served offices',()=>{
  const html=render(fixture({leader:{name:'Served ruler',native:'Native name',office:'President',must_leave_by:'1996-06-01',heir:{name:'Named successor',office:'Heir',since:'1985-01-01'},also:[{name:'Other official',office:'Prime Minister',since:'1989-01-01'}]}}));
  for(const text of ['Native name','1996-06-01','Named successor','Other official','1989-01-01'])assert.ok(html.includes(text),text);
});
test('briefing uses exact server statistics and policy explanations',()=>{
  const html=render(fixture());for(const text of ['31.5 / 100','63.2 / 100','Recovery depends on current conditions.','12.7%','Inflation affects political standing.'])assert.ok(html.includes(text),text);
  assert.match(html,/<dd>31\.5 \/ 100<small>Recovery depends on current conditions\.<\/small><\/dd>/);
  assert.match(html,/data-gov-route="cash_flow"/);assert.match(html,/data-gov-route="construction"/);
});
test('server attention/good/info tones map to visible accessible treatments',()=>{
  const html=render(fixture());assert.match(html,/gov-ui-tone-positive"><h3>Your cabinet holds a majority/);assert.match(html,/gov-ui-tone-warning"><h3>Consider the coalition cost/);assert.match(html,/gov-ui-tone-neutral"><h3>Takeover routes are closed/);
  assert.match(html,/data-gov-review="2"/);
});
test('only allowlisted short policy routes become navigation controls',()=>{
  const data=fixture();data.briefing.links.push({label:'Unsafe',route:'javascript:alert(1)'},{label:'Unexpected',route:'external'},{label:'Research',route:'research'},{label:'Forces',route:'military'},{label:'Diplomacy',route:'diplomacy'});
  const html=render(data);assert.doesNotMatch(html,/javascript:|data-gov-route="external"/);
  for(const route of ['research','military','diplomacy'])assert.match(html,new RegExp(`data-gov-route="${route}"`));
});
test('foreign governments retain inspection with only return-to-mine navigation',()=>{
  const html=render(fixture({mine:false}));assert.match(html,/Foreign government · view only/);assert.match(html,/Return to your government/);
  assert.ok(buttons(html,'data-gov-route').every(button=>button.includes('data-gov-route="mine"')));
  assert.ok(buttons(html,'data-gov-review').every(button=>/\bdisabled\b/.test(button)));
});
test('tabs have one selected keyboard stop and a matching active panel',()=>{
  for(const tab of ['overview','decisions','politics','watch']){
    const html=render(fixture(),{tab}),tabs=buttons(html,'role="tab"');assert.equal(tabs.length,4);
    assert.equal(tabs.filter(b=>b.includes('aria-selected="true"')).length,1);assert.equal(tabs.filter(b=>b.includes('tabindex="0"')).length,1);
    assert.match(html,new RegExp(`id="gov-tab-${tab}" role="tab"[^>]*aria-selected="true" tabindex="0"`));
    assert.match(html,new RegExp(`id="gov-panel-${tab}" role="tabpanel" aria-labelledby="gov-tab-${tab}"`));
  }
});
test('unrecognized tab state safely defaults to overview',()=>{assert.match(render(fixture(),{tab:'<script>'}),/data-gov-section="overview"/);});
test('default available filter retains original action IDs and discloses refusal reasons',()=>{
  const html=render(fixture(),{tab:'decisions'});
  assert.deepEqual(buttons(html,'data-gov-review').map(b=>b.match(/data-gov-review="(\d+)"/)[1]),['1','2']);
  assert.match(html,/2 of 4 decisions shown/);assert.match(html,/data-gov-detail="unavailable"/);assert.match(html,/The government is too young/);assert.match(html,/current rules/);
  assert.match(html,/data-gov-filter="available" aria-pressed="true"/);
});
test('all decisions includes disabled review controls with the original index and reason',()=>{
  const html=render(fixture(),{tab:'decisions',actionFilter:'all'}),controls=buttons(html,'data-gov-review');assert.equal(controls.length,4);
  assert.match(controls[0],/data-gov-review="0" disabled title="The government is too young/);assert.match(controls[3],/data-gov-review="3" disabled/);assert.match(html,/4 of 4 decisions shown/);
});
test('decision search does not reindex commands and can find refusal conditions',()=>{
  const html=render(fixture(),{tab:'decisions',query:'Small partner'});assert.equal(buttons(html,'data-gov-review').length,1);assert.match(html,/data-gov-review="2"/);
  const refused=render(fixture(),{tab:'decisions',query:'current rules'});assert.match(refused,/data-gov-decision="3"/);assert.match(refused,/No matching decisions/);
});
test('missing commands or prices remain unavailable instead of becoming zero-cost orders',()=>{
  const data=fixture();delete data.actions[1].price;delete data.actions[2].command;
  const html=render(data,{tab:'decisions',actionFilter:'all'});assert.ok(buttons(html,'data-gov-review').every(b=>/\bdisabled\b/.test(b)));assert.match(html,/Price unavailable/);
});
test('busy state disables all decision reviews without hiding their effects',()=>{
  const html=render(fixture(),{tab:'decisions',busy:true});assert.ok(buttons(html,'data-gov-review').every(b=>/\bdisabled\b/.test(b)));assert.match(html,/Loyalty responds to policy afterward/);
});
test('foreign decision inspection never offers an enabled order',()=>{
  const html=render(fixture({mine:false}),{tab:'decisions',actionFilter:'all'});assert.equal(buttons(html,'data-gov-review').length,4);assert.ok(buttons(html,'data-gov-review').every(b=>/\bdisabled\b/.test(b)));assert.match(html,/Only your own government can take this decision/);
});
test('party tables distinguish public support from parliamentary seat shares',()=>{
  const html=render(fixture(),{tab:'politics'});assert.match(html,/<th scope="col">Support<\/th><th scope="col">Seat share/);assert.match(html,/<td>43\.8%<\/td><td>51\.4%<\/td>/);assert.match(html,/Solidarność/);assert.match(html,/Leads government/);assert.match(html,/Pariah status · banned/);
  assert.match(html,/1\.2 PC \/ month/);assert.match(html,/1993-10/);assert.match(html,/64\.3% movement share/);
});
test('party search supports native names without changing political totals',()=>{
  const html=render(fixture(),{tab:'politics',query:'solidarność'});assert.match(html,/Solidarity/);assert.doesNotMatch(html,/Social Democrats/);assert.match(html,/64\.3% movement share/);
});
test('foreign backing keeps domestic support distinct and protects unexposed sponsors',()=>{
  const html=render(fixture(),{tab:'politics'});assert.match(html,/Foreign backing · separate from domestic support/);assert.match(html,/Western · 8%/);assert.match(html,/sponsor undisclosed · 3%/);assert.match(html,/Visible sponsor · exposed support · 2%/);assert.match(html,/Patronage · 3%/);assert.doesNotMatch(html,/SECRET SPONSOR/);
});
test('regimes show actual institutions, loyalty and dormant party records',()=>{
  const html=render(fixture({electoral:false,system:'Institutional regime',ruling_institution:'State council'}),{tab:'politics'});
  assert.match(html,/Regime &amp; institutions/);assert.match(html,/State council/);assert.match(html,/Armed forces/);assert.match(html,/61\.2% loyal/);assert.match(html,/Dormant party table/);assert.doesNotMatch(html,/<dt>Next election<\/dt>/);
});
test('ideology-off electoral campaigns keep parliament and cabinet mechanics visible',()=>{
  const data=fixture({on:false,bar:[],takeover:null}),html=render(data,{tab:'politics'});
  assert.match(html,/Parties in the political system/);assert.match(html,/Solidarity/);assert.match(html,/51\.4%/);assert.match(html,/1\.2 PC \/ month/);assert.match(html,/Ordinary government institutions/);
  assert.doesNotMatch(html,/gov-ui-bloc-bar|movement share|foreign-backing|does not provide a parliament/);
});
test('ideology-off regimes retain ordinary institutional loyalty and coup pressure',()=>{
  const html=render(fixture({on:false,electoral:false,bar:[],takeover:null}),{tab:'politics'});assert.match(html,/Armed forces/);assert.match(html,/61\.2% loyal/);assert.match(html,/<dt>Coup pressure<\/dt><dd>6\.5<\/dd>/);
});
test('ideology-off overview preserves supplied leadership and ordinary fallback metrics',()=>{
  const data=fixture({on:false,briefing:null}),html=render(data);assert.match(html,/Tadeusz Mazowiecki/);assert.match(html,/31\.5 PC/);assert.match(html,/Government seat share/);assert.doesNotMatch(html,/<dt>Discontent<\/dt>/);
});
test('ideology-off watch explains its scope without declaring ordinary government disabled',()=>{
  const html=render(fixture({on:false}),{tab:'watch'});assert.match(html,/ideological takeover watch is inactive/);assert.match(html,/Ordinary elections, cabinet failures and institutional regime coups remain active/);assert.doesNotMatch(html,/data-gov-detail="watch:/);
});
test('watch preserves served route status, gauges and progress without inferring chance',()=>{
  const html=render(fixture(),{tab:'watch'});assert.match(html,/Route closed/);assert.match(html,/closed for calibration/);assert.match(html,/91% · at least 70%/);assert.match(html,/Condition not met/);assert.match(html,/width:20%;/);assert.match(html,/27% · between 20% and 60%/);assert.match(html,/6 · at least 5/);assert.match(html,/All reported conditions met/);
  assert.match(html,/not probabilities|not a probability|not predict/);assert.match(html,/data-gov-detail="watch:coup"/);
});
test('missing watch routes report missing readings without inventing conditions',()=>{
  const html=render(fixture({takeover:null}),{tab:'watch'});assert.match(html,/No conditions are available in this reading/);assert.doesNotMatch(html,/undefined|NaN|null/);
});
test('valid review uses actual cost, exact before/after strings and one confirmation',()=>{
  const html=reviewHtml(fixture()),controls=confirm(html);assert.equal(controls.length,1);assert.match(controls[0],/data-gov-confirm="2"/);assert.doesNotMatch(controls[0],/\bdisabled\b/);
  assert.match(html,/<dt>Political capital cost<\/dt><dd>3 PC/);assert.match(html,/<dt>Financial cost<\/dt><dd>\$0/);assert.match(html,/<td>51\.4%<\/td><td>43\.9%<\/td>/);assert.match(html,/Only 3 PC is deducted/);assert.match(html,/id="govReviewTitle" tabindex="-1"/);
});
test('financial preview displays authoritative money units including small actual payments',()=>{
  for(const [amount,label] of [[.008,'$8m'],[1.25,'$1.25bn'],[.0000025,'$2.5k'],[.000000007,'$7'],[1250,'$1.25tn']]){
    const html=reviewHtml(fixture(),{data:quote({money_cost_bn:amount})});assert.ok(html.includes(`<dd>${label}</dd>`),label);
  }
});
test('refused review shows the live reason and cannot confirm',()=>{
  const html=reviewHtml(fixture(),{data:quote({valid:false,reason:'The political state has changed.'})});assert.match(html,/The political state has changed/);assert.match(confirm(html)[0],/\bdisabled\b/);
});
test('loading reviews hide stale effects and disable other decision reviews',()=>{
  const html=reviewHtml(fixture(),{loading:true,data:quote({description:'STALE DESCRIPTION',changes:[{label:'STALE CHANGE',before:0,after:1}],effects:['STALE EFFECT'],warnings:['STALE WARNING']})});
  assert.doesNotMatch(html,/STALE|Financial cost/);assert.match(html,/Checking the current cost/);assert.match(confirm(html)[0],/\bdisabled\b/);assert.ok(buttons(html,'data-gov-review').every(b=>/\bdisabled\b/.test(b)));
});
test('failed reviews hide stale effects and offer a separately identified retry',()=>{
  const html=reviewHtml(fixture(),{error:'Network failed',data:quote({effects:['STALE EFFECT'],description:'STALE DESCRIPTION'})});assert.match(html,/role="alert"/);assert.match(html,/data-gov-retry/);assert.doesNotMatch(html,/STALE|Financial cost/);assert.match(confirm(html)[0],/\bdisabled\b/);
});
test('busy, missing and mismatched-index review states cannot confirm',()=>{
  for(const [extra,state] of [[{}, {busy:true}],[{index:999},{}],[{index:'2'},{}],[{data:null},{}],[{data:quote({price_pc:null})},{}],[{data:quote({command:null})},{}],[{index:0},{}]])assert.match(confirm(reviewHtml(fixture(),extra,state))[0],/\bdisabled\b/);
  assert.match(confirm(reviewHtml(fixture({mine:false})))[0],/\bdisabled\b/);
});
test('missing statistics are unknown, while explicit zero remains zero',()=>{
  const data=fixture();data.briefing.stats=[{label:'Unknown',value:null},{label:'Known zero',value:0}];const html=render(data);assert.match(html,/<dt>Unknown<\/dt><dd>—/);assert.match(html,/<dt>Known zero<\/dt><dd>0/);assert.doesNotMatch(html,/NaN|undefined/);
});
test('labels, search, errors and before/after values are HTML escaped',()=>{
  const attack='<img src=x onerror="boom">',data=fixture({nation_name:attack,leader:{name:attack}});data.actions[2].label=attack;data.briefing.attention[0].title=attack;
  const html=reviewHtml(data,{data:quote({description:attack,changes:[{label:attack,before:attack,after:attack}],warnings:[attack]})},{query:attack,notice:attack});
  assert.doesNotMatch(html,/<img src=x|onerror="boom"|<script/);assert.match(html,/&lt;img src=x onerror=&quot;boom&quot;&gt;/);assert.match(html,/value="&lt;img/);
  assert.doesNotMatch(render({error:attack}),/<img src=x/);
});
test('untrusted styling metadata cannot become inline CSS or event attributes',()=>{
  const data=fixture();data.bar[0].bloc='x" onclick="boom';data.groups[0].bloc='url(evil)';data.briefing.attention[0].tone='x" onclick="boom';
  const html=render(data,{tab:'politics'})+render(data);assert.doesNotMatch(html,/onclick=|url\(evil\)/);assert.match(html,/Unspecified alignment/);
});
test('loading and read failure offer honest states without decisions',()=>{
  assert.match(render(null),/Preparing your government briefing/);const html=render({error:'Read failed'});assert.match(html,/data-gov-refresh/);assert.match(html,/role="alert"/);assert.doesNotMatch(html,/data-gov-confirm|data-gov-review=/);
});
test('search and disclosures expose stable hooks for host focus and open-state restoration',()=>{
  const decisions=render(fixture(),{tab:'decisions'}),politics=render(fixture(),{tab:'politics'});
  assert.match(decisions,/<label[^>]*>[^]*?<span>Find a decision<\/span><input type="search" data-gov-search/);assert.match(decisions,/data-gov-detail="unavailable"/);assert.match(politics,/data-gov-detail="foreign-backing"/);assert.match(politics,/role="region"[^>]*tabindex="0"/);
});
