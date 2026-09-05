"use strict";
// All conditions, outcomes and progress are supplied by the simulation.
const AGENCY = { state: null, busy: false };
function agencyEscape(value) {
  return String(value ?? "").replace(/[&<>"']/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));
}
function agencyPanel() {
  let panel = document.getElementById("agencyPanel");
  if (panel) return panel;
  panel = document.createElement("dialog");
  panel.id = "agencyPanel";
  panel.setAttribute("aria-labelledby", "agencyTitle");
  panel.innerHTML = '<header><div><small>Your government, your decisions</small><h2 id="agencyTitle">Decisions & commitments</h2></div><button type="button" id="agencyClose" aria-label="Close decisions">Close</button></header><div id="agencyBody"></div><p id="agencyStatus" role="status" aria-live="polite"></p>';
  panel.addEventListener("keydown", event => event.stopPropagation());
  document.body.appendChild(panel);
  panel.querySelector("#agencyClose").onclick = () => panel.close();
  panel.addEventListener("click", event => { if (event.target === panel) { const r=panel.getBoundingClientRect(); if(event.clientX<r.left || event.clientX>r.right || event.clientY<r.top || event.clientY>r.bottom) panel.close(); } });
  return panel;
}
function openAgency() {
  clockPause();
  const panel = agencyPanel();
  renderAgency(S);
  if (!panel.open) panel.showModal();
  document.getElementById("agencyClose").focus();
}
async function agencyCommand(command) {
  if (AGENCY.busy) return;
  AGENCY.busy = true;
  let message = "Decision recorded.";
  try {
    const response = await api("/api/command", { commands: [command] });
    if (response.errors?.length) message = response.errors.join(" ");
    await adopt(response, false);
  } catch (error) { message = error.message || String(error); }
  finally { AGENCY.busy = false; renderAgency(S); document.getElementById("agencyStatus").textContent = message; }
}
function renderAgency(state) {
  AGENCY.state = state?.agency;
  const button = document.getElementById("agencyBtn");
  if (button) { button.disabled = !AGENCY.state; button.classList.toggle("attention", !!AGENCY.state?.offers.length); }
  const count = document.getElementById("agencyCount");
  if (count) count.textContent = AGENCY.state?.offers.length || "";
  const body = document.getElementById("agencyBody");
  if (!body) return;
  const a = AGENCY.state, esc = agencyEscape;
  if (!a) { body.textContent = "Choose a country to make decisions."; return; }
  body.innerHTML = `<section><h3>Diplomatic inbox <span>${a.offers.length} pending</span></h3><p>${esc(a.expiry_rule)}</p>${a.offers.length ? a.offers.map(o => `<article class="agency-offer"><h4>${esc(o.from_name)} proposes ${esc(o.title)}</h4><p class="agency-deadline">Reply by ${esc(o.expires)} · ${o.days_remaining} days remaining</p><p>${esc(o.consequence)}</p>${o.accept_blocked ? `<p class="agency-refusal">${esc(o.accept_blocked)}</p>` : ""}<div class="agency-actions"><button type="button" data-agency-accept="${o.id}" ${o.accept_blocked ? "disabled" : ""}>Accept</button><button type="button" data-agency-decline="${o.id}">Decline</button></div></article>`).join("") : '<p class="agency-empty">No requests awaiting your answer.</p>'}</section>
    <section><h3>Standing diplomatic policy</h3><p>Apply automatically to future requests. Existing requests keep their own reply deadline.</p><form id="agencyPolicy">${[["defense_pacts","Defense pacts"],["trade_treaties","Trade treaties"],["calls_to_arms","Calls to arms"]].map(([key,label]) => `<label>${label}<select name="${key}">${[["review","Ask me"],["accept","Accept when legal"],["decline","Decline"]].map(([value,text]) => `<option value="${value}" ${a.policy[key]===value ? "selected" : ""}>${text}</option>`).join("")}</select></label>`).join("")}<button type="submit">Save standing policy</button></form></section>
    <section><h3>Monetary commitment</h3>${a.monetary.kind === "pegged" ? `<p>Your currency is pegged. The policy rate is held at ${(a.monetary.rate*100).toFixed(2)}%. Exit the peg before changing rates.</p><p>Exit cost: ${a.break_peg_pc} political capital, −5 stability, +2 percentage points of inflation. The automatic central bank then resumes.</p><button type="button" id="agencyBreakPeg">Exit currency peg · ${a.break_peg_pc} PC</button>` : `<p>Floating currency · ${a.automatic_bank ? "automatic central bank" : "manual policy rate"}.</p>${a.automatic_bank ? "" : '<button type="button" id="agencyResumeBank">Resume automatic central bank · free</button>'}`}</section>
    <section><h3>Recent decisions</h3>${a.history.length ? `<ul>${a.history.slice().reverse().map(h=>`<li>Request #${h.offer.id}: ${esc(h.outcome)}</li>`).join("")}</ul>` : '<p class="agency-empty">Your responses will be recorded here and retained in your save.</p>'}</section>`;
  body.querySelectorAll("[data-agency-accept]").forEach(b => b.onclick = () => agencyCommand({kind:"respond_diplomacy",offer:Number(b.dataset.agencyAccept),accept:true}));
  body.querySelectorAll("[data-agency-decline]").forEach(b => b.onclick = () => agencyCommand({kind:"respond_diplomacy",offer:Number(b.dataset.agencyDecline),accept:false}));
  body.querySelector("#agencyPolicy").onsubmit = event => { event.preventDefault(); agencyCommand({kind:"set_diplomatic_policy",policy:Object.fromEntries(new FormData(event.target))}); };
  const exit = body.querySelector("#agencyBreakPeg");
  if (exit) exit.onclick = () => agencyCommand({kind:"break_currency_peg"});
  const resume = body.querySelector("#agencyResumeBank");
  if (resume) resume.onclick = () => agencyCommand({kind:"resume_automatic_bank"});
  body.querySelectorAll("button,select").forEach(b => { if (AGENCY.busy) b.disabled = true; });
}
