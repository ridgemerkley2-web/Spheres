/* Pure presentation of the sim's fiscal_recovery_view. The host owns current
   session checks, confirmation and the existing command/navigation channels. */
(function(root){
  "use strict";
  const escape=value=>String(value??"").replace(/[&<>"']/g,char=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"})[char]);
  const number=(value,digits=1)=>Number.isFinite(value)?value.toLocaleString("en-US",{minimumFractionDigits:digits,maximumFractionDigits:digits}):"—";
  const percent=value=>Number.isFinite(value)?`${number(value*100)}%`:"—";
  const points=value=>Number.isFinite(value)?`${value>0?"+":""}${number(value*100)} percentage points`:"—";
  const metric=(label,value,note)=>`<div><dt>${escape(label)}</dt><dd>${escape(value)}</dd><small>${escape(note)}</small></div>`;
  const actionIds=new Set(["tax","budget","construction","plays"]);
  const provenanceLabels=Object.freeze({
    population_age_0_14:"population aged 0–14",
    population_age_15_64:"population aged 15–64",
    population_age_65_plus:"population aged 65 and over",
    labor_force_participation_15_64:"labor force participation among people aged 15–64",
    education_attainment_secondary_25_plus:"secondary school attainment among adults aged 25 and over",
    education_attainment_tertiary_25_plus:"higher education attainment among adults aged 25 and over",
  });
  const readableProvenance=note=>String(note??"").replace(/\b[a-z][a-z0-9]*(?:_[a-z0-9]+)+\b/g,key=>provenanceLabels[key]||key);

  function render(view,options={}){
    if(!view?.available)return "";
    const title=options.compact?"Debt and recovery":"Government debt and recovery";
    if(!view.enabled)return `<section class="cf-priorities fiscal-recovery" aria-label="${title}"><h2>${title}</h2><p>${escape(view.enrollment?.reason)}</p><p class="cf-note">${escape(view.enrollment?.effect)}</p>${view.enrollment?.allowed?`<div class="cf-actions"><button type="button" data-fiscal-enable>Review fiscal recovery adoption</button></div>`:""}</section>`;
    const a=view.assessment;
    if(!a)return `<section class="cf-priorities fiscal-recovery" aria-label="${title}"><h2>${title}</h2><p role="status">The first fiscal reading is being prepared.</p></section>`;
    const observed=a.months_observed>0?`${number(a.months_observed,0)} observed months`:"Provisional outlook; less than one full month observed";
    const actions=(view.actions||[]).filter(action=>actionIds.has(action.id)).map(action=>`<button type="button" data-fiscal-action="${action.id}">${escape(action.label)}</button>`).join("");
    const head=`<section class="cf-priorities fiscal-recovery" aria-label="${title}"><p class="cf-kicker">${escape(view.name)} · ${escape(view.date)}</p><h2>${escape(a.status_label)}</h2><p>${escape(a.reason)}</p><dl class="cf-balances">${metric("Debt / GDP",percent(a.debt_gdp),"Current government debt")}${metric("Interest / revenue",a.interest_revenue===null?"No positive revenue":percent(a.interest_revenue),"Positive net interest only")}${metric("Adjustment gap",points(a.current_policy_adjustment_gdp),"Current tax policy; latest posted spending")}</dl><p><strong>Next step:</strong> ${escape(a.next_action)}</p><div class="cf-actions">${actions}</div>`;
    if(options.compact)return head+"</section>";
    const stability=Number.isFinite(a.stability_change_per_month)?`${number(a.stability_change_per_month,3)} points / month`:"—";
    return head+`<details class="cf-budget"><summary>How this reading is measured</summary><div class="cf-budget-body"><p>${escape(observed)}. Partial months use only the days actually observed.</p><dl class="cf-balances">${metric("Primary balance",percent(a.primary_balance_gdp),"Posted revenue less spending, before interest")}${metric("Net interest / GDP",percent(a.interest_gdp),"Signed amount from the existing real-rate model")}${metric("Debt trend",a.months_observed>0?points(a.annual_debt_change_gdp):"Awaiting receipts","Annualized change in debt / GDP")}${metric("Debt / GDP in five years",percent(a.projected_debt_gdp_5y),"Illustrative unchanged-policy scenario")}${metric("Adjustment time remaining",`${number(a.grace_months_remaining,0)} months`,"Interest and deficits remain payable")}${metric("Fiscal stability effect",stability,"This fiscal channel only")}${metric("Other recurring obligations",percent(a.other_obligations_gdp),"Observed residual; not a second expense")}</dl><p class="cf-note">${escape(a.forecast_assumptions)}</p><p class="cf-note">${escape(view.accounting_note)}</p></div></details></section>`;
  }

  function renderConnected(view,options={}){
    if(!view)return "";
    const upgrade=view.enabled?"":`<section class="cf-priorities"><p class="cf-kicker">People · Work · Public finances</p><h2>Connect your economy</h2><p>${escape(view.upgrade?.effect)}</p>${view.upgrade?.reason?`<p class="cf-message">${escape(view.upgrade.reason)}</p>`:""}<div class="cf-actions"><button type="button" data-economy-upgrade ${view.upgrade?.available?"":"disabled"}>Review economy upgrade</button></div></section>`;
    const p=view.population;
    const people=value=>Number.isFinite(value)?`${number(value,3)} million`:"—";
    const policies=p?(p.policies||[]).map(q=>`<article class="cf-priority"><h3>${escape(q.name)}${q.selected?" · Current focus":""}</h3><p>${escape(q.description)}</p><p>${number(q.political_cost,0)} political capital${q.cooldown_days?` · ${number(q.cooldown_days,0)} days until review`:""}</p>${q.reason?`<p class="cf-note">${escape(q.reason)}</p>`:""}<button type="button" data-economy-policy="${escape(q.policy)}" data-economy-available="${q.available===true}" ${q.available?"":"disabled"}>Review ${escape(q.name)}</button></article>`).join(""):"";
    const workforce=p?`<section class="cf-priorities" aria-label="People and work"><p class="cf-kicker">${escape(view.date)}</p><h2>People and work</h2><dl class="cf-balances">${metric("Residents",people(p.population_m),"Current population accounts")}${metric("Employed",people(p.employed_m),"People currently holding jobs")}${metric("Unemployment",percent(p.unemployment_rate),"Share of the labor force")}${metric("Skilled vacancies",people(p.skilled_vacancies_m),"Open jobs needing qualifications")}${metric("In education",people(p.students_m),"School and adult students")}${metric("Training capacity",people(p.training_capacity_m),"Available course places in the model")}</dl><details class="cf-budget"><summary>Choose a training focus</summary><div class="cf-budget-body"><p>Review a focus before enacting it. Training takes time and uses the existing education budget; choosing a focus does not grant graduates.</p><div class="cf-priority-grid">${policies}</div>${p.courses?.length?`<h3>Courses in progress</h3><ul>${p.courses.map(c=>`<li>${escape(c.name)} · ${people(c.people_m)} people · ${percent(c.progress)} complete · ${number(c.days_remaining,0)} days remaining</li>`).join("")}</ul>`:"<p>No courses are currently in progress.</p>"}${(p.notes||[]).map(note=>`<p class="cf-note">${escape(readableProvenance(note))}</p>`).join("")}</div></details></section>`:"";
    const i=view.industry;
    const sites=i?(i.facilities||[]).filter(row=>!row.inherited):[];
    const industry=i?`<section class="cf-priorities" aria-label="Industry operations"><h2>What your industry is doing</h2><dl class="cf-balances">${metric("New-site jobs filled",number(i.jobs_filled,0),"Workers used in dated operation")}${metric("New-site jobs required",number(i.jobs_required,0),"Positions required for full operation")}${metric("Advanced components",number(i.advanced_components_stock,2),"Government stock available")}</dl><p>New facilities need workers, skills, power and supplies. Latest output shows completed production; readiness shows what can run now.</p>${i.note?`<details class="cf-budget"><summary>How industry is accounted for</summary><div class="cf-budget-body"><p class="cf-note">${escape(i.note)}</p></div></details>`:""}${sites.length?`<details class="cf-budget"><summary>Inspect ${number(sites.length,0)} operating sites</summary><div class="cf-budget-body">${sites.slice(0,6).map(row=>`<article class="cf-priority"><h3>${escape(row.name)} · ${escape(row.district)}</h3><p>${escape(row.reason)}</p><dl class="cf-balances">${metric("Latest output",`${number(row.output_daily,3)} ${row.output_unit||""}`,row.recorded_day==null?"No paid operating receipt yet":`Recorded ${row.recorded_date||"date unavailable"}`)}${metric("Current readiness",percent(row.utilization),"Conditional forecast, not paid output")}${metric("Jobs used",number(row.jobs_filled,0),"From the latest operating receipt")}</dl></article>`).join("")}${sites.length>6?"<p>Showing the first six sites. Open operating industry for its full ledger.</p>":""}<div class="cf-actions"><button type="button" data-economy-industry>Open operating industry</button></div></div></details>`:"<p>New sites will appear as projects finish. Inherited economic activity stays in its existing account and does not grant government stock.</p>"}</section>`:"";
    return upgrade+workforce+industry+(options.recovery!==false&&view.recovery?.enabled?render(view.recovery):"");
  }

  function bind(element,host={}){
    if(!element)return ()=>{};
    const allowed=()=>typeof host.isCurrent==="function"&&host.isCurrent()&&!(typeof host.isBusy==="function"&&host.isBusy());
    element.querySelectorAll("[data-fiscal-action],[data-fiscal-enable],[data-economy-upgrade],[data-economy-policy],[data-economy-industry]").forEach(button=>{
      const unavailable=button.hasAttribute("data-economy-policy")&&button.dataset.economyAvailable!=="true";
      const upgradeUnavailable=button.hasAttribute("data-economy-upgrade")&&host.canUpgrade!==true;
      button.disabled=!allowed()||unavailable||upgradeUnavailable;
    });
    const click=event=>{
      const button=event.target?.closest?.("[data-fiscal-action],[data-fiscal-enable],[data-economy-upgrade],[data-economy-policy],[data-economy-industry]");
      if(!button||!element.contains(button)||button.disabled||!allowed())return;
      if(button.hasAttribute("data-economy-upgrade")){if(host.canUpgrade===true&&typeof host.onEnable==="function")host.onEnable();return;}
      if(button.hasAttribute("data-economy-policy")){if(button.dataset.economyAvailable==="true"&&typeof host.onPolicy==="function")host.onPolicy(button.dataset.economyPolicy);return;}
      if(button.hasAttribute("data-economy-industry")){if(typeof host.onNavigate==="function")host.onNavigate("industry");return;}
      if(button.hasAttribute("data-fiscal-enable")){if(typeof host.onEnable==="function")host.onEnable();return;}
      const action=button.dataset.fiscalAction;
      if(actionIds.has(action)&&typeof host.onNavigate==="function")host.onNavigate(action);
    };
    element.addEventListener("click",click);
    return ()=>element.removeEventListener("click",click);
  }
  const api=Object.freeze({render,renderConnected,bind});
  if(typeof module!=="undefined"&&module.exports)module.exports=api;
  else root.FiscalRecoveryUI=api;
})(typeof globalThis!=="undefined"?globalThis:this);
