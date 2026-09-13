// Pure, selected-platform guidance. All legality and capability figures come
// from native design_preview. Search priorities describe suggestions, not an
// exhaustive optimum, a production order, or an affordable supplier offer.
fn guidance_primary(platform: &str, p: &eq::CompiledProfile) -> f64 {
    if let Some(a) = &p.aviation { return a.strike_factor; }
    match (platform, &p.ground_roles) {
        ("ground_artillery", Some(g)) => g.fire_support,
        ("ground_air_defense", Some(g)) => g.air_defense,
        ("ground_recon", Some(g)) => g.reconnaissance,
        ("ground_apc", Some(g)) => g.protected_mobility,
        ("ground_ifv", Some(g)) => (g.fire_support + g.protected_mobility) * 0.5,
        _ => p.land_factor,
    }
}

// Enumerate only small groups whose native configuration rules require parts
// to work together. These generate candidates; the compiler remains the rule
// authority. All other slots are explored independently.
fn guidance_groups(platform: &str) -> Vec<&'static [&'static str]> {
    match platform {
        "tank_standard" | "tank_heavy" | "tank_light" | "tank_destroyer" => vec![&["turret", "armament"]],
        "ground_recon" => vec![&["turret", "armament", "ammunition"]],
        "ground_air_defense" => vec![&["armament", "ammunition", "radar", "fire_control"]],
        "ground_artillery" => vec![&["ammunition", "fire_control"]],
        "air_light_attack" | "air_tactical_strike" => vec![&["air_payload", "air_avionics"]],
        _ => vec![],
    }
}

fn guidance_moves(platform: &str, allowed: impl Fn(&eq::ComponentDef) -> bool) -> Vec<Vec<(&'static str, &'static str)>> {
    let choices: Vec<_> = eq::all_components().filter(|c|eq::component_compatible(platform,c) && allowed(c)).collect();
    let mut moves: Vec<_> = choices.iter().map(|c|vec![(c.slot,c.id)]).collect();
    for group in guidance_groups(platform) {
        let mut packages = vec![vec![]];
        for slot in group {
            packages = packages.into_iter().flat_map(|package|choices.iter().filter(move |c|c.slot==*slot).map(move |c| {
                let mut next=package.clone(); next.push((c.slot,c.id)); next
            })).collect();
        }
        moves.extend(packages);
    }
    moves
}

fn guidance_apply(base: &eq::DesignSpec, changes: &[(&str,&str)]) -> eq::DesignSpec {
    let mut next=base.clone();
    for (slot,id) in changes { next.components.insert((*slot).into(),(*id).into()); }
    next
}

fn guidance_better(platform: &str, advanced: bool, a: &eq::CompiledProfile, b: &eq::CompiledProfile) -> bool {
    let secondary=|p:&eq::CompiledProfile|p.aviation.as_ref().map(|a|a.sorties_per_aircraft_month)
        .unwrap_or(p.land+p.protection+p.mobility+p.recon);
    let ordering = if advanced {
        guidance_primary(platform,a).total_cmp(&guidance_primary(platform,b))
            .then_with(||secondary(a).total_cmp(&secondary(b)))
            .then_with(||b.fabrication_cost_bn.total_cmp(&a.fabrication_cost_bn))
            .then_with(||b.maintenance_bn_day.total_cmp(&a.maintenance_bn_day))
    } else {
        b.fabrication_cost_bn.total_cmp(&a.fabrication_cost_bn)
            .then_with(||b.maintenance_bn_day.total_cmp(&a.maintenance_bn_day))
            .then_with(||guidance_primary(platform,a).total_cmp(&guidance_primary(platform,b)))
    };
    ordering.is_gt()
}

