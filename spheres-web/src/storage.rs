//! Campaign files are presentation envelopes around the unchanged sim save.
//! Every replacement is staged and synced; the previous valid bytes remain in
//! a backup. Neither file time nor storage metadata enters WorldState.
use crate::{
    history::{Event, Snapshot},
    Game,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

const VERSION: u32 = 1;
#[derive(Serialize, Deserialize)]
struct Campaign {
    format: String,
    version: u32,
    world: Value,
    history: Vec<Snapshot>,
    log: Vec<Event>,
    #[serde(default)]
    history_epoch: u64,
    #[serde(default)]
    journey: crate::campaign_journey::Journey,
    saved_date: String,
    player: Option<String>,
    saved_unix: u64,
}
pub(crate) fn encode(g: &Game) -> Result<String, String> {
    let file = Campaign {
        format: "spheres-campaign".into(),
        version: VERSION,
        world: serde_json::from_str(&crate::save(&g.world)).map_err(|e| e.to_string())?,
        history: g.history.clone(),
        log: g.log.clone(),
        history_epoch: g.history_epoch,
        journey: g.journey.clone(),
        saved_date: g.world.date_str(),
        player: g.world.player.map(|p| p.name().into()),
        saved_unix: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };
    serde_json::to_string(&file).map_err(|e| e.to_string())
}
pub(crate) fn decode(text: &str) -> Result<Game, String> {
    // Keep the potentially large history/log in their compact typed form.
    // Typed Serde structs skip unknown fields, unlike Value, so successful typed
    // parsing alone is insufficient: even ignored numbers/depth must obey the
    // original JSON parser's limits. The validation walk retains no value tree.
    if let Ok(file) = serde_json::from_str::<Campaign>(text) {
        if file.format == "spheres-campaign" && file.version == VERSION
            && serde_json::from_str::<CheckedJson>(text).is_ok()
        {
            return restore_campaign(file);
        }
    }
    // Preserve legacy migrations, last-key-wins duplicate handling and the
    // original error strings for every archive outside the fast path.
    decode_value(text)
}

struct CheckedJson;
impl<'de> Deserialize<'de> for CheckedJson {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CheckedJson;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { f.write_str("a JSON value") }
            fn visit_unit<E: serde::de::Error>(self) -> Result<CheckedJson, E> { Ok(CheckedJson) }
            fn visit_bool<E: serde::de::Error>(self, _: bool) -> Result<CheckedJson, E> { Ok(CheckedJson) }
            fn visit_i64<E: serde::de::Error>(self, _: i64) -> Result<CheckedJson, E> { Ok(CheckedJson) }
            fn visit_u64<E: serde::de::Error>(self, _: u64) -> Result<CheckedJson, E> { Ok(CheckedJson) }
            fn visit_f64<E: serde::de::Error>(self, _: f64) -> Result<CheckedJson, E> { Ok(CheckedJson) }
            fn visit_str<E: serde::de::Error>(self, _: &str) -> Result<CheckedJson, E> { Ok(CheckedJson) }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<CheckedJson, A::Error> {
                while seq.next_element::<CheckedJson>()?.is_some() {}
                Ok(CheckedJson)
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<CheckedJson, A::Error> {
                while map.next_key::<CheckedJson>()?.is_some() { map.next_value::<CheckedJson>()?; }
                Ok(CheckedJson)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

fn decode_value(text: &str) -> Result<Game, String> {
    let value: Value =
        serde_json::from_str(text).map_err(|e| format!("Cannot read campaign: {e}"))?;
    if value.get("format").is_none() || matches!(value["format"].as_str(), Some("spheres-equipment-save" | "spheres-party-leadership-save" | "spheres-economy-save" | "spheres-companies-save" | "spheres-integrated-save")) {
        // The original CLI/browser format is still supported and uses every
        // simulation migration. It cannot invent an archive it never recorded.
        let mut g = crate::loaded_play_game(spheres_sim::load_value(value)?);
        g.storage_notice=Some("Simulation save restored. Earlier history was not recorded in this file; new campaign saves preserve it.".into());
        return Ok(g);
    }
    if value["format"] != "spheres-campaign" || value["version"] != VERSION {
        return Err(
            "This campaign format/version is not supported. The live campaign was kept.".into(),
        );
    }
    let file: Campaign =
        serde_json::from_value(value).map_err(|e| format!("Invalid campaign archive: {e}"))?;
    restore_campaign(file)
}

fn restore_campaign(file: Campaign) -> Result<Game, String> {
    let world = spheres_sim::load_value(file.world)?;
    let mut g = crate::loaded_play_game(world);
    let current = Snapshot::from_world(&g.world).t;
    if file.history.iter().any(|s| {
        !s.t.is_finite()
            || s.t > current
            || !(1..=12).contains(&s.month)
            || s.day
                .is_some_and(|d| d < 1 || d > spheres_sim::world::days_in_month(s.year, s.month))
    }) || file.history.windows(2).any(|p| p[0].t >= p[1].t)
    {
        return Err("The campaign history has invalid dates. The live campaign was kept.".into());
    }
    if !file.history.is_empty() {
        g.history = file.history;
    }
    g.log = file.log;
    g.history_epoch = file.history_epoch;
    g.journey = file.journey;
    g.storage_notice = Some("Campaign and its history restored.".into());
    Ok(g)
}

fn slot_path(root: &Path, slot: &str) -> Result<PathBuf, String> {
    if slot == "default" || slot.is_empty() {
        return Ok(root.join("save.json"));
    }
    if slot.len() > 64
        || !slot
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err("Use a save name of 1–64 letters, numbers, hyphens or underscores.".into());
    }
    Ok(root.join("saves").join(format!("{slot}.json")))
}
pub(crate) fn slot(payload: &Value) -> Result<&str, String> {
    match payload.get("slot") {
        None => Ok("default"),
        Some(v) => v
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("Choose a valid save slot.".into()),
    }
}
fn temporary(path: &Path) -> PathBuf {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    path.with_extension(format!(
        "tmp-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ))
}
fn stage(
    path: &Path,
    write: impl FnOnce(&mut File) -> std::io::Result<()>,
) -> std::io::Result<PathBuf> {
    let temp = temporary(path);
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        write(&mut file)?;
        file.sync_all()?;
        Ok(())
    })();
    match result {
        Ok(()) => Ok(temp),
        Err(e) => {
            let _ = fs::remove_file(&temp);
            Err(e)
        }
    }
}
/// std::fs::rename replaces an existing FILE atomically on both Windows and
/// Unix. Never remove the destination first. Save data and backup are synced
/// before replacement; Unix also syncs the containing directory afterward.
fn atomic_write(
    path: &Path,
    backup_previous: bool,
    write: impl FnOnce(&mut File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let staged = stage(path, write)?;
    let result = (|| {
        if backup_previous && path.exists() {
            let backup = path.with_extension("json.bak");
            let old = fs::read(path)?;
            let backup_temp = stage(&backup, |file| file.write_all(&old))?;
            if let Err(e) = fs::rename(&backup_temp, &backup) {
                let _ = fs::remove_file(backup_temp);
                return Err(e);
            }
        }
        fs::rename(&staged, path)?;
        #[cfg(unix)]
        if let Some(parent) = path.parent() {
            File::open(parent)?.sync_all()?;
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(staged);
    }
    result
}
pub(crate) fn write(root: &Path, slot: &str, g: &Game) -> Result<Value, String> {
    let path = slot_path(root, slot)?;
    let bytes = encode(g)?;
    // A damaged current file must never overwrite the last known-good backup.
    let valid_previous = fs::read_to_string(&path)
        .ok()
        .is_some_and(|text| decode(&text).is_ok());
    atomic_write(&path, valid_previous, |f| f.write_all(bytes.as_bytes()))
        .map_err(|e| format!("Could not save campaign: {e}"))?;
    Ok(
        json!({"ok":true,"slot":slot,"path":path.to_string_lossy(),"date":g.world.date_str(),
        "history_points":g.history.len(),"dispatches":g.log.len()}),
    )
}
pub(crate) fn read(root: &Path, slot: &str, backup: bool) -> Result<Game, String> {
    let path = slot_path(root, slot)?;
    let path = if backup {
        path.with_extension("json.bak")
    } else {
        path
    };
    let text = fs::read_to_string(path).map_err(|e| format!("Could not open campaign: {e}"))?;
    decode(&text)
}
pub(crate) fn list(root: &Path) -> Value {
    let mut slots = vec![("default".to_string(), root.join("save.json"))];
    if let Ok(files) = fs::read_dir(root.join("saves")) {
        for entry in files.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|s| s == "json") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    slots.push((name.to_string(), path));
                }
            }
        }
    }
    slots.sort_by(|a, b| a.0.cmp(&b.0));
    let records = slots
        .into_iter()
        .filter(|(_, p)| p.is_file())
        .map(|(slot, path)| {
            let data = fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str::<Value>(&s).ok());
            let envelope = data.as_ref().is_some_and(|v| v.get("format").is_some());
            json!({"slot":slot,"date":data.as_ref().and_then(|v|v.get("saved_date")),
            "player":data.as_ref().and_then(|v|v.get("player")),"legacy":!envelope,
            "readable":data.is_some(),"backup":path.with_extension("json.bak").is_file(),
            "autosave":slot.starts_with("auto-"),"bytes":fs::metadata(&path).ok().map(|m|m.len())})
        })
        .collect::<Vec<_>>();
    let directory = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    json!({"slots":records,"directory":directory.to_string_lossy(),"slots_directory":directory.join("saves").to_string_lossy(),
        "autosave":"At the first advance after a calendar month ends; three rotating slots with backups."})
}
/// Called by the HTTP boundary only. Tests and headless simulation never do IO.
pub(crate) fn autosave(root: &Path, g: &mut Game) {
    let month = crate::month_index(g.world.year, g.world.month);
    if g.world.player.is_none() || month <= g.autosaved_month {
        return;
    }
    let slot = format!("auto-{}", month % 3 + 1);
    match write(root,&slot,g) {
        Ok(_)=>{g.autosaved_month=month;g.storage_notice=Some(format!("Autosaved {} to {slot}.",g.world.date_str()));}
        Err(error)=>g.storage_notice=Some(format!("Autosave failed: {error}. Your live campaign is still running; save again from Campaigns.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_same_campaign(actual: &Game, expected: &Game) {
        assert!(crate::save(&actual.world) == crate::save(&expected.world), "Exact world bytes differ");
        assert!(serde_json::to_vec(&actual.history).unwrap() == serde_json::to_vec(&expected.history).unwrap(), "Exact history bytes differ");
        assert!(serde_json::to_vec(&actual.log).unwrap() == serde_json::to_vec(&expected.log).unwrap(), "Exact dispatch bytes differ");
        assert_eq!(actual.history_epoch, expected.history_epoch);
        assert_eq!(actual.journey, expected.journey);
        assert_eq!(actual.storage_notice, expected.storage_notice);
        assert_eq!(actual.autosaved_month, expected.autosaved_month);
    }

    fn assert_decode_oracle(text: &str) {
        // Original full-Value decoder remains the fallback and reference path.
        match (decode(text), decode_value(text)) {
            (Ok(actual), Ok(expected)) => assert_same_campaign(&actual, &expected),
            (Err(actual), Err(expected)) => assert_eq!(actual, expected, "Legacy error text changed"),
            (Ok(_), Err(error)) => panic!("Typed path accepted an archive rejected by the original: {error}"),
            (Err(error), Ok(_)) => panic!("Typed path rejected an archive accepted by the original: {error}"),
        }
    }

    #[test]
    fn typed_campaign_decode_keeps_complete_history_log_journey_and_numeric_bytes() {
        let mut g = crate::Game::new(1990, Some(crate::NationId::France));
        crate::play_rules(&mut g);
        g.record("Archive: accents é, Unicode 東京, quotes \" and a newline\nremain intact.".into());
        g.world.day = 2;
        g.snapshot();
        g.history_epoch = 41;
        g.journey.beyond_2035 = true;
        g.journey.transitions.push(crate::campaign_journey::Transition {
            from: crate::NationId::France, to: crate::NationId::Belgium, date: "2 Jan 1990".into(),
        });
        let archive = encode(&g).unwrap();
        assert_decode_oracle(&archive);
        let mut value: Value = serde_json::from_str(&archive).unwrap();
        value["history"][0]["oil"] = json!("FLOAT_LITERAL");
        let template = value.to_string();
        for literal in ["-0", "-0.0", "1e-320", "2.2250738585072014e-308", "9007199254740993",
            "18446744073709551615", "1.0000000000000002", "1.7976931348623157e308"] {
            assert_decode_oracle(&template.replace("\"FLOAT_LITERAL\"", literal));
        }
        let mut defaults: Value = serde_json::from_str(&archive).unwrap();
        defaults.as_object_mut().unwrap().remove("history_epoch");
        defaults.as_object_mut().unwrap().remove("journey");
        defaults["history"] = json!([]);
        assert_decode_oracle(&defaults.to_string());
    }

    #[test]
    fn typed_campaign_decode_preserves_duplicate_keys_legacy_migrations_and_exact_errors() {
        let mut g = crate::Game::new(1990, Some(crate::NationId::France));
        crate::play_rules(&mut g);
        g.record("A retained dispatch.".into());
        let value: Value = serde_json::from_str(&encode(&g).unwrap()).unwrap();
        let archive = value.to_string();
        for text in [
            archive.replacen("\"history\":", "\"history\":[],\"history\":", 1),
            format!("{},\"history\":[]}}", &archive[..archive.len()-1]),
            archive.replacen("\"history\":[{", "\"history\":[{\"t\":-1,", 1),
            archive.replacen("\"log\":[{", "\"log\":[{\"text\":\"Superseded duplicate\",", 1),
            archive.replacen("\"version\":1", "\"version\":999,\"version\":1", 1),
        ] { assert_decode_oracle(&text); }
        for mutate in [
            (|v: &mut Value| v["history"][0]["month"] = json!(13)) as fn(&mut Value),
            |v| v["history"][0]["day"] = json!(32),
            |v| v["history"][0]["t"] = json!(1e20),
            |v| { let point = v["history"][0].clone(); v["history"].as_array_mut().unwrap().push(point); },
            |v| { v.as_object_mut().unwrap().remove("log"); },
            |v| v["saved_unix"] = json!("not an integer"),
            |v| v["format"] = json!("unsupported-campaign"),
            |v| v["version"] = json!(999),
            |v| v["world"]["format"] = json!("unsupported-world"),
        ] {
            let mut invalid = value.clone(); mutate(&mut invalid);
            assert!(decode_value(&invalid.to_string()).is_err());
            assert_decode_oracle(&invalid.to_string());
        }
        assert_decode_oracle(&crate::save(&g.world));
        let raw = serde_json::to_string(&g.world).unwrap();
        assert_decode_oracle(&raw);
        for text in ["{", "null", "[]", "{\"format\":true}", "{\"format\":\"spheres-campaign\",\"version\":1}"] {
            assert_decode_oracle(text);
        }
    }

    #[test]
    fn typed_campaign_decode_cannot_skip_invalid_unknown_numbers_or_recursion_limits() {
        let g = crate::Game::new(1990, Some(crate::NationId::France));
        let archive = encode(&g).unwrap();
        let deep = format!("{}0{}", "[".repeat(140), "]".repeat(140));
        for extra in ["1e400", "-1e400", deep.as_str()] {
            let text = format!("{},\"ignored_extension\":{extra}}}", &archive[..archive.len()-1]);
            assert!(serde_json::from_str::<Value>(&text).is_err());
            assert!(serde_json::from_str::<CheckedJson>(&text).is_err());
            assert_decode_oracle(&text);
        }
        // Invalid ignored fields can also live inside typed timeline entries.
        let nested = archive.replacen("\"history\":[{", "\"history\":[{\"ignored_extension\":1e400,", 1);
        assert!(serde_json::from_str::<Value>(&nested).is_err());
        assert_decode_oracle(&nested);
        let valid_unknown = format!("{},\"ignored_extension\":{{\"unicode\":\"é東京\",\"all\":[null,true,false,-1,1.25,\"text\"]}}}}", &archive[..archive.len()-1]);
        assert!(serde_json::from_str::<CheckedJson>(&valid_unknown).is_ok());
        assert_decode_oracle(&valid_unknown);
    }

    #[test]
    #[ignore = "Read-only real archive parity; set SPHERES_S22_INPUT and coordinate with measured runs"]
    fn typed_campaign_decode_matches_actual_checkpoint_value_oracle() {
        let input = std::path::PathBuf::from(std::env::var_os("SPHERES_S22_INPUT").expect("SPHERES_S22_INPUT"));
        let text = fs::read_to_string(&input).unwrap();
        let started = std::time::Instant::now();
        let actual = decode(&text).unwrap();
        let typed_ms = started.elapsed().as_secs_f64()*1000.0;
        let started = std::time::Instant::now();
        let expected = decode_value(&text).unwrap();
        let value_ms = started.elapsed().as_secs_f64()*1000.0;
        assert_same_campaign(&actual, &expected);
        assert!(fs::read_to_string(&input).unwrap() == text, "Immutable source changed");
        eprintln!("Exact actual archive parity: {} bytes, {} history rows, {} dispatches; typed+validation load {typed_ms:.3} ms, original Value load {value_ms:.3} ms. Sequential diagnostic timings only, not memory or qualification.", text.len(), actual.history.len(), actual.log.len());
    }

    #[test]
    fn consuming_archive_world_matches_string_import_and_keeps_paid_timeline() {
        let crate::s05_campaign_api_tests::PaidWorkFixture{mut game,..}=crate::s05_campaign_api_tests::paid_work_fixture();
        game.record("Preserve the paid campaign archive through consuming decode.".into());
        let archived=encode(&game).unwrap();
        let file:Campaign=serde_json::from_str(&archived).unwrap();
        // This is the former world import boundary: serialize the already parsed
        // world and reload it. The new consuming boundary must preserve its bytes.
        let original_world=crate::load(&serde_json::to_string(&file.world).unwrap()).unwrap();
        let mut restored=decode(&archived).unwrap();
        assert_eq!(crate::save(&restored.world),crate::save(&original_world));
        assert_eq!(restored.history,file.history);
        assert_eq!(restored.log,file.log);
        assert_eq!(restored.history_epoch,file.history_epoch);
        assert_eq!(serde_json::to_value(&restored.journey).unwrap(),serde_json::to_value(&file.journey).unwrap());
        game.advance_days(1,vec![]);
        restored.advance_days(1,vec![]);
        assert_eq!(crate::save(&restored.world),crate::save(&game.world));
        assert_eq!(restored.history,game.history);
        assert_eq!(restored.log,game.log);
        let mut bad:Value=serde_json::from_str(&archived).unwrap();
        bad["world"]["supplier_operations_version"]=json!(999);
        let world_error=crate::load(&bad["world"].to_string()).unwrap_err();
        assert_eq!(decode(&bad.to_string()).err().unwrap(),world_error);
    }
    #[test]
    fn integrated_paid_work_survives_interrupted_write_and_valid_backup_recovery() {
        let crate::s05_campaign_api_tests::PaidWorkFixture{mut game,formation,assignment,..}=crate::s05_campaign_api_tests::paid_work_fixture();
        let root=root();let path=root.join("save.json");
        let open_world=crate::save(&game.world);let open_history=game.history.clone();let open_log=game.log.clone();
        write(&root,"default",&game).unwrap();
        let original=fs::read(&path).unwrap();
        let failed=atomic_write(&path,true,|file|{
            file.write_all(b"incomplete integrated campaign")?;
            Err(std::io::Error::other("injected write interruption"))
        });
        assert!(failed.is_err());assert_eq!(fs::read(&path).unwrap(),original);
        let restored=read(&root,"default",false).unwrap();
        assert_eq!(crate::save(&restored.world),open_world);assert_eq!(restored.history,open_history);assert_eq!(restored.log,open_log);
        game.advance_days(1,vec![]);
        let closed_world=crate::save(&game.world);
        write(&root,"default",&game).unwrap();
        let backup_path=path.with_extension("json.bak");
        assert_eq!(fs::read(&backup_path).unwrap(),original);
        fs::write(&path,b"corrupted newest integrated campaign").unwrap();
        assert!(read(&root,"default",false).is_err());
        let mut backup=read(&root,"default",true).unwrap();
        assert_eq!(crate::save(&backup.world),open_world);assert_eq!(backup.history,open_history);assert_eq!(backup.log,open_log);
        assert_ne!(backup.session_id,game.session_id);
        for payload in [&formation,&assignment] {
            assert!(crate::immediate_request(&mut backup,payload).unwrap_err().requires_review);
            assert_eq!(crate::save(&backup.world),open_world);
        }
        backup.advance_days(1,vec![]);
        assert_eq!(crate::save(&backup.world),closed_world,"The recovered receivable settles exactly once on its original day");
        assert_eq!(backup.history,game.history);assert_eq!(backup.log,game.log);
        write(&root,"default",&game).unwrap();
        assert_eq!(fs::read(&backup_path).unwrap(),original,"Damaged newest bytes cannot replace the valid integrated backup");
        let twice=read(&root,"default",false).unwrap();
        assert_eq!(crate::save(&twice.world),closed_world);
        assert!(!fs::read_dir(&root).unwrap().any(|entry|entry.unwrap().file_name().to_string_lossy().contains("tmp-")));
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn integrated_warfare_archive_and_standalone_import_keep_every_capability() {
        let mut g=crate::Game::new(1990,Some(crate::NationId::France));crate::play_rules(&mut g);
        spheres_sim::connected_economy::enable(&mut g.world).unwrap();
        spheres_sim::resources::tick(&mut g.world);
        spheres_sim::company_network::enable(&mut g.world).unwrap();
        spheres_sim::operational_warfare::enable(&mut g.world).unwrap();
        let saved=crate::save(&g.world);let value:Value=serde_json::from_str(&saved).unwrap();
        assert_eq!(value["format"],"spheres-integrated-save");
        let once=decode(&saved).unwrap();assert_eq!(crate::save(&once.world),saved);
        let twice=decode(&crate::save(&once.world)).unwrap();assert_eq!(crate::save(&twice.world),saved);
        assert_eq!(crate::save(&decode(&encode(&g).unwrap()).unwrap().world),saved);
        for field in ["equipment_version","party_leadership_version","economy_version","company_network_version","supplier_operations_version","warfare_version"] {
            let mut bad=value.clone();bad[field]=json!(999);assert!(decode(&bad.to_string()).is_err(),"unknown {field}");
        }
    }
    #[test]
    fn combined_company_save_import_retains_capability_and_archive_without_load_adoption() {
        let mut g=crate::Game::new(1990,Some(crate::NationId::France));
        crate::play_rules(&mut g);
        let old=crate::save(&g.world);
        let legacy=decode(&old).unwrap();
        assert!(!legacy.world.sector_contractors.enabled);
        assert!(!spheres_sim::supplier_operations::enabled(&legacy.world));
        spheres_sim::connected_economy::enable(&mut g.world).unwrap();
        spheres_sim::resources::tick(&mut g.world);
        spheres_sim::company_network::enable(&mut g.world).unwrap();
        g.record("Company network explicitly adopted.".into());
        let saved=crate::save(&g.world);
        let value:Value=serde_json::from_str(&saved).unwrap();
        assert_eq!(value["format"],"spheres-companies-save");
        let loaded=decode(&saved).unwrap();
        assert_eq!(crate::save(&loaded.world),saved);
        let twice=decode(&crate::save(&loaded.world)).unwrap();
        assert_eq!(crate::save(&twice.world),saved);
        let archived=decode(&encode(&g).unwrap()).unwrap();
        assert_eq!(crate::save(&archived.world),saved);
        assert_eq!(archived.log,g.log);
        assert_eq!(archived.history,g.history);
        for field in ["equipment_version","party_leadership_version","economy_version","supplier_operations_version"] {
            let mut unknown=value.clone();unknown[field]=json!(999);
            assert!(decode(&unknown.to_string()).is_err(),"unknown {field} cannot be silently stripped");
        }
    }
    fn root() -> PathBuf {
        let p = std::env::temp_dir().join(format!("spheres-storage-{}", crate::fresh_session_id()));
        fs::create_dir_all(&p).unwrap();
        p
    }
    #[test]
    fn failed_write_keeps_previous_valid_campaign_and_archive() {
        let root = root();
        let path = root.join("save.json");
        let mut g = crate::Game::new(1990, Some(crate::NationId::USA));
        crate::play_rules(&mut g);
        g.history.clear();
        g.snapshot();
        g.record("A campaign milestone.".into());
        write(&root, "default", &g).unwrap();
        let previous = fs::read(&path).unwrap();
        let error = atomic_write(&path, true, |file| {
            file.write_all(b"partial campaign")?;
            Err(std::io::Error::other("injected disk failure"))
        });
        assert!(error.is_err());
        assert_eq!(fs::read(&path).unwrap(), previous);
        let restored = read(&root, "default", false).unwrap();
        assert_eq!(restored.log, g.log);
        assert_eq!(restored.history, g.history);
        g.world.day = 2;
        g.snapshot();
        write(&root, "default", &g).unwrap();
        assert_eq!(fs::read(path.with_extension("json.bak")).unwrap(), previous);
        assert_eq!(read(&root, "default", false).unwrap().history.len(), 2);
        fs::write(&path, b"damaged current save").unwrap();
        write(&root, "default", &g).unwrap();
        assert_eq!(
            fs::read(path.with_extension("json.bak")).unwrap(),
            previous,
            "a damaged save must not replace the valid backup"
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn envelope_load_preserves_sim_timeline_and_archive_legacy_still_loads() {
        let mut g = crate::Game::new(7, Some(crate::NationId::USA));
        crate::play_rules(&mut g);
        g.history.clear();
        g.snapshot();
        g.advance_days(3, vec![]);
        let mut loaded = decode(&encode(&g).unwrap()).unwrap();
        assert_eq!(loaded.log, g.log);
        assert_eq!(loaded.history, g.history);
        assert_ne!(
            loaded.session_id, g.session_id,
            "stale tabs cannot mutate the loaded branch"
        );
        g.advance_days(3, vec![]);
        loaded.advance_days(3, vec![]);
        assert_eq!(crate::save(&g.world), crate::save(&loaded.world));
        assert_eq!(loaded.history, g.history);
        assert_eq!(loaded.log, g.log);
        assert_eq!(decode(&crate::save(&g.world)).unwrap().history.len(), 1);
        let mut future: Value = serde_json::from_str(&encode(&g).unwrap()).unwrap();
        future["version"] = 999.into();
        assert!(decode(&future.to_string()).is_err());
    }
    #[test]
    fn slots_cannot_escape_root_and_three_autosaves_rotate() {
        let root = root();
        let mut g = crate::Game::new(1990, Some(crate::NationId::USA));
        for bad in ["../lost", "C:\\save", "a/b", ".."] {
            assert!(write(&root, bad, &g).is_err());
        }
        write(&root, "France-1990", &g).unwrap();
        for month in 2..=6 {
            g.world.month = month;
            autosave(&root, &mut g);
        }
        let slots = list(&root);
        assert_eq!(slots["slots"].as_array().unwrap().len(), 4);
        assert!(root.join("saves/auto-2.json.bak").is_file());
        fs::remove_dir_all(root).unwrap();
    }
}
