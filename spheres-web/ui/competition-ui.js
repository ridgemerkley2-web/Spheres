/* The Exchange paints Rust read models. No prices, GDP effects, production
   recipes, AI decisions or dependency scores are reconstructed in JavaScript. */
const COMP = { open:false, tab:"industry", data:null, error:"", loading:false, seq:0,
  stale:true, busy:false, lastFocus:null, quotes:null, quoteSeq:0, searching:false,
  trade:{good:"intermediates",quantity:10,delivery_days:90}, filter:"", tier:"", pending:null,
  moduleOpen:false,moduleDistrict:"",moduleQuotes:null,moduleSeq:0,moduleLoading:false,
  materialOpen:false,materialDraft:null,materialQuote:null,materialSeq:0,materialLoading:false };
function competitionText(value) { return String(value??"").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#39;"); }
function competitionNumber(value) { return Number.isFinite(value) ? (value===0?0:value).toLocaleString("en-US",{maximumSignificantDigits:5}) : "—"; }
function competitionMoney(value) { return Number.isFinite(value) ? economyMoney(value) : "Books not opened"; }
function competitionName(id) { return S?.nations?.find(n=>n.id===id)?.name || String(id??"Unknown country"); }
function competitionGood(good) { return good==="capital_goods" ? "Capital goods" : "Intermediate packs"; }
function competitionDistrict(id) { return typeof DINDEX!=="undefined" && DINDEX[id]?.name || id; }
function competitionMetric(label,value) { return `<div><dt>${competitionText(label)}</dt><dd>${competitionText(value)}</dd></div>`; }
function competitionButton(action,label,attrs="",primary=false) { return `<button type="button" data-comp-action="${action}" ${attrs} class="${primary?"comp-primary":""}" ${COMP.busy||COMP.pending?"disabled":""}>${label}</button>`; }
function competitionBadge(status) {
  const warning=["blocked","waiting","limited","awaiting_dispatch","struggling","idle","replenish"].some(s=>String(status).includes(s));
  return `<span class="comp-status ${warning?"warn":""}">${competitionText(String(status||"ready").replace(/_/g," "))}</span>`;
}
function competitionArt() { return `<svg class="comp-art" viewBox="0 0 340 230" aria-hidden="true"><circle cx="152" cy="112" r="87" fill="#274651" stroke="#adcfc1" stroke-width="2"/><ellipse cx="152" cy="112" rx="45" ry="87" fill="none" stroke="#89b4ac" opacity=".6"/><path d="M69 82h166M66 136h171M152 25v174" fill="none" stroke="#89b4ac" opacity=".6"/><path d="M45 163Q147 10 290 77M58 49Q186 222 300 158" fill="none" stroke="#e3c995" stroke-width="3" stroke-dasharray="5 8"/><circle cx="82" cy="117" r="11" fill="#c6b6df"/><circle cx="211" cy="80" r="10" fill="#e3c995"/><circle cx="190" cy="168" r="9" fill="#b6d8c4"/><path d="M216 131l35-19 36 19v53l-36 20-35-20z" fill="#c6b6df"/><path d="M216 131l35 21 36-21M251 152v52" fill="none" stroke="#635472" stroke-width="2"/><path d="M20 161l28-16 29 16v40l-29 17-28-17z" fill="#d9c28e"/><path d="M20 161l28 17 29-17M48 178v40" fill="none" stroke="#81714f" stroke-width="2"/></svg>`; }
function competitionHero(kicker,title,text) { return `<div class="comp-hero"><div><div class="comp-kicker">${competitionText(kicker)}</div><h1>${competitionText(title)}</h1><p>${competitionText(text)}</p></div>${competitionArt()}</div>`; }

function competitionModuleHtml(data) {
  const board=data.module_board;if(!board)return "";
  const selected=COMP.moduleDistrict||board.selection?.district;
  const response=COMP.moduleQuotes||(selected===board.selection?.district?board.selection:null);
  const quotes=response?.quotes||[];
  return `<section class="comp-workshop" aria-labelledby="moduleHeading"><div class="comp-workshop-heading"><div><span class="comp-kicker">Start small · build something useful</span><h3 id="moduleHeading">Your first workshop. Room to grow.</h3><p>A working production package sized to your budget—not a full-size factory bill.</p></div><span class="comp-workshop-art" aria-hidden="true">▥ <span>✦</span></span></div>
    ${board.coverage_reason?`<p class="comp-note">${competitionText(board.coverage_reason)}</p>`:`<div class="comp-actions">${competitionButton("module-toggle",COMP.moduleOpen?"Close workshop builder":"Choose a workshop size","",true)}${competitionButton("budget","Set the workshop budget")}</div>`}
    ${COMP.moduleOpen&&!board.coverage_reason?`<div class="comp-form"><label>Build in your province<select id="competitionModuleProvince">${(board.provinces||[]).map(p=>`<option value="${competitionText(p.id)}" ${p.id===selected?"selected":""}>${competitionText(p.name)}${p.capacity>0?` · ${competitionNumber(p.capacity)} capacity built`:""}</option>`).join("")}</select></label></div>
    <p>Budget fit uses the current Factories department allocation and available construction capacity. It targets about a year of work and funding, with a 90-day minimum. Raw shortages, competing projects and war can extend it.</p>
    ${COMP.moduleLoading?`<p role="status">Checking this province and its budget…</p>`:`<div class="comp-grid">${quotes.map((q,i)=>`<article class="comp-card comp-module-choice"><span class="comp-kicker">${competitionText(q.label)}</span><h3>${competitionMoney(q.cost_bn)}</h3><p>Installation budget · raw inputs purchased separately</p><dl class="comp-metrics">${competitionMetric("Standard capacity",`${competitionNumber(q.scale*100)}%`)}${competitionMetric("Intermediates / day",competitionNumber(q.output_daily))}${competitionMetric("Earliest at current funding",Number.isFinite(q.lower_bound_days)?`${competitionNumber(Math.ceil(q.lower_bound_days))} days`:"Needs annual funding")}${competitionMetric("Political cost",`${competitionNumber(q.political_cost)} PC`)}</dl>
      <p>Includes a workshop estate, generation, local grid and processing. Output requires stocked inputs and operating funds.</p>${q.reason?`<p class="comp-note">${competitionText(q.reason)}</p>`:competitionButton("module-build","Build this workshop",`data-module-quote="${i}"`,true)}
      <details><summary>What this size needs</summary>${(q.requirements||[]).map(r=>`<p>${competitionText(r.name||r.commodity)} · ${competitionNumber(r.required)} ${competitionText(r.unit||"")} required · ${competitionNumber(r.stock_available)} in stock</p>`).join("")}<p>${competitionText(board.note)}</p></details></article>`).join("")}</div>`}`:""}
    ${(board.projects||[]).length?`<div class="comp-grid">${board.projects.map(p=>`<article class="comp-card">${competitionBadge(p.status)}<h3>${competitionText(p.province?.name||p.province?.id)}</h3><label class="comp-module-progress">Workshop construction · ${competitionNumber(p.progress*100)}%<progress max="1" value="${competitionText(p.progress)}">${competitionNumber(p.progress*100)}%</progress></label><p>${competitionText(p.reason||"Paid work is progressing.")}</p><dl class="comp-metrics">${competitionMetric("Spent so far",competitionMoney(p.finance?.spent_bn))}${competitionMetric("Installation remaining",competitionMoney(p.finance?.remaining_bn))}</dl></article>`).join("")}</div>`:""}
    ${board.legacy_active?`<p class="comp-note">${board.legacy_active} existing full-size project(s) keep their original cost and paid work. Manage those on the production board; cancelling does not refund sunk costs.</p>`:""}</section>`;
}
function competitionCapacityHtml(data) {
  const plan=data.capacity_plan;
  if(!plan||!Array.isArray(plan.goods))return `<section class="comp-capacity" aria-label="Industry planning"><div class="comp-section-heading"><h3>Build what you need</h3></div><p class="comp-empty">Planning data is not available. Refresh the Exchange to see your tracked capacity and demand.</p></section>`;
  const choices=[{good:"intermediates",name:"Materials",symbol:"◈",description:"Intermediate packs for your production chain"},
    {good:"capital_goods",name:"Machinery",symbol:"⚙",description:"Capital-goods packs for upgrades and research"}];
  return `<section class="comp-capacity" aria-labelledby="capacityHeading"><div class="comp-section-heading"><div><span class="comp-kicker">A place for every investment</span><h3 id="capacityHeading">Build what you need</h3></div></div>
    <p>Compare tracked physical assets and planned use—not a 1990 factory census. This is advisory: you can still choose your own construction projects.</p>
    <div class="comp-capacity-grid">${choices.map(choice=>{
      const row=plan.goods.find(g=>g.good===choice.good);
      if(!row)return `<article class="comp-card comp-capacity-card"><h4>${choice.name}</h4><p>Planning data is not available for this good.</p></article>`;
      return `<article class="comp-card comp-capacity-card comp-capacity-${choice.good}"><div class="comp-capacity-title"><div><h4>${choice.name}</h4><p>${choice.description}</p></div><span class="comp-symbol" aria-hidden="true">${choice.symbol}</span></div>
        ${competitionBadge(row.status)}<p class="comp-capacity-reason">${competitionText(row.reason)}</p>
        <dl class="comp-metrics">${competitionMetric("Installed capacity / day",competitionNumber(row.installed_daily))}${competitionMetric("Queued extra / day",competitionNumber(row.committed_daily))}${competitionMetric("Planned demand / day",competitionNumber(row.demand_daily))}</dl>
        <details><summary>Stock, incoming & demand detail</summary><dl class="comp-metrics">${competitionMetric("Packs in stock",competitionNumber(row.stock))}${competitionMetric("Packs incoming",competitionNumber(row.incoming))}${Number.isFinite(row.contracted_daily)?competitionMetric("Domestic contracted / day",competitionNumber(row.contracted_daily)):""}${Number.isFinite(row.contracted_remaining)?competitionMetric("Domestic contracts remaining",competitionNumber(row.contracted_remaining)):""}${competitionMetric("Domestic use / day",competitionNumber(row.domestic_daily))}${competitionMetric("Export demand / day",competitionNumber(row.export_daily))}${competitionMetric("Uncovered capacity / day",competitionNumber(row.expansion_daily))}</dl><p>Incoming is not usable until delivery. Domestic contracts are paid production commitments, not installed factories or goods already in stock. Capacity is potential output, not a promise: inputs, funding, power and storage still limit actual production.</p></details>
      </article>`;
    }).join("")}</div>
    <details><summary>Power & storage behind the plan</summary><dl class="comp-metrics">${competitionMetric("Generation capacity / day",competitionNumber(plan.generation_daily))}${competitionMetric("Generation queued / day",competitionNumber(plan.generation_committed_daily))}${competitionMetric("Planned power use / day",competitionNumber(plan.power_required_daily))}${competitionMetric("Storage per good",competitionNumber(plan.storage))}${competitionMetric("Storage queued per good",competitionNumber(plan.storage_committed))}</dl><p>Power uses modeled units; goods use industrial packs. These tracked capacities do not measure the whole inherited national economy.</p></details>
  </section>`;
}
function competitionSupplyName(good) { return good==="capital_goods" ? "Machinery" : good==="intermediates" ? "Materials" : String(good??"Unknown good").replace(/_/g," "); }
function competitionSupplyForecastHtml(forecast) {
  if(!forecast||!Array.isArray(forecast.lines))return "";
  const choices=[{good:"intermediates",symbol:"◈",description:"Intermediate packs that keep projects and machinery moving"},
    {good:"capital_goods",symbol:"⚙",description:"Machine packs for construction, upgrades and research"}];
  return `<section class="comp-supply" aria-labelledby="supplyForecastHeading"><div class="comp-section-heading"><div><span class="comp-kicker">Your next ${competitionNumber(forecast.horizon_days)} days</span><h3 id="supplyForecastHeading">Keep the production chain supplied</h3></div></div>
    <p>This is the government’s live supply forecast. Coverage counts stock, paid incoming lots, finite domestic contracts and projected recent output—not estimated capacity or free goods.</p>
    <div class="comp-supply-grid">${choices.map(choice=>{const line=forecast.lines.find(row=>row?.good===choice.good);if(!line)return `<article class="comp-card comp-supply-card"><h4>${choice.good==="intermediates"?"Materials":"Machinery"}</h4><p>Forecast data is not available for this good.</p></article>`;return `<article class="comp-card comp-supply-card comp-supply-${choice.good}"><div class="comp-capacity-title"><div><h4>${competitionSupplyName(line.good)}</h4><p>${choice.description}</p></div><span class="comp-symbol" aria-hidden="true">${choice.symbol}</span></div>
      ${competitionBadge(line.status)}<p class="comp-supply-reason">${competitionText(line.reason)}</p>
      <dl class="comp-supply-score">${competitionMetric("Need",`${competitionNumber(line.target)} packs`)}${competitionMetric("Covered",`${competitionNumber(line.coverage)} packs`)}${competitionMetric("Gap",`${competitionNumber(line.shortage)} packs`)}</dl>
      <details><summary>What makes up this forecast</summary><dl class="comp-metrics">${competitionMetric("Operating use / day",competitionNumber(line.operating_daily))}${competitionMetric("Unfinished project need",competitionNumber(line.project_remaining))}${competitionMetric("Startup reserve",competitionNumber(line.startup_reserve))}${competitionMetric("In stock now",competitionNumber(line.stock))}${competitionMetric("Paid imports incoming",competitionNumber(line.imports))}${competitionMetric("Domestic contracts remaining",competitionNumber(line.domestic_contracts))}${competitionMetric("Recent domestic output / day",competitionNumber(line.recent_domestic_daily))}${competitionMetric("Projected domestic output",competitionNumber(line.projected_domestic))}${competitionMetric("Storage capacity",competitionNumber(line.storage_capacity))}${competitionMetric("Storage headroom",competitionNumber(line.storage_headroom))}</dl><p>Snapshot day ${competitionNumber(forecast.as_of_day)} · ${competitionNumber(forecast.horizon_days)}-day horizon. Projected domestic output uses the latest settled positive output; unfinished, blocked and estimated capacity is not called supply.</p></details>
    </article>`;}).join("")}</div></section>`;
}
function competitionStartingIndustryHtml(data) {
  if(!data.starting_industry||typeof data.starting_industry!=="object"||Array.isArray(data.starting_industry)||typeof economyStartingIndustryHtml!=="function")return "";
  const sectors=Array.isArray(data.capacity_plan?.inherited_sectors)?data.capacity_plan.inherited_sectors.filter(s=>s&&typeof s==="object"):[];
  return `<section class="comp-inherited" aria-labelledby="startingIndustryHeading"><div class="comp-section-heading"><div><span class="comp-kicker">Your starting foundation</span><h3 id="startingIndustryHeading">Your inherited industry · 1990 estimates</h3></div></div>
    ${economyStartingIndustryHtml(data.starting_industry,"nation",true)}
    ${sectors.length?`<details class="comp-inherited-plan"><summary>How funded projects fit this starting economy</summary><p>Capacity context in annual value added, not packs. This does not replace actual pack demand: an estimate cannot fill a warehouse or a trade order. Only delivered, paid operating output becomes usable goods. Manual construction remains your choice.</p>
      <div class="comp-inherited-plan-grid">${sectors.map(s=>`<article class="comp-card"><h4>${competitionText(s.name||s.key)}</h4>${competitionBadge(s.status)}<p>${competitionText(s.reason)}</p>
        <dl class="comp-metrics">${competitionMetric("Inherited capacity / year",economyMoney(s.inherited_capacity_annual_bn))}${competitionMetric("Funded capacity / year",economyMoney(s.funded_capacity_annual_bn))}${competitionMetric("Queued extra / year",economyMoney(s.committed_capacity_annual_bn))}${competitionMetric("Current output / year",economyMoney(s.output_annual_bn))}${competitionMetric("Total capacity / year",economyMoney(s.total_capacity_annual_bn))}${competitionMetric("Uncovered annual capacity",economyMoney(s.expansion_annual_bn))}</dl></article>`).join("")}</div></details>`:""}</section>`;
}
function competitionMaterialsSelection(data=COMP.data) {
  const m=data?.materials,provinces=Array.isArray(m?.provinces)?m.provinces:[];
  const province=COMP.materialDraft?provinces.find(p=>p.district===COMP.materialDraft.district):provinces[0];
  const draft=COMP.materialDraft||(province?{district:province.district,quantity:province.recommended_quantity,delivery_days:province.recommended_days}:null);
  const quote=COMP.materialQuote||(!COMP.materialDraft?province?.quote:null);
  const matches=quote&&draft&&quote.district===draft.district&&quote.quantity===draft.quantity&&quote.delivery_days===draft.delivery_days;
  return {province,draft,quote:matches?quote:null};
}
function competitionMaterialsArt() {
  return `<svg class="comp-material-art" viewBox="0 0 230 160" aria-hidden="true"><ellipse cx="119" cy="133" rx="99" ry="18" fill="#152a31"/><path d="M21 93l58-28 57 29-58 31z" fill="#d8c49e"/><path d="M21 93v28l57 29v-25z" fill="#a08e73"/><path d="M78 125v25l58-29V94z" fill="#b6a585"/><path d="M71 85V46l27 15 28-13v42l-23 14z" fill="#a6c8b7"/><path d="M98 61v39l28-10V48z" fill="#7fa39b"/><path d="M82 57V29l12 6v29M111 58V23l12 6v28" fill="none" stroke="#d9dbc4" stroke-width="9"/><path d="M146 96l30-17 30 17-30 18z" fill="#d3c3e7"/><path d="M146 96v27l30 17v-26z" fill="#a698be"/><path d="M176 114v26l30-17V96z" fill="#bdafd0"/><path d="M152 57l5-11 5 11 11 5-11 5-5 11-5-11-11-5z" fill="#e8ce9c"/><circle cx="192" cy="49" r="4" fill="#bfdace"/></svg>`;
}
function competitionMaterialsQuoteHtml(q) {
  if(!q)return `<p class="comp-note">Check your draft to see its current price and requirements. Quotes reserve nothing.</p>`;
  return `<article class="comp-material-quote" aria-label="Materials order quote"><div class="comp-section-heading"><div><span class="comp-kicker">A finite, paid order</span><h4>${competitionNumber(q.quantity)} packs · ${competitionNumber(q.delivery_days)} days</h4></div><span class="comp-status">${competitionNumber(q.political_cost)} PC</span></div>
    <p>Factories convert your inputs into intermediate packs. Payment follows actual work; this is not a purchase of the factory.</p>
    <dl class="comp-metrics">${competitionMetric("Factories · full-order fee",competitionMoney(q.conversion_total_bn))}${competitionMetric("Energy · full-order cost",competitionMoney(q.energy_total_bn))}${competitionMetric("Reserved capacity / day",competitionNumber(q.reserved_daily))}${competitionMetric("Current-supply ceiling · packs",competitionNumber(q.feasible_today))}</dl>
    <p>An upper bound, not a delivery promise. Existing funded plants run first and share these supplies.</p>
    ${q.refusal?`<p class="comp-note">${competitionText(q.refusal)}</p>`:""}
    ${(q.blockers||[]).length?`<div class="comp-material-warning"><strong>Work needs attention</strong><ul>${q.blockers.map(b=>`<li>${competitionText(b)}</li>`).join("")}</ul><p>${q.can_start?"You can place this order now, but only available inputs, power and funding produce goods.":"This order is not ready to place. Resolve the approval requirements and check again."} Any unfinished remainder expires at its deadline.</p></div>`:""}
    <details><summary>Daily funding & raw ingredients</summary><p>The government supplies the raw inputs; their purchase is separate from these conversion and energy costs.</p><dl class="comp-metrics">${competitionMetric("Factories / productive day",competitionMoney(q.conversion_daily_bn))}${competitionMetric("Energy / productive day",competitionMoney(q.energy_daily_bn))}${competitionMetric("Factories funds available",competitionMoney(q.available_conversion_bn))}${competitionMetric("Energy funds available",competitionMoney(q.available_energy_bn))}</dl>
      ${(q.requirements||[]).filter(r=>r&&r.required>0).map(r=>`<p><strong>${competitionText(r.name||r.commodity)}</strong> · ${competitionNumber(r.required)} ${competitionText(r.unit)} per productive day · ${competitionNumber(r.stock_available)} ${competitionText(r.unit)} in stock</p>`).join("")}<p>${competitionText(q.note)}</p></details>
    <div class="comp-actions">${q.can_start?competitionButton("materials-order","Order materials","",true):""}${competitionButton("budget","Fund Factories & Energy")}${competitionButton("resources","Find raw inputs")}</div>
  </article>`;
}
function competitionMaterialsHtml(data) {
  const m=data?.materials;if(!m||typeof m!=="object"||Array.isArray(m))return "";
  const {draft,quote}=competitionMaterialsSelection(data),provinces=Array.isArray(m.provinces)?m.provinces:[];
  const orders=Array.isArray(m.orders)?m.orders:[],active=orders.filter(o=>["pending","running","limited","paused","blocked"].includes(o.status));
  return `<section class="comp-materials" aria-labelledby="materialsHeading"><div class="comp-material-heading"><div><span class="comp-kicker">The Materials studio · operating pilot</span><h3 id="materialsHeading">Materials, made here.</h3><p>Put a slice of your inherited industry to work. Your government supplies the raw inputs and pays for conversion and power.</p></div>${competitionMaterialsArt()}</div>
    <div class="comp-material-stats"><article class="comp-material-stat capacity"><span aria-hidden="true">▥</span><h4>Capacity</h4><strong>${competitionNumber(m.capacity_daily)}</strong><p>Potential packs / day</p></article><article class="comp-material-stat output"><span aria-hidden="true">◈</span><h4>Actual output</h4><strong>${competitionNumber(m.output_daily)}</strong><p>Packs · last settled day</p></article><article class="comp-material-stat demand"><span aria-hidden="true">↗</span><h4>Demand</h4><strong>${competitionNumber(m.demand_daily)}</strong><p>Tracked need / day</p></article></div>
    <div class="comp-material-signal">${m.status?competitionBadge(m.status):""}<p>${competitionText(m.reason||"Refresh to see the current operating position.")}</p></div>
    <p class="comp-material-promise">Capacity is not a promise of free goods. Only completed, paid work reaches your stockpile.</p>
    <div class="comp-actions">${provinces.length?competitionButton("materials-toggle",COMP.materialOpen?"Close order desk":"Make an order","",true):""}${competitionButton("materials-expand","Expand",'title="Build additional paid processing capacity"')}${competitionButton("materials-upgrade","Upgrade",'title="Open automation projects for paid industrial sites"')}${competitionButton("materials-import","Import")}${competitionButton("materials-sell","Sell")}</div>
    ${!provinces.length?`<p class="comp-note">No mapped inherited Materials capacity is available for this pilot. Unallocated industry stays in the national accounts; it is not invented on the map. You can still import or build paid capacity.</p>`:""}
    ${COMP.materialOpen&&draft?`<div class="comp-material-desk"><form id="competitionMaterialsForm" class="comp-form"><label>Your province<select name="district" id="competitionMaterialsProvince">${provinces.map(p=>`<option value="${competitionText(p.district)}" ${p.district===draft.district?"selected":""}>${competitionText(p.name||p.district)}</option>`).join("")}</select></label><label>Packs to make<input name="quantity" type="number" min="0.000000001" step="any" value="${competitionText(draft.quantity)}" required></label><label>Production window · days<input name="delivery_days" type="number" min="${competitionText(m.min_delivery_days)}" max="${competitionText(m.max_delivery_days)}" step="1" value="${competitionText(draft.delivery_days)}" required></label><button id="competitionMaterialsCheck" type="submit" ${COMP.materialLoading||COMP.busy?"disabled":""}>${COMP.materialLoading?"Checking this order…":"Check this order"}</button></form>
      <div id="competitionMaterialsQuote" aria-live="polite">${COMP.materialLoading?`<p role="status">Checking capacity, funding and inputs…</p>`:COMP.stale?`<p class="comp-note">Refresh the Exchange before placing an order.</p>`:competitionMaterialsQuoteHtml(quote)}</div></div>`:""}
    ${active.length?`<div class="comp-section-heading"><h4>On the workbench</h4></div><div class="comp-grid">${active.map(o=>`<article class="comp-card comp-material-order">${competitionBadge(o.status)}<h4>${competitionText(competitionDistrict(o.district))}</h4><strong class="comp-big">${competitionNumber(o.delivered)} packs delivered</strong><p>${competitionNumber(o.remaining)} remain · ${competitionNumber(o.delivery_days)}-day production window</p>${o.reason?`<p class="comp-note">${competitionText(o.reason)}</p>`:""}<dl class="comp-metrics">${competitionMetric("Made last settled day",competitionNumber(o.output_today))}${competitionMetric("Factories paid",competitionMoney(o.spent_conversion_bn))}${competitionMetric("Energy paid",competitionMoney(o.spent_energy_bn))}</dl>${competitionButton("materials-cancel","Cancel remaining work",`data-id="${competitionText(o.id)}"`)}</article>`).join("")}</div>`:""}
    <details class="comp-material-ledger"><summary>Stock, GDP & the pilot’s boundaries</summary><dl class="comp-metrics">${competitionMetric("Materials in stock",competitionNumber(m.stock))}${competitionMetric("Storage limit",competitionNumber(m.storage_capacity))}${competitionMetric("Ordered capacity / day",competitionNumber(m.reserved_daily))}${competitionMetric("Imports / day",competitionNumber(m.imports_daily))}${competitionMetric("Exports / day",competitionNumber(m.exports_daily))}${competitionMetric("Already included in GDP",competitionMoney(m.inherited_gdp_annual_bn))}${competitionMetric("Additional GDP",competitionMoney(m.new_gdp_annual_bn))}</dl><p>GDP figures are annual-equivalent value added, not cash. Observed production replaces the inherited output it makes explicit before any additional output counts.</p><p>Expand and Upgrade open paid construction projects. They do not increase the fixed inherited estimate. Demand is tracked project and trade need, not a model of all household purchases.</p><p>${competitionText(m.note)}</p>
      ${orders.length>active.length?`<h4>Finished orders</h4>${orders.filter(o=>!["pending","running","limited","paused","blocked"].includes(o.status)).map(o=>`<p>${competitionBadge(o.status)} ${competitionText(competitionDistrict(o.district))} · ${competitionNumber(o.delivered)} packs delivered${o.reason?` · ${competitionText(o.reason)}`:""}</p>`).join("")}`:""}</details></section>`;
}
function competitionIndustryHtml(data) {
  const b=data.balance||{}, ind=data.industry||{}, goods=data.commerce?.goods||[];
  return `${competitionHero("Industry • opportunity • influence","Make something the world needs.","Build a productive province. Keep its inputs moving. Sell useful goods, and turn reliable delivery into influence.")}
    <div class="comp-grid"><article class="comp-card"><span>National GDP</span><strong class="comp-big">${competitionMoney(b.gdp_bn)}</strong><p>Annual economic output, not government cash.</p></article>
    <article class="comp-card"><span>Project contribution to GDP</span><strong class="comp-big">${competitionMoney(b.project_gdp_bn)}</strong><p>Current annual-equivalent value added from real project activity.</p></article>
    <article class="comp-card"><span>Available treasury cash</span><strong class="comp-big">${competitionMoney(b.cash_bn)}</strong><p>Manufactured imports reserve cash. They never borrow automatically.</p></article></div>
    <div class="comp-actions">${competitionButton("build","Build in a province →","",true)}${competitionButton("budget","Fund your ministries")}${competitionButton("resources","Secure raw inputs")}</div>
    ${competitionMaterialsHtml(data)}
    ${competitionSupplyForecastHtml(data.supply_forecast)}
    ${competitionStartingIndustryHtml(data)}
    ${competitionCapacityHtml(data)}
    ${competitionModuleHtml(data)}
    <div class="comp-section-heading"><h3>Your industrial stock</h3><span>${competitionNumber(ind.power_used_daily)} / ${competitionNumber(ind.power_capacity_daily)} power used</span></div>
    <div class="comp-grid">${goods.map(g=>`<article class="comp-card"><span class="comp-symbol" aria-hidden="true">${g.good==="capital_goods"?"⚙":"◈"}</span><h3>${competitionText(g.name)}</h3><strong class="comp-big">${competitionNumber(g.stock)} packs</strong><dl class="comp-metrics">${competitionMetric("Current demand",competitionNumber(g.demand))}${competitionMetric("On the way",competitionNumber(g.incoming))}${competitionMetric("Unfilled need",competitionNumber(g.shortage))}</dl>${competitionButton("trade","Find buyers & suppliers →",`data-good="${g.good}"`)}</article>`).join("")}</div>
    <div class="comp-section-heading"><h3>What is running?</h3></div>
    <p class="comp-note">${data.industry_settlement?`Last industry settlement: ${competitionText(data.industry_settlement.label)}. Output and spending below are settled receipts, not a forecast for today.`:"No industry day has settled yet. Output and spending appear after actual work."}</p>
    ${(ind.sites||[]).length ? `<div class="comp-grid">${ind.sites.map(s=>`<article class="comp-card">${competitionBadge(s.status)}<h3>${s.kind==="starter_industry"?"Starter workshop":competitionText(String(s.kind).replace(/_/g," "))}</h3><p>${competitionText(competitionDistrict(s.district))}</p><dl class="comp-metrics">${competitionMetric("Output per day",competitionNumber(s.output_daily))}${competitionMetric("Operating spend / day",competitionMoney(s.cash_spent_daily_bn))}</dl>${s.reason?`<p>${competitionText(s.reason)}</p>`:""}</article>`).join("")}</div>` : `<div class="comp-empty">No completed civilian production lines yet. A starter workshop includes its own power and processing; larger standalone facilities remain on the production board.</div>`}
    <div class="comp-section-heading"><h3>Research workshops</h3></div>
    ${(ind.research_operations||[]).length ? `<div class="comp-grid">${ind.research_operations.map(r=>`<article class="comp-card">${competitionBadge(r.status)}<h3>${competitionText(r.technology_name||"Choose a research project")}</h3><p>${competitionText(competitionDistrict(r.district))}</p><p>${competitionText(r.reason||"Funded prototype work reduces this technology's remaining acquisition bill.")}</p><dl class="comp-metrics">${competitionMetric("Prototype credit",competitionNumber(r.prototype_credit))}${competitionMetric("Spend / day",competitionMoney(r.cash_spent_daily_bn))}</dl></article>`).join("")}</div>` : `<div class="comp-empty">Completed research centers can test prototypes for a specific technology, using Science funding and real industrial equipment.</div>`}
    <details><summary>How production, cash and GDP connect</summary><p>${competitionText(data.note)}</p><p>${competitionText(ind.note)}</p></details>`;
}

function competitionTradeHtml(data) {
  const market=data.commerce||{}, goods=market.goods||[], account=market.account||{};
  const quotes=COMP.quotes?.quotes||[];
  return `${competitionHero("The trading floor","Built here. Needed there.","Buy materials or machinery from a consenting supplier. Keep a reserve for your own projects, then offer the surplus to the world.")}
    <div class="comp-grid"><article class="comp-card"><span>Cash reserved in open orders</span><strong class="comp-big">${competitionMoney(market.escrow_bn)}</strong></article><article class="comp-card"><span>Export receipts to date</span><strong class="comp-big">${competitionMoney(account.exports_received_bn||0)}</strong></article></div>
    ${!data.balance?.on_the_books?`<div class="comp-note">Open your ministry budget before buying or selling manufactured goods. ${competitionButton("budget","Open the Cabinet")}</div>`:""}
    <article class="comp-card"><h3>Find a supplier</h3><form id="competitionQuoteForm" class="comp-form"><label>What do you need?<select name="good"><option value="intermediates" ${COMP.trade.good==="intermediates"?"selected":""}>Intermediate packs</option><option value="capital_goods" ${COMP.trade.good==="capital_goods"?"selected":""}>Capital goods</option></select></label><label>How many packs?<input name="quantity" type="number" min="0.000000001" max="1000000" step="any" value="${competitionText(COMP.trade.quantity)}" required></label><label>Loading window (days)<input name="delivery_days" type="number" min="1" max="365" value="${competitionText(COMP.trade.delivery_days)}" required></label><button class="comp-primary" type="submit" ${COMP.searching?"disabled":""}>${COMP.searching?"Checking routes…":"Find suppliers"}</button></form>
    <p>The loading window is not an instant-delivery promise. Freight takes its actual route time. Quotes reserve nothing until you confirm.</p>
    ${COMP.quotes ? quotes.length ? `<div class="comp-grid">${quotes.map((q,i)=>`<article class="comp-card"><span class="comp-kicker">${competitionNumber(q.quantity)} ${competitionGood(q.good)}</span><h3>${competitionText(competitionName(q.seller))}</h3><strong class="comp-big">${competitionMoney(q.total_price_bn)}</strong><p>${competitionNumber(q.estimated_days)} days in transit after dispatch</p><p>${competitionText(q.reason)}</p>${competitionButton("buy","Reserve this lot · 2 PC",`data-quote="${i}"`,true)}</article>`).join("")}</div>` : `<div class="comp-empty">No affordable, reachable supplier is offering this good right now. Try a smaller lot, fund your treasury, or build local production. AI industries need time to produce real surplus.</div>` : ""}</article>
    <div class="comp-section-heading"><h3>Put your surplus on the market</h3></div><div class="comp-grid">${goods.map(g=>`<article class="comp-card"><h3>${competitionText(g.name)}</h3><p>${competitionNumber(g.stock)} in stock · ${competitionNumber(g.demand)} needed by current work</p><form class="comp-form comp-sale-form" data-good="${g.good}"><label>Keep at home (packs)<input name="reserve" type="number" min="0" max="1000000" step="any" value="${competitionText(g.sale?.reserve??g.demand)}" required></label><label>Ask price (% of reference)<input name="ask" type="number" min="25" max="400" step="any" value="${competitionText((g.sale?.ask_multiplier??1.05)*100)}" required></label><label class="comp-check"><input name="enabled" type="checkbox" ${g.sale?.enabled?"checked":""}> Allow exports</label><button type="submit" ${COMP.busy||COMP.pending?"disabled":""}>Apply export policy</button></form><p>Reference: ${competitionMoney(g.reference_price_bn)} per pack. An offer is not a guaranteed buyer.</p></article>`).join("")}</div>
    ${(market.offers||[]).length?`<div class="comp-section-heading"><h3>Counteroffers</h3></div><div class="comp-grid">${market.offers.map(o=>`<article class="comp-card"><h3>${competitionText(competitionName(o.seller))} → ${competitionText(competitionName(o.buyer))}</h3><p>${competitionNumber(o.quantity)} ${competitionGood(o.good)} at ${competitionMoney(o.unit_price_bn)} each</p><p>${competitionText(o.reason)}</p>${o.buyer===data.nation?competitionButton("accept","Accept counteroffer · 2 PC",`data-id="${o.id}"`):"<p>Awaiting buyer acceptance.</p>"}</article>`).join("")}</div>`:""}
    <div class="comp-section-heading"><h3>Contracts & deliveries</h3></div>
    ${(market.contracts||[]).length?`<div class="comp-grid">${market.contracts.slice().reverse().slice(0,24).map(c=>`<article class="comp-card">${competitionBadge(c.status)}<h3>${competitionGood(c.good)} · #${c.id}</h3><p>${competitionText(competitionName(c.seller))} → ${competitionText(competitionName(c.buyer))}</p><dl class="comp-metrics">${competitionMetric("Ordered",competitionNumber(c.quantity))}${competitionMetric("Delivered",competitionNumber(c.delivered_quantity))}${competitionMetric("Awaiting dispatch",competitionNumber(c.remaining_quantity))}${competitionMetric("Escrow remaining",competitionMoney(c.escrow_bn))}</dl>${c.reason?`<p>${competitionText(c.reason)}</p>`:""}${c.remaining_quantity>0?competitionButton("cancel","Cancel undispatched remainder",`data-id="${c.id}"`):""}</article>`).join("")}</div>`:`<div class="comp-empty">No manufactured-goods contracts yet.</div>`}
    ${(market.cargo||[]).length?`<details open><summary>Paid goods in transit · ${market.cargo.length} shipments</summary>${market.cargo.map(c=>`<p><strong>${competitionNumber(c.quantity)} ${competitionGood(c.good)}</strong> · ${competitionText(competitionName(c.seller))} → ${competitionText(competitionName(c.buyer))}<br>${competitionText(c.hold_reason || `${Math.max(0,c.due_day-data.day)} days to scheduled arrival`)} · ${competitionText(c.route?.bottleneck||"Physical freight route")}</p>`).join("")}</details>`:""}
    <details><summary>The cash and ownership rules</summary><p>${competitionText(market.note)}</p><p>Imports reserved to date: ${competitionMoney(account.imports_reserved_bn||0)}. Refunded undispatched orders: ${competitionMoney(account.imports_refunded_bn||0)}.</p></details>`;
}

function competitionWorldHtml(data) {
  const rows=(data.countries||[]).filter(n=>(!COMP.tier||n.tier===COMP.tier)&&n.name.toLowerCase().includes(COMP.filter.toLowerCase()));
  return `${competitionHero("A world that invests","Every economy has a next move.","Governments face the same costs, materials and limits as you. Read what they are building—and why another country is waiting.")}
    <div class="comp-form"><label>Find a country<input id="competitionFilter" type="search" value="${competitionText(COMP.filter)}" placeholder="Country name"></label><label>Economic size<select id="competitionTier"><option value="">Every size</option>${["Micro","Small","Medium","Large","Major"].map(t=>`<option ${COMP.tier===t?"selected":""}>${t}</option>`).join("")}</select></label></div>
    <p>${rows.length} countries shown. These are live decisions, not a claim that every economy must grow through wars or recessions.</p>
    <div class="comp-table-wrap"><table class="comp-table"><thead><tr><th scope="col">Country</th><th scope="col">Economy</th><th scope="col">Building / completed</th><th scope="col">Current decision</th></tr></thead><tbody>${rows.map(n=>{const plan=n.plan,forecast=plan?.supply_review,lines=Array.isArray(forecast?.lines)?forecast.lines:[];const target=plan?.district||plan?.project_kind?`<p class="comp-world-target"><strong>Next industrial target</strong><br>${competitionText(competitionSupplyName(plan?.project_kind||"project"))}${plan?.district?` · ${competitionText(competitionDistrict(plan.district))}`:""}</p>`:"";const supply=lines.length?`<details class="comp-world-supply"><summary>Supply snapshot · ${competitionNumber(forecast.horizon_days)} days</summary>${lines.map(line=>`<section><div><strong>${competitionText(competitionSupplyName(line.good))}</strong> ${competitionBadge(line.status)}</div><dl class="comp-world-supply-score">${competitionMetric("Need",competitionNumber(line.target))}${competitionMetric("Covered",competitionNumber(line.coverage))}${competitionMetric("Gap",competitionNumber(line.shortage))}</dl><p>${competitionText(line.reason)}</p></section>`).join("")}${plan?.funding?`<section><strong>Funding outlook</strong><dl class="comp-world-supply-score">${competitionMetric("Available authority",competitionMoney(plan.funding.available_authority_bn))}${competitionMetric("Work cost remaining",competitionMoney(plan.funding.remaining_work_cost_bn))}${competitionMetric("Earliest years",competitionNumber(plan.funding.earliest_years))}</dl><p>${competitionText(plan.funding.basis)}</p></section>`:""}<p class="comp-world-asof">Snapshot day ${competitionNumber(forecast.as_of_day)}. This captures supply after the government's recorded review action.</p></details>`:"";return `<tr><td class="comp-country" data-label="Country">${competitionText(n.name)}${n.is_player?" · You":""}</td><td data-label="Economy">${competitionMoney(n.gdp_bn)}<br>${competitionText(n.tier)}</td><td data-label="Building / completed">${n.production?.active??0} active · ${n.production?.completed??0} full-site levels${n.production?.module_provinces?`<p>${competitionNumber(n.production.module_provinces)} workshop province(s)<br>${competitionNumber(n.production.module_capacity)} standard capacity</p>`:""}</td><td class="comp-decision-cell" data-label="Current decision">${competitionBadge(n.is_player?"Your decision":plan?.action||"awaiting first review")}${target}<p>${competitionText(plan?.reason||(n.is_player?"Your cabinet and project decisions remain yours.":"The investment planner has not reviewed this country yet."))}</p>${supply}</td></tr>`;}).join("")}</tbody></table></div>
    <details><summary>How to read progress</summary><p>Full-site levels and fractional workshop capacity are shown separately. A small workshop is not rounded into a full-sized factory. An industrial estate or power grid enables production; it is not automatically a profitable factory. Tests separately check completed capacity and actual output.</p></details>`;
}

function competitionSphereHtml(data) {
  const sphere=data.sphere||{};
  return `${competitionHero("Reliability becomes influence","Build a sphere worth joining.","A strong trading relationship can become formal leadership. Partners need trust, protection and a credible economic patron—and can regain independence.")}
    ${sphere.overlord?`<article class="comp-card"><span class="comp-kicker">Your formal overlord</span><h3>${competitionText(competitionName(sphere.overlord))}</h3><p>You keep your provinces, budget and production. Leaving breaks the formal tie and costs standing and relations; it never starts a war automatically.</p>${competitionButton("leave",`Reassert independence · ${competitionNumber(sphere.exit_cost)} PC`)}</article>`:""}
    ${(sphere.partners||[]).length?`<div class="comp-section-heading"><h3>Your formal partners</h3></div><div class="comp-grid">${sphere.partners.map(id=>{const compact=(sphere.compacts||[]).find(c=>c.partner===id);return `<article class="comp-card"><h3>${competitionText(competitionName(id))}</h3>${competitionBadge(compact?(compact.strained_reviews?"strained compact":"economic compact"):"formal subject")}<p>${competitionText(compact?.reason||"Part of your formal hierarchy. Their economy remains their own.")}</p>${compact?.strained_reviews?`<p>${compact.strained_reviews} of 3 strained reviews before an AI partner may leave.</p>`:""}${competitionButton("release","Release as independent",`data-nation="${competitionText(id)}"`)}</article>`;}).join("")}</div>`:""}
    <div class="comp-section-heading"><h3>Potential partners</h3></div><div class="comp-grid">${(sphere.opportunities||[]).map(q=>`<article class="comp-card">${competitionBadge(q.ready?"ready to consent":"relationship in progress")}<h3>${competitionText(competitionName(q.partner))}</h3><dl class="comp-metrics">${competitionMetric("Dependence on you",`${competitionNumber(q.dependency*100)}%`)}${competitionMetric("Relations",competitionNumber(q.relations))}</dl><p>${competitionText(q.reason)}</p>${q.ready?competitionButton("compact",`Offer compact · ${competitionNumber(q.political_cost)} PC`,`data-nation="${competitionText(q.partner)}"`,true):""}</article>`).join("")}</div>
    ${(sphere.join_opportunities||[]).length?`<details><summary>Choose to join another sphere</summary><p>This makes you formally subordinate and does not count as your world domination.</p>${sphere.join_opportunities.map(q=>`<div class="comp-actions">${competitionButton("join",`Join ${competitionText(competitionName(q.patron))} · ${competitionNumber(q.political_cost)} PC`,`data-nation="${competitionText(q.patron)}"`)}</div>`).join("")}</details>`:""}
    <div class="comp-note">Economic output is not transferred when a compact is signed. World domination still requires every surviving government to fall within your formal hierarchy.</div>`;
}

function competitionRender() {
  if (!COMP.open) return;
  const body=document.getElementById("competitionBody"); if(!body)return;
  body.setAttribute("aria-labelledby",`comp-tab-${COMP.tab}`);
  document.getElementById("competitionDate").textContent=COMP.data?.date||S?.date||"";
  document.querySelectorAll("[data-comp-tab]").forEach(b=>{const on=b.dataset.compTab===COMP.tab;b.setAttribute("aria-selected",String(on));b.tabIndex=on?0:-1;});
  let html=COMP.error?`<div class="comp-error" role="alert">${competitionText(COMP.error)}</div>`:"";
  if(COMP.pending) html+=`<div class="comp-error"><strong>An order is awaiting confirmation.</strong><p>Check its receipt before sending another order. Retrying the same receipt cannot purchase twice.</p><div class="comp-actions"><button type="button" data-comp-retry ${COMP.busy?"disabled":""}>Check order receipt</button><button type="button" data-comp-dismiss ${COMP.busy?"disabled":""}>I reviewed the ledger</button></div></div>`;
  if(!COMP.data) html+=`<div class="comp-empty">${COMP.loading?"Opening the Exchange…":"The Exchange could not load."}<button type="button" data-comp-refresh>Refresh</button></div>`;
  else if(!COMP.data.enabled) html+=`${competitionHero("Economic Competition • review build","Give the world something to build.","Enable AI civilian investment, manufactured trade, funded research workshops and economic compacts in this campaign. Existing saves are not silently upgraded.")}<article class="comp-card"><h3>A shared set of rules</h3><p>Countries gain no free factories, inventory or money. This adds decisions to the daily simulation while preserving your existing world.</p>${competitionButton("enable","Enable Economic Competition","",true)}</article>`;
  else html+=({industry:competitionIndustryHtml,trade:competitionTradeHtml,world:competitionWorldHtml,sphere:competitionSphereHtml}[COMP.tab]||competitionIndustryHtml)(COMP.data);
  const scroll=body.scrollTop;
  body.innerHTML=`<div class="comp-content">${html}</div>`;
  body.scrollTop=scroll;
  competitionWire();
}
function competitionInvalidate() {
  // A previous snapshot must not win after a command or day advance.
  COMP.stale=true;COMP.quotes=null;++COMP.quoteSeq;COMP.searching=false;
  COMP.moduleQuotes=null;COMP.moduleDistrict="";++COMP.moduleSeq;COMP.moduleLoading=false;
  COMP.materialQuote=null;++COMP.materialSeq;COMP.materialLoading=false;
  ++COMP.seq;COMP.loading=false;
}
function competitionApi(path,body) {
  const session=S?.session_id;
  if(!session)return Promise.reject(new Error("Continue a campaign before opening the Exchange."));
  return body===undefined
    ? api(`${path}?session_id=${encodeURIComponent(session)}`)
    : api(path,{...body,session_id:session});
}
async function competitionFetch() {
  if(!COMP.open||COMP.loading)return;
  const seq=++COMP.seq,session=S?.session_id;COMP.loading=true;competitionRender();
  try {const data=await competitionApi("/api/competition");if(seq===COMP.seq&&session===S?.session_id){COMP.data=data;COMP.stale=false;}}
  catch(error){if(seq===COMP.seq)COMP.error=error.message;}
  finally{if(seq===COMP.seq){COMP.loading=false;competitionRender();}}
}
function competitionPendingStore() {
  try{if(COMP.pending)sessionStorage.setItem("spheres.pending-economic-order",JSON.stringify(COMP.pending));else sessionStorage.removeItem("spheres.pending-economic-order");}catch(_){/* The live receipt is still retained when browser storage is unavailable. */}
}
async function competitionSendPending() {
  if(!COMP.pending||COMP.busy)return;
  if(COMP.pending.session_id!==S?.session_id){COMP.error="This pending order belongs to an older campaign. Review its ledger; it will not be sent to the new world.";competitionRender();return;}
  COMP.busy=true;COMP.error="";competitionRender();
  try {
    const result=await api("/api/command",COMP.pending);
    COMP.pending=null;competitionPendingStore();
    COMP.error=(result.errors||[]).join(" ");
    await adopt(result,false);
    COMP.stale=true;
  }catch(error){COMP.error=`${error.message} The order's outcome is not assumed; check its receipt before trying a new order.`;}
  finally{COMP.busy=false;competitionRender();await competitionFetch();}
}
async function competitionCommand(command) {
  if(COMP.busy||COMP.pending)return;
  if(advancing||pendingAdvance){COMP.error="Resolve the pending turn before placing an economic order.";competitionRender();return;}
  COMP.pending={session_id:S.session_id,...nextAdvanceIdentity(),commands:[command]};competitionPendingStore();
  await competitionSendPending();
}
async function competitionFindSuppliers(form) {
  const good=form.elements.good.value,quantity=Number(form.elements.quantity.value),delivery_days=Number(form.elements.delivery_days.value);
  COMP.trade={good,quantity,delivery_days};COMP.searching=true;COMP.error="";const seq=++COMP.quoteSeq;
  competitionRender();
  try{const result=await competitionApi("/api/goods-quotes",COMP.trade);if(seq===COMP.quoteSeq)COMP.quotes=result;}
  catch(e){if(seq===COMP.quoteSeq)COMP.error=e.message;}
  finally{if(seq===COMP.quoteSeq){COMP.searching=false;competitionRender();}}
}
async function competitionFindModule(district) {
  COMP.moduleDistrict=district;COMP.moduleQuotes=null;COMP.moduleLoading=true;COMP.error="";
  const seq=++COMP.moduleSeq,session=S?.session_id;competitionRender();
  try{const result=await competitionApi("/api/industry-module-quotes",{district});if(seq===COMP.moduleSeq&&session===S?.session_id)COMP.moduleQuotes=result;}
  catch(error){if(seq===COMP.moduleSeq)COMP.error=error.message;}
  finally{if(seq===COMP.moduleSeq){COMP.moduleLoading=false;competitionRender();}}
}
function competitionMaterialsDraft(draft) {
  COMP.materialDraft={district:String(draft.district),quantity:Number(draft.quantity),delivery_days:Number(draft.delivery_days)};
  COMP.materialQuote=null;++COMP.materialSeq;COMP.materialLoading=false;
}
async function competitionFindMaterials(draft) {
  const restoreFocus=document.activeElement?.closest?.("#competitionMaterialsForm")?document.activeElement.name||"check":null;
  competitionMaterialsDraft(draft);
  COMP.materialLoading=true;COMP.error="";
  const seq=++COMP.materialSeq,session=S?.session_id,request={...COMP.materialDraft};competitionRender();
  try {
    const result=await competitionApi("/api/materials-quote",request);
    if(seq===COMP.materialSeq&&session===S?.session_id)COMP.materialQuote=result;
  }catch(error){if(seq===COMP.materialSeq&&session===S?.session_id)COMP.error=error.message;}
  finally{if(seq===COMP.materialSeq){COMP.materialLoading=false;competitionRender();
    if(restoreFocus&&session===S?.session_id&&COMP.open&&COMP.tab==="industry"){
      const form=document.getElementById?.("competitionMaterialsForm");
      (form?.elements?.namedItem(restoreFocus)||document.getElementById?.("competitionMaterialsCheck"))?.focus({preventScroll:true});
    }
  }}
}
function competitionWire() {
  const body=document.getElementById("competitionBody");
  body.querySelectorAll("[data-comp-action]").forEach(b=>b.onclick=()=>competitionAction(b));
  body.querySelector("[data-comp-retry]")?.addEventListener("click",competitionSendPending);
  body.querySelector("[data-comp-refresh]")?.addEventListener("click",competitionFetch);
  body.querySelector("[data-comp-dismiss]")?.addEventListener("click",()=>{if(confirm("Have you reviewed the contracts and current treasury? Clearing this notice does not cancel or refund any order.")){COMP.pending=null;COMP.error="";competitionPendingStore();competitionRender();}});
  body.querySelector("#competitionQuoteForm")?.addEventListener("submit",e=>{e.preventDefault();competitionFindSuppliers(e.currentTarget);});
  body.querySelectorAll(".comp-sale-form").forEach(form=>form.onsubmit=e=>{e.preventDefault();competitionCommand({kind:"set_goods_sale",good:form.dataset.good,reserve:Number(form.elements.reserve.value),ask_multiplier:Number(form.elements.ask.value)/100,enabled:form.elements.enabled.checked});});
  body.querySelector("#competitionTier")?.addEventListener("change",e=>{COMP.tier=e.target.value;competitionRender();});
  body.querySelector("#competitionModuleProvince")?.addEventListener("change",e=>competitionFindModule(e.target.value));
  const materialsForm=body.querySelector("#competitionMaterialsForm");
  if(materialsForm){
    const draft=()=>({district:materialsForm.elements.district.value,quantity:Number(materialsForm.elements.quantity.value),delivery_days:Number(materialsForm.elements.delivery_days.value)});
    materialsForm.addEventListener("submit",e=>{e.preventDefault();competitionFindMaterials(draft());});
    materialsForm.addEventListener("input",()=>{
      competitionMaterialsDraft(draft());
      // Do not replace the focused number field while the player is typing.
      const quote=body.querySelector("#competitionMaterialsQuote");if(quote)quote.innerHTML=competitionMaterialsQuoteHtml(null);
      const submit=materialsForm.querySelector('[type="submit"]');if(submit){submit.disabled=COMP.busy;submit.textContent="Check this order";}
    });
    materialsForm.elements.district.addEventListener("change",()=>{
      const p=COMP.data?.materials?.provinces?.find(p=>p.district===materialsForm.elements.district.value);
      if(p)competitionFindMaterials({district:p.district,quantity:p.recommended_quantity,delivery_days:p.recommended_days});
    });
  }
  body.querySelector("#competitionFilter")?.addEventListener("input",e=>{COMP.filter=e.target.value;const pos=e.target.selectionStart;competitionRender();const input=document.getElementById("competitionFilter");input.focus();input.setSelectionRange(pos,pos);});
}
function competitionAction(button) {
  const action=button.dataset.compAction,target=button.dataset.nation;
  if(action==="materials-toggle"){COMP.materialOpen=!COMP.materialOpen;competitionRender();
    document.querySelector?.(COMP.materialOpen?"#competitionMaterialsProvince":'[data-comp-action="materials-toggle"]')?.focus({preventScroll:true});return;}
  if(action==="materials-order"){
    if(COMP.stale||COMP.materialLoading)return;
    const q=competitionMaterialsSelection().quote;if(!q?.can_start||q.refusal)return;
    const warning=q.blockers?.length?` Work is currently limited: ${q.blockers.join(" ")}`:"";
    if(!confirm(`Order ${competitionNumber(q.quantity)} intermediate packs from ${competitionDistrict(q.district)} over ${competitionNumber(q.delivery_days)} days? ${competitionNumber(q.political_cost)} political capital now. If fully made: ${competitionMoney(q.conversion_total_bn)} from Factories and ${competitionMoney(q.energy_total_bn)} from Energy, charged as work happens. You supply the raw inputs. Unfinished work expires.${warning}`))return;
    return competitionCommand({kind:"order_materials",district:q.district,quantity:q.quantity,delivery_days:q.delivery_days});
  }
  if(action==="materials-cancel"){
    if(COMP.stale)return;
    const order=COMP.data?.materials?.orders?.find(o=>o.id===Number(button.dataset.id)&&["pending","running","limited","paused","blocked"].includes(o.status));
    if(order&&confirm("Cancel only the remaining Materials work? Delivered packs stay yours; completed work and the approval cost are not refunded."))return competitionCommand({kind:"cancel_materials_order",order:order.id});
    return;
  }
  if(action==="materials-import"||action==="materials-sell"){
    COMP.tab="trade";COMP.trade.good="intermediates";COMP.quotes=null;competitionRender();
    const destination=document.querySelector(action==="materials-sell"?'.comp-sale-form[data-good="intermediates"]':'#competitionQuoteForm');
    destination?.scrollIntoView({block:"start"});destination?.querySelector("input,select,button")?.focus({preventScroll:true});return;
  }
  if(action==="materials-expand"||action==="materials-upgrade"){
    closeCompetition();openProduction();setProductionMode("build");
    PROD.pickKind=action==="materials-expand"?"processing_plant":"automation";PROD.view="provinces";renderProductionPanel();return;
  }
  if(action==="enable")return competitionCommand({kind:"enable_economic_competition"});
  if(action==="module-toggle"){COMP.moduleOpen=!COMP.moduleOpen;competitionRender();return;}
  if(action==="module-build"){
    if(COMP.moduleLoading||COMP.stale)return;
    const board=COMP.data?.module_board;
    const response=COMP.moduleQuotes||(!COMP.moduleDistrict||COMP.moduleDistrict===board?.selection?.district?board?.selection:null);
    const q=response?.quotes?.[Number(button.dataset.moduleQuote)];
    if(!q?.can_start||q.reason)return;
    if(!confirm(`Build ${competitionNumber(q.scale*100)}% of a standard workshop in ${competitionDistrict(q.district)}? Installation: ${competitionMoney(q.cost_bn)}, plus raw materials and ${competitionNumber(q.political_cost)} political capital. Size is fixed when ordered. No goods arrive until construction finishes and operating inputs are available.`))return;
    return competitionCommand({kind:"start_industry_module",district:q.district,capacity_micros:q.capacity_micros});
  }
  if(action==="trade"){COMP.tab="trade";COMP.trade.good=button.dataset.good||COMP.trade.good;COMP.quotes=null;competitionRender();return;}
  if(action==="buy"){
    const q=COMP.quotes?.quotes?.[Number(button.dataset.quote)];if(!q)return;
    if(!confirm(`Reserve ${competitionNumber(q.quantity)} ${competitionGood(q.good)} from ${competitionName(q.seller)} for ${competitionMoney(q.total_price_bn)} and 2 political capital? Goods become usable only after arrival.`))return;
    return competitionCommand({kind:"propose_goods_trade",target:q.seller,good:q.good,quantity:q.quantity,unit_price_bn:q.unit_price_bn,delivery_days:COMP.quotes.delivery_days});
  }
  if(action==="accept")return competitionCommand({kind:"accept_goods_offer",offer:Number(button.dataset.id)});
  if(action==="cancel"){
    if(confirm("Cancel only the undispatched remainder? Already paid cargo stays in transit; cancellation costs reputation."))return competitionCommand({kind:"cancel_goods_trade",contract:Number(button.dataset.id)});return;
  }
  if(action==="compact"||action==="join"){
    const join=action==="join";
    if(confirm(join?`Join ${competitionName(target)} as a formally subordinate government? You retain your economy and may later leave.`:`Offer formal leadership to ${competitionName(target)}? Their economy stays theirs; the agreement counts toward world domination and can unravel.`))return competitionCommand({kind:join?"join_economic_union":"propose_economic_union",target});return;
  }
  if(action==="leave"||action==="release"){
    if(confirm(action==="leave"?"Reassert independence? This costs political capital, reputation and relations, but does not automatically start a war.":`Release ${competitionName(target)} from your formal hierarchy?`))return competitionCommand(action==="leave"?{kind:"leave_economic_union"}:{kind:"release_subject",target});return;
  }
  closeCompetition();
  if(action==="budget")toggleGameDrawer("cabinetDrawer");
  if(action==="build"){setProductionMode("build");openProduction();}
  if(action==="resources")openStock();
}
function openCompetition() {
  if(!S?.player)return;
  const focus=document.activeElement;
  // Retire map-only introductory guidance when entering this dedicated room.
  const notice=document.getElementById("banner");
  if(notice?.textContent.startsWith("Click any nation on the map"))notice.style.display="none";
  if(keysCardIsOpen())setKeysCard(false);
  closeGameDrawers();if(PROD.open)closeProduction();if(LOGI.open)closeLogistics();
  if(stock.open)closeStock();if(tech.open)closeTech();if(dominationIsOpen())closeDomination();closeSheet();closeTechMenu();
  COMP.open=true;COMP.lastFocus=focus;COMP.error="";
  // Storage is a reload fallback, never authority to discard a live uncertain
  // receipt (private browsing/quota failures can leave it only in memory).
  if(!COMP.pending){try{COMP.pending=JSON.parse(sessionStorage.getItem("spheres.pending-economic-order")||"null");}catch(_){/* Keep live request state unchanged. */}}
  const room=document.getElementById("competitionRoom");room.hidden=false;room.setAttribute("aria-hidden","false");
  document.getElementById("app").inert=true;
  document.getElementById("competitionDockBtn").setAttribute("aria-expanded","true");
  competitionRender();competitionFetch();document.getElementById("competitionClose").focus({preventScroll:true});
}
function closeCompetition() {
  if(!COMP.open)return;
  COMP.open=false;++COMP.seq;COMP.loading=false;++COMP.quoteSeq;COMP.searching=false;
  ++COMP.moduleSeq;COMP.moduleLoading=false;
  ++COMP.materialSeq;COMP.materialLoading=false;
  const room=document.getElementById("competitionRoom");room.hidden=true;room.setAttribute("aria-hidden","true");
  document.getElementById("app").inert=false;
  document.getElementById("competitionDockBtn").setAttribute("aria-expanded","false");
  const focus=COMP.lastFocus?.isConnected?COMP.lastFocus:document.getElementById("competitionDockBtn");focus?.focus({preventScroll:true});COMP.lastFocus=null;
}
function resetCompetition() {closeCompetition();COMP.data=null;COMP.quotes=null;COMP.moduleQuotes=null;COMP.moduleDistrict="";COMP.moduleOpen=false;COMP.materialOpen=false;COMP.materialDraft=null;COMP.materialQuote=null;++COMP.materialSeq;COMP.materialLoading=false;COMP.stale=true;COMP.error="";COMP.filter="";COMP.tier="";}
function competitionSetTab(tab) {if(!["industry","trade","world","sphere"].includes(tab))return;COMP.tab=tab;competitionRender();}
function competitionInit() {
  document.getElementById("competitionDockBtn").onclick=openCompetition;
  document.getElementById("competitionClose").onclick=closeCompetition;
  document.getElementById("competitionRefresh").onclick=()=>{COMP.stale=true;competitionFetch();};
  document.querySelectorAll("[data-comp-tab]").forEach(b=>{b.onclick=()=>competitionSetTab(b.dataset.compTab);b.onkeydown=e=>{if(!["ArrowLeft","ArrowRight","Home","End"].includes(e.key))return;e.preventDefault();const tabs=[...document.querySelectorAll("[data-comp-tab]")],i=tabs.indexOf(b),next=e.key==="Home"?0:e.key==="End"?tabs.length-1:(i+(e.key==="ArrowRight"?1:-1)+tabs.length)%tabs.length;competitionSetTab(tabs[next].dataset.compTab);tabs[next].focus();};});
}
if(typeof document!=="undefined")document.addEventListener("DOMContentLoaded",competitionInit);
