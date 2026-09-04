/* Economic dossiers only display the simulation's ledger. Sector weights,
   project value added and district attribution are never calculated here. */
const ECONOMIC_LEDGER = { generation: 0, entries: new Map(), details: new Map() };
const PROVINCE_DOSSIER_UI = { campaign: 0, views: new Map() };

function economyText(value) {
  return String(value ?? "").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;")
    .replace(/"/g,"&quot;").replace(/'/g,"&#39;");
}

function economyMoney(value) {
  if (!Number.isFinite(value)) return "—";
  if (value === 0) return "$0";
  const magnitude = Math.abs(value);
  const [divisor,suffix] = magnitude >= 1000 ? [1000,"tn"] : magnitude >= 1 ? [1,"bn"]
    : magnitude >= .001 ? [.001,"m"] : magnitude >= .000001 ? [.000001,"k"] : [.000000001,""];
  return "$" + (value/divisor).toLocaleString("en-US",{maximumSignificantDigits:4}) + suffix;
}

function economyPercent(value, signed=false) {
  return Number.isFinite(value) ? (signed && value > 0 ? "+" : "") + (value*100).toFixed(1) + "%" : "—";
}

function economyProjectClass(value) {
  const names = {
    incremental_value_added:"Project value added", inherited_value_added:"Existing project output",
    inherited_activity:"Inherited activity", legacy_imputed:"Inherited activity", legacy_unmodeled:"Inherited activity",
    unpriced_output:"Output · not priced into GDP", enabling_asset:"Enabling capacity", inactive_capacity:"Capacity · not operating",
    pending_order:"Order · not delivered output",
    construction:"Construction work", active_construction:"Construction work", construction_value_added:"Construction work",
    production:"Operating production", industry:"Operating production", industrial_value_added:"Operating production",
    manufacturing:"Delivered manufacturing", manufacturing_value_added:"Delivered manufacturing",
    extraction:"Resource extraction", resource_value_added:"Resource extraction",
    enabling:"Enabling capacity", enabler:"Enabling capacity", enabling_effect:"Enabling capacity",
    military_order:"Order · not delivered output", equipment_order:"Order · not delivered output",
  };
  return names[value] || String(value || "Project effect").replace(/_/g," ");
}

function economySceneHtml() {
  return `<svg class="pe-scene" viewBox="0 0 320 128" aria-hidden="true" focusable="false">
    <path d="M8 111Q98 60 168 96T312 83V125H8Z" fill="#a8d4c3" opacity=".22"/>
    <circle cx="268" cy="30" r="21" fill="#ebcc91" opacity=".65"/>
    <path d="M28 102V61l35-17v17l33-17v58Z" fill="#a8d4c3"/><path d="M35 60V29h12v25" fill="#8bbbad"/>
    <path d="M111 104V34h47v70" fill="#c6b5e3"/><path d="M162 106V63h43v43" fill="#efb4a5"/>
    <path d="M121 48h7m13 0h7m-27 15h7m13 0h7m-27 15h7m13 0h7M38 80h10m14 0h10m12 0h5M173 76h9m9 0h7" stroke="#24333e" stroke-width="7"/>
    <path d="M235 107V60m-19 14 19-21 20 21m-37 10 17-18 18 18" fill="none" stroke="#a8d4c3" stroke-width="8" stroke-linecap="round"/>
    <path d="M18 117h283" stroke="#ebcc91" stroke-width="3" stroke-dasharray="13 8"/>
  </svg>`;
}

function economyReceiptLabel(data) {
  if (data.receipt_date_label) return String(data.receipt_date_label);
  if (data.as_of_label) return String(data.as_of_label);
  return "Latest settled work · not a forecast or the next day's allowance";
}

function economySectorsHtml(sectors) {
  if (!Array.isArray(sectors) || !sectors.length) return `<p class="pe-empty">No sector composition is available for this reading.</p>`;
  return `<div class="pe-sectors">${sectors.map((sector,index)=>{
    const width = Number.isFinite(sector.share) ? Math.max(0,Math.min(100,sector.share*100)) : 0;
    return `<article class="pe-sector pe-tone-${index%5}"><div><h4>${economyText(sector.name || sector.id)}</h4><span>${economyPercent(sector.share)}</span></div>
      <strong>${economyMoney(sector.gdp_bn)}<small> / year</small></strong>
      <div class="pe-meter" aria-hidden="true"><i style="width:${width}%"></i></div></article>`;
  }).join("")}</div>`;
}

function economyProjectsHtml(projects) {
  if (!Array.isArray(projects) || !projects.length) return `<p class="pe-empty">No project contribution in this reading. The inherited economy still produces output.</p>`;
  return `<div class="pe-projects">${projects.map(project=>`<article class="pe-project">
    <div class="pe-project-top"><span class="pe-badge">${economyText(economyProjectClass(project.classification))}</span><span>${economyText(String(project.status || "Recorded").replace(/_/g," "))}</span></div>
    <h4>${economyText(project.name || project.kind || "Project")}</h4>
    ${project.district ? `<p class="pe-place">${economyText(project.district)}</p>` : ""}
    <div class="pe-project-output"><strong>${economyMoney(project.annual_gdp_bn)}</strong><span>${project.counted ? "GDP attributed to this project / year" : "additional direct GDP / year"}</span></div>
    <p>${economyText(project.reason || "The simulation has not supplied an explanation for this effect.")}</p>
    ${["inherited_activity","legacy_imputed","legacy_unmodeled"].includes(project.classification) ? `<p class="pe-note">Already represented in the inherited economy. Its separate inherited value is not estimated here.</p>` : ""}
    ${project.classification === "inherited_value_added" ? `<p class="pe-note">Existing output reclassified from the inherited baseline, not a new GDP bonus.</p>` : ""}
    ${project.counted === false ? `<p class="pe-note">Not an added GDP contribution.</p>` : ""}
    ${project.valuation_basis ? `<p class="pe-note">Valuation: ${economyText(String(project.valuation_basis).replace(/_/g," "))}</p>` : ""}
    <dl class="pe-receipt"><div><dt>Value added / day</dt><dd>${economyMoney(project.daily_value_added_bn)}</dd></div>
      <div><dt>Gross output / day</dt><dd>${economyMoney(project.gross_output_daily_bn)}</dd></div>
      <div><dt>Intermediate inputs / day</dt><dd>${economyMoney(project.intermediate_inputs_daily_bn)}</dd></div>
      ${Number.isFinite(project.output_quantity_daily) ? `<div><dt>Physical output / day</dt><dd>${project.output_quantity_daily.toLocaleString("en-US",{maximumSignificantDigits:4})} ${economyText(project.output_unit)}</dd></div>` : ""}
      ${Number.isFinite(project.payments_daily_bn) ? `<div><dt>Payments / day · not GDP</dt><dd>${economyMoney(project.payments_daily_bn)}</dd></div>` : ""}</dl>
  </article>`).join("")}</div>`;
}

function economicCompositionHtml(data, scope) {
  if (!data || typeof data !== "object") return `<div class="pe-empty">Economic composition is not available on this server yet. No province GDP has been guessed.</div>`;
  const nation = scope === "nation";
  return `<div class="pe-composition">
    <header class="pe-hero"><div><div class="pe-kicker">${nation ? "An economy, made visible" : "The local economic engine"}</div><h3>${nation ? "What powers this nation?" : "What powers this province?"}</h3>
      <div class="pe-total">${economyMoney(data.total_gdp_bn)}</div><p>Annual output · GDP, not government cash</p></div>${economySceneHtml()}</header>
    <div class="pe-parts"><article><span>Inherited economy</span><strong>${economyMoney(data.inherited_gdp_bn)}</strong><small>modeled annual output</small></article>
      <article><span>Project contribution</span><strong>${economyMoney(data.project_gdp_bn)}</strong><small>current annualized value added</small></article></div>
    <p class="pe-caveat">Modeled estimates, not measured historical state accounts. The inherited economy and counted project activity make up this output.</p>
    <details class="pe-details" data-detail-key="economy-sectors"><summary data-econ-focus="economy-sectors">Inside the economy <span>${Array.isArray(data.sectors) ? data.sectors.length : 0} sectors</span></summary>
      ${economySectorsHtml(data.sectors)}<p class="pe-note">Sector totals combine modeled inherited activity and counted project output. They are not ministry budgets or extra GDP bonuses.</p></details>
    <details class="pe-details" data-detail-key="economy-projects"><summary data-econ-focus="economy-projects">Projects &amp; their impact <span>${Array.isArray(data.projects) ? data.projects.length : 0} entries</span></summary>
      <p class="pe-note">${economyText(economyReceiptLabel(data))}. Construction contributes while work is performed; operating plants contribute actual value added. Enabling capacity can have $0 direct GDP. Orders are not delivered military production.</p>
      ${economyProjectsHtml(data.projects)}</details>
    <details class="pe-details pe-method" data-detail-key="economy-method"><summary data-econ-focus="economy-method">How to read this estimate</summary>
      <dl class="pe-receipt"><div><dt>Opening annual output</dt><dd>${economyMoney(data.opening_gdp_bn)}</dd></div><div><dt>Change since opening</dt><dd>${economyPercent(data.change_since_opening,true)}</dd></div></dl>
      <p>${economyText(data.note || "This is a simulation estimate. Its accounting and attribution are supplied by the game engine.")}</p></details>
    ${nation ? economyProvincesHtml(data) : ""}</div>`;
}

function provinceEconomyHtml(reading) {
  if (!reading || reading.loading) return `<section class="pe-province" aria-label="Province economy"><div class="pe-empty" role="status">Reading the province's economic ledger…</div></section>`;
  if (reading.error) return `<section class="pe-province" aria-label="Province economy"><div class="pe-empty" role="status">Economic reading unavailable. ${economyText(reading.error)}</div></section>`;
  return `<section class="pe-province" aria-label="Province economy">${economicCompositionHtml(reading.economy,"province")}</section>`;
}

function economyProvincesHtml(data) {
  const provinces = Array.isArray(data.provinces) ? data.provinces : [];
  const count = Number.isInteger(data.province_count) ? data.province_count : provinces.length;
  return `<details class="pe-details" data-detail-key="economy-provinces"><summary data-econ-focus="economy-provinces">Explore the provinces <span>${count} mapped</span></summary>
    ${Number.isFinite(data.unallocated_gdp_bn) && data.unallocated_gdp_bn !== 0 ? `<p class="pe-note">${economyMoney(data.unallocated_gdp_bn)} of annual output is not assigned to a mapped province. It remains part of the national total.</p>` : ""}
    <div class="pe-provinces">${provinces.map(province=>`<button type="button" data-economy-province="${economyText(province.id)}" ${typeof DINDEX === "undefined" || !DINDEX[province.id] ? "disabled" : ""}>
      <span>${economyText(province.name || province.id)}</span><strong>${economyMoney(province.total_gdp_bn)}<small> / year ↗</small></strong></button>`).join("")}</div>
    ${!provinces.length ? `<p class="pe-empty">No mapped province breakdown. The national total is still shown above.</p>` : ""}</details>`;
}

function economicDetailsState(root) {
  return new Map(root ? [...root.querySelectorAll("details[data-detail-key]")].map(el=>[el.dataset.detailKey,el.open]) : []);
}

function economicRestoreDetails(root, state) {
  if (!root || !state) return;
  root.querySelectorAll("details[data-detail-key]").forEach(el=>{if(state.has(el.dataset.detailKey)) el.open=state.get(el.dataset.detailKey);});
}

// renderMap replaces the whole drawer node on every adopted day. Keep these
// choices outside that node, by district and campaign, including while an
// asynchronous reading temporarily has no economic disclosure to render.
function rememberProvinceDossier() {
  const box=document.querySelector("#provinceDossier");
  const district=box?.dataset.province;
  if (!district || box.dataset.economyCampaign!==String(PROVINCE_DOSSIER_UI.campaign)) return;
  const view=PROVINCE_DOSSIER_UI.views.get(district) || {details:new Map(),scroll:0};
  for (const [key,open] of economicDetailsState(box)) view.details.set(key,open);
  view.scroll=box.scrollTop;
  PROVINCE_DOSSIER_UI.views.set(district,view);
}

function provinceDossierView(district) {
  return PROVINCE_DOSSIER_UI.views.get(district) || {details:new Map(),scroll:0};
}

function wireProvinceDossierState(box) {
  box.dataset.economyCampaign=String(PROVINCE_DOSSIER_UI.campaign);
  box.querySelectorAll("details[data-detail-key]").forEach(details=>{
    details.ontoggle=()=>rememberProvinceDossier();
  });
  box.onscroll=()=>rememberProvinceDossier();
}

function resetProvinceDossierState() {
  ++PROVINCE_DOSSIER_UI.campaign;
  PROVINCE_DOSSIER_UI.views.clear();
  // A response issued by the previous world cannot become the new baseline.
  ++PROVINCE_POPULATION_REQUEST;
  PROVINCE_POPULATION=null;
  selectedDistrict=null;
}

function rememberNationEconomy() {
  const box = document.querySelector("#nationEconomicLedger");
  if (!box?.dataset.nation) return;
  const state=ECONOMIC_LEDGER.details.get(box.dataset.nation) || new Map();
  for (const [key,open] of economicDetailsState(box)) state.set(key,open);
  ECONOMIC_LEDGER.details.set(box.dataset.nation,state);
}

function invalidateEconomicLedger() {
  ++ECONOMIC_LEDGER.generation;
  ECONOMIC_LEDGER.entries.clear();
}

function economicLedgerKey(nation) {
  return JSON.stringify([ECONOMIC_LEDGER.generation,nation,S?.date,S?.year,S?.month,S?.day]);
}

function nationEconomicBody(entry) {
  if (entry?.data) return economicCompositionHtml(entry.data,"nation");
  if (entry?.error) return `<div class="pe-empty" role="status"><strong>Economic ledger unavailable</strong><p>${economyText(entry.error)}</p><button type="button" data-economy-retry>Try again</button></div>`;
  return `<div class="pe-empty" role="status">Reading what this nation's GDP is made of…</div>`;
}

function nationEconomyHtml(nation) {
  const entry=ECONOMIC_LEDGER.entries.get(nation);
  return `<div class="pe-nation" id="nationEconomicLedger" data-nation="${economyText(nation)}">${nationEconomicBody(entry?.key===economicLedgerKey(nation) ? entry : null)}</div>`;
}

function renderNationEconomy(nation,entry) {
  const box=document.querySelector("#nationEconomicLedger");
  if (!box || box.dataset.nation!==nation || selected!==nation) return;
  rememberNationEconomy();
  const focused=box.contains(document.activeElement) ? document.activeElement?.dataset.econFocus : null;
  box.innerHTML=nationEconomicBody(entry);
  box.setAttribute("aria-busy",String(!!entry?.loading));
  economicRestoreDetails(box,ECONOMIC_LEDGER.details.get(nation));
  box.querySelectorAll("[data-economy-province]").forEach(button=>{button.onclick=()=>{
    const id=button.dataset.economyProvince;
    if (!DINDEX[id]) return;
    closeSheet(); showTab("map"); selectProvince(id,true);
  };});
  box.querySelectorAll("[data-economy-retry]").forEach(button=>{button.onclick=()=>fillNationEconomy(nation,true);});
  if (focused) [...box.querySelectorAll("[data-econ-focus]")].find(el=>el.dataset.econFocus===focused)?.focus({preventScroll:true});
}

async function fillNationEconomy(nation, force=false) {
  if (!nation || selected!==nation || nationView!=="economy") return;
  const key=economicLedgerKey(nation), cached=ECONOMIC_LEDGER.entries.get(nation);
  if (!force && cached?.key===key) { renderNationEconomy(nation,cached); return; }
  const entry={key,loading:true,data:null,error:null};
  ECONOMIC_LEDGER.entries.set(nation,entry); renderNationEconomy(nation,entry);
  try {
    const data=await api("/api/economic-ledger/"+encodeURIComponent(nation));
    if (ECONOMIC_LEDGER.entries.get(nation)!==entry || economicLedgerKey(nation)!==key) return;
    entry.data=data;
  } catch(error) {
    if (ECONOMIC_LEDGER.entries.get(nation)!==entry || economicLedgerKey(nation)!==key) return;
    entry.error=error.message || "This server may not yet provide an economic ledger.";
  } finally {
    if (ECONOMIC_LEDGER.entries.get(nation)===entry && economicLedgerKey(nation)===key) {
      entry.loading=false;
      if (selected===nation && nationView==="economy") renderNationEconomy(nation,entry);
    }
  }
}
