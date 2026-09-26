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
    let material=resources.map(|r|government::army_resource_loyalty_target(r,income,n.war_exhaustion));
    let target=material.map(|r|(r-penalty).clamp(0.0,1.0));
    let needed_fraction=((0.40-0.20+n.war_exhaustion*0.45+penalty)/0.65).clamp(0.0,1.0);
    let full_basket_share=personnel.map(|p|useful_resources*p/(n.gdp*1e9));
    let useful_needed_share=full_basket_share.map(|s|s*needed_fraction);
    let fiscal=spheres_sim::economy::Fiscal::of(n,&spheres_sim::economy::growth_terms(n,n.state_invest_gdp,n.interest_rate,&spheres_sim::economy::Conditions::of(w,id)));
    let affordable=(fiscal.balance_gdp+n.mil_spend_gdp).max(0.0);
    let floor=government::ai_army_funding_floor(w,id);
    let command_affordable=floor.map(|share|spheres_sim::affordable(w,&spheres_sim::Command::SetMilSpend{nation:id,share}));
    let raw_coalition_support:f64=g.support.iter().filter(|(p,_)|g.in_government(p)).map(|(_,v)|v).sum();
    json!({"date":[w.year,w.month,w.day],"country":format!("{id:?}"),"country_name":id.name(),
      "alive":n.alive,"electoral":government::is_electoral(w,id),"elected":g.elected,"unrestricted_mandate":g.unrestricted_mandate,
      "awaiting_first_election":g.awaiting_first_election,"office_months":g.months_in_office,"next_election":g.next_election,
      "record":g.political_record.as_ref().map(|r|json!({"government":r.government,"months":r.months,"performance":r.performance})),
      "holder":format!("{:?}",blocs::leader_row(w,id).and_then(|r|r.tie_now())),"coalition":g.coalition,"banned":g.banned,
      "coalition_support_conditional":raw_coalition_support,"national_coalition_support_proxy":national_consent,"eligible_public_mandate_proxy":eligible_consent,
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
#[derive(Default)] struct Summary {electoral_months:u32,inherited_months:u32,completed_months:u32,interim_months:u32,crisis_months:u32,positive_penalty_months:u32,saturated_penalty_months:u32,unaffordable_needed_months:u32,inherited_coups:u32,completed_coups:u32,interim_coups:u32,first_coups:u32,repeat_coups:u32,funding_increases:u32,max_mil:f64,max_penalty:f64,source_high_leverage_months:u32,source_high_leverage_crisis_months:u32,closest_pre_first:Option<Value>}
fn val(v:&Value,key:&str)->f64{v[key].as_f64().unwrap_or(0.0)}
fn flag(v:&Value,key:&str)->bool{v[key].as_bool().unwrap_or(false)}
fn main(){
    let args:Vec<String>=std::env::args().collect(); let from=args.get(1).and_then(|s|s.parse().ok()).unwrap_or(0u64); let to=args.get(2).and_then(|s|s.parse().ok()).unwrap_or(12u64);
    println!("{}",json!({"kind":"contract","schema":1,"seeds":[from,to],"months":252,"confidence_weight":government::ARMY_CRISIS_CONFIDENCE_WEIGHT,
       "pre_state_scope":"start of monthly tick; economy/drift may change before the coup is settled","funding_scope":"observed end-to-end monthly appropriation changes, not a private AI command trace","proxy_scope":"national mass accounting with exact frozen20 civilian_public_mandate function reproduced, including attested opening mandate; raw source copied and hashed","commands":[],"switches":{"ideology_blocs":true,"ideology_takeover":true},"other_rules":"GameRules::default"}));
    for seed in from..to {
        let mut w=world_1990(GameRules{seed,ideology_blocs:true,ideology_takeover:true,..GameRules::default()});
        let mut sums:BTreeMap<NationId,Summary>=BTreeMap::new(); let mut couped:BTreeMap<NationId,u32>=BTreeMap::new();
        for month in 0..252 {
            let mut before=BTreeMap::new();
            for n in w.nations.iter().filter(|n|n.alive) {
                let Some(g)=government::state(&w,n.id) else{continue};
                if !g.pillars.iter().any(|(p,_)|*p==Pillar::Army) {continue;}
                let row=snap(&w,n.id); let t=sums.entry(n.id).or_default();
                if flag(&row,"electoral") {
                    t.electoral_months+=1;
                    if flag(&row,"awaiting_first_election"){t.interim_months+=1;}else if flag(&row,"elected"){t.completed_months+=1;}else{t.inherited_months+=1;}
                    if val(&row,"discontent")>=government::ELECTORAL_COUP_DISCONTENT {t.crisis_months+=1;}
                    if val(&row,"confidence_penalty")>0.0 {t.positive_penalty_months+=1; if flag(&row,"resource_saturated"){t.saturated_penalty_months+=1;}}
                    if flag(&row,"needed_unaffordable"){t.unaffordable_needed_months+=1;}
                }
                t.max_mil=t.max_mil.max(n.mil_spend_gdp);t.max_penalty=t.max_penalty.max(val(&row,"confidence_penalty"));
                let source_high=g.army_authority.as_ref().is_some_and(|a|a.source.assessment>=0.5);
                if source_high&&flag(&row,"electoral") {
                    t.source_high_leverage_months+=1;
                    if val(&row,"discontent")>=government::ELECTORAL_COUP_DISCONTENT {t.source_high_leverage_crisis_months+=1;}
                }
                if flag(&row,"electoral")&&couped.get(&n.id).copied().unwrap_or(0)==0 {
                    let rank=|r:&Value| (val(r,"discontent")>=government::ELECTORAL_COUP_DISCONTENT&&val(r,"settled_months")>=12.0&&!flag(r,"awaiting_first_election"),val(r,"coup_pressure"),-val(r,"effective_loyalty"),val(r,"confidence_penalty"));
                    if t.closest_pre_first.as_ref().is_none_or(|old|rank(&row)>rank(old)) {t.closest_pre_first=Some(row.clone());}
                }
                before.insert(n.id,row);
            }
            let news=tick_month(&mut w,&[]);
            for (&id,old) in &before {
                let Some(n)=w.nation_opt(id).filter(|n|n.alive) else{continue};
                if n.mil_spend_gdp>val(old,"military_share")+0.000999999 {
                    sums.entry(id).or_default().funding_increases+=1;
                    // All meaningful increases retained, including countries with no coup.
                    println!("{}",json!({"kind":"funding_increase","seed":seed,"month":month+1,"country":format!("{id:?}"),"before":old,"after":snap(&w,id)}));
                }
            }
            for headline in news.iter().filter(|h|h.starts_with("COUP IN ")&&h.contains("elected government")) {
                let id=w.nations.iter().map(|n|n.id).find(|id|headline.starts_with(&format!("COUP IN {}:",id.name().to_uppercase()))).expect("coup headline has a country");
                let old=before.get(&id).expect("coup has an Army pre-state");let seen=couped.entry(id).or_default();*seen+=1;
                let t=sums.entry(id).or_default();if *seen==1{t.first_coups+=1}else{t.repeat_coups+=1};
                let category=if flag(old,"awaiting_first_election"){t.interim_coups+=1;"awaiting_first_election"}else if flag(old,"elected"){t.completed_coups+=1;"completed_model_ballot"}else{t.inherited_coups+=1;"no_completed_model_ballot"};
                println!("{}",json!({"kind":"electoral_coup","seed":seed,"month":month+1,"country":format!("{id:?}"),"ordinal_in_country":seen,"first":*seen==1,"mandate_category":category,"headline":headline,"before":old,"after":snap(&w,id)}));
            }
        }
        for (id,t) in sums {println!("{}",json!({"kind":"country_summary","seed":seed,"country":format!("{id:?}"),"electoral_months":t.electoral_months,"inherited_months":t.inherited_months,"completed_months":t.completed_months,"interim_months":t.interim_months,"crisis_months":t.crisis_months,"positive_penalty_months":t.positive_penalty_months,"saturated_penalty_months":t.saturated_penalty_months,"unaffordable_needed_months":t.unaffordable_needed_months,"first_coups":t.first_coups,"repeat_coups":t.repeat_coups,"inherited_coups":t.inherited_coups,"completed_coups":t.completed_coups,"interim_coups":t.interim_coups,"funding_increases":t.funding_increases,"max_military_share":t.max_mil,"max_confidence_penalty":t.max_penalty,"source_high_leverage_months":t.source_high_leverage_months,"source_high_leverage_crisis_months":t.source_high_leverage_crisis_months,"closest_pre_first":t.closest_pre_first}));}
        println!("{}",json!({"kind":"seed_complete","seed":seed,"state_hash":spheres_sim::state_hash(&w),"rng_state":w.rng.state}));
    }
}