fn guidance_candidates(w: &WorldState, me: NationId, current: &eq::DesignSpec) -> Vec<(bool,eq::DesignSpec,eq::CompiledProfile)> {
    if !eq::PLATFORMS.iter().any(|p|p.id==current.platform) { return vec![]; }
    let Some(n)=w.nation_opt(me).filter(|n|n.alive) else { return vec![]; };
    let moves=guidance_moves(&current.platform,|c|eq::component_known(n,c));
    let seeds: Vec<_> = [eq::default_spec(&current.platform),eq::editable_spec(current)].into_iter().filter_map(|s| {
        let p=eq::design_preview(w,me,&s);
        p.valid.then(||p.profile.map(|p|(s,p))).flatten()
    }).collect();
    [false,true].into_iter().filter_map(|advanced| {
        let mut best=seeds.first()?.clone();
        for seed in seeds.iter().skip(1) {
            if guidance_better(&current.platform,advanced,&seed.1,&best.1) {best=seed.clone();}
        }
        // At most one improvement per slot-count pass; stable registry order
        // resolves exact ties. No world mutation, RNG, quote token or cache.
        for _ in 0..eq::platform_slots(&current.platform).len() {
            let mut step=best.clone();
            for change in &moves {
                let s=guidance_apply(&best.0,change);
                if s==best.0 {continue;}
                let p=eq::design_preview(w,me,&s);
                if !p.valid {continue;}
                if let Some(p)=p.profile {
                    if guidance_better(&current.platform,advanced,&p,&step.1) {step=(s,p);}
                }
            }
            if step.0==best.0 {break;}
            best=step;
        }
        Some((advanced,best.0,best.1))
    }).collect()
}

fn guidance_costs(w: &WorldState, p: &eq::CompiledProfile) -> Vec<Value> {
    let (acquisition,capital)=companies::design_cost_estimate(w,p,1);
    vec![
        cost("Development and trials",p.development_cost_bn,"government R&D · one model, paid as work progresses"),
        cost("Manufacturer tooling",p.tooling_cost_bn,"company capital · once per licensed model"),
        cost("Company fabrication",p.fabrication_cost_bn,"company cost per unit · purchased inputs separate"),
        cost("Indicative later acquisition",acquisition,"per unit at current input prices · estimate, not a stock offer"),
        cost("Company capital for tooling and one unit",capital,"illustrative company working capital · separate from the government purchase"),
        cost("Maintenance requirement",p.maintenance_bn_day,"government support per delivered unit / day · ammunition separate"),
    ]
}

fn guidance_conditions(w: &WorldState, me: NationId) -> Vec<String> {
    let mut rows=vec![
        "Acquisition estimates use today's native fabrication, material and operating-input prices plus the modeled company margin. Actual finished stock is priced from its paid cost basis; review its current stock, access and procurement authority before buying.".into(),
        "Development payments come from government R&D. Company cash funds tooling and inventory. Buying the finished equipment and maintaining delivered units are separate government payments; research and saved drafts create no equipment.".into(),
        format!("Current unused Defense authority: ${:.3}m R&D; ${:.3}m procurement. These balances do not guarantee future funding or make an indicative price an affordable offer.",
            spheres_sim::programs::available_bn(w,me,BUDGET_DEFENSE,4)*1000.0,
            spheres_sim::programs::available_bn(w,me,BUDGET_DEFENSE,3)*1000.0),
        "Commissioning still needs native review of the government, departmental budget, company and facility. A zero development limit pauses work. Staffing, power, paid inputs and earlier company work can delay certification or stock.".into(),
    ];
    let firms: Vec<_>=w.companies.firms.iter().filter(|c|c.nation==me).collect();
    if firms.is_empty() {
        rows.push("No domestic contractor exists. To commission this design, establish one with a free completed Arms Plant slot and separately reviewed capital; foreign finished-stock purchasing is a separate path.".into());
    } else {
        for c in firms {
            if let Some(reason)=companies::facility_blocker(w,c) {rows.push(format!("{}: {reason}",c.name));}
        }
    }
    rows
}

fn design_guidance(w: &WorldState, me: NationId, current: &eq::DesignSpec, profile: Option<&eq::CompiledProfile>) -> Value {
    let conditions=guidance_conditions(w,me);
    let recommendations: Vec<_>=guidance_candidates(w,me,current).into_iter().map(|(advanced,s,p)| {
        json!({"id":if advanced{"advanced"}else{"economical"},"name":if advanced{"Advanced researched configuration"}else{"Economical researched configuration"},
            "reason":if advanced{"Prioritizes this platform's supported role rating, then its secondary design ratings, using known components. Additional capability can cost more to develop, acquire and support."}else{"Prioritizes lower company fabrication cost, then lower maintenance, using known components. Lower costs can mean weaker mission ratings; this is not an affordability claim."},
            "specification_key":eq::specification_key(&s),"spec":s,"metrics":profile_metrics(&p),"costs":guidance_costs(w,&p),"conditions":conditions,"blockers":[]})
    }).collect();
    json!({"title":"Choose a design direction","detail":"Two deterministic suggestions from a bounded search of known components and compatible packages on this platform. Compare their actual ratings and tradeoffs; neither is a guaranteed best design. Applying one only edits the draft.",
        "platform":current.platform,"recommendations":recommendations,"costs":profile.map(|p|guidance_costs(w,p)).unwrap_or_default(),"conditions":conditions})
}

