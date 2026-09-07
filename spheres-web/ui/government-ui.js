/* Government presentation only. Political conditions, prices, effects and commands
   come from the server. The host owns navigation, stale-state guards and orders. */
(function (root, factory) {
  const api = factory();
  if (typeof module !== "undefined" && module.exports) module.exports = api;
  root.GovernmentUI = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";
  const TABS = [["overview", "Overview"], ["decisions", "Governing decisions"], ["politics", "Parliament & parties"], ["watch", "Takeover watch"]];
  const ROUTES = new Set(["budget", "cash_flow", "construction", "diplomacy", "research", "military", "mine"]);
  const COLORS = {Western:"#719fdb", Communist:"#d17970", Nationalist:"#b29b80", Islamist:"#80b69a", NonAligned:"#a8adba"};
  const LABELS = {Western:"Western", Communist:"Communist", Nationalist:"Nationalist", Islamist:"Islamist", NonAligned:"Non-Aligned"};
  const rows = value => Array.isArray(value) ? value : [];
  const esc = value => String(value ?? "").replace(/[&<>"']/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));
  const num = value => Number.isFinite(value) ? value.toLocaleString("en-US", {maximumFractionDigits:2}) : "—";
  const pct = value => Number.isFinite(value) ? `${(value * 100).toLocaleString("en-US", {maximumFractionDigits:1})}%` : "—";
  const valueText = value => value == null || typeof value === "object" ? "—" : typeof value === "number" ? num(value) : String(value);
  const textOf = value => typeof value === "object" && value ? value.detail || value.label || value.value || "" : value;
  const color = bloc => COLORS[bloc] || "#a8adba";
  const blocLabel = bloc => LABELS[bloc] || "Unspecified alignment";
  const width = value => Number.isFinite(value) ? Math.min(1, Math.max(0, value)) * 100 : 0;
  const locked = state => !!(state.busy || state.blocked || state.loading || state.review?.loading);
  const money = bn => {
    if (!Number.isFinite(bn)) return "—";
    const amount = Math.abs(bn) * 1e9, sign = bn < 0 ? "−" : "";
    for (const [scale, suffix] of [[1e12,"tn"],[1e9,"bn"],[1e6,"m"],[1e3,"k"]]) {
      if (amount >= scale) return `${sign}$${(amount / scale).toLocaleString("en-US", {maximumFractionDigits:2})}${suffix}`;
    }
    return `${sign}$${amount.toLocaleString("en-US", {maximumFractionDigits:2})}`;
  };
  function routeButton(route, label, data, style = "") {
    if (!ROUTES.has(route) || (!data.mine && route !== "mine")) return "";
    return `<button type="button" class="gov-ui-route ${style}" data-gov-route="${esc(route)}">${esc(label)}<span aria-hidden="true">↗</span></button>`;
  }
  function actionReason(action, data) {
    if (!data.mine) return "Only your own government can take this decision.";
    if (action.refusal) return action.refusal;
    if (!action.command || !Number.isFinite(action.price) || action.price < 0) return "This decision is not available in the current reading.";
    return "";
  }
  function reviewButton(index, data, state, label = "Review decision", style = "") {
    const action = rows(data.actions)[index];
    if (!Number.isInteger(index) || !action) return "";
    const reason = actionReason(action, data);
    return `<button type="button" class="${style}" data-gov-review="${index}" ${reason || locked(state) ? "disabled" : ""}${reason ? ` title="${esc(reason)}"` : ""}>${esc(label)}</button>`;
  }
  function sectionTitle(kicker, title, description = "", id = "") {
    return `<div class="gov-ui-section-heading"><div>${kicker ? `<p class="gov-ui-kicker">${esc(kicker)}</p>` : ""}<h2${id ? ` id="${esc(id)}"` : ""}>${esc(title)}</h2>${description ? `<p>${esc(description)}</p>` : ""}</div></div>`;
  }
  function metrics(items, style = "") {
    return rows(items).length ? `<dl class="gov-ui-metrics ${style}">${rows(items).map(item => `<div><dt>${esc(item.label || item.key)}</dt><dd>${esc(valueText(item.value))}${item.detail ? `<small>${esc(item.detail)}</small>` : ""}</dd></div>`).join("")}</dl>` : "";
  }
  function hero(data, tab) {
    const briefing = data.briefing || {}, leader = data.leader || {};
    return `<header class="gov-ui-hero${tab !== "overview" ? " gov-ui-hero-compact" : ""}"><img class="gov-ui-council-art" src="/art/government/council-v1.png" alt="" aria-hidden="true" width="1536" height="1024" decoding="async" draggable="false"><div class="gov-ui-hero-copy"><p class="gov-ui-kicker">${data.mine ? "Your government" : "Foreign government · view only"}${briefing.date_label ? ` · ${esc(briefing.date_label)}` : ""}</p><h1>${esc(data.nation_name || data.nation || "Government")}</h1><p class="gov-ui-hero-summary">${esc(briefing.summary || (data.on === false ? "Review your country's administration and open the government tools available in this campaign." : "Understand who holds power, review your decisions and follow the conditions shaping political change."))}</p>${(leader.name || leader.described) ? `<p class="gov-ui-leader-line"><strong>${esc(leader.name || leader.described)}</strong>${leader.office ? ` <span>${esc(leader.office)}</span>` : ""}</p>` : ""}<div class="gov-ui-hero-actions"><button type="button" class="gov-ui-primary" data-gov-tab="decisions">${data.mine ? "Review governing decisions" : "Inspect governing decisions"}</button>${routeButton(data.mine ? "budget" : "mine", data.mine ? "Open national budget" : "Return to your government", data)}</div></div></header>`;
  }
  function navigation(tab, data) {
    return `<nav class="gov-ui-tabs" role="tablist" aria-label="Government sections">${TABS.map(([id,label]) => `<button type="button" id="gov-tab-${id}" role="tab" data-gov-tab="${id}" aria-controls="gov-panel-${id}" aria-selected="${tab === id}" tabindex="${tab === id ? 0 : -1}">${esc(id === "politics" && !data.electoral ? "Regime & institutions" : label)}</button>`).join("")}</nav>`;
  }
  function ruler(data) {
    const leader = data.leader || {};
    return `<section class="gov-ui-panel">${sectionTitle("Leadership", "Who holds power")}<div class="gov-ui-leadership"><h3>${esc(leader.name || leader.described || data.ruling_institution || "No leader recorded")}</h3>${leader.native && leader.native !== leader.name ? `<p class="gov-ui-native">${esc(leader.native)}</p>` : ""}<p>${esc(leader.office || data.system || "")}${leader.since ? ` · Since ${esc(leader.since)}` : ""}</p>${data.on !== false && data.ruling_bloc ? `<span class="gov-ui-badge"><i style="--bloc-color:${color(data.ruling_bloc)}" aria-hidden="true"></i>${esc(blocLabel(data.ruling_bloc))}</span>` : ""}${data.government_of_the_day ? `<p class="gov-ui-note">Governing party: ${esc(data.government_of_the_day_name || data.government_of_the_day)}</p>` : ""}${leader.must_leave_by ? `<p class="gov-ui-note">Must leave office by ${esc(leader.must_leave_by)}</p>` : ""}${leader.heir ? `<p class="gov-ui-note">Successor: ${esc(leader.heir.name)}${leader.heir.office ? ` · ${esc(leader.heir.office)}` : ""}${leader.heir.since ? ` · Since ${esc(leader.heir.since)}` : ""}</p>` : ""}${rows(leader.also).map(person => `<p class="gov-ui-note">${esc(person.name)} · ${esc(person.office)}${person.since ? ` · Since ${esc(person.since)}` : ""}</p>`).join("")}</div></section>`;
  }
  function attention(data, state) {
    const items = rows(data.briefing?.attention);
    return `<section class="gov-ui-panel">${sectionTitle("National briefing", "What needs your attention", "Read the current conditions before choosing a response.")}${items.length ? `<div class="gov-ui-attention">${items.map(item => {
      const tone = item.tone === "attention" ? "warning" : item.tone === "good" ? "positive" : ["warning","danger","positive","neutral"].includes(item.tone) ? item.tone : "neutral";
      return `<article class="gov-ui-attention-card gov-ui-tone-${tone}"><h3>${esc(item.title)}</h3><p>${esc(item.detail)}</p><div class="gov-ui-actions">${Number.isInteger(item.action_index) ? reviewButton(item.action_index,data,state,"Review this decision") : ""}${routeButton(item.route,"Open related policy",data)}</div></article>`;
    }).join("")}</div>` : '<p class="gov-ui-note">No priority notes were supplied in this reading. Review governing decisions and the political watch for details.</p>'}</section>`;
  }
  function overview(data, state) {
    const briefing = data.briefing || {}, stats = rows(briefing.stats);
    const fallback = [{label:"Political capital",value:Number.isFinite(data.political_capital) ? `${num(data.political_capital)} PC` : null},...(data.on === false ? [] : [{label:"Discontent",value:pct(data.discontent)}]),data.electoral ? {label:"Government seat share",value:pct(data.government_seats)} : {label:"Ruling institution",value:data.ruling_institution},{label:data.electoral ? "Next election" : "Government system",value:data.electoral ? data.next_election : data.system}];
    const drivers = rows(briefing.drivers), links = rows(briefing.links).filter(link => ROUTES.has(link.route) && (data.mine || link.route === "mine"));
    return `${metrics(stats.length ? stats : fallback,"gov-ui-key-stats")}<div class="gov-ui-overview-grid">${ruler(data)}${attention(data,state)}</div>${drivers.length ? `<section class="gov-ui-panel">${sectionTitle("Policy connections","What is shaping the reading","These explanations come from the current government briefing.")}<div class="gov-ui-drivers">${drivers.map(item => `<article><h3>${esc(item.label)}</h3>${item.value != null ? `<strong>${esc(valueText(item.value))}</strong>` : ""}<p>${esc(item.detail)}</p>${routeButton(item.route,"Review related policy",data)}</article>`).join("")}</div></section>` : ""}${links.length ? `<section class="gov-ui-destinations">${sectionTitle("Continue governing","Open the relevant department")}<div>${links.map(item => `<article><h3>${esc(item.label)}</h3><p>${esc(item.detail)}</p>${routeButton(item.route,item.label,data)}</article>`).join("")}</div></section>` : ""}`;
  }
  function actionCard(action, index, data, state, compact = false) {
    const reason = actionReason(action,data), price = Number.isFinite(action.price) ? `${num(action.price)} PC` : "Price unavailable";
    return `<article class="gov-ui-decision${reason ? " is-unavailable" : ""}" data-gov-decision="${index}"><div class="gov-ui-decision-heading"><h3>${esc(action.label || "Government decision")}</h3><span class="gov-ui-price">${esc(price)}</span></div>${action.detail ? `<p>${esc(action.detail)}</p>` : ""}${!compact && action.blurb ? `<p class="gov-ui-note">${esc(action.blurb)}</p>` : ""}${reason ? `<p class="gov-ui-refusal">${esc(reason)}</p>` : ""}${!compact && rows(action.effects).length ? `<ul class="gov-ui-effects">${action.effects.map(effect => `<li>${esc(textOf(effect))}</li>`).join("")}</ul>` : ""}${!compact ? `<div class="gov-ui-actions">${reviewButton(index,data,state,"Review decision",reason ? "" : "gov-ui-primary")}</div>` : ""}</article>`;
  }
  function decisions(data, state) {
    const actions = rows(data.actions).map((action,index) => ({action,index}));
    const available = actions.filter(({action}) => !actionReason(action,data));
    const query = String(state.query || "").trim().toLocaleLowerCase("en-US");
    const matches = ({action}) => [action.label,action.detail,action.blurb,action.category,action.refusal,...rows(action.effects).map(textOf)].some(value => String(value || "").toLocaleLowerCase("en-US").includes(query));
    const filtered = actions.filter(matches), showAll = state.actionFilter === "all", visible = filtered.filter(({action}) => showAll || !actionReason(action,data));
    const unavailable = filtered.filter(({action}) => actionReason(action,data));
    const categories = [...new Set(visible.map(({action}) => action.category || "Government decisions"))];
    return `${sectionTitle("Your choices", "Governing decisions", "Review the political capital cost, effects and conditions before committing.")}<div class="gov-ui-decision-tools"><div class="gov-ui-filters" role="group" aria-label="Decision availability"><button type="button" data-gov-filter="available" aria-pressed="${!showAll}">Available <span>${available.length}</span></button><button type="button" data-gov-filter="all" aria-pressed="${showAll}">All decisions <span>${actions.length}</span></button></div><label class="gov-ui-search"><span>Find a decision</span><input type="search" data-gov-search value="${esc(state.query || "")}" placeholder="Search decisions and conditions" autocomplete="off"></label></div>${!data.mine ? '<p class="gov-ui-message">This is a foreign government. Its decisions are shown for inspection; only your government can take orders.</p>' : ""}<p class="gov-ui-result" role="status">${visible.length} of ${actions.length} decisions shown${query ? ` for “${esc(state.query)}”` : ""}</p>${visible.length ? categories.map(category => `<section class="gov-ui-decision-group"><h3 class="gov-ui-group-title">${esc(String(category).replaceAll("_"," "))}</h3><div class="gov-ui-decision-grid">${visible.filter(({action}) => (action.category || "Government decisions") === category).map(({action,index}) => actionCard(action,index,data,state)).join("")}</div></section>`).join("") : `<div class="gov-ui-empty"><h3>${query ? "No matching decisions" : "No decisions available right now"}</h3><p>${query ? "Try another word or review all decisions." : "Review the conditions below to see what prevents each decision."}</p></div>`}${!showAll && unavailable.length ? `<details class="gov-ui-unavailable" data-gov-detail="unavailable"><summary>${unavailable.length} unavailable ${unavailable.length === 1 ? "decision" : "decisions"} · see the reasons</summary><div class="gov-ui-decision-grid">${unavailable.map(({action,index}) => actionCard(action,index,data,state,true)).join("")}</div></details>` : ""}`;
  }
  function blocs(data) {
    const bar = rows(data.bar);
    if (!bar.length) return '<p class="gov-ui-note">No movement shares are supplied in this reading.</p>';
    return `<div class="gov-ui-bloc-bar" role="img" aria-label="Political movement shares; exact values are listed below">${bar.map(item => `<span style="width:${width(item.share)}%;--bloc-color:${color(item.bloc)}" class="${item.ruling ? "is-ruling" : item.governing ? "is-governing" : ""}"></span>`).join("")}</div><ul class="gov-ui-bloc-legend">${bar.map(item => `<li><i style="--bloc-color:${color(item.bloc)}" aria-hidden="true"></i><strong>${esc(blocLabel(item.bloc))}</strong><span>${pct(item.share)}</span>${item.ruling ? '<b>Rules</b>' : item.governing ? '<b>In government</b>' : ""}${item.banned ? '<b>Banned</b>' : ""}</li>`).join("")}</ul>${bar.some(item => item.backing > 0) ? `<details class="gov-ui-details" data-gov-detail="foreign-backing"><summary>Foreign backing · separate from domestic support</summary>${bar.filter(item => item.backing > 0).map(item => `<div class="gov-ui-backing"><h4>${esc(blocLabel(item.bloc))} · ${pct(item.backing)}</h4><ul>${rows(item.abroad).map(entry => `<li>${entry.kind === "patronage" ? "Patronage" : entry.exposed && entry.sponsor ? `${esc(entry.sponsor)} · exposed support` : "Covert backing · sponsor undisclosed"}${Number.isFinite(entry.weight) ? ` · ${pct(entry.weight)}` : ""}</li>`).join("")}</ul></div>`).join("")}</details>` : ""}`;
  }
  function groups(data, state) {
    const query = String(state.query || "").trim().toLocaleLowerCase("en-US");
    const items = rows(data.groups).map(group => ({...group,parties:rows(group.parties).filter(party => [party.name,party.native,party.family,blocLabel(group.bloc)].some(value => String(value || "").toLocaleLowerCase("en-US").includes(query)))})).filter(group => group.parties.length);
    return `<label class="gov-ui-search gov-ui-party-search"><span>Find a party or movement</span><input type="search" data-gov-search value="${esc(state.query || "")}" placeholder="Search party names or alignment" autocomplete="off"></label>${items.length ? items.map(group => `<section class="gov-ui-party-group"><h3><i class="gov-ui-dot" style="--bloc-color:${color(group.bloc)}" aria-hidden="true"></i>${esc(group.label || blocLabel(group.bloc))}${data.on !== false ? `<span>${pct(group.share)} movement share${group.banned ? " · banned" : ""}</span>` : ""}</h3><div class="gov-ui-table-scroll" role="region" aria-label="${esc(group.label || blocLabel(group.bloc))} party support and seat shares" tabindex="0"><table class="gov-ui-party-table"><caption>Support, seat share and role in government</caption><thead><tr><th scope="col">Party</th><th scope="col">Support</th><th scope="col">Seat share</th><th scope="col">Role</th></tr></thead><tbody>${group.parties.map(party => `<tr class="${party.leads ? "is-leading" : party.in_government ? "is-governing" : ""}"><th scope="row">${esc(party.name)}${party.native && party.native !== party.name ? `<small>${esc(party.native)}</small>` : ""}<small>${esc(party.family)}</small></th><td>${pct(party.support)}</td><td>${pct(party.seats)}</td><td>${party.leads ? "Leads government" : party.in_government ? "In government" : party.pariah ? "Pariah status" : "Outside government"}${party.banned ? " · banned" : ""}</td></tr>`).join("")}</tbody></table></div></section>`).join("") : '<p class="gov-ui-empty">No parties match this reading or search.</p>'}`;
  }
  function politics(data, state) {
    const electoral = !!data.electoral;
    const stats = electoral ? [{label:"Political system",value:data.system},{label:"Government seat share",value:pct(data.government_seats)},{label:"Next election",value:data.next_election},{label:"Term",value:Number.isFinite(data.term_months) ? `${num(data.term_months)} months` : null},{label:"Time in office",value:Number.isFinite(data.months_in_office) ? `${num(data.months_in_office)} months` : null},{label:"Coalition strain",value:data.strain},{label:"Coalition upkeep",value:Number.isFinite(data.upkeep) ? `${num(data.upkeep)} PC / month` : null}] : [{label:"Ruling institution",value:data.ruling_institution},{label:"Coup pressure",value:data.coup_pressure}];
    return `${sectionTitle("Political structure",electoral ? "Parliament & parties" : "Regime & institutions",electoral ? "See the governing coalition, opposition support and shares of the chamber." : "Review the institutions sustaining the regime and its political movements.")}${metrics(stats)}${data.on !== false ? `<section class="gov-ui-panel">${sectionTitle("Movements", "The balance of support")}${blocs(data)}</section>` : `<p class="gov-ui-message">The ideological movement lens is off. Ordinary government institutions, cabinet decisions and elections remain active.</p>`}${!electoral ? `<section class="gov-ui-panel">${sectionTitle("Institutional support", "Pillars of the regime")}<div class="gov-ui-pillars">${rows(data.pillars).map(pillar => `<article><div><h3>${esc(pillar.name || pillar.key)}</h3><strong>${pct(pillar.loyalty)} loyal</strong></div><p>${esc(pillar.key)} · Installs ${esc(blocLabel(pillar.bloc))}</p><div class="gov-ui-meter" aria-hidden="true"><span style="width:${width(pillar.loyalty)}%;--meter-color:${color(pillar.bloc)}"></span></div></article>`).join("") || '<p class="gov-ui-note">No institutional loyalty readings are available.</p>'}</div></section>` : ""}<section class="gov-ui-panel">${sectionTitle(electoral ? "The chamber" : "Political organizations",electoral ? "Parties in the political system" : "Dormant party table", "Support and seats are percentages, not counts of individual seats.")}${groups(data,state)}</section>`;
  }
  function gaugeValue(gauge) {
    const big = gauge.trigger > 1 || (gauge.upper != null && gauge.upper > 1);
    const format = value => big ? num(value) : pct(value);
    const relation = gauge.sense === "Above" ? "at least" : gauge.sense === "Below" ? "at most" : gauge.sense === "Inside" ? "between" : "threshold";
    const target = gauge.sense === "Inside" ? `${format(gauge.trigger)} and ${format(gauge.upper)}` : format(gauge.trigger);
    return `${format(gauge.value)} · ${relation} ${target}`;
  }
  function watch(data) {
    if (data.on === false) return unavailablePolitics();
    const takeover = data.takeover || {};
    return `${sectionTitle("Political conditions", "Takeover watch", "These are the conditions for each route to political change. Gauge progress is not a probability or a prediction.")}<div class="gov-ui-watch-grid">${[["coup","Coup"],["uprising","Uprising"],["round_table","Round table"],["collapse","Collapse"]].map(([key,label]) => {
      const road = takeover[key];
      if (!road) return `<article class="gov-ui-watch-card"><h3>${label}</h3><p>No conditions are available in this reading.</p></article>`;
      return `<article class="gov-ui-watch-card${road.half_armed ? " has-attention" : ""}"><div class="gov-ui-watch-heading"><h3>${label}</h3><span class="gov-ui-badge">${road.open ? "Route enabled" : "Route closed"}</span></div>${road.reason ? `<p class="gov-ui-watch-reason">${esc(road.reason)}</p>` : ""}${road.armed ? '<p class="gov-ui-condition-summary">All reported conditions met</p>' : road.half_armed ? '<p class="gov-ui-condition-summary">A condition has reached the watch level</p>' : ""}<details class="gov-ui-details" data-gov-detail="watch:${key}"><summary>Inspect ${rows(road.gauges).length} ${rows(road.gauges).length === 1 ? "condition" : "conditions"}</summary><div class="gov-ui-gauges">${rows(road.gauges).map(gauge => `<div><h4>${esc(gauge.name)}${gauge.bloc ? ` · ${esc(blocLabel(gauge.bloc))}` : ""}</h4><p>${esc(gaugeValue(gauge))}</p><span class="gov-ui-gauge-status">${gauge.met ? "Condition met" : "Condition not met"}</span>${Number.isFinite(gauge.progress) ? `<div class="gov-ui-meter" aria-hidden="true"><span style="width:${width(gauge.progress)}%;--meter-color:${gauge.met ? "#e5bc89" : "#91a7b1"}"></span></div>` : ""}</div>`).join("")}</div></details></article>`;
    }).join("")}</div>`;
  }
  function unavailablePolitics() {
    return '<section class="gov-ui-empty"><h2>The ideological takeover watch is inactive</h2><p>The ideological movement lens is off in this campaign. Ordinary elections, cabinet failures and institutional regime coups remain active; review Parliament &amp; parties or Regime &amp; institutions for the current government.</p><button type="button" data-gov-tab="overview">Return to overview</button></section>';
  }
  function review(data, state) {
    const current = state.review;
    if (!current) return "";
    const q = current.data, index = current.index, action = rows(data.actions)[index];
    const fresh = !current.loading && !current.error && !!q;
    const canConfirm = fresh && q.valid === true && Number.isFinite(q.price_pc) && q.price_pc >= 0 && !!q.command && Number.isInteger(index) && !!action && !actionReason(action,data) && !locked(state);
    return `<section class="gov-ui-review" id="govReview" role="region" aria-labelledby="govReviewTitle"><div class="gov-ui-review-heading"><div><p class="gov-ui-kicker">Review before committing</p><h2 id="govReviewTitle" tabindex="-1">${esc(q?.title || action?.label || "Government decision")}</h2></div><button type="button" data-gov-review-close aria-label="Close decision review">Close review</button></div>${current.loading ? '<p class="gov-ui-message" role="status">Checking the current cost, conditions and effects…</p>' : ""}${current.error ? `<div class="gov-ui-message is-error" role="alert"><p>${esc(current.error)}</p><button type="button" data-gov-retry>Retry decision review</button></div>` : ""}${fresh ? `${q.description ? `<p>${esc(q.description)}</p>` : ""}${q.reason ? `<p class="gov-ui-refusal" role="status">${esc(q.reason)}</p>` : ""}<dl class="gov-ui-review-costs"><div><dt>Political capital cost</dt><dd>${Number.isFinite(q.price_pc) ? `${num(q.price_pc)} PC` : "—"}</dd></div>${q.money_cost_bn != null ? `<div><dt>Financial cost</dt><dd>${esc(money(q.money_cost_bn))}</dd></div>` : ""}</dl>${rows(q.changes).length ? `<div class="gov-ui-table-scroll" role="region" tabindex="0" aria-label="Decision effects before and after"><table class="gov-ui-change-table"><caption>What this decision changes</caption><thead><tr><th scope="col">Measure</th><th scope="col">Before</th><th scope="col">After</th></tr></thead><tbody>${q.changes.map(change => `<tr><th scope="row">${esc(change.label || change.key)}${change.detail ? `<small>${esc(change.detail)}</small>` : ""}</th><td>${esc(valueText(change.before))}</td><td>${esc(valueText(change.after))}</td></tr>`).join("")}</tbody></table></div>` : ""}${rows(q.effects).length ? `<h3>Reported effects</h3><ul class="gov-ui-effects">${q.effects.map(effect => `<li>${esc(textOf(effect))}</li>`).join("")}</ul>` : ""}${rows(q.warnings).length ? `<ul class="gov-ui-warnings">${q.warnings.map(warning => `<li>${esc(textOf(warning))}</li>`).join("")}</ul>` : ""}` : ""}<div class="gov-ui-actions"><button type="button" class="gov-ui-primary" data-gov-confirm="${Number.isInteger(index) ? index : ""}" ${canConfirm ? "" : "disabled"}>Confirm decision</button><button type="button" data-gov-review-close>Keep considering</button></div></section>`;
  }
  function render(data, state = {}) {
    if (!data || data.error) return `<div class="gov-ui"><section class="gov-ui-empty" ${data?.error ? 'role="alert"' : 'role="status"'}><p class="gov-ui-kicker">Government</p><h1>${data?.error ? "The government reading could not be loaded" : "Preparing your government briefing"}</h1><p>${esc(data?.error || "Reading the current administration, political conditions and available decisions…")}</p>${data?.error ? '<button type="button" data-gov-refresh>Retry government reading</button>' : ""}</section></div>`;
    const tab = TABS.some(([id]) => id === state.tab) ? state.tab : "overview";
    const body = tab === "overview" ? overview(data,state) : tab === "decisions" ? decisions(data,state) : tab === "politics" ? politics(data,state) : watch(data);
    return `<div class="gov-ui" data-gov-section="${tab}">${hero(data,tab)}${navigation(tab,data)}${state.notice ? `<p class="gov-ui-message" role="status">${esc(state.notice)}</p>` : ""}${locked(state) ? `<p class="gov-ui-message" role="status">${state.review?.loading ? 'Checking the decision. Other actions are temporarily unavailable.' : 'An order or turn is being resolved. Review the reading while actions are unavailable.'}</p>` : ""}${review(data,state)}<div id="gov-panel-${tab}" role="tabpanel" aria-labelledby="gov-tab-${tab}" tabindex="0">${body}</div><footer class="gov-ui-footer"><p>${esc(data.briefing?.date_label || "Current government reading")}${!data.mine ? " · Foreign government, view only" : ""}</p><button type="button" data-gov-refresh ${locked(state) ? "disabled" : ""}>Refresh reading</button></footer></div>`;
  }
  return Object.freeze({render});
});
