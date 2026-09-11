//! Country company directory and pure, current-state contract reviews.
use serde_json::{json, Value};
use spheres_sim::{sector_contractors, world::{NationId, WorldState}, Command};

pub fn snapshot(w: &WorldState, nation: NationId, session: &str) -> Value {
    if w.player != Some(nation) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return json!({"session_id":session,"nation":nation,"available":false,"directory":[]});
    }
    let mut view = spheres_sim::company_network::view(w, nation);
    view["session_id"] = json!(session);
    view["nation"] = json!(nation);
    view["name"] = json!(nation.name());
    view["date"] = json!(w.date_str());
    view["available"] = json!(true);
    view["suppliers"] = crate::equipment_view::company_directory_board(w, nation);
    if let Some(rows) = view["directory"].as_array_mut() {
        for row in rows {
            row["kind"] = json!(match row["role"].as_str().unwrap_or("") {
                "supplier" => "supplier", "sector_contractor" => "contractor", _ => "unknown",
            });
        }
    }
    // These receipts belong to service contractors. They are not a cash balance.
    if let Some(rows) = view["contractors"]["companies"].as_array_mut() {
        for row in rows {
            row["fee_basis"] = json!(match row["sector"].as_str().unwrap_or("") {
                "construction" => "Accepted service fees fit the existing daily construction funding ceiling. Paid base work is retained; no material pack or crew assignment is required.",
                _ => "Only eligible future work can earn the reviewed service fee. Funding and operating requirements still apply; existing supplier prices and reservations keep their terms.",
            });
        }
    }
    view
}

