/* Theatre intent and staff reports. All forecasts, observations and outcomes
   are supplied by the simulation; this module only formats and submits them. */
(function (root, factory) {
  "use strict";
  const ui = factory();
  if (typeof module === "object" && module.exports) module.exports = ui;
  root.CampaignOperationsUI = ui;
})(typeof globalThis === "undefined" ? this : globalThis, function () {
  "use strict";
  const approaches = [
    ["probe", "Probe", "Test resistance"],
    ["advance", "Advance", "Sustain an offensive"],
    ["breakthrough", "Breakthrough", "Concentrate the attack"],
    ["hold", "Hold", "Prepare and recover"],
    ["fighting_withdrawal", "Fighting withdrawal", "Give ground and preserve force"]
  ];
  const airMissions = [["none", "No air mission"], ["reconnaissance", "Reconnaissance"],
    ["interception", "Interception"], ["ground_support", "Ground support"], ["interdiction", "Interdiction"]];
  const navalMissions = [["none", "No naval mission"], ["transport", "Transport"],
    ["escort", "Escort"], ["sea_denial", "Sea denial"]];
  const esc = value => String(value ?? "").replace(/[&<>"']/g,
    char => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[char]));
  const number = (value, digits = 2) => Number.isFinite(value) ? value.toFixed(digits) : "Unknown";
  const percent = value => Number.isFinite(value) ? `${number(value * 100, 0)}%` : "Unknown";
  const band = (low, high, scale = 1) => Number.isFinite(low) && Number.isFinite(high)
    ? `${number(low * scale)}–${number(high * scale)}` : "Not yet assessed";
  const options = (rows, chosen) => rows.map(([value, label]) =>
    `<option value="${esc(value)}"${chosen === value ? " selected" : ""}>${esc(label)}</option>`).join("");
  const metric = (label, value, note = "") => `<div class="campaign-metric"><dt>${esc(label)}</dt>
    <dd>${esc(value)}</dd>${note ? `<small>${esc(note)}</small>` : ""}</div>`;
  const dateLabel = (label, day) => label || (Number.isFinite(day) ? `Simulation day ${day}` : "Date unavailable");

  function html(data, peace = null) {
    const view = data?.operation || data;
    if (!view?.enabled) return "";
    const order = view.order || {}, supply = view.supply || {}, forecast = view.forecast || {};
    const enemy = view.enemy || {}, report = view.last_report;
    const conflict = view.conflict, name = data?.theatre_name || view.theatre_name;
    const targets = (view.targets || []).filter(row => typeof row.id === "string");
    const targetRows = targets.map(row => [row.id, `${row.name || row.id}${row.held ? " · held" : ""}`]);
    if (order.target && !targetRows.some(row => row[0] === order.target)) {
      // A saved target can leave the legal target list after a border changes.
      // Show that fact without silently substituting a different player order.
      targetRows.push([order.target, `${order.target} · review target`]);
    }
    const hasObservation = Number.isFinite(enemy.low) && Number.isFinite(enemy.high);
    const period = forecast.period || "per day";
    return `<section class="campaign-command" aria-label="Campaign operations${name ? ` in ${esc(name)}` : ""}">
      <header class="campaign-heading"><div><span class="campaign-kicker">Theatre command</span>
        <h3>${esc(name || "Plan the operation")}</h3></div>
        <span class="campaign-tag">${esc(supply.status || "Awaiting supply report")}</span></header>
      <dl class="campaign-metrics">
        ${metric("Readiness", percent(view.readiness), "Cohesion and fatigue")}
        ${metric("Supply coverage", percent(supply.coverage), supply.eta_days == null ? "Delivery time unknown" : `Next delivery: ${supply.eta_days} day${supply.eta_days === 1 ? "" : "s"}`)}
        ${metric("Fielded force", number(view.fielded), "Force points")}
        ${metric("National reserve", number(view.reserve), "Shared across theatres")}
      </dl>
      <p class="campaign-caption campaign-force-detail">This theatre: ${esc(number(view.in_transit))} in transit · ${esc(number(view.garrison))} on garrison duty.</p>
      <div class="campaign-brief"><span class="campaign-kicker">Staff assessment</span>
        <p>${esc(forecast.reason || "The staff has not assessed this operation yet.")}</p>
        ${supply.reason ? `<p class="campaign-secondary">${esc(supply.reason)}</p>` : ""}
        <dl class="campaign-estimates">
          <div><dt>Enemy force estimate</dt><dd>${esc(band(enemy.low, enemy.high))}</dd>
            <small>${hasObservation ? `${esc(typeof enemy.confidence === "number" ? percent(enemy.confidence) : enemy.confidence || "Unknown")} confidence · ${esc(dateLabel(enemy.observed_label, enemy.observed_day))}` : "No current observation"}</small></div>
          <div><dt>Local control change</dt><dd>${esc(band(forecast.advance_low, forecast.advance_high, 100))}</dd><small>Control points ${esc(period)}</small></div>
          <div><dt>Expected losses</dt><dd>${esc(band(forecast.loss_low, forecast.loss_high))}</dd><small>Force points ${esc(period)}</small></div>
        </dl>
        <p class="campaign-caption">Assessment of the standing order · preparation ${esc(percent(view.preparation))}. Changing the draft below does not recalculate this report.</p>
      </div>
      <form class="campaign-order" data-campaign-command="operation" data-conflict="${esc(conflict)}">
        <fieldset><legend>Choose an approach</legend><div class="campaign-approaches">
          ${approaches.map(([value, label, detail]) => `<label class="campaign-approach"><input type="radio" name="approach" value="${value}"${(order.approach || "hold") === value ? " checked" : ""}><span><strong>${label}</strong><small>${detail}</small></span></label>`).join("")}
        </div></fieldset>
        <div class="campaign-order-fields"><label>Objective district<select name="target" aria-label="Objective district"><option value=""${order.target == null ? " selected" : ""}>Staff selects the front</option>${options(targetRows, order.target)}</select></label>
          <label>Keep in reserve (%)<input type="number" name="reserve_percent" aria-label="Keep in reserve (%)" min="0" max="100" step="0.01" value="${Number.isInteger(order.reserve_bp) ? order.reserve_bp / 100 : 20}" required></label>
          <label>Air mission<select name="air" aria-label="Air mission">${options(airMissions, order.air || "none")}</select></label>
          <label>Naval mission<select name="naval" aria-label="Naval mission">${options(navalMissions, order.naval || "none")}</select></label></div>
        ${(view.mission_constraints || []).length ? `<ul class="campaign-mission-constraints" aria-label="Support mission constraints">${view.mission_constraints.map(reason => `<li>${esc(reason)}</li>`).join("")}</ul>` : ""}
        <p class="campaign-caption">Staff assigns the forces. Your national allocation, equipment, access and commitment still limit what can deploy.</p>
        <div class="campaign-actions"><button type="submit" class="campaign-primary">Apply operation · free</button><span class="campaign-status" role="status" aria-live="polite"></span></div>
      </form>
      ${report ? `<details class="campaign-receipt" open><summary>Last field report · ${esc(dateLabel(report.day_label, report.day))}</summary><p>${esc(report.summary)}</p><p class="campaign-caption">Losses: ${esc(number(report.losses))} force points · readiness after action: ${esc(percent(report.readiness))}</p></details>` : `<p class="campaign-caption campaign-no-report">No field report yet. Advance the day to resolve the standing order.</p>`}
      ${peaceHtml(peace || data?.peace, conflict)}
    </section>`;
  }

  function peaceHtml(data, conflict) {
    if (!data) return "";
    const aim = data.aim || {}, targets = (data.targets || []).map(row => [row.id, row.name || row.id]);
    const aimRows = (data.aim_options || []).map(row => [row.kind, row.label]);
    const selectedAim = aimRows.find(row => row[0] === aim.kind);
    const aimDescription = (data.aim_options || []).find(row => row.kind === aim.kind)?.description;
    const price = key => Number.isFinite(data.prices?.[key]) ? data.prices[key] === 0 ? "free" : `${number(data.prices[key], 0)} PC` : "cost unavailable";
    const status = '<span class="campaign-status" role="status" aria-live="polite"></span>';
    const garrison = data.garrison;
    const limits = data.limits || {}, minReparations = limits.reparations_min_bp, maxReparations = limits.reparations_max_bp;
    const hasReparationQuote = Number.isInteger(minReparations) && Number.isInteger(maxReparations) && minReparations > 0 && maxReparations >= minReparations;
    const maxGarrison = limits.garrison_max_bp;
    return `<section class="campaign-peace" aria-label="War aims and peace"><h4>Aim and exit</h4>
      <p>${selectedAim ? `Current aim: <strong>${esc(selectedAim[1])}</strong>. ` : ""}${esc(aimDescription || "Choose the result your government is seeking.")}</p>
      ${aimRows.length ? `<form data-campaign-command="set_aim" data-conflict="${esc(conflict)}">
        <div class="campaign-peace-fields"><label>War aim<select name="aim" aria-label="War aim">${options(aimRows, aim.kind)}</select></label>
          <label data-campaign-recover${aim.kind === "recover" ? "" : " hidden"}>Territory to recover<select name="aim_district" aria-label="Territory to recover"><option value="">Choose a district</option>${options(targets, aim.districts?.[0])}</select></label></div>
        <p class="campaign-caption">A government change seeks a political transition; it does not appoint a chosen leader.</p>
        <div class="campaign-actions"><button type="submit"${Number.isFinite(data.prices?.aim) ? "" : " disabled"}>Set war aim · ${esc(price("aim"))}</button>${status}</div>
      </form>` : ""}
      ${(data.proposals || []).map(offer => `<article class="campaign-peace-offer"><span class="campaign-kicker">${offer.incoming ? "Peace offer received" : "Proposal awaiting responses"}</span>
        <p><strong>${esc(offer.from_name || "Coalition proposal")}</strong> · ${esc(offer.summary || "Terms awaiting a report")}</p>
        <p class="campaign-caption">${esc(offer.expires_label ? `Response due ${offer.expires_label}` : Number.isFinite(offer.days_left) ? `Response due in ${offer.days_left} day${offer.days_left === 1 ? "" : "s"}` : Number.isFinite(offer.expires_day) ? `Response due on simulation day ${offer.expires_day}` : "Response deadline unavailable")}</p>
        ${offer.can_respond ? `<form data-campaign-command="respond" data-offer="${esc(offer.id)}"><div class="campaign-actions"><button type="submit" name="accept" value="false">Reject offer</button><button type="submit" name="accept" value="true" class="campaign-primary">Accept terms · ${esc(price("response"))}</button>${status}</div></form>` : ""}
      </article>`).join("")}
      ${data.can_propose ? `<details class="campaign-peace-compose"><summary>Prepare a peace proposal</summary><form data-campaign-command="propose" data-conflict="${esc(conflict)}" data-reparations-min="${esc(minReparations)}" data-reparations-max="${esc(maxReparations)}">
        <div class="campaign-peace-fields"><label>Proposed terms<select name="terms" aria-label="Proposed peace terms">${options([["ceasefire", "Ceasefire · restore legal borders"], ["cede", "Territorial concession"], ["reparations", "Reparations"], ["transition", "Political opening"]], "ceasefire")}</select></label>
          <label data-campaign-cede hidden>Demanded district<select name="cede_district" aria-label="Demanded district"><option value="">Choose a district</option>${options(targets, null)}</select></label>
          <label data-campaign-reparations hidden>Reparations (% of payer's annual GDP)<input type="number" name="reparation_percent" aria-label="Reparations (% of payer's annual GDP)" min="${hasReparationQuote ? minReparations / 100 : 0}" max="${hasReparationQuote ? maxReparations / 100 : 0}" step="0.01" value="${hasReparationQuote ? Math.min(maxReparations, Math.max(minReparations, 100)) / 100 : ""}"${hasReparationQuote ? "" : " disabled"}></label></div>
        <p class="campaign-caption">Demands apply to the opposing principal. Every coalition member must consent. A ceasefire restores legal borders.</p>
        <div class="campaign-actions"><button type="submit"${Number.isFinite(data.prices?.proposal) ? "" : " disabled"}>Propose terms · ${esc(price("proposal"))}</button>${status}</div>
      </form></details>` : data.reason ? `<p class="campaign-caption">${esc(data.reason)}</p>` : ""}
      ${garrison ? `<details class="campaign-garrison"><summary>Occupation and garrison policy</summary>
        <form data-campaign-command="garrison" data-conflict="${esc(conflict)}" data-garrison-max="${esc(maxGarrison)}"><div class="campaign-peace-fields">
          <label>Theatre force for garrisons (%)<input type="number" name="garrison_percent" aria-label="Theatre force for garrisons (%)" min="0" max="${Number.isInteger(maxGarrison) ? maxGarrison / 100 : 0}" step="0.01" value="${Number.isInteger(garrison.share_bp) ? garrison.share_bp / 100 : 0}" required></label>
          <label>Occupation policy<select name="policy" aria-label="Occupation policy">${options([["restraint", "Restraint"], ["security", "Security"], ["reconstruction", "Reconstruction"]], garrison.policy || "restraint")}</select></label></div>
          <div class="campaign-actions"><button type="submit">Apply garrison policy</button>${status}</div></form>
        ${(data.occupation || []).length ? `<div class="campaign-occupation-list">${data.occupation.map(row => `<article><strong>${esc(row.name || row.district)}</strong><dl>${metric("Resistance", percent(row.resistance))}${metric("Garrison coverage", percent(row.coverage))}${metric("Days occupied", number(row.days, 0))}</dl></article>`).join("")}</div>` : '<p class="campaign-caption">No occupied districts to administer.</p>'}
        <p class="campaign-caption">Occupation controls the ground. Sovereignty changes only through settlement.</p>
      </details>` : ""}
    </section>`;
  }

  const validEnum = (value, rows) => rows.some(row => row[0] === value);
  let sending = false;
  function share(value, ceiling, label) {
    // Decimal text is parsed as basis points without silently rounding an order.
    if (!/^(?:\d{1,3}(?:\.\d{0,2})?|\.\d{1,2})$/.test(value || "")) throw Error(`${label} must be between 0 and ${ceiling}%, with at most two decimal places.`);
    const [whole, fraction = ""] = value.split(".");
    const bp = Number(whole) * 100 + Number(fraction.padEnd(2, "0"));
    if (bp > ceiling * 100) throw Error(`${label} must be between 0 and ${ceiling}%.`);
    return bp;
  }
  function commandFor(form, event) {
    const value = name => form.querySelector(`[name="${name}"]`)?.value;
    if (form.dataset.campaignCommand === "respond") {
      const offer = Number(form.dataset.offer), answer = event.submitter?.value;
      if (!/^\d+$/.test(form.dataset.offer || "") || !Number.isSafeInteger(offer) || offer < 0 || !["true", "false"].includes(answer)) throw Error("Choose Accept terms or Reject offer.");
      return { kind: "war_diplomacy", order: { kind: "respond", offer, accept: answer === "true" } };
    }
    const conflict = Number(form.dataset.conflict);
    if (!/^\d+$/.test(form.dataset.conflict || "") || !Number.isInteger(conflict) || conflict < 0 || conflict > 4294967295) throw Error("This conflict is unavailable. Refresh the campaign.");
    if (form.dataset.campaignCommand === "set_aim") {
      const kind = value("aim"), district = value("aim_district");
      if (!["expel", "recover", "concession", "government_change"].includes(kind)) throw Error("Choose a valid war aim.");
      if (kind === "recover" && !district) throw Error("Choose the territory to recover.");
      const aim = kind === "recover" ? { kind, districts: [district] } : { kind };
      return { kind: "war_diplomacy", order: { kind: "set_aim", conflict, aim } };
    }
    if (form.dataset.campaignCommand === "propose") {
      const kind = value("terms"), district = value("cede_district");
      if (!["ceasefire", "cede", "reparations", "transition"].includes(kind)) throw Error("Choose valid peace terms.");
      if (kind === "cede" && !district) throw Error("Choose the district to demand.");
      let terms = kind === "cede" ? { kind, districts: [district] } : { kind };
      if (kind === "reparations") {
        const min = Number(form.dataset.reparationsMin), max = Number(form.dataset.reparationsMax);
        if (!Number.isInteger(min) || !Number.isInteger(max) || min <= 0 || max < min) throw Error("Refresh the campaign to get current reparations limits.");
        const bp = share(value("reparation_percent"), max / 100, "Reparations");
        if (bp < min) throw Error(`Reparations must be at least ${min / 100}% of annual GDP.`);
        terms = { kind, share_bp: bp };
      }
      return { kind: "war_diplomacy", order: { kind: "propose", conflict, terms } };
    }
    if (form.dataset.campaignCommand === "garrison") {
      const policy = value("policy");
      if (!["restraint", "security", "reconstruction"].includes(policy)) throw Error("Choose a valid occupation policy.");
      const max = Number(form.dataset.garrisonMax);
      if (!Number.isInteger(max) || max < 0 || !form.dataset.garrisonMax) throw Error("Refresh the campaign to get the current garrison limit.");
      return { kind: "war_diplomacy", order: { kind: "garrison", conflict,
        share_bp: share(value("garrison_percent"), max / 100, "Garrison force"), policy } };
    }
    if (form.dataset.campaignCommand !== "operation") throw Error("This action is unavailable. Refresh the campaign.");
    const approach = form.querySelector('[name="approach"]:checked')?.value;
    const air = value("air"), naval = value("naval"), reserve = value("reserve_percent");
    if (!validEnum(approach, approaches) || !validEnum(air, airMissions) || !validEnum(naval, navalMissions)) {
      throw Error("Choose a valid approach and support missions.");
    }
    const reserve_bp = share(reserve, 100, "Reserve");
    const target = value("target");
    if (target === undefined) throw Error("Choose an objective district or let the staff select it.");
    return { kind: "operation", conflict, target: target || null, approach, reserve_bp, air, naval };
  }

  function bind(container, send, onEdit) {
    if (!container || typeof send !== "function") return;
    container.querySelectorAll("form[data-campaign-command]").forEach(form => {
      // The host can pause its live clock before the sheet refreshes a draft.
      // No browser-global clock or cross-campaign draft cache lives here.
      if (typeof onEdit === "function") form.onfocusin = () => onEdit();
      const aim = form.querySelector('[name="aim"]'), terms = form.querySelector('[name="terms"]');
      const reveal = (selector, visible) => { const row = form.querySelector(selector); if (row) row.hidden = !visible; };
      if (aim) aim.onchange = () => reveal("[data-campaign-recover]", aim.value === "recover");
      if (terms) terms.onchange = () => {
        reveal("[data-campaign-cede]", terms.value === "cede");
        reveal("[data-campaign-reparations]", terms.value === "reparations");
      };
      form.onsubmit = async event => {
        event.preventDefault();
        const button = event.submitter || form.querySelector('button[type="submit"]'), status = form.querySelector('[role="status"]');
        if (!button || !status) return;
        if (sending || button.disabled) { status.textContent = "Wait for the current order to finish."; return; }
        let command;
        try {
          command = commandFor(form, event);
        } catch (error) { status.textContent = error.message; return; }
        sending = true; button.disabled = true; status.textContent = "Sending order…";
        try {
          const response = await send(command);
          if (response?.errors?.length) throw Error(response.errors.join(" "));
          status.textContent = command.kind === "operation" ? "Operation order applied." : "Decision recorded.";
        } catch (error) { status.textContent = error.message || "Could not confirm the order. Review its receipt before trying again."; }
        finally { sending = false; button.disabled = false; }
      };
    });
  }
  return { html, peaceHtml, bind };
});
