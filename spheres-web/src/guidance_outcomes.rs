//! Dated campaign results for guidance, copied from saved native records under
//! the guidance lock. Nothing here dispatches, charges, settles or moves the
//! clock. A record that does not exist is served as null, never as zero or done;
//! a section is null when its system is not enabled for this campaign.
use serde_json::{json, Value};
use spheres_sim::{airbases, airmissions::MissionStatus, clock, districts, equipment, fiscal_recovery, industrial_modules, industry, production,
    production::{ProjectKind, PROJECT_KINDS}, resources::{self, Commodity},
    world::{days_in_month, NationId, WorldState}};

const COMPLETIONS: usize = 20;
const MISSIONS: usize = 10;

/// Log dates are `WorldState::date_str` text. Anything that does not round-trip
/// exactly stays undated rather than guessed.
fn log_day(date:&str)->Option<i32> {
    let mut parts=date.split(' ');
    let (d,m,y)=(parts.next()?,parts.next()?,parts.next()?);
    if parts.next().is_some() {return None;}
    let (day,year)=(d.parse::<u32>().ok()?,y.parse::<i32>().ok()?);
    let month=crate::MONTH_NAMES.iter().position(|name|*name==m)? as u32+1;
    (day>=1&&day<=days_in_month(year,month)&&format!("{day} {m} {year}")==date).then(||clock::date_day(year,month,day))
}

/// Only the construction headlines written for this nation: production.rs's two
/// (`completes <catalog name> level N in <id>.` and the Starter Industry module)
/// and resources.rs's mine opening (`opens the <commodity> mine|field in <name>.`).
/// Returns the site as (district id, ProjectKind key, or "mine"); either is None
/// when the text does not map to exactly one.
fn construction_headline(text:&str,me:NationId,w:&WorldState)->Option<(Option<String>,Option<&'static str>)> {
    let rest=text.strip_prefix(me.name())?.strip_suffix('.')?;
    if let Some((what,name))=rest.strip_prefix(" opens the ").and_then(|t|t.rsplit_once(" in ")) {
        let c=*resources::ALL.iter().find(|c|what==format!("{} {}",c.name(),if matches!(c,Commodity::Oil|Commodity::Gas){"field"}else{"mine"}))?;
        // The text carries the display name, which several districts can share;
        // the mine record written with the headline breaks a tie, or it stays null.
        let ids:Vec<&String>=w.districts.keys().filter(|d|districts::name_of(d)==Some(name)).collect();
        if ids.is_empty() {return None;}
        let built:Vec<&String>=ids.iter().copied().filter(|d|resources::mine_at(w,d,c).is_some()).collect();
        let district=if ids.len()==1 {Some(ids[0])} else if built.len()==1 {Some(built[0])} else {None};
        return Some((district.cloned(),Some("mine")));
    }
    let (what,district)=rest.strip_prefix(" completes ")?.rsplit_once(" in ")?;
    districts::name_of(district)?;
    let kind=match what.strip_prefix("a ").and_then(|t|t.strip_suffix("% Starter Industry module")) {
        Some(share)=>{share.parse::<f64>().ok().filter(|s|s.is_finite()&&*s>=0.0)?;Some(ProjectKind::StarterIndustry)}
        None=>{let (name,level)=what.rsplit_once(" level ")?;level.parse::<u8>().ok()?;
            PROJECT_KINDS.into_iter().find(|k|*k!=ProjectKind::StarterIndustry&&production::catalog(*k).name==name)}
    };
    Some((Some(district.to_string()),kind.map(ProjectKind::key)))
}

fn money(w:&WorldState,me:NationId)->Value {
    let fiscal=w.fiscal_recovery.nations.get(&me);
    // The journal opens at the first recorded payment or decision, so an enabled
    // book that has not opened yet has recorded no decision.
    let decisions=match fiscal.and_then(|f|f.money_journal.as_ref()) {
        Some(journal)=>{let mut rows:Vec<_>=journal.decisions.iter().collect();rows.sort_by_key(|d|d.id);
            Some(rows.into_iter().map(|d|json!({"id":d.id,"day":d.day,"kind":d.kind})).collect::<Vec<_>>())},
        None=>(fiscal_recovery::enabled(w)&&fiscal.is_some()).then(Vec::new),
    };
    let program=w.nation_opt(me).map(|n|{let p=n.program_budget.as_ref();
        json!({"enabled":p.is_some(),"fiscal_year":p.map(|p|p.fiscal_year),"settled_day":p.and_then(|p|p.settled_day)})});
    json!({"journal_available":decisions.is_some(),"decisions":decisions,"program":program})
}

