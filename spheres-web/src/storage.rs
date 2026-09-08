//! Campaign files are presentation envelopes around the unchanged sim save.
//! Every replacement is staged and synced; the previous valid bytes remain in
//! a backup. Neither file time nor storage metadata enters WorldState.
use crate::{
    history::{Event, Snapshot},
    Game,
};
use serde::{de::{DeserializeSeed, MapAccess, SeqAccess, Visitor}, Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, Read, Write},
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
    let value: Value =
        serde_json::from_str(text).map_err(|e| format!("Cannot read campaign: {e}"))?;
    if value.get("format").is_none() || value["format"] == "spheres-equipment-save" {
        // The original CLI/browser format is still supported and uses every
        // simulation migration. It cannot invent an archive it never recorded.
        let mut g = crate::loaded_play_game(crate::load(text)?);
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
    let world = crate::load(&serde_json::to_string(&file.world).map_err(|e| e.to_string())?)?;
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

#[derive(Default)]
struct ListingMetadata {
    format_present: bool,
    saved_date: Value,
    player: Value,
}
#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ListingField { Format, SavedDate, Player, #[serde(other)] Other }

// Validate every value, but retain only the top-level listing fields. Using
// deserialize_any also preserves Value's number, Unicode and nesting checks;
// IgnoredAny's fast skip would accept some files the old listing rejected.
struct ListingSeed(bool);
impl<'de> DeserializeSeed<'de> for ListingSeed {
    type Value = ListingMetadata;
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for ListingSeed {
    type Value = ListingMetadata;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON value")
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> { Ok(ListingMetadata::default()) }
    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { Ok(ListingMetadata::default()) }
    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Ok(ListingMetadata::default()) }
    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Ok(ListingMetadata::default()) }
    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Ok(ListingMetadata::default()) }
    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Ok(ListingMetadata::default()) }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while seq.next_element_seed(ListingSeed(false))?.is_some() {}
        Ok(ListingMetadata::default())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut metadata = ListingMetadata::default();
        while let Some(field) = map.next_key::<ListingField>()? {
            match (self.0, field) {
                (true, ListingField::Format) => {
                    metadata.format_present = true;
                    map.next_value_seed(ListingSeed(false))?;
                }
                (true, ListingField::SavedDate) => metadata.saved_date = map.next_value()?,
                (true, ListingField::Player) => metadata.player = map.next_value()?,
                _ => { map.next_value_seed(ListingSeed(false))?; }
            }
        }
        Ok(metadata)
    }
}
fn listing_metadata(reader: impl Read) -> Result<ListingMetadata, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let metadata = ListingSeed(true).deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(metadata)
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
            let data = File::open(&path)
                .ok()
                .and_then(|file| listing_metadata(BufReader::with_capacity(64 * 1024, file)).ok());
            let envelope = data.as_ref().is_some_and(|v| v.format_present);
            json!({"slot":slot,"date":data.as_ref().map(|v|&v.saved_date),
            "player":data.as_ref().map(|v|&v.player),"legacy":!envelope,
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

    #[test]
    fn save_listing_preserves_metadata_and_full_json_readability() {
        let root=root();
        fs::create_dir_all(root.join("saves")).unwrap();
        let mut cases:Vec<(&str,Vec<u8>)>=vec![
            ("campaign",br#"{"format":"spheres-campaign","world":{"year":2035},"history":[],"log":[],"saved_date":"1 Jan 2035","player":"France"}"#.to_vec()),
            ("raw",br#"{"year":2035,"player":"USA"}"#.to_vec()),
            ("equipment",br#"{"format":"spheres-equipment-save","world":{}}"#.to_vec()),
            ("null-fields",br#"{"format":null,"saved_date":null,"player":null}"#.to_vec()),
            ("unusual-fields",br#"{"format":{"unknown":true},"saved_date":2035,"player":["France"]}"#.to_vec()),
            ("duplicate-fields",br#"{"saved_date":"old","player":"USA","format":false,"saved_date":"new","player":null}"#.to_vec()),
            ("nested-fields",br#"{"world":{"format":"nested","saved_date":"nested","player":"USA"}}"#.to_vec()),
            ("array",br#"[{"format":"nested"},null,1.5,true]"#.to_vec()),
            ("string",br#""legacy JSON scalar""#.to_vec()),
            ("number",b"123.5".to_vec()),
            ("boolean",b"true".to_vec()),
            ("null",b"null".to_vec()),
            ("whitespace",b" \r\n {\"player\":\"USA\"} \t\n".to_vec()),
            ("malformed-nested",br#"{"saved_date":"2035","world":[{"value":true,}]}"#.to_vec()),
            ("truncated",br#"{"format":"spheres-campaign","world":[1,2"#.to_vec()),
            ("trailing",br#"{"saved_date":"2035"} false"#.to_vec()),
            ("overflow",br#"{"world":[1e999]}"#.to_vec()),
            ("leading-zero",br#"{"world":[01]}"#.to_vec()),
            ("invalid-surrogate",br#"{"world":{"text":"\uD800"}}"#.to_vec()),
            ("invalid-utf8",b"{\"world\":\"\xff\"}".to_vec()),
        ];
        cases.push(("too-deep",format!("{{\"world\":{}0{}}}","[".repeat(130),"]".repeat(130)).into_bytes()));
        for (name,bytes) in &cases {fs::write(root.join("saves").join(format!("{name}.json")),bytes).unwrap();}
        let listing=list(&root);
        let slots=listing["slots"].as_array().unwrap();
        assert_eq!(slots.len(),cases.len());
        for (name,bytes) in &cases {
            // This is the previous listing contract, not load validation:
            // any JSON value is readable, and format presence marks envelopes.
            let previous=serde_json::from_slice::<Value>(bytes).ok();
            let slot=slots.iter().find(|s|s["slot"]==*name).unwrap();
            assert_eq!(slot["readable"],json!(previous.is_some()),"{name}");
            assert_eq!(slot["legacy"],json!(!previous.as_ref().is_some_and(|v|v.get("format").is_some())),"{name}");
            assert_eq!(slot["date"],previous.as_ref().and_then(|v|v.get("saved_date")).cloned().unwrap_or(Value::Null),"{name}");
            assert_eq!(slot["player"],previous.as_ref().and_then(|v|v.get("player")).cloned().unwrap_or(Value::Null),"{name}");
            assert_eq!(slot["bytes"],json!(bytes.len()),"{name}");
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn listing_metadata_handles_split_utf8_escapes_and_trailing_data() {
        let text=r#"{"world":{"text":"日本 \"archive\" \uD83C\uDF0D"},"saved_date":"1 Jan 2035","player":"日本","format":null}"#;
        let metadata=listing_metadata(BufReader::with_capacity(1,text.as_bytes())).unwrap();
        assert!(metadata.format_present);
        assert_eq!(metadata.saved_date,"1 Jan 2035");
        assert_eq!(metadata.player,"日本");
        let trailing=format!("{text} false");
        assert!(listing_metadata(BufReader::with_capacity(1,trailing.as_bytes())).is_err());
    }
}
