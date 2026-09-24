from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, subprocess

here = Path(__file__).resolve().parent
repo = here.parents[1] / 'integration'
sha = lambda data: hashlib.sha256(data).hexdigest()
old = (here / 'iteration-20-confidence-diagnostic.rs').read_text(encoding='utf-8')
prefix = old.split('#[derive(Default)] struct Summary')[0]
assert 'fn main' not in prefix
# These observational helpers reproduce private read-only national-mass queries;
# runtime simulation and all actions still come from the public compiled library.
prefix = prefix.replace('exact frozen20', 'frozen current source')
prefix = prefix.replace('let material=resources.map(|r|government::army_resource_loyalty_target(r,income,n.war_exhaustion));',
'''let material=Some(resources.map_or_else(
        ||government::army_loyalty_target(n.mil_spend_gdp,n.war_exhaustion,None,government::ARMY_PAY_WEIGHT),
        |r|government::army_resource_loyalty_target(r,income,n.war_exhaustion)));''')
needle = '    let raw_coalition_support:f64='
assert prefix.count(needle) == 1
prefix = prefix.replace(needle, '''    let known_leverage=spheres_sim::army_authority::current_leverage(w,id);
    let has_army=g.pillars.iter().any(|(p,_)|*p==Pillar::Army);
    let political_leverage=if has_army&&is_electoral(w,id) {
        known_leverage.unwrap_or_else(||((n.authoritarianism-0.20)/0.40).clamp(0.0,1.0))
    }else{0.0};
    let record_matches=g.political_record.as_ref().is_some_and(|r|
        g.leader().is_some_and(|p|r.government==format!("party:{p}")));
    let record_established=g.political_record.as_ref().is_some_and(|r|
        record_matches&&r.months>=6.0&&r.performance.is_finite());
    let sustained_crisis=g.political_record.as_ref().map(|r|
        0.5*blocs::discontent(w,id)+0.5*(-r.performance).clamp(0.0,1.0));
    let floor_delta=floor.map(|f|f-n.mil_spend_gdp);
    let target_at_floor=floor.map(|f| {
        let material=personnel.map_or_else(
            ||government::army_loyalty_target(f,n.war_exhaustion,None,government::ARMY_PAY_WEIGHT),
            |p|government::army_resource_loyalty_target(f*n.gdp*1e9/p,income,n.war_exhaustion));
        (material-penalty).clamp(0.0,1.0)
    });
    let below_hysteresis=floor_delta.is_some_and(|d|d>1e-12&&d<0.001)
        &&command_affordable==Some(true)
        &&(blocs::effective_army_loyalty(w,id)<government::ELECTORAL_COUP_ARMY
            ||target.is_some_and(|t|t<government::ELECTORAL_COUP_ARMY));
    let price=floor.and_then(|share|spheres_sim::price_of(w,&spheres_sim::Command::SetMilSpend{nation:id,share}));
    let raw_coalition_support:f64=''')
needle = '"opening_mandate":g.opening_mandate,'
assert prefix.count(needle)==1
prefix=prefix.replace(needle, '''"has_army":has_army,"known_current_leverage":known_leverage,
      "political_channel_leverage":political_leverage,"leverage_basis":if known_leverage.is_some(){"saved_assessment"}else{"unknown_authoritarianism_proxy"},
      "record_matches":record_matches,"record_established":record_established,"sustained_crisis_component":sustained_crisis,
      "proposed_funding_delta":floor_delta,"proposed_funding_price":price,"target_at_proposed_floor":target_at_floor,
      "below_hysteresis_affordable_hostility":below_hysteresis,
      "below_hysteresis_would_reach_coup_line":below_hysteresis&&target_at_floor.is_some_and(|t|t>=government::ELECTORAL_COUP_ARMY),
      "explicit_annual_plan":n.annual_budget.is_some(),"program_owner":spheres_sim::programs::enrolled(w,id),
      "opening_mandate":g.opening_mandate,''')
