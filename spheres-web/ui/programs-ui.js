/* Ministry programmes: presentation and draft controls only. Available money,
   prices, requirements and actual spending always come from the simulation. */
const PG = { key: null, preview: null, seq: 0, pending: false, error: "", timer: null, department: 0 };
function invalidateProgramPreview() {
  ++PG.seq; PG.key=null; PG.preview=null; PG.error=""; PG.failedKey=null; PG.requestKey=null; PG.pending=false;
}
function programAttr(value) { return escText(String(value ?? "")).replace(/"/g,"&quot;").replace(/'/g,"&#39;"); }

function programPlanOf() {
  const queuedPlan = queued.find(c => c.kind === "program_budget");
  const rows = queuedPlan?.departments || S?.programs?.departments;
  return rows ? rows.map(row => row.slice()) : null;
}

function programBudgetCommand(m) {
  const command = annualBudgetOf(m), departments = programPlanOf();
  return departments ? { ...command, kind: "program_budget", departments } : command;
}

// A UI allocation operation, not an economic effect: move one department's
// share and distribute the remaining basis points across its four siblings.
function programRedistribute(row, selected, value) {
  if (!Array.isArray(row) || row.length !== 5 || !Number.isInteger(selected) || selected < 0 || selected > 4) return null;
  if (Array.from(row).some(x => !Number.isInteger(x) || x < 0) || row.reduce((a,b) => a+b,0) !== 10000 || !Number.isFinite(value)) return null;
  const target = Math.min(10000, Math.max(0, Math.round(value))), remaining = 10000 - target;
  const siblings = row.map((_,i) => i).filter(i => i !== selected), total = siblings.reduce((s,i) => s+row[i],0);
  const out = row.map((x,i) => i === selected ? target : total ? Math.floor(remaining*x/total) : Math.floor(remaining/4));
  let rest = 10000 - out.reduce((a,b) => a+b,0);
  for (const i of siblings) { if (rest-- > 0) out[i]++; }
  return out;
}

function programDraftKey(m) {
  return JSON.stringify([S?.player, S?.date, programBudgetCommand(m)]);
}

function programView(m) {
  return PG.key === programDraftKey(m) && PG.preview ? PG.preview : S?.programs;
}

function programMoney(value) {
  if (!Number.isFinite(value)) return "—";
  const a=Math.abs(value);
  if (a>0 && a<0.001) return a>=0.000001 ? `$${(value*1e6).toFixed(a>=0.00001 ? 0 : 1)}k` : `$${(value*1e9).toFixed(a>=1e-9 ? 0 : 2)}`;
  return fmt.money(value);
}
function programHasDraft() { return queued.some(c => c.kind === "program_budget" || c.kind === "annual_budget"); }

function programDepartmentCard(d, row, plan) {
  const share = plan[d.index] / 100;
  const editable = !!row.editable;
  const selected = row.index === 5 && PG.department === d.index;
  return `<article class="pg-department${selected ? " pg-selected" : ""}">
    <div class="pg-label">${d.kind === "capital" ? "Investment budget" : "Operating services"}</div>
    <h4>${escText(d.name)}</h4><div class="pg-number">${programMoney(d.annual_bn)}<small> / year</small></div>
    <label class="pg-split" for="pgShare-${row.index}-${d.index}"><span>${share.toFixed(1)}% of this ministry</span>
      ${editable ? `<input id="pgShare-${row.index}-${d.index}" type="range" min="0" max="10000" step="1" value="${plan[d.index]}" data-pg-share="${d.index}" aria-label="${programAttr(d.name)} share of ${programAttr(row.name)}" aria-valuetext="${share.toFixed(1)} percent">` : `<progress value="${plan[d.index]}" max="10000" aria-label="${programAttr(d.name)} budget share"></progress>`}</label>
    <p class="pg-meta">${escText(d.description || (d.kind === "capital" ? "Unspent project allowance stays available within the financial year." : "Automatically managed as part of the ministry's service envelope."))}</p>
    <dl class="pg-ledger"><div><dt>Daily allowance</dt><dd>${programMoney(d.daily_bn)}</dd></div><div><dt>Available for work</dt><dd>${d.kind === "capital" ? programMoney(d.available_bn) : "Managed daily"}</dd></div><div><dt>Spent this year</dt><dd>${programMoney(d.spent_ytd_bn)}</dd></div></dl>
    ${row.index === 5 ? `<button type="button" data-pg-department="${d.index}" aria-pressed="${selected}">View investments →</button>` : ""}
  </article>`;
}

function programInvestmentHtml(c, enabled) {
  const ready = enabled && c.enabled;
  return `<article class="pg-investment"><div class="pg-tag">${escText(c.tag || "Province investment")}</div><div class="pg-icon" aria-hidden="true"><svg viewBox="0 0 180 160"><use href="/assets/programs-art.svg#${["factory","power","mine","freight","automation"][c.department] || "factory"}"></use></svg></div>
    <h4>${escText(c.name)}</h4><p>${escText(c.description)}</p><p><strong>${escText(c.effect)}</strong></p>
    <div class="pg-meta">${c.total_days ? `${c.total_days} base work-days · ` : ""}${Number.isFinite(c.pc_cost) ? `${c.pc_cost} political capital` : ""}</div>
    ${Number.isFinite(c.work_cost_bn) ? `<p class="pg-meta">${programMoney(c.work_cost_bn)} total domestic work · paid as built. Materials are additional.</p>` : ""}
    <button type="button" data-pg-invest="${programAttr(c.id)}" ${ready ? "" : "disabled"}>${c.project_kind ? "Choose a province →" : "Choose a deposit →"}</button>
    ${!ready ? `<p class="pg-warning">${escText(!enabled ? "Enact your department budget first." : c.reason || "No eligible province right now. See the production board for requirements.")}</p>` : ""}</article>`;
}

function programBoardHtml(m, ministryIndex) {
  if (!S?.programs) return "";
  const view = programView(m), row = view?.ministryrows?.[ministryIndex], plan = programPlanOf();
  if (!row || !plan) return "";
  const choices = (view.investment_choices || S.programs.investment_choices || []).filter(c => c.department === PG.department);
  const names = row.departments || [];
  return `<section class="pg-board" id="programBoard" aria-label="${programAttr(row.name)} departments" aria-busy="${PG.pending}">
    <div class="pg-heading"><div><div class="cab-kicker">One ministry · five departments</div><h3>Give every dollar a job.</h3><p>${escText(row.name)} · ${row.editable ? "Move funding between departments. The five always add up to your ministry budget." : "Five visible service budgets, managed together. Their detailed operating model remains automatic."}</p></div>${ministryIndex === 5 ? `<img src="/assets/programs-art.svg" alt="An illustrated industrial district with factories, clean power and freight">` : ""}</div>
    <div class="pg-summary"><article><span>Annual ministry plan</span><strong>${programMoney(row.annual_bn)}</strong></article><article><span>Daily allowance</span><strong>${programMoney(row.daily_bn)}</strong></article><article><span>Available project funds</span><strong>${programMoney(row.available_bn)}</strong></article><article><span>Spent this year</span><strong>${programMoney(row.spent_ytd_bn)}</strong></article></div>
    <p class="pg-note" id="pgPreviewStatus" role="status">${PG.error ? escText(PG.error) : PG.pending ? "Updating the server's funding preview…" : programHasDraft() ? "Draft allocations · not enacted. Spending to date remains the live ledger." : !S.programs.enabled ? "Ready to start: enacting your first department plan opens the shared daily funding ledger." : view.due ? "A new financial year: renew the department plan to release project funding." : "Enacted funding plan · unspent investment money is not charged as completed work."} Available funds include the allowance for your next daily work cycle.</p>
    ${PG.error ? `<button type="button" data-pg-retry>Retry funding preview</button>` : ""}
    ${row.editable ? `<div class="pg-presets" role="group" aria-label="Department allocation presets"><span>Quick draft</span><button type="button" data-pg-preset="balanced">Balanced</button>${ministryIndex === 5 ? `<button type="button" data-pg-preset="development">Build industry</button><button type="button" data-pg-preset="energy">Power first</button><button type="button" data-pg-preset="supply">Secure supplies</button>` : ""}<button type="button" data-pg-preset="current">Reset this ministry</button></div>` : ""}
    <div class="pg-departments">${names.map(d => programDepartmentCard(d,row,plan[ministryIndex])).join("")}</div>
    ${ministryIndex === 7 ? `<p class="pg-note">Plan effects: supported force ${fmtQ(view.defense_force)} · ammunition refill ${fmtQ(view.magazine_refill_mult)}×. Personnel, operations and military research support forces; maintenance supports ammunition; procurement pays real equipment orders and arms-plant work.</p>` : ""}
    ${ministryIndex === 5 ? `<div class="pg-heading"><div><div class="cab-kicker">Ten ways to invest · two per department</div><h3>${escText(names[PG.department]?.name || "Investment choices")}</h3><p>Build capacity, supply it with inputs, and put it to work. These are projects—not instant GDP bonuses.</p></div></div><div class="pg-investments">${choices.map(c=>programInvestmentHtml(c,!!S.programs.enabled&&!S.programs.due)).join("")}</div>${programIndustryHtml(view.industry || S.programs.industry)}` : ""}
    <details class="pg-note"><summary>How funding is counted</summary><p>Annual amounts are the current GDP-share run-rate. The simulation releases the correct daily fraction. Departments share one ministry envelope; projects share their department's funds. Materials and foreign purchases are shown separately from domestic project work. Unspent capital authority is not a second treasury. Operating services remain automatically managed where their sub-models are not yet separate.</p></details>
  </section>`;
}

function programIndustryHtml(data) {
  if (!data) return "";
  const sites = data.sites || [];
  return `<div class="pg-heading"><div><div class="cab-kicker">Industry in motion</div><h3>Your working industrial base</h3><p>${escText(data.note || "Usable output lives in this physical ledger. Its local value added is shown in the province economy; inventory is not treasury cash.")}</p><button type="button" class="pg-button" onclick="closeGameDrawers(); openNation(S.player); selectNationView('economy')">Explore your GDP breakdown →</button></div></div>
    <div class="pg-summary"><article><span>Intermediate packs</span><strong>${fmtQ(data.goods?.intermediates || 0)}</strong></article><article><span>Capital-goods packs</span><strong>${fmtQ(data.goods?.capital_goods || 0)}</strong></article><article><span>Storage / goods type</span><strong>${fmtQ(data.capacity_each || 0)}</strong></article><article><span>Industrial power used / capacity</span><strong>${fmtQ(data.power_used_daily || 0)} / ${fmtQ(data.power_capacity_daily || 0)}</strong></article></div>
    ${sites.length ? `<div class="pg-investments">${sites.map(site=>`<article class="pg-investment"><div class="pg-tag">${escText(site.status || "Built")}</div><h4>${escText(String(site.kind || "Industry").replace(/_/g," "))} · ${escText(site.district)}</h4><p>${escText(site.reason || "Capacity ready for use.")}</p><dl class="pg-ledger"><div><dt>Level</dt><dd>${site.level}</dd></div><div><dt>Output / day</dt><dd>${fmtQ(site.output_daily || 0)} packs</dd></div><div><dt>Work cost / day</dt><dd>${programMoney(site.cash_spent_daily_bn)}</dd></div></dl></article>`).join("")}</div>` : `<div class="pg-empty">Your new industrial base starts with a funded project. Completed sites and their actual operating status will appear here.</div>`}`;
}

function queueProgramRow(m, ministryIndex, row) {
  const next = programBudgetCommand(m);
  if (!next.departments) return;
  next.departments[ministryIndex] = row;
  queued = queued.filter(c => !["program_budget","annual_budget","military","invest"].includes(c.kind));
  queued.push(next); PG.error = ""; noteQueued(); paintCabinetDraft(m);
}

function wireProgramBoard(m, ministryIndex) {
  const board = document.querySelector("#programBoard");
  if (!board) return;
  const retry=board.querySelector("[data-pg-retry]"); if(retry) retry.onclick=()=>refreshProgramPreview(m,true);
  board.querySelectorAll("[data-pg-share]").forEach(input => {
    input.oninput = () => {
      const plan = programPlanOf(), row = programRedistribute(plan[ministryIndex], +input.dataset.pgShare, +input.value);
      if (!row) return;
      queueProgramRow(m,ministryIndex,row);
      board.querySelectorAll("[data-pg-share]").forEach(el => { const bp=row[+el.dataset.pgShare]; el.value=String(bp); el.setAttribute("aria-valuetext",`${(bp/100).toFixed(1)} percent`); el.previousElementSibling.textContent=`${(bp/100).toFixed(1)}% of this ministry`; });
      const status = board.querySelector("#pgPreviewStatus"); if (status) status.textContent = "Draft changed. Release the control to refresh money amounts.";
    };
    input.onchange = () => refreshProgramPreview(m, true);
  });
  board.querySelectorAll("[data-pg-preset]").forEach(b => { b.onclick = () => {
    const presets = { balanced:[2000,2000,2000,2000,2000], development:[4000,2000,1500,1500,1000], energy:[1500,4500,1500,1500,1000], supply:[1500,1500,3000,3000,1000] };
    const row = b.dataset.pgPreset === "current" ? S.programs.departments[ministryIndex].slice() : presets[b.dataset.pgPreset];
    if (!row) return; queueProgramRow(m,ministryIndex,row.slice()); renderProgramBoard(m,ministryIndex); refreshProgramPreview(m,true);
  }; });
  board.querySelectorAll("[data-pg-department]").forEach(b => { b.onclick = () => { PG.department=+b.dataset.pgDepartment; renderProgramBoard(m,ministryIndex); document.querySelector(`[data-pg-department="${PG.department}"]`)?.focus({preventScroll:true}); }; });
  board.querySelectorAll("[data-pg-invest]").forEach(b => { b.onclick = () => openProgramInvestment(b.dataset.pgInvest); });
  refreshProgramPreview(m);
}

function renderProgramBoard(m, index) {
  const board = document.querySelector("#programBoard");
  if (!board) return;
  const activeId = board.contains(document.activeElement) ? document.activeElement.id : null;
  board.outerHTML = programBoardHtml(m,index); wireProgramBoard(m,index);
  if (activeId) document.getElementById(activeId)?.focus({preventScroll:true});
}

async function refreshProgramPreview(m, force=false) {
  if (!S?.programs || !m) return;
  const key = programDraftKey(m);
  if (!force && (PG.key === key || PG.failedKey === key || PG.pending && PG.requestKey === key)) return;
  const seq=++PG.seq; PG.pending=true; PG.requestKey=key;
  try {
    const response = await api("/api/program-preview",programBudgetCommand(m));
    if (seq !== PG.seq || key !== programDraftKey(me())) return;
    PG.preview=response; PG.key=key; PG.error=""; PG.failedKey=null;
  } catch(error) { if (seq !== PG.seq) return; PG.error=(error.message||"Funding preview unavailable")+". Showing the enacted ledger; your draft is kept."; PG.preview=null; PG.key=null; PG.failedKey=key; }
  finally {
    if(seq===PG.seq) { PG.pending=false; const current=me(); if(current && cabinetIsOpen()) { renderProgramBoard(current,Math.max(0,MINISTRIES.findIndex(s=>s.id===CAB.ministry))); paintCabinetDraft(current); } }
  }
}

async function openProgramInvestment(id) {
  const choice = (S?.programs?.investment_choices || []).find(c=>c.id===id);
  if (!choice?.enabled || !S.programs.enabled || S.programs.due) return;
  if (choice.project_kind) {
    PROD.mode="build"; openProduction();
    PROD.pickKind=choice.project_kind; PROD.view="provinces"; renderProductionPanel();
  } else {
    closeGameDrawers(); openStock();
  }
}
