/* A read-only view of dated government cash receipts and the enacted plan.
   All totals and affordability judgments come from the server. */
const CFLOW={open:false,data:null,state:null,session:null,nation:null,seq:0,loading:false,stale:true,error:"",detailOpen:false,focus:null,scroll:0};
const CASH_FLOW_NAV=new Set(["budget","policy","construction","industry","resources","trade"]);
function cashFlowText(value) {
  return String(value??"").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#39;");
}
function cashFlowMoney(value,signed=false) {
  return Number.isFinite(value)?`${value<0?"-":signed&&value>0?"+":""}${economyMoney(Math.abs(value))}`:"—";
}
function cashFlowContextActive() {
  return typeof S!=="undefined"&&!!S?.player&&typeof CAB!=="undefined"&&CAB.tab==="overview"
    && typeof cabinetIsOpen==="function"&&cabinetIsOpen();
}
function cashFlowActive() {return CFLOW.open&&cashFlowContextActive();}
function cashFlowOrdersPending() {
  return (typeof advancing!=="undefined"&&advancing)||(typeof pendingAdvance!=="undefined"&&!!pendingAdvance)
    ||(typeof COMMAND_CHANNEL!=="undefined"&&!!(COMMAND_CHANNEL.busy||COMMAND_CHANNEL.pending))
    ||(typeof SESSION!=="undefined"&&SESSION.busy)||(typeof CAB!=="undefined"&&CAB.busy)
    ||(typeof PROD!=="undefined"&&PROD.busy)||(typeof COMP!=="undefined"&&!!(COMP.busy||COMP.pending));
}
function cashFlowCurrent(data=CFLOW.data,state=CFLOW.state) {
  return cashFlowActive()&&!!data&&data===CFLOW.data&&state===CFLOW.state&&state===S
    &&!CFLOW.loading&&!CFLOW.stale&&!CFLOW.error&&!cashFlowOrdersPending();
}
function cashFlowAction(value) {
  const action=typeof value==="string"?{action:value}:value;
  return action&&CASH_FLOW_NAV.has(action.action)?action:null;
}
function cashFlowActionLabel(action) {
  if(action.action==="policy"&&action.control==="rate")return action.label||"Review debt and rates";
  if(action.action==="policy"&&action.control==="tax")return action.label||"Review tax policy";
  return action.label||{budget:"Review annual budget",policy:"Review tax and rates",construction:"Review construction funding",industry:"Review industry operations",resources:"Find raw inputs",trade:"Review trade"}[action.action];
}
function cashFlowButton(value,label) {
  const action=cashFlowAction(value);if(!action)return "";
  return `<button type="button" data-cash-flow-action="${cashFlowText(JSON.stringify(action))}" ${cashFlowCurrent()&&action.enabled!==false&&action.available!==false?"":"disabled"}>${cashFlowText(label||cashFlowActionLabel(action))}</button>`;
}
function cashFlowMetric(label,value,note="",signed=false) {
  return `<div><dt>${cashFlowText(label)}</dt><dd>${cashFlowText(cashFlowMoney(value,signed))}</dd>${note?`<small>${cashFlowText(note)}</small>`:""}</div>`;
}
function cashFlowRow(label,value,note="") {
  return `<div class="cf-flow-row"><dt>${cashFlowText(label)}</dt><dd>${cashFlowText(cashFlowMoney(value))}</dd>${note?`<small>${cashFlowText(note)}</small>`:""}</div>`;
}
function cashFlowSettlementHtml(data) {
  const settled=data.settled;
  if(!settled)return `<section class="cf-settlement" aria-labelledby="cashFlowSettlementTitle"><p class="cf-kicker">Latest daily government budget</p><h2 id="cashFlowSettlementTitle">No settled daily receipt yet</h2><p class="cf-note">Daily government revenue, spending and balance will appear after the first budget settlement. The annual figures below describe the current budget plan.</p></section>`;
  return `<section class="cf-settlement" aria-labelledby="cashFlowSettlementTitle"><div class="cf-section-heading"><div><p class="cf-kicker">Latest daily government budget</p><h2 id="cashFlowSettlementTitle">Revenue, spending and balance</h2></div><span class="cf-date">${cashFlowText(settled.label||"Latest daily receipt")}</span></div>
    <dl class="cf-flow-totals">${cashFlowMetric("Revenue received",settled.revenue_bn,"Recorded for this day")}${cashFlowMetric("Budget outflow",settled.total_outflow_bn,"Ministry spending and interest")}${cashFlowMetric("Budget balance",settled.balance_bn,"Positive: surplus · Negative: deficit",true)}</dl>
    <div class="cf-flow-columns"><div class="cf-flow-column"><h3>Government spending</h3><dl class="cf-flow-rows">${cashFlowRow("Services",settled.services_bn)}${cashFlowRow("Industry and research operations",settled.plant_operating_bn)}${cashFlowRow("Construction work",settled.construction_bn)}${cashFlowRow("Other capital spending",settled.other_capital_bn)}${cashFlowRow("Ministry spending total",settled.ministry_spend_bn,"These components are included in budget outflow.")}</dl></div>
    <div class="cf-flow-column"><h3>Interest and earlier payments</h3><dl class="cf-flow-rows">${cashFlowRow("Interest paid",settled.interest_bn)}${cashFlowRow("Previously paid funds used",settled.prepaid_used_bn,"Paid earlier; this usage is not another cash outflow.")}</dl></div></div>
    <p class="cf-note">This budget result covers government revenue, ministry spending and interest. Trade, transfers and other transactions can also change cash and debt. A budget surplus repays debt before building treasury cash.</p></section>`;
}
function cashFlowPrioritiesHtml(data) {
  const priorities=(Array.isArray(data.priorities)?data.priorities:[]).filter(priority=>priority&&typeof priority.title==="string").slice(0,3);
  if(!priorities.length)return "";
  return `<section class="cf-priorities" aria-labelledby="cashFlowPrioritiesTitle"><h2 id="cashFlowPrioritiesTitle">Financial priorities</h2><div class="cf-priority-grid">${priorities.map((priority,index)=>{
    const ministry=priority.action?.action==="budget"&&priority.action.ministry?(data.ministries||[]).find(row=>row.key===priority.action.ministry):null;
    const label=ministry?`Review ${ministry.name||ministry.key}`:null;
    return `<article class="cf-priority ${priority.level==="attention"?"cf-priority-attention":""}" aria-labelledby="cashFlowPriority${index}"><h3 id="cashFlowPriority${index}">${cashFlowText(priority.title)}</h3><p>${cashFlowText(priority.detail)}</p><dl>${(Array.isArray(priority.metrics)?priority.metrics:[]).map(metric=>cashFlowMetric(metric.label,metric.amount_bn,metric.period||"Period unavailable")).join("")}</dl><div class="cf-actions">${cashFlowButton(priority.action,label)}</div></article>`;
  }).join("")}</div></section>`;
}
function cashFlowConstructionHtml(data) {
  const budget=data.construction;if(!budget)return "";
  const affordability=budget.affordability||{};
  return `<section class="cf-construction" aria-labelledby="cashFlowConstructionTitle"><div class="cf-section-heading"><div><p class="cf-kicker">From budget to work</p><h2 id="cashFlowConstructionTitle">Construction funding</h2></div>${affordability.title?`<span class="cf-funding-state">${cashFlowText(affordability.title)}</span>`:""}</div>
    ${affordability.detail?`<p>${cashFlowText(affordability.detail)}</p>`:""}<dl class="cf-construction-metrics">${cashFlowMetric("Daily spending limit",budget.daily_budget_bn,"Maximum per day")}${cashFlowMetric("Planned construction",budget.planned_daily_bn,"Per day at the current setting")}${cashFlowMetric("Available for work",budget.available_bn,"Available capital authorization")}</dl>
    <p class="cf-note">Available capital funding: ${cashFlowText(cashFlowMoney(budget.authority_bn))}. This is budget authorization, not treasury cash. Unused daily construction funding is not charged.</p>
    ${budget.reason?`<p class="cf-note">${cashFlowText(budget.reason)}</p>`:""}<div class="cf-actions">${cashFlowButton("construction","Review construction funding")}${cashFlowButton("budget","Review capital allocations")}${cashFlowButton("industry","Review operating costs")}</div></section>`;
}
function cashFlowAnnualHtml(data) {
  const annual=data.annual||{},ministries=Array.isArray(data.ministries)?data.ministries:[];
  return `<details class="cf-budget" id="cashFlowBudgetDetails" ${CFLOW.detailOpen?"open":""}><summary data-cash-flow-focus="budget">${annual.renewed?"Enacted annual plan":"Annual plan"} & ministry spending</summary><div class="cf-budget-body"><p>${cashFlowText(annual.fiscal_year!=null?`Fiscal year ${annual.fiscal_year}`:"Fiscal year unavailable")} · Plan at full use, not a settled daily bill.</p>
    <dl class="cf-budget-plan">${cashFlowMetric("Planned annual revenue",annual.revenue_bn)}${cashFlowMetric("Authorized ministry spending",annual.authorized_spend_bn)}${cashFlowMetric("Annual interest estimate",annual.interest_bn)}${cashFlowMetric("Total annual cost at full use",annual.total_at_full_use_bn)}${cashFlowMetric("Annual balance at full use",annual.balance_at_full_use_bn,"Estimate at full budget use",true)}${cashFlowMetric("Posted spending run rate",annual.posted_spending_run_rate_bn,"Annualized posted spending; not a second charge")}</dl>
    <dl class="cf-flow-rows">${Number.isFinite(annual.tax_revenue_bn)?cashFlowRow("Annual tax income estimate",annual.tax_revenue_bn):""}${Number.isFinite(annual.resource_revenue_bn)?cashFlowRow("Oil-related fiscal revenue",annual.resource_revenue_bn):""}</dl>
    <p class="cf-note">Annual revenue estimates use current economic conditions and tax policy. Oil-related fiscal revenue is not gross exports or sales income. The daily receipt above reports total revenue without an income split. The spending run rate annualizes one settled day.</p>
    <div class="cf-table-wrap"><table><caption class="cf-note">Available authorization can include previously paid equipment. Missing expenditure is shown as —.</caption><thead><tr><th scope="col">Ministry</th><th scope="col">Annual allocation</th><th scope="col">Daily authorization</th><th scope="col">Available authorization</th><th scope="col">Latest daily spend</th><th scope="col">Spent this year</th></tr></thead><tbody>${ministries.map(ministry=>`<tr><th scope="row"><span>${cashFlowText(ministry.name||ministry.key)}</span><div class="cf-ministry-action">${cashFlowButton({action:"budget",ministry:ministry.key},`Review ${ministry.name||ministry.key}`)}</div></th><td>${cashFlowText(cashFlowMoney(ministry.annual_bn))}</td><td>${cashFlowText(cashFlowMoney(ministry.daily_authorized_bn))}</td><td>${cashFlowText(cashFlowMoney(ministry.available_bn))}</td><td>${cashFlowText(cashFlowMoney(ministry.last_spent_bn))}</td><td>${cashFlowText(cashFlowMoney(ministry.spent_ytd_bn))}</td></tr>`).join("")||`<tr><td colspan="6">No ministry breakdown is available in this reading.</td></tr>`}</tbody></table></div>
    <div class="cf-actions">${cashFlowButton("budget","Open annual budget")}${cashFlowButton("policy","Review taxes and rates")}</div></div></details>`;
}
function cashFlowContentHtml() {
  const data=CFLOW.data;
  const heading=`<header class="cf-header"><div><p class="cf-kicker">Economy · Government finances</p><h1 id="cashFlowTitle" tabindex="-1" data-cash-flow-focus="title">Your cash flow</h1><p>${cashFlowText(data?`${data.name||"Government"} · Reading for ${data.date||"the current campaign"}`:"Reading the annual budget and latest cash receipt")}</p></div><button type="button" data-cash-flow-focus="refresh" data-cash-flow-refresh ${CFLOW.loading||cashFlowOrdersPending()?"disabled":""}>Refresh reading</button></header>`;
  const status=CFLOW.error?`<div class="cf-message error" role="alert"><strong>Cash flow could not be refreshed.</strong><p>${cashFlowText(CFLOW.error)}</p>${data?"<p>The previous dated reading remains below. Navigation is disabled until the reading is refreshed.</p>":""}<button type="button" data-cash-flow-focus="retry" data-cash-flow-retry ${cashFlowOrdersPending()?"disabled":""}>Retry cash flow</button></div>`
    :CFLOW.loading||CFLOW.stale?`<p class="cf-message" role="status">${data?"Updating the cash-flow reading…":"Loading government cash flow…"}</p>`
    :cashFlowOrdersPending()?`<p class="cf-message" role="status">Review the pending turn or order before opening another financial task.</p>`:"";
  const draft=typeof cashFlowDraftNotice==="function"?cashFlowDraftNotice():"";
  const draftHtml=draft?`<p class="cf-message cf-draft" role="status">${cashFlowText(draft)}</p>`:"";
  if(!data)return heading+status+draftHtml;
  const balances=data.balances||{};
  const priorities=(Array.isArray(data.priorities)?data.priorities:[]).filter(priority=>priority&&typeof priority.title==="string").slice(0,3);
  const alerts=(Array.isArray(data.alerts)?data.alerts:[]).filter(alert=>(alert.title!==data.construction?.affordability?.title||alert.detail!==data.construction?.affordability?.detail)&&!priorities.some(priority=>(alert.id&&priority.id===alert.id)||(priority.title===alert.title&&priority.detail===alert.detail))).map(alert=>`<div class="cf-message"><strong>${cashFlowText(alert.title)}</strong><p>${cashFlowText(alert.detail)}</p></div>`).join("");
  const extraActions=(Array.isArray(data.actions)?data.actions:[]).filter(action=>!["budget","construction","industry","policy"].includes(action.action));
  return `${heading}${status}${draftHtml}<dl class="cf-balances">${cashFlowMetric("Treasury cash",balances.treasury_bn,"Cash currently held")}${cashFlowMetric("Public debt",balances.debt_bn,"Outstanding debt")}${cashFlowMetric("Cash minus debt",balances.net_position_bn,"Net financial position",true)}</dl>
    ${cashFlowSettlementHtml(data)}${cashFlowPrioritiesHtml(data)}${alerts}${cashFlowConstructionHtml(data)}${cashFlowAnnualHtml(data)}
    ${extraActions.length?`<div class="cf-actions cf-next-actions">${extraActions.map(action=>cashFlowButton(action)).join("")}</div>`:""}`;
}
function cashFlowRememberView() {
  if(!CFLOW.open||typeof CAB==="undefined"||CAB.tab!=="overview")return;
  const root=document.querySelector("#cashFlowRoot");if(!root||root.dataset.cashFlowSession!==String(CFLOW.session??""))return;
  const scroller=document.querySelector("#left");if(scroller)CFLOW.scroll=scroller.scrollTop;
  const detail=root.querySelector("#cashFlowBudgetDetails");if(detail)CFLOW.detailOpen=detail.open;
  const focus=document.activeElement?.dataset?.cashFlowFocus;
  if(["refresh","retry","budget","title"].includes(focus))CFLOW.focus=focus;
}
function cashFlowPanelHtml() {
  cashFlowRememberView();
  return `<div id="cashFlowRoot" class="cash-flow" data-cash-flow-session="${cashFlowText(CFLOW.session)}" aria-busy="${CFLOW.loading}">${cashFlowContentHtml()}</div>`;
}
function cashFlowRender() {
  if(!cashFlowActive())return;
  const root=document.querySelector("#cashFlowRoot");if(!root)return;
  cashFlowRememberView();root.innerHTML=cashFlowContentHtml();root.dataset.cashFlowSession=String(CFLOW.session??"");root.setAttribute("aria-busy",String(CFLOW.loading));cashFlowBind(false);
}
function cashFlowNavigateCurrent(value,data=CFLOW.data,state=CFLOW.state) {
  const action=cashFlowAction(value);
  if(!cashFlowCurrent(data,state)||!action||action.enabled===false||action.available===false)return false;
  const clean={action:action.action};
  for(const key of ["ministry","department","control","kind","district","project","good"])if(typeof action[key]==="string"||Number.isFinite(action[key]))clean[key]=action[key];
  cashFlowNavigate(clean);return true;
}
function cashFlowBind(fetchIfNeeded=true) {
  if(!cashFlowContextActive())return;
  cashFlowResetCampaign();CFLOW.open=true;
  const root=document.querySelector("#cashFlowRoot");if(!root)return;
  const data=CFLOW.data,state=CFLOW.state;
  root.querySelectorAll("[data-cash-flow-action]").forEach(button=>{
    let action;try{action=JSON.parse(button.dataset.cashFlowAction);}catch{return;}
    button.onclick=()=>cashFlowNavigateCurrent(action,data,state);
  });
  for(const key of ["refresh","retry"]){const button=root.querySelector(`[data-cash-flow-${key}]`);if(button)button.onclick=()=>{if(!cashFlowOrdersPending()){CFLOW.focus=key;cashFlowFetch(true);}};}
  const detail=root.querySelector("#cashFlowBudgetDetails");if(detail)detail.ontoggle=()=>{CFLOW.detailOpen=detail.open;};
  const scroller=document.querySelector("#left");if(scroller)scroller.scrollTop=CFLOW.scroll;
  const focus=CFLOW.focus;
  if(focus){const control=root.querySelector(`[data-cash-flow-focus="${focus}"]`)||(focus==="retry"?root.querySelector("[data-cash-flow-refresh]"):null);if(control&&!control.disabled){control.focus({preventScroll:true});CFLOW.focus=null;}}
  if(fetchIfNeeded&&CFLOW.stale&&!CFLOW.loading&&!CFLOW.error)cashFlowFetch();
}
async function cashFlowFetch(force=false) {
  if(!cashFlowActive()||CFLOW.loading||(!force&&!CFLOW.stale))return false;
  cashFlowResetCampaign();
  const state=S,seq=++CFLOW.seq;CFLOW.loading=true;CFLOW.stale=true;CFLOW.error="";cashFlowRender();
  try{
    const data=await api(`/api/cash-flow?session_id=${encodeURIComponent(state.session_id)}`);
    if(seq!==CFLOW.seq||state!==S||!cashFlowActive())return false;
    if(!data||data.session_id!==state.session_id||data.nation!==state.player||!data.balances||!data.annual||!Array.isArray(data.ministries)||!Array.isArray(data.alerts))throw new Error("The cash-flow reading did not match this campaign. Retry to read the current finances.");
    CFLOW.data=data;CFLOW.state=state;CFLOW.stale=false;return true;
  }catch(error){if(seq===CFLOW.seq&&state===S&&cashFlowActive()){CFLOW.error=error.message||"The cash-flow service could not be reached.";CFLOW.stale=true;}return false;}
  finally{if(seq===CFLOW.seq){CFLOW.loading=false;cashFlowRender();}}
}
function cashFlowResetCampaign() {
  if(CFLOW.session===S?.session_id&&CFLOW.nation===S?.player)return;
  CFLOW.session=S?.session_id;CFLOW.nation=S?.player;CFLOW.data=null;CFLOW.state=null;CFLOW.error="";CFLOW.detailOpen=false;CFLOW.focus=null;CFLOW.scroll=0;CFLOW.stale=true;CFLOW.loading=false;++CFLOW.seq;
}
function cashFlowOnStateChanged() {
  cashFlowResetCampaign();CFLOW.state=null;CFLOW.stale=true;CFLOW.error="";CFLOW.loading=false;++CFLOW.seq;
  if(cashFlowActive()){cashFlowRender();cashFlowFetch();}
}
function cashFlowClose() {
  cashFlowRememberView();CFLOW.open=false;CFLOW.loading=false;CFLOW.stale=true;CFLOW.focus=null;++CFLOW.seq;
}
function cashFlowTabChanged() {
  if(!cashFlowContextActive()){cashFlowClose();return;}
  cashFlowResetCampaign();CFLOW.open=true;cashFlowBind();
}
function cashFlowDraftChanged() {if(cashFlowActive())cashFlowRender();}
function cashFlowReveal() {
  CFLOW.scroll=0;
  const scroller=document.querySelector("#left");if(scroller)scroller.scrollTop=0;
  const title=document.querySelector("#cashFlowTitle");
  if(title){title.focus({preventScroll:true});CFLOW.focus=null;}else CFLOW.focus="title";
}
