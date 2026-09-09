//! Export real read-only leadership views for static UI QA; no server or save is loaded.
use serde_json::json;
use spheres_sim::{party_leadership, world::{GameRules, NationId}};

fn main() {
    let world=spheres_sim::init::world_1990(GameRules {
        ideology_blocs:true,historical_party_leadership:true,daily_simulation:true,..Default::default()
    });
    let original=spheres_sim::save(&world);
    let nations=[NationId::USA,NationId::UK,NationId::Japan,NationId::France,NationId::Germany,
        NationId::China,NationId::India,NationId::Italy,NationId::Brazil,NationId::SouthAfrica,
        NationId::Canada,NationId::Australia];
    let views=nations.into_iter().map(|nation|json!({
        "nation":nation,
        "reference_1990":party_leadership::reference_view(&world,nation,"1990-01-01"),
        "future_reference_2030":party_leadership::reference_view(&world,nation,"2030-01-01")
    })).collect::<Vec<_>>();
    assert_eq!(spheres_sim::save(&world),original,"reference exports must not mutate the campaign");
    let value=json!({"version":1,"purpose":"Static leadership UI QA using native simulation read models",
        "campaign_date":"1990-01-01","historical_reference_through":party_leadership::future::HISTORICAL_THROUGH,
        "read_only":true,"note":"The 2030 view is a fictional-future reference beside unchanged 1990 campaign bindings; no future government is predicted or seated.",
        "nations":views});
    let text=serde_json::to_string_pretty(&value).unwrap();
    if let Some(path)=std::env::args().nth(1) {
        std::fs::write(&path,text).expect("write static leadership views");
        eprintln!("Exported 1990/2030 read-only leadership views for 12 nations to {path}");
    } else {println!("{text}");}
}
