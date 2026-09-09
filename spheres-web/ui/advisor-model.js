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
  function evaluate(state,production=null){
    if(!obj(state)||!str(state.player))return [];
    const players=rows(state.nations).filter(n=>n.id===state.player&&n.alive===true);
    if(players.length!==1)return [];
    const me=players[0],day=dayIndex(state.year,state.month,state.day),cards=[];
    const add=(id,area,priority,title,reason,evidence,caution,action,actionLabel)=>{
      if(cards.some(c=>c.id===id))return;
      cards.push({id,area,priority,title,reason,evidence:evidence.filter(str).slice(0,5),caution,action,actionLabel});
    };
    const annualDue=me.annual_budget?.due===true||state.programs?.due===true;
    if(annualDue)add("annual-budget","economy","attention","Review your yearly budget",
      "The current budget reading says annual renewal is due.",
      Number.isSafeInteger(me.annual_budget?.fiscal_year)?[`Budget year: ${me.annual_budget.fiscal_year}.`]:[],
      "Review ministry allocations and their effects before enacting a budget.",{kind:"budget"},"Review yearly budget");
    if(finite(me.treasury)&&me.treasury<=0)add("treasury-balance","economy","attention","Review your treasury balance",
      "Your national treasury reports a zero or negative cash balance.",[`Treasury: ${money(me.treasury)}.`],
      "This balance alone does not establish borrowing limits or whether a particular order is affordable.",{kind:"budget"},"Review national finances");

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
    return cards.map((card,index)=>({card,index})).sort((a,b)=>ranks[a.card.priority]-ranks[b.card.priority]||a.index-b.index)
      .slice(0,LIMIT).map(row=>row.card);
  }
  return Object.freeze({evaluate});
});
