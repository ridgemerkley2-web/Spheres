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
    if party_leadership::fictional_person(person_id).is_some() || spheres_sim::institutional_leadership::candidate(person_id).is_some() {
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
    json!({"url":format!("/art/people/{name}"),"credit":p["credit"],"method":p["method"],"style":p["style"],"status":p["status"],"from":p["from"],"to":p["to"],"source_url":p["source_url"],"license":p["license"],"license_url":p["license_url"],"derivative_license":p["derivative_license"],"source_license":p["identity_source"]["license"],"source_license_url":p["identity_source"]["license_url"],"source_credit":p["identity_source"]["credit"],"composition":p["composition"],"era_note":p["era_note"]})
}

fn fictional_portrait_from(manifest: &Value, person_id: &str, date: &str) -> Value {
    if !exact_date(date) || date < "2026-09-08" || date >= "2036-01-01" { return Value::Null }
    let metadata=party_leadership::fictional_person(person_id).map(|c|(c.person.name.as_str(),c.appearance_seed.as_str()))
        .or_else(||spheres_sim::institutional_leadership::candidate(person_id).map(|c|(c.person.name.as_str(),c.appearance_seed.as_str())));
    let Some((candidate_name,appearance_seed))=metadata else {return Value::Null};
    let person = &manifest["people"][person_id];
    if person["name"] != candidate_name || person["appearance_seed"] != appearance_seed { return Value::Null }
    let Some(records) = person["portraits"].as_array() else { return Value::Null };
    let matches = records.iter().filter(|p| {
        p["design_source"]["kind"] == "authored_fiction"
            && p["design_source"]["person_id"] == person_id
            && p["design_source"]["appearance_seed"] == appearance_seed
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
                    let is_future = future_preview && (party_leadership::fictional_person(id).is_some() || spheres_sim::institutional_leadership::candidate(id).is_some());
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

/// Resolve the same saved executive used by government cards. Legacy saves
/// may reference only an intact, explicitly linked original office; a successor
/// or name-only resemblance never inherits that person's art.
fn campaign_person(w: &WorldState, nation: NationId) -> Option<&'static party_leadership::Person> {
    if !w.nation_opt(nation).is_some_and(|n| n.alive) { return None }
    if w.rules.historical_party_leadership {
        return party_leadership::executive_person(w,nation);
    }
    let offices=w.leadership.as_ref()?;
    let roster=party_leadership::roster().ok()?;
    let office=offices.iter().find(|o| o.nation==nation && o.emergent.is_none() && o.name.is_some())?;
    let link=roster.office_links.iter().find(|l| l.nation==nation && l.since==office.since)?;
    let person=party_leadership::person(&link.person)?;
    (office.name.as_deref()==Some(person.name.as_str()) || office.name.as_deref()==person.native.as_deref()).then_some(person)
}

/// Compact, read-only nation art contract for both the opening picker and live
/// nation surfaces. Identity follows the campaign; art follows its exact date.
/// No national figure or another era's portrait substitutes for missing art.
pub(crate) fn campaign_leader(w: &WorldState, nation: NationId) -> Value {
    let date=format!("{:04}-{:02}-{:02}",w.year,w.month,w.day);
    let leader=w.nation_opt(nation).filter(|n|n.alive).and_then(|_|spheres_sim::blocs::leader(w,nation));
    let person=leader.as_ref().and_then(|_|campaign_person(w,nation));
    let identity_status=if person.is_some() {"linked_person"}
        else if leader.as_ref().is_some_and(|l|l.name.is_some()) {"unlinked_person"}
        else if leader.is_some() {"institutional"} else {"unavailable"};
    json!({
        "date":date,
        "person_id":person.map(|p|p.id.as_str()),
        "origin":person.map(|p|if party_leadership::fictional_person(&p.id).is_some() {"fictional_successor"} else {"historical_person"}),
        "name":leader.as_ref().and_then(|l|l.name.as_deref()),
        "native":leader.as_ref().and_then(|l|l.native.as_deref()),
        "described":leader.as_ref().and_then(|l|l.described.as_deref()),
        "office":leader.as_ref().map(|l|l.office.as_str()),
        "since":leader.as_ref().and_then(|l|l.since.as_deref()),
        "portrait":person.map(|p|portrait(&p.id,&date)).unwrap_or(Value::Null),
        "identity_status":identity_status,
    })
}

pub(crate) fn campaign_view(w: &WorldState, nation: NationId) -> Value {
    let mut value = party_leadership::view(w,nation);
    // Use the same exact saved executive as the selector, including an inherited
    // Crown identity that has no party-office assignment. Keep fiction metadata.
    value["executive_person"]=campaign_person(w,nation)
        .map(|p|party_leadership::person_view(&p.id)).unwrap_or(Value::Null);
    let date=value["date"].as_str().unwrap_or("").to_string();
    decorate(&mut value,&date);
    value["institutional_leadership"]=institutional_view(w,nation);
    value
}

pub(crate) fn institutional_view(w:&WorldState,nation:NationId)->Value {
    let mut value=spheres_sim::institutional_leadership::view(w,nation);
    if value.is_null() { return value; }
    let date=value["date"].as_str().unwrap_or("").to_string();
    decorate(&mut value,&date);
    if let Some(bindings)=value["historical_bindings"].as_array_mut() {
        for binding in bindings {
            let id=binding["person_id"].as_str().unwrap_or("");
            let mut person=party_leadership::person_view(id);
            // A portrait reference date is display context, never a tenure.
            let at=binding["appearance_date"].as_str()
                .or_else(||binding["holder"]["attested_on"].as_str())
                .or_else(||binding["holder"]["from"].as_str())
                .or_else(||binding["holder"]["attested_period"]["from"].as_str());
            let context=at.filter(|d|exact_date(d)).map(str::to_string);
            if let Some(at)=context.as_deref() {decorate(&mut person,at);}
            binding["person"]=person;
            binding["portrait_reference_date"]=json!(context);
            binding["portrait_date_is_tenure"]=json!(false);
        }
    }
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
    fn campaign_leader_uses_the_saved_executive_and_exact_era_without_mutation() {
        let mut w=spheres_sim::init::world_1990(spheres_sim::world::GameRules {
            ideology_blocs:true, historical_party_leadership:true, ..Default::default()
        });
        spheres_sim::government::ensure_all(&mut w);
        party_leadership::ensure_all(&mut w);
        let before=spheres_sim::save(&w);
        let uk=campaign_leader(&w,NationId::UK);
        assert_eq!(uk["date"],"1990-01-01");
        assert_eq!(uk["person_id"],"margaret_thatcher");
        assert_eq!(uk["name"],"Margaret Thatcher");
        assert_eq!(uk["identity_status"],"linked_person");
        assert_eq!(uk["origin"],"historical_person");
        assert_eq!(uk["portrait"]["url"],"/art/people/margaret-thatcher-cartoon-1990-v3.png");
        // France's national executive is the served incumbent,
        // not the separate Socialist Party leader or a national icon.
        assert_eq!(campaign_leader(&w,NationId::France)["person_id"],"francois_mitterrand");
        let _=reference_view(&w,NationId::UK,"1990-11-27").unwrap();
        assert_eq!(campaign_leader(&w,NationId::UK),uk);
        assert_eq!(spheres_sim::save(&w),before);
        // Date-only fixture: no election happened. The saved person remains,
        // but an expired portrait must not be shown as current-era artwork.
        w.year=1995;
        let before=spheres_sim::save(&w);
        let later=campaign_leader(&w,NationId::UK);
        assert_eq!(later["person_id"],"margaret_thatcher");
        assert_eq!(later["date"],"1995-01-01");
        assert!(later["portrait"].is_null());
        assert_eq!(spheres_sim::save(&w),before);
    }

    #[test]
    fn campaign_leader_follows_actual_succession_and_survives_save_reload() {
        let mut w=spheres_sim::init::world_1990(spheres_sim::world::GameRules {
            seed:13, ideology_blocs:true, historical_party_leadership:true, ..Default::default()
        });
        spheres_sim::government::ensure_all(&mut w);
        party_leadership::ensure_all(&mut w);
        // Authored future fixture, not a claim that simulated years elapsed.
        w.year=2027;
        spheres_sim::government::seat_office(&mut w,NationId::UK,&spheres_sim::government::Succession::Death);
        let before=spheres_sim::save(&w);
        let current=campaign_leader(&w,NationId::UK);
        let id=current["person_id"].as_str().expect("fixture selects a named fictional successor");
        assert_ne!(id,"margaret_thatcher");
        assert!(party_leadership::fictional_person(id).is_some());
        assert_eq!(current["origin"],"fictional_successor");
        assert_eq!(current["name"],party_leadership::person(id).unwrap().name);
        assert_ne!(current["portrait"]["url"],"/art/people/margaret-thatcher-cartoon-1990-v3.png");
        assert_eq!(campaign_leader(&spheres_sim::load(&before).unwrap(),NationId::UK),current);
        assert_eq!(spheres_sim::save(&w),before);
    }

    #[test]
    fn campaign_leader_never_borrows_identity_for_unknown_or_unseated_offices() {
        let mut w=spheres_sim::init::world_1990(spheres_sim::world::GameRules {
            ideology_blocs:true, ..Default::default()
        });
        spheres_sim::government::ensure_all(&mut w);
        assert_eq!(campaign_leader(&w,NationId::USA)["person_id"],"george_h_w_bush");
        let office=w.leadership.as_mut().unwrap().iter_mut().find(|o|o.nation==NationId::USA).unwrap();
        office.name=Some("Unlinked fixture incumbent".into());
        let before=spheres_sim::save(&w);
        let unknown=campaign_leader(&w,NationId::USA);
        assert_eq!(unknown["name"],"Unlinked fixture incumbent");
        assert_eq!(unknown["identity_status"],"unlinked_person");
        assert!(unknown["person_id"].is_null() && unknown["portrait"].is_null());
        let absent=campaign_leader(&w,NationId::Russia);
        assert_eq!(absent["identity_status"],"unavailable");
        assert!(absent["name"].is_null() && absent["person_id"].is_null() && absent["portrait"].is_null());
        assert_eq!(spheres_sim::save(&w),before);
        w.nation_mut(NationId::USA).alive=false;
        assert_eq!(campaign_leader(&w,NationId::USA)["identity_status"],"unavailable");
    }

    #[test]
    fn tupou_cartoon_exposes_its_artwork_license_with_exact_person_and_era() {
        let art=portrait("taufaahau_tupou_iv","1990-01-01");
        assert_eq!(art["url"],"/art/people/taufaahau-tupou-iv-cartoon-1990-v1.png");
        assert_eq!(art["method"],"generated");
        assert_eq!(art["license"],"generated");
        assert_eq!(art["derivative_license"],"CC BY-SA 4.0");
        assert_eq!(art["license_url"],"https://creativecommons.org/licenses/by-sa/4.0/");
        assert_eq!(art["source_license"],"CC BY-SA 4.0");
        assert_eq!(portrait("taufaahau_tupou_iv","1990-12-31"),art);
        assert!(portrait("taufaahau_tupou_iv","1989-12-31").is_null());
        let later=portrait("taufaahau_tupou_iv","1991-01-01");
        assert_eq!(later["url"],"/art/people/tonga-taufaahau-tupou-iv-cartoon-1998-v1.png");
        assert_eq!(later["from"],"1991-01-01");
        assert_eq!(later["to"],"2006-09-11");
        assert_eq!(portrait("taufaahau_tupou_iv","2006-09-10"),later);
        assert!(portrait("taufaahau_tupou_iv","2006-09-11").is_null());
        assert!(portrait("Tonga","1990-01-01").is_null());
    }
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
            let (name,seed)=party_leadership::fictional_person(id).map(|c|(c.person.name.as_str(),c.appearance_seed.as_str()))
                .or_else(||spheres_sim::institutional_leadership::candidate(id).map(|c|(c.person.name.as_str(),c.appearance_seed.as_str())))
                .expect("known exact fictional party or institutional person");
            assert_eq!(person["name"],name);
            assert_eq!(person["appearance_seed"],seed);
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
    fn tonga_institution_cards_keep_reference_civilian_and_crown_identities_separate() {
        let mut w=spheres_sim::init::world_1990(spheres_sim::world::GameRules{
            ideology_blocs:true,historical_party_leadership:true,daily_simulation:true,..Default::default()
        });
        party_leadership::ensure_all(&mut w);
        let saved=spheres_sim::save(&w);
        assert!(institutional_view(&w,NationId::France).is_null());
        let cards=institutional_view(&w,NationId::Tonga);
        assert_eq!(cards["prime_minister"]["person"]["id"],"fatafehi_tuipelehake");
        assert_eq!(cards["prime_minister"]["opening_reference"],true);
        assert!(cards["prime_minister"]["appointment"]["selected_on"].is_null());
        assert_eq!(cards["future_preview"].as_array().unwrap().len(),4);
        let restricted=cards["future_preview"].as_array().unwrap().iter().find(|c|c["person"]["id"]=="fictional_to_kalolo_matalehu").unwrap();
        assert!(restricted["actions"].as_array().unwrap().is_empty());
        assert_eq!(campaign_leader(&w,NationId::Tonga)["person_id"],"taufaahau_tupou_iv");
        assert_eq!(spheres_sim::save(&w),saved);
        spheres_sim::government::seat_office(&mut w,NationId::Tonga,&spheres_sim::government::Succession::Death);
        assert_eq!(campaign_leader(&w,NationId::Tonga)["person_id"],"siaosi_taufaahau_manumataongo");
        assert_eq!(campaign_leader(&w,NationId::Tonga)["office"],"King");
        let after_succession=spheres_sim::save(&w);
        let campaign=campaign_view(&w,NationId::Tonga);
        assert_eq!(campaign["executive_person"]["id"],"siaosi_taufaahau_manumataongo");
        assert_eq!(campaign["executive_person"]["portrait"]["url"],
            "/art/people/tonga-george-tupou-v-cartoon-1990-v1.png");
        let government=super::super::government_json(&w,NationId::Tonga);
        assert_eq!(government["party_leadership"]["executive_person"],campaign["executive_person"]);
        assert_eq!(spheres_sim::save(&w),after_succession);
    }
    #[test]
    fn tonga_king_iv_reference_portrait_uses_assent_context_without_inventing_tenure() {
        let w=spheres_sim::init::world_1990(spheres_sim::world::GameRules{
            ideology_blocs:true,historical_party_leadership:true,daily_simulation:true,..Default::default()
        });
        let saved=spheres_sim::save(&w);
        let raw=spheres_sim::institutional_leadership::view(&w,NationId::Tonga);
        let cards=institutional_view(&w,NationId::Tonga);
        let binding_id="to_cast_crown_king_141b845110e8e610";
        let original=raw["historical_bindings"].as_array().unwrap().iter()
            .find(|b|b["id"]==binding_id).unwrap();
        let king=cards["historical_bindings"].as_array().unwrap().iter()
            .find(|b|b["id"]==binding_id).unwrap();
        assert_eq!(king["holder"],original["holder"]);
        assert!(king["holder"]["attested_on"].is_null());
        assert!(king["holder"]["from"].is_null());
        assert_eq!(king["holder"]["until"],"2006-09-11");
        assert_eq!(king["portrait_reference_date"],"1990-07-12");
        assert_eq!(king["portrait_date_is_tenure"],false);
        assert_eq!(king["person"]["id"],"taufaahau_tupou_iv");
        assert_eq!(king["person"]["portrait"]["url"],
            "/art/people/taufaahau-tupou-iv-cartoon-1990-v1.png");
        assert_eq!(spheres_sim::save(&w),saved);
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
