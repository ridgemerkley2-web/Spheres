"use strict";
// All conditions, outcomes and progress are supplied by the simulation.
const AGENCY = { state: null, world: null, busy: false, review: null, action: null, notice: "" };
const AGENCY_REVIEW_KINDS = new Set(["respond_diplomacy", "set_diplomatic_policy", "break_currency_peg", "resume_automatic_bank", "sanction", "lift", "improve", "choose_campaign_aim", "continue_sandbox"]);
function agencyEscape(value) {
  return String(value ?? "").replace(/[&<>"']/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));
}
function agencyPanel() {
  let panel = document.getElementById("agencyPanel");
  if (panel) return panel;
  panel = document.createElement("dialog");
  panel.id = "agencyPanel";
  panel.setAttribute("aria-labelledby", "agencyTitle");
  panel.innerHTML = `<header><div><small>Your government, your decisions</small><h2 id="agencyTitle">Decisions & commitments</h2></div><button type="button" id="agencyClose" aria-label="Close decisions">Close</button></header><div class="agency-room-art">${globalThis.AreaArt?.html("diplomacy", "banner") || ""}</div><div id="agencyBody"></div><p id="agencyStatus" role="status" aria-live="polite" tabindex="-1"></p>`;
  panel.addEventListener("keydown", event => event.stopPropagation());
  // Escape, backdrop and explicit close all invalidate an unfinished review.
  const clearReview = () => { AGENCY.review = null; AGENCY.notice = ""; };
  panel.addEventListener("cancel", clearReview);
  panel.addEventListener("close", clearReview);
  document.body.appendChild(panel);
  panel.querySelector("#agencyClose").onclick = () => panel.close();
  panel.addEventListener("click", event => { if (event.target === panel) { const r=panel.getBoundingClientRect(); if(event.clientX<r.left || event.clientX>r.right || event.clientY<r.top || event.clientY>r.bottom) panel.close(); } });
  return panel;
}
function openAgency() {
  clockPause();
  const panel = agencyPanel();
  if (!panel.open) { AGENCY.review = null; AGENCY.notice = ""; }
  renderAgency(S);
  if (!panel.open) panel.showModal();
  document.getElementById("agencyClose").focus();
}
function agencyCanonical(value) {
  if (Array.isArray(value)) return "[" + value.map(agencyCanonical).join(",") + "]";
  if (value && typeof value === "object") return "{" + Object.keys(value).sort().map(key => JSON.stringify(key) + ":" + agencyCanonical(value[key])).join(",") + "}";
  return JSON.stringify(value);
}
function agencyActionBlocked() {
  return AGENCY.busy || (typeof SESSION !== "undefined" && SESSION.busy)
    || (typeof advancing !== "undefined" && advancing) || (typeof pendingAdvance !== "undefined" && !!pendingAdvance)
    || (typeof COMMAND_CHANNEL !== "undefined" && (COMMAND_CHANNEL.busy || !!COMMAND_CHANNEL.pending));
}
function agencyReviewCurrent(review) {
  return !!review && AGENCY.review === review && document.getElementById("agencyPanel") === review.panel && review.panel.open
    && S === review.world && AGENCY.world === review.world && S?.session_id === review.session && S?.player === review.nation;
}
function agencyReviewValid(review) {
  return agencyReviewCurrent(review) && !review.loading && !review.error && review.data?.valid === true
    && typeof review.data.review_token === "string" && review.data.review_token.length > 0
    && review.data.session_id === review.session && review.data.nation === review.nation
    && agencyCanonical(review.command) === review.canonical && agencyCanonical(review.data.command) === review.canonical;
}
async function agencyReview(command) {
  if (agencyActionBlocked() || !S?.player || !S?.session_id || !AGENCY_REVIEW_KINDS.has(command?.kind)) return;
  const copied = JSON.parse(JSON.stringify(command));
  openAgency();
  const review = { command: copied, canonical: agencyCanonical(copied), world: S, session: S.session_id, nation: S.player,
    panel: agencyPanel(), loading: true, data: null, error: "" };
  AGENCY.review = review; AGENCY.notice = ""; renderAgency(S);
  try {
    const result = await api("/api/decisions/preview", { session_id: review.session, nation: review.nation, command: copied });
    if (!agencyReviewCurrent(review)) return;
    if (result.session_id !== review.session || result.nation !== review.nation || agencyCanonical(result.command) !== review.canonical)
      throw new Error("The reviewed decision changed. Review it again before confirming.");
    if (result.valid && (typeof result.review_token !== "string" || !result.review_token))
      throw new Error("This decision has no current review. Review it again before confirming.");
    review.data = result;
  } catch (error) {
    if (agencyReviewCurrent(review)) review.error = error.message || String(error);
  } finally {
    if (agencyReviewCurrent(review)) {
      review.loading = false; renderAgency(S);
      document.getElementById("agencyReviewTitle")?.focus({preventScroll:true});
      document.getElementById("agencyReview")?.scrollIntoView({block:"nearest"});
    }
  }
}
// Existing inbox and aim controls, and diplomatic dossier actions, share the
// same preview flow. Only the separate confirmation below can send an order.
function agencyCommand(command) { return agencyReview(command); }
async function agencyConfirm(review) {
  if (agencyActionBlocked() || !agencyReviewValid(review)) return;
  const action = {}; AGENCY.action = action; AGENCY.busy = true; renderAgency(S);
  try {
    const response = await api("/api/command", { commands: [JSON.parse(JSON.stringify(review.command))],
      review_token: review.data.review_token, review_kind: "decisions" });
    if (!agencyReviewCurrent(review) || AGENCY.action !== action) return;
    if (response.session_id !== review.session) throw new Error("The campaign changed. Open a fresh decision review.");
    const message = response.errors?.length ? response.errors.join(" ") : `Decision recorded${review.data.date_label ? " on " + review.data.date_label : ""}. The figures now show the result.`;
    AGENCY.review = null;
    if (response.errors?.length) {
      // api() can synthesize an error envelope from the old local S. A stale
      // review refusal therefore needs a real read, not adoption of that copy.
      const world = S;
      try {
        const fresh = await api("/api/state");
        if (S === world && fresh.session_id === review.session && review.panel.open && AGENCY.action === action) await adopt(fresh, false);
      } catch (_) { /* Preserve the original refusal and pending receipt. */ }
    } else await adopt(response, false);
    if (AGENCY.action === action && S?.session_id === review.session && review.panel.open) {
      AGENCY.notice = message;
    }
  } catch (error) {
    if (agencyReviewCurrent(review) && AGENCY.action === action) {
      AGENCY.review = null; AGENCY.notice = error.message || String(error);
      // An uncertain command receipt is recovered by the shared command channel.
      // Refresh reads do not retry the order or reuse its reviewed token.
      const world = S;
      try {
        const fresh = await api("/api/state");
        if (S === world && fresh.session_id === review.session && review.panel.open && AGENCY.action === action) await adopt(fresh, false);
      } catch (_) { /* Keep the original actionable error visible. */ }
    }
  } finally {
    if (AGENCY.action === action) {
      AGENCY.action = null; AGENCY.busy = false; renderAgency(S);
      if (AGENCY.notice && S?.session_id === review.session && S?.player === review.nation && review.panel.open) {
        const status = document.getElementById("agencyStatus");
        status?.focus({preventScroll:true}); status?.scrollIntoView({block:"nearest"});
      }
    }
  }
}
function agencyChangeCards(changes) {
  if (!changes?.length) return "";
  const esc = agencyEscape;
  return `<section class="agency-changes" aria-label="Decision effects before and after"><h4>What this decision changes</h4><dl class="agency-change-list">${changes.map((change,index) => `<div class="agency-change" data-agency-change="${index}"><dt>${esc(change.label)}</dt><dd><div class="agency-change-values"><div><span>Before</span><strong data-change-before>${esc(change.before)}</strong></div><div><span>After</span><strong data-change-after>${esc(change.after)}</strong></div></div>${change.detail ? `<p>${esc(change.detail)}</p>` : ""}</dd></div>`).join("")}</dl></section>`;
}
function renderAgencyReview(body) {
  const review = AGENCY.review;
  if (!agencyReviewCurrent(review)) return;
  const q = review.data, esc = agencyEscape, section = document.createElement("section");
  section.id = "agencyReview"; section.setAttribute("aria-labelledby", "agencyReviewTitle");
  section.innerHTML = `<p class="agency-kicker">Review before committing${q?.date_label ? " · " + esc(q.date_label) : ""}</p><h3 id="agencyReviewTitle" tabindex="-1">${esc(q?.title || "Review decision")}</h3>${review.loading ? '<p role="status">Checking current conditions and effects…</p>' : ""}${review.error ? `<p class="agency-refusal" role="alert">${esc(review.error)}</p><button type="button" data-agency-review-retry>Review again</button>` : ""}${q ? `${q.description ? `<p>${esc(q.description)}</p>` : ""}${q.reason ? `<p class="agency-refusal" role="status">${esc(q.reason)}</p>` : ""}${agencyChangeCards(q.changes)}${q.warnings?.length ? `<ul>${q.warnings.map(w => `<li>${esc(w)}</li>`).join("")}</ul>` : ""}` : ""}<div class="agency-actions"><button type="button" class="agency-confirm" data-agency-confirm ${!AGENCY.busy && agencyReviewValid(review) ? "" : "disabled"}>Confirm decision</button><button type="button" data-agency-review-cancel>Keep considering</button></div>`;
  body.prepend(section);
  section.querySelector("[data-agency-confirm]").onclick = () => agencyConfirm(review);
  section.querySelector("[data-agency-review-cancel]").onclick = () => {
    if (AGENCY.review === review) { AGENCY.review = null; AGENCY.notice = "No decision sent."; renderAgency(S); document.getElementById("agencyClose")?.focus(); }
  };
  const retry = section.querySelector("[data-agency-review-retry]");
  if (retry) retry.onclick = () => agencyReview(review.command);
}
function renderAgency(state) {
  if (AGENCY.world !== state) {
    if (AGENCY.review) AGENCY.notice = "The campaign changed. Review this decision again before confirming.";
    AGENCY.review = null; AGENCY.world = state;
  }
  AGENCY.state = state?.agency;
  const button = document.getElementById("agencyBtn");
  if (button) { button.disabled = !AGENCY.state; button.classList.toggle("attention", !!AGENCY.state?.offers.length); }
  const count = document.getElementById("agencyCount");
  if (count) count.textContent = AGENCY.state?.offers.length || "";
  const body = document.getElementById("agencyBody");
  if (!body) return;
  const a = AGENCY.state, esc = agencyEscape;
  if (!a) { body.textContent = "Choose a country to make decisions."; return; }
  // Native ISO deadlines retain their order even when overdue requests all
  // report zero days remaining after loading an older campaign.
  const offers = a.offers.slice().sort((left,right) => left.expires < right.expires ? -1 : left.expires > right.expires ? 1 : left.id - right.id);
  const remaining = offer => offer.days_remaining > 0
    ? `${esc(offer.days_remaining)} ${offer.days_remaining === 1 ? "day" : "days"} remaining`
    : "Reply window closed";
  body.innerHTML = `<section id="agencyInbox" aria-labelledby="agencyInboxTitle"><h3 id="agencyInboxTitle">Diplomatic inbox <span>${offers.length} pending</span></h3>${offers.length ? `<p class="agency-inbox-summary">Nearest deadline: <strong>${esc(offers[0].expires)}</strong> · ${remaining(offers[0])}</p><p>Review the effects, then confirm your reply. Opening a review sends no decision.</p>` : ""}<p>${esc(a.expiry_rule)}</p>${offers.length ? offers.map(o => `<article class="agency-offer" data-agency-offer="${esc(o.id)}"><h4>${esc(o.from_name)} · ${esc(o.title)}</h4><p class="agency-deadline">Reply before <time>${esc(o.expires)}</time> · ${remaining(o)}</p><p>${esc(o.consequence)}</p>${o.accept_blocked ? `<p class="agency-refusal">${esc(o.accept_blocked)}</p>` : ""}<div class="agency-actions"><button type="button" data-agency-accept="${esc(o.id)}" ${o.accept_blocked ? "disabled" : ""}>Review acceptance</button><button type="button" data-agency-decline="${esc(o.id)}">Review decline</button></div></article>`).join("") : '<p class="agency-empty">No requests awaiting your answer.</p>'}</section>
    <section><h3>Standing diplomatic policy</h3><p>Apply automatically to future requests. Existing requests keep their own reply deadline.</p><form id="agencyPolicy">${[["defense_pacts","Defense pacts"],["trade_treaties","Trade treaties"],["calls_to_arms","Calls to arms"]].map(([key,label]) => `<label>${label}<select name="${key}">${[["review","Ask me"],["accept","Accept when legal"],["decline","Decline"]].map(([value,text]) => `<option value="${value}" ${a.policy[key]===value ? "selected" : ""}>${text}</option>`).join("")}</select></label>`).join("")}<button type="submit">Save standing policy</button></form></section>
    <section><h3>Monetary commitment</h3>${a.monetary.kind === "pegged" ? `<p>Your currency is pegged. The policy rate is held at ${(a.monetary.rate*100).toFixed(2)}%. Exit the peg before changing rates.</p><p>Exit cost: ${a.break_peg_pc} political capital, −5 stability, +2 percentage points of inflation. The automatic central bank then resumes.</p><button type="button" id="agencyBreakPeg">Exit currency peg · ${a.break_peg_pc} PC</button>` : `<p>Floating currency · ${a.automatic_bank ? "automatic central bank" : "manual policy rate"}.</p>${a.automatic_bank ? "" : '<button type="button" id="agencyResumeBank">Resume automatic central bank · free</button>'}`}</section>
    <section><h3>Recent decisions</h3>${a.history.length ? `<ul>${a.history.slice().reverse().map(h=>`<li>${h.date_label ? `<time>${esc(h.date_label)}</time> · ` : ""}${h.from_name ? esc(h.from_name) + " · " : ""}${h.title ? esc(h.title) + " · " : ""}Request #${esc(h.offer.id)}: ${esc(h.outcome)}</li>`).join("")}</ul>` : '<p class="agency-empty">Your responses will be recorded here and retained in your save.</p>'}</section>`;
  renderCampaignAims(state?.campaign_aims, body);
  body.querySelectorAll("[data-agency-accept]").forEach(b => b.onclick = () => agencyCommand({kind:"respond_diplomacy",offer:Number(b.dataset.agencyAccept),accept:true}));
  body.querySelectorAll("[data-agency-decline]").forEach(b => b.onclick = () => agencyCommand({kind:"respond_diplomacy",offer:Number(b.dataset.agencyDecline),accept:false}));
  body.querySelector("#agencyPolicy").onsubmit = event => { event.preventDefault(); agencyCommand({kind:"set_diplomatic_policy",policy:Object.fromEntries(new FormData(event.target))}); };
  const exit = body.querySelector("#agencyBreakPeg");
  if (exit) exit.onclick = () => agencyCommand({kind:"break_currency_peg"});
  const resume = body.querySelector("#agencyResumeBank");
  if (resume) resume.onclick = () => agencyCommand({kind:"resume_automatic_bank"});
  renderAgencyReview(body);
  body.querySelectorAll("button,select").forEach(b => { if (agencyActionBlocked()) b.disabled = true; });
  document.getElementById("agencyStatus").textContent = AGENCY.notice;
}

function renderCampaignAims(aims, body) {
  if (!aims) return;
  const esc=agencyEscape, active=aims.active, evaluation=aims.evaluation;
  const section=document.createElement("section");
  section.className="agency-aims";
  const title=aim => aims.offers.find(o=>o.aim===aim)?.title || aim;
  section.innerHTML=`<h3>Campaign aims</h3><p>${esc(aims.note)}</p>${active ? `<article class="agency-offer"><h4>${esc(title(active.aim))} ${active.completed_day!==null ? "· Achieved" : "· Active"}</h4><p>Fixed target: ${active.target.toFixed(2)} ${esc(evaluation.metric)}. Current: ${evaluation.value.toFixed(2)}.</p><progress max="1" value="${active.completed_day!==null ? 1 : evaluation.progress}" aria-label="Campaign target progress"></progress><p>${active.held_days} / ${active.hold_days} qualifying days. ${active.completed_day!==null ? "Achievement recorded. The world can keep running." : "Breaking any condition resets the consecutive-day count."}</p>${active.completed_day===null && evaluation.blockers.length ? `<ul>${evaluation.blockers.map(b=>`<li>${esc(b)}</li>`).join("")}</ul>` : ""}<button type="button" data-sandbox>Continue in sandbox${active.completed_day===null ? " · set aim aside" : ""}</button></article>` : `<div class="agency-aim-grid">${aims.offers.map(o=>`<article class="agency-offer"><h4>${esc(o.title)}</h4><p>${esc(o.description)}</p>${o.unavailable ? `<p class="agency-refusal">${esc(o.unavailable)}</p>` : ""}<button type="button" data-campaign-aim="${o.aim}" ${o.unavailable ? "disabled" : ""}>Choose this aim · free</button></article>`).join("")}</div>`}${aims.history.length ? `<details><summary>Campaign record (${aims.history.length})</summary><ul>${aims.history.slice().reverse().map(r=>`<li>${esc(title(r.goal.aim))}: ${esc(r.outcome)} · ${r.goal.held_days} qualifying days</li>`).join("")}</ul></details>` : ""}`;
  body.appendChild(section);
  section.querySelectorAll("[data-campaign-aim]").forEach(b=>b.onclick=()=>agencyCommand({kind:"choose_campaign_aim",aim:b.dataset.campaignAim}));
  const sandbox=section.querySelector("[data-sandbox]");
  if(sandbox) sandbox.onclick=()=>agencyCommand({kind:"continue_sandbox"});
}
