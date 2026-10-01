//! Disposable authored-date fixtures for the production Tonga browser journey.
//! No ticks, simulated years, historical succession or campaign qualification.
use serde_json::{json, Value};
use spheres_sim::{
    institutional_leadership as institutions,
    world::{GameRules, NationId, WorldState},
    Command,
};
use std::{fs, io::Write, path::Path};

fn exclusive(path: &Path, bytes: &[u8]) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}

fn world(year: i32, month: u32, day: u32) -> WorldState {
    let mut w = spheres_sim::init::world_1990(GameRules {
        seed: 13,
        ideology_blocs: true,
        historical_party_leadership: true,
        daily_simulation: true,
        ..Default::default()
    });
    w.player = Some(NationId::Tonga);
    spheres_sim::party_leadership::ensure_all(&mut w);
    w.year = year;
    w.month = month;
    w.day = day;
    w.nation_mut(NationId::Tonga).political_capital = 500.0;
    w
}

fn act(w: &mut WorldState, action: institutions::Action) {
    spheres_sim::apply_command(
        w,
        &Command::TongaInstitutions {
            nation: NationId::Tonga,
            action,
        },
    )
    .unwrap();
}

fn export(root: &Path, name: &str, w: WorldState, actions: Value) -> Value {
    institutions::validate(&w).unwrap();
    let w = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    let before = spheres_sim::save(&w);
    let view = institutions::view(&w, NationId::Tonga);
    assert_eq!(
        spheres_sim::save(&w),
        before,
        "Reading never appoints an officeholder"
    );
    assert_eq!(
        spheres_sim::save(&spheres_sim::load(&before).unwrap()),
        before
    );
    let file = format!("{name}.json");
    exclusive(&root.join(&file), before.as_bytes());
    json!({"name":name,"file":file,"date":format!("{:04}-{:02}-{:02}",w.year,w.month,w.day),
        "player":"Tonga","days_advanced":0,"native_actions":actions,"institutions":view})
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "Usage: tonga_browser_fixtures NEW_ABSOLUTE_DIRECTORY EXACT_SOURCE_REVISION"
    );
    let root = Path::new(&args[1]);
    let revision = &args[2];
    assert!(
        root.is_absolute() && !root.exists(),
        "Never overwrite a fixture or user save"
    );
    assert!(revision.len() == 40 && revision.bytes().all(|b| b.is_ascii_hexdigit()));
    fs::create_dir(root).unwrap();
    let mut endpoint = world(2035, 12, 31);
    act(&mut endpoint, institutions::Action::Reform);
    act(&mut endpoint, institutions::Action::HoldElection);
    act(
        &mut endpoint,
        institutions::Action::RecommendPrimeMinister {
            person_id: "fictional_to_sitani_lolohea".into(),
        },
    );
    act(
        &mut endpoint,
        institutions::Action::AppointMinister {
            person_id: "fictional_to_pisila_tukuafu".into(),
        },
    );
    endpoint.year = 2036;
    endpoint.month = 1;
    endpoint.day = 1;
    let cases = vec![
        export(root, "tonga-reference-cutoff", world(2026, 9, 7), json!([])),
        export(root, "tonga-first-fiction", world(2026, 9, 8), json!([])),
        export(root, "tonga-last-fiction", world(2035, 12, 31), json!([])),
        export(
            root,
            "tonga-after-endpoint",
            endpoint,
            json!([
                "reform",
                "hold_election",
                "recommend_prime_minister",
                "appoint_minister"
            ]),
        ),
    ];
    let manifest = json!({"version":1,"fixture":"tonga-authored-date-browser","source_revision":revision,
        "scope":"Public native init/command/save API fixtures. Dates and 500 political capital are authored test inputs, no days were simulated. Opening Crown and PM are intentionally retained counterfactually. Future commands are explicit; this is not historical chronology, a 46-year campaign, human signoff or country qualification. Normal production Load may adopt its standard legacy-save capabilities.",
        "days_advanced":0,"cases":cases});
    exclusive(
        &root.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap().as_bytes(),
    );
    println!(
        "TONGA_BROWSER_FIXTURES={}",
        root.join("manifest.json").display()
    );
}
