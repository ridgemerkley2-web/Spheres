//! The company directory is a read-only projection of simulation state.
use spheres_sim::{companies, world::{NationId, WorldState}};

pub fn snapshot(w: &WorldState, nation: NationId) -> serde_json::Value {
    let mut view = companies::snapshot(w, nation);
    if let Some(rows) = view["companies"].as_array_mut() {
        for row in rows {
            row["fee_basis"] = serde_json::json!(match row["sector"].as_str().unwrap_or("") {
                "mining" => "Fee on modeled extraction operating cost (30% of reference output value), funded by Industry → Minerals & processing. Without funding the mine keeps its ordinary output.",
                "logistics" => "Fee on a modeled $10 per tonne handled at this terminal. Paid by your treasury through the normal cash/debt system, only when freight moves.",
                "energy" => "Fee on actual dispatched power operating costs, paid from Energy supply funding.",
                "construction" => "Fee on actual construction work, within your shared daily construction budget.",
                "research" => "Fee on funded prototype work at your research centers. Technology and prerequisite gates still apply.",
                _ => "Fee on actual operating work, within the existing project or department funding.",
            });
        }
    }
    view
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn company_directory_reads_never_enroll_or_advance_a_campaign() {
        let mut w = spheres_sim::init::world_1990(Default::default());
        let before = spheres_sim::save(&w);
        assert_eq!(snapshot(&w, NationId::USA)["enabled"], false);
        assert_eq!(before, spheres_sim::save(&w));
        w.rules.daily_simulation = true;
        companies::enable(&mut w);
        let before = spheres_sim::save(&w);
        let view = snapshot(&w, NationId::USA);
        assert!(view["companies"].as_array().unwrap().len() >= 21);
        assert!(view["companies"].as_array().unwrap().iter().all(|c| c["fee_basis"].is_string()));
        assert_eq!(before, spheres_sim::save(&w));
    }
    #[test]
    fn company_commands_use_current_player_and_reject_malformed_ids() {
        let w = spheres_sim::init::world_1990(Default::default());
        let me = NationId::USA;
        let value = serde_json::json!({"kind":"assign_company","nation":"Botswana","company":1,
            "target":{"kind":"construction","project":7}});
        assert!(matches!(crate::parse_command(&w,&value,me),Some(spheres_sim::Command::AssignCompany {nation,..}) if nation==me));
        for id in [serde_json::json!(-1),serde_json::json!(4294967296u64),serde_json::json!(1.5)] {
            let mut invalid=value.clone();invalid["company"]=id;
            assert!(crate::parse_command(&w,&invalid,me).is_none());
        }
        let mut invalid=value.clone();invalid["target"]=serde_json::json!({"kind":"unrecognized","project":7});
        assert!(crate::parse_command(&w,&invalid,me).is_none());
        assert!(crate::exchange_read_path("/api/companies"));
        assert!(!crate::exchange_session_matches(&tiny_http::Method::Get,"/api/companies",&serde_json::json!({}),"current"));
        assert!(crate::exchange_session_matches(&tiny_http::Method::Get,"/api/companies?session_id=current",&serde_json::json!({}),"current"));
    }
}