// Research examples may contain unlearned parts but must be structurally valid.
// They never grant those parts or become current-design recommendations.
fn component_example(w: &WorldState, me: NationId, c: &eq::ComponentDef) -> Option<(String,eq::CompiledProfile,eq::CompiledProfile)> {
    for platform in eq::PLATFORMS.iter().filter(|p|eq::component_compatible(p.id,c)) {
        let base=eq::default_spec(platform.id);
        let before=eq::design_preview(w,me,&base).profile?;
        let single=guidance_apply(&base,&[(c.slot,c.id)]);
        if eq::configuration_refusals(&single).is_empty() {
            if let Some(after)=eq::design_preview(w,me,&single).profile {
                if after.installation_used<=after.installation_capacity {
                    // Zero or one changed slot is the smallest possible
                    // example containing this part. Any package with that
                    // same count has exactly this specification and price.
                    return Some((platform.name.into(),before,after));
                }
            }
        }
        let mut best: Option<(usize,eq::CompiledProfile)>=None;
        for changes in guidance_moves(platform.id,|_|true).into_iter().filter(|m|m.iter().any(|(slot,id)|*slot==c.slot&&*id==c.id)) {
            let spec=guidance_apply(&base,&changes);
            if !eq::configuration_refusals(&spec).is_empty() {continue;}
            let Some(p)=eq::design_preview(w,me,&spec).profile else {continue;};
            if p.installation_used>p.installation_capacity {continue;}
            let count=spec.components.iter().filter(|(k,v)|base.components.get(*k)!=Some(*v)).count();
            if best.as_ref().is_none_or(|(old,q)|count<*old||(count==*old&&p.fabrication_cost_bn<q.fabrication_cost_bn)) {best=Some((count,p));}
        }
        if let Some((_,after))=best {return Some((platform.name.into(),before,after));}
    }
    None
}

fn research_component(w: &WorldState, me: NationId, c: &eq::ComponentDef) -> Value {
    let mut metrics=vec![metric("Component installation load",format!("{} points",c.load))];
    for (label,value) in [("Component firepower input",c.land),("Component protection input",c.protection),("Component mobility input",c.mobility),("Component observation input",c.recon)] {
        if value!=0.0 {metrics.push(metric(label,format!("{value:+.3} rating")));}
    }
    let mut tradeoffs=vec![
        "Unlocks a selectable part. Research alone changes no vehicle, stock, delivered fleet or national combat rating.".into(),
        "Installation space, compatible companion parts, paid development and continuing support still apply. Component inputs are not universal whole-vehicle gains.".into(),
    ];
    if let Some((name,before,after))=component_example(w,me,c) {
        let old=profile_rows(&before);
        for row in profile_rows(&after).into_iter().filter(|r|!matches!(r.3,"bn"|"days"|"points")) {
            if let Some(prior)=old.iter().find(|p|p.0==row.0) {
                if prior.2.to_bits()!=row.2.to_bits() {metrics.push(metric(&format!("{} · {name} example",row.1),format!("{:.3} → {:.3} {}",prior.2,row.2,row.3)));}
            }
        }
        tradeoffs.push(format!("Example uses the {name} starting configuration with this part and any required companion parts. Other research may be needed; effects depend on the complete configuration and its role."));
    }
    json!({"id":c.id,"name":c.name,"slot":c.slot,"description":c.detail,"known":eq::component_known(w.nation(me),c),
        "compatible_platforms":eq::PLATFORMS.iter().filter(|p|eq::component_compatible(p.id,c)).map(|p|json!({"id":p.id,"name":p.name})).collect::<Vec<_>>(),
        "metrics":metrics,"costs":[cost("Component fabrication",c.cost_bn,"company cost per installed unit · not the complete vehicle price"),cost("Component maintenance",c.upkeep_bn_day,"per delivered unit / day · part of the compiled maintenance requirement")],"tradeoffs":tradeoffs})
}

