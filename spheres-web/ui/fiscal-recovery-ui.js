/* Fiscal assessments are supplied by the simulation. This desk formats those
   readings and opens existing controls; it never projects debt in JavaScript. */
const FISCAL_UI={busy:false,error:"",session:null};
function fiscalText(value){return String(value??"").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#39;");}
function fiscalNumber(value,digits=1){return Number.isFinite(value)?value.toLocaleString("en-US",{minimumFractionDigits:digits,maximumFractionDigits:digits}):"—";}
function fiscalPercent(value){return Number.isFinite(value)?`${fiscalNumber(value*100)}%`:"—";}
function fiscalPoints(value){return Number.isFinite(value)?`${value>0?"+":""}${fiscalNumber(value*100)} pp`:"—";}
function fiscalOrdersPending(){return FISCAL_UI.busy||(typeof advancing!=="undefined"&&advancing)||(typeof pendingAdvance!=="undefined"&&!!pendingAdvance)||(typeof COMMAND_CHANNEL!=="undefined"&&!!(COMMAND_CHANNEL.busy||COMMAND_CHANNEL.pending))||(typeof SESSION!=="undefined"&&SESSION.busy)||(typeof CAB!=="undefined"&&CAB.busy);}
function fiscalUiReset(){if(FISCAL_UI.session!==S?.session_id){FISCAL_UI.session=S?.session_id;FISCAL_UI.error="";FISCAL_UI.busy=false;}}
function fiscalMetric(label,value,note){return `<div><dt>${fiscalText(label)}</dt><dd>${fiscalText(value)}</dd><small>${fiscalText(note)}</small></div>`;}
function fiscalRecoveryHtml(compact=false){
  fiscalUiReset();
  if(!S?.player)return "";
  const a=S.fiscal_recovery,locked=fiscalOrdersPending(),button=(label,action)=>`<button type="button" data-fiscal-action="${action}" ${locked?"disabled":""}>${label}</button>`;
  if(compact)return `<section class="fiscal-recovery fiscal-compact" aria-label="Fiscal recovery"><div><p class="fr-kicker">Debt &amp; government confidence</p><h2>${fiscalText(S.fiscal_recovery_enabled?(a?.status_label||"Reading fiscal position"):"Fiscal recovery is available")}</h2><p>${S.fiscal_recovery_enabled?`${fiscalPercent(a?.debt_gdp)} debt / GDP · ${fiscalText(a?.reason||"The first assessment is being prepared.")}`:"This saved campaign can adopt shared debt accounting and recovery rules."}</p></div>${button("Review debt &amp; recovery →","recovery")}</section>`;
  const message=FISCAL_UI.error?`<p class="fr-message" role="alert">${fiscalText(FISCAL_UI.error)}</p>`:"";
  if(!S.fiscal_recovery_enabled)return `<section class="fiscal-recovery" aria-labelledby="fiscalRecoveryTitle"><p class="fr-kicker">A clearer path out of debt</p><h2 id="fiscalRecoveryTitle">Enable fiscal recovery</h2><p>Use shared debt accounting, recovery monitoring and gradual political pressure for every country. Governments must make debt payments affordable and stabilize their debt path.</p><p>This permanently changes the economic rules of this campaign. Recorded debt remains due; countries with unopened treasury accounts begin paying interest on it. It grants no cash or debt relief. Keep a separate save to retain the previous rules.</p><p>Your taxes and budget remain your decisions. AI governments review their own recovery choices.</p>${message}<div class="fr-actions"><button type="button" data-fiscal-enable ${locked?"disabled":""}>${FISCAL_UI.busy?"Enabling…":"Enable fiscal recovery for this campaign"}</button></div></section>`;
  if(!a)return `<section class="fiscal-recovery"><h2 id="fiscalRecoveryTitle">Fiscal recovery</h2><p role="status">The first assessment is being prepared.</p>${message}</section>`;
  const tone=["stable","watch","adjustment","recovering","crisis"].includes(a.status)?a.status:"watch";
  const observed=a.months_observed>0?`${a.months_observed} observed month${a.months_observed===1?"":"s"}`:"Provisional: no complete monthly receipt yet";
  const pressure=Number.isFinite(a.stability_change_per_month)?`${a.stability_change_per_month>0?"+":""}${fiscalNumber(a.stability_change_per_month,3)} points / month`:"—";
  const offers=(S.stratagems?.offers||[]).filter(o=>["austerity","debt_restructuring"].includes(o.id));
  return `<section class="fiscal-recovery fr-${tone}" aria-labelledby="fiscalRecoveryTitle"><header><div><p class="fr-kicker">Debt &amp; government confidence</p><h2 id="fiscalRecoveryTitle">${fiscalText(a.status_label)}</h2></div><span class="fr-status">${a.recovery_required?"Recovery needed":"Payments under review"}</span></header><p>${fiscalText(a.reason)}</p>${message}
    <dl class="fr-metrics">${fiscalMetric("Debt / GDP",fiscalPercent(a.debt_gdp),"Current government debt")}${fiscalMetric("Debt trend",a.months_observed>0?fiscalPoints(a.annual_debt_change_gdp):"Awaiting first report","Annualized change in debt / GDP")}${fiscalMetric("Interest / revenue",a.interest_revenue===null?"No positive revenue":fiscalPercent(a.interest_revenue),"Revenue absorbed by interest")}${fiscalMetric("Primary balance",fiscalPercent(a.primary_balance_gdp),"GDP share before interest")}${fiscalMetric("Debt / GDP in five years",fiscalPercent(a.projected_debt_gdp_5y),"Projection under stated assumptions")}${fiscalMetric(a.months_observed>0?"Observed adjustment gap":"Provisional adjustment gap",fiscalPoints(a.adjustment_needed_gdp),"Annual primary balance / GDP")}</dl>
    <div class="fr-next"><h3>Your next move</h3><p>${fiscalText(a.next_action)}</p><div class="fr-actions">${button("Review taxes →","tax")}${button("Review ministry spending →","budget")}${button("Review Cabinet recovery actions →","plays")}</div>${offers.length?`<p class="fr-note">Available Cabinet options: ${offers.map(o=>`${fiscalText(o.name)} (${fiscalNumber(o.cost,0)} PC${o.affordable?"":", more political capital needed"})`).join(" · ")}. Review their consequences before enacting.</p>`:`<p class="fr-note">Fiscal Consolidation and Restructure the Debt appear in Cabinet plays when their conditions are met. Their costs and consequences still apply.</p>`}</div>
    <details class="fr-detail" id="fiscalRecoveryDetails"><summary>How recovery and pressure are measured</summary><dl class="fr-method">${fiscalMetric("Adjustment time remaining",`${fiscalNumber(a.grace_months_remaining,0)} months`,"Interest remains payable during this period")}${fiscalMetric("Sustained fiscal stress",`${fiscalNumber(a.stress_months,0)} months`,"Tracked at monthly reviews")}${fiscalMetric("Improving results",`${fiscalNumber(a.improving_months,0)} months`,"A plan alone does not clear pressure")}${fiscalMetric("Fiscal stability effect",pressure,"This fiscal channel only; other pressures are separate")}${fiscalMetric("Primary balance needed",fiscalPercent(a.required_primary_balance_gdp),"GDP share to control the debt path")}${fiscalMetric("Growth used in projection",fiscalPercent(a.annual_real_growth),"Annual real GDP growth")}${Number.isFinite(a.current_policy_adjustment_gdp)?fiscalMetric("Gap with current tax policy",fiscalPoints(a.current_policy_adjustment_gdp),"Latest cash spending; receipts must confirm it"):""}</dl><p><strong>${fiscalText(observed)}.</strong> ${fiscalText(a.forecast_assumptions)}</p><p>Under control means a stable debt path and affordable payments. There is no single debt ceiling for every country. Monthly reviews track actual results; AI policy reviews are quarterly.</p><p>Unused construction authority is not cash spending. Cutting it cannot count as a cash saving. Fiscal pressure eases with improving results; inflation, service cuts and unemployment keep their existing consequences.</p><p>Cash readings use recent posted results, so policy changes take time to appear. Draft tax and budget changes take effect only after enactment.</p></details></section>`;
}
function fiscalRecoveryNavigate(action,state=S){
  if(state!==S||fiscalOrdersPending())return false;
  if(action==="recovery"){openConstructionCabinet("policy");document.querySelector("#fiscalRecoveryTitle")?.scrollIntoView({block:"start"});return true;}
  if(action==="tax"||action==="budget")return cashFlowNavigate(action==="tax"?{action:"policy",control:"tax"}:{action:"budget"});
  if(action==="plays"){openConstructionCabinet("overview");const panel=document.querySelector("#cabinetPlays");if(!panel)return false;panel.open=true;panel.querySelector("summary")?.focus();panel.scrollIntoView({block:"start"});return true;}
  return false;
}
function fiscalRecoverySync(){
  const locked=fiscalOrdersPending();
  document.querySelectorAll("[data-fiscal-action]").forEach(button=>{button.disabled=locked||button.fiscalState!==S;});
  const enable=document.querySelector("[data-fiscal-enable]");
  if(enable)enable.disabled=locked||enable.fiscalState!==S||!!S?.fiscal_recovery_enabled;
}
function fiscalRecoveryBind(){
  const state=S;
  document.querySelectorAll("[data-fiscal-action]").forEach(button=>{button.fiscalState=state;button.onclick=()=>fiscalRecoveryNavigate(button.dataset.fiscalAction,state);});
  const enable=document.querySelector("[data-fiscal-enable]");
  if(enable){enable.fiscalState=state;enable.onclick=()=>fiscalRecoveryEnable(state);}
  fiscalRecoverySync();
}
async function fiscalRecoveryEnable(state=S){
  if(state!==S||!state?.player||state.fiscal_recovery_enabled||fiscalOrdersPending())return false;
  FISCAL_UI.busy=true;FISCAL_UI.error="";renderLeft(me());
  try{const response=await api("/api/command",{commands:[{kind:"enable_fiscal_recovery"}]});if(S!==state)return false;if(response.errors?.length)throw new Error(response.errors.join(" "));await adopt(response,false);return true;}
  catch(error){if(S?.session_id===state.session_id)FISCAL_UI.error=error.message||"Fiscal recovery could not be enabled.";return false;}
  finally{if(S?.session_id===state.session_id){FISCAL_UI.busy=false;renderLeft(me());}}
}
