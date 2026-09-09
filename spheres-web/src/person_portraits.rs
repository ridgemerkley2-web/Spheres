//! Person/era-bound character art. Never falls back to a national figure.
use serde_json::{json, Value};
use std::sync::OnceLock;
use spheres_sim::{party_leadership, world::{NationId, WorldState}};

include!("person_avatar_assets.rs");

pub(crate) struct Asset { pub bytes: &'static [u8], pub content_type: &'static str }
pub(crate) fn asset(name: &str) -> Option<Asset> {
    if let Some(bytes) = cartoon_asset(name) { return Some(Asset {bytes, content_type:"image/png"}) }
    let bytes: &'static [u8] = match name {
        "margaret_thatcher_1990_v2.glb" => include_bytes!("../ui/person-models/margaret_thatcher_1990_v2.glb"),
        "neil_kinnock_1990_v2.glb" => include_bytes!("../ui/person-models/neil_kinnock_1990_v2.glb"),
        "paddy_ashdown_1990_v2.glb" => include_bytes!("../ui/person-models/paddy_ashdown_1990_v2.glb"),
        "george_h_w_bush_1990_v2.glb" => include_bytes!("../ui/person-models/george_h_w_bush_1990_v2.glb"),
        // Retain the previous immutable URLs for already-open character viewers.
        "margaret_thatcher_1990_v1.glb" => include_bytes!("../ui/person-models/margaret_thatcher_1990_v1.glb"),
        "neil_kinnock_1990_v1.glb" => include_bytes!("../ui/person-models/neil_kinnock_1990_v1.glb"),
        "paddy_ashdown_1990_v1.glb" => include_bytes!("../ui/person-models/paddy_ashdown_1990_v1.glb"),
        "george_h_w_bush_1990_v1.glb" => include_bytes!("../ui/person-models/george_h_w_bush_1990_v1.glb"),
        _ => return None,
    };
    Some(Asset {bytes, content_type:"model/gltf-binary"})
}

pub(crate) const MODEL_DATA: &str = include_str!("../data/person_models.json");
#[cfg(test)]
fn models() -> &'static Value {
    static DATA: OnceLock<Value> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(MODEL_DATA).expect("validated character model catalogue"))
}

fn manifest() -> &'static Value {
    static DATA: OnceLock<Value> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(include_str!("../data/person_portraits.json")).expect("validated person avatar manifest"))
}

fn fictional_manifest() -> &'static Value {
    static DATA: OnceLock<Value> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(include_str!("../data/fictional_portraits.json")).expect("validated fictional design manifest"))
}

fn exact_date(date: &str) -> bool {
    spheres_sim::data::parse_date(date).is_some_and(|(y,m,d)| {
        date == format!("{y:04}-{m:02}-{d:02}") && (1..=12).contains(&m) && (1..=spheres_sim::world::days_in_month(y,m)).contains(&d)
    })
}

pub(crate) fn portrait(person_id: &str, date: &str) -> Value {
    if !exact_date(date) { return Value::Null; }
    if party_leadership::fictional_person(person_id).is_some() {
        return fictional_portrait_from(fictional_manifest(),person_id,date);
    }
    // Active presentation is illustrated art. Archived character models remain
    // downloadable, but never substitute for a reviewed person/era avatar.
    let Some(records) = manifest()["people"][person_id]["portraits"].as_array() else { return Value::Null };
    let matches = records.iter().filter(|p| {
        p["identity_source"]["person_id"] == person_id
            && p["style"] == "cartoon" && p["method"] == "generated" && p["status"] == "illustrated-likeness"
            && ["identity","likeness","era","visual"].iter().all(|key| p["review"][*key] == true)
            && p["from"].as_str().is_some_and(|start| exact_date(start) && start <= date)
            && (p["to"].is_null() || p["to"].as_str().is_some_and(|end| exact_date(end) && date < end))
    }).collect::<Vec<_>>();
    if matches.len() != 1 { return Value::Null }
    let p = matches[0];
    let Some(name) = p["asset"].as_str().and_then(|s| s.strip_prefix("spheres-web/ui/person-portraits/")) else { return Value::Null };
    if !asset(name).is_some_and(|a| a.content_type == "image/png") { return Value::Null }
    json!({"url":format!("/art/people/{name}"),"credit":p["credit"],"method":p["method"],"style":p["style"],"status":p["status"],"from":p["from"],"to":p["to"],"source_url":p["source_url"],"license":p["license"],"license_url":p["license_url"],"source_license":p["identity_source"]["license"],"source_license_url":p["identity_source"]["license_url"],"source_credit":p["identity_source"]["credit"],"composition":p["composition"],"era_note":p["era_note"]})
}

