/* Small, importable presentation helpers. No simulation rules or commands. */
(function(root,factory){const api=factory();if(typeof module==="object"&&module.exports)module.exports=api;else root.DecisionTools=api;})(typeof globalThis!=="undefined"?globalThis:this,function(){
  function overlap(a,b,pad=3){return Math.abs(a[0]-b[0])<(a[2]+b[2])/2+pad&&Math.abs(a[1]-b[1])<(a[3]+b[3])/2+pad;}
  function search(nations,districts,owner,query,home){
    const q=String(query||"").trim().toLocaleLowerCase();
    const rows=nations.filter(n=>n.alive!==false).map(n=>({id:n.id,name:n.name,kind:"nation",owner:n.id}));
    for(const [id,d] of Object.entries(districts)) rows.push({id,name:d.name||id,kind:"province",owner:owner(id)});
    return rows.filter(r=>!q||`${r.name} ${r.id} ${r.owner}`.toLocaleLowerCase().includes(q))
      .sort((a,b)=>(b.name.toLocaleLowerCase()===q)-(a.name.toLocaleLowerCase()===q)||(b.owner===home)-(a.owner===home)||(a.kind==="province")-(b.kind==="province")||a.name.localeCompare(b.name)).slice(0,60);
  }
  function research(nodes,domain,query,filter="now"){
    const q=String(query||"").toLocaleLowerCase();
    return nodes.filter(n=>(domain==="all"||n.domain===domain)&&(!q||`${n.name} ${(n.effects||[]).join(" ")}`.toLocaleLowerCase().includes(q))&&(filter==="all"||filter==="known"?filter==="all"||n.state==="known":n.state==="open"&&(filter!=="now"||n.earliest_available)))
      .sort((a,b)=>(b.focus?1:0)-(a.focus?1:0)||(a.year-b.year)||a.name.localeCompare(b.name));
  }
  function guide(programs,works){
    const budget=works?.construction_budget;
    if(programs?.due)return {step:0,title:"Renew your yearly budget",text:"The financial year needs a renewed budget. Review your ministry funding before planning more construction.",action:"budget",actionLabel:"Review yearly budget"};
    if(budget?.enrolled===false || !programs?.enabled && budget?.enrolled!==true)return {step:0,title:"Open construction funding",text:"Apply a daily construction funding limit to open the funding ledger. Your budget pays only for work delivered; unused money is not charged.",action:"works",actionLabel:"Set construction funding"};
    const queue=[...(works?.queue||[]),...(works?.mine_queue||[])];
    if(budget?.daily_budget_bn===0)return {step:2,title:"Construction funding is paused",text:"The daily funding limit is zero. Projects keep their progress. Choose a positive limit when you want construction to resume.",project:queue[0],action:"works",actionLabel:"Review daily funding"};
    const blocked=queue.find(p=>["blocked","paused","slowed"].includes(p.status));
    if(blocked)return {step:2,title:"Review a project that needs attention",text:blocked.reason||"Check the daily funding limit, available capital funding and this project's prerequisites.",project:blocked,action:"works",actionLabel:"Review this project"};
    if(queue.length && Number.isFinite(budget?.available_bn) && budget.available_bn<=0)return {step:2,title:"Check available construction funding",text:budget.reason||"No construction funding is available in the current reading. Review the daily limit and available capital funding before adding more projects.",project:queue[0],action:"works",actionLabel:"Review construction funding"};
    const suggestion=(works?.suggestions?.items||[]).find(item=>item && typeof item.project_kind==="string" && typeof item.district==="string" && item.can_start!==false && item.eligible!==false);
    if(suggestion)return {step:1,title:`Review ${suggestion.name||"a suggested project"}`,text:suggestion.reason||"Review this project's province and country effects before deciding.",suggestion,action:"suggestion",actionLabel:"Review suggested effects"};
    if(queue.length)return {step:3,title:"Your funded projects are progressing",text:"Compare spending, remaining cost and progress in the construction queue. Advance a few days when you are ready; estimates follow current funding and project lead times.",project:queue[0],action:"works",actionLabel:"View this project"};
    if((works?.completed||[]).length)return {step:4,title:"Inspect your completed industry",text:"Compare completed facilities with their actual operating output. Operating facilities still need their normal inputs and running budget.",action:"economy",actionLabel:"Manage industry in Economy"};
    return {step:1,title:"Choose a useful project",text:works?.suggestions?.note||"Compare a factory, smaller workshop or infrastructure project in Construction. All projects share the daily funding limit.",action:"works",actionLabel:"Explore construction"};
  }
  function progressText(value){if(!Number.isFinite(value))return "—";const fraction=Math.max(0,Math.min(1,value)),percent=Math.round(fraction*1000)/10;return fraction>0&&percent===0?"<0.1%":`${percent}%`;}
  function causes(policy){return ["war","sanctions","embargo","debt_drag","unrest","oil","bubble","demand_output_now"].map(key=>({key,value:Number.isFinite(policy?.[key])?policy[key]*(["war","sanctions","embargo","debt_drag","unrest"].includes(key)?-1:1):null})).filter(r=>Number.isFinite(r.value)&&r.value!==0).sort((a,b)=>Math.abs(b.value)-Math.abs(a.value)).slice(0,4);}
  function stabilityHtml(policy){
    const s=policy?.stability;if(!s||!Number.isFinite(s.monthly_points_before_bounds))return "";
    const esc=v=>String(v??"").replace(/[&<>"']/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));
    const signed=n=>`${n>=0?"+":""}${n.toFixed(4)}`;
    const terms=(s.terms||[]).filter(t=>Number.isFinite(t.monthly_points)&&t.monthly_points!==0).slice().sort((a,b)=>Math.abs(b.monthly_points)-Math.abs(a.monthly_points));
    const labels={budget:"Review ministry budgets and costs",decisions:"Review monetary and diplomatic decisions",world:"Review conflicts and commitments"};
    const actions=[...new Set(terms.map(t=>t.action))].filter(a=>Object.hasOwn(labels,a));
    return `<section aria-label="Current stability contributors"><h3>Current economic stability pressure</h3><p><strong>${signed(s.monthly_points_before_bounds)} stability points per month</strong> at current conditions, before the 0–100 bounds.${s.month_fraction<1&&Number.isFinite(s.step_points_before_bounds)?` Today's daily step is ${signed(s.step_points_before_bounds)} points before bounds.`:""}</p><p>The same economic terms and gradual return toward 60 used by the simulation, shown as monthly equivalents. Conditions change during a turn; political events, war outcomes and other direct changes are separate. This does not attribute the change since your last decision.</p>${terms.length?`<ul>${terms.map(t=>`<li>${esc(t.label)}: ${signed(t.monthly_points)} points/month</li>`).join("")}</ul>`:"<p>No current economic pressure.</p>"}<div class="decision-actions">${actions.map(a=>`<button type="button" data-stability-action="${a}">${labels[a]}</button>`).join("")}</div></section>`;
  }
  return {overlap,search,research,guide,progressText,causes,stabilityHtml};
});

if(typeof window!=="undefined"){
  const DTOOLS={seq:0,view:"guide",researchFilter:"available",returnFocus:null};
  function toolsEsc(v){return String(v??"").replace(/[&<>"']/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));}
  function toolsDialog(title,body){
    if(typeof clockPause==="function")clockPause();
    ++DTOOLS.seq;
    let box=document.getElementById("decisionDialog");
    if(!box){box=document.createElement("dialog");box.id="decisionDialog";box.className="decision-dialog";document.body.append(box);box.addEventListener("keydown",e=>e.stopPropagation());box.addEventListener("close",()=>{if(box.open)return;++DTOOLS.seq;DTOOLS.returnFocus?.focus({preventScroll:true});});}
    box.setAttribute("aria-label",title);
    if(!box.open)DTOOLS.returnFocus=document.activeElement;
    box.innerHTML=`<header><h2>${toolsEsc(title)}</h2><button type="button" data-tools-close aria-label="Close ${toolsEsc(title)}">Close</button></header><div class="decision-body">${body}</div>`;
    box.querySelector("[data-tools-close]").onclick=()=>box.close();
    if(!box.open)box.showModal();
    return box;
  }
  window.homeNation=function(){if(!S?.player)return;closeGameDrawers();showTab("map");const point=anchorOf(S.player);if(point)camTween(point[0],point[1],2.2);};
  window.openBuildInfo=async function(){const box=toolsDialog("About SPHERES","<p>Reading build information…</p>"),seq=DTOOLS.seq;try{const b=await api("/api/build");if(seq!==DTOOLS.seq||!box.open)return;box.querySelector(".decision-body").innerHTML=`<p><strong>SPHERES ${toolsEsc(b.version)}</strong> · ${toolsEsc(b.revision)}</p><p>Branch: ${toolsEsc(b.branch)} · Built ${toolsEsc(new Date(b.built_at_unix_seconds*1000).toISOString())}</p><p>${toolsEsc(b.distribution)}</p><p>Save directory: <code>${toolsEsc(b.save_directory)}</code></p><p>${toolsEsc(b.campaign_format)}</p>${typeof openPerformanceSample==="function"?'<button onclick="openPerformanceSample()">Performance sample</button>':""}<button onclick="openPlaytestFeedback()">Record playtest feedback</button>`;}catch(e){if(seq===DTOOLS.seq&&box.open)box.querySelector(".decision-body").textContent=e.message;}};
  window.openWorldFinder=function(){
    if(!S)return;
    const box=toolsDialog("Find a nation, province or city",`<label for="worldFindInput">Place name, code or owner</label><input id="worldFindInput" type="search" autocomplete="off" placeholder="Kathmandu, Tokyo, California…"><p id="worldFindCount" role="status"></p><div id="worldFindRows" class="decision-list"></div>`);
    const input=box.querySelector("input");
    function update(){
      const cities=window.CityDetail ? CityDetail.search(window.CITIES||[],input.value,12) : [];
      const rows=[...cities.map(city=>({...city,kind:'city',owner:city.capital?'Capital':'Explore terrain'})),...DecisionTools.search(S.nations,DINDEX,nationOfDistrict,input.value,S.player)];
      box.querySelector("#worldFindCount").textContent=`${rows.length} matches`;
      box.querySelector("#worldFindRows").innerHTML=rows.map(r=>`<button type="button" data-find-kind="${r.kind}" data-find-id="${toolsEsc(r.id)}"><strong>${toolsEsc(r.name)}</strong><span>${r.kind} · ${toolsEsc(r.owner||"Unassigned")}</span></button>`).join("");
      box.querySelectorAll("[data-find-id]").forEach(b=>b.onclick=()=>{
        box.close();showTab("map");
        if(b.dataset.findKind==='city'){const city=cities.find(c=>c.id===b.dataset.findId);if(city)selectMapCity(city,true);}
        else if(b.dataset.findKind==="province")selectProvince(b.dataset.findId,true);
        else{const p=anchorOf(b.dataset.findId);if(p)camTween(p[0],p[1],2.2);openNation(b.dataset.findId);}
      });
    }
    input.oninput=update;input.onkeydown=e=>{if(e.key==="Enter"){e.preventDefault();box.querySelector("[data-find-id]")?.click();}};update();input.focus();
  };
  window.openAdvisor=async function(){
    if(!S?.player)return;DTOOLS.view="guide";const state=S;
    const box=toolsDialog("Your development advisor",`<p role="status">Reading your current budget and projects…</p>`);
    const seq=DTOOLS.seq;
    try{const works=await api("/api/production");if(seq!==DTOOLS.seq||!box.open)return;
      if(S!==state)throw new Error("The campaign changed while advice was loading. Refresh advice to inspect the current state.");
      const g=DecisionTools.guide(state.programs,works),p=g.project,s=g.suggestion,causes=DecisionTools.causes(state.policy);
      const project=p?`<h3>${toolsEsc(p.name)} · ${toolsEsc(p.province?.name||p.province?.id||"Province")}</h3><p>${toolsEsc(DecisionTools.progressText(p.progress))} complete · ${Number.isFinite(p.eta_days)?`about ${Math.ceil(p.eta_days)} days at current funding`:"ETA pending funding"}</p><p>Cost ${constructionMoney(p.finance?.cost_bn)} · spent ${constructionMoney(p.finance?.spent_bn)} · remaining ${constructionMoney(p.finance?.remaining_bn)}</p><p>Planned per day: ${constructionMoney(p.finance?.daily_request_bn)}.</p>`:"";
      const suggestion=s?`<p><strong>${toolsEsc(s.district_name||s.district)}</strong>${s.priority?` · ${toolsEsc(s.priority)}`:""}</p><p><strong>${constructionMoney(s.cost_bn)} total project cost</strong> · ${Number.isFinite(s.minimum_days)?`at least ${Math.ceil(s.minimum_days)} days`:"Lead time available in the effects review"}</p><p>${Number.isFinite(s.eta_days)?`About ${Math.ceil(s.eta_days)} days at current funding.`:"Completion awaits available funding."}</p>${s.caution?`<p>${toolsEsc(s.caution)}</p>`:""}${Array.isArray(s.evidence)&&s.evidence.length?`<details><summary>Supporting evidence</summary><ul>${s.evidence.map(text=>`<li>${toolsEsc(text)}</li>`).join("")}</ul></details>`:""}`:"";
      const stability=DecisionTools.stabilityHtml(state.policy);
      const diagnostics=causes.length||stability?`<details class="advisor-diagnostics"><summary>Economic pressures and stability</summary>${causes.length?`<h3>Largest current growth contributions</h3><p>These are the simulation's current terms, not an attribution of every change since your last decision.</p><ul>${causes.map(c=>`<li>${toolsEsc(c.key.replaceAll("_"," "))}: ${c.value>=0?"+":""}${(c.value*100).toFixed(2)} percentage points / year</li>`).join("")}</ul><button id="advisorCauses">Inspect money and policy</button>`:""}${stability}</details>`:"";
      box.querySelector(".decision-body").innerHTML=`<div class="decision-list"><article><small>Your next step</small><h3>${toolsEsc(g.title)}</h3><p>${toolsEsc(g.text)}</p>${suggestion}${project}<div class="decision-actions"><button id="advisorNext">${toolsEsc(g.actionLabel)}</button></div></article></div><p id="advisorStatus" role="status"></p><div class="decision-actions"><button id="advisorExchange">Explore construction</button><button onclick="openAdvisor()">Refresh advice</button></div>${diagnostics}<p>Optional guidance. You review the effects and choose every order.</p><details><summary>Playtest feedback</summary><button onclick="openPlaytestFeedback()">Record playtest feedback</button></details>`;
      // Even a read-only destination must retain the campaign and recommendation
      // the player saw. The effects screen requests a fresh authoritative quote.
      const current=()=>{
        if(seq!==DTOOLS.seq||!box.open)return false;
        if(S!==state){box.querySelector("#advisorStatus").textContent="The world has changed. Refresh advice before following this recommendation.";return false;}
        return true;
      };
      box.querySelector("#advisorNext").onclick=()=>{
        if(!current())return false;
        if(g.action==="suggestion" && (typeof advancing!=="undefined" && advancing || typeof pendingAdvance!=="undefined" && pendingAdvance || typeof PROD!=="undefined" && PROD.busy)){
          box.querySelector("#advisorStatus").textContent="Wait for the current turn or order to finish before reviewing this suggestion. Resolve any pending confirmation first.";
          return false;
        }
        box.close();
        if(g.action==="budget")openConstructionCabinet("budget");
        else if(g.action==="economy")openIndustry();
        else if(g.action==="suggestion"){openConstruction();return constructionPreviewProject(s.project_kind,s.district,s.capacity_micros);}
        else openConstruction({project:p?.id});
      };
      box.querySelector("#advisorExchange").onclick=()=>{if(!current())return false;box.close();openConstruction({catalog:true});};
      box.querySelector("#advisorCauses")?.addEventListener("click",()=>{if(!current())return false;box.close();openConstructionCabinet("policy");});
      box.querySelectorAll("[data-stability-action]").forEach(b=>b.onclick=()=>{
        if(!current())return false;box.close();
        if(b.dataset.stabilityAction==="decisions")openAgency();
        else if(b.dataset.stabilityAction==="world")toggleGameDrawer("intelDrawer");
        else openConstructionCabinet("budget");
      });
    }catch(error){if(seq===DTOOLS.seq&&box.open)box.querySelector(".decision-body").innerHTML=`<p role="alert">${toolsEsc(error.message)}</p><button onclick="openAdvisor()">Retry</button>`;}
  };
  window.openResearchList=function(){
    const box=toolsDialog("Research decisions",`${globalThis.AreaArt?.html("research", "banner") || ""}<label for="researchListQuery">Search discoveries and effects</label><input id="researchListQuery" type="search"><label for="researchListFilter">Show</label><select id="researchListFilter"><option value="now">Available now</option><option value="available">Prerequisites met</option><option value="all">All discoveries</option><option value="known">Already held</option></select><p>Estimates use today's research allocation and price. Calendar gates and prerequisites still apply; changing focus uses the existing command preview.</p><div id="researchListRows" class="decision-list"></div>`);
    const draw=()=>{const rows=DecisionTools.research(tech.data||[],"all",box.querySelector("input").value,box.querySelector("select").value);box.querySelector("#researchListRows").innerHTML=rows.map(n=>`<article><h3>${toolsEsc(n.name)}</h3><p>${toolsEsc(n.domain)} · ${n.state==="known"?"Already held":n.state==="locked"?"Prerequisites missing":n.earliest_available?"Available now":`Calendar gate: ${n.year}`}</p><p>${(n.effects||[]).map(toolsEsc).join(" · ")||"Opens the listed successor technologies."}</p><p>Prerequisites: ${(n.prereqs||[]).map(p=>`${toolsEsc(p.name)} ${p.known?"✓":"(missing)"}`).join(", ")||"none"}</p><p>${n.state==="known"?"":n.estimated_days==null?"No reliable completion estimate at current funding.":`About ${n.estimated_days} days at today's allocation.`} ${n.floor_binds?"Physical build-cost floor is binding.":""}</p><button data-research-id="${toolsEsc(n.id)}">Inspect ${toolsEsc(n.name)}</button></article>`).join("")||"<p>No matching discoveries.</p>";box.querySelectorAll("[data-research-id]").forEach(b=>b.onclick=()=>{box.close();const i=tech.byId.get(b.dataset.researchId);if(i!==undefined)techGoTo(i);});};box.querySelector("input").oninput=draw;box.querySelector("select").onchange=draw;draw();box.querySelector("input").focus();
  };
  window.openPlaytestFeedback=function(){const box=toolsDialog("Playtest notes",`<p>Try a funded project through to operation, research a discovery, respond to diplomacy, then save and resume. Record where the next action or consequence was unclear.</p><label for="playtestNotes">What happened, what you expected, and how to reproduce it</label><textarea id="playtestNotes" rows="8"></textarea><button id="downloadPlaytest">Download notes</button><p>Saved to your device. Nothing is sent automatically.</p>`);box.querySelector("button#downloadPlaytest").onclick=()=>{const blob=new Blob([JSON.stringify({version:S?.build,date:S?.date,nation:S?.player,notes:box.querySelector("textarea").value},null,2)],{type:"application/json"});const a=document.createElement("a");a.href=URL.createObjectURL(blob);a.download="spheres-playtest.json";a.click();setTimeout(()=>URL.revokeObjectURL(a.href),1000);};};
}
