//! Export the runtime catalog for production tracking; no campaign is loaded.
use spheres_sim::party_leadership;
use serde_json::json;
use std::collections::BTreeSet;

fn main() {
    let candidates = party_leadership::future_catalog();
    let parties: BTreeSet<_> = candidates.iter().map(|c| (c.nation, c.party.as_str())).collect();
    let countries: BTreeSet<_> = candidates.iter().map(|c| c.nation).collect();
    let rows = candidates.iter().map(|c| json!({
        "person_id":c.person.id,"name":c.person.name,"nation":c.nation,
        "party":c.party,"component":c.component,"origin":c.origin,
        "component_available":party_leadership::future::component_eligible(c.nation, &c.party, c.component.as_deref()),
        "executive_eligibility":party_leadership::executive_eligibility::future_info(c),
        "historical_continuation_status":party_leadership::future::historical_continuation(c.nation, &c.party).map(|r|r.status.as_str()),
        "historical_continuation":party_leadership::future::historical_continuation(c.nation, &c.party),
        "profile_id":c.profile_id,"appearance_seed":c.appearance_seed,
        "presentation":c.presentation,"name_pool":c.name_pool,
        "editorial_status":c.editorial_status,"avatar_status":"tracked_separately_in_fictional_portraits_manifest"
    })).collect::<Vec<_>>();
    let output = json!({"version":1,"historical_reference_through":party_leadership::future::HISTORICAL_THROUGH,
        "from":party_leadership::future::FROM,"until_exclusive":party_leadership::future::UNTIL,
        "candidate_count":candidates.len(),"party_count":parties.len(),"country_count":countries.len(),
        "future_leadership_seat_policy":party_leadership::leadership_seats::policy().ok(),
        "scope":"Current simulation party inventory; fictional character templates are not finished avatars or researched historical people.",
        "candidates":rows});
    let text = serde_json::to_string_pretty(&output).unwrap();
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(&path, text).expect("write catalog export");
        eprintln!("Exported {} fictional candidates for {} parties in {} countries to {}", candidates.len(), parties.len(), countries.len(), path);
    } else { println!("{text}"); }
}