fn construction(g:&crate::Game,me:NationId)->Value {
    let w=&g.world;
    if !w.rules.production_system {return Value::Null;}
    let mut projects:Vec<_>=production::projects_for(w,me).collect();
    projects.sort_by_key(|p|p.id);
    let projects:Vec<Value>=projects.into_iter().map(|p|{
        let f=w.production.industry.projects.get(&p.id);
        let scale=industrial_modules::scale(p);
        json!({"id":p.id,"kind":p.kind.key(),"district":p.district,
            "spent_bn":f.map(|f|f.spent_bn),"contract_cost_bn":f.and_then(|f|f.contract_cost_bn),
            "last_day":f.and_then(|f|f.last_day),"last_spent_bn":f.and_then(|f|f.last_spent_bn),
            "progress_days":p.progress_days*scale,
            "total_days":if p.capacity_micros.is_some(){json!(p.total_days as f64*scale)}else{json!(p.total_days)}})
    }).collect();
    let mut completions:Vec<Value>=g.log.iter().rev()
        .filter_map(|e|e.tags.contains(&me).then(||construction_headline(&e.text,me,w)).flatten().map(|site|(e,site)))
        .take(COMPLETIONS).map(|(e,(district,kind))|json!({"date":e.date,"day":log_day(&e.date),"text":e.text,
            "district":district,"kind":kind})).collect();
    completions.reverse();
    // Mine work is paid daily from the same construction budget. A legacy
    // prepaid row has no funding entry, so its payment fields stay null.
    let mut mines:Vec<_>=w.resources.mine_projects.iter().filter(|p|p.started_by==me).collect();
    mines.sort_by(|a,b|(a.district.as_str(),a.commodity).cmp(&(b.district.as_str(),b.commodity)));
    let mines:Vec<Value>=mines.into_iter().map(|p|{let f=w.production.industry.mines.get(&industry::mine_key(&p.district,p.commodity));
        json!({"district":p.district,"commodity":p.commodity.key(),"spent_bn":f.map(|f|f.spent_bn),"last_day":f.and_then(|f|f.last_day),
            "progress_days":f.map(|f|f.progress_days),"total_days":f.map(|f|f.total_days)})}).collect();
    let operating:Vec<Value>=industry::snapshot(w,me).sites.iter().filter_map(|s|{
        let o=s.operation.as_ref().filter(|o|o.nation==me)?;
        (s.output_daily.is_finite()&&s.output_daily>0.0).then(||
            json!({"district":s.district,"kind":s.kind.key(),"day":o.day,"output_daily":s.output_daily}))
    }).collect();
    json!({"projects":projects,"completions":completions,"operating":operating,"mines":mines})
}

fn research(w:&WorldState,me:NationId)->Value {
    if !clock::is_daily(w)||!w.rules.military_operations {return Value::Null;}
    let Some(n)=w.nation_opt(me) else {return Value::Null};
    // Every equipment record is created through this state, so none yet is empty.
    let empty=equipment::EquipmentState::default();
    let s=n.equipment.as_ref().unwrap_or(&empty);
    let mut projects:Vec<_>=s.projects.iter().filter(|p|p.kind==equipment::ProjectKind::Development).collect();
    projects.sort_by_key(|p|p.id);
    json!({
        "active":s.active_research.as_ref().map(|id|json!({"component":id,"progress":s.research_progress,
            "cost":equipment::research(id).map(|r|r.points)})),
        "learned":s.learned.len(),"last_completed_day":s.last_research_completed_day,
        // The designer's own check, against what this nation knows today.
        "drafts":s.drafts.values().map(|d|json!({"name":d.name,"updated_day":d.updated_day,
            "valid":equipment::design_preview(w,me,&d.spec).valid})).collect::<Vec<_>>(),
        "revisions":s.revisions.values().map(|r|json!({"id":r.id,"platform":r.spec.platform,"created_day":r.created_day,
            "certified_day":r.certified_day,"imported":r.id.starts_with("import-")})).collect::<Vec<_>>(),
        "projects":projects.into_iter().map(|p|json!({"id":p.id,"revision":p.revision_id,"started_day":p.started_day,
            "completed_day":p.completed_day,"last_day":p.last_day,"last_spent_bn":p.last_spent_bn,"spent_bn":p.spent_bn})).collect::<Vec<_>>(),
    })
}