pub fn preview(w: &WorldState, nation: NationId, session: &str, payload: &Value) -> Result<Value,String> {
    if w.player != Some(nation) || !w.nation_opt(nation).is_some_and(|n|n.alive) {
        return Err("Choose a living country in this campaign first.".into());
    }
    let mut command = payload.get("command").filter(|v|v.is_object()).cloned()
        .ok_or("Choose a company action to review.")?;
    let kind = command["kind"].as_str().ok_or("Choose a company action.")?.to_string();
    let (title, note, quote) = match kind.as_str() {
        "enable_companies" => ("Adopt the company network", "Add modeled domestic service specialists and operating requirements for future supplier work. Existing supplier accounts, stock, deliveries and paid contracts retain their property. No historical company ownership, government cash or free inputs are granted. Save a separate campaign slot first if you want to retain its former rules.", None),
        "assign_sector_contractor" => {
            let company = command["company"].as_u64().and_then(|v|u32::try_from(v).ok()).ok_or("Choose a valid contractor.")?;
            let target = serde_json::from_value(command["target"].clone()).map_err(|_|"Choose a valid work assignment.")?;
            let q = serde_json::to_value(sector_contractors::assignment_quote(w,nation,company,&target)).map_err(|e|e.to_string())?;
            ("Review service assignment", "This changes eligible future work only. Public work retains its funding ceiling. Supplier stock, certified prices, development agreements and refit reservations are unchanged.", Some(q))
        },
        "unassign_sector_contractor" => {
            let target = serde_json::from_value(command["target"].clone()).map_err(|_|"Choose a valid work assignment.")?;
            let q = serde_json::to_value(sector_contractors::release_quote(w,nation,&target)).map_err(|e|e.to_string())?;
            ("Review ending this assignment", "End future service work on this assignment. Delivered work and earned service fees remain recorded; no paid work is refunded or repeated.", Some(q))
        },
        _ => return Err("This action belongs in the equipment procurement review.".into()),
    };
    if let Some(q) = &quote { command["quote"] = q["token"].clone(); }
    let parsed = crate::parse_command(w,&command,nation).ok_or("Invalid company action.")?;
    if !matches!(parsed,Command::EnableCompanies{..}|Command::AssignSectorContractor{..}|Command::UnassignSectorContractor{..}) {
        return Err("Invalid company action.".into());
    }
    let reason = quote.as_ref().filter(|q|q["valid"]==false).and_then(|q|q["reason"].as_str()).map(str::to_owned)
        .or_else(||spheres_sim::apply_command(&mut w.clone(),&parsed).err());
    Ok(json!({"session_id":session,"nation":nation,"valid":reason.is_none(),"reason":reason,
        "title":title,"note":note,"quote":quote,"command":command}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directory_fixture_records_paid_company_and_construction_service() {
        let me=NationId::France;
        let mut g=crate::Game::new(1990,Some(me));crate::play_rules(&mut g);
        // A stated synthetic starting plant/appropriation isolates UI behavior.
        // Formation, fees, payments and progress below use actual game commands.
        let district=g.world.districts.iter().find(|(_,owner)|**owner==me).unwrap().0.clone();
        if let Some(p)=g.world.production.provinces.iter_mut().find(|p|p.district==district) {p.arms_plants=1;}
        else {g.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities{district:district.clone(),arms_plants:1,infrastructure:0,civilian_industry:0,power_grid:0,research_centers:0});}
        spheres_sim::programs::set_construction_budget(&mut g.world,me,0.001).unwrap();
        g.world.nation_mut(me).program_budget.as_mut().unwrap().available_bn[spheres_sim::world::BUDGET_DEFENSE][3]=2.0;
        spheres_sim::connected_economy::enable(&mut g.world).unwrap();
        spheres_sim::resources::tick(&mut g.world);
        let adoption=preview(&g.world,me,&g.session_id,&json!({"command":{"kind":"enable_companies"}})).unwrap();
        assert_eq!(adoption["valid"],true);
        spheres_sim::apply_command(&mut g.world,&Command::EnableCompanies{nation:me}).unwrap();
        let formation=crate::equipment_view::preview(&g.world,me,&g.session_id,&json!({"command":{
            "kind":"company_establish","name":"Scenario Arsenal","district":district,"capital_mn":25.0}})).unwrap();
        assert_eq!(formation["valid"],true,"{formation}");
        let command=crate::parse_command(&g.world,&formation["actions"][0]["command"],me).unwrap();
        spheres_sim::apply_command(&mut g.world,&command).unwrap();
        let project=spheres_sim::production::start_project(&mut g.world,me,&district,spheres_sim::production::ProjectKind::Infrastructure).unwrap();
        let company=g.world.sector_contractors.roster.iter().find(|c|c.nation==me&&c.sector==sector_contractors::CompanySector::Construction).unwrap().id;
        let action_review=preview(&g.world,me,&g.session_id,&json!({"command":{"kind":"assign_sector_contractor","company":company,"target":{"kind":"construction","project":project}}})).unwrap();
        assert_eq!(action_review["valid"],true,"{action_review}");
        let command=crate::parse_command(&g.world,&action_review["command"],me).unwrap();
        spheres_sim::apply_command(&mut g.world,&command).unwrap();
        g.advance_days(1,vec![]);
        let supplier=g.world.companies.firms.iter().find(|c|c.nation==me).unwrap();
        assert_eq!(supplier.capital_received_bn,0.025);
        assert!(g.world.production.industry.projects[&project].company_fees_bn>0.0);
        let before=spheres_sim::save(&g.world);
        let directory=snapshot(&g.world,me,&g.session_id);
        assert!(directory["directory"].as_array().unwrap().iter().any(|r|r["kind"]=="supplier"));
        assert!(directory["directory"].as_array().unwrap().iter().any(|r|r["kind"]=="contractor"));
        assert_eq!(spheres_sim::save(&g.world),before);
        if let Ok(file)=std::env::var("SPHERES_S03_UI_FIXTURE") {
            std::fs::write(file,serde_json::to_vec_pretty(&json!({"fixture_note":"Synthetic starting plant and appropriation; all formation, assignment, settlement and work are native executed actions.","snapshot":directory,"adoption":adoption,"action_review":action_review,"formation_review":formation})).unwrap()).unwrap();
        }
    }
    #[test]
    fn directory_and_adoption_preview_are_pure_and_player_scoped() {
        let mut g = crate::Game::new(1990,Some(NationId::France));
        crate::play_rules(&mut g);
        let before = spheres_sim::save(&g.world);
        let view = snapshot(&g.world,NationId::France,&g.session_id);
        assert!(view["directory"].is_array());
        assert!(view["suppliers"]["firms"].is_array());
        let _ = preview(&g.world,NationId::France,&g.session_id,&json!({"command":{"kind":"enable_companies"}})).unwrap();
        assert_eq!(spheres_sim::save(&g.world),before);
        assert_eq!(snapshot(&g.world,NationId::Japan,&g.session_id)["available"],false);
        assert!(preview(&g.world,NationId::Japan,&g.session_id,&json!({"command":{"kind":"enable_companies"}})).is_err());
    }
    #[test]
    fn contractor_parser_bounds_identity_and_requires_reviewed_tokens() {
        let w=spheres_sim::init::world_1990(Default::default());
        let me=NationId::France;
        let valid=json!({"kind":"assign_sector_contractor","nation":"Japan","company":1,
            "target":{"kind":"construction","project":7},"quote":"reviewed"});
        assert!(matches!(crate::parse_command(&w,&valid,me),Some(Command::AssignSectorContractor{nation,..}) if nation==me));
        for id in [json!(-1),json!(4294967296u64),json!(1.5),json!("supplier:1")] {
            let mut bad=valid.clone();bad["company"]=id;assert!(crate::parse_command(&w,&bad,me).is_none());
        }
        for token in [Value::Null,json!(""),json!("x".repeat(257))] {
            let mut bad=valid.clone();bad["quote"]=token;assert!(crate::parse_command(&w,&bad,me).is_none());
        }
        let mut bad=valid.clone();bad["target"]=json!({"kind":"supplier","project":7});
        assert!(crate::parse_command(&w,&bad,me).is_none());
        for path in ["/api/companies","/api/companies-preview"] {
            assert!(crate::exchange_read_path(path));
            assert!(!crate::exchange_session_matches(&tiny_http::Method::Post,path,&json!({}),"current"));
            assert!(crate::exchange_session_matches(&tiny_http::Method::Post,path,&json!({"session_id":"current"}),"current"));
        }
    }
}
