/* Companies are persistent simulation entities. Only the server supplies eligible
   work and receipts; this directory never invents a production or growth bonus. */
"use strict";
function companiesActive(){return CDESK.open&&typeof S!=="undefined"&&!!S?.player&&typeof CAB!=="undefined"&&CAB.tab==="companies"&&(typeof cabinetIsOpen!=="function"||cabinetIsOpen());}
function companiesExternalPending(){return (typeof advancing!=="undefined"&&advancing)||(typeof pendingAdvance!=="undefined"&&!!pendingAdvance)||(typeof COMMAND_CHANNEL!=="undefined"&&!!(COMMAND_CHANNEL.busy||COMMAND_CHANNEL.pending))||(typeof SESSION!=="undefined"&&SESSION.busy)||(typeof CAB!=="undefined"&&CAB.busy)||(typeof PROD!=="undefined"&&PROD.busy)||(typeof COMP!=="undefined"&&!!(COMP.busy||COMP.pending))||(typeof EQUIP!=="undefined"&&EQUIP.busy);}
function companiesPending(){return CDESK.busy||companiesExternalPending();}
function companiesSnapshotCurrent(data=CDESK.data,state=CDESK.state){return companiesActive()&&!!data&&data===CDESK.data&&state===CDESK.state&&state===S&&!CDESK.stale&&!CDESK.loading&&!CDESK.error;}
function companiesCurrent(data=CDESK.data,state=CDESK.state){return companiesSnapshotCurrent(data,state)&&!companiesPending();}
function companyReference(value){return typeof value==="string"&&/^(supplier|contractor):\d+$/.test(value)?value:null;}
function companyRows(){
  const data=CDESK.data;if(!Array.isArray(data?.directory))return [];
  return data.directory.flatMap(row=>{
    const reference=companyReference(row.reference);if(!reference)return [];
    const [kind,id]=reference.split(":");if(kind!==row.kind||String(row.id)!==id)return [];
    const source=(kind==="supplier"?data.suppliers?.firms:data.contractors?.companies)?.find(c=>String(c.id)===id);
    if(!source)return [];
    return [{...source,...row,reference,kind,source,sector:row.sector||(kind==="supplier"?"defense":source.sector)}];
  });
}
function companyAssignments(company){return company.kind==="contractor"?(company.source.assignments||(CDESK.data?.contractors?.assignments||[]).filter(row=>row.company_id===company.id)):[];}
function companyCapacity(company){return Number(company.effective_capacity??company.capacity)||0;}
function companyFreeCapacity(company){return Math.max(0,companyCapacity(company)-(Number.isFinite(company.used_capacity)?company.used_capacity:companyAssignments(company).length));}
function companyProducts(company){return (CDESK.data?.suppliers?.products||[]).filter(row=>String(row.company)===String(company.id));}
function companyOperation(company){return (CDESK.data?.operations?.companies||[]).find(row=>String(row.company)===String(company.id));}
function companyTarget(value){
  if(!value||typeof value!=="object")return null;
  if(["construction","equipment","custom_equipment"].includes(value.kind)&&Number.isSafeInteger(value.project)&&value.project>=0)return {kind:value.kind,project:value.project};
  if(value.kind==="facility"&&typeof value.district==="string"&&["mining","manufacturing","energy","logistics"].includes(value.sector))return {kind:"facility",district:value.district,sector:value.sector};
  if(value.kind==="research"&&typeof value.domain==="string")return {kind:"research",domain:value.domain};return null;
}
function companyTargetKey(value){const clean=companyTarget(value);return clean?JSON.stringify(clean):"";}
function companyTargets(company){return (CDESK.data?.contractors?.targets||[]).filter(row=>row.sector===company.sector&&companyTarget(row.target));}
function companyTargetAvailable(row){return !!row&&row.enabled!==false&&row.current_company_id==null;}
function companyMatches(company){
  const normalize=value=>String(value??"").normalize("NFD").replace(/[\u0300-\u036f]/g,"").toLocaleLowerCase("en-US");
  const text=normalize([company.name,company.home_name,company.home_district,company.specialty,company.sector,company.kind].join(" "));
  const available=company.kind==="supplier"?companyProducts(company).some(p=>p.availability?.ready_stock>0):companyFreeCapacity(company)>0;
  return (CDESK.role==="all"||company.kind===CDESK.role)&&(CDESK.sector==="all"||company.sector===CDESK.sector)&&(!CDESK.onlyAvailable||available)&&normalize(CDESK.query).trim().split(/\s+/).filter(Boolean).every(token=>text.includes(token));
}
function companyMetric(label,value,note=""){return `<div><dt>${companyText(label)}</dt><dd>${companyText(value)}</dd>${note?`<small>${companyText(note)}</small>`:""}</div>`;}
function companyOperatingHtml(company){
  const op=companyOperation(company);if(!op)return '<p class="company-note">Operating requirements are available after company-network adoption.</p>';
  const r=op.readiness||{},p=op.next_packet||{},receipt=op.last_receipt;
  const inputs=(p.inputs||[]).map(i=>`<tr><th scope="row">${companyText(i.name)}</th><td>${companyNumber(i.required)} ${companyText(i.unit)}</td><td>${companyNumber(i.available)}</td><td>${companyNumber(i.missing)}</td></tr>`).join("");
  return `<div class="company-operating"><h4>What this manufacturer needs</h4><p class="company-message">${companyText(op.reason||"Review the next production step below.")}</p>${op.grandfathered?'<p class="company-note">Existing paid work keeps its original terms and owned inputs.</p>':""}<dl class="company-economics">${companyMetric("Spendable company cash",companyMoney(op.company_cash_available_bn),"Settled corporate funds")}${companyMetric("Reserved working capital",companyMoney(op.locked_working_capital_bn),"Separate from spendable cash")}${companyMetric("Customer advances held",companyMoney(op.public_refit_escrow_bn),"Held for refit service")}${companyMetric("Current staffing",companyPercent(r.staffing_fraction),"Readiness, not completed work")}${companyMetric("Available work today",companyPercent(r.work_fraction),r.slot_available?"Existing leased Arms Plant":"Facility unavailable")}${companyMetric("Next company payment",companyMoney(p.company_cash_required_bn),"For the next production step")}</dl><p class="company-note">Power available / required: ${companyNumber(r.power_available)} / ${companyNumber(r.power_required)}. Development funding needed: ${companyMoney(p.public_development_required_bn)}.</p>${inputs?`<div class="company-input-scroll"><table class="company-inputs"><caption>Next step inputs · company-funded purchases from available public stock</caption><thead><tr><th>Input</th><th>Required</th><th>Public stock</th><th>Missing</th></tr></thead><tbody>${inputs}</tbody></table></div>`:""}<p class="company-note">This is a conditional next step. Public supplies are paid for by the company; ready inputs do not mean work has already completed.</p>${receipt?`<p class="company-receipt">Latest paid work${receipt.date?` · ${companyText(receipt.date)}`:""}: ${companyNumber(receipt.work_days)} work days · ${companyNumber(receipt.units)} units · ${companyMoney(receipt.cash_paid_bn)} company payment.</p>`:'<p class="company-note">No completed operating receipt recorded yet.</p>'}</div>`;
}
function companySupplierHtml(company){
  const products=companyProducts(company),ready=products.filter(p=>p.availability?.ready_stock>0),op=companyOperation(company),ref=company.reference,current=companiesCurrent();
  const status=op?.reason||company.source.status;
  return `<p class="company-specialty">${companyText(status)}</p><p class="company-note">State-owned equipment manufacturer. Company cash and unsold inventory remain separate from government equipment.</p><dl class="company-economics">${companyMetric("Products ready to buy",companyNumber(ready.length),"Buy only after reviewing price and quantity")}${companyMetric("Models and supplies",companyNumber(products.length),"Development and certified products")}</dl>${ready.length?`<ul class="company-stock">${ready.slice(0,3).map(p=>`<li><strong>${companyText(p.name)}</strong><span>${companyNumber(p.availability.ready_stock)} ready · ${companyMoney(p.availability.unit_price_bn)} each</span></li>`).join("")}</ul>`:""}<div class="company-actions"><button type="button" class="company-primary" data-company-supplier="${ref}" data-company-tab="companies" ${current?"":"disabled"}>Review stock, funding &amp; service</button><button type="button" data-company-supplier="${ref}" data-company-tab="designer" ${current?"":"disabled"}>Design for this manufacturer</button></div><details class="company-history" data-company-history="${ref}" ${CDESK.histories.has(ref)?"open":""}><summary data-company-focus="history:${ref}">Operating requirements &amp; accounts</summary>${companyOperatingHtml(company)}<dl class="company-economics">${(company.source.metrics||[]).map(m=>companyMetric(m.label,m.value)).join("")}</dl><dl class="company-economics">${(company.source.costs||[]).map(m=>companyMetric(m.label,companyMoney(m.amount_bn),m.period)).join("")}</dl></details>`;
}
function companyContractorHtml(company){
  const ref=company.reference,assignments=companyAssignments(company),targets=companyTargets(company),selected=CDESK.selected.get(ref)||"",chosen=targets.find(row=>companyTargetKey(row.target)===selected),free=companyFreeCapacity(company),current=companiesCurrent();
  const contracts=assignments.map((a,index)=>`<article class="company-contract"><strong>${companyText(a.label||targets.find(t=>companyTargetKey(t.target)===companyTargetKey(a.target))?.label||"Assigned work")}</strong>${a.last_day!=null&&a.last_day>=a.assigned_day?`<p class="company-receipt">Latest delivered work${a.receipt_label?` · ${companyText(a.receipt_label)}`:""}: ${companyNumber(a.work_today)} · ${companyMoney(a.fees_today_bn)} service fees</p>`:'<p class="company-note">Results appear after funded work is delivered.</p>'}<button type="button" data-company-release="${ref}" data-company-assignment="${index}" ${current?"":"disabled"}>Review ending assignment</button></article>`).join("");
  return `<p class="company-specialty">${companyText(company.specialty)}</p><p class="company-note">Modeled domestic service specialist. Nationality does not imply government ownership.</p><div class="company-traits"><p><span><strong>Advantage</strong>${companyText(company.strength||company.description)}</span></p><p><span><strong>Tradeoff</strong>${companyText(company.weakness)}</span></p></div><dl class="company-economics">${companyMetric("Eligible work improvement",companyPercent(company.effective_work_bonus??company.work_bonus,true))}${companyMetric("Eligible input savings",companyPercent(company.effective_input_saving??company.input_saving,true))}${companyMetric("Service fee",companyPercent(company.fee_rate),"Only for delivered work")}${companyMetric("Earned service fees",companyMoney(company.total_fees_bn),"Cumulative receipts, not a cash balance")}${companyMetric("Available assignments",`${free} / ${companyCapacity(company)}`)}${companyMetric("Experience",companyNumber(company.experience),company.level||"Established")}</dl><p class="company-note">${companyText(company.fee_basis)}</p>${contracts}<label for="company-target-${ref}">Choose compatible work</label><select id="company-target-${ref}" data-company-target="${ref}" data-company-focus="target:${ref}" ${current&&free>0?"":"disabled"}><option value="">Choose a project or operation…</option>${targets.map(row=>`<option value="${companyText(companyTargetKey(row.target))}" ${selected===companyTargetKey(row.target)?"selected":""} ${companyTargetAvailable(row)?"":"disabled"}>${companyText(row.label)}${row.current_company_id!=null?" · already assigned":row.enabled===false?` · ${companyText(row.reason||"Unavailable")}`:""}</option>`).join("")}</select>${!targets.length?'<p class="company-note">Start compatible public work, then return to review an assignment.</p>':""}<button type="button" class="company-primary" data-company-assign="${ref}" ${current&&free>0&&companyTargetAvailable(chosen)?"":"disabled"}>Review assignment</button>`;
}
function companyCardHtml(company){return `<article class="company-card" data-company-record="${company.reference}"><header>${companyLogo(company)}<div class="company-identity"><p class="company-kicker">${companyText(companySectorLabel(company.sector))} · ${company.kind==="supplier"?"Manufacturer":"Service specialist"}</p><h3>${companyText(company.name)}</h3><p class="company-home">${companyText(company.home_name||company.home_district||CDESK.data?.name)}${company.founded_label?` · Est. ${companyText(company.founded_label)}`:""}</p></div></header>${company.kind==="supplier"?companySupplierHtml(company):companyContractorHtml(company)}</article>`;}
function companyReviewHtml(){const review=CDESK.review;if(!review)return "";const q=review.value;
  return `<section class="company-review" aria-labelledby="company-review-title"><p class="company-kicker">Current campaign review</p><h2 id="company-review-title" tabindex="-1">${companyText(q.title)}</h2><p>${companyText(q.note)}</p>${q.quote?`<dl class="company-economics">${companyMetric("Reviewed work rate",Number.isFinite(q.quote.work_rate)?`${companyNumber(q.quote.work_rate)}×`:"—")}${companyMetric("Reviewed input rate",Number.isFinite(q.quote.input_rate)?`${companyNumber(q.quote.input_rate)}×`:"—")}${companyMetric("Service fee",companyPercent(q.quote.fee_rate))}</dl><p>${companyText(q.quote.fee_basis)}</p>`:""}${q.reason?`<p role="alert">${companyText(q.reason)}</p>`:""}<div class="company-actions"><button type="button" data-company-confirm class="company-primary" ${q.valid&&companiesCurrent(review.data,review.state)?"":"disabled"}>Confirm reviewed action</button><button type="button" data-company-cancel ${CDESK.busy?"disabled":""}>Close review</button></div></section>`;}