fn procurement(w:&WorldState,me:NationId)->Value {
    if !clock::is_daily(w)||!w.rules.military_operations||!w.rules.resource_market||!w.rules.manufacturing_system {return Value::Null;}
    let c=&w.companies;
    let products:Vec<_>=c.firms.iter().filter(|f|f.nation==me).flat_map(|f|&f.products).filter(|p|p.cancelled_day.is_none()).collect();
    let mut deliveries:Vec<_>=c.deliveries.iter().filter(|d|d.buyer==me).collect();
    deliveries.sort_by_key(|d|d.id);
    let mut imports:Vec<_>=c.imports.contracts.iter().filter(|d|d.buyer==me).collect();
    imports.sort_by_key(|d|d.id);
    json!({
        "companies":c.firms.iter().filter(|f|f.nation==me).count(),
        "certified_products":products.iter().filter(|p|p.certified_day.is_some()).count(),
        "developing_products":products.iter().filter(|p|p.certified_day.is_none()).count(),
        "deliveries":deliveries.into_iter().map(|d|json!({"id":d.id,
            "company":c.firms.iter().find(|f|f.id==d.company).map(|f|f.name.as_str()),"revision":d.revision_id,
            "quantity":d.quantity,"purchased_day":d.purchased_day,"settled_day":d.settled_day,"due_day":d.due_day,
            "delivered_day":d.delivered_day,"status":d.status})).collect::<Vec<_>>(),
        "imports":imports.into_iter().map(|d|json!({"id":d.id,"seller":d.seller,"revision":d.buyer_revision,
            "ammunition":d.ammunition,"quantity":d.quantity,"purchased_day":d.purchased_day,"settled_day":d.settled_day,
            "due_day":d.due_day,"delivered_day":d.delivered_day,"cancelled_day":d.cancelled_day,"status":d.status})).collect::<Vec<_>>(),
    })
}

fn aviation(w:&WorldState,me:NationId)->Value {
    if !clock::is_daily(w) {return Value::Null;}
    let mut bases:Vec<_>=w.airbases.iter().flat_map(|s|&s.bases).collect();
    bases.sort_by(|a,b|a.id.cmp(&b.id));
    let bases:Vec<Value>=bases.into_iter().filter_map(|b|{
        let project=b.project.as_ref().filter(|p|p.sponsor==me);
        let mut history:Vec<_>=b.history.iter().filter(|p|p.sponsor==me).collect();
        history.sort_by_key(|p|p.id);
        (b.sponsor==me||project.is_some()||!history.is_empty()).then(||json!({"id":b.id,"name":b.name,
            "project":project.map(|p|json!({"track":p.track.key(),"target_level":p.target_level,"started_day":p.started_day,
                "last_paid_day":p.last_paid_day,"paid_bn":p.paid_bn,"total_cost_bn":p.total_cost_bn,
                "completed_day":p.completed_day,"cancelled_day":p.cancelled_day})),
            "history":history.into_iter().map(|p|json!({"track":p.track.key(),"target_level":p.target_level,
                "started_day":p.started_day,"completed_day":p.completed_day})).collect::<Vec<_>>()}))
    }).collect();
    let mut squadrons:Vec<_>=w.nation_opt(me).and_then(|n|n.aviation.as_ref()).map_or(vec![],|a|a.squadrons.iter().collect());
    squadrons.sort_by_key(|s|s.id);
    let squadrons:Vec<Value>=squadrons.into_iter().map(|sq|{let blocker=airbases::squadron_blocker(w,me,sq);
        json!({"id":sq.id,"assigned":sq.assigned,"ready":blocker.is_none(),"blocker":blocker})}).collect();
    let mut orders:Vec<_>=w.air_missions.iter().flat_map(|s|&s.orders).filter(|o|o.nation==me).collect();
    orders.sort_by_key(|o|o.id);
    // The newest orders, plus the first flight when it has scrolled out of them,
    // so the dated first mission is never cut to look unflown (at most 11 rows).
    let start=orders.len().saturating_sub(MISSIONS);
    let first_flown=orders.iter().position(|o|o.status==MissionStatus::Flown).filter(|&i|i<start);
    let missions:Vec<Value>=first_flown.into_iter().chain(start..orders.len()).map(|i|orders[i]).map(|o|json!({"id":o.id,"status":o.status,
        "report_day":o.report.as_ref().map(|r|r.day),"aircraft":o.report.as_ref().map(|r|r.aircraft)})).collect();
    json!({"bases":bases,"squadrons":squadrons,"missions":missions})
}

