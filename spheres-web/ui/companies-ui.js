/* Companies are persistent simulation entities. Only the server supplies eligible
   work and receipts; this directory never invents a production or growth bonus. */
"use strict";
const CDESK={open:false,data:null,state:null,session:null,nation:null,seq:0,loading:false,stale:true,error:"",busy:false,message:"",query:"",sector:"all",onlyAvailable:false,selected:new Map(),expanded:new Set(),histories:new Set(),focus:null,scroll:0};
const COMPANY_SECTORS=[["construction","Construction"],["mining","Mining"],["manufacturing","Manufacturing"],["energy","Energy"],["logistics","Logistics"],["research","Research"],["defense","Defense"]];
const COMPANY_PALETTES=[["#e7b46b","#262b32","#fbdfaa"],["#92cbb8","#173b3c","#d5eee2"],["#b8b2e6","#292d51","#e3dff7"],["#df9d85","#472d32","#f4d6c8"],["#8ec6d8","#163e52","#d2ecf2"],["#cbcd8b","#353e27","#edebc1"],["#d1a5c4","#412a44","#f1d9e9"],["#bec8d2","#293342","#edf0ef"]];
function companyText(value){return String(value??"").replace(/[&<>"']/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));}
function companyNumber(value){return Number.isFinite(value)?value.toLocaleString("en-US",{maximumFractionDigits:2}):"—";}
function companyPercent(value,signed=false){return Number.isFinite(value)?`${signed&&value>0?"+":""}${companyNumber(value*100)}%`:"—";}
function companyMoney(value){return Number.isFinite(value)?(typeof economyMoney==="function"?economyMoney(value):`$${companyNumber(value*1e9)}`):"—";}
function companyHash(value){let hash=2166136261;for(const c of String(value)){hash^=c.codePointAt(0);hash=Math.imul(hash,16777619);}return hash>>>0;}
function companySectorLabel(sector){return COMPANY_SECTORS.find(([id])=>id===sector)?.[1]||String(sector||"Specialist");}
function companyLogo(company){
  const identity=`${company.nation||""}:${company.id}:${company.name}:${company.logo?.seed??""}`,hash=companyHash(identity);
  const palette=COMPANY_PALETTES[Math.abs(Number(company.logo?.palette??hash))%COMPANY_PALETTES.length];
  const [accent,ink,paper]=palette,mark=Math.abs(Number(company.logo?.mark??(hash>>>5)))%16;
  const initials=String(company.logo?.initials||String(company.name||"C").split(/\s+/).map(x=>x[0]).slice(0,3).join("")).slice(0,3);
  const weight=2+(hash%3)*.6,notch=8+((hash>>>12)%9),tilt=((hash>>>17)%3-1)*8;
  const symbols=[
    '<path d="M21 44V26l15-9 15 9v18M28 44V30l8-5 8 5v14M17 44h38"/>',
    '<path d="m18 33 18-17 18 17-18 16ZM26 33l10-10 10 10-10 9Z"/>',
    '<path d="M19 42V29l10 6V23l11 6V17h13v25ZM19 46h34"/>',
    '<path d="m39 15-15 21h12l-3 15 17-23H38Z"/>',
    '<path d="M17 27h29l9 9-9 9H17M25 20h19M25 52h19M17 36h30m-6-6 6 6-6 6"/>',
    '<circle cx="36" cy="33" r="14"/><path d="M22 33h28M36 19v28M25 22l22 22M25 44l22-22"/><circle cx="36" cy="33" r="4"/>',
    '<path d="m36 16 17 7v12c0 9-17 16-17 16S19 44 19 35V23ZM28 33l6 6 12-14"/>',
    '<path d="m18 44 18-27 18 27ZM26 44l10-15 10 15M21 49h30"/>',
    '<path d="M18 24h36M18 34h36M18 44h36M25 20v28M47 20v28"/><circle cx="36" cy="34" r="8"/>',
    '<path d="M20 40c0-26 32-26 32 0M20 40h32M25 40v8m22-8v8M36 18v22"/>',
    '<path d="m20 23 16-8 16 8v21l-16 8-16-8ZM20 23l16 9 16-9M36 32v20"/>',
    '<path d="M19 44V19h11v25M36 44V26h17v18M16 49h40M30 19h13v7"/>',
    '<path d="M20 39c4-20 14-25 16-25s12 5 16 25l-16 11ZM20 39l16-9 16 9M36 14v36"/>',
    '<path d="M18 27h13V16h10v11h13v12H41v11H31V39H18Z"/><path d="M29 34h14"/>',
    '<circle cx="36" cy="34" r="15"/><path d="m24 43 9-19 8 12 7-7M22 49h28"/>',
    '<path d="m17 37 11-19 8 13 8-13 11 19-19 14ZM17 37h38M28 18l8 33 8-33"/>'
  ];
  const frames=[`<path d="M${notch} 7H${72-notch}L65 ${notch}V${64-notch}L${72-notch} 60H${notch}L7 ${64-notch}V${notch}Z"/>`,'<rect x="7" y="7" width="58" height="53" rx="14"/>','<path d="M36 5 65 20v26L36 61 7 46V20Z"/>','<path d="M7 7h58v35c0 13-29 19-29 19S7 55 7 42Z"/>'];
  // The identity is also a visible 32-bit band of eight stepped glyphs: distinct
  // geometry persists even when two firms share a sector, palette and initials.
  // Multiplication by an odd integer permutes the u32 ID space without
  // collisions, guaranteeing every persistent company its own vector band.
  const identityBits=Number.isInteger(company.id)?Math.imul(company.id,2654435761)>>>0:hash;
  const signature=Array.from({length:8},(_,i)=>{const n=(identityBits>>>(i*4))&15;return `<path d="M${13+i*6} 67v-${2+n%4}h${2+(n>>>2)}"/>`;}).join("");
  return `<svg class="company-logo" viewBox="0 0 72 86" role="img" aria-label="${companyText(company.name)} company logo" xmlns="http://www.w3.org/2000/svg"><rect width="72" height="86" rx="12" fill="${ink}"/><g fill="none" stroke="${accent}" stroke-width="1" opacity=".7">${frames[(hash>>>9)%frames.length]}</g><g fill="none" stroke="${paper}" stroke-width="${weight}" stroke-linecap="round" stroke-linejoin="round" transform="rotate(${tilt} 36 33)">${symbols[mark]}</g><g stroke="${accent}" fill="none" stroke-width="1.3">${signature}</g><text x="36" y="80" fill="${paper}" font-family="system-ui,sans-serif" font-size="8" font-weight="750" letter-spacing="2" text-anchor="middle">${companyText(initials)}</text></svg>`;
}
function companiesActive(){return CDESK.open&&typeof S!=="undefined"&&!!S?.player&&typeof CAB!=="undefined"&&CAB.tab==="companies"&&(typeof cabinetIsOpen!=="function"||cabinetIsOpen());}
function companiesExternalPending(){return (typeof advancing!=="undefined"&&advancing)||(typeof pendingAdvance!=="undefined"&&!!pendingAdvance)||(typeof COMMAND_CHANNEL!=="undefined"&&!!(COMMAND_CHANNEL.busy||COMMAND_CHANNEL.pending))||(typeof SESSION!=="undefined"&&SESSION.busy)||(typeof CAB!=="undefined"&&CAB.busy)||(typeof PROD!=="undefined"&&PROD.busy)||(typeof COMP!=="undefined"&&!!(COMP.busy||COMP.pending))||(typeof EQUIP!=="undefined"&&EQUIP.busy);}
function companiesPending(){return CDESK.busy||companiesExternalPending();}
function companiesCurrent(data=CDESK.data,state=CDESK.state){return companiesActive()&&!!data&&data===CDESK.data&&state===CDESK.state&&state===S&&!CDESK.stale&&!CDESK.loading&&!CDESK.error&&!companiesPending();}
function companyRows(){return Array.isArray(CDESK.data?.companies)?CDESK.data.companies:[];}
function companyAssignments(company){return Array.isArray(company.assignments)?company.assignments:(CDESK.data?.assignments||[]).filter(row=>row.company_id===company.id);}
function companyCapacity(company){return Number(company.effective_capacity??company.capacity)||0;}
function companyFreeCapacity(company){return Math.max(0,companyCapacity(company)-(Number.isFinite(company.used_capacity)?company.used_capacity:companyAssignments(company).length));}
function companyTarget(value){
  if(!value||typeof value!=="object")return null;
  if(["construction","equipment","custom_equipment"].includes(value.kind)&&Number.isInteger(value.project)&&value.project>=0)return {kind:value.kind,project:value.project};
  if(value.kind==="facility"&&typeof value.district==="string"&&["mining","manufacturing","energy","logistics"].includes(value.sector))return {kind:"facility",district:value.district,sector:value.sector};
  if(value.kind==="research"&&typeof value.domain==="string")return {kind:"research",domain:value.domain};
  return null;
}
function companyTargetKey(value){const clean=companyTarget(value);return clean?JSON.stringify(clean):"";}
function companyTargetLabel(value){const key=companyTargetKey(value);return (CDESK.data?.targets||[]).find(row=>companyTargetKey(row.target)===key)?.label||value?.district||value?.domain||`${String(value?.kind||"Work").replace(/_/g," ")} #${value?.project??""}`;}
function companyTargets(company){return (CDESK.data?.targets||[]).filter(row=>row.sector===company.sector&&companyTarget(row.target));}
function companyTargetAvailable(row){return !!row&&row.enabled!==false&&row.current_company_id==null;}
function companyMatches(company){const normalize=v=>String(v??"").normalize("NFD").replace(/[\u0300-\u036f]/g,"").toLocaleLowerCase("en-US");const text=normalize([company.name,company.home_name,company.home_district,company.specialty,company.sector].join(" "));return (CDESK.sector==="all"||company.sector===CDESK.sector)&&(!CDESK.onlyAvailable||companyFreeCapacity(company)>0)&&normalize(CDESK.query).trim().split(/\s+/).filter(Boolean).every(token=>text.includes(token));}
function companyMetric(label,value,note=""){return `<div><dt>${companyText(label)}</dt><dd>${companyText(value)}</dd>${note?`<small>${companyText(note)}</small>`:""}</div>`;}
function companyContractHtml(company,assignment,index){
  const hasReceipt=Number.isFinite(assignment.last_day)&&assignment.last_day>=0&&assignment.last_day>=(assignment.assigned_day??0);
  return `<article class="company-contract"><div class="company-contract-head"><strong>${companyText(assignment.label||companyTargetLabel(assignment.target))}</strong><button type="button" data-company-release="${company.id}:${index}" data-company-focus="release:${company.id}:${index}" ${companiesCurrent()?"":"disabled"}>End contract</button></div>${hasReceipt?`<p class="company-receipt">Latest work receipt${assignment.receipt_label?` · ${companyText(assignment.receipt_label)}`:""}</p><dl class="company-receipt-metrics">${companyMetric("Bonus work",companyNumber(assignment.bonus_today))}${companyMetric("Contractor fees",companyMoney(assignment.fees_today_bn))}</dl>${assignment.work_today===0?'<p class="company-note">No delivered work in this receipt. A contractor needs funded activity and the required supplies.</p>':""}`:'<p class="company-note">Assigned. Results will appear after funded work is delivered.</p>'}</article>`;
}
function companyCardHtml(company){
  const esc=companyText,assignments=companyAssignments(company),free=companyFreeCapacity(company),targets=companyTargets(company),open=CDESK.expanded.has(String(company.id));
  const selected=CDESK.selected.get(String(company.id))||"",selectedTarget=targets.find(row=>companyTargetKey(row.target)===selected),current=companiesCurrent(),canAssign=current&&CDESK.data?.enabled!==false&&free>0&&companyTargetAvailable(selectedTarget);
  const level=company.level||"Established",experience=companyNumber(company.experience),strength=company.strength||company.description||company.specialty;
  return `<article class="company-card${assignments.length?" contracted":""}" data-company-record="${company.id}"><header>${companyLogo(company)}<div class="company-identity"><p class="company-kicker">${esc(companySectorLabel(company.sector))}</p><h3>${esc(company.name)}</h3><p class="company-home">${esc(company.home_name||company.home_district||CDESK.data?.name||company.nation)}${company.founded_label?` · Est. ${esc(company.founded_label)}`:""}</p></div></header><p class="company-specialty">${esc(company.specialty)}</p>
    <div class="company-traits"><p><span class="company-trait-icon" aria-hidden="true">+</span><span><strong>Advantage</strong>${esc(strength)}</span></p><p><span class="company-trait-icon caution" aria-hidden="true">↔</span><span><strong>Tradeoff</strong>${esc(company.weakness||"Limited contract capacity; fees apply to delivered work.")}</span></p></div>
    <dl class="company-economics">${companyMetric(company.sector==="logistics"?"Handling capacity":"Work rate",companyPercent(company.effective_work_bonus??company.work_bonus,true),"On eligible work")}${companyMetric((company.effective_input_saving??company.input_saving)<0?"Extra inputs":"Input savings",companyPercent(Math.abs(company.effective_input_saving??company.input_saving)),"On eligible inputs")}${companyMetric("Contractor fee",companyPercent(company.fee_rate),"When work is delivered")}</dl>
    <div class="company-standing"><span><i class="company-capacity-dot ${free>0?"available":""}" aria-hidden="true"></i><strong>${free} / ${companyCapacity(company)} slots free</strong></span><span>${esc(level)} · ${experience} XP</span></div>
    <details class="company-history" data-company-history="${company.id}" ${CDESK.histories.has(String(company.id))?"open":""}><summary data-company-focus="history:${company.id}">Company story &amp; experience</summary><p>${esc(company.origin||company.description||"A domestic specialist, shaped by the country's industries.")}</p><p>${esc(company.experience_note||"Experience comes from delivered work. Continued use improves this company's specialty.")}</p><p><strong>Fee basis:</strong> ${esc(company.fee_basis||"The quoted fee is a share of actual work spending. Idle contracts have no daily charge.")}</p></details>
    ${assignments.length?`<div class="company-contracts"><h4>Active contracts</h4>${assignments.map((row,index)=>companyContractHtml(company,row,index)).join("")}</div>`:""}
    <button type="button" class="company-review-button" data-company-expand="${company.id}" data-company-focus="expand:${company.id}" aria-expanded="${open}" aria-controls="company-hire-${company.id}">${open?"Close assignment":free===0?"View contract options":"Choose work for this company"}<span aria-hidden="true">${open?"−":"↗"}</span></button>
    <div class="company-hire" id="company-hire-${company.id}" ${open?"":"hidden"}>${free===0?'<p class="company-note">All contract slots are occupied. End a current contract to assign different work.</p>':""}${targets.length?`<label for="company-target-${company.id}">Assign to compatible work</label><select id="company-target-${company.id}" data-company-target="${company.id}" data-company-focus="target:${company.id}" ${current&&free>0?"":"disabled"}><option value="">Choose a project or operation…</option>${targets.map(row=>{const key=companyTargetKey(row.target);return `<option value="${esc(key)}" ${selected===key?"selected":""} ${companyTargetAvailable(row)?"":"disabled"}>${esc(row.label)}${row.current_company_id!=null?" · already contracted":row.enabled===false?` · ${esc(row.reason||"unavailable")}`:""}</option>`;}).join("")}</select>${selectedTarget?`<p class="company-assignment-note">${esc(company.name)} will support <strong>${esc(selectedTarget.label)}</strong>. Fees apply when work is delivered. Missing funding or supplies still stop work.</p>`:'<p class="company-note">One lead company per assignment. Contract slots limit simultaneous work.</p>'}<button type="button" class="company-primary" data-company-assign="${company.id}" data-company-focus="assign:${company.id}" ${canAssign?"":"disabled"}>Assign contractor</button>`:`<p class="company-note">No compatible work is available yet. Start a ${esc(company.sector==="defense"?"equipment production":company.sector)} project or operation, then return to assign this specialist.</p>`}</div></article>`;
}
function companyGrowthHtml(){
  const growth=CDESK.data?.growth,rows=Array.isArray(growth)?growth:[],news=Array.isArray(CDESK.data?.news)?CDESK.data.news:[];
  const progress=growth&&!Array.isArray(growth)?`<dl class="company-growth-progress">${companyMetric("Growth observed",`${companyNumber(growth.months_observed)} / ${companyNumber(growth.required_months)} months`)}${companyMetric("Economy expansion",companyPercent(growth.cumulative_growth,true),`${companyPercent(growth.required_growth)} needed over the period`)}${companyMetric("Months of growth",companyNumber(growth.growing_months),"12 growing months needed")}${companyMetric("New entrants",companyNumber(growth.entrants),growth.cooldown_months_remaining>0?`${companyNumber(growth.cooldown_months_remaining)} months until another opportunity`:"Watching for new opportunities")}</dl>${growth.eligible_sector?`<p class="company-note">${companyText(companySectorLabel(growth.eligible_sector))} has qualifying activity. A new entrant may appear as the economy continues growing.</p>`:""}`:"";
  return `<aside class="company-growth"><div class="company-growth-symbol" aria-hidden="true">↗</div><div><p class="company-kicker">Your next generation of companies</p><h2>Growth creates new choices.</h2><p>Company availability follows the size and diversity of your economy. Sustained expansion can produce new specialists with their own names, logos, strengths and tradeoffs.</p>${progress}${rows.length?`<div class="company-growth-sectors">${rows.map(row=>`<div><strong>${companyText(row.label||companySectorLabel(row.sector))}</strong>${Number.isFinite(row.qualifying_months)?`<span>${companyNumber(row.qualifying_months)}${Number.isFinite(row.required_months)?` / ${companyNumber(row.required_months)}`:""} growth months</span>`:""}${row.description?`<p>${companyText(row.description)}</p>`:""}</div>`).join("")}</div>`:""}<p class="company-note">${companyText(CDESK.data?.growth_note||"A temporary spike does not create a new company. Its sector must also deliver growing amounts of actual work. New entrants start with modest capacity and develop through contracts.")}</p>${news.length?`<div class="company-news"><h3>New on the exchange</h3><ul>${news.slice(-4).reverse().map(item=>`<li>${companyText(item.message)}</li>`).join("")}</ul></div>`:""}</div></aside>`;
}
function companiesContentHtml(){
  const data=CDESK.data,rows=companyRows(),shown=rows.filter(companyMatches),active=rows.reduce((sum,c)=>sum+companyAssignments(c).length,0),sectors=new Set(rows.map(c=>c.sector)).size;
  const hero=`<header class="company-hero"><div><p class="company-kicker">The company exchange</p><h1>Find the right<br><em>company for the job.</em></h1><p>Choose specialists. Put their strengths to work.<br>Build the firms that will shape your economy.</p></div><div class="company-market"><p>${companyText(data?.name||"Domestic companies")}</p><strong>${data?rows.length:"—"}<span>companies</span></strong><div><span>${data?sectors:"—"} sectors</span><span>${data?active:"—"} active contracts</span></div></div></header>`;
  const status=CDESK.error?`<div class="company-message error" role="alert"><strong>The company directory could not be refreshed.</strong><p>${companyText(CDESK.error)}</p><button type="button" data-company-refresh data-company-focus="refresh" ${companiesPending()?"disabled":""}>Retry directory</button></div>`:CDESK.loading||CDESK.stale?`<p class="company-message" role="status">${data?"Updating companies and contract availability…":"Loading your domestic companies…"}</p>`:companiesPending()?'<p class="company-message" role="status">Waiting for the current order or turn. Contract changes are temporarily unavailable.</p>':"";
  const message=CDESK.message?`<p class="company-message" role="status" aria-live="polite">${companyText(CDESK.message)}</p>`:"";
  if(!data)return hero+status+message;
  const legacy=data.enabled===false?`<section class="company-legacy"><h2>Introduce companies to this campaign</h2><p>This save predates the company system. Enable domestic companies to add a starting roster based on your economy. Existing work continues, and you choose where to hire specialists.</p><button type="button" class="company-primary" data-company-enable data-company-focus="enable" ${companiesCurrent()?"":"disabled"}>Enable companies</button></section>`:"";
  const toolbar=`<section class="company-directory" aria-labelledby="company-directory-title"><div class="company-section-heading"><div><p class="company-kicker">Domestic specialists</p><h2 id="company-directory-title">Your company directory</h2></div><button type="button" data-company-refresh data-company-focus="refresh" ${CDESK.loading||companiesPending()?"disabled":""}>Refresh directory</button></div><div class="company-toolbar"><label class="company-search" for="company-search"><span>Search companies</span><input type="search" id="company-search" data-company-focus="search" value="${companyText(CDESK.query)}" placeholder="Name, specialty or home province…" autocomplete="off" aria-controls="company-list"></label><label class="company-available"><input type="checkbox" data-company-available data-company-focus="available" ${CDESK.onlyAvailable?"checked":""}>Has contract capacity</label></div><div class="company-filters" role="group" aria-label="Company sectors">${[["all","All sectors"],...COMPANY_SECTORS].map(([id,label])=>{const count=id==="all"?rows.length:rows.filter(c=>c.sector===id).length;return `<button type="button" data-company-sector="${id}" data-company-focus="sector:${id}" aria-pressed="${CDESK.sector===id}">${label}<span>${count}</span></button>`;}).join("")}</div><p class="company-results" role="status" aria-live="polite">${shown.length} of ${rows.length} companies${CDESK.sector!=="all"?` · ${companyText(companySectorLabel(CDESK.sector))}`:""}</p>`;
  const cards=shown.length?`<div id="company-list" class="company-grid">${shown.map(companyCardHtml).join("")}</div>`:`<div id="company-list" class="company-empty"><h3>${rows.length?"No companies match your search":"No companies in this directory yet"}</h3><p>${rows.length?"Try a different specialty or include companies at full capacity.":"Company rosters follow the scale and diversity of the economy. Growth can bring new specialists into the market."}</p>${rows.length?'<button type="button" data-company-reset data-company-focus="reset">Clear search and filters</button>':""}</div>`;
  return hero+status+message+legacy+toolbar+cards+"</section>"+companyGrowthHtml();
}
function companiesRememberView(){
  if(!companiesActive())return;const root=document.querySelector("#companiesRoot");if(!root||root.dataset.companySession!==String(CDESK.session??""))return;
  const scroller=document.querySelector("#left");if(scroller)CDESK.scroll=scroller.scrollTop;
  const active=document.activeElement;if(active?.dataset?.companyFocus)CDESK.focus={key:active.dataset.companyFocus,start:active.selectionStart,end:active.selectionEnd};
}
function companiesPanelHtml(){companiesRememberView();return `<div id="companiesRoot" class="companies-desk" data-company-session="${companyText(CDESK.session)}" aria-busy="${CDESK.loading||CDESK.busy}">${companiesContentHtml()}</div>`;}
function companiesRender(){if(!companiesActive())return;const root=document.querySelector("#companiesRoot");if(!root)return;companiesRememberView();root.innerHTML=companiesContentHtml();root.dataset.companySession=String(CDESK.session??"");root.setAttribute("aria-busy",String(CDESK.loading||CDESK.busy));companiesBind(false);}
async function companiesIssue(command,data=CDESK.data,state=CDESK.state){
  if(!companiesCurrent(data,state))return false;
  if(!command||!["enable_companies","assign_company","unassign_company"].includes(command.kind))return false;
  const session=S.session_id;CDESK.busy=true;CDESK.message="";companiesRender();
  try{const result=await api("/api/command",{commands:[command]});if(session!==S?.session_id)throw new Error("The campaign changed. Review the companies in your current campaign.");if(result.command_pending)throw new Error("This order is awaiting confirmation. Use the pending order controls before sending another.");if(result.errors?.length)throw new Error(result.errors.join(" "));await adopt(result,false);
    if(session!==S?.session_id)return false;CDESK.message=command.kind==="enable_companies"?"Companies are now available. Choose a specialist to assign to your work.":command.kind==="unassign_company"?"Contract ended. Its company slot is available for new work.":"Contract assigned. The company will earn experience and provide its bonus when work is delivered.";return true;
  }catch(error){if(session===S?.session_id)CDESK.message=error.message||"The contract change could not be confirmed.";return false;
  }finally{if(session===S?.session_id){CDESK.busy=false;CDESK.stale=true;companiesRender();if(companiesActive())companiesFetch(true);}}
}
function companiesAssign(id,data=CDESK.data,state=CDESK.state){
  if(!companiesCurrent(data,state)||data.enabled===false)return false;
  const company=companyRows().find(c=>String(c.id)===String(id));if(!company||companyFreeCapacity(company)<=0)return false;
  const row=companyTargets(company).find(target=>companyTargetKey(target.target)===CDESK.selected.get(String(id)));if(!companyTargetAvailable(row))return false;
  return companiesIssue({kind:"assign_company",company:company.id,target:companyTarget(row.target)},data,state);
}
function companiesRelease(id,index,data=CDESK.data,state=CDESK.state){
  if(!companiesCurrent(data,state))return false;const company=companyRows().find(c=>String(c.id)===String(id));if(!company)return false;
  const assignment=companyAssignments(company)[index],target=companyTarget(assignment?.target);if(!target)return false;
  return companiesIssue({kind:"unassign_company",target},data,state);
}
function companiesBind(fetchIfNeeded=true){
  if(!companiesActive())return;const root=document.querySelector("#companiesRoot");if(!root)return;const data=CDESK.data,state=CDESK.state;
  const search=root.querySelector("#company-search");if(search)search.oninput=()=>{CDESK.query=search.value;companiesRender();};
  const available=root.querySelector("[data-company-available]");if(available)available.onchange=()=>{CDESK.onlyAvailable=available.checked;companiesRender();};
  root.querySelectorAll("[data-company-sector]").forEach(button=>button.onclick=()=>{const sector=button.dataset.companySector;if(sector!=="all"&&!COMPANY_SECTORS.some(([id])=>id===sector))return;CDESK.sector=sector;companiesRender();});
  root.querySelectorAll("[data-company-refresh]").forEach(button=>button.onclick=()=>{if(!companiesPending())companiesFetch(true);});
  root.querySelectorAll("[data-company-expand]").forEach(button=>button.onclick=()=>{const id=button.dataset.companyExpand;if(CDESK.expanded.has(id))CDESK.expanded.delete(id);else CDESK.expanded.add(id);companiesRender();});
  root.querySelectorAll("[data-company-history]").forEach(detail=>detail.ontoggle=()=>{const id=detail.dataset.companyHistory;if(detail.open)CDESK.histories.add(id);else CDESK.histories.delete(id);});
  root.querySelectorAll("[data-company-target]").forEach(select=>select.onchange=()=>{if(!companiesCurrent(data,state))return;const company=companyRows().find(c=>String(c.id)===select.dataset.companyTarget);if(!company)return;const row=companyTargets(company).find(row=>companyTargetKey(row.target)===select.value);if(select.value&&!companyTargetAvailable(row))return;CDESK.selected.set(String(company.id),select.value);companiesRender();});
  root.querySelectorAll("[data-company-assign]").forEach(button=>button.onclick=()=>companiesAssign(button.dataset.companyAssign,data,state));
  root.querySelectorAll("[data-company-release]").forEach(button=>button.onclick=()=>{const [id,index]=button.dataset.companyRelease.split(":");return companiesRelease(id,Number(index),data,state);});
  const enable=root.querySelector("[data-company-enable]");if(enable)enable.onclick=()=>{if(data?.enabled===false)companiesIssue({kind:"enable_companies"},data,state);};
  const reset=root.querySelector("[data-company-reset]");if(reset)reset.onclick=()=>{CDESK.query="";CDESK.sector="all";CDESK.onlyAvailable=false;CDESK.focus={key:"search"};companiesRender();};
  const scroller=document.querySelector("#left");if(scroller)scroller.scrollTop=CDESK.scroll;
  if(CDESK.focus){const focus=CDESK.focus,control=[...root.querySelectorAll("[data-company-focus]")].find(el=>el.dataset.companyFocus===focus.key);if(control&&!control.disabled){control.focus({preventScroll:true});if(control.type==="search"&&Number.isFinite(focus.start))control.setSelectionRange(focus.start,focus.end??focus.start);CDESK.focus=null;}}
  if(fetchIfNeeded&&CDESK.stale&&!CDESK.loading&&!CDESK.error)companiesFetch();
}
async function companiesFetch(force=false){
  if(!companiesActive()||CDESK.loading||(!force&&!CDESK.stale))return false;const state=S,seq=++CDESK.seq;CDESK.loading=true;CDESK.stale=true;CDESK.error="";companiesRender();
  try{const data=await api(`/api/companies?session_id=${encodeURIComponent(state.session_id)}`);if(seq!==CDESK.seq||S!==state||!companiesActive())return false;
    if(!data||data.session_id!==state.session_id||data.nation!==state.player||!Array.isArray(data.companies)||!Array.isArray(data.targets))throw new Error("The directory did not match this campaign. Refresh to review its current companies.");
    CDESK.data=data;CDESK.state=state;CDESK.stale=false;return true;
  }catch(error){if(seq===CDESK.seq&&S===state&&companiesActive()){CDESK.error=error.message||"The company service could not be reached.";CDESK.stale=true;}return false;
  }finally{if(seq===CDESK.seq){CDESK.loading=false;companiesRender();}}
}
function companiesResetCampaign(){if(CDESK.session===S?.session_id&&CDESK.nation===S?.player)return;Object.assign(CDESK,{data:null,state:null,session:S?.session_id,nation:S?.player,query:"",sector:"all",onlyAvailable:false,focus:null,scroll:0,stale:true,loading:false,busy:false,error:"",message:""});CDESK.selected.clear();CDESK.expanded.clear();CDESK.histories.clear();++CDESK.seq;}
function companiesOnStateChanged(){companiesResetCampaign();CDESK.stale=true;CDESK.state=null;CDESK.error="";CDESK.loading=false;++CDESK.seq;if(companiesActive()){companiesRender();companiesFetch();}}
function companiesClose(){companiesRememberView();CDESK.open=false;CDESK.loading=false;CDESK.stale=true;CDESK.focus=null;++CDESK.seq;}
function openCompanies(options={}){if(typeof S==="undefined"||!S?.player)return false;companiesResetCampaign();if(COMPANY_SECTORS.some(([id])=>id===options.sector))CDESK.sector=options.sector;CDESK.open=true;CDESK.error="";openCompaniesCabinet();companiesBind();return true;}
window.openCompanies=openCompanies;
