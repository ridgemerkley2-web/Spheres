/* Government presentation only. Political conditions, prices, effects and commands
   come from the server. The host owns navigation, stale-state guards and orders. */
(function (root, factory) {
  const api = factory();
  if (typeof module !== "undefined" && module.exports) module.exports = api;
  root.GovernmentUI = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";
  const TABS = [["overview", "Overview"], ["decisions", "Governing decisions"], ["politics", "Parliament & parties"], ["leadership", "Party leadership"], ["watch", "Takeover watch"]];
  const tabsFor = data => TABS.filter(([id]) => id !== "leadership" || data.party_leadership !== undefined);
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
    return `<header class="gov-ui-hero${tab !== "overview" ? " gov-ui-hero-compact" : ""}"><img class="gov-ui-council-art" src="/art/government/council-v1.png" alt="" aria-hidden="true" width="1536" height="1024" decoding="async" draggable="false"><div class="gov-ui-hero-copy"><p class="gov-ui-kicker">${data.mine ? "Your government" : "Foreign government · view only"}${briefing.date_label ? ` · ${esc(briefing.date_label)}` : ""}</p><h1>${esc(data.nation_name || data.nation || "Government")}</h1><p class="gov-ui-hero-summary">${esc(briefing.summary || (data.on === false ? "Review your country's administration and open the government tools available in this campaign." : "Understand who holds power, review your decisions and follow the conditions shaping political change."))}</p>${(leader.name || leader.described) ? `<p class="gov-ui-leader-line"><strong>${esc(leader.name || leader.described)}</strong>${fictionOf(data.party_leadership?.executive_person) ? ' <span class="gov-ui-fiction-badge">Fictional successor</span>' : ""}${leader.office ? ` <span>${esc(leader.office)}</span>` : ""}</p>` : ""}<div class="gov-ui-hero-actions"><button type="button" class="gov-ui-primary" data-gov-tab="decisions">${data.mine ? "Review governing decisions" : "Inspect governing decisions"}</button>${routeButton(data.mine ? "budget" : "mine", data.mine ? "Open national budget" : "Return to your government", data)}</div></div></header>`;
  }
  function navigation(tab, data) {
    return `<nav class="gov-ui-tabs" role="tablist" aria-label="Government sections">${tabsFor(data).map(([id,label]) => `<button type="button" id="gov-tab-${id}" role="tab" data-gov-tab="${id}" aria-controls="gov-panel-${id}" aria-selected="${tab === id}" tabindex="${tab === id ? 0 : -1}">${esc(id === "politics" && !data.electoral ? "Regime & institutions" : label)}</button>`).join("")}</nav>`;
  }
  function ruler(data) {
    const leader = data.leader || {};
    const person = data.party_leadership?.executive_person, portrait = person?.portrait || {};
    const evidence = fictionOf(person) ? `<details class="gov-ui-person-evidence" data-gov-detail="executive-fiction"><summary>Fictional background &amp; research basis</summary>${fictionEvidence(person)}${avatarAttribution(portrait)}</details>` : portraitUrl(portrait.url) ? `<details class="gov-ui-person-evidence" data-gov-detail="executive-avatar"><summary>Avatar &amp; sources</summary>${avatarAttribution(portrait)}${sourceList([...rows(person.sources),portrait.source_url])}</details>` : "";
    return `<section class="gov-ui-panel">${sectionTitle("Leadership", "Who holds power")}<div class="gov-ui-leadership">${person ? personPortrait(person) : ""}<h3>${esc(leader.name || leader.described || data.ruling_institution || "No leader recorded")}</h3>${fictionSummary(person)}${leader.native && leader.native !== leader.name ? `<p class="gov-ui-native">${esc(leader.native)}</p>` : ""}<p>${esc(leader.office || data.system || "")}${leader.since ? ` · Since ${esc(leader.since)}` : ""}</p>${data.on !== false && data.ruling_bloc ? `<span class="gov-ui-badge"><i style="--bloc-color:${color(data.ruling_bloc)}" aria-hidden="true"></i>${esc(blocLabel(data.ruling_bloc))}</span>` : ""}${data.government_of_the_day ? `<p class="gov-ui-note">Governing party: ${esc(data.government_of_the_day_name || data.government_of_the_day)}</p>` : ""}${leader.must_leave_by ? `<p class="gov-ui-note">Must leave office by ${esc(leader.must_leave_by)}</p>` : ""}${leader.heir ? `<p class="gov-ui-note">Successor: ${esc(leader.heir.name)}${leader.heir.office ? ` · ${esc(leader.heir.office)}` : ""}${leader.heir.since ? ` · Since ${esc(leader.heir.since)}` : ""}</p>` : ""}${rows(leader.also).map(person => `<p class="gov-ui-note">${esc(person.name)} · ${esc(person.office)}${person.since ? ` · Since ${esc(person.since)}` : ""}</p>`).join("")}${evidence}</div></section>`;
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
  const isoDay = value => typeof value === "string" && /^\d{4}-\d{2}-\d{2}$/.test(value);
  const sourceUrl = value => typeof value === "string" && /^https:\/\/[^\s\\<>"']+$/.test(value) ? value : "";
  const portraitUrl = value => typeof value === "string" && /^\/art\/[a-zA-Z0-9_/-]+\.(?:png|webp|jpg|jpeg)$/.test(value) && !value.includes("..") ? value : "";
  const successionReason = value => ({initial_reference:"In office when this campaign began",election:"Seated after a campaign election",coup:"Seated after a campaign coup",takeover:"Seated after a change of regime",term_limit:"Succession after a term limit",death:"Succession after an officeholder's death",programme:"Seated through the government programme"})[value] || value;
  function dateBound(value) {
    if (!value || value.kind === "unknown") return "Date not established";
    if (value.kind === "open") return "Current at the research cutoff";
    if (value.kind === "year" && /^\d{4}$/.test(value.value)) return `${value.value} · year recorded`;
    if (value.kind === "month" && /^\d{4}-\d{2}$/.test(value.value)) return `${value.value} · month recorded`;
    if (value.kind === "day" && isoDay(value.value)) return value.value;
    return "Date not established";
  }
  function sourceList(values) {
    const links = [...new Set(rows(values).map(sourceUrl).filter(Boolean))];
    return links.length ? `<ul class="gov-ui-leader-sources">${links.map((url,index) => `<li><a href="${esc(url)}" target="_blank" rel="noopener noreferrer">${esc(url.split("/")[2])} <span>· source ${index + 1}</span><span class="gov-ui-sr-only"> (opens in a new tab)</span></a></li>`).join("")}</ul>` : '<p class="gov-ui-note">No source links were supplied for this record.</p>';
  }
  function personPortrait(person) {
    const portrait = person?.portrait, url = portraitUrl(portrait?.url), fictional = !!fictionOf(person);
    const label = fictional ? "Fictional cartoon avatar" : "Cartoon avatar";
    const initials = String(person?.name || "").trim().split(/\s+/).slice(0,2).map(word => Array.from(word)[0] || "").join("");
    return url ? `<figure class="gov-ui-person-art gov-ui-cartoon-art"><img src="${esc(url)}" alt="${label} of ${esc(person.name || "the recorded person")}" width="240" height="360" loading="lazy" decoding="async"><figcaption>${label}${portrait.from ? `<span>${fictional ? "Imagined appearance" : "Appearance reference"}: from ${esc(valueText(portrait.from))}${portrait.to && portrait.to !== portrait.from ? ` to before ${esc(valueText(portrait.to))}` : ""}</span>` : ""}</figcaption></figure>` : `<div class="gov-ui-person-art gov-ui-art-pending"><span aria-hidden="true">${esc(initials || "?")}</span><p>${fictionOf(person) ? "Fictional avatar pending" : person?.name ? "Avatar not yet available" : "No person recorded"}</p></div>`;
  }
  const fictionOf = person => person?.fiction?.origin === "fictional_successor" ? person.fiction : null;
  function fictionSummary(person) {
    const fiction = fictionOf(person);
    return fiction ? `<span class="gov-ui-fiction-badge">Fictional successor</span>${fiction.profile_label ? `<p class="gov-ui-fiction-profile">${esc(fiction.profile_label)}${fiction.ideology ? ` · ${esc(fiction.ideology)}` : ""}</p>` : ""}${fiction.fictional_biography ? `<p class="gov-ui-fiction-biography">${esc(fiction.fictional_biography)}</p>` : ""}` : "";
  }
  function fictionEvidence(person) {
    const fiction = fictionOf(person);
    if (!fiction) return "";
    return `<p class="gov-ui-note">This person, biography and birth date are invented game content. Research informs the background; it does not establish a real person or predict who will win.</p>${person.born ? `<p class="gov-ui-note">Fictional birth date: ${esc(dateBound(person.born))}</p>` : ""}${isoDay(fiction.eligible_from) ? `<p class="gov-ui-note">Available from ${esc(fiction.eligible_from)} through 2035, subject to campaign events.</p>` : ""}${rows(fiction.research_basis).map(basis => `<div class="gov-ui-fiction-basis">${basis.fact ? `<p class="gov-ui-note">${esc(basis.fact)}</p>` : ""}${basis.applies_to ? `<p class="gov-ui-note">${esc(basis.applies_to)}</p>` : ""}${sourceList([basis.url])}</div>`).join("")}${rows(fiction.assumptions).length ? `<ul class="gov-ui-coverage-gaps">${fiction.assumptions.map(note => `<li>${esc(note)}</li>`).join("")}</ul>` : ""}${fiction.editorial_status === "authored_fiction_needs_country_review" ? '<p class="gov-ui-note">Country-specific editorial review is still pending.</p>' : ""}`;
  }
  function futureEntries(party) {
    const candidates = rows(party.future_candidates), preview = rows(party.future_preview);
    return (candidates.length ? candidates : preview).filter(entry => fictionOf(entry.person) && entry.component_available !== false);
  }
  function futureSeatInfo(party) {
    const seats = party?.future_leadership_seats;
    if (!seats || seats.target_holders !== 2) return "";
    return `<div class="gov-ui-future-seats"><p class="gov-ui-person-role">Future succession: two co-leaders</p><p class="gov-ui-note">${seats.historical_single_chair_retained ? "The existing single chair remains in office. A pair is selected when that leadership becomes vacant." : "Serving co-leaders keep their appointments. Succession fills only the vacant positions."} The calendar does not replace a leader.</p>${seats.institutional_fact ? `<p class="gov-ui-note">${esc(seats.institutional_fact)}</p>` : ""}${seats.gameplay_assumption ? `<p class="gov-ui-note">${esc(seats.gameplay_assumption)}</p>` : ""}${seats.qualification_note ? `<p class="gov-ui-note">${esc(seats.qualification_note)}</p>` : ""}${sourceList(seats.sources)}</div>`;
  }
  function historicalContinuation(entry) {
    const disclosure = entry?.historical_continuation;
    if (!disclosure || !["ceased", "unverified"].includes(disclosure.status)) return "";
    const label = disclosure.status === "ceased" ? "Historical organization has closed" : "Historical continuation needs review";
    return `<div class="gov-ui-future-seats"><p class="gov-ui-person-role">${label}</p>${disclosure.note ? `<p class="gov-ui-note">${esc(disclosure.note)}</p>` : ""}<p class="gov-ui-note">A party that survives in your campaign can follow a different path. This historical note does not remove serving leaders or schedule a successor.</p>${sourceList(disclosure.sources)}</div>`;
  }
  function futureDisclosure(party, mode, board, key) {
    if (mode === "reference" || !board.future_policy) return "";
    const entries = futureEntries(party);
    return `<details class="gov-ui-leader-disclosure gov-ui-future-disclosure" data-gov-detail="leader-future:${esc(key)}"><summary>Future candidates through 2035 <span>${entries.length}</span></summary><p class="gov-ui-note">Read-only preview of fictional successors. Historical people are researched through ${esc(board.future_policy.historical_reference_through || "2026-09-07")}; these candidates enter the succession pool from ${esc(board.future_policy.from || "2026-09-08")}. These future paths depend on the party or component remaining active in your campaign. Your campaign's existing leaders remain in place until an actual succession event.</p>${futureSeatInfo(party)}${entries.length ? entries.map((entry,i) => leadershipPerson({...entry,eligible:board.enabled === true && entry.eligible === true},party,"future",`${key}:future:${i}`)).join("") : '<p class="gov-ui-note">No fictional candidate preview is supplied for this party yet. No future artwork is implied.</p>'}</details>`;
  }
  function futurePolicy(board, isReference) {
    const policy = board?.future_policy;
    if (!policy) return "";
    return `<aside class="gov-ui-future-policy"><p class="gov-ui-kicker">Historical people · Fictional future</p><p>Real people through ${esc(policy.historical_reference_through || "2026-09-07")}. Research-informed fictional successors from ${esc(policy.from || "2026-09-08")} through 2035. ${isReference ? "This historical view contains source records only; future previews are in Your campaign." : "Open a party's future candidates to explore its cast. Succession follows campaign events; no election winner is scheduled."}</p></aside>`;
  }
  function termDescription(term) {
    if (!term) return "";
    const kind = term.kind === "acting" ? "Acting leader" : term.kind === "co_leader" ? "Co-leader" : term.kind === "candidate" ? "Historical candidate" : "Historical leader";
    return `<p class="gov-ui-person-role">${esc(term.role || kind)}${term.role ? ` <span>· ${kind}</span>` : ""}</p><p class="gov-ui-person-dates">${esc(dateBound(term.from))} <span aria-hidden="true">→</span><span class="gov-ui-sr-only"> until </span> ${esc(dateBound(term.until))}</p>`;
  }
  function executiveRole(entry) {
    const policy = entry?.executive_eligibility;
    if (!policy || typeof policy.authorized !== "boolean") return "";
    const names = {presidential_contender:"Presidential contender",parliamentary_government_contender:"Government contender",national_executive_contender:"National office contender",legacy_gameplay_contender:"National office eligible · game rule"};
    const label = policy.authorized ? names[policy.role] || "National office contender" : "Party office only";
    return `<p class="gov-ui-executive-role${policy.authorized ? " is-authorized" : ""}">${esc(label)}</p>`;
  }
  function executiveEvidence(entry) {
    const policy = entry?.executive_eligibility;
    if (!policy) return "";
    return `${policy.note ? `<p class="gov-ui-note">${esc(policy.note)}</p>` : ""}${policy.condition ? `<p class="gov-ui-note">${esc(policy.condition)}</p>` : ""}${sourceList(policy.sources)}`;
  }
  function executivePolicy(board) {
    const policy = board?.executive_policy;
    if (!policy) return "";
    const institution = policy.institution || {};
    return `<details class="gov-ui-leader-disclosure gov-ui-office-policy" data-gov-detail="leadership-office-policy"><summary>How party roles affect government</summary><p class="gov-ui-note">Party succession and national office have separate requirements. A party chair is not automatically a president or prime minister. Existing campaign appointments remain in place.</p>${institution.fact ? `<p class="gov-ui-note">${esc(institution.fact)}</p>` : ""}${institution.gameplay_assumption ? `<p class="gov-ui-note">${esc(institution.gameplay_assumption)}</p>` : ""}${sourceList(institution.sources)}</details>`;
  }
  function avatarAttribution(portrait) {
    const linked = (label,url) => sourceUrl(url) ? `<a href="${esc(url)}" target="_blank" rel="noopener noreferrer">${esc(label)}<span class="gov-ui-sr-only"> (opens in a new tab)</span></a>` : esc(label);
    return `${portrait.era_note ? `<p class="gov-ui-note">${esc(portrait.era_note)}</p>` : ""}${portrait.credit ? `<p class="gov-ui-note">Avatar: ${esc(portrait.credit)}</p>` : ""}${portrait.license && portrait.license !== "generated" ? `<p class="gov-ui-note">Artwork license: ${linked(portrait.license,portrait.license_url)}</p>` : ""}${portrait.source_credit ? `<p class="gov-ui-note">Reference image: ${esc(portrait.source_credit)}</p>` : ""}${portrait.source_license ? `<p class="gov-ui-note">Reference image license: ${linked(portrait.source_license,portrait.source_license_url)}</p>` : ""}`;
  }
  function leadershipPerson(entry, party, context, key) {
    const person = entry?.person;
    if (!person) return `<article class="gov-ui-person is-missing"><div><h4>Identity not established</h4><p class="gov-ui-note">This record does not name a verified person.</p></div></article>`;
    const component = rows(party.components).find(component => component.id === (entry.component || entry.term?.component));
    const portrait = person.portrait || {}, portraitSource = sourceUrl(portrait.source_url);
    const isCampaign = context === "campaign", fiction = fictionOf(person);
    const sources = [...rows(entry.term?.sources),...rows(person.sources),...(portraitSource ? [portraitSource] : [])];
    const role = isCampaign ? `<p class="gov-ui-person-role">${esc(entry.role || "Campaign officeholder")}${entry.since_label ? ` · Since ${esc(entry.since_label)}` : ""}</p>${entry.reason ? `<p class="gov-ui-note">${esc(successionReason(entry.reason))}</p>` : ""}${!fiction && entry.historical_reference_continues === false ? '<p class="gov-ui-campaign-diverged">Your campaign has continued beyond this historical term.</p>' : ""}` : fiction ? `<p class="gov-ui-person-role">${entry.eligible === true ? "Eligible for future campaign succession" : "Future cast preview · no appointment"}</p>` : termDescription(entry.term);
    const evidence = fiction ? fictionEvidence(person) : `${person.born ? `<p class="gov-ui-note">Born: ${esc(dateBound(person.born))}</p>` : ""}${person.died ? `<p class="gov-ui-note">Historical death: ${esc(dateBound(person.died))}</p>` : ""}${entry.term?.note ? `<p class="gov-ui-note">${esc(entry.term.note)}</p>` : ""}${sourceList(sources)}`;
    return `<article class="gov-ui-person${fiction ? " gov-ui-fiction-person" : ""}" data-gov-person="${esc(person.id || entry.person_id || key)}">${personPortrait(person)}<div class="gov-ui-person-copy">${component ? `<p class="gov-ui-kicker">${esc(component.name)}</p>` : ""}<h4>${esc(person.name || "Name not supplied")}</h4>${fictionSummary(person)}${person.native && person.native !== person.name ? `<p class="gov-ui-native">${esc(person.native)}</p>` : ""}${role}${executiveRole(entry)}${historicalContinuation(entry)}${person.age_label && context !== "future" ? `<p class="gov-ui-note">${esc(person.age_label)}</p>` : ""}<details class="gov-ui-person-evidence" data-gov-detail="leader-person:${esc(key)}"><summary>${fiction ? "Fictional background, avatar &amp; research basis" : "Dates, avatar &amp; sources"}</summary>${evidence}${executiveEvidence(entry)}${avatarAttribution(portrait)}</details></div></article>`;
  }
  function partyLeadershipCard(party, mode, board, index) {
    const campaign = rows(party.campaign), historical = rows(party.historical), eligible = rows(party.eligible), uncertain = rows(party.uncertain_historical);
    const showCampaign = mode === "campaign" && board.enabled === true;
    const displayed = showCampaign ? campaign : historical;
    const key = String(party.party_id || index), coverage = party.coverage === "verified" ? "Sourced coverage" : party.coverage === "partial" ? "Partial research" : "Research needed";
    const gaps = rows(party.gaps);
    return `<article class="gov-ui-party-leadership-card" data-gov-leadership-party="${esc(key)}"><header><div><p class="gov-ui-kicker">${party.kind === "coalition" ? "Coalition / combined game row" : party.kind === "collective" ? "Collective organization" : "Political party"}</p><h3>${esc(party.party_name || key)}</h3></div><span class="gov-ui-badge">${coverage}</span></header>${party.identity_note ? `<p class="gov-ui-identity-note">${esc(party.identity_note)}</p>` : ""}<div class="gov-ui-party-leaders">${displayed.length ? displayed.map((entry,i) => leadershipPerson(entry,party,showCampaign ? "campaign" : "historical",`${key}:${mode}:${i}`)).join("") : `<div class="gov-ui-leader-gap"><h4>${showCampaign ? "No campaign leader recorded" : "No verified leader for this date"}</h4><p>${esc(party.reason || (showCampaign ? "This party has no saved leader assignment. Historical coverage is shown below where available." : "The sources do not establish an officeholder at the selected date. No identity has been invented."))}</p></div>`}</div>${showCampaign && historical.length ? `<details class="gov-ui-leader-disclosure" data-gov-detail="leader-reference:${esc(key)}"><summary>Historical reference at the campaign date <span>${historical.length}</span></summary>${historical.map((entry,i) => leadershipPerson(entry,party,"historical",`${key}:historical:${i}`)).join("")}</details>` : ""}${uncertain.length ? `<details class="gov-ui-leader-disclosure" data-gov-detail="leader-uncertain:${esc(key)}"><summary>Date uncertainty · possible historical records <span>${uncertain.length}</span></summary><p class="gov-ui-note">These records may overlap the selected date. They do not establish an officeholder or an eligible candidate for that date.</p>${uncertain.map((entry,i) => `<div>${entry.reason ? `<p class="gov-ui-note">${esc(successionReason(entry.reason))}</p>` : ""}${leadershipPerson(entry,party,"uncertain",`${key}:uncertain:${i}`)}</div>`).join("")}</details>` : ""}<details class="gov-ui-leader-disclosure" data-gov-detail="leader-candidates:${esc(key)}"><summary>${mode === "reference" ? "Historical party succession candidates" : "Party succession candidates"} <span>${eligible.length}</span></summary><p class="gov-ui-note">Eligibility for party succession comes from the researched dates and current rules. National office requires a separate eligible role shown on each card. This list does not schedule an election winner.</p>${eligible.length ? eligible.map((entry,i) => leadershipPerson(entry,party,"candidate",`${key}:candidate:${i}`)).join("") : '<p class="gov-ui-note">No verified eligible candidate is supplied for this date.</p>'}</details><details class="gov-ui-leader-disclosure" data-gov-detail="leader-coverage:${esc(key)}"><summary>Research coverage &amp; party sources${gaps.length ? ` <span>${gaps.length} ${gaps.length === 1 ? "gap" : "gaps"}</span>` : ""}</summary>${gaps.length ? `<ul class="gov-ui-coverage-gaps">${gaps.map(gap => `<li><p>${esc(gap.reason || "Coverage is incomplete.")}</p>${gap.from?.kind !== "unknown" && gap.until?.kind !== "unknown" ? `<small>${esc(dateBound(gap.from))} → ${esc(dateBound(gap.until))}</small>` : ""}${rows(gap.sources).length ? sourceList(gap.sources) : ""}</li>`).join("")}</ul>` : `<p class="gov-ui-note">${party.coverage === "verified" ? "No coverage gaps are recorded for this party." : "Research is incomplete. A missing leader or avatar does not mean this party did not exist."}</p>`}${sourceList(party.sources)}</details>${futureDisclosure(party,mode,board,key)}</article>`;
  }
  function leadership(data, state) {
    const campaign = data.party_leadership, mode = state.leadershipMode === "reference" ? "reference" : "campaign";
    const reference = state.leadershipReference || {}, isReference = mode === "reference";
    const minDate = isoDay(campaign?.reference_from) ? campaign.reference_from : "1990-01-01";
    const maxDate = isoDay(campaign?.reference_through) ? campaign.reference_through : "2026-09-07";
    const chosenDate = isoDay(state.leadershipDate) ? state.leadershipDate : minDate;
    const minYear = Number(minDate.slice(0,4)), maxYear = Number(maxDate.slice(0,4));
    const yearOptions = Number.isInteger(minYear) && Number.isInteger(maxYear) && maxYear >= minYear && maxYear - minYear < 100 ? Array.from({length:maxYear-minYear+1},(_,i) => minYear+i) : [1990];
    const controls = `<div class="gov-ui-leadership-tools"><div class="gov-ui-filters" role="group" aria-label="Leadership record"><button type="button" data-gov-leadership-mode="campaign" aria-pressed="${!isReference}">Your campaign</button><button type="button" data-gov-leadership-mode="reference" aria-pressed="${isReference}">Historical reference</button></div>${isReference ? `<div class="gov-ui-reference-date"><label><span>Reference year</span><select data-gov-leadership-year>${yearOptions.map(year => `<option value="${year}"${String(year) === chosenDate.slice(0,4) ? " selected" : ""}>${year}</option>`).join("")}</select></label><label><span>Exact reference date</span><input type="date" data-gov-leadership-date min="${esc(minDate)}" max="${esc(maxDate)}" value="${esc(chosenDate)}"></label></div>` : ""}<label class="gov-ui-search"><span>Find a party or person</span><input type="search" data-gov-leadership-search value="${esc(state.leadershipQuery || "")}" placeholder="Party, component or leader" autocomplete="off"></label></div>`;
    const heading = sectionTitle("People behind the parties", "Party leadership", "Every party has a place here, including small partners and the separate organizations within combined game rows.");
    const context = `<p class="gov-ui-message">${isReference ? `Historical reference · Read only. Browsing a date does not change your campaign or appoint a leader. Sources are checked through ${esc(maxDate)}.` : "Campaign leaders follow events in your game. Historical succession dates do not replace a saved incumbent."}</p>`;
    const refBoard = reference.data;
    const mismatched = isReference && refBoard && ((campaign?.nation != null && refBoard.nation != null && campaign.nation !== refBoard.nation) || (isoDay(refBoard.date) && refBoard.date !== chosenDate));
    if (isReference && (reference.loading || (!reference.error && (!refBoard || mismatched)))) return `${heading}${controls}${context}<div class="gov-ui-empty" role="status"><h3>Loading the historical reference</h3><p>Reading the selected nation and date…</p></div>`;
    const board = isReference ? refBoard : campaign;
    const error = isReference ? reference.error || board?.error : board?.error;
    if (error || !board) return `${heading}${controls}${context}<div class="gov-ui-empty" role="alert"><h3>Party leadership could not be read</h3><p>${esc(error || "No party leadership reading was supplied.")}</p><button type="button" ${isReference ? "data-gov-leadership-retry" : "data-gov-refresh"}>Retry leadership reading</button></div>`;
    const all = rows(board.parties), query = String(state.leadershipQuery || "").trim().toLocaleLowerCase("en-US");
    const shown = all.filter(party => [party.party_name,party.party_id,party.identity_note,...rows(party.components).map(component => component.name),...rows(party.campaign).concat(rows(party.historical),rows(party.eligible),rows(party.uncertain_historical),isReference ? [] : futureEntries(party)).flatMap(entry => [entry.person?.name,entry.person?.native])].some(value => String(value || "").toLocaleLowerCase("en-US").includes(query)));
    return `${heading}${controls}${context}${futurePolicy(board,isReference)}${executivePolicy(board)}${!isReference && board.enabled === false ? '<p class="gov-ui-message">Historical party succession is off in this campaign. The cards show sourced historical references; they are not saved campaign appointments.</p>' : ""}<p class="gov-ui-result" role="status">${shown.length} of ${all.length} ${all.length === 1 ? "party" : "parties"} shown · ${esc(board.date || (isReference ? chosenDate : "Campaign date"))}${query ? ` · Search: “${esc(state.leadershipQuery)}”` : ""}</p>${shown.length ? `<div class="gov-ui-leadership-grid">${shown.map((party,index) => partyLeadershipCard(party,mode,board,index)).join("")}</div>` : `<div class="gov-ui-empty"><h3>${query ? "No matching parties or people" : "No party records supplied"}</h3><p>${query ? "Try a party name, component organization or leader." : "This reading contains no party catalogue. No identities have been inferred."}</p></div>`}`;
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
    const tab = tabsFor(data).some(([id]) => id === state.tab) ? state.tab : "overview";
    const body = tab === "overview" ? overview(data,state) : tab === "decisions" ? decisions(data,state) : tab === "politics" ? politics(data,state) : tab === "leadership" ? leadership(data,state) : watch(data);
    return `<div class="gov-ui" data-gov-section="${tab}">${hero(data,tab)}${navigation(tab,data)}${state.notice ? `<p class="gov-ui-message" role="status">${esc(state.notice)}</p>` : ""}${locked(state) ? `<p class="gov-ui-message" role="status">${state.review?.loading ? 'Checking the decision. Other actions are temporarily unavailable.' : 'An order or turn is being resolved. Review the reading while actions are unavailable.'}</p>` : ""}${tab === "leadership" ? "" : review(data,state)}<div id="gov-panel-${tab}" role="tabpanel" aria-labelledby="gov-tab-${tab}" tabindex="0">${body}</div><footer class="gov-ui-footer"><p>${esc(data.briefing?.date_label || "Current government reading")}${!data.mine ? " · Foreign government, view only" : ""}</p><button type="button" data-gov-refresh ${locked(state) ? "disabled" : ""}>Refresh reading</button></footer></div>`;
  }
  return Object.freeze({render});
});