function companiesContentHtml(){
  const data=CDESK.data,rows=companyRows(),shown=rows.filter(companyMatches),suppliers=rows.filter(c=>c.kind==="supplier").length;
  const hero=`<header class="company-hero"><div><p class="company-kicker">${companyText(data?.name||"Your country")} · Company directory</p><h1>Your companies.<br><em>Your next move.</em></h1><p>Commission equipment. Buy finished stock.<br>Choose specialists for funded public work.</p></div><div class="company-market"><strong>${data?rows.length:"—"}<span>companies</span></strong><div><span>${suppliers} manufacturers</span><span>${rows.length-suppliers} specialists</span></div></div></header>`;
  const status=CDESK.error?`<p class="company-message error" role="alert">${companyText(CDESK.error)} <button type="button" data-company-refresh>Retry</button></p>`:CDESK.loading||CDESK.stale?'<p class="company-message" role="status">Updating company availability…</p>':companiesPending()?'<p class="company-message" role="status">Waiting for the current review, order or turn…</p>':"";
  const message=CDESK.message?`<p class="company-message" role="status">${companyText(CDESK.message)}</p>`:"";
  if(!data)return hero+status+message;
  const adoption=data.enabled?"":`<section class="company-legacy"><h2>Connect the company network</h2><p>Add domestic service specialists and operating requirements for future supplier work. Review the change before adopting it. Existing supplier stock and contracts retain their property.</p><button type="button" data-company-enable class="company-primary" ${companiesCurrent()?"":"disabled"}>Review company-network adoption</button></section>`;
  return hero+status+message+companyReviewHtml()+adoption+`<section class="company-directory" aria-labelledby="company-directory-title"><div class="company-section-heading"><h2 id="company-directory-title">Find your next opportunity</h2><div class="company-actions"><button type="button" data-company-equipment ${companiesPending()?"disabled":""}>Equipment procurement</button><button type="button" data-company-refresh ${CDESK.loading||companiesPending()?"disabled":""}>Refresh</button></div></div><div class="company-toolbar"><label class="company-search" for="company-search"><span>Search companies</span><input type="search" id="company-search" data-company-focus="search" value="${companyText(CDESK.query)}" placeholder="Name, specialty or province…" autocomplete="off"></label><label class="company-available"><input type="checkbox" data-company-available ${CDESK.onlyAvailable?"checked":""}>Ready stock or available assignments</label></div><div class="company-filters" role="group" aria-label="Company roles">${[["all","All companies"],["supplier","Equipment manufacturers"],["contractor","Service specialists"]].map(([id,label])=>`<button type="button" data-company-role="${id}" aria-pressed="${CDESK.role===id}">${label}</button>`).join("")}</div><div class="company-filters" role="group" aria-label="Company sectors">${[["all","All sectors"],...COMPANY_SECTORS].map(([id,label])=>`<button type="button" data-company-sector="${id}" aria-pressed="${CDESK.sector===id}">${label}</button>`).join("")}</div><p class="company-results" role="status">${shown.length} of ${rows.length} companies shown</p><div id="company-list" class="company-grid">${shown.map(companyCardHtml).join("")||'<div class="company-empty"><h3>No matching companies</h3><p>Clear the filters or review equipment procurement to establish your first manufacturer.</p><button type="button" data-company-reset>Clear filters</button></div>'}</div></section>`;
}
function companiesRememberView(){if(!companiesActive())return;const root=document.querySelector("#companiesRoot");if(!root)return;const scroller=document.querySelector("#left");if(scroller)CDESK.scroll=scroller.scrollTop;const active=document.activeElement;if(active?.dataset?.companyFocus)CDESK.focus={key:active.dataset.companyFocus,start:active.selectionStart,end:active.selectionEnd};}
function companiesPanelHtml(){companiesRememberView();return `<div id="companiesRoot" class="companies-desk" aria-busy="${CDESK.loading||CDESK.busy}">${companiesContentHtml()}</div>`;}
function companiesRender(){if(!companiesActive())return;const root=document.querySelector("#companiesRoot");if(!root)return;companiesRememberView();root.innerHTML=companiesContentHtml();root.setAttribute("aria-busy",String(CDESK.loading||CDESK.busy));companiesBind(false);}
function companyCommandKey(command){if(!command||typeof command!=="object")return null;if(command.kind==="enable_companies")return JSON.stringify({kind:command.kind});const target=companyTarget(command.target);if(!target)return null;if(command.kind==="unassign_sector_contractor")return JSON.stringify({kind:command.kind,target});if(command.kind==="assign_sector_contractor"&&Number.isInteger(command.company)&&command.company>=0&&command.company<=4294967295)return JSON.stringify({kind:command.kind,company:command.company,target});return null;}
async function companiesReview(command,data=CDESK.data,state=CDESK.state){
  const key=companyCommandKey(command);if(!key||!companiesCurrent(data,state))return false;
  CDESK.busy=true;CDESK.review=null;CDESK.message="";companiesRender();
  try{const value=await api("/api/companies-preview",{session_id:state.session_id,command:JSON.parse(JSON.stringify(command))});
    if(!companiesSnapshotCurrent(data,state)||companiesExternalPending())return false;
    if(!value||value.session_id!==state.session_id||value.nation!==state.player||typeof value.valid!=="boolean"||companyCommandKey(value.command)!==key||command.kind!=="enable_companies"&&(typeof value.command.quote!=="string"||!value.command.quote.length))throw new Error("The review did not match this company action. Refresh and review again.");
    CDESK.review={value,data,state,key};return true;
  }catch(error){if(state===S)CDESK.message=error.message||"Could not review the assignment.";return false;
  }finally{if(state===S){CDESK.busy=false;companiesRender();document.querySelector("#company-review-title")?.focus({preventScroll:true});}}
}
async function companiesConfirm(){
  const review=CDESK.review;if(!review||!review.value.valid||!companiesCurrent(review.data,review.state)||companyCommandKey(review.value.command)!==review.key)return false;
  const state=S;CDESK.busy=true;companiesRender();
  try{const result=await api("/api/command",{commands:[JSON.parse(JSON.stringify(review.value.command))]});
    if(state.session_id!==S?.session_id)return false;
    if(result.command_pending)throw new Error("The order awaits confirmation. Use the pending order controls before sending another.");
    if(result.errors?.length)throw new Error(result.errors.join(" "));
    await adopt(result,false);if(state.session_id!==S?.session_id)return false;
    CDESK.review=null;CDESK.message="Company action recorded. Future work follows the reviewed terms.";return true;
  }catch(error){if(state.session_id===S?.session_id)CDESK.message=error.message||"The action could not be confirmed.";return false;
  }finally{if(state.session_id===S?.session_id){CDESK.busy=false;CDESK.stale=true;companiesRender();if(companiesActive())companiesFetch(true);}}
}
function companiesAssign(reference,data=CDESK.data,state=CDESK.state){if(!companiesCurrent(data,state))return false;const company=companyRows().find(c=>c.reference===reference&&c.kind==="contractor");if(!company||companyFreeCapacity(company)<=0)return false;const target=companyTargets(company).find(row=>companyTargetKey(row.target)===CDESK.selected.get(reference));if(!companyTargetAvailable(target))return false;return companiesReview({kind:"assign_sector_contractor",company:company.id,target:companyTarget(target.target)},data,state);}
function companiesRelease(reference,index,data=CDESK.data,state=CDESK.state){if(!companiesCurrent(data,state)||!Number.isInteger(index)||index<0)return false;const company=companyRows().find(c=>c.reference===reference&&c.kind==="contractor"),target=companyTarget(company&&companyAssignments(company)[index]?.target);return target?companiesReview({kind:"unassign_sector_contractor",target},data,state):false;}
function companiesOpenSupplier(reference,tab,data=CDESK.data,state=CDESK.state){if(!companiesCurrent(data,state)||!["companies","designer"].includes(tab))return false;const company=companyRows().find(c=>c.reference===reference&&c.kind==="supplier");if(!company)return false;return openEquipment({tab,company:company.id});}
function companiesBind(fetchIfNeeded=true){
  if(!companiesActive())return;const root=document.querySelector("#companiesRoot");if(!root)return;const data=CDESK.data,state=CDESK.state;
  const search=root.querySelector("#company-search");if(search)search.oninput=()=>{CDESK.query=search.value;companiesRender();};
  root.querySelectorAll("[data-company-role]").forEach(b=>b.onclick=()=>{if(["all","supplier","contractor"].includes(b.dataset.companyRole)){CDESK.role=b.dataset.companyRole;companiesRender();}});
  root.querySelectorAll("[data-company-sector]").forEach(b=>b.onclick=()=>{if(b.dataset.companySector==="all"||COMPANY_SECTORS.some(([id])=>id===b.dataset.companySector)){CDESK.sector=b.dataset.companySector;companiesRender();}});
  const available=root.querySelector("[data-company-available]");if(available)available.onchange=()=>{CDESK.onlyAvailable=available.checked;companiesRender();};
  root.querySelectorAll("[data-company-refresh]").forEach(b=>b.onclick=()=>{if(!companiesPending())companiesFetch(true);});
  root.querySelectorAll("[data-company-target]").forEach(select=>select.onchange=()=>{if(!companiesCurrent(data,state))return;const company=companyRows().find(c=>c.reference===select.dataset.companyTarget&&c.kind==="contractor");if(!company)return;const row=companyTargets(company).find(t=>companyTargetKey(t.target)===select.value);if(select.value&&!companyTargetAvailable(row))return;CDESK.selected.set(company.reference,select.value);CDESK.review=null;companiesRender();});
  root.querySelectorAll("[data-company-history]").forEach(detail=>detail.ontoggle=()=>{if(detail.open)CDESK.histories.add(detail.dataset.companyHistory);else CDESK.histories.delete(detail.dataset.companyHistory);});
  root.querySelectorAll("[data-company-assign]").forEach(b=>b.onclick=()=>companiesAssign(b.dataset.companyAssign,data,state));
  root.querySelectorAll("[data-company-release]").forEach(b=>b.onclick=()=>companiesRelease(b.dataset.companyRelease,Number(b.dataset.companyAssignment),data,state));
  root.querySelectorAll("[data-company-supplier]").forEach(b=>b.onclick=()=>companiesOpenSupplier(b.dataset.companySupplier,b.dataset.companyTab,data,state));
  root.querySelectorAll("[data-company-equipment]").forEach(b=>b.onclick=()=>{if(companiesCurrent(data,state))openEquipment({tab:"companies",company:null});});
  const enable=root.querySelector("[data-company-enable]");if(enable)enable.onclick=()=>companiesReview({kind:"enable_companies"},data,state);
  const confirm=root.querySelector("[data-company-confirm]");if(confirm)confirm.onclick=companiesConfirm;
  const cancel=root.querySelector("[data-company-cancel]");if(cancel)cancel.onclick=()=>{if(!CDESK.busy){CDESK.review=null;companiesRender();}};
  root.querySelectorAll("[data-company-reset]").forEach(b=>b.onclick=()=>{CDESK.query="";CDESK.role="all";CDESK.sector="all";CDESK.onlyAvailable=false;companiesRender();});
  const scroller=document.querySelector("#left");if(scroller)scroller.scrollTop=CDESK.scroll;
  if(CDESK.focus){const focus=CDESK.focus,control=[...root.querySelectorAll("[data-company-focus]")].find(el=>el.dataset.companyFocus===focus.key);if(control&&!control.disabled){control.focus({preventScroll:true});if(control.type==="search"&&Number.isFinite(focus.start))control.setSelectionRange(focus.start,focus.end??focus.start);CDESK.focus=null;}}
  if(fetchIfNeeded&&CDESK.stale&&!CDESK.loading&&!CDESK.error&&!CDESK.busy)companiesFetch();
}
async function companiesFetch(force=false){
  if(!companiesActive()||CDESK.loading||CDESK.busy||(!force&&!CDESK.stale))return false;const state=S,seq=++CDESK.seq;CDESK.loading=true;CDESK.stale=true;CDESK.review=null;CDESK.error="";companiesRender();
  try{const data=await api(`/api/companies?session_id=${encodeURIComponent(state.session_id)}`);if(seq!==CDESK.seq||S!==state||!companiesActive())return false;
    if(!data||data.session_id!==state.session_id||data.nation!==state.player||!Array.isArray(data.directory)||!Array.isArray(data.contractors?.companies)||!Array.isArray(data.suppliers?.firms))throw new Error("The directory did not match this campaign. Refresh to review current companies.");
    CDESK.data=data;CDESK.state=state;CDESK.stale=false;return true;
  }catch(error){if(seq===CDESK.seq&&S===state&&companiesActive()){CDESK.error=error.message||"The directory could not be reached.";CDESK.stale=true;}return false;
  }finally{if(seq===CDESK.seq){CDESK.loading=false;companiesRender();}}
}
function companiesResetCampaign(){if(CDESK.session===S?.session_id&&CDESK.nation===S?.player)return;Object.assign(CDESK,{data:null,state:null,session:S?.session_id,nation:S?.player,query:"",role:"all",sector:"all",onlyAvailable:false,focus:null,scroll:0,stale:true,loading:false,busy:false,error:"",message:"",review:null});CDESK.selected.clear();CDESK.expanded.clear();CDESK.histories.clear();++CDESK.seq;}
function companiesOnStateChanged(){companiesResetCampaign();CDESK.stale=true;CDESK.state=null;CDESK.review=null;CDESK.error="";CDESK.loading=false;++CDESK.seq;if(companiesActive()){companiesRender();if(!CDESK.busy)companiesFetch();}}
function companiesClose(){companiesRememberView();CDESK.open=false;CDESK.loading=false;CDESK.stale=true;CDESK.review=null;CDESK.focus=null;++CDESK.seq;}
function openCompanies(options={}){if(typeof S==="undefined"||!S?.player)return false;companiesResetCampaign();if(COMPANY_SECTORS.some(([id])=>id===options.sector))CDESK.sector=options.sector;if(["all","supplier","contractor"].includes(options.role))CDESK.role=options.role;CDESK.open=true;CDESK.error="";openCompaniesCabinet();companiesBind();return true;}
window.openCompanies=openCompanies;
const CDESK={open:false,data:null,state:null,session:null,nation:null,seq:0,loading:false,stale:true,error:"",busy:false,message:"",query:"",sector:"all",onlyAvailable:false,role:"all",review:null,selected:new Map(),expanded:new Set(),histories:new Set(),focus:null,scroll:0};
const COMPANY_SECTORS=[["construction","Construction"],["mining","Mining"],["manufacturing","Manufacturing"],["energy","Energy"],["logistics","Logistics"],["research","Research"],["defense","Defense"]];
const COMPANY_PALETTES=[["#e7b46b","#262b32","#fbdfaa"],["#92cbb8","#173b3c","#d5eee2"],["#b8b2e6","#292d51","#e3dff7"],["#df9d85","#472d32","#f4d6c8"],["#8ec6d8","#163e52","#d2ecf2"],["#cbcd8b","#353e27","#edebc1"],["#d1a5c4","#412a44","#f1d9e9"],["#bec8d2","#293342","#edf0ef"]];
function companyText(value){return String(value??"").replace(/[&<>"']/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));}
function companyNumber(value){return Number.isFinite(value)?value.toLocaleString("en-US",{maximumFractionDigits:2}):"—";}
function companyPercent(value,signed=false){return Number.isFinite(value)?`${signed&&value>0?"+":""}${companyNumber(value*100)}%`:"—";}
function companyMoney(value){return Number.isFinite(value)?(typeof economyMoney==="function"?economyMoney(value):`$${companyNumber(value*1e9)}`):"—";}
function companyHash(value){let hash=2166136261;for(const c of String(value)){hash^=c.codePointAt(0);hash=Math.imul(hash,16777619);}return hash>>>0;}
function companySectorLabel(sector){return COMPANY_SECTORS.find(([id])=>id===sector)?.[1]||String(sector||"Specialist");}
function companyLogo(company){
  const identity=`${company.kind==="supplier"?"supplier:":""}${company.nation||""}:${company.id}:${company.name}:${company.logo?.seed??""}`,hash=companyHash(identity);
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
  const identityBits=Number.isInteger(company.id)&&company.kind!=="supplier"?Math.imul(company.id,2654435761)>>>0:hash;
  const signature=Array.from({length:8},(_,i)=>{const n=(identityBits>>>(i*4))&15;return `<path d="M${13+i*6} 67v-${2+n%4}h${2+(n>>>2)}"/>`;}).join("");
  return `<svg class="company-logo" viewBox="0 0 72 86" role="img" aria-label="${companyText(company.name)} company logo" xmlns="http://www.w3.org/2000/svg"><rect width="72" height="86" rx="12" fill="${ink}"/><g fill="none" stroke="${accent}" stroke-width="1" opacity=".7">${frames[(hash>>>9)%frames.length]}</g><g fill="none" stroke="${paper}" stroke-width="${weight}" stroke-linecap="round" stroke-linejoin="round" transform="rotate(${tilt} 36 33)">${symbols[mark]}</g><g stroke="${accent}" fill="none" stroke-width="1.3">${signature}</g><text x="36" y="80" fill="${paper}" font-family="system-ui,sans-serif" font-size="8" font-weight="750" letter-spacing="2" text-anchor="middle">${companyText(initials)}</text></svg>`;
}
