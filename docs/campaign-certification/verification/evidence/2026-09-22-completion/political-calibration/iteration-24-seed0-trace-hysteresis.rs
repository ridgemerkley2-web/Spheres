#![recursion_limit = "256"]
use spheres_sim::{init::world_1990, world::{GameRules, WorldState}, government::{self, Pillar, state, polity_in, bloc_of, is_electoral}, blocs, tick_month, nations::NationId};
use serde_json::{json, Value};
use std::collections::BTreeMap;

// Read-only copy of the current national-mass accounting, retained separately
// from its eligibility guard so absent historical eligibility stays visible.
fn reproduced_attested_parties(w: &WorldState, id: NationId) -> Option<&'static [String]> {
    if !w.rules.ideology_blocs || !government::is_electoral(w,id) { return None; }
    let g = government::state(w,id)?;
    if g.elected || g.awaiting_first_election || !g.banned.is_empty() { return None; }
    let marker = g.opening_mandate.as_ref()?;
    let row = spheres_sim::opening_mandates::sources().iter().find(|r| r.nation == id && r.key == marker.source_key)?;
    if marker.ballot_on != row.ballot_on || marker.governing_parties != row.governing_parties { return None; }
    let leader = g.leader()?;
    if !row.governing_parties.iter().any(|p| p == leader) { return None; }
    let office = spheres_sim::blocs::leader_row(w,id)?;
    let current_office = office.emergent.as_ref().map_or(office.office.as_str(), |e| e.office.as_str());
    if current_office != row.opening_office || office.tie_now() != Some(spheres_sim::data::Tie::Party(leader.to_string())) { return None; }
    Some(&row.governing_parties)
}


fn reproduced_civilian_public_mandate(w: &WorldState, id: NationId) -> f64 {
    if !w.rules.ideology_blocs || !is_electoral(w, id) { return 0.0; }
    let Some(g) = state(w, id) else { return 0.0; };
    let modeled_ballot = g.elected && g.unrestricted_mandate;
    let opening_parties = if modeled_ballot { None } else { reproduced_attested_parties(w, id) };
    if g.awaiting_first_election || !g.banned.is_empty() || (!modeled_ballot && opening_parties.is_none()) {
        return 0.0;
    }
    let Some(pol) = polity_in(w, id) else { return 0.0; };
    let mut mandate = 0.0;
    for (bloc, national) in spheres_sim::blocs::bloc_shares(w, id) {
        if !national.is_finite() { return 0.0; }
        let mut represented = 0.0;
        let mut governing = 0.0;
        for (party, share) in &g.support {
            if !share.is_finite() || *share < 0.0 { return 0.0; }
            if bloc_of(id, party) != bloc { continue; }
            represented += share;
            if g.in_government(party) && g.seat_share(party) > 0.0
                && (modeled_ballot || opening_parties.is_some_and(|parties| parties.contains(party)))
                && pol.parties.iter().any(|p| p.id == party)
            {
                governing += share;
            }
        }
        if represented > 0.0 {
            mandate += national.clamp(0.0, 1.0) * (governing / represented).clamp(0.0, 1.0);
        }
    }
    mandate.clamp(0.0, 1.0)
}


