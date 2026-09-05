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
    if(!programs?.enabled||programs.due)return {step:0,title:"Fund your plan",text:"Review the ten ministry budgets and the full-use funding gap, including debt service. Enact the plan to open this year's project authority.",action:"budget"};
    const queue=works?.queue||[];
    const blocked=queue.find(p=>p.status!=="building"||(p.requirements||[]).some(r=>r.shortfall>0));
    if(blocked)return {step:2,title:"Unblock the supply chain",text:blocked.reason||"Inspect this project's inputs, department funding and prerequisites.",project:blocked,action:"supply"};
    if(queue.length)return {step:3,title:"Watch work become capacity",text:"Advance a few days, then compare actual work and delivered inputs. The estimate assumes today's throughput continues; it is not a completion promise.",project:queue[0],action:"works"};
    if((works?.completed||[]).length)return {step:4,title:"Put completed capacity to work",text:"Inspect the completed site's operating status, inputs and actual output. Compare its value added with your opening economy; a building alone does not guarantee production.",action:"economy"};
    return {step:1,title:"Choose one useful project",text:"Start with a project whose funding and inputs you can sustain. A small economy can obtain a paid, scaled workshop through Exchange. Every project quote shows prerequisites and political cost before you commit.",action:"works"};
  }
  function causes(policy){return ["war","sanctions","embargo","debt_drag","unrest","oil","bubble","demand_output_now"].map(key=>({key,value:policy?.[key]})).filter(r=>Number.isFinite(r.value)&&r.value!==0).sort((a,b)=>Math.abs(b.value)-Math.abs(a.value)).slice(0,4);}
  return {overlap,search,research,guide,causes};
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
    const box=toolsDialog("Find a nation or province",`<label for="worldFindInput">Name, code or owner</label><input id="worldFindInput" type="search" autocomplete="off" placeholder="Japan, US-CA, Kuwait…"><p id="worldFindCount" role="status"></p><div id="worldFindRows" class="decision-list"></div>`);
    const input=box.querySelector("input");
    function update(){const rows=DecisionTools.search(S.nations,DINDEX,nationOfDistrict,input.value,S.player);box.querySelector("#worldFindCount").textContent=`${rows.length} matches${rows.length===60?" · refine your search for more":""}`;box.querySelector("#worldFindRows").innerHTML=rows.map(r=>`<button type="button" data-find-kind="${r.kind}" data-find-id="${toolsEsc(r.id)}"><strong>${toolsEsc(r.name)}</strong><span>${r.kind} · ${toolsEsc(r.owner||"Unassigned")}</span></button>`).join("");box.querySelectorAll("[data-find-id]").forEach(b=>b.onclick=()=>{box.close();showTab("map");if(b.dataset.findKind==="province")selectProvince(b.dataset.findId,true);else{const p=anchorOf(b.dataset.findId);if(p)camTween(p[0],p[1],2.2);openNation(b.dataset.findId);}});}
    input.oninput=update;input.onkeydown=e=>{if(e.key==="Enter"){e.preventDefault();box.querySelector("[data-find-id]")?.click();}};update();input.focus();
  };
  window.openAdvisor=async function(){
    if(!S?.player)return;DTOOLS.view="guide";const state=S;
    const box=toolsDialog("Your development advisor",`<p role="status">Reading your current budget and projects…</p>`);
    const seq=DTOOLS.seq;
    try{const works=await api("/api/production");if(seq!==DTOOLS.seq||!box.open)return;
      if(S!==state)throw new Error("The campaign changed while advice was loading. Refresh advice to inspect the current state.");
      const g=DecisionTools.guide(state.programs,works),p=g.project,causes=DecisionTools.causes(state.policy);
      const steps=["Fund","Choose","Supply","Build","Operate"];
      const next=p?`<article><h3>${toolsEsc(p.name)} · ${toolsEsc(p.province?.name)}</h3><p>${Math.round((p.progress||0)*100)}% complete · ${p.eta_days==null?"No reliable ETA while blocked":`about ${p.eta_days} days at current throughput`}</p><p>${toolsEsc(p.effect)}</p><ul>${(p.requirements||[]).map(r=>`<li>${toolsEsc(r.name)}: ${r.stock_available} ${toolsEsc(r.unit)} on hand · ${r.incoming_quantity||0} in transit${r.shortfall>0?` · missing ${r.shortfall} for the next work cycle`:""}${r.shortfall>0?` <button data-supply="${toolsEsc(r.commodity)}">Find supplies</button>`:""}</li>`).join("")}</ul><p>Department: ${toolsEsc(p.funding?.ministry_name)}. ${toolsEsc(p.reason||"Ready for funded work.")}</p></article>`:"";
      box.querySelector(".decision-body").innerHTML=`<ol class="advisor-steps">${steps.map((s,i)=>`<li ${i===g.step?'aria-current="step"':""}>${s}</li>`).join("")}</ol><h3>${g.title}</h3><p>${g.text}</p>${next}<div class="decision-actions"><button id="advisorNext">${g.action==="budget"?"Review budget":g.action==="economy"?"Inspect economic output":"Open National Works"}</button><button id="advisorExchange">Scaled workshops & supply planning</button><button onclick="openAdvisor()">Refresh advice</button></div>${causes.length?`<h3>Largest current growth contributions</h3><p>These are the simulation's current terms, not an attribution of every change since your last decision.</p><ul>${causes.map(c=>`<li>${toolsEsc(c.key.replaceAll("_"," "))}: ${c.value>=0?"+":""}${(c.value*100).toFixed(2)} percentage points / year</li>`).join("")}</ul><button id="advisorCauses">Inspect policies and costs</button>`:""}<p>Optional guidance. You choose and enact every policy.</p><button onclick="openPlaytestFeedback()">Record playtest feedback</button>`;
      box.querySelector("#advisorNext").onclick=()=>{box.close();if(g.action==="budget"){if(!cabinetIsOpen())toggleGameDrawer("cabinetDrawer");}else if(g.action==="economy"){openNation(S.player);selectNationView("economy");}else openProduction();};
      box.querySelector("#advisorExchange").onclick=()=>{box.close();openCompetition();};
      box.querySelector("#advisorCauses")?.addEventListener("click",()=>{box.close();if(!cabinetIsOpen())toggleGameDrawer("cabinetDrawer");});
      box.querySelectorAll("[data-supply]").forEach(b=>b.onclick=()=>{box.close();openStock(b.dataset.supply);});
    }catch(error){if(seq===DTOOLS.seq&&box.open)box.querySelector(".decision-body").innerHTML=`<p role="alert">${toolsEsc(error.message)}</p><button onclick="openAdvisor()">Retry</button>`;}
  };
  window.openResearchList=function(){
    const box=toolsDialog("Research decisions",`<label for="researchListQuery">Search discoveries and effects</label><input id="researchListQuery" type="search"><label for="researchListFilter">Show</label><select id="researchListFilter"><option value="now">Available now</option><option value="available">Prerequisites met</option><option value="all">All discoveries</option><option value="known">Already held</option></select><p>Estimates use today's research allocation and price. Calendar gates and prerequisites still apply; changing focus uses the existing command preview.</p><div id="researchListRows" class="decision-list"></div>`);
    const draw=()=>{const rows=DecisionTools.research(tech.data||[],"all",box.querySelector("input").value,box.querySelector("select").value);box.querySelector("#researchListRows").innerHTML=rows.map(n=>`<article><h3>${toolsEsc(n.name)}</h3><p>${toolsEsc(n.domain)} · ${n.state==="known"?"Already held":n.state==="locked"?"Prerequisites missing":n.earliest_available?"Available now":`Calendar gate: ${n.year}`}</p><p>${(n.effects||[]).map(toolsEsc).join(" · ")||"Opens the listed successor technologies."}</p><p>Prerequisites: ${(n.prereqs||[]).map(p=>`${toolsEsc(p.name)} ${p.known?"✓":"(missing)"}`).join(", ")||"none"}</p><p>${n.state==="known"?"":n.estimated_days==null?"No reliable completion estimate at current funding.":`About ${n.estimated_days} days at today's allocation.`} ${n.floor_binds?"Physical build-cost floor is binding.":""}</p><button data-research-id="${toolsEsc(n.id)}">Inspect ${toolsEsc(n.name)}</button></article>`).join("")||"<p>No matching discoveries.</p>";box.querySelectorAll("[data-research-id]").forEach(b=>b.onclick=()=>{box.close();const i=tech.byId.get(b.dataset.researchId);if(i!==undefined)techGoTo(i);});};box.querySelector("input").oninput=draw;box.querySelector("select").onchange=draw;draw();box.querySelector("input").focus();
  };
  window.openPlaytestFeedback=function(){const box=toolsDialog("Playtest notes",`<p>Try a funded project through to operation, research a discovery, respond to diplomacy, then save and resume. Record where the next action or consequence was unclear.</p><label for="playtestNotes">What happened, what you expected, and how to reproduce it</label><textarea id="playtestNotes" rows="8"></textarea><button id="downloadPlaytest">Download notes</button><p>Saved to your device. Nothing is sent automatically.</p>`);box.querySelector("button#downloadPlaytest").onclick=()=>{const blob=new Blob([JSON.stringify({version:S?.build,date:S?.date,nation:S?.player,notes:box.querySelector("textarea").value},null,2)],{type:"application/json"});const a=document.createElement("a");a.href=URL.createObjectURL(blob);a.download="spheres-playtest.json";a.click();setTimeout(()=>URL.revokeObjectURL(a.href),1000);};};
}
