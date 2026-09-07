/* Industry displays server-owned operations and navigation. It creates no orders,
   converts no capacity into output, and never treats a missing receipt as zero. */
const IDESK = {open:false,data:null,state:null,session:null,nation:null,seq:0,loading:false,stale:true,error:"",
  query:"",filter:"all",details:new Set(),focus:null,scroll:0};
const INDUSTRY_FILTERS = [["all","All"],["attention","Needs attention"],["producing","Producing"],["supporting","Supporting"]];
const INDUSTRY_NAV = new Set(["construction","budget","resources","trade","research","manufacture","province","inherited"]);

function industryText(value) {
  return String(value??"").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#39;");
}
function industryNumber(value) {
  return Number.isFinite(value)?(value===0?0:value).toLocaleString("en-US",{maximumSignificantDigits:5}):"—";
}
function industryMoney(value) { return Number.isFinite(value)?economyMoney(value):"—"; }
function industryActive() {
  return IDESK.open && typeof S!=="undefined" && !!S?.player && typeof CAB!=="undefined" && CAB.tab==="industry"
    && (typeof cabinetIsOpen!=="function" || cabinetIsOpen());
}
function industryOrdersPending() {
  return (typeof advancing!=="undefined" && advancing) || (typeof pendingAdvance!=="undefined" && !!pendingAdvance)
    || (typeof COMMAND_CHANNEL!=="undefined" && !!(COMMAND_CHANNEL.busy||COMMAND_CHANNEL.pending))
    || (typeof SESSION!=="undefined" && SESSION.busy) || (typeof CAB!=="undefined" && CAB.busy)
    || (typeof PROD!=="undefined" && PROD.busy) || (typeof COMP!=="undefined" && !!(COMP.busy||COMP.pending));
}
function industryCurrent(data=IDESK.data,state=IDESK.state) {
  return industryActive() && !!data && data===IDESK.data && state===S && state===IDESK.state
    && !IDESK.loading && !IDESK.stale && !IDESK.error && !industryOrdersPending();
}
function industryActions(rows) {
  return (Array.isArray(rows)?rows:[]).filter(row=>row && INDUSTRY_NAV.has(row.action));
}
function industryButton(action,label=action?.label,primary=false) {
  if(!action || !INDUSTRY_NAV.has(action.action)) return "";
  return `<button type="button" class="industry-action${primary?" primary":""}" data-industry-action="${industryText(JSON.stringify(action))}" ${industryCurrent()&&action.enabled!==false&&action.available!==false?"":"disabled"}>${industryText(label||"Review")}</button>`;
}
function industryNeedsAttention(site) {
  return site.attention===true;
}
function industryProducing(site) {
  return site.has_receipt===true && (Number.isFinite(site.output_daily)&&site.output_daily>0
    || Number.isFinite(site.research?.prototype_credit)&&site.research.prototype_credit>0);
}
function industryMatchesFilter(site,filter) {
  if(filter==="attention") return industryNeedsAttention(site);
  if(filter==="producing") return industryProducing(site);
  if(filter==="supporting") return site.productive===false;
  return true;
}
function industrySiteMatches(site,query) {
  const normalize=value=>String(value??"").normalize("NFD").replace(/[\u0300-\u036f]/g,"").toLocaleLowerCase("en-US");
  const text=normalize(`${site.district_name||""} ${site.district||""} ${site.name||""} ${String(site.kind||"").replace(/_/g," ")}`);
  return normalize(query).trim().split(/\s+/).filter(Boolean).every(token=>text.includes(token));
}
function industrySites(data=IDESK.data) { return Array.isArray(data?.sites)?data.sites:[]; }
function industryVisibleSites(data=IDESK.data) {
  return industrySites(data).filter(site=>industryMatchesFilter(site,IDESK.filter)&&industrySiteMatches(site,IDESK.query));
}
function industryMetric(label,value,note="") {
  return `<div><dt>${industryText(label)}</dt><dd>${industryText(value)}</dd>${note?`<small>${industryText(note)}</small>`:""}</div>`;
}
function industryProgressText(fraction) {
  if(!Number.isFinite(fraction)) return "Progress unavailable";
  const percent=Math.round(Math.max(0,Math.min(1,fraction))*1000)/10;
  return `${fraction>0&&percent===0?"<0.1":industryNumber(percent)}% complete`;
}
function industryStatus(site) {
  if(site.productive===false && !industryNeedsAttention(site) && !site.research) return "Supporting";
  if(site.productive===true && !site.has_receipt && !industryNeedsAttention(site)) return "Awaiting first operation";
  return String(site.status||"Recorded").replace(/_/g," ").replace(/^./,letter=>letter.toUpperCase());
}
function industrySiteHtml(site) {
  const actual=site.has_receipt===true;
  const receipt=actual?(site.receipt_label?`Recorded · ${site.receipt_label}`:"Latest recorded operation"):"Awaiting first operation";
  const supporting=site.productive===false;
  const output=actual&&Number.isFinite(site.output_daily)?`${industryNumber(site.output_daily)} ${site.output_unit||"units"}`:null;
  const metrics=[];
  if(site.productive===true) metrics.push(industryMetric("Recorded output",output||"Awaiting first operation",output?receipt:"No production receipt yet"));
  if(site.research && actual) metrics.push(industryMetric("Prototype credit",Number.isFinite(site.research.prototype_credit)?`${industryNumber(site.research.prototype_credit)} research units`:"—",site.research.technology_name||"Recorded research"));
  if(actual&&Number.isFinite(site.cash_spent_daily_bn)) metrics.push(industryMetric("Recorded operating spending",industryMoney(site.cash_spent_daily_bn),receipt));
  if(actual&&Number.isFinite(site.power_used_daily)) metrics.push(industryMetric("Recorded power use",`${industryNumber(site.power_used_daily)} modeled units`,receipt));
  const scale=Number.isFinite(site.capacity_micros)?`${industryNumber(site.capacity_micros/10000)}% of a standard workshop`
    :Number.isFinite(site.level)?`Level ${industryNumber(site.level)}`:"";
  const detailKey=String(site.id??`${site.kind}:${site.district}`);
  return `<article class="industry-site${industryNeedsAttention(site)?" needs-attention":""}"><header><div><p class="industry-place">${industryText(site.district_name||site.district||"Province")}</p><h3>${industryText(site.name||"Facility")}</h3>${scale?`<p class="industry-note">${industryText(scale)}</p>`:""}</div><span class="industry-status${industryNeedsAttention(site)?" attention":""}">${industryText(industryStatus(site))}</span></header>
    ${site.reason&&(!supporting||site.reason!==site.effect)?`<p class="industry-reason">${industryText(site.reason)}</p>`:""}
    ${supporting?`<p class="industry-support">${industryText(site.effect||"Supports other activity in this province.")}</p>`:""}
    ${metrics.length?`<dl class="industry-site-metrics">${metrics.join("")}</dl>`:""}
    ${!supporting&&!actual?`<p class="industry-note">Operating spending will appear after work is recorded.</p>`:""}
    ${!supporting&&site.effect?`<details data-industry-detail="${industryText(detailKey)}" ${IDESK.details.has(detailKey)?"open":""}><summary>Facility role</summary><p>${industryText(site.effect)}</p></details>`:""}
    <div class="industry-actions">${industryActions(site.actions).map(action=>industryButton(action,industryActionLabel(action))).join("")}</div></article>`;
}
function industryActionLabel(action) {
  if(action.action==="budget") return action.ministry==="science"?"Fund research":action.ministry==="defense"?"Fund equipment":action.department===1?"Fund energy":"Fund operations";
  return {province:"Open province",resources:"Find raw inputs",trade:"Trade goods",research:"Review research",construction:"Build here",manufacture:"Manage equipment",inherited:"View inherited industry"}[action.action]||action.label;
}
function industryQueueHtml(data) {
  const queue=Array.isArray(data.queue)?data.queue:[];
  if(!queue.length) return "";
  const empty=!industrySites(data).length;
  return `<section class="industry-pipeline" aria-labelledby="industryPipelineTitle"><div class="industry-section-heading"><div><p class="industry-kicker">Still in construction</p><h2 id="industryPipelineTitle">${empty?"Your production base is taking shape":"On the way"}</h2><p>Construction funds these projects as work progresses. Completion comes before operation. Completed mines remain in Resources.</p></div>${industryButton({action:"construction"},"View construction queue")}</div><div class="industry-queue">${queue.map(project=>{
    const known=Number.isFinite(project.progress),percent=known?Math.max(0,Math.min(100,project.progress*100)):null;
    const progress=industryProgressText(project.progress);
    return `<article class="industry-queued"><div><h3>${industryText(project.name||"Construction project")}</h3><p>${industryText(project.district_name||project.district||"Province")}</p></div><div class="industry-queue-progress"><strong>${industryText(progress)}</strong>${known?`<div class="industry-progress" role="progressbar" aria-label="${industryText(project.name||"Project")} progress" aria-valuenow="${percent}" aria-valuemin="0" aria-valuemax="100"><i style="width:${percent}%"></i></div>`:""}<small>${Number.isFinite(project.eta_days)?`About ${industryNumber(Math.ceil(project.eta_days))} days remaining`:"Completion estimate unavailable"}</small></div>${industryButton({action:"construction",project:project.id,district:project.district},"Open construction project")}</article>`;
  }).join("")}</div></section>`;
}
function industryContentHtml() {
  const data=IDESK.data,sites=industrySites(data),visible=industryVisibleSites(data);
  const hero=`<header class="industry-hero"><div><p class="industry-kicker">Economy · Industry</p><h1>From construction to production</h1><p>Check completed facilities, their recorded work, and the supplies that keep them running.</p></div><div class="industry-reading"><strong>${industryText(data?.name||"National operations")}</strong><span>Current date · ${industryText(data?.date||"Loading")}</span><span>${industryText(data?.settlement?.label?`Latest industry settlement · ${data.settlement.label}`:"No industry settlement recorded yet")}</span></div></header>`;
  const status=IDESK.error?`<div class="industry-message error" role="alert"><strong>Industry could not be refreshed.</strong><p>${industryText(IDESK.error)}</p>${data?"<p>The previous reading is shown below. Actions are disabled until it is refreshed.</p>":""}<button type="button" data-industry-retry ${industryOrdersPending()?"disabled":""}>Retry Industry</button></div>`
    :IDESK.loading||IDESK.stale?`<p class="industry-message" role="status">${data?"Updating the operations reading…":"Loading your industry…"}</p>`
    :industryOrdersPending()?`<p class="industry-message" role="status">Finish or review the pending turn or order before opening another task.</p>`:"";
  if(!data) return hero+status;
  const goods=(Array.isArray(data.goods)?data.goods:[]).map(good=>industryMetric(good.name||good.good,`${industryNumber(good.stock)} packs in stock`,Number.isFinite(good.capacity)?`${industryNumber(good.capacity)} packs of storage for this good`:"Storage capacity unavailable")).join("");
  const power=industryMetric("Modeled power capacity",`${industryNumber(data.power?.capacity_daily)} units / day`,Number.isFinite(data.power?.used_daily)?`${industryNumber(data.power.used_daily)} units used in the recorded operation`:"Capacity is not recorded output");
  const toolbar=`<div class="industry-toolbar"><label for="industrySearch">Find a province or facility</label><div class="industry-search"><input id="industrySearch" type="search" value="${industryText(IDESK.query)}" placeholder="Province, workshop, grid…" autocomplete="off" aria-controls="industrySiteList"><button type="button" data-industry-clear ${IDESK.query?"":"disabled"}>Clear search</button></div><div class="industry-filters" aria-label="Facility filters">${INDUSTRY_FILTERS.map(([key,label])=>`<button type="button" data-industry-filter="${key}" aria-pressed="${IDESK.filter===key}">${label} <span>${sites.filter(site=>industryMatchesFilter(site,key)).length}</span></button>`).join("")}</div><p class="industry-note" role="status" aria-live="polite">${visible.length} of ${sites.length} completed facilities shown</p></div>`;
  const cards=visible.length?`<div class="industry-sites" id="industrySiteList">${visible.map(industrySiteHtml).join("")}</div>`
    :sites.length?`<div class="industry-empty" id="industrySiteList"><h3>No facilities match these filters</h3><p>Try another province or facility name, or show all facilities.</p><button type="button" data-industry-reset>Clear search and filters</button></div>`
    :`<div class="industry-empty" id="industrySiteList"><h2>No completed facilities here yet</h2><p>${data.queue?.length?"Your queued projects are listed below. Production receipts begin after facilities are complete and operating.":"Add a workshop or another facility in Construction, then return here to follow its operation."}</p>${industryButton({action:"construction"},"Explore construction",true)}</div>`;
  return `${hero}${status}${data.enabled===false?`<p class="industry-message">Daily facility operation is not enabled in this campaign. Completed assets remain visible below.</p>`:""}<dl class="industry-stock">${goods}${power}</dl>
    <div class="industry-actions industry-global-actions">${industryButton({action:"budget",ministry:"industry"},"Fund operations")}${industryButton({action:"resources"},"Find raw inputs")}${industryButton({action:"construction"},"Build a facility")}</div>
    <section class="industry-facilities" aria-labelledby="industryFacilitiesTitle"><div class="industry-section-heading"><div><p class="industry-kicker">Completed assets</p><h2 id="industryFacilitiesTitle">Your facilities</h2></div><button type="button" data-industry-refresh ${IDESK.loading||industryOrdersPending()?"disabled":""}>Refresh reading</button></div>${sites.length?toolbar:""}${cards}</section>
    ${industryQueueHtml(data)}<aside class="industry-inherited"><div><h2>The economy beyond these facilities</h2><p>Your inherited industry remains part of the national economy. Review its broader production and economic accounts alongside these facilities.</p></div>${industryButton({action:"inherited"},"See the wider national economy")}</aside>
    ${data.note?`<p class="industry-note industry-model-note">${industryText(data.note)}</p>`:""}`;
}
function industryRememberView() {
  if(!IDESK.open||typeof CAB==="undefined"||CAB.tab!=="industry") return;
  const root=document.querySelector("#industryRoot");
  if(!root || root.dataset.industrySession!==String(IDESK.session??"")) return;
  const scroller=document.querySelector("#left");
  if(scroller) IDESK.scroll=scroller.scrollTop;
  const active=document.activeElement;
  if(active?.id==="industrySearch") IDESK.focus={id:active.id,start:active.selectionStart,end:active.selectionEnd};
  else if(INDUSTRY_FILTERS.some(([key])=>key===active?.dataset?.industryFilter)) IDESK.focus={filter:active.dataset.industryFilter};
  else if(active?.dataset && "industryRefresh" in active.dataset) IDESK.focus={control:"refresh"};
  else if(active?.dataset && "industryRetry" in active.dataset) IDESK.focus={control:"retry"};
  root.querySelectorAll("details[data-industry-detail]").forEach(detail=>{
    if(detail.open) IDESK.details.add(detail.dataset.industryDetail); else IDESK.details.delete(detail.dataset.industryDetail);
  });
}
function industryPanelHtml() {
  industryRememberView();
  return `<div id="industryRoot" class="industry-desk" data-industry-session="${industryText(IDESK.session)}" aria-busy="${IDESK.loading}">${industryContentHtml()}</div>`;
}
function industryRender() {
  if(!industryActive()) return;
  const root=document.querySelector("#industryRoot");
  if(!root) return;
  industryRememberView();root.innerHTML=industryContentHtml();root.dataset.industrySession=String(IDESK.session??"");root.setAttribute("aria-busy",String(IDESK.loading));
  industryBind(false);
}
function industryNavigateCurrent(action,data=IDESK.data,state=IDESK.state) {
  if(!industryCurrent(data,state)||!action||!INDUSTRY_NAV.has(action.action)||action.enabled===false||action.available===false) return false;
  const clean={action:action.action};
  for(const key of ["kind","district","good","ministry","department","project"]) {
    if(typeof action[key]==="string" || Number.isFinite(action[key])) clean[key]=action[key];
  }
  industryNavigate(clean);return true;
}
function industryBind(fetchIfNeeded=true) {
  if(!industryActive()) return;
  const root=document.querySelector("#industryRoot");
  if(!root) return;
  const data=IDESK.data,state=IDESK.state;
  root.querySelectorAll("[data-industry-action]").forEach(button=>{
    let action;try{action=JSON.parse(button.dataset.industryAction);}catch{return;}
    button.onclick=()=>industryNavigateCurrent(action,data,state);
  });
  const search=root.querySelector("#industrySearch");
  if(search) search.oninput=()=>{IDESK.query=search.value;industryRender();};
  const clear=root.querySelector("[data-industry-clear]");
  if(clear) clear.onclick=()=>{IDESK.query="";industryRender();root.querySelector("#industrySearch")?.focus({preventScroll:true});};
  const reset=root.querySelector("[data-industry-reset]");
  if(reset) reset.onclick=()=>{IDESK.query="";IDESK.filter="all";industryRender();};
  root.querySelectorAll("[data-industry-filter]").forEach(button=>{button.onclick=()=>{
    if(INDUSTRY_FILTERS.some(([key])=>key===button.dataset.industryFilter)){IDESK.filter=button.dataset.industryFilter;IDESK.focus={filter:button.dataset.industryFilter};industryRender();}
  };});
  root.querySelectorAll("details[data-industry-detail]").forEach(detail=>{detail.ontoggle=()=>{
    if(detail.open) IDESK.details.add(detail.dataset.industryDetail); else IDESK.details.delete(detail.dataset.industryDetail);
  };});
  for(const key of ["retry","refresh"]) {
    const button=root.querySelector(`[data-industry-${key}]`);
    if(button) button.onclick=()=>{if(!industryOrdersPending()){IDESK.focus={control:key};industryFetch(true);}};
  }
  const scroller=document.querySelector("#left");if(scroller) scroller.scrollTop=IDESK.scroll;
  if(IDESK.focus?.id==="industrySearch" && search){search.focus({preventScroll:true});if(IDESK.focus.start!=null) search.setSelectionRange(IDESK.focus.start,IDESK.focus.end);IDESK.focus=null;}
  else if(IDESK.focus?.filter) {
    const button=root.querySelector(`[data-industry-filter="${IDESK.focus.filter}"]`);
    if(button){button.focus({preventScroll:true});IDESK.focus=null;}
  } else if(IDESK.focus?.control) {
    const key=IDESK.focus.control;
    const button=root.querySelector(`[data-industry-${key}]`) || (key==="retry"?root.querySelector("[data-industry-refresh]"):null);
    if(button&&!button.disabled){button.focus({preventScroll:true});IDESK.focus=null;}
  }
  if(fetchIfNeeded&&IDESK.stale&&!IDESK.loading&&!IDESK.error) industryFetch();
}
async function industryFetch(force=false) {
  if(!industryActive()||IDESK.loading||(!force&&!IDESK.stale)) return false;
  const state=S,seq=++IDESK.seq;
  IDESK.loading=true;IDESK.stale=true;IDESK.error="";industryRender();
  try {
    const data=await api(`/api/industry?session_id=${encodeURIComponent(state.session_id)}`);
    if(seq!==IDESK.seq||S!==state||!industryActive()) return false;
    if(!data||data.session_id!==state.session_id||data.nation!==state.player||!Array.isArray(data.sites)||!Array.isArray(data.goods)||!Array.isArray(data.queue)) throw new Error("The industry reading did not match this campaign. Retry to read the current operations.");
    IDESK.data=data;IDESK.state=state;IDESK.stale=false;return true;
  } catch(error) {
    if(seq===IDESK.seq&&S===state&&industryActive()){IDESK.error=error.message||"The operations service could not be reached.";IDESK.stale=true;}
    return false;
  } finally {
    if(seq===IDESK.seq){IDESK.loading=false;industryRender();}
  }
}
function industryOnStateChanged() {
  industryResetCampaign();
  IDESK.stale=true;IDESK.state=null;IDESK.error="";IDESK.loading=false;++IDESK.seq;
  if(industryActive()){industryRender();industryFetch();}
}
function industryResetCampaign() {
  if(IDESK.session===S?.session_id&&IDESK.nation===S?.player) return;
  IDESK.data=null;IDESK.state=null;IDESK.session=S?.session_id;IDESK.nation=S?.player;
  IDESK.query="";IDESK.filter="all";IDESK.details.clear();IDESK.focus=null;IDESK.scroll=0;
  IDESK.stale=true;IDESK.loading=false;++IDESK.seq;
}
function industryClose() {
  industryRememberView();IDESK.open=false;IDESK.loading=false;IDESK.stale=true;IDESK.focus=null;++IDESK.seq;
}
function openIndustry(options={}) {
  if(typeof S==="undefined"||!S?.player) return false;
  industryResetCampaign();
  if(typeof options.query==="string"||typeof options.district==="string") IDESK.query=options.query??options.district;
  if(INDUSTRY_FILTERS.some(([key])=>key===options.filter)) IDESK.filter=options.filter;
  IDESK.open=true;IDESK.error="";openIndustryCabinet();industryBind();return true;
}
window.openIndustry=openIndustry;