fn fictional_portrait_from(manifest: &Value, person_id: &str, date: &str) -> Value {
    if !exact_date(date) || date < "2026-09-08" || date >= "2036-01-01" { return Value::Null }
    let Some(candidate) = party_leadership::fictional_person(person_id) else { return Value::Null };
    let person = &manifest["people"][person_id];
    if person["name"] != candidate.person.name.as_str() || person["appearance_seed"] != candidate.appearance_seed.as_str() { return Value::Null }
    let Some(records) = person["portraits"].as_array() else { return Value::Null };
    let matches = records.iter().filter(|p| {
        p["design_source"]["kind"] == "authored_fiction"
            && p["design_source"]["person_id"] == person_id
            && p["design_source"]["appearance_seed"] == candidate.appearance_seed.as_str()
            && p["design_source"]["catalog"] == "spheres-web/data/future_candidates_2035.json"
            && p["style"] == "cartoon" && p["method"] == "generated" && p["status"] == "fictional-character"
            && p["review"]["design"] == true && p["review"]["visual"] == true
            && p["identity_source"].is_null() && p["source_url"].is_null()
            && ["identity","likeness","era"].iter().all(|key| p["review"][*key].is_null())
            && p["from"].as_str().is_some_and(|start| exact_date(start) && start >= "2026-09-08" && start <= date)
            && p["to"].as_str().is_some_and(|end| exact_date(end) && end <= "2036-01-01" && date < end)
    }).collect::<Vec<_>>();
    if matches.len() != 1 { return Value::Null }
    let p = matches[0];
    let Some(name) = p["asset"].as_str().and_then(|s| s.strip_prefix("spheres-web/ui/person-portraits/")) else { return Value::Null };
    if !asset(name).is_some_and(|a| a.content_type == "image/png") { return Value::Null }
    json!({"url":format!("/art/people/{name}"),"credit":p["credit"],"method":p["method"],"style":p["style"],"status":p["status"],"from":p["from"],"to":p["to"],"license":p["license"],"composition":p["composition"],"era_note":p["era_note"],"origin":"fictional_successor"})
}

fn decorate(value: &mut Value, date: &str) {
    decorate_context(value,date,false);
}

fn decorate_context(value: &mut Value, date: &str, future_preview: bool) {
    match value {
        Value::Array(items) => for item in items { decorate_context(item,date,future_preview); },
        Value::Object(obj) => {
            for (key,item) in obj.iter_mut() { decorate_context(item,date,future_preview || key == "future_preview"); }
            if let (Some(id), Some(_name)) = (obj.get("id").and_then(Value::as_str), obj.get("name").and_then(Value::as_str)) {
                if party_leadership::person(id).is_some() {
                    let is_future = future_preview && party_leadership::fictional_person(id).is_some();
                    let mut art = portrait(id,if is_future { "2026-09-08" } else { date });
                    if is_future && !art.is_null() {
                        art["presentation_context"] = json!("fictional_future_preview");
                        art["preview_date"] = json!("2026-09-08");
                    }
                    obj.insert("portrait".into(),art);
                }
            }
        },
        _ => {},
    }
}

pub(crate) fn campaign_view(w: &WorldState, nation: NationId) -> Value {
    let mut value = party_leadership::view(w,nation);
    // An old campaign may show its original executive's explicitly linked art
    // without enabling or inventing saved party succession. Never rebind a
    // successor: an intact named original, exact office start and identity are
    // all required. This is a read-only presentation reference.
    if !w.rules.historical_party_leadership {
        if let (Some(offices),Ok(roster)) = (&w.leadership,party_leadership::roster()) {
            if let Some(office)=offices.iter().find(|o| o.nation==nation && o.emergent.is_none() && o.name.is_some()) {
                if let Some(link)=roster.office_links.iter().find(|l| l.nation==nation && l.since==office.since) {
                    if let Some(person)=party_leadership::person(&link.person) {
                        if office.name.as_deref()==Some(person.name.as_str()) || office.name.as_deref()==person.native.as_deref() {
                            value["executive_person"]=json!(person);
                        }
                    }
                }
            }
        }
    }
    let date=value["date"].as_str().unwrap_or("").to_string();
    decorate(&mut value,&date);
    value
}

