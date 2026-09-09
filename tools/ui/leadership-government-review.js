(function (root, factory) {
  const api = factory();
  if (typeof module !== "undefined" && module.exports) module.exports = api;
  if (root.document) api.mount(root.document, root.GovernmentUI);
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";
  const NAMES = {USA:"United States",UK:"United Kingdom",SouthAfrica:"South Africa"};
  const MODES = {
    campaign:{title:"Sample starting campaign · 1 January 1990",note:"These are bindings created by a fresh in-memory game with historical party leadership enabled. They are a review sample, not your saved campaign. Uncertain handovers and missing artwork remain visible."},
    history:{title:"Historical reference · 1 January 1990",note:"Only sourced references for this date are shown. Partial date bounds stay uncertain. This view contains no campaign appointments or fictional successors."},
    future:{title:"Fictional cast reference · 1 January 2030",note:"Every successor here is invented. These are conditional character templates, not a future government or predicted election result. A surviving campaign party, an actual vacancy and the disclosed rules are required before anyone can take office."}
  };
  const copy = value => JSON.parse(JSON.stringify(value));
  function presentation(nation, mode, query="") {
    if (!MODES[mode]) throw new Error("Unknown static snapshot");
    const board = copy(mode === "future" ? nation.future_reference_2030 : nation.reference_1990);
    if (mode !== "campaign") {
      board.enabled = false;
      board.executive_person = null;
      for (const p of board.parties) {
        p.campaign = [];
        if (mode === "history") { p.future_candidates = []; p.future_preview = []; }
      }
    }
    const data = {nation:nation.nation,nation_name:NAMES[nation.nation] || nation.nation,mine:false,
      on:true,actions:[],party_leadership:board,briefing:{date_label:MODES[mode].title,summary:MODES[mode].note}};
    const state = {tab:"leadership",leadershipMode:mode === "history" ? "reference" : "campaign",leadershipQuery:query};
    if (mode === "history") Object.assign(state,{leadershipDate:"1990-01-01",leadershipReference:{data:board}});
    return {data,state};
  }
  // The production renderer has already applied its /art/ allowlist. This host
  // only maps two explicit repository asset routes; arbitrary URLs never pass.
  function assetPath(url) {
    if (url === "/art/government/council-v1.png") return "../../spheres-web/ui/government-art/council-v1.png";
    const match = /^\/art\/people\/([A-Za-z0-9_-]+\.png)$/.exec(url || "");
    return match ? "../../spheres-web/ui/person-portraits/" + match[1] : null;
  }
  async function mount(document, renderer) {
    const el = id => document.getElementById(id);
    let fixture, mode="campaign", selected="Japan", details=new Set(), autoOpenFuture=true;
    const params = new URLSearchParams(location.search);
    if (MODES[params.get("view")]) mode=params.get("view");
    if (params.get("nation")) selected=params.get("nation");
    function render() {
      const nation = fixture.nations.find(n => n.nation === selected) || fixture.nations[0];
      selected=nation.nation; el("nation").value=selected; el("snapshot").value=mode;
      const {data,state}=presentation(nation,mode,el("search").value);
      el("govBody").innerHTML=renderer.render(data,state);
      // This page owns only leadership review. Hide unavailable game navigation
      // and replace live-context copy with the explicit static snapshot label.
      const screen=el("govBody"), hero=screen.querySelector(".gov-ui-hero .gov-ui-kicker");
      if (hero) hero.textContent="Static read-only review · "+MODES[mode].title;
      screen.querySelectorAll(".gov-ui-message").forEach(n=>n.remove());
      screen.querySelectorAll(".gov-ui-tabs,.gov-ui-hero-actions,.gov-ui-leadership-tools,.gov-ui-footer").forEach(n=>n.remove());
      if (mode === "future") {
        screen.querySelectorAll(".gov-ui-party-leaders,[data-gov-detail^='leader-candidates:']").forEach(n=>n.remove());
      }
      screen.querySelectorAll("img").forEach(img=>{
        const path=assetPath(img.getAttribute("src"));
        if (path) img.setAttribute("src",path);
        else img.removeAttribute("src");
      });
      screen.querySelectorAll("details[data-gov-detail]").forEach(node=>{
        const key=node.dataset.govDetail;
        node.open=details.has(key) || autoOpenFuture && mode === "future" && key.startsWith("leader-future:");
        if (node.open) details.add(key);
      });
      autoOpenFuture=false;
      screen.querySelectorAll("button").forEach(button=>{button.disabled=true;});
      el("snapshot-title").textContent=MODES[mode].title;
      el("snapshot-note").textContent=MODES[mode].note;
      const count=screen.querySelectorAll("[data-gov-leadership-party]").length;
      el("load-status").textContent=`${data.nation_name} · ${count} of ${data.party_leadership.parties.length} party rows · No live campaign connection`;
      const url=new URL(location.href);url.searchParams.set("nation",selected);url.searchParams.set("view",mode);history.replaceState(null,"",url);
    }
    try {
      const response=await fetch("leadership-government-review.json",{cache:"no-store"});
      if (!response.ok) throw new Error(`Static fixture could not be read (${response.status}).`);
      fixture=await response.json();
      if (fixture.read_only !== true || fixture.presentation?.sample_campaign_only !== true || !fixture.nations?.length) throw new Error("The file is not a validated static review fixture.");
      for (const nation of fixture.nations) {
        const option=document.createElement("option");option.value=nation.nation;option.textContent=NAMES[nation.nation] || nation.nation;el("nation").append(option);
      }
      el("nation").firstElementChild.remove();
      el("provenance").textContent=JSON.stringify({read_only:fixture.read_only,campaign_date:fixture.campaign_date,historical_reference_through:fixture.historical_reference_through,presentation:fixture.presentation,source_hashes:fixture.source_hashes},null,2);
      ["nation","snapshot","search","expand","collapse"].forEach(id=>el(id).disabled=false);
      el("review-controls").addEventListener("submit",event=>event.preventDefault());
      el("nation").addEventListener("change",()=>{selected=el("nation").value;details.clear();autoOpenFuture=true;render();});
      el("snapshot").addEventListener("change",()=>{mode=el("snapshot").value;details.clear();autoOpenFuture=true;render();});
      el("search").addEventListener("input",render);
      el("govBody").addEventListener("toggle",event=>{const key=event.target.dataset?.govDetail;if(key) event.target.open ? details.add(key) : details.delete(key);},true);
      el("expand").addEventListener("click",()=>el("govBody").querySelectorAll("details").forEach(n=>n.open=true));
      el("collapse").addEventListener("click",()=>el("govBody").querySelectorAll("details").forEach(n=>n.open=false));
      render();
    } catch (error) {
      el("load-status").textContent="Static review unavailable";
      const notice=document.createElement("p");notice.className="review-error";notice.textContent=error.message;el("govBody").replaceChildren(notice);
      el("snapshot-note").textContent="Regenerate the read-only export and review fixture. No game server or campaign is required.";
    }
  }
  return {presentation,assetPath,mount};
});
