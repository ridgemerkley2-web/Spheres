/* Server-authoritative national force allocation. No combat arithmetic here. */
(function (root) {
  "use strict";
  const escape = value => String(value ?? "").replace(/[&<>"']/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));
  const number = value => Number.isFinite(value) ? value.toFixed(2) : "—";
  const count = value => Number.isFinite(value) ? value.toLocaleString("en-US", {maximumFractionDigits: 3}) : "—";
  const percent = value => Number.isFinite(value) ? `${(value * 100).toFixed(1)}%` : "Not assessed";
  function groundRoles(row) {
    // Standoff strikes do not carry the ground force's observation or movement effects.
    if (row.rung === 6) return "";
    const roles = row.capabilities?.ground_roles;
    if (!roles) return "";
    const ammo = row.ammunition;
    const rows = [
      ["Find targets", roles.reconnaissance, "Reconnaissance helps the force find and engage targets. It does not reveal exact enemy strength."],
      ["Protected movement", roles.protected_mobility, "IFVs and APCs support ground advance. Terrain, preparation and the standing order still matter."],
      ["Fire support", ammo ? ammo.fire_support : roles.fire_support, ammo ? "Support from currently supplied weapons. This is already included in the operation's firing effect." : "Serviceable support under the shared magazine system."],
      ["Air defense", ammo ? ammo.air_defense : roles.air_defense, ammo ? "Protection against exposed air attacks with compatible interceptors. No exposure or no usable stores can give zero protection." : "Local protection from serviceable air-defense equipment under the shared magazine system."]
    ];
    return `<details class="military-ground-roles"><summary>How your ground equipment helps here</summary>
      <p>These are the simulation's current support effects for this commitment, before local combat conditions. They are not extra force points or guaranteed battle results.</p>
      <dl class="military-role-grid">${rows.map(([label, value, detail]) => `<div><dt>${label}</dt><dd>${percent(value)}</dd><small>${detail}</small></div>`).join("")}</dl>
      ${ammo ? `<p class="military-stores-note">${ammo.physical_dry ? "Custom offensive weapons have no usable firing supply. Their remaining movement and observation do not create free shots." : "Compatible physical stores are shared across all current operations."} Firing support: ${percent(ammo.fire_fraction)}. Movement support: ${percent(ammo.maneuver_fraction)}.</p>` : '<p class="military-stores-note">Custom ground weapons currently share the national magazine. Review Ammunition to prepare compatible stores and activate physical supply.</p>'}
    </details>`;
  }
  function groundFleet(data) {
    if (!data || typeof data !== "object") return "";
    const models = Array.isArray(data.models) ? data.models : [];
    const actions = (Array.isArray(data.actions) ? data.actions : []).filter(action => ["service", "ammunition", "companies"].includes(action?.tab));
    return `<section class="military-ground-fleet" aria-label="Ground equipment and support"><div class="military-fleet-heading"><span>Equipment desk</span><h4>${escape(data.title || "Your ground fleet")}</h4></div>
      ${data.detail ? `<p>${escape(data.detail)}</p>` : ""}
      ${data.maintenance ? `<p class="military-maintenance">${data.maintenance.recorded_day == null ? "Maintenance has not settled yet." : `Last settled fleet support: ${percent(data.maintenance.coverage)}.${data.maintenance.day_label ? ` Support date: ${escape(data.maintenance.day_label)}.` : ""}`}</p>` : ""}
      ${models.length ? `<details class="military-fleet-models"><summary>${models.length} ground model${models.length === 1 ? "" : "s"} in national holdings</summary><div class="military-model-grid">${models.map(model => `<article class="military-model" data-ground-revision="${escape(model.revision)}"><span>${escape(model.role || model.platform || "Ground equipment")}</span><h5>${escape(model.name || model.revision)}</h5><dl><div><dt>Owned</dt><dd>${count(model.delivered)}</dd></div><div><dt>Available</dt><dd>${count(model.available)}</dd></div><div><dt>In refit</dt><dd>${count(model.reserved)}</dd></div></dl><p>${model.ammunition_family ? `Ammunition: ${escape(model.ammunition_name || model.ammunition_family)}. ` : ""}Condition and upkeep support: ${percent(model.supported_fraction)}. This is a capability factor, not a count of ready vehicles.</p></article>`).join("")}</div><p>Available holdings support the national force. They are shared across commitments; each theatre does not receive another copy of these vehicles.</p></details>` : '<p class="military-fleet-empty">No custom ground models in service. Review company stock to equip the force.</p>'}
      ${groundLossReport(data.last_report)}
      ${actions.length ? `<div class="military-equipment-actions">${actions.map(action => `<button type="button" data-ground-equipment-tab="${escape(action.tab)}">${escape(action.label || "Review equipment")}</button>`).join("")}<span class="military-navigation-status" role="status" aria-live="polite"></span></div>` : ""}
    </section>`;
  }
  function groundLossReport(report) {
    if (!report || !Array.isArray(report.revisions)) return '<p class="military-loss-empty">No ground-equipment loss settlement has been recorded yet.</p>';
    const conflicts = (Array.isArray(report.conflicts) ? report.conflicts : []).map(id => `#${escape(id)}`).join(", ");
    return `<details class="military-ground-receipt" open><summary>Last national equipment report · ${escape(report.day_label || "Date unavailable")}</summary>
      <p>One national settlement${conflicts ? ` across conflicts ${conflicts}` : ""}. These vehicle losses are shared across the listed conflicts, not the losses of this theatre alone. Force-point casualties are reported separately.</p>
      <div class="military-loss-grid">${report.revisions.map(model => `<article class="military-loss-model"><h5>${escape(model.name || model.revision_id)}</h5><span>${escape(model.role || model.platform || "Ground equipment")}</span><dl><div><dt>Vehicles lost</dt><dd>${count(model.lost)}</dd></div><div><dt>Available after combat</dt><dd>${count(model.remaining_available)}</dd></div><div><dt>Protected in refit</dt><dd>${count(model.remaining_reserved)}</dd></div></dl><p>Owned at settlement: ${count(model.opening_delivered)} → ${count(model.remaining_delivered)}.</p></article>`).join("")}</div>
      <p>This dated record stays unchanged after later deliveries, refits or retirement. Current holdings are shown above.</p>
    </details>`;
  }
  function html(data, wars = [], focus = null) {
    if (!data?.enabled) return "";
    const rows = (data.deployments || []).filter(r => focus == null || r.conflict === focus);
    return `<section class="military-operations" aria-label="National force allocation">
      <header><span>National command</span><h3>Your forces and reserves</h3></header>
      ${focus == null ? (root.AreaArt?.html("military", "banner") || "") : ""}
      <dl class="military-force-totals"><div><dt>National force</dt><dd>${number(data.structure)}</dd></div><div><dt>Committed</dt><dd>${number(data.deployed)}</dd></div><div><dt>Uncommitted</dt><dd>${number(data.reserve)}</dd></div></dl>
      <p>Overseas: ${number(data.overseas_deployed)} committed · ${number(data.overseas_limit)} sustainable. These are force points, not troop counts.</p>
      <p>Commitments share one national force. Access, equipment and your commitment rung limit allocations. Forces must still travel; theatre command shows those that have arrived.</p>
      ${rows.map(row => {
        const war = wars.find(w => w.id === row.conflict);
        const choices = [...new Set([0,500,1000,1500,2500,5000,7500,10000,row.allocation_bp].filter(Number.isFinite))].sort((a,b) => a-b);
        return `<form class="military-allocation" data-force-id="${escape(row.conflict)}">
          <h4>${escape(war?.theatre_name || `Conflict #${row.conflict}`)}</h4>
          <p>${row.overseas ? "Overseas" : "Home theatre"} · ${number(row.deployed)} committed · ${number(row.effective_force)} supported commitment · magazine use ${Number.isFinite(row.burn_monthly) ? (row.burn_monthly * 100).toFixed(2) : "—"}% / month</p>
          <label>Ceiling as a share of national force<select name="share" aria-label="Force ceiling for ${escape(war?.theatre_name || row.conflict)}"><option value=""${row.allocation_bp == null ? " selected" : ""}>Automatic staff allocation</option>${choices.map(bp => `<option value="${bp}"${row.allocation_bp === bp ? " selected" : ""}>${bp / 100}%${bp === 0 ? " · keep in reserve" : ""}</option>`).join("")}</select></label>
          <button type="submit">Apply allocation · free</button><span class="military-allocation-status" role="status" aria-live="polite"></span>
          ${groundRoles(row)}
        </form>`;
      }).join("") || "<p>No deployments. Your force remains in reserve.</p>"}
      <details><summary>How equipment supports operations</summary><p>Land support ${number(data.capabilities?.land)} · strike support ${number(data.capabilities?.strike)} · overseas lift ${number(data.capabilities?.lift)}. A balanced arsenal supports more kinds of operations. Combat can destroy fielded equipment, while procurement orders retain their delivery schedule.</p></details>
      ${groundFleet(data.ground_fleet)}
    </section>`;
  }
  let sending = false;
  function bind(container, send, navigate) {
    if (!container) return;
    if (typeof navigate === "function") container.querySelectorAll("[data-ground-equipment-tab]").forEach(button => {
      button.onclick = async () => {
        const tab = button.dataset.groundEquipmentTab, status = container.querySelector(".military-navigation-status");
        if (sending || button.disabled || !["service", "ammunition", "companies"].includes(tab)) return;
        button.disabled = true;
        try { await navigate(tab); }
        catch (error) { if (status) status.textContent = error.message || "Reopen Operations to review this equipment."; }
        finally { button.disabled = false; }
      };
    });
    container.querySelectorAll("form[data-force-id]").forEach(form => {
      form.onsubmit = async event => {
        event.preventDefault();
        const button = form.querySelector("button"), status = form.querySelector("[role=status]");
        if (sending || button.disabled) { status.textContent = "Wait for the current order to finish."; return; }
        const value = form.querySelector("select").value;
        const share = value === "" ? null : Number(value);
        if (share !== null && (!Number.isInteger(share) || share < 0 || share > 10000)) {
          status.textContent = "Choose a valid force ceiling."; return;
        }
        sending = true; button.disabled = true; status.textContent = "Applying allocation…";
        try {
          const response = await send({ kind: "force_allocation", conflict: Number(form.dataset.forceId), share_bp: share });
          if (response?.command_pending) throw Error("This order awaits confirmation. Review the pending order before submitting again.");
          if (response?.errors?.length) throw Error(response.errors.join(" "));
          status.textContent = "Allocation applied.";
        } catch (error) { status.textContent = error.message || "Could not apply allocation. Try again."; }
        finally { sending = false; button.disabled = false; }
      };
    });
  }
  root.MilitaryOperationsUI = { html, bind };
})(typeof window === "undefined" ? globalThis : window);