pub(crate) fn reference_view(w: &WorldState, nation: NationId, date: &str) -> Result<Value,String> {
    if !exact_date(date) { return Err("Choose a valid calendar date.".into()) }
    let roster = party_leadership::roster().map_err(str::to_string)?;
    if date < roster.reference_from.as_str() || date > roster.reference_through.as_str() {
        return Err(format!("Historical reference dates run from {} through {}.",roster.reference_from,roster.reference_through))
    }
    let mut value = party_leadership::reference_view(w,nation,date);
    if let Some(error) = value["error"].as_str() { return Err(error.to_string()) }
    // A reference lookup carries no current campaign holders or forecasts.
    value["executive_person"] = Value::Null;
    value["enabled"] = json!(false);
    value["eligibility_context"] = json!("historical_reference");
    if let Some(parties) = value["parties"].as_array_mut() {
        for party in parties {
            party["campaign"] = json!([]);
            party["future_candidates"] = json!([]);
            party["future_preview"] = json!([]);
            party["status"] = json!("reference_only");
        }
    }
    decorate(&mut value,date);
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn avatars_are_exact_people_with_half_open_eras_and_no_national_fallback() {
        for (id,file) in [
            ("margaret_thatcher","margaret-thatcher-cartoon-1990-v3.png"),
            ("neil_kinnock","neil-kinnock-cartoon-1990-v1.png"),
            ("paddy_ashdown","paddy-ashdown-cartoon-1990-v1.png"),
            ("george_h_w_bush","george-h-w-bush-cartoon-1990-v1.png"),
        ] {
            let art=portrait(id,"1990-01-01");
            assert_eq!(art["url"],format!("/art/people/{file}"));
            assert_eq!(art["method"],"generated");
            assert_eq!(art["style"],"cartoon");
            assert_eq!(art["status"],"illustrated-likeness");
            assert!(art["model_id"].is_null());
            assert_eq!(portrait(id,"1994-12-31"),art);
            assert!(portrait(id,"1995-01-01").is_null());
            assert!(portrait(id,"1989-12-31").is_null());
        }
        assert!(portrait("UK","1990-01-01").is_null());
        assert!(portrait("USA","1990-01-01").is_null());
        assert!(portrait("unknown_person","1990-01-01").is_null());
        assert!(portrait("margaret_thatcher","1990-02-30").is_null());
        assert!(asset("../person_portraits.json").is_none());
        assert!(asset("margaret-thatcher-character-1990-v1.png").is_none());
    }
    #[test]
    fn historical_browsing_is_read_only_and_retains_every_party() {
        let w=spheres_sim::init::world_1990(spheres_sim::world::GameRules::default());
        let before=serde_json::to_string(&w).unwrap();
        let view=reference_view(&w,NationId::UK,"1990-01-01").unwrap();
        assert_eq!(view["parties"].as_array().unwrap().len(),4);
        assert!(view["parties"].as_array().unwrap().iter().all(|p| p["future_preview"].as_array().unwrap().is_empty() && p["future_candidates"].as_array().unwrap().is_empty()));
        assert_eq!(serde_json::to_string(&w).unwrap(),before);
        assert!(reference_view(&w,NationId::UK,"2026-12-31").is_err());
        assert!(reference_view(&w,NationId::UK,"1989-12-31").is_err());
    }
    #[test]
    fn fictional_designs_require_exact_catalogue_identity_and_future_eras() {
        let candidate=&party_leadership::future_catalog()[0];
        let id=&candidate.person.id;
        // An existing allowlisted PNG is only a selector-test fixture. The
        // physical art validator separately rejects historical-image reuse.
        let record=json!({"from":"2026-09-08","to":"2036-01-01","method":"generated","style":"cartoon","status":"fictional-character",
            "asset":"spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png",
            "design_source":{"kind":"authored_fiction","person_id":id,"appearance_seed":candidate.appearance_seed,"catalog":"spheres-web/data/future_candidates_2035.json"},
            "review":{"design":true,"visual":true},"credit":"Synthetic fixture, not game artwork."});
        let mut test_manifest=json!({"version":1,"people":{}});
        test_manifest["people"][id]=json!({"name":candidate.person.name,"appearance_seed":candidate.appearance_seed,"portraits":[record]});
        let valid=fictional_portrait_from(&test_manifest,id,"2026-09-08");
        assert_eq!(valid["status"],"fictional-character");
        assert_eq!(valid["origin"],"fictional_successor");
        assert!(valid["source_url"].is_null());
        for date in ["1990-01-01","2026-09-07","2036-01-01","2030-02-30"] {
            assert!(fictional_portrait_from(&test_manifest,id,date).is_null(),"{date}");
        }
        assert!(!fictional_portrait_from(&test_manifest,id,"2035-12-31").is_null());
        assert!(fictional_portrait_from(&test_manifest,"margaret_thatcher","2030-01-01").is_null());
        let mut bad=test_manifest.clone();bad["people"][id]["appearance_seed"]=json!("another-design");
        assert!(fictional_portrait_from(&bad,id,"2030-01-01").is_null());
        let mut bad=test_manifest.clone();bad["people"][id]["name"]=json!("Another person");
        assert!(fictional_portrait_from(&bad,id,"2030-01-01").is_null());
        let mut bad=test_manifest.clone();bad["people"][id]["portraits"][0]["review"]["likeness"]=json!(true);
        assert!(fictional_portrait_from(&bad,id,"2030-01-01").is_null());
        let mut bad=test_manifest.clone();bad["people"][id]["portraits"][0]["review"]["design"]=json!(false);
        assert!(fictional_portrait_from(&bad,id,"2030-01-01").is_null());
        let mut bad=test_manifest.clone();let duplicate=bad["people"][id]["portraits"][0].clone();
        bad["people"][id]["portraits"].as_array_mut().unwrap().push(duplicate);
        assert!(fictional_portrait_from(&bad,id,"2030-01-01").is_null());
    }
    #[test]
    fn registered_fictional_art_only_decorates_explicit_future_previews_before_cutoff() {
        for (id,person) in fictional_manifest()["people"].as_object().unwrap() {
            let candidate=party_leadership::fictional_person(id).expect("known fictional person");
            assert_eq!(person["name"],candidate.person.name);
            assert!(manifest()["people"].get(id).is_none(),"fiction must stay outside historical portrait records");
            for art in person["portraits"].as_array().unwrap() {
                let first=art["from"].as_str().unwrap();
                assert_eq!(portrait(id,first)["status"],"fictional-character");
                assert!(portrait(id,"2026-09-07").is_null());
                assert!(portrait(id,"2036-01-01").is_null());
            }
            let mut value=json!({"campaign":[{"person":party_leadership::person_view(id)}],"future_preview":[{"person":party_leadership::person_view(id)}]});
            decorate(&mut value,"1990-01-01");
            assert!(value["campaign"][0]["person"]["portrait"].is_null());
            let expected=portrait(id,"2026-09-08");
            let preview=&value["future_preview"][0]["person"]["portrait"];
            if expected.is_null() { assert!(preview.is_null()); }
            else { assert_eq!(preview["url"],expected["url"]);assert_eq!(preview["presentation_context"],"fictional_future_preview");assert_eq!(preview["preview_date"],"2026-09-08"); }
        }
    }
    #[test]
    fn registered_cartoons_exist_and_have_source_and_visual_review() {
        let mut registered=std::collections::BTreeSet::new();
        for (id,p) in manifest()["people"].as_object().unwrap() {
            let person=party_leadership::person(id).expect("known exact person");
            assert_eq!(p["name"],person.name);
            for art in p["portraits"].as_array().unwrap() {
                assert!(registered.insert((id.as_str(),art["from"].as_str().unwrap())),"duplicate active avatar era for {id}");
                assert!(!portrait(id,art["from"].as_str().unwrap()).is_null(),"unserved or unreviewed avatar {id}");
                assert_eq!(art["composition"],"full-body");
                assert_eq!(art["identity_source"]["person_id"],id.as_str());
                assert!(art["source_url"].as_str().is_some_and(|url| url.starts_with("https://")));
                assert!(art["credit"].as_str().is_some_and(|credit| !credit.is_empty()));
                let file=art["asset"].as_str().unwrap().strip_prefix("spheres-web/ui/person-portraits/").unwrap();
                let image=asset(file).expect("allowlisted cartoon asset");
                assert_eq!(image.content_type,"image/png");
                assert_eq!(&image.bytes[..8],b"\x89PNG\r\n\x1a\n");
            }
        }
        assert!(registered.len() >= 4, "reviewed pilot art must remain registered");
    }

    #[test]
    fn archived_models_remain_available_but_never_become_active_avatars() {
        let mut seen=std::collections::BTreeSet::new();
        for model in models()["characters"].as_array().unwrap() {
            let id=model["person_id"].as_str().unwrap();
            assert_eq!(model["name"],party_leadership::person(id).unwrap().name);
            assert!(seen.insert(model["id"].as_str().unwrap()));
            let art=portrait(id,model["from"].as_str().unwrap());
            assert!(art["model_id"].is_null());
            assert!(art["download_url"].is_null());
            assert_eq!(art["style"],"cartoon");
            assert!(portrait(id,model["to"].as_str().unwrap()).is_null());
            let archive=asset(&format!("{}.glb",model["id"].as_str().unwrap())).unwrap();
            assert_eq!(archive.content_type,"model/gltf-binary");
            assert_eq!(&archive.bytes[..4],b"glTF");
        }
    }

    #[test]
    fn original_executive_art_is_read_only_in_old_campaigns_and_never_survives_succession() {
        let mut w=spheres_sim::init::world_1990(spheres_sim::world::GameRules::default());
        w.rules.ideology_blocs=true;
        spheres_sim::government::ensure_all(&mut w);
        let before=serde_json::to_string(&w).unwrap();
        assert_eq!(campaign_view(&w,NationId::USA)["executive_person"]["id"],"george_h_w_bush");
        assert_eq!(serde_json::to_string(&w).unwrap(),before);
        let office=w.leadership.as_mut().unwrap().iter_mut().find(|o|o.nation==NationId::USA).unwrap();
        office.name=None;
        assert!(campaign_view(&w,NationId::USA)["executive_person"].is_null());
    }
}