fn consent_proxy(w: &WorldState, id: NationId) -> (f64, f64) {
    let Some(g) = government::state(w,id) else {return (0.0,0.0)};
    let Some(pol) = government::polity_in(w,id) else {return (0.0,0.0)};
    let mut national_consent=0.0;
    for (bloc,national) in blocs::bloc_shares(w,id) {
        let mut represented=0.0; let mut governing=0.0;
        for (party,share) in &g.support {
            if government::bloc_of(id,party)!=bloc {continue;}
            represented+=share;
            if g.in_government(party)&&g.seat_share(party)>0.0&&pol.parties.iter().any(|p|p.id==party) {governing+=share;}
        }
        if represented>0.0 {national_consent+=national.clamp(0.0,1.0)*(governing/represented).clamp(0.0,1.0);}
    }
    let actual_eligible = reproduced_civilian_public_mandate(w,id);
    (national_consent.clamp(0.0,1.0),actual_eligible)
}
fn snap(w:&WorldState,id:NationId)->Value {
    let n=w.nation(id); let g=government::state(w,id).unwrap();
    let (national_consent,eligible_consent)=consent_proxy(w,id);
    let resources=government::army_resources_per_member(w,id);
    let personnel=government::army_personnel_assessment(w,id).map(|a|a.members);
    let income=n.gdp*1000.0/n.population;
    let useful_resources=2.0*government::army_operating_allowance(income);
    let penalty=government::army_civilian_confidence_penalty(w,id);
    let material=Some(resources.map_or_else(
        ||government::army_loyalty_target(n.mil_spend_gdp,n.war_exhaustion,None,government::ARMY_PAY_WEIGHT),
        |r|government::army_resource_loyalty_target(r,income,n.war_exhaustion)));
    let target=material.map(|r|(r-penalty).clamp(0.0,1.0));
    let needed_fraction=((0.40-0.20+n.war_exhaustion*0.45+penalty)/0.65).clamp(0.0,1.0);
    let full_basket_share=personnel.map(|p|useful_resources*p/(n.gdp*1e9));
    let useful_needed_share=full_basket_share.map(|s|s*needed_fraction);
    let fiscal=spheres_sim::economy::Fiscal::of(n,&spheres_sim::economy::growth_terms(n,n.state_invest_gdp,n.interest_rate,&spheres_sim::economy::Conditions::of(w,id)));
    let affordable=(fiscal.balance_gdp+n.mil_spend_gdp).max(0.0);
    let floor=government::ai_army_funding_floor(w,id);
    let command_affordable=floor.map(|share|spheres_sim::affordable(w,&spheres_sim::Command::SetMilSpend{nation:id,share}));
    let known_leverage=spheres_sim::army_authority::current_leverage(w,id);
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
    let raw_coalition_support:f64=g.support.iter().filter(|(p,_)|g.in_government(p)).map(|(_,v)|v).sum();
    json!({"date":[w.year,w.month,w.day],"country":format!("{id:?}"),"country_name":id.name(),
      "alive":n.alive,"electoral":government::is_electoral(w,id),"elected":g.elected,"unrestricted_mandate":g.unrestricted_mandate,
      "awaiting_first_election":g.awaiting_first_election,"office_months":g.months_in_office,"next_election":g.next_election,
      "record":g.political_record.as_ref().map(|r|json!({"government":r.government,"months":r.months,"performance":r.performance})),
      "holder":format!("{:?}",blocs::leader_row(w,id).and_then(|r|r.tie_now())),"coalition":g.coalition,"banned":g.banned,
      "coalition_support_conditional":raw_coalition_support,"national_coalition_support_proxy":national_consent,"eligible_public_mandate_proxy":eligible_consent,
      "has_army":has_army,"known_current_leverage":known_leverage,
      "political_channel_leverage":political_leverage,"leverage_basis":if known_leverage.is_some(){"saved_assessment"}else{"unknown_authoritarianism_proxy"},
      "record_matches":record_matches,"record_established":record_established,"sustained_crisis_component":sustained_crisis,
      "proposed_funding_delta":floor_delta,"proposed_funding_price":price,"target_at_proposed_floor":target_at_floor,
      "below_hysteresis_affordable_hostility":below_hysteresis,
      "below_hysteresis_would_reach_coup_line":below_hysteresis&&target_at_floor.is_some_and(|t|t>=government::ELECTORAL_COUP_ARMY),
      "explicit_annual_plan":n.annual_budget.is_some(),"program_owner":spheres_sim::programs::enrolled(w,id),
      "opening_mandate":g.opening_mandate,"army_authority":g.army_authority,"settled_months":government::electoral_coup_settled_months(w,id),"political_only_full_provisions_target":0.85-penalty,"government_seats":g.government_seats(),"authoritarianism":n.authoritarianism,"discontent":blocs::discontent(w,id),
      "loyalty":g.loyalty(Pillar::Army),"effective_loyalty":blocs::effective_army_loyalty(w,id),"coup_pressure":g.coup_pressure,
      "confidence_penalty":penalty,"material_target_before_confidence":material,"actual_resource_target":target,"war_exhaustion":n.war_exhaustion,
      "gdp_bn":n.gdp,"population_m":n.population,"gdp_per_head":income,"military_share":n.mil_spend_gdp,"fiscal_balance_share":fiscal.balance_gdp,
      "fiscally_affordable_share":affordable,"army_funding_floor":floor,"funding_command_affordable":command_affordable,"political_capital":n.political_capital,
      "resources_per_member":resources,"useful_resources_per_member_cap":useful_resources,"resource_saturated":resources.map(|r|r>=useful_resources-1e-7),
      "full_basket_military_share":full_basket_share,"useful_needed_military_share":useful_needed_share,"needed_resource_fraction":needed_fraction,
      "needed_unaffordable":useful_needed_share.map(|s|s>affordable+1e-7),"full_basket_cannot_reach_point40":0.85-n.war_exhaustion*0.45-penalty<0.40,
      "personnel_estimate_or_source":format!("{:?}",government::army_personnel_assessment(w,id)),"explicit_budget":n.on_the_books()})
}

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
            if electoral_army(&row)&&flag(&row,"below_hysteresis_affordable_hostility") {
                emit(json!({"kind":"below_hysteresis_snapshot","seed":SEED,"month":month,"snapshot":row}));
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