main = r'''
// Fixed prospective diagnostic: no seed/month arguments and no threshold changes.
const SEED:u64=0;
const MONTHS:u32=252;
fn val(v:&Value,key:&str)->f64{v[key].as_f64().unwrap_or(0.0)}
fn flag(v:&Value,key:&str)->bool{v[key].as_bool().unwrap_or(false)}
fn electoral_army(v:&Value)->bool{flag(v,"electoral")&&flag(v,"has_army")}
fn prerequisites(v:&Value)->bool{
    electoral_army(v)&&!flag(v,"awaiting_first_election")
        &&val(v,"settled_months")>=government::ELECTORAL_COUP_SETTLED as f64
        &&val(v,"discontent")>=government::ELECTORAL_COUP_DISCONTENT
}
fn rank(v:&Value)->(bool,f64,f64,f64){
    (prerequisites(v),val(v,"coup_pressure"),-val(v,"effective_loyalty"),val(v,"confidence_penalty"))
}
#[derive(Default)] struct Summary {
    observed:u32,electoral:u32,army:u32,crisis:u32,settled:u32,pending:u32,
    record_missing:u32,penalty:u32,funded_crisis:u32,below_hysteresis:u32,hysteresis_cross:u32,
    first:u32,repeats:u32,appropriation_changes:u32,appropriation_increases:u32,
    min_effective:Option<Value>,max_pressure:Option<Value>,max_discontent:Option<Value>,closest:Option<Value>,
}
fn emit(v:Value){println!("{}",v)}
fn main(){
    assert_eq!(std::env::args().count(),1,"This prospective trace accepts no alternate seeds or horizons");
    let mut w=world_1990(GameRules{seed:SEED,ideology_blocs:true,ideology_takeover:true,..GameRules::default()});
    assert!(w.player.is_none());
    emit(json!({"kind":"contract","schema":1,"seed":SEED,"months":MONTHS,
        "confidence_weight":government::ARMY_CRISIS_CONFIDENCE_WEIGHT,
        "programme_veto_weight":government::ARMY_PROGRAMME_VETO_WEIGHT,
        "commands":[],"default_other_rules":true,
        "snapshot_scope":"start/end of the whole monthly tick; not the internal pre-coup firing point",
        "funding_scope":"observed appropriation changes and public price quote; no internal command receipt or inferred exact PC debit",
        "helper_scope":"private mandate readers reproduced read-only from frozen source; actual simulation uses the public linked library"}));
    let mut summaries:BTreeMap<NationId,Summary>=BTreeMap::new();
    let mut coup_counts:BTreeMap<NationId,u32>=BTreeMap::new();
    let mut latest_increase:BTreeMap<NationId,Value>=BTreeMap::new();
    for month in 1..=MONTHS {
        let read_hash=spheres_sim::state_hash(&w);
        let mut before=BTreeMap::new();
        for n in w.nations.iter().filter(|n|n.alive) {
            if state(&w,n.id).is_none(){continue;}
            let mut row=snap(&w,n.id);
            row["latest_prior_observed_increase"]=latest_increase.get(&n.id).cloned().unwrap_or(Value::Null);
            let t=summaries.entry(n.id).or_default(); t.observed+=1;
            if flag(&row,"electoral"){t.electoral+=1;}
            if flag(&row,"has_army"){t.army+=1;}
            if electoral_army(&row) {
                if val(&row,"discontent")>=government::ELECTORAL_COUP_DISCONTENT{t.crisis+=1;}
                if val(&row,"settled_months")>=government::ELECTORAL_COUP_SETTLED as f64{t.settled+=1;}
                if flag(&row,"awaiting_first_election"){t.pending+=1;}
                if !flag(&row,"record_established"){t.record_missing+=1;}
                if val(&row,"confidence_penalty")>0.0{t.penalty+=1;}
                if val(&row,"discontent")>=government::ELECTORAL_COUP_DISCONTENT&&flag(&row,"resource_saturated"){t.funded_crisis+=1;}
                if flag(&row,"below_hysteresis_affordable_hostility"){t.below_hysteresis+=1;}
                if flag(&row,"below_hysteresis_would_reach_coup_line"){t.hysteresis_cross+=1;}
                if t.min_effective.as_ref().is_none_or(|old|val(&row,"effective_loyalty")<val(old,"effective_loyalty")){t.min_effective=Some(row.clone());}
                if t.max_pressure.as_ref().is_none_or(|old|val(&row,"coup_pressure")>val(old,"coup_pressure")){t.max_pressure=Some(row.clone());}
                if t.max_discontent.as_ref().is_none_or(|old|val(&row,"discontent")>val(old,"discontent")){t.max_discontent=Some(row.clone());}
                if t.closest.as_ref().is_none_or(|old|rank(&row)>rank(old)){t.closest=Some(row.clone());}
            }
            before.insert(n.id,row);
        }
        assert_eq!(spheres_sim::state_hash(&w),read_hash,"diagnostic reads must be inert");
        let news=tick_month(&mut w,&[]);
        for (&id,old) in &before {
            let Some(n)=w.nation_opt(id).filter(|n|n.alive) else{continue;};
            if state(&w,id).is_none(){continue;}
            let delta=n.mil_spend_gdp-val(old,"military_share");
            if delta.abs()>1e-12 {
                let t=summaries.entry(id).or_default();t.appropriation_changes+=1;
                let observed=json!({"month":month,"before_share":old["military_share"],"after_share":n.mil_spend_gdp,
                    "delta":delta,"before_pc":old["political_capital"],"after_pc":n.political_capital,
                    "prior_floor":old["army_funding_floor"],"prior_price_quote":old["proposed_funding_price"],
                    "prior_affordable":old["funding_command_affordable"],"scope":"whole-tick observed net change; not a command receipt"});
                if delta>0.0 {t.appropriation_increases+=1;latest_increase.insert(id,observed.clone());}
                emit(json!({"kind":"appropriation_change","seed":SEED,"country":format!("{id:?}"),"observation":observed}));
            }
            let now=snap(&w,id);
            if old["holder"]!=now["holder"]||old["coalition"]!=now["coalition"]
                ||old["electoral"]!=now["electoral"]||old["next_election"]!=now["next_election"]
                ||old["awaiting_first_election"]!=now["awaiting_first_election"] {
                // Only Army-bearing states need full lifecycle rows for this audit.
                if flag(old,"has_army")||flag(&now,"has_army") {
                    emit(json!({"kind":"authority_or_calendar_change","seed":SEED,"month":month,
                        "country":format!("{id:?}"),"before":old,"after":now}));
                }
            }
        }
        for headline in news.iter().filter(|h|h.starts_with("COUP IN ")&&h.contains("removes the elected government")) {
            let id=w.nations.iter().map(|n|n.id).find(|id|headline.starts_with(&format!("COUP IN {}:",id.name().to_uppercase())))
                .expect("elected-coup headline must name a country");
            let ordinal=coup_counts.entry(id).or_default();*ordinal+=1;
            let t=summaries.entry(id).or_default();if *ordinal==1{t.first+=1}else{t.repeats+=1};
            emit(json!({"kind":"electoral_coup","seed":SEED,"month":month,"country":format!("{id:?}"),
                "ordinal_in_country":ordinal,"headline":headline,"before":before.get(&id),"after":snap(&w,id)}));
        }
    }
    let mut first=0;let mut repeats=0;
    for (id,t) in summaries {
        first+=t.first;repeats+=t.repeats;
        emit(json!({"kind":"country_summary","seed":SEED,"country":format!("{id:?}"),
            "observed_months":t.observed,"electoral_months":t.electoral,"army_months":t.army,
            "electoral_army_crisis_months":t.crisis,"electoral_army_settled_months":t.settled,
            "electoral_army_pending_months":t.pending,"electoral_army_unestablished_record_months":t.record_missing,
            "electoral_army_positive_penalty_months":t.penalty,"electoral_army_full_resource_crisis_months":t.funded_crisis,
            "below_hysteresis_affordable_hostility_months":t.below_hysteresis,"below_hysteresis_crossing_months":t.hysteresis_cross,
            "first_coups":t.first,"repeat_coups":t.repeats,"appropriation_changes":t.appropriation_changes,
            "appropriation_increases":t.appropriation_increases,"min_effective_loyalty":t.min_effective,
            "max_coup_pressure":t.max_pressure,"max_discontent":t.max_discontent,"closest_prerequisites":t.closest}));
    }
    emit(json!({"kind":"seed_complete","seed":SEED,"months":MONTHS,"first_coups":first,"repeat_coups":repeats,
        "total_electoral_coups":first+repeats,"state_hash":spheres_sim::state_hash(&w),"rng_state":w.rng.state}));
}
'''
out=here/'iteration-24-seed0-trace.rs'
assert not out.exists(), 'Preserve any existing prospective source'
out.write_text(prefix+main,encoding='utf-8',newline='\n')
files=subprocess.check_output(['git','ls-files','-z'],cwd=repo).decode('utf-8').split('\0')
inputs=[]
for rel in sorted(p for p in files if p and (p.startswith('spheres-sim/') or p in ['Cargo.toml','Cargo.lock','.gitattributes'])):
    p=repo/rel
    if not p.is_file():continue
    raw=p.read_bytes()
    inputs.append({'path':rel,'bytes':len(raw),'raw_sha256':sha(raw),'canonical_sha256':sha(raw.replace(b'\r\n',b'\n'))})
