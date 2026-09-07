//! Person/era-bound character art. Never falls back to a national figure.
use serde_json::{json, Value};
use std::sync::OnceLock;
use spheres_sim::{party_leadership, world::{NationId, WorldState}};

pub(crate) struct Asset { pub bytes: &'static [u8], pub content_type: &'static str }
pub(crate) fn asset(name: &str) -> Option<Asset> {
    let bytes: &'static [u8] = match name {
        "margaret_thatcher_1990_v1.glb" => include_bytes!("../ui/person-models/margaret_thatcher_1990_v1.glb"),
        "neil_kinnock_1990_v1.glb" => include_bytes!("../ui/person-models/neil_kinnock_1990_v1.glb"),
        "paddy_ashdown_1990_v1.glb" => include_bytes!("../ui/person-models/paddy_ashdown_1990_v1.glb"),
        "george_h_w_bush_1990_v1.glb" => include_bytes!("../ui/person-models/george_h_w_bush_1990_v1.glb"),
        _ => return None,
    };
    Some(Asset {bytes, content_type:"model/gltf-binary"})
}

pub(crate) const MODEL_DATA: &str = include_str!("../data/person_models.json");
fn models() -> &'static Value {
    static DATA: OnceLock<Value> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(MODEL_DATA).expect("validated character model catalogue"))
}

fn character(person_id: &str, date: &str) -> Value {
    let Some(records) = models()["characters"].as_array() else { return Value::Null };
    let matches = records.iter().filter(|p| p["person_id"] == person_id
        && p["from"].as_str().is_some_and(|s| exact_date(s) && s <= date)
        && p["to"].as_str().is_some_and(|s| exact_date(s) && date < s)).collect::<Vec<_>>();
    if matches.len() != 1 { return Value::Null }
    let p=matches[0];
    let Some(id)=p["id"].as_str() else { return Value::Null };
    let file=format!("{id}.glb");
    if asset(&file).is_none() { return Value::Null }
    json!({"model_id":id,"download_url":format!("/art/people/{file}"),"method":"procedural_3d","status":p["status"],"from":p["from"],"to":p["to"],"credit":p["credit"],"source_url":p["sources"][0],"composition":"full-body","era_note":"Early-1990s 3D likeness study. Model appearance does not determine office or party membership."})
}

fn manifest() -> &'static Value {
    static DATA: OnceLock<Value> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(include_str!("../data/person_portraits.json")).expect("validated person avatar manifest"))
}

fn exact_date(date: &str) -> bool {
    spheres_sim::data::parse_date(date).is_some_and(|(y,m,d)| {
        date == format!("{y:04}-{m:02}-{d:02}") && (1..=12).contains(&m) && (1..=spheres_sim::world::days_in_month(y,m)).contains(&d)
    })
}

pub(crate) fn portrait(person_id: &str, date: &str) -> Value {
    if !exact_date(date) { return Value::Null; }
    let model=character(person_id,date);
    if !model.is_null() { return model; }
    let Some(records) = manifest()["people"][person_id]["portraits"].as_array() else { return Value::Null };
    let matches = records.iter().filter(|p| {
        p["identity_source"]["person_id"] == person_id
            && ["identity","likeness","era","visual"].iter().all(|key| p["review"][*key] == true)
            && p["from"].as_str().is_some_and(|start| exact_date(start) && start <= date)
            && (p["to"].is_null() || p["to"].as_str().is_some_and(|end| exact_date(end) && date < end))
    }).collect::<Vec<_>>();
    if matches.len() != 1 { return Value::Null }
    let p = matches[0];
    let Some(name) = p["asset"].as_str().and_then(|s| s.strip_prefix("spheres-web/ui/person-portraits/")) else { return Value::Null };
    if asset(name).is_none() { return Value::Null }
    json!({"url":format!("/art/people/{name}"),"credit":p["credit"],"method":p["method"],"from":p["from"],"to":p["to"],"source_url":p["source_url"],"license":p["license"],"license_url":p["license_url"],"source_license":p["identity_source"]["license"],"source_license_url":p["identity_source"]["license_url"],"source_credit":p["identity_source"]["credit"],"composition":p["composition"],"era_note":p["era_note"]})
}

fn decorate(value: &mut Value, date: &str) {
    match value {
        Value::Array(items) => for item in items { decorate(item,date); },
        Value::Object(obj) => {
            for item in obj.values_mut() { decorate(item,date); }
            if let (Some(id), Some(_name)) = (obj.get("id").and_then(Value::as_str), obj.get("name").and_then(Value::as_str)) {
                if party_leadership::person(id).is_some() {
                    let art = portrait(id,date);
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
        for party in parties { party["campaign"] = json!([]); party["status"] = json!("reference_only"); }
    }
    decorate(&mut value,date);
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn avatars_are_exact_people_with_half_open_eras_and_no_national_fallback() {
        assert_eq!(portrait("margaret_thatcher","1990-01-01")["model_id"],"margaret_thatcher_1990_v1");
        assert_eq!(portrait("margaret_thatcher","1994-12-31")["method"],"procedural_3d");
        assert!(portrait("margaret_thatcher","1995-01-01").is_null());
        assert!(portrait("margaret_thatcher","1989-12-31").is_null());
        assert!(portrait("UK","1990-01-01").is_null());
        assert!(portrait("margaret_thatcher","1990-02-30").is_null());
        assert!(asset("../person_portraits.json").is_none());
    }
    #[test]
    fn historical_browsing_is_read_only_and_retains_every_party() {
        let w=spheres_sim::init::world_1990(spheres_sim::world::GameRules::default());
        let before=serde_json::to_string(&w).unwrap();
        let view=reference_view(&w,NationId::UK,"1990-01-01").unwrap();
        assert_eq!(view["parties"].as_array().unwrap().len(),4);
        assert_eq!(serde_json::to_string(&w).unwrap(),before);
        assert!(reference_view(&w,NationId::UK,"2026-12-31").is_err());
        assert!(reference_view(&w,NationId::UK,"1989-12-31").is_err());
    }
    #[test]
    fn registered_characters_exist_and_have_source_and_visual_review() {
        for (id,p) in manifest()["people"].as_object().unwrap() {
            let person=party_leadership::person(id).expect("known exact person");
            assert_eq!(p["name"],person.name);
            for art in p["portraits"].as_array().unwrap() {
                assert!(!portrait(id,art["from"].as_str().unwrap()).is_null(),"unserved or unreviewed avatar {id}");
                assert_eq!(art["composition"],"full-body");
            }
        }
        let mut seen=std::collections::BTreeSet::new();
        for model in models()["characters"].as_array().unwrap() {
            let id=model["person_id"].as_str().unwrap();
            assert_eq!(model["name"],party_leadership::person(id).unwrap().name);
            assert!(seen.insert(model["id"].as_str().unwrap()));
            let art=portrait(id,model["from"].as_str().unwrap());
            assert_eq!(art["model_id"],model["id"]);
            assert_eq!(art["status"],"likeness-study");
            assert!(portrait(id,model["to"].as_str().unwrap()).is_null());
            let bytes=asset(&format!("{}.glb",model["id"].as_str().unwrap())).unwrap().bytes;
            assert_eq!(&bytes[..4],b"glTF");
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
