const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const base=path.resolve(__dirname,'../..');
const source=fs.readFileSync(path.join(base,'spheres-web/ui/fiscal-recovery-ui.js'),'utf8');
const index=fs.readFileSync(path.join(base,'spheres-web/ui/index.html'),'utf8');
const server=fs.readFileSync(path.join(base,'spheres-web/src/main.rs'),'utf8');
const css=fs.readFileSync(path.join(base,'spheres-web/ui/fiscal-recovery-ui.css'),'utf8');
const plain=value=>JSON.parse(JSON.stringify(value));
function reading(extra={}){return {debt_gdp:1.234,annual_debt_change_gdp:-.015,primary_balance_gdp:.027,
  required_primary_balance_gdp:.038,adjustment_needed_gdp:.011,current_policy_adjustment_gdp:.004,interest_revenue:.124,
  projected_debt_gdp_5y:1.115,annual_real_growth:.023,months_observed:4,grace_months_remaining:8,
  stress_months:3,improving_months:2,recovery_required:true,status:'recovering',status_label:'Recovery taking hold',
  reason:'Debt is falling but payments are still stretched.',next_action:'Protect essential services and review existing commitments.',
  stability_change_per_month:-.023,forecast_assumptions:'The model holds enacted policy steady.',...extra};}
function fixture(){
  const calls=[],nodes={};
  const c=vm.createContext({console,S:{session_id:'one',player:'USA',fiscal_recovery_enabled:true,fiscal_recovery:reading(),
    stratagems:{offers:[{id:'debt_restructuring',name:'Restructure the Debt',cost:30,affordable:false}]},
    policy:{money:{debt_gdp:999,interest_revenue:999}}},
    document:{querySelector:s=>nodes[s]||null,querySelectorAll:()=>[]},
    advancing:false,pendingAdvance:null,COMMAND_CHANNEL:{busy:false,pending:null},SESSION:{busy:false},CAB:{busy:false},
    renderLeft:()=>calls.push(['render']),me:()=>({}),openConstructionCabinet:tab=>calls.push(['tab',tab]),
    cashFlowNavigate:action=>{calls.push(['navigate',plain(action)]);return true;},
    api:async(url,body)=>{calls.push(['api',url,plain(body)]);return {...c.S,fiscal_recovery_enabled:true,fiscal_recovery:reading()};},
    adopt:async response=>{calls.push(['adopt']);c.S=response;},
  });
  vm.runInContext(source,c);c.ui=vm.runInContext('FISCAL_UI',c);Object.assign(c,{calls,nodes});return c;
}
test('fiscal card preserves simulation assessment, units, actual observation period and served Cabinet prices',()=>{
  const c=fixture(),before=plain(c.S),html=c.fiscalRecoveryHtml();
  for(const text of ['Recovery taking hold','123.4%','-1.5 pp','12.4%','2.7%','111.5%','+1.1 pp','8 months','3 months','2 months','-0.023 points / month','4 observed months','Annualized change','30 PC, more political capital needed','The model holds enacted policy steady.'])assert(html.includes(text),text);
  assert.doesNotMatch(html,/999/);assert.deepEqual(plain(c.S),before);
  assert.match(html,/Unused construction authority is not cash spending/);
  assert.match(html,/A plan alone does not clear pressure/);
  assert.match(html,/Observed adjustment gap<\/dt><dd>\+1.1 pp/);
  assert.match(html,/Gap with current tax policy<\/dt><dd>\+0.4 pp/);
  assert.match(html,/policy changes take time to appear/);
});
test('missing observations and zero revenue are explicit without fabricated fallback figures',()=>{
  const c=fixture();c.S.fiscal_recovery=reading({months_observed:0,interest_revenue:null,projected_debt_gdp_5y:null});
  const html=c.fiscalRecoveryHtml();assert.match(html,/Provisional: no complete monthly receipt yet/);assert.match(html,/No positive revenue/);
  assert.match(html,/Debt \/ GDP in five years<\/dt><dd>—/);assert.doesNotMatch(html,/999|NaN|Infinity/);
});
test('first report never presents a provisional zero-debt surplus as an observed debt decline',()=>{
  const c=fixture();c.S.fiscal_recovery=reading({months_observed:0,debt_gdp:0,annual_debt_change_gdp:-.17});
  const html=c.fiscalRecoveryHtml();
  assert.match(html,/Debt trend<\/dt><dd>Awaiting first report/);
  assert.match(html,/Provisional adjustment gap/);
  assert.doesNotMatch(html,/-17.0 pp|Observed adjustment gap/);
  c.S.fiscal_recovery.months_observed=1;c.S.fiscal_recovery.annual_debt_change_gdp=0;
  const reported=c.fiscalRecoveryHtml();
  assert.match(reported,/Debt trend<\/dt><dd>0.0 pp/);
  assert.match(reported,/Observed adjustment gap/);
  assert.doesNotMatch(reported,/Awaiting first report|Provisional adjustment gap/);
});
test('legacy campaign offers a permanent explicit upgrade and never submits on rendering',()=>{
  const c=fixture();c.S.fiscal_recovery_enabled=false;c.S.fiscal_recovery=null;
  const html=c.fiscalRecoveryHtml();assert.match(html,/permanently changes the economic rules/);assert.match(html,/begin paying interest/);
  assert.match(html,/data-fiscal-enable/);assert.match(html,/grants no cash or debt relief/);assert.equal(c.calls.length,0);
  assert.match(c.fiscalRecoveryHtml(true),/Fiscal recovery is available/);
});
test('upgrade uses the ordinary command channel once and adopts the result',async()=>{
  const c=fixture();c.S.fiscal_recovery_enabled=false;c.S.fiscal_recovery=null;c.fiscalRecoveryHtml();
  assert.equal(await c.fiscalRecoveryEnable(),true);
  assert.deepEqual(c.calls.filter(a=>a[0]==='api'),[['api','/api/command',{commands:[{kind:'enable_fiscal_recovery'}]}]]);
  assert.equal(c.S.fiscal_recovery_enabled,true);assert.equal(await c.fiscalRecoveryEnable(),false);
});
test('upgrade blocks duplicate and stale commands and does not adopt another campaign response',async()=>{
  const c=fixture();c.S.fiscal_recovery_enabled=false;c.fiscalRecoveryHtml();const state=c.S;
  c.COMMAND_CHANNEL.pending={};assert.equal(await c.fiscalRecoveryEnable(),false);c.COMMAND_CHANNEL.pending=null;
  assert.equal(await c.fiscalRecoveryEnable({...state}),false);
  let resolve;c.api=()=>new Promise(r=>{resolve=r;});const first=c.fiscalRecoveryEnable();
  assert.equal(await c.fiscalRecoveryEnable(),false);c.S={...state,session_id:'two'};resolve({...state,fiscal_recovery_enabled:true});
  assert.equal(await first,false);assert.equal(c.S.session_id,'two');assert.equal(c.S.fiscal_recovery_enabled,false);
});
test('upgrade refusal is visible and does not claim the rules changed',async()=>{
  const c=fixture();c.S.fiscal_recovery_enabled=false;c.fiscalRecoveryHtml();c.api=async()=>({...c.S,errors:['Finish the current month first.']});
  assert.equal(await c.fiscalRecoveryEnable(),false);assert.equal(c.ui.busy,false);assert.equal(c.S.fiscal_recovery_enabled,false);
  assert.match(c.fiscalRecoveryHtml(),/role="alert">Finish the current month first/);
});
test('recovery actions navigate to existing budget, tax and Cabinet controls without submitting a policy',()=>{
  const c=fixture();const panel={open:false,querySelector:()=>({focus:()=>c.calls.push(['focus'])}),scrollIntoView:()=>c.calls.push(['scroll'])};c.nodes['#cabinetPlays']=panel;
  assert.equal(c.fiscalRecoveryNavigate('tax'),true);assert.equal(c.fiscalRecoveryNavigate('budget'),true);assert.equal(c.fiscalRecoveryNavigate('plays'),true);
  assert.deepEqual(c.calls.filter(a=>a[0]==='navigate'),[['navigate',{action:'policy',control:'tax'}],['navigate',{action:'budget'}]]);
  assert.equal(panel.open,true);assert(!c.calls.some(a=>a[0]==='api'));
  const state=c.S;c.S={...state};assert.equal(c.fiscalRecoveryNavigate('tax',state),false);
  c.pendingAdvance={};assert.equal(c.fiscalRecoveryNavigate('tax'),false);
});
test('completed Cabinet enact unlocks fiscal navigation rendered during the busy advance without another order',async()=>{
  const c=fixture(),elements={},buttons=[{dataset:{fiscalAction:'tax'},disabled:false}];
  const element=id=>elements[id]||(elements[id]={innerHTML:'',textContent:'',setAttribute(){},focus(){}});
  c.$=element;c.document.querySelectorAll=selector=>selector==='[data-fiscal-action]'?buttons:[];
  const nation={annual_budget:{fiscal_year:2035,due:false},political_capital:80};
  c.me=()=>nation;c.queued=[];c.S.year=2035;c.S.date='1 Jan 2035';
  c.annualBudgetOf=()=>({kind:'annual_budget',fiscal_year:2035});c.annualPoliticalCost=()=>0;
  c.cabinetBudgetSummary=()=>'';c.escText=String;c.revertOrders=()=>{};c.cabinetIsOpen=()=>true;
  for(const declaration of ['function paintCabinetDraft(m) {','async function cabinetEnact() {']){
    const start=index.indexOf(declaration),end=index.indexOf('\n}',start)+2;assert(start>=0);vm.runInContext(index.slice(start,end),c);
  }
  c.fiscalRecoveryBind();assert.equal(buttons[0].disabled,false);
  let finish,advances=0;
  c.advance=()=>{
    advances++;c.S={...c.S,date:'2 Jan 2035',log:[]};
    assert.equal(c.CAB.busy,true);
    assert.match(c.fiscalRecoveryHtml(),/data-fiscal-action="tax" disabled/);
    buttons[0]={dataset:{fiscalAction:'tax'},disabled:true};c.fiscalRecoveryBind();
    return new Promise(resolve=>{finish=resolve;});
  };
  const running=c.cabinetEnact();
  assert.equal(buttons[0].disabled,true);assert.equal(buttons[0].onclick(),false);
  await c.cabinetEnact();assert.equal(advances,1,'a second click cannot queue another turn');
  finish(true);await running;
  assert.equal(c.CAB.busy,false);assert.equal(buttons[0].disabled,false,'completion must refresh disabled fiscal controls without rebuilding the card');
  assert.match(element('#cabinetLive').textContent,/calendar advanced one day/);
  assert.equal(buttons[0].onclick(),true);assert.deepEqual(c.calls.filter(a=>a[0]==='navigate'),[['navigate',{action:'policy',control:'tax'}]]);
  assert.equal(advances,1);assert(!c.calls.some(a=>a[0]==='api'),'navigation must not submit a policy or replay the completed command');
});
test('fiscal control refresh retains pending and stale-state locks and binds upgrade only once',async()=>{
  const c=fixture(),action={dataset:{fiscalAction:'tax'}},enable={};
  c.S.fiscal_recovery_enabled=false;c.document.querySelectorAll=selector=>selector==='[data-fiscal-action]'?[action]:[];
  c.nodes['[data-fiscal-enable]']=enable;c.fiscalRecoveryHtml();
  c.fiscalRecoveryBind();c.fiscalRecoveryBind();assert.equal(action.disabled,false);assert.equal(enable.disabled,false);
  c.pendingAdvance={};c.fiscalRecoverySync();assert.equal(action.disabled,true);assert.equal(enable.disabled,true);
  c.pendingAdvance=null;c.fiscalRecoverySync();assert.equal(action.disabled,false);assert.equal(enable.disabled,false);
  c.S={...c.S};c.fiscalRecoverySync();assert.equal(action.disabled,true);assert.equal(enable.disabled,true);
  c.fiscalRecoveryBind();assert.equal(action.disabled,false);assert.equal(enable.disabled,false);
  assert.equal(await enable.onclick(),true);assert.equal(c.calls.filter(a=>a[0]==='api').length,1);
});
test('assessment text is escaped and unknown status cannot create arbitrary markup',()=>{
  const c=fixture();c.S.fiscal_recovery=reading({status:'"><img>',status_label:'<b>bad</b>',reason:'<script>x</script>',next_action:'<img>',forecast_assumptions:'<svg>'});
  const html=c.fiscalRecoveryHtml();assert.doesNotMatch(html,/<img|<script|<svg|<b>bad/);assert.match(html,/&lt;b&gt;bad/);assert.match(html,/fr-watch/);
});
test('budget forecast displays only the server full-use scenario and keeps its cash-saving limitation',()=>{
  const start=index.indexOf('function cabinetBudgetSummary(m) {'),end=index.indexOf('\n}',start)+2;
  const c=fixture();c.S.programs={};c.PG={key:'draft',preview:{fiscal:{revenue_bn:10,total_at_full_use_bn:15,interest_bn:2,balance_at_full_use_bn:-5,
    projected_debt_gdp_5y_at_full_use:1.387}}};c.programDraftKey=()=> 'draft';
  c.fmt={money:v=>`$${v}bn`,pct:v=>`${(v*100).toFixed(1)}%`};vm.runInContext(index.slice(start,end),c);
  const html=c.cabinetBudgetSummary({});assert.match(html,/Five-year debt \/ GDP at full use: <strong>138.7%/);
  assert.match(html,/with all authority spent/);assert.match(html,/Reducing unused authority is not a cash saving/);assert.doesNotMatch(html,/111.5%/);
  c.PG.preview.fiscal.projected_debt_gdp_5y_at_full_use=null;assert.doesNotMatch(c.cabinetBudgetSummary({}),/Five-year debt/);
  c.PG.key='stale';assert.doesNotMatch(c.cabinetBudgetSummary({}),/138.7|Five-year debt/);
});
test('active state and simulation stability source are connected to the shipped responsive UI',()=>{
  assert.match(index,/fiscalRecoveryHtml\(true\)/);assert.match(index,/fiscalRecoveryHtml\(\)/);assert.match(index,/fiscalRecoveryBind\(\)/);
  assert.match(index,/src="\/fiscal-recovery-ui.js"/);assert.match(index,/href="\/fiscal-recovery-ui.css"/);
  assert.match(server,/"fiscal_recovery": w.player.and_then\(\|p\| spheres_sim::fiscal_recovery::assessment\(w, p\)\)/);
  assert.match(server,/"fiscal_confidence", "Fiscal confidence", -t.fiscal_confidence_drag/);
  assert.match(css,/@media\(max-width:760px\)/);assert.match(css,/grid-template-columns:repeat\(2,minmax\(0,1fr\)\)/);
});