/// `as_of_day` is the absolute day of the date `state` serves in the same read.
pub(crate) fn outcomes_json(g:&crate::Game,me:NationId)->Value {
    let w=&g.world;
    json!({"nation":me,"date":w.date_str(),"as_of_day":clock::absolute_day(w),
        "money":money(w,me),"construction":construction(g,me),"research":research(w,me),
        "procurement":procurement(w,me),"aviation":aviation(w,me)})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guidance_log_dates_parse_only_the_exact_served_format() {
        assert_eq!(log_day("1 Jan 1990"),Some(0));
        assert_eq!(log_day("31 Dec 1990"),Some(364));
        assert_eq!(log_day("29 Feb 1992"),Some(clock::date_day(1992,2,29)));
        for bad in ["","01 Jan 1990","+1 Jan 1990","0 Jan 1990","32 Jan 1990","29 Feb 1991","1 January 1990","1 Jan 1990 ","Jan 1 1990","1  Jan 1990"] {
            assert_eq!(log_day(bad),None,"{bad:?} must stay undated");
        }
    }

    #[test]
    fn guidance_completion_headlines_match_only_this_nations_construction() {
        let me=NationId::France;
        let mut w=crate::Game::new(1,Some(me)).world;
        let (twin,other_twin,c)=w.districts.keys().filter_map(|a|{let n=districts::name_of(a)?;
            let b=w.districts.keys().find(|b|*b!=a&&districts::name_of(b)==Some(n))?;
            let c=resources::ALL.into_iter().find(|c|resources::mine_at(&w,a,*c).is_none()&&resources::mine_at(&w,b,*c).is_none())?;
            Some((a.clone(),b.clone(),c))}).next().expect("the census shares some district names");
        let twin_name=districts::name_of(&twin).unwrap();
        let opened=format!("France opens the {} {} in {twin_name}.",c.name(),if matches!(c,Commodity::Oil|Commodity::Gas){"field"}else{"mine"});
        assert_eq!(construction_headline(&opened,me,&w),Some((None,Some("mine"))),"a shared name alone names no site");
        let at=w.resources.mines.binary_search_by(|m|(m.district.as_str(),m.commodity).cmp(&(twin.as_str(),c))).unwrap_err();
        w.resources.mines.insert(at,resources::Mine{district:twin.clone(),commodity:c,output:1.0,completed:0});
        assert_eq!(construction_headline(&opened,me,&w),Some((Some(twin.clone()),Some("mine"))),"the opened mine breaks the tie");
        assert!(resources::mine_at(&w,&other_twin,c).is_none());
        let named=|d:&String|districts::name_of(d).filter(|n|w.districts.keys().filter(|o|districts::name_of(o)==Some(n)).count()==1);
        let district=w.districts.iter().filter(|(_,o)|**o==me).map(|(d,_)|d).find(|d|named(d).is_some()).unwrap().clone();
        let name=districts::name_of(&district).unwrap();
        let site=|kind:&'static str|Some((Some(district.clone()),Some(kind)));
        let read=|text:String|construction_headline(&text,me,&w);
        assert_eq!(read(format!("France completes Materials Processing level 2 in {district}.")),site("processing_plant"));
        assert_eq!(read(format!("France completes Power Grid level 1 in {district}.")),site("power_grid"));
        assert_eq!(read(format!("France completes a 0.5000% Starter Industry module in {district}.")),site("starter_industry"));
        for kind in PROJECT_KINDS.into_iter().filter(|k|*k!=ProjectKind::StarterIndustry) {
            assert_eq!(read(format!("France completes {} level 3 in {district}.",production::catalog(kind).name)),site(kind.key()));
        }
        assert_eq!(read(format!("France completes Orbital Works level 1 in {district}.")),Some((Some(district.clone()),None)),
            "an unmapped facility is still this nation's completion, but at no known site");
        assert_eq!(read(format!("France opens the iron mine in {name}.")),site("mine"));
        assert_eq!(read(format!("France opens the platinum group mine in {name}.")),site("mine"));
        assert_eq!(read(format!("France opens the oil field in {name}.")),site("mine"));
        for other in [format!("Germany completes Materials Processing level 2 in {district}."),
            "France completes its domination agenda: influence in Europe.".into(),
            format!("France completes Materials Processing level 2 in {district}"),
            format!("France completes Materials Processing level two in {district}."),
            format!("France completes a lot% Starter Industry module in {district}."),
            "France completes Materials Processing level 2 in Nowhere.".into(),
            format!("Germany opens the iron mine in {name}."),
            format!("France opens the oil mine in {name}."),
            format!("France opens the iron field in {name}."),
            format!("France opens the Iron mine in {name}."),
            format!("France opens the iron mine in {name}"),
            "France opens the iron mine in Nowhere.".into()] {
            assert_eq!(read(other.clone()),None,"{other}");
        }
    }
}