#[cfg(test)]
mod guidance_tests {
    use super::*;
    const ME: NationId=NationId::USA;
    fn game() -> super::super::Game {
        let mut g=super::super::Game::new(1990,Some(ME));
        super::super::play_rules(&mut g);
        g
    }
    fn all_research(w: &mut WorldState) {
        // Synthetic read fixture only: never campaign qualification or an
        // alternate implementation of research completion.
        let n=w.nation_mut(ME);
        n.equipment.get_or_insert_with(Default::default).learned.extend(eq::RESEARCH.iter().map(|r|r.id.to_string()));
        for id in eq::all_components().filter_map(|c|c.technology) {
            let index=spheres_sim::tech::index_of(id).unwrap();
            if !n.tech.knows_index(index) {n.tech.known.push(index);}
        }
        n.tech.known.sort();
    }
    fn amount(rows: &[Value], label: &str) -> f64 {
        rows.iter().find(|r|r["label"]==label).unwrap()["amount_bn"].as_f64().unwrap()
    }
    #[test]
    fn s09_all_platform_recommendations_are_known_legal_deterministic_and_pure() {
        let mut g=game();
        assert_eq!(eq::PLATFORMS.len(),11);
        for researched in [false,true] {
            if researched {all_research(&mut g.world);} else {
                g.world.nation_mut(ME).tech.known.clear();
                g.world.nation_mut(ME).equipment=None;
            }
            let before=spheres_sim::save(&g.world);
            for platform in eq::PLATFORMS {
                let base=eq::default_spec(platform.id);
                let profile=eq::design_preview(&g.world,ME,&base).profile.unwrap();
                let first=design_guidance(&g.world,ME,&base,Some(&profile));
                assert_eq!(first,design_guidance(&g.world,ME,&base,Some(&profile)),"{} deterministic",platform.id);
                let recommendations=first["recommendations"].as_array().unwrap();
                assert_eq!(recommendations.len(),2,"{} has two current directions",platform.id);
                for r in recommendations {
                    let s: eq::DesignSpec=serde_json::from_value(r["spec"].clone()).unwrap();
                    let p=eq::design_preview(&g.world,ME,&s);
                    assert!(p.valid,"{} {}: {:?}",platform.id,r["id"],p.blockers);
                    assert_eq!(s.platform,platform.id);
                    assert_eq!(r["specification_key"],eq::specification_key(&s));
                    assert!(s.components.values().all(|id|eq::component_known(g.world.nation(ME),eq::component(id).unwrap())));
                    let p=p.profile.unwrap();
                    assert!(p.installation_used<=p.installation_capacity);
                    if r["id"]=="economical" {assert!(p.fabrication_cost_bn<=profile.fabrication_cost_bn);}
                    else {assert!(guidance_primary(platform.id,&p)>=guidance_primary(platform.id,&profile));}
                    assert!(r["blockers"].as_array().unwrap().is_empty());
                }
            }
            assert_eq!(spheres_sim::save(&g.world),before);
        }
    }
    #[test]
    fn s09_guidance_enumerates_coupled_packages_without_bypassing_research_or_load() {
        let mut g=game();
        g.world.nation_mut(ME).equipment=None;
        g.world.nation_mut(ME).tech.known.clear();
        let closed=guidance_moves("ground_air_defense",|c|eq::component_known(g.world.nation(ME),c));
        assert!(!closed.iter().flatten().any(|(_,id)|*id=="ground_aa_missiles"));
        all_research(&mut g.world);
        for (platform,part) in [("tank_standard","gun_125"),("ground_recon","ground_gun_35"),("ground_air_defense","ground_aa_missiles"),("ground_artillery","ground_ammo_guided"),("air_light_attack","air_payload_guided")] {
            let base=eq::default_spec(platform);
            let c=eq::component(part).unwrap();
            let single=guidance_apply(&base,&[(c.slot,c.id)]);
            assert!(!eq::configuration_refusals(&single).is_empty(),"{part} requires companion parts");
            let packages=guidance_moves(platform,|c|eq::component_known(g.world.nation(ME),c));
            assert!(packages.iter().filter(|p|p.iter().any(|(_,id)|*id==part)).any(|p| {
                p.len()>1 && eq::design_preview(&g.world,ME,&guidance_apply(&base,p)).valid
            }),"{part} has a legal researched package");
        }
        let mut overloaded=eq::default_spec("tank_standard");
        for slot in eq::platform_slots("tank_standard") {
            let c=eq::all_components().filter(|c|c.slot==*slot&&eq::component_compatible("tank_standard",c)).max_by_key(|c|c.load).unwrap();
            overloaded.components.insert((*slot).into(),c.id.into());
        }
        let p=eq::design_preview(&g.world,ME,&overloaded);
        assert!(p.profile.as_ref().is_some_and(|p|p.installation_used>p.installation_capacity));
        assert!(!p.valid);
        for (_,s,_) in guidance_candidates(&g.world,ME,&overloaded) {assert!(eq::design_preview(&g.world,ME,&s).valid);}
        overloaded.platform="unsupported-fighter".into();
        assert!(guidance_candidates(&g.world,ME,&overloaded).is_empty());
    }
    #[test]
    fn s09_guidance_costs_match_native_development_terms_not_purchase_affordability() {
        let mut g=game();
        for operations in [false,true] {
            if operations {g.world.supplier_operations.version=spheres_sim::supplier_operations::VERSION;}
            if let Some(m)=g.world.resources.market.as_mut() {for (i,p) in m.prices.iter_mut().enumerate() {*p*=1.137+i as f64*0.071;}}
            let before=spheres_sim::save(&g.world);
            for platform in ["tank_standard","ground_apc","air_tactical_strike"] {
                let s=eq::default_spec(platform);
                let p=eq::design_preview(&g.world,ME,&s).profile.unwrap();
                let q=companies::development_quote(&g.world,ME,u32::MAX,"Illustrative design",&s,0.0005,1);
                assert!(!q.valid,"missing company must not become an available offer");
                let rows=guidance_costs(&g.world,&p);
                assert_eq!(amount(&rows,"Indicative later acquisition").to_bits(),q.unit_price_bn.to_bits());
                assert_eq!(amount(&rows,"Company capital for tooling and one unit").to_bits(),q.company_cash_needed_bn.to_bits());
                assert_eq!(amount(&rows,"Development and trials").to_bits(),q.cost_bn.to_bits());
                assert_eq!(amount(&rows,"Maintenance requirement").to_bits(),q.maintenance_bn_day.to_bits());
                // Literal native input/cost ordering is independent of the new
                // wrapper and of labels in the adapter.
                let raw=spheres_sim::resources::ALL.iter().map(|c|p.recipe[c.idx()]*spheres_sim::resources::market_current_price(&g.world,*c)/1e9).sum::<f64>()
                    + spheres_sim::supplier_operations::new_refit_inputs_bn(&g.world,p.fabrication_cost_bn,p.production_days);
                assert_eq!(amount(&rows,"Indicative later acquisition").to_bits(),((p.fabrication_cost_bn+raw)*1.15).to_bits());
                assert!(amount(&rows,"Indicative later acquisition")>p.fabrication_cost_bn);
            }
            let conditions=guidance_conditions(&g.world,ME).join(" ");
            assert!(conditions.contains("Current unused Defense authority"));
            assert!(conditions.contains("No domestic contractor"));
            assert!(conditions.contains("zero development limit"));
            assert_eq!(spheres_sim::save(&g.world),before);
        }
    }
    #[test]
    fn s09_research_unlock_details_use_real_parts_and_prospective_compiled_examples() {
        let g=game();let before=spheres_sim::save(&g.world);
        let rows=research_board(&g.world,ME);
        for row in rows {
            for value in row["unlock_components"].as_array().unwrap() {
                let c=eq::component(value["id"].as_str().unwrap()).unwrap();
                assert_eq!(value["description"],c.detail);
                assert_eq!(value["known"],eq::component_known(g.world.nation(ME),c));
                assert_eq!(amount(value["costs"].as_array().unwrap(),"Component fabrication").to_bits(),c.cost_bn.to_bits());
                assert_eq!(amount(value["costs"].as_array().unwrap(),"Component maintenance").to_bits(),c.upkeep_bn_day.to_bits());
                let platforms:Vec<_>=value["compatible_platforms"].as_array().unwrap().iter().map(|p|p["id"].as_str().unwrap()).collect();
                assert_eq!(platforms,eq::PLATFORMS.iter().filter(|p|eq::component_compatible(p.id,c)).map(|p|p.id).collect::<Vec<_>>());
                assert!(value["tradeoffs"].as_array().unwrap().iter().any(|s|s.as_str().unwrap().contains("Research alone changes no vehicle")));
            }
        }
        let guided=research_component(&g.world,ME,eq::component("air_payload_guided").unwrap());
        assert!(guided["metrics"].as_array().unwrap().iter().any(|r|r["label"].as_str().unwrap().contains("strike effectiveness")));
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn s09_single_component_example_shortcut_matches_exhaustive_package_selection() {
        // Original exhaustive selection remains an independent test oracle.
        fn original(w: &WorldState, me: NationId, c: &eq::ComponentDef) -> Option<(String,eq::CompiledProfile,eq::CompiledProfile)> {
            for platform in eq::PLATFORMS.iter().filter(|p|eq::component_compatible(p.id,c)) {
                let base=eq::default_spec(platform.id);
                let before=eq::design_preview(w,me,&base).profile?;
                let mut best: Option<(usize,eq::CompiledProfile)>=None;
                for changes in guidance_moves(platform.id,|_|true).into_iter().filter(|m|m.iter().any(|(slot,id)|*slot==c.slot&&*id==c.id)) {
                    let spec=guidance_apply(&base,&changes);
                    if !eq::configuration_refusals(&spec).is_empty() {continue;}
                    let Some(p)=eq::design_preview(w,me,&spec).profile else {continue;};
                    if p.installation_used>p.installation_capacity {continue;}
                    let count=spec.components.iter().filter(|(k,v)|base.components.get(*k)!=Some(*v)).count();
                    if best.as_ref().is_none_or(|(old,q)|count<*old||(count==*old&&p.fabrication_cost_bn<q.fabrication_cost_bn)) {best=Some((count,p));}
                }
                if let Some((_,after))=best {return Some((platform.name.into(),before,after));}
            }
            None
        }
        let mut g=game();
        for known in [false,true] {
            if known {all_research(&mut g.world);} else {
                g.world.nation_mut(ME).tech.known.clear();
                g.world.nation_mut(ME).equipment=None;
            }
            let before=spheres_sim::save(&g.world);
            for c in eq::all_components() {
                assert_eq!(serde_json::to_value(component_example(&g.world,ME,c)).unwrap(),
                    serde_json::to_value(original(&g.world,ME,c)).unwrap(),"{} all compiled fields, known={known}",c.id);
            }
            assert_eq!(spheres_sim::save(&g.world),before);
        }
    }
    #[test]
    fn s09_preview_and_comparison_leave_old_revision_holdings_and_research_unchanged() {
        let mut g=game();let original=eq::baseline_spec();
        let mut frozen=eq::design_preview(&g.world,ME,&original).profile.unwrap();
        frozen.fabrication_cost_bn=0.123;
        let n=g.world.nation_mut(ME);let s=n.equipment.get_or_insert_with(Default::default);
        s.revisions.insert("historic".into(),eq::DesignRevision {id:"historic".into(),name:"Original frozen model".into(),specification_key:eq::specification_key(&original),spec:original,profile:frozen.clone(),created_day:0,certified_day:Some(0)});
        spheres_sim::arsenal::deliver_design(n,"historic",20,12.0).unwrap();
        let target=eq::default_spec("ground_apc");
        let before=spheres_sim::save(&g.world);
        let p=preview(&g.world,ME,&g.session_id,&json!({"name":"Read only proposal","platform":target.platform,"components":target.components,"comparison_id":"historic"})).unwrap();
        assert_eq!(p["guidance"]["recommendations"].as_array().unwrap().len(),2);
        assert_eq!(p["comparison"]["same_role"],false);
        let rows=p["comparison"]["rows"].as_array().unwrap();
        assert_eq!(rows.iter().find(|r|r["key"]=="fabrication").unwrap()["before"],0.123);
        let acquisition=rows.iter().find(|r|r["key"]=="acquisition_estimate").unwrap();
        assert_eq!(acquisition["before"].as_f64().unwrap().to_bits(),companies::design_cost_estimate(&g.world,&frozen,1).0.to_bits());
        assert!(p["comparison"]["conditions"][0].as_str().unwrap().contains("not the historical purchase price"));
        assert_eq!(spheres_sim::save(&g.world),before);
    }
}
