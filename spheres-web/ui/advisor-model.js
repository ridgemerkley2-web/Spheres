/* Read-only advice from /api/state and an optional, matching /api/production.
 * These are review destinations, never commands or a second simulation. */
(function(root,factory){
  const api=factory();
  if(typeof module==="object"&&module.exports)module.exports=api;
  else root.AdvisorModel=api;
})(typeof globalThis!=="undefined"?globalThis:this,function(){
  "use strict";
  const LIMIT=12, ranks={attention:0,opportunity:1,routine:2};
  const obj=v=>v!==null&&typeof v==="object"&&!Array.isArray(v);
  const rows=v=>Array.isArray(v)?v.filter(obj):[];
  const str=v=>typeof v==="string"&&v.trim().length>0;
  const finite=v=>typeof v==="number"&&Number.isFinite(v);
  const nonnegative=v=>finite(v)&&v>=0;
  const count=v=>Number.isSafeInteger(v)&&v>=0;
  const text=v=>str(v)?v.trim():null;
  const number=v=>v.toFixed(4).replace(/\.?0+$/,"");
  const money=v=>`$${number(v)}bn`;
  const order=(a,b)=>a<b?-1:a>b?1:0;
  function dayIndex(year,month,day){
    if(!Number.isInteger(year)||year<1990||year>9999||!Number.isInteger(month)||month<1||month>12||!Number.isInteger(day)||day<1||day>31)return null;
    const time=Date.UTC(year,month-1,day),date=new Date(time);
    if(date.getUTCFullYear()!==year||date.getUTCMonth()!==month-1||date.getUTCDate()!==day)return null;
    return (time-Date.UTC(1990,0,1))/86400000;
  }
  function isoDay(value){
    if(typeof value!=="string"||!/^\d{4}-\d{2}-\d{2}$/.test(value))return null;
    const [y,m,d]=value.split("-").map(Number);
    return dayIndex(y,m,d);
  }
  function projectAction(project){return count(project.id)?{kind:"project",id:project.id}:{kind:"construction"};}
  function candidate(item,production){
    if(!str(item.id)||!str(item.project_kind)||!str(item.district)||!str(item.name)||!str(item.reason)||!nonnegative(item.cost_bn)||!Number.isSafeInteger(item.minimum_days)||item.minimum_days<=0)return false;
    if(["can_start","eligible"].some(k=>Object.hasOwn(item,k)&&item[k]!==true))return false;
    if(!Array.isArray(item.evidence)||!item.evidence.some(str))return false;
    const provinces=rows(production.provinces).filter(p=>p.id===item.district);
    if(provinces.length!==1)return false;
    const province=provinces[0];
    if(item.project_kind==="starter_industry"){
      // This sized project has no catalog row. The server filters suggestions
      // through its exact-size can_start preview; standard-size permission is
      // intentionally insufficient for a smaller, affordable workshop.
      return Number.isSafeInteger(item.capacity_micros)&&item.capacity_micros>0&&item.capacity_micros<=1000000;
    }
    if(item.capacity_micros!==null&&item.capacity_micros!==undefined)return false;
    const catalog=rows(production.catalog).filter(p=>p.kind===item.project_kind);
    return catalog.length===1&&catalog[0].actions?.start===true
      &&Array.isArray(catalog[0].eligible_provinces)&&catalog[0].eligible_provinces.includes(item.district)
      &&Array.isArray(province.actions?.start)&&province.actions.start.includes(item.project_kind)
      &&!text(province.start_refusals?.[item.project_kind])&&!text(catalog[0].start_reason);
  }
  function evaluate(state,production=null,outcomes=null,receipts=null){
    if(!obj(state)||!str(state.player))return [];
    const players=rows(state.nations).filter(n=>n.id===state.player&&n.alive===true);
    if(players.length!==1)return [];
    const me=players[0],day=dayIndex(state.year,state.month,state.day),cards=[];
    const add=(id,area,priority,title,reason,evidence,caution,action,actionLabel)=>{
      if(cards.some(c=>c.id===id))return;
      cards.push({id,area,priority,title,reason,evidence:evidence.filter(str).slice(0,5),caution,action,actionLabel});
    };
    // programs.due is also true while programmes are disabled, so it needs enabled===true.
    const annualDue=me.annual_budget?.due===true||(state.programs?.enabled===true&&state.programs?.due===true);
    if(annualDue)add("annual-budget","economy","attention","Review your yearly budget",
      "The current budget reading says annual renewal is due.",
      Number.isSafeInteger(me.annual_budget?.fiscal_year)?[`Budget year: ${me.annual_budget.fiscal_year}.`]:[],
      "Review ministry allocations and their effects before enacting a budget.",{kind:"budget"},"Review yearly budget");
    // Deficits spend cash down to zero before borrowing, so zero is ordinary play.
    if(finite(me.treasury)&&me.treasury<0)add("treasury-balance","economy","attention","Review your treasury balance",
      "Your national treasury reports a negative cash balance. Deficits normally spend cash down to zero and then borrow, so a balance below zero needs review.",[`Treasury: ${money(me.treasury)}.`],
      "This balance alone does not establish borrowing limits or whether a particular order is affordable.",{kind:"budget"},"Review national finances");
    const recovery=obj(state.fiscal_recovery)?state.fiscal_recovery:null;
    if(recovery&&recovery.recovery_required===true&&["adjustment","crisis"].includes(recovery.status))add("fiscal-recovery","economy","attention",
      recovery.status==="crisis"?"Review the fiscal confidence crisis":"Review money and recovery",
      text(recovery.reason)||"The fiscal recovery reading says an adjustment is required.",
      [text(recovery.status_label)?`Status: ${text(recovery.status_label)}.`:null,count(recovery.months_observed)?`Months observed: ${recovery.months_observed}.`:null],
      text(recovery.next_action)||"Review taxes, actual spending and construction funding together. This card changes no policy.",{kind:"cash_flow"},"Review money and recovery");

    // A dated, matching production view is required for all queue/funding advice.
    // The controller must additionally protect against same-day stale responses.
    const works=obj(production)&&production.mode==="province_projects"&&production.nation===state.player
      &&day!==null&&production.suggestions?.as_of_day===day?production:null;
    if(works){
      const budget=obj(works.construction_budget)?works.construction_budget:{};
      const queue=[...rows(works.queue),...rows(works.mine_queue)];
      const unopened=budget.enrolled===false;
      const paused=budget.enrolled===true&&budget.daily_budget_bn===0;
      const exhausted=queue.length>0&&finite(budget.available_bn)&&budget.available_bn<=0;
      if(unopened)add("construction-enrollment","economy","attention","Open construction funding",
        "The construction ledger is not enrolled in a funding budget.",[],
        text(budget.reason)||"Review the daily cash limit and available civilian capital appropriations.",{kind:"construction"},"Review construction funding");
      else if(paused)add("construction-paused","economy","attention","Construction funding is paused",
        "The enrolled daily construction limit is zero.",["Daily construction limit: $0bn."],
        "A pause may be intentional. Review the limit before deciding whether to fund more work.",{kind:"construction"},"Review daily funding");
      else if(exhausted)add("construction-cash","economy","attention","Review available construction funding",
        text(budget.reason)||"The construction reading reports no available cash for queued work.",[`Available construction funding: ${money(budget.available_bn)}.`],
        "Review funding and project prerequisites; this reading does not promise a completion date.",{kind:"construction"},"Review construction funding");
      const blocked=queue.filter(p=>["blocked","paused","slowed"].includes(p.status))
        .sort((a,b)=>(["blocked","paused","slowed"].indexOf(a.status)-["blocked","paused","slowed"].indexOf(b.status))||order(String(a.id),String(b.id)))[0];
      const unassigned=works.industry_rebuild?.enabled===true?queue.find(p=>p.allocation?.assigned===0&&finite(p.allocation.requested)&&p.allocation.requested>0):null;
      if(unassigned)add("construction-allocation","economy","attention","Review construction capacity allocation",
        "An active industry rebuild reports a queued project with requested capacity and none assigned.",
        [`Project: ${text(unassigned.name)||"Construction project"}.`,`Requested capacity: ${number(unassigned.allocation.requested)}; assigned capacity: 0.`],
        "Capacity allocation is separate from the cash budget. Inspect the current allocation before adding more projects.",projectAction(unassigned),"Review project allocation");
      if(blocked)add("construction-project","economy","attention",`Review ${text(blocked.name)||"a project needing attention"}`,
        text(blocked.reason)||`The construction queue reports this project as ${blocked.status}.`,
        [`Status: ${blocked.status}.`,str(blocked.province?.name)?`Province: ${blocked.province.name}.`:null],
        "The effects and funding screens provide the current requirements. No additional project is ordered here.",projectAction(blocked),"Review this project");
      const canPlan=!annualDue&&!unopened&&!paused&&!exhausted&&!blocked&&!unassigned&&budget.enrolled===true
        &&finite(budget.daily_budget_bn)&&budget.daily_budget_bn>0&&finite(budget.available_bn)&&budget.available_bn>0;
      const suggestion=canPlan&&state.simulation_cadence==="daily"?rows(works.suggestions.items).find(s=>candidate(s,works)):null;
      if(suggestion)add("construction-suggestion","economy","opportunity",`Review ${suggestion.name}`,
        suggestion.reason,[`Province: ${text(suggestion.district_name)||suggestion.district}.`,`Project cost: ${money(suggestion.cost_bn)}; minimum lead time: ${suggestion.minimum_days} days.`,...suggestion.evidence],
        text(suggestion.caution)||"This is a current server suggestion. Review a fresh effects quote before choosing to build.",
        {kind:"suggestion",project_kind:suggestion.project_kind,district:suggestion.district,capacity_micros:suggestion.capacity_micros??null},"Review suggested effects");
      else if(!unopened&&!paused&&!exhausted&&!blocked&&!unassigned){
        const building=queue.find(p=>p.status==="building"&&nonnegative(p.progress)&&p.progress<=1);
        if(building)add("construction-progress","economy","routine","Review construction progress",
          "The queue contains a project marked as building.",[`Project: ${text(building.name)||"Construction project"}.`,`Recorded progress: ${number(building.progress*100)}%.`],
          "Status and progress are the current reading; future funding and completion times can change.",projectAction(building),"View construction queue");
        else if(rows(works.completed).some(p=>str(p.province?.id)))add("completed-industry","economy","routine","Inspect your completed industry",
          "The construction reading lists completed facilities or industrial modules.",[],
          "Check actual operating output, inputs and running budgets before expanding.",{kind:"industry"},"Review industry");
        else if(Array.isArray(works.queue)&&Array.isArray(works.mine_queue)&&queue.length===0)add("construction-explore","economy","routine","Explore construction options",
          "The current construction queues are empty.",[],
          "No verified suggestion is being recommended. Compare province and country effects before choosing a project.",{kind:"construction"},"Explore construction");
      }
    }else if(count(state.production_summary?.attention)&&state.production_summary.attention>0){
      const summary=state.production_summary;
      add("construction-summary","economy","attention","Review construction needing attention",
        "Your state summary reports projects needing attention.",[`Projects needing attention: ${summary.attention}.`],
        "Detailed funding and project causes are unavailable in this reading. Open Construction for a fresh view.",
        count(summary.attention_ids?.[0])?{kind:"project",id:summary.attention_ids[0]}:{kind:"construction"},"Review construction");
    }

    const takeover=obj(me.takeover)?me.takeover:{};
    for(const [key,label] of [["coup","Coup"],["uprising","Uprising"],["round_table","Round-table transition"],["collapse","State collapse"]]){
      const route=takeover[key];
      if(!obj(route)||route.open!==true||route.armed!==true&&route.half_armed!==true)continue;
      add(`government-${key}`,"government","attention",`Review ${label.toLowerCase()} warning`,
        `The government reading marks this available transition route as ${route.armed===true?"armed":"half-armed"}.`,
        rows(route.gauges).filter(g=>g.met===true&&str(g.name)).map(g=>`Reported condition met: ${g.name}.`),
        text(route.reason)||"This is a served warning state, not a predicted event or a new threshold calculated by the advisor.",{kind:"government"},"Review government conditions");
    }

    const research=obj(state.research)&&state.research.nation===me.name?state.research:null;
    if(research){
      const domains=rows(research.domains).filter(d=>str(d.domain)&&str(d.name));
      const stalled=domains.find(d=>obj(d.project)&&str(d.project.name)&&d.wait==="stalled"&&nonnegative(d.cost)&&nonnegative(d.banked)&&d.cost>d.banked&&nonnegative(d.rate)&&d.rate<=1e-9);
      if(stalled)add("research-stalled","research","attention","Review stalled research effort",
        `${stalled.project.name} is reported stalled with effort still unpaid.`,[`Domain: ${stalled.name}.`,`Banked effort: ${number(stalled.banked)} of ${number(stalled.cost)}.`,`Reported monthly effort: ${number(stalled.rate)}.`],
        "Review the actual research inputs and funding contributors; this card does not assume money is the only cause.",{kind:"research",domain:stalled.domain},"Review research inputs");
      const calendar=domains.find(d=>obj(d.project)&&str(d.project.name)&&d.wait==="year"&&Number.isSafeInteger(d.project.year)&&Number.isSafeInteger(state.year)&&d.project.year>state.year&&nonnegative(d.banked)&&nonnegative(d.cost)&&d.banked>=d.cost);
      if(calendar)add("research-calendar","research","routine","Research is waiting for its availability year",
        `${calendar.project.name} has paid its research effort and is waiting for ${calendar.project.year}.`,[`Domain: ${calendar.name}.`,`Banked effort: ${number(calendar.banked)} of ${number(calendar.cost)}.`],
        "Increasing research funding does not bring the technology's calendar availability forward.",{kind:"research",domain:calendar.domain},"Review calendar limit");
      const quota=domains.find(d=>obj(d.project)&&d.acquisition_wait===true&&d.acquisitions_remaining===0);
      if(quota)add("research-quota","research","routine","Review the research acquisition limit",
        "A selected research domain reports no acquisitions remaining in this month's allowance.",[`Domain: ${quota.name}.`,"Remaining acquisitions: 0."],
        "This monthly acquisition limit is separate from research funding. Review the served reset information.",{kind:"research",domain:quota.domain},"Review acquisition allowance");
      const ready=domains.map(d=>({domain:d,option:d.project===null?rows(d.options).find(o=>str(o.id)&&str(o.name)&&Number.isSafeInteger(o.year)&&o.year>=0&&Number.isSafeInteger(state.year)&&o.year<=state.year):null})).find(r=>r.option);
      if(ready)add("research-project","research","opportunity","Choose a current research project",
        `${ready.domain.name} has no selected project and lists ${ready.option.name} among its current-year options.`,[`Listed availability year: ${ready.option.year}.`],
        "Review prerequisites, costs and alternatives before choosing. Researching an option is a separate decision.",{kind:"research",domain:ready.domain.domain},"Compare research options");
      else if(research.priority===null&&domains.some(d=>obj(d.project))&&finite(research.monthly)&&research.monthly>0)add("research-focus","research","opportunity","Review your research priority",
        "Research effort is positive and projects are selected, but no priority domain is set.",[`Reported research effort: ${number(research.monthly)} per month.`],
        "A priority redistributes emphasis. Compare domains and current waits before changing it.",{kind:"research"},"Review research focus");
    }

    const wars=rows(state.wars).filter(w=>count(w.id)&&rows(w.posture).some(p=>p.id===state.player));
    const warIds=new Set(wars.map(w=>w.id));
    const deployments=state.operations?.enabled===true?rows(state.operations.deployments).filter(d=>d.nation===state.player&&warIds.has(d.conflict)):[];
    const short=deployments.filter(d=>nonnegative(d.requested)&&nonnegative(d.deployed)&&d.requested>d.deployed);
    if(short.length)add("military-deployment","military","attention","Review deployment shortfalls",
      "An active player operation reports fewer deployed forces than requested.",short.slice(0,3).map(d=>`Conflict ${d.conflict}: ${number(d.deployed)} deployed of ${number(d.requested)} requested.`),
      "Review allocation, available forces and overseas limits. A shortfall does not identify its cause by itself.",{kind:"world"},"Review active operations");
    else if(wars.length)add("military-conflicts","military","attention","Review your active conflicts",
      "The live conflict roster explicitly includes your country.",wars.slice(0,3).map(w=>`${text(w.theatre_name)||text(w.theatre)||`Conflict ${w.id}`}${str(w.class)?`: ${w.class}`:""}.`),
      "Review each conflict's objectives and escalation level; roster membership alone does not establish active combat.",{kind:"world"},"Review conflicts");
    if(count(state.manufacturing_summary?.attention)&&state.manufacturing_summary.attention>0)add("equipment-attention","military","attention","Review equipment needs",
      "The equipment manufacturing summary reports items needing attention.",[`Items needing attention: ${state.manufacturing_summary.attention}.`],
      "Inspect the actual equipment or program constraints before making a purchase or changing a design.",{kind:"equipment"},"Review equipment");

    const alive=new Set(rows(state.nations).filter(n=>n.alive===true&&str(n.id)).map(n=>n.id));
    // Only agency.offers and resources.offers are served incoming requests.
    // Stratagems and domination offers are menus of possible player actions.
    const offers=day===null?[]:rows(state.agency?.offers).filter(o=>count(o.id)&&str(o.title)&&o.from!==state.player&&alive.has(o.from)&&count(o.days_remaining)&&isoDay(o.expires)!==null&&isoDay(o.expires)>=day&&isoDay(o.expires)-day===o.days_remaining)
      .sort((a,b)=>(isoDay(a.expires)-isoDay(b.expires))||(a.id-b.id));
    if(offers.length){
      const offer=offers[0];
      add("diplomacy-request","diplomacy",offer.days_remaining===0?"attention":"opportunity","Review an incoming diplomatic request",
        offer.title,[`From: ${text(offer.from_name)||offer.from}.`,`Expires: ${offer.expires}.`,text(offer.consequence)],
        text(offer.accept_blocked)||"Review the stated consequences and deadline before responding; no response is sent by the advisor.",{kind:"decisions"},"Review diplomatic requests");
    }
    const deals=rows(state.resources?.offers).filter(o=>count(o.id)&&str(o.legs)&&o.from_id!==state.player&&alive.has(o.from_id)&&count(o.expires_in_days)&&nonnegative(o.accept_pc))
      .sort((a,b)=>(a.expires_in_days-b.expires_in_days)||(a.id-b.id));
    if(deals.length){
      const deal=deals[0];
      add("diplomacy-resource-offer","diplomacy",deal.expires_in_days===0?"attention":"opportunity","Review an incoming resource offer",
        deal.legs,[`From: ${text(deal.from)||deal.from_id}.`,`Deadline in days: ${deal.expires_in_days}.`,`Acceptance political cost: ${number(deal.accept_pc)}.`],
        "Review both sides of the exchange and current affordability before responding. No trade is accepted here.",{kind:"resources"},"Review resource offers");
    }
    // One route card at most: the first unachieved step with an obstacle, unless a card already opens that screen.
    if(outcomes!==null&&outcomes!==undefined){
      const route=recognize({state,production,outcomes},receipts);
      const next=route.status==="ready"?route.steps.find(s=>s.status!=="achieved"&&s.obstacle):null;
      const action=next?{...next.obstacle.action}:null;
      if(next&&!cards.some(c=>JSON.stringify(c.action)===JSON.stringify(action)))add("route-next",ROUTE_AREAS[next.id],"opportunity",next.obstacle.title,next.obstacle.reason,
        [`First-hour step: ${next.title}.`,next.summary],"Route advice comes from dated campaign records. It opens a review screen and sends no order.",action,next.obstacle.actionLabel);
    }
    return cards.map((card,index)=>({card,index})).sort((a,b)=>ranks[a.card.priority]-ranks[b.card.priority]||a.index-b.index)
      .slice(0,LIMIT).map(row=>row.card);
  }

  /* First-hour route. Results come only from dated native records in one identity-checked
   * /api/guidance reading (response.outcomes); save/load receipts are browser copies of native
   * responses. Reading progress never enters here. Pure: no storage, clock, commands or retained input. */
  const MONTHS=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
  const ROWS=2000,RECEIPTS=20,MAXDAY=dayIndex(9999,12,31),BAD=Symbol("bad"),FUTURE=Symbol("future");
  const STEPS=[
    ["finances","Enact a yearly budget",["budget-treasury"],["budget_enacted"],["budget_enacted"]],
    ["construction","Fund a useful construction project",["construction-effects","industry-operations"],["work_paid","project_completed","site_producing"],["work_paid","project_completed"]],
    ["research_design","Research toward a design",["research-components"],["research_active","design_saved","design_commissioned","development_paid","research_completed"],["design_saved","design_commissioned","development_paid","research_completed"]],
    ["procurement","Buy equipment from a company",["equipment-procurement"],["manufacturer","certified_product","purchase_placed","purchase_paid","equipment_delivered"],["equipment_delivered"]],
    ["air_force","Prepare an air force",["air-force"],["base_funded","base_completed","squadron_formed","squadron_ready","mission_flown"],["base_completed","squadron_ready"]],
    ["save_resume","Save and resume",["save-review"],["campaign_saved","campaign_resumed"],["campaign_resumed"]]];
  const LABELS={budget_enacted:"Yearly budget enacted",work_paid:"Construction work paid",project_completed:"Project completed",site_producing:"Completed site producing",
    research_active:"Component research under way",design_saved:"Design saved",design_commissioned:"Design commissioned",development_paid:"Development paid",research_completed:"Component research completed",
    manufacturer:"Manufacturer established",certified_product:"Certified product available",purchase_placed:"Purchase placed",purchase_paid:"Purchase paid",equipment_delivered:"Equipment delivered",
    base_funded:"Airbase work funded",base_completed:"Airbase improvement completed",squadron_formed:"Squadron formed",squadron_ready:"Squadron ready",mission_flown:"Mission flown",
    campaign_saved:"Campaign saved",campaign_resumed:"Resumed from a save"};
  const ROUTE_AREAS={finances:"economy",construction:"economy",research_design:"research",procurement:"military",air_force:"military",save_resume:"economy"};
  // Own data properties only: getters, prototypes and holes read as missing.
  function own(o,k){if(o===null||typeof o!=="object")return undefined;const d=Object.getOwnPropertyDescriptor(o,k);return d&&"value" in d?d.value:undefined;}
  function list(o,k){const v=own(o,k);if(!Array.isArray(v))return null;const n=own(v,"length");if(!Number.isSafeInteger(n)||n<0)return null;
    const out=[];for(let i=0;i<Math.min(n,ROWS);i++)out.push(own(v,String(i)));return {rows:out,over:n>ROWS};}
  const dayOf=(v,asOf)=>v===null?null:!Number.isSafeInteger(v)||v<0?BAD:v>asOf?FUTURE:v;
  const iso=d=>Number.isSafeInteger(d)&&d>=0&&d<=MAXDAY?new Date(Date.UTC(1990,0,1)+d*86400000).toISOString().slice(0,10):null;
  const clip=(v,n=120)=>typeof v==="string"?v.slice(0,400).trim().slice(0,n)||null:null;
  const short=v=>typeof v==="string"&&v.length<=200&&v.trim()!==""?v:null;
  function dateDay(v){const m=typeof v==="string"&&v.length<=11?/^([1-9]\d?) ([A-Z][a-z]{2}) (\d{4})$/.exec(v):null,month=m?MONTHS.indexOf(m[2]):-1;return month<0?null:dayIndex(Number(m[3]),month+1,Number(m[1]));}
  function freeze(v){if(v&&typeof v==="object"){Object.values(v).forEach(freeze);Object.freeze(v);}return v;}
  const mile=(id,status,day=null,detail=null)=>({id,label:LABELS[id],status,day:status==="done"?day:null,date:status==="done"?iso(day):null,detail});
  const block=(title,reason,action,actionLabel)=>({title,reason,action,actionLabel});
  const blank=(def,why)=>assemble(def,def[3].map(id=>mile(id,"unknown")),null,why);
  // test(row) returns {day,detail} when the row proves the milestone, BAD when it is malformed, otherwise null.
  function scan(id,sets,test,pending=null){
    let best=null,bad=false;
    for(const set of sets){if(!set){bad=true;continue;}if(set.over)bad=true;
      for(const row of set.rows){const hit=test(row);if(hit===BAD)bad=true;else if(hit&&(best===null||(hit.day??Infinity)<(best.day??Infinity)))best=hit;}}
    return best?mile(id,"done",best.day,best.detail):bad?mile(id,"unknown"):mile(id,"pending",null,pending);
  }
  const tally=(id,v,what)=>count(v)?v>0?mile(id,"done",null,`${what}: ${v}.`):mile(id,"pending"):mile(id,"unknown");
  function assemble([id,title,lessons,,goals],ms,obstacle,why){
    const done=ms.filter(m=>m.status==="done"),goal=done.filter(m=>goals.includes(m.id));
    const status=goal.length?"achieved":ms.some(m=>goals.includes(m.id)&&m.status==="unknown")?"unknown":done.length?"progress":ms.some(m=>m.status==="unknown")?"unknown":"not_yet";
    const lead=status==="achieved"?goal.reduce((a,m)=>(m.day??Infinity)<(a.day??Infinity)?m:a):status==="progress"?done[done.length-1]:null;
    const summary=lead?`${status==="progress"?`In progress: ${lead.label.toLowerCase()}`:lead.label}${lead.date?` on ${lead.date}`:""}.`
      :status==="not_yet"?"Not yet achieved in this campaign.":why||"This reading does not have enough data to judge this step.";
    return {id,title,lessons:[...lessons],status,summary,milestones:ms,obstacle:(status==="not_yet"||status==="progress")&&obstacle?obstacle():null};
  }
  const unknownRoute=reason=>freeze({status:"unknown",reason,as_of:null,steps:STEPS.map(def=>blank(def,reason))});
  // A dated payment: positive last_spent_bn needs a valid last_day; unpaid rows carry null or zero.
  function payment(r,c,detail){if(!obj(r))return BAD;const spent=own(r,"last_spent_bn");if(spent===null||spent===0)return null;if(!finite(spent)||spent<0)return BAD;
    const d=dayOf(own(r,"last_day"),c.day);return d===BAD||d===null?BAD:d===FUTURE?null:{day:d,detail:detail(spent)};}
  // Player-facing province name when served; the id stays the matching key.
  const where=r=>clip(own(r,"district_name"),60)||clip(own(r,"district"),40);
  const place=r=>[clip(own(r,"kind"),60)?.replace(/_/g," "),where(r)].filter(Boolean).join(" in ")||"a project";

  function finances(def,sec,c){
    const start=dayIndex(c.year,1,1),decisions=own(sec,"journal_available")===true?list(sec,"decisions"):null;
    let enacted=scan("budget_enacted",[decisions],r=>{
      // construction_budget natively enacts this year's plan (programs::set_construction_budget dispatches SetAnnualBudget{fiscal_year: w.year}).
      if(!obj(r))return BAD;const kind=own(r,"kind");if(typeof kind!=="string")return BAD;if(kind!=="program_budget"&&kind!=="annual_budget"&&kind!=="construction_budget")return null;
      const d=dayOf(own(r,"day"),c.day);return d===BAD?BAD:typeof d==="number"&&d>=start?{day:d,detail:kind==="annual_budget"?"Annual budget enacted.":kind==="construction_budget"?"Inherited plan enacted with construction funding.":"Ministry budget enacted."}:null;
    },`No ${c.year} budget decision in the money journal yet.`);
    if(enacted.status==="pending"){
      // The journal keeps only its newest decisions; a trimmed book reaching into this year cannot prove absence.
      let first=null;for(const r of decisions.rows){const id=own(r,"id");if(!Number.isSafeInteger(id)||id<1){first=BAD;break;}if(first===null||id<first.id)first={id,day:dayOf(own(r,"day"),c.day)};}
      if(first===BAD||first&&first.id>1&&!(typeof first.day==="number"&&first.day<start))enacted=mile("budget_enacted","unknown",null,"Older money-journal entries were trimmed.");
    }
    return assemble(def,[enacted],()=>block("Enact this year's budget",`Your budget sets what each ministry spends in ${c.year}. Until you enact one, the inherited plan keeps running unchanged.`,{kind:"budget"},"Review yearly budget"),
      "The money journal is not available in this campaign.");
  }

  function construction(def,sec,c){
    const projects=list(sec,"projects"),mines=list(sec,"mines"),ends=list(sec,"completions"),sites=new Set(),site=(d,k)=>JSON.stringify([d,k]);let unsure=!ends||ends.over;
    const pre=(l,f)=>l&&{rows:l.rows.map(f),over:l.over};
    // Mine funding stamps last_day on every settle attempt, so it bounds the payment date rather than naming it.
    const mined=r=>{if(!obj(r))return BAD;const spent=own(r,"spent_bn"),last=own(r,"last_day"),d=dayOf(last,c.day),district=where(r),what=clip(own(r,"commodity"),30);
      if(!district||!what)return BAD;if(spent===null)return last===null?null:BAD;if(!finite(spent)||spent<0||d===BAD||spent>0&&d===null)return BAD;
      return spent>0&&typeof d==="number"?{day:null,detail:clip(`Paid mine work: ${money(spent)} for ${what.replace(/_/g," ")} in ${district} by ${iso(d)}.`)}:null;};
    const work=scan("work_paid",[pre(projects,r=>payment(r,c,v=>`Paid ${money(v)} for ${place(r)}.`)),pre(mines,mined)],x=>x);
    // district/kind are null when a headline cannot be mapped exactly; any other value is malformed.
    const completed=scan("project_completed",[ends],r=>{if(!obj(r))return BAD;const d=dayOf(own(r,"day"),c.day),district=own(r,"district"),kind=own(r,"kind");if(d===BAD)return BAD;
      if(!(district===null||str(district))||!(kind===null||str(kind))){unsure=true;return BAD;}if(typeof d!=="number")return null;
      if(district!==null&&kind!==null)sites.add(site(district,kind));return {day:d,detail:clip(own(r,"text"))};});
    // Output counts only at a site (exact district and kind) of one of this player's dated completions.
    const producing=scan("site_producing",[list(sec,"operating")],r=>{if(!obj(r))return BAD;const out=own(r,"output_daily"),district=own(r,"district"),kind=own(r,"kind"),d=dayOf(own(r,"day"),c.day);
      if(!finite(out)||out<0||!str(district)||!str(kind)||d===BAD)return BAD;if(!(out>0&&typeof d==="number"))return null;
      return sites.has(site(district,kind))?{day:d,detail:`Output recorded at ${place(r)}.`}:unsure?BAD:null;});
    return assemble(def,[work,completed,producing],()=>{
      // A queued mine with a funding row waits for paid work the same way a queued project does.
      const p=projects.rows.find(obj),m=!p&&mines?mines.rows.find(r=>obj(r)&&finite(own(r,"spent_bn"))):null;
      if(!p&&!m)return block("Start a useful construction project","Construction turns money into lasting capacity: jobs, output and a bigger tax base. Compare the effects, then fund one project.",{kind:"construction"},"Explore construction");
      const id=p?own(p,"id"):null,action=count(id)?{kind:"project",id}:{kind:"construction"},b=c.budget,what=p?"project":"mine";
      return obj(b)&&(own(b,"enrolled")===false||own(b,"daily_budget_bn")===0)
        ?block("Open construction funding",`Your ${what} is queued, but the daily construction budget is closed or zero, so no work gets paid.`,action,`Review ${what} funding`)
        :block(`Your ${what} awaits paid work`,"Work is paid as each day closes, within your daily construction budget. Paid work is what turns money into lasting capacity.",action,p?"Review this project":"Review construction");
    });
  }

  function research(def,sec,c){
    const active=own(sec,"active"),progress=own(active,"progress"),cost=own(active,"cost"),component=clip(own(active,"component"),80)?.replace(/_/g," ");
    const act=active===null?mile("research_active","pending",null,"No component research selected.")
      :obj(active)&&component&&finite(progress)&&progress>=0&&finite(cost)&&cost>0?mile("research_active","done",null,`${component}: ${number(progress)} of ${number(cost)} points.`):mile("research_active","unknown");
    // Only a draft the designer's native validity check accepts is a saved design; null means unchecked, so unknown.
    let invalid=false,saved=scan("design_saved",[list(sec,"drafts")],r=>{if(!obj(r))return BAD;const d=dayOf(own(r,"updated_day"),c.day),n=clip(own(r,"name"),60),ok=own(r,"valid");
      if(d===BAD||typeof ok!=="boolean")return BAD;if(typeof d!=="number")return null;if(!ok){invalid=true;return null;}return {day:d,detail:n?`Saved "${n}".`:"Design saved."};});
    if(invalid&&saved.status==="pending")saved=mile("design_saved","pending",null,"Saved drafts are not valid designs yet.");
    const commissioned=scan("design_commissioned",[list(sec,"revisions")],r=>{if(!obj(r))return BAD;const imported=own(r,"imported");if(imported===true)return null;if(imported!==false)return BAD;
      const a=dayOf(own(r,"created_day"),c.day),b=dayOf(own(r,"certified_day"),c.day);if(a===BAD||b===BAD)return BAD;
      const d=typeof a==="number"?a:typeof b==="number"?b:null;return d===null?null:{day:d,detail:typeof b==="number"?`Certified on ${iso(b)}.`:"In company development."};});
    const rev=r=>clip(own(r,"revision"),60)?` toward ${clip(own(r,"revision"),60)}`:"";
    const developed=scan("development_paid",[list(sec,"projects")],r=>{const hit=payment(r,c,v=>`Paid ${money(v)}${rev(r)}.`);if(hit!==null)return hit;
      // The daily tick zeroes last_spent_bn on every project; spent_bn keeps the paid total, dated by completion when it has one.
      const total=own(r,"spent_bn"),last=dayOf(own(r,"last_day"),c.day),end=dayOf(own(r,"completed_day"),c.day);if(total===null||total===0)return null;
      if(!finite(total)||total<0||last===BAD||last===null||end===BAD)return BAD;return last===FUTURE||end===FUTURE?null:{day:typeof end==="number"?end:null,detail:`Paid ${money(total)} in total${rev(r)}.`};});
    // start_research clears last_completed_day, but the native learned set only grows, so it keeps the result (undated).
    const last=dayOf(own(sec,"last_completed_day"),c.day),known=own(sec,"learned");
    const learned=last===BAD||!count(known)||typeof last==="number"&&known===0?mile("research_completed","unknown")
      :typeof last==="number"?mile("research_completed","done",last,"A component research project finished.")
      :known>0?mile("research_completed","done",null,`Components researched: ${known}.`):mile("research_completed","pending");
    const fix=saved.status==="pending"&&invalid,designer={kind:"equipment",tab:"designer"};
    return assemble(def,[act,saved,commissioned,developed,learned],()=>act.status==="done"
      ?block(fix?"Make a saved design valid":"Save a design while research runs",`${act.detail} Research counts when it finishes; a ${fix?"valid ":""}saved design counts right away and costs no money.`,designer,"Open the designer")
      :fix?block("Make a saved design valid","Your saved drafts are not valid designs yet. The designer shows what each one still needs; a valid saved design counts right away and costs no money.",designer,"Open the designer")
      :block("Save your first design","A saved design is your blueprint: companies develop it and component research improves it. Saving costs no money.",designer,"Open the designer"));
  }

  function procurement(def,sec,c){
    const maker=tally("manufacturer",own(sec,"companies"),"Companies"),cert=tally("certified_product",own(sec,"certified_products"),"Certified products"),developing=own(sec,"developing_products");
    const buy=(r,foreign)=>{if(!obj(r))return BAD;
      if(foreign){const ammo=own(r,"ammunition"),cancel=own(r,"cancelled_day");if(typeof ammo!=="boolean"||cancel!==null&&!count(cancel))return BAD;if(ammo||cancel!==null)return null;}
      const qty=own(r,"quantity"),due=own(r,"due_day"),st=own(r,"status"),p=dayOf(own(r,"purchased_day"),c.day),s=dayOf(own(r,"settled_day"),c.day),d=dayOf(own(r,"delivered_day"),c.day);
      if(!Number.isSafeInteger(qty)||qty<=0||due!==null&&!count(due)||st!==undefined&&typeof st!=="string"||p===BAD||s===BAD||d===BAD)return BAD;
      if(typeof p!=="number")return null;
      if(typeof s==="number"&&s<p||typeof d==="number"&&(typeof s!=="number"||d<s))return BAD;
      return {placed:p,paid:typeof s==="number"?s:null,got:typeof d==="number"?d:null,due:count(due)?due:null,st:typeof st==="string"&&st.length<=40?st:null,label:`${qty} × ${clip(own(r,"revision"),60)||"equipment"}${foreign?" (import)":""}`};};
    const sets=[list(sec,"deliveries"),list(sec,"imports")].map((l,i)=>l&&{rows:l.rows.map(r=>buy(r,i===1)),over:l.over});
    const at=(id,key,verb)=>scan(id,sets,b=>b===BAD?BAD:b&&b[key]!==null?{day:b[key],detail:`${verb}: ${b.label}.`}:null);
    return assemble(def,[maker,cert,at("purchase_placed","placed","Ordered"),at("purchase_paid","paid","Paid"),at("equipment_delivered","got","Delivered")],()=>{
      // Only a lot the server reports as in_transit is promised an arrival; a blocked lot's due day slides daily and means nothing.
      const live=sets.flatMap(l=>l?l.rows:[]).filter(b=>b&&b!==BAD),open=live.filter(b=>b.paid!==null&&b.got===null),unpaid=live.filter(b=>b.paid===null);
      const stuck=open.find(b=>b.st==="blocked"),transit=open.find(b=>b.st==="in_transit"),waiting=unpaid.find(b=>b.st==="awaiting_settlement");
      if(stuck)return block("Delivery blocked","Shipping for your paid equipment is paused, and it has no arrival date until the cause is resolved. Companies & Procurement shows the reason, such as a lost shipping province or a closed route.",{kind:"companies"},"Review the blocked delivery");
      if(transit){const due=transit.due!==null&&transit.due>=c.day?iso(transit.due):null;return block("Delivery on its way",`Your paid equipment is in transit${due?` and arrives on its due date, ${due}`:""}. No action is needed.`,{kind:"companies"},"Track deliveries");}
      if(open.length)return block("Awaiting delivery","Your paid equipment has not arrived yet. Companies & Procurement shows its current status and the reason.",{kind:"companies"},"Review deliveries");
      if(waiting)return block("Awaiting fiscal settlement","Your order is placed. It is paid when the day's accounts close, and then shipping starts. No action is needed.",{kind:"companies"},"Review your order");
      if(unpaid.length)return block("Order not yet paid","Your order is placed but not paid yet. Companies & Procurement shows its current status and the reason.",{kind:"companies"},"Review your order");
      if(cert.status==="done")return block("Buy from available stock","You have a certified design. A company can sell it once stock is ready, so check what is available and what it costs.",{kind:"companies"},"Review available stock");
      if(maker.status==="done"&&cert.status==="pending")return block("Finish a design in development",`Companies sell only certified designs, and development takes months: at least 180 days for ground designs and 240 for aircraft. ${count(developing)&&developing>0?"One is already in development.":"Start one early."}`,{kind:"companies"},"Review company development");
      return maker.status==="pending"?block("Establish a manufacturer","Equipment is bought from companies. Without a manufacturer, nobody can develop or build your designs.",{kind:"companies"},"Review companies"):null;
    });
  }

  function air(def,sec,c){
    const bases=list(sec,"bases"),works=[],history=[];let bad=!bases||bases.over;
    for(const b of bases?bases.rows:[]){if(!obj(b)){bad=true;continue;}const p=own(b,"project"),h=list(b,"history");if(p!==null)works.push(p);
      if(!h||h.over)bad=true;else for(const r of h.rows){if(history.length>=ROWS){bad=true;break;}history.push(r);}}
    const what=r=>{const level=own(r,"target_level");return `${clip(own(r,"track"),40)?.replace(/_/g," ")||"airbase"}${Number.isSafeInteger(level)?` level ${level}`:""}`;};
    const projects=works.map(p=>{if(!obj(p))return BAD;const paid=own(p,"paid_bn"),total=own(p,"total_cost_bn"),cancel=own(p,"cancelled_day"),last=dayOf(own(p,"last_paid_day"),c.day),fin=dayOf(own(p,"completed_day"),c.day);
      return !finite(paid)||paid<0||!finite(total)||total<0||cancel!==null&&!count(cancel)||last===BAD||fin===BAD?BAD:{paid,total,last,fin,live:cancel===null,what:what(p)};});
    const finished=history.map(h=>{if(!obj(h))return BAD;const d=dayOf(own(h,"completed_day"),c.day);return d===BAD?BAD:typeof d==="number"?{day:d,what:what(h)}:null;});
    const sets=[{rows:projects,over:bad},{rows:finished,over:false}];
    const funded=scan("base_funded",sets,x=>x===BAD?BAD:!x?null:"paid" in x?(x.live&&x.paid>0&&typeof x.last==="number"?{day:x.last,detail:`Paid ${money(x.paid)} of ${money(x.total)} for ${x.what}.`}:null):{day:x.day,detail:`Paid through completion of ${x.what}.`});
    const completed=scan("base_completed",sets,x=>x===BAD?BAD:!x?null:"paid" in x?(x.live&&typeof x.fin==="number"?{day:x.fin,detail:`Completed ${x.what}.`}:null):{day:x.day,detail:`Completed ${x.what}.`});
    const squadrons=list(sec,"squadrons"),wings=squadrons&&{rows:squadrons.rows.map(r=>{if(!obj(r))return BAD;const n=own(r,"assigned"),ready=own(r,"ready"),blocker=own(r,"blocker");
      return !count(n)||typeof ready!=="boolean"||blocker!==null&&typeof blocker!=="string"?BAD:{n,ready:ready&&n>0&&blocker===null,blocker:clip(blocker)};}),over:squadrons.over};
    const blocked=wings?wings.rows.find(x=>x!==BAD&&x.n>0&&x.blocker):null;
    const formed=scan("squadron_formed",[wings],x=>x===BAD?BAD:x.n>0?{day:null,detail:`${x.n} aircraft assigned.`}:null,"Needs delivered aircraft.");
    const ready=scan("squadron_ready",[wings],x=>x===BAD?BAD:x.ready?{day:null,detail:"Aircraft assigned, based and unblocked."}:null,blocked?blocked.blocker:"Needs delivered aircraft and a base.");
    const flown=scan("mission_flown",[list(sec,"missions")],r=>{if(!obj(r))return BAD;const status=own(r,"status");if(typeof status!=="string")return BAD;if(status!=="flown")return null;
      const d=dayOf(own(r,"report_day"),c.day),n=own(r,"aircraft");return d===BAD||d===null||!count(n)?BAD:typeof d==="number"&&n>0?{day:d,detail:`${n} aircraft flew.`}:null;},"Needs ready aircraft and an eligible conflict.");
    return assemble(def,[funded,completed,formed,ready,flown],()=>{
      const live=projects.find(x=>x!==BAD&&x.live&&typeof x.fin!=="number");
      if(!live)return block("Fund an airbase foundation","Aircraft need a home base before squadrons can form. A foundation needs about 12 funded days of construction money.",{kind:"air",page:"bases"},"Review airbases");
      return live.paid>0?block("Keep airbase work funded",`${money(live.paid)} of ${money(live.total)} is paid. Airbase work is paid daily from the construction budget, so keep that funding open until it completes.`,{kind:"air",page:"bases"},"Review airbases")
        :block("Fund your airbase work","Your airbase project has no paid work yet. It advances only on days the construction budget pays for it.",{kind:"air",page:"bases"},"Review airbases");
    });
  }

  // Receipts: own data properties only, the newest RECEIPTS of each kind; invalid rows are ignored.
  function receiptRows(o,k,test){const v=own(o,k);if(!Array.isArray(v))return null;const n=own(v,"length");if(!Number.isSafeInteger(n)||n<0)return null;
    const out=[];for(let i=Math.max(0,n-RECEIPTS);i<n;i++){const r=test(own(v,String(i)));if(r)out.push(r);}return out;}
  const saveRow=r=>{if(!obj(r))return null;const session=short(own(r,"session_id")),player=short(own(r,"player")),slot=short(own(r,"slot")),day=dateDay(own(r,"date"));
    return session&&player&&slot&&day!==null&&count(own(r,"dispatches"))?{session,player,slot,day}:null;};
  const loadRow=r=>{if(!obj(r))return null;const session=short(own(r,"session_id")),player=short(own(r,"player")),slot=short(own(r,"slot")),day=dateDay(own(r,"date")),backup=own(r,"backup");
    return session&&player&&slot&&day!==null&&typeof backup==="boolean"&&count(own(r,"dispatch_count"))?{session,player,slot,day,backup}:null;};
  function saveResume(def,receipts,c){
    try{
      if(!obj(receipts))return blank(def,"Save and load receipts are not available in this browser.");
      const saved=receiptRows(receipts,"saves",saveRow),loaded=receiptRows(receipts,"loads",loadRow),first=a=>a.length?a.reduce((x,y)=>y.day<x.day?y:x):null;
      // Resumed only when this very session began from a native load of this player's campaign.
      const load=loaded&&first(loaded.filter(l=>l.session===c.session&&l.player===c.player&&l.day<=c.day));
      const save=saved&&first(saved.filter(s=>s.session===c.session&&s.player===c.player&&s.day<=c.day||load&&s.slot===load.slot&&s.player===load.player&&s.day===load.day));
      const ms=[saved===null?mile("campaign_saved","unknown"):save?mile("campaign_saved","done",save.day,`Saved "${clip(save.slot,60)}".`):mile("campaign_saved","pending"),
        loaded===null?mile("campaign_resumed","unknown"):load?mile("campaign_resumed","done",load.day,`Loaded "${clip(load.slot,60)}"${load.backup?" from its backup":""}.`):mile("campaign_resumed","pending")];
      return assemble(def,ms,()=>ms[0].status==="done"
        ?block("Resume from your save","Loading proves your save restores the campaign. It replaces anything played since that save, so load when you want to return to it.",{kind:"campaign"},"Open campaigns")
        :block("Save your campaign","A named save lets you return to this exact moment, and saving never changes the campaign.",{kind:"campaign"},"Open campaigns"));
    }catch(_){return blank(def,"Save and load receipts could not be checked.");}
  }

  function read(response,receipts){
    const state=own(response,"state"),outcomes=own(response,"outcomes");
    if(!obj(state))return unknownRoute("No campaign reading is available.");
    const player=short(own(state,"player")),session=short(own(state,"session_id")),date=own(state,"date"),year=own(state,"year");
    if(!player||!session)return unknownRoute("This reading has no player campaign.");
    const nations=list(state,"nations");
    if(!nations||nations.over||nations.rows.filter(n=>obj(n)&&own(n,"id")===player&&own(n,"alive")===true).length!==1)return unknownRoute("Your nation is not uniquely alive in this reading.");
    const day=dayIndex(year,own(state,"month"),own(state,"day"));
    if(day===null||dateDay(date)!==day)return unknownRoute("This reading has no valid date.");
    if(!obj(outcomes))return unknownRoute("This reading has no campaign results.");
    if(own(outcomes,"nation")!==player)return unknownRoute("Campaign results belong to a different nation.");
    if(own(outcomes,"as_of_day")!==day||own(outcomes,"date")!==date)return unknownRoute("Campaign results are from a different date.");
    const production=own(response,"production"),works=obj(production)&&own(production,"nation")===player&&own(production,"as_of_day")===day?production:null;
    const c={day,year,player,session,budget:works?own(works,"construction_budget"):null};
    const section=(key,build,i)=>{const sec=own(outcomes,key);return obj(sec)?build(STEPS[i],sec,c):blank(STEPS[i],"This system is not enabled in this campaign reading.");};
    return {status:"ready",reason:null,as_of:{session_id:session,player,date,day},
      steps:[section("money",finances,0),section("construction",construction,1),section("research",research,2),section("procurement",procurement,3),section("aviation",air,4),saveResume(STEPS[5],receipts,c)]};
  }
  function recognize(response,receipts){
    try{return freeze(read(response,receipts));}catch(_){return unknownRoute("This reading could not be checked.");}
  }
  return Object.freeze({evaluate,recognize});
});
