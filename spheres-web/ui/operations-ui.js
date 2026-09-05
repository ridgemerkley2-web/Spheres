/* Server-authoritative national force allocation. No combat arithmetic here. */
(function (root) {
  "use strict";
  const escape = value => String(value ?? "").replace(/[&<>"']/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));
  const number = value => Number.isFinite(value) ? value.toFixed(2) : "—";
  function html(data, wars = [], focus = null) {
    if (!data?.enabled) return "";
    const rows = (data.deployments || []).filter(r => focus == null || r.conflict === focus);
    return `<section class="military-operations" aria-label="National force allocation">
      <header><span>National command</span><h3>Your forces and reserves</h3></header>
      <dl class="military-force-totals"><div><dt>National force</dt><dd>${number(data.structure)}</dd></div><div><dt>Deployed</dt><dd>${number(data.deployed)}</dd></div><div><dt>In reserve</dt><dd>${number(data.reserve)}</dd></div></dl>
      <p>Overseas: ${number(data.overseas_deployed)} deployed · ${number(data.overseas_limit)} sustainable. These are force points, not troop counts.</p>
      <p>Allocation limits share one national force. Access, equipment and your commitment rung can reduce what actually deploys.</p>
      ${rows.map(row => {
        const war = wars.find(w => w.id === row.conflict);
        const choices = [...new Set([0,500,1000,1500,2500,5000,7500,10000,row.allocation_bp].filter(Number.isFinite))].sort((a,b) => a-b);
        return `<form class="military-allocation" data-force-id="${escape(row.conflict)}">
          <h4>${escape(war?.theatre_name || `Conflict #${row.conflict}`)}</h4>
          <p>${row.overseas ? "Overseas" : "Home theatre"} · ${number(row.deployed)} deployed · ${number(row.effective_force)} effective · magazine use ${(row.burn_monthly * 100).toFixed(2)}% / month</p>
          <label>Ceiling as a share of national force<select name="share" aria-label="Force ceiling for ${escape(war?.theatre_name || row.conflict)}"><option value=""${row.allocation_bp == null ? " selected" : ""}>Automatic staff allocation</option>${choices.map(bp => `<option value="${bp}"${row.allocation_bp === bp ? " selected" : ""}>${bp / 100}%${bp === 0 ? " · keep in reserve" : ""}</option>`).join("")}</select></label>
          <button type="submit">Apply allocation · free</button><span class="military-allocation-status" role="status" aria-live="polite"></span>
        </form>`;
      }).join("") || "<p>No deployments. Your force remains in reserve.</p>"}
      <details><summary>How equipment supports operations</summary><p>Land support ${number(data.capabilities?.land)} · strike support ${number(data.capabilities?.strike)} · overseas lift ${number(data.capabilities?.lift)}. A balanced arsenal supports more kinds of operations. Combat can destroy fielded equipment, while procurement orders retain their delivery schedule.</p></details>
    </section>`;
  }
  function bind(container, send) {
    if (!container) return;
    container.querySelectorAll("form[data-force-id]").forEach(form => {
      form.onsubmit = async event => {
        event.preventDefault();
        const button = form.querySelector("button"), status = form.querySelector("[role=status]");
        if (button.disabled) return;
        const value = form.querySelector("select").value;
        const share = value === "" ? null : Number(value);
        if (share !== null && (!Number.isInteger(share) || share < 0 || share > 10000)) {
          status.textContent = "Choose a valid force ceiling."; return;
        }
        button.disabled = true; status.textContent = "Applying allocation…";
        try {
          await send({ kind: "force_allocation", conflict: Number(form.dataset.forceId), share_bp: share });
          status.textContent = "Allocation applied.";
        } catch (error) { status.textContent = error.message || "Could not apply allocation. Try again."; }
        finally { button.disabled = false; }
      };
    });
  }
  root.MilitaryOperationsUI = { html, bind };
})(typeof window === "undefined" ? globalThis : window);
