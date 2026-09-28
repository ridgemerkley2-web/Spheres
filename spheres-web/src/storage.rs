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
    // A derived struct also accepts positional JSON arrays. The archive
    // envelope historically requires an object with a named format field.
    if text.trim_start().starts_with('{') {
        if let Ok(file) = serde_json::from_str::<Campaign>(text) {
            if file.format == "spheres-campaign" && file.version == VERSION
                && serde_json::from_str::<CheckedJson>(text).is_ok()
            {
                return restore_campaign(file);
            }
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
    stage_at(temporary(path), write)
}
// Separate the chosen path so collision/error ownership can be tested without
// racing the process-wide unique-name counter. This extraction changes no IO.
fn stage_at(
    temp: PathBuf,
    write: impl FnOnce(&mut File) -> std::io::Result<()>,
) -> std::io::Result<PathBuf> {
    // Until exclusive creation succeeds this path belongs to somebody else
    // (for example an interrupted process whose PID has since been reused).
    // Never clean up a candidate that this operation did not create.
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    let result = (|| {
        write(&mut file)?;
        file.sync_all()?;
        Ok(())
    })();
    drop(file);
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
// Only our compact archive's terminal wall-clock field is excluded. Compare
// the entire remaining envelope byte-for-byte: world, history, log, journey and
// metadata all participate. This is called only after the old archive decodes;
// legacy, pretty, reordered and other noncanonical saves keep normal rotation.
fn compact_archive_content(text: &str) -> Option<&str> {
    if !text.starts_with("{\"format\":\"spheres-campaign\",\"version\":1,\"world\":") {
        return None;
    }
    let (content, tail) = text.rsplit_once(",\"saved_unix\":")?;
    let timestamp = tail.strip_suffix('}')?;
    if timestamp.is_empty() || !timestamp.bytes().all(|b| b.is_ascii_digit())
        || (timestamp.len() > 1 && timestamp.starts_with('0'))
        || timestamp.parse::<u64>().is_err()
    {
        return None;
    }
    Some(content)
}

pub(crate) fn write(root: &Path, slot: &str, g: &Game) -> Result<Value, String> {
    let path = slot_path(root, slot)?;
    let bytes = encode(g)?;
    // A damaged current file must never overwrite the last known-good backup.
    // Nor may retrying an already completed save erase the earlier recovery
    // point just because this encode has a newer wall-clock timestamp. If no
    // backup exists yet, an identical second save still establishes that copy.
    let backup_exists = path.with_extension("json.bak").is_file();
    let backup_previous = fs::read_to_string(&path)
        .ok()
        .is_some_and(|text| decode(&text).is_ok() && (!backup_exists || match
            (compact_archive_content(&text), compact_archive_content(&bytes)) {
                (Some(previous), Some(next)) => previous != next,
                _ => true,
            }));
    atomic_write(&path, backup_previous, |f| f.write_all(bytes.as_bytes()))
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
    let mut slots = std::collections::BTreeMap::from([
        ("default".to_string(), root.join("save.json")),
    ]);
    if let Ok(files) = fs::read_dir(root.join("saves")) {
        for entry in files.flatten() {
            let path = entry.path();
            if !path.is_file() { continue; }
            let Some(file) = path.file_name().and_then(|s| s.to_str()) else { continue; };
            let Some(name) = file.strip_suffix(".json.bak").or_else(|| file.strip_suffix(".json")) else { continue; };
            // `default` is the root save.json alias, never saves/default.json.
            // Enumerate only names that the ordinary load endpoint can address.
            if name.is_empty() || name == "default" { continue; }
            if let Ok(primary) = slot_path(root, name) {
                slots.insert(name.to_string(), primary);
            }
        }
    }
    let records = slots
        .into_iter()
        .filter(|(_, p)| p.is_file() || p.with_extension("json.bak").is_file())
        .map(|(slot, path)| {
            let current_exists = path.is_file();
            let backup_path = path.with_extension("json.bak");
            let backup = backup_path.is_file();
            let data = fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str::<Value>(&s).ok());
            // Listing describes JSON metadata without decoding every potentially
            // large campaign. A backup's presence is not a validity promise;
            // explicit Load previous backup still performs the full decoder.
            let backup_data = if data.is_none() && backup {
                fs::read_to_string(&backup_path).ok()
                    .and_then(|s| serde_json::from_str::<Value>(&s).ok())
            } else { None };
            let metadata_from_backup = backup_data.is_some();
            let metadata = data.as_ref().or(backup_data.as_ref());
            let envelope = metadata.is_some_and(|v| v.get("format").is_some());
            json!({"slot":slot,"date":metadata.and_then(|v|v.get("saved_date")),
            "player":metadata.and_then(|v|v.get("player")),"legacy":!envelope,
            "readable":data.is_some(),"backup":backup,"current_exists":current_exists,
            "metadata_from_backup":metadata_from_backup,
            "autosave":slot.starts_with("auto-"),"bytes":fs::metadata(&path).ok().filter(|m|m.is_file()).map(|m|m.len())})
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
    fn typed_campaign_decode_preserves_rejection_of_positional_archive_arrays() {
        let g = crate::Game::new(1990, Some(crate::NationId::France));
        let archive = encode(&g).unwrap();
        let value: Value = serde_json::from_str(&archive).unwrap();
        let positional = Value::Array([
            "format", "version", "world", "history", "log", "history_epoch",
            "journey", "saved_date", "player", "saved_unix",
        ].into_iter().map(|field| value[field].clone()).collect()).to_string();
        assert!(serde_json::from_str::<Campaign>(&positional).is_ok(),
            "the derived struct accepts this otherwise complete positional archive");
        assert!(serde_json::from_str::<CheckedJson>(&positional).is_ok());
        for text in [positional.clone(), format!(" \r\n\t{positional}")] {
            assert!(decode_value(&text).is_err(), "the original named-format archive decoder rejects arrays");
            assert_decode_oracle(&text);
        }
        assert_decode_oracle(&format!(" \r\n\t{archive}"));
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

    // Match new_game's playable capabilities and first history point. Legacy
    // Game::new defaults intentionally migrate on load and are not a byte-exact
    // roundtrip fixture for current campaign recovery.
    fn recovery_game() -> Game {
        let mut game = Game::new_fresh(1990, Some(crate::NationId::France));
        crate::fresh_play_rules(&mut game).unwrap();
        crate::resources::warm(&mut game.world);
        game.history.clear();
        game.snapshot();
        game
    }

    #[test]
    fn recovery_backup_only_default_and_named_slots_remain_discoverable() {
        let root = root();
        let mut game = recovery_game();
        game.record("Earlier recoverable point.".into());
        let old_world = crate::save(&game.world);
        let old_log = game.log.clone();
        for slot in ["default", "France-recovery"] { write(&root, slot, &game).unwrap(); }
        game.world.nation_mut(crate::NationId::France).gdp += 0.25;
        game.record("Newest save whose primary file is now missing.".into());
        for slot in ["default", "France-recovery"] {
            write(&root, slot, &game).unwrap();
            fs::remove_file(slot_path(&root, slot).unwrap()).unwrap();
        }
        let listing = list(&root);
        let rows = listing["slots"].as_array().unwrap();
        assert_eq!(rows.len(), 2, "Both backup-only slots must remain selectable");
        for slot in ["default", "France-recovery"] {
            let row = rows.iter().find(|row| row["slot"] == slot).unwrap();
            assert_eq!(row["current_exists"], false);
            assert_eq!(row["readable"], false, "No primary can be loaded");
            assert_eq!(row["backup"], true);
            assert_eq!(row["metadata_from_backup"], true);
            assert_eq!(row["player"], "France");
            assert_eq!(row["date"], game.world.date_str());
            assert!(row["bytes"].is_null());
            assert!(read(&root, slot, false).is_err());
            let recovered = read(&root, slot, true).unwrap();
            assert!(crate::save(&recovered.world) == old_world, "Recovered world must match the earlier point exactly");
            assert!(recovered.log == old_log, "Recovered log must match the earlier point exactly");
            assert_ne!(recovered.session_id, game.session_id);
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recovery_listing_uses_backup_metadata_without_claiming_backup_validity() {
        let root = root();
        let mut game = recovery_game();
        write(&root, "France-recovery", &game).unwrap();
        game.world.day = 2;
        game.record("A later primary.".into());
        write(&root, "France-recovery", &game).unwrap();
        let primary = slot_path(&root, "France-recovery").unwrap();
        fs::write(&primary, b"{interrupted").unwrap();
        fs::write(root.join("saves/OnlyDamage.json.bak"), b"{broken backup").unwrap();
        // These names cannot be selected by slot_path and must not appear as
        // misleading options (including the reserved default alias in saves/).
        for name in ["bad.name.json", "bad.name.json.bak", ".json", "default.json", "default.json.bak"] {
            fs::write(root.join("saves").join(name), encode(&game).unwrap()).unwrap();
        }
        let listing = list(&root);
        let rows = listing["slots"].as_array().unwrap();
        assert_eq!(rows.len(), 2, "Union only valid, addressable current/backup slot names");
        let row = rows.iter().find(|row| row["slot"] == "France-recovery").unwrap();
        assert_eq!(row["current_exists"], true);
        assert_eq!(row["readable"], false);
        assert_eq!(row["metadata_from_backup"], true);
        assert_eq!(row["date"], "1 Jan 1990");
        assert_eq!(row["player"], "France");
        let broken = rows.iter().find(|row| row["slot"] == "OnlyDamage").unwrap();
        assert_eq!(broken["backup"], true, "Existence is not a full validity claim");
        assert_eq!(broken["readable"], false);
        assert_eq!(broken["metadata_from_backup"], false);
        assert!(broken["date"].is_null());
        assert!(read(&root, "OnlyDamage", true).is_err());
        // A parseable primary keeps precedence; listing is not full decode.
        fs::write(&primary, br#"{"format":"future-unknown","saved_date":"primary label","player":"primary label"}"#).unwrap();
        let listing = list(&root);
        let row = listing["slots"].as_array().unwrap().iter().find(|row| row["slot"] == "France-recovery").unwrap();
        assert_eq!(row["readable"], true);
        assert_eq!(row["metadata_from_backup"], false);
        assert_eq!(row["date"], "primary label");
        assert!(read(&root, "France-recovery", false).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recovery_timestamp_only_repeated_save_preserves_the_prior_point() {
        let root = root();
        let mut game = recovery_game();
        write(&root, "default", &game).unwrap();
        let path = slot_path(&root, "default").unwrap();
        let previous = fs::read(&path).unwrap();
        game.world.nation_mut(crate::NationId::France).gdp += 0.25;
        game.record("Current point with an interior saved_unix: 9 and a quoted \"saved_unix\" label.".into());
        write(&root, "default", &game).unwrap();
        assert!(fs::read(path.with_extension("json.bak")).unwrap() == previous, "Changed state must retain the prior archive bytes exactly");
        let text = fs::read_to_string(&path).unwrap();
        let (prefix, _) = text.rsplit_once(",\"saved_unix\":").unwrap();
        let timestamp_only = format!("{prefix},\"saved_unix\":0}}");
        assert!(decode(&timestamp_only).is_ok());
        fs::write(&path, timestamp_only).unwrap();
        let world = crate::save(&game.world);
        let history = game.history.clone();
        let log = game.log.clone();
        for _ in 0..2 { write(&root, "default", &game).unwrap(); }
        assert!(fs::read(path.with_extension("json.bak")).unwrap() == previous,
            "A lost save response followed by retry cannot replace the earlier recovery point");
        let restored = read(&root, "default", false).unwrap();
        assert!(crate::save(&restored.world) == world, "Repeated save must roundtrip the full world exactly");
        assert!(restored.history == history, "Repeated save must roundtrip all history exactly");
        assert!(restored.log == log, "Repeated save must roundtrip the complete log exactly");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recovery_same_day_world_history_log_and_journey_changes_still_rotate() {
        let root = root();
        let mut game = recovery_game();
        write(&root, "default", &game).unwrap();
        let path = slot_path(&root, "default").unwrap();
        let date = game.world.date_str();
        for change in 0..5 {
            let previous = fs::read(&path).unwrap();
            match change {
                0 => game.world.nation_mut(crate::NationId::France).gdp += 0.25,
                1 => game.record("A same-day dispatch containing \"saved_unix\":123.".into()),
                2 => game.history[0].oil += 0.5,
                3 => game.history_epoch += 1,
                _ => game.journey.observing = true,
            }
            assert_eq!(game.world.date_str(), date);
            write(&root, "default", &game).unwrap();
            assert!(fs::read(path.with_extension("json.bak")).unwrap() == previous,
                "Same-day change {change} must create a distinct recovery point");
            let restored = read(&root, "default", false).unwrap();
            assert!(crate::save(&restored.world) == crate::save(&game.world), "Same-day change {change} must roundtrip the full world exactly");
            assert!(restored.history == game.history, "Same-day change {change} must roundtrip all history exactly");
            assert!(restored.log == game.log, "Same-day change {change} must roundtrip the complete log exactly");
            assert_eq!(restored.history_epoch, game.history_epoch);
            assert!(restored.journey == game.journey, "Same-day change {change} must roundtrip the complete journey exactly");
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recovery_noncanonical_and_legacy_valid_saves_keep_normal_rotation() {
        let root = root();
        let game = recovery_game();
        let path = slot_path(&root, "default").unwrap();
        let compact = encode(&game).unwrap();
        let value: Value = serde_json::from_str(&compact).unwrap();
        for text in [format!("{compact}\n"), serde_json::to_string_pretty(&value).unwrap(),
            serde_json::to_string(&value).unwrap(), crate::save(&game.world)] {
            assert!(decode(&text).is_ok());
            fs::write(&path, &text).unwrap();
            write(&root, "default", &game).unwrap();
            assert!(fs::read(path.with_extension("json.bak")).unwrap() == text.as_bytes(),
                "Conservative duplicate detection must not canonicalize legacy or reordered input");
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recovery_invalid_primary_never_replaces_the_retained_valid_backup() {
        let root = root();
        let mut game = recovery_game();
        write(&root, "default", &game).unwrap();
        let path = slot_path(&root, "default").unwrap();
        let good = fs::read(&path).unwrap();
        game.world.nation_mut(crate::NationId::France).gdp += 0.25;
        write(&root, "default", &game).unwrap();
        for bad in ["{truncated".to_owned(), encode(&game).unwrap().replacen("\"version\":1", "\"version\":999", 1)] {
            assert!(decode(&bad).is_err());
            fs::write(&path, bad).unwrap();
            write(&root, "default", &game).unwrap();
            assert!(fs::read(path.with_extension("json.bak")).unwrap() == good, "Invalid primary must preserve the valid backup bytes exactly");
            assert!(crate::save(&read(&root, "default", false).unwrap().world) == crate::save(&game.world), "Replacement primary must roundtrip the full world exactly");
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recovery_preexisting_temporary_candidate_is_not_ours_to_delete() {
        let root = root();
        let candidate = root.join("save.tmp-collision");
        let original = b"Earlier interrupted operation's candidate";
        fs::write(&candidate, original).unwrap();
        let error = stage_at(candidate.clone(), |_| panic!("Writer must not run after create_new fails")).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read(&candidate).ok().as_deref(), Some(original.as_slice()),
            "Failed exclusive creation cannot delete a pre-existing file");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recovery_backup_promotion_failure_keeps_primary_and_cleans_owned_temps() {
        let root = root();
        let mut game = recovery_game();
        write(&root, "default", &game).unwrap();
        let path = slot_path(&root, "default").unwrap();
        let primary = fs::read(&path).unwrap();
        let backup = path.with_extension("json.bak");
        fs::create_dir(&backup).unwrap();
        let sentinel = backup.join("keep.txt");
        fs::write(&sentinel, b"Directory blocks backup rename").unwrap();
        game.record("Would be a new save, but backup promotion fails.".into());
        assert!(write(&root, "default", &game).is_err());
        assert!(fs::read(&path).unwrap() == primary, "Failed backup promotion must leave primary bytes unchanged");
        assert_eq!(fs::read(&sentinel).unwrap(), b"Directory blocks backup rename");
        assert!(!fs::read_dir(&root).unwrap().any(|e| e.unwrap().file_name().to_string_lossy().contains("tmp-")));
        assert!(read(&root, "default", false).is_ok());
        fs::remove_dir_all(root).unwrap();
    }


    #[test]
    fn recovery_first_repeated_save_establishes_backup_then_keeps_it() {
        let root = root();
        let game = recovery_game();
        write(&root, "default", &game).unwrap();
        let path = slot_path(&root, "default").unwrap();
        let backup = path.with_extension("json.bak");
        assert!(!backup.exists());
        let initial = fs::read_to_string(&path).unwrap();
        let (prefix, _) = initial.rsplit_once(",\"saved_unix\":").unwrap();
        let first = format!("{prefix},\"saved_unix\":0}}");
        fs::write(&path, &first).unwrap();
        write(&root, "default", &game).unwrap();
        assert!(fs::read(&backup).unwrap() == first.as_bytes(),
            "The first retry can establish a backup when none exists");
        let second = format!("{prefix},\"saved_unix\":1}}");
        fs::write(&path, second).unwrap();
        write(&root, "default", &game).unwrap();
        assert!(fs::read(&backup).unwrap() == first.as_bytes(),
            "Further identical saves keep the established recovery copy");
        assert!(crate::save(&read(&root, "default", true).unwrap().world) == crate::save(&game.world), "First repeated-save backup must roundtrip the full world exactly");
        fs::remove_dir_all(root).unwrap();
    }

}