plan={'schema':1,'prepared_utc':datetime.now(timezone.utc).isoformat(),
 'status':'Prepared only; no compilation or simulation execution',
 'seed':0,'months':252,'rules':'ideology_blocs=true, ideology_takeover=true, all other GameRules defaults; no commands, no player',
 'hypothesis':'Diagnose observed first/repeat concentration and near-miss causal requirements; no candidate or outcome acceptance claim',
 'output':['actual elected-coup headlines with ordinal','whole-tick appropriation net changes with prior public price/affordability','Army-state authority/calendar changes','all-country extrema and blocker counts','final state/RNG hash'],
 'extra_hysteresis_check':'Positive affordable proposed military-share delta below .001, current raw-target or effective loyalty below .35; separately whether public proposed-floor target reaches .35',
 'limitations':['No internal pre-coup checkpoint; pre/post snapshots bracket whole monthly tick','No internal paid-command receipt; observed appropriation and public quote are not proof of exact within-tick PC charge','Extrema selected by declared deterministic rank; counts overlap','Private mandate read helpers copied only for explanation; no copied simulation transitions','No parameter sweep, seed search, target change or reserved holdout'],
 'build_requirement':'Root must link against fresh verified iteration24 rlib and record rlib/dependency/executable hashes before execution; no Cargo from this agent',
 'source_file':out.name,'source_sha256':sha(out.read_bytes()),
 'git_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo).decode().strip(),
 'input_scope':'All tracked spheres-sim files and root Cargo lock/config plus attributes; final root workspace receipt supplies broader compiled provenance',
 'inputs':inputs}
(here/'iteration-24-seed0-trace-plan.json').write_text(json.dumps(plan,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'source_bytes':out.stat().st_size,'source_sha256':plan['source_sha256'],'inputs':len(inputs),'status':plan['status']}))
