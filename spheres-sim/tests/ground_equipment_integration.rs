//! Controlled ground-designer accounting and role invariants, not historical
//! calibration claims. Ratings and prices are explicit game assumptions.
use spheres_sim::{arsenal, clock, equipment as eq, operations, war};
use spheres_sim::world::{Belligerent,Conflict,GameRules,NationId as N,Objective,WorldState};

fn world() -> WorldState {
    let mut w=spheres_sim::init::world_1990(GameRules {daily_simulation:true,military_operations:true,manufacturing_system:true,resource_market:true,..GameRules::default()});
    w.player=Some(N::USA); w.nation_mut(N::USA).political_capital=1000.0; w
}
fn revision(w:&mut WorldState, platform:&str, id:&str) {
    let spec=eq::default_spec(platform); let preview=eq::design_preview(w,N::USA,&spec);
    assert!(preview.valid,"{platform}: {:?}",preview.blockers);
    let day=clock::absolute_day(w);
    w.nation_mut(N::USA).equipment.get_or_insert_with(Default::default).revisions.insert(id.into(),eq::DesignRevision {
        id:id.into(),name:id.into(),specification_key:eq::specification_key(&spec),spec,
        profile:preview.profile.unwrap(),created_day:day,certified_day:Some(day),
    });
}
fn conflict(w:&WorldState,id:u32,enemy:N)->Conflict {
    Conflict {id,theatre:war::theatre_between(w,N::USA,enemy),side_a:vec![N::USA],side_b:vec![enemy],
        posture:vec![Belligerent::new(N::USA,8,Objective::Seize),Belligerent::new(enemy,8,Objective::Hold)],
        control:0.0,months:0,quiet_months:0,frozen_since:None,start_year:1990,start_month:1,
        origin_attacker:N::USA,invasion_declared:true,front:Default::default(),pockets:vec![],aim:None}
}

#[test]
fn every_family_has_a_complete_compatible_starter_and_rejects_foreign_mission_hardware() {
    let mut w=world(); let original=spheres_sim::save(&w);
    for platform in eq::PLATFORMS {
        let spec=eq::default_spec(platform.id);
        assert!(eq::design_preview(&w,N::USA,&spec).valid,"{}",platform.id);
        assert_eq!(spec.components.len(),eq::platform_slots(platform.id).len());
        for (slot,id) in &spec.components {
            let c=eq::component(id).unwrap(); assert_eq!(c.slot,slot); assert!(eq::component_compatible(platform.id,c));
            let mut missing=spec.clone(); missing.components.remove(slot);
            assert!(!eq::design_preview(&w,N::USA,&missing).valid);
        }
    }
    assert_eq!(spheres_sim::save(&w),original,"all previews are read-only");
    for (platform,slot,id) in [("ground_apc","armament","gun_120"),("tank_standard","armament","ground_mg_127"),
        ("ground_recon","radar","ground_radar_search"),("ground_air_defense","ammunition","ground_ammo_missiles")] {
        let mut bad=eq::default_spec(platform); bad.components.insert(slot.into(),id.into());
        assert!(!eq::design_preview(&w,N::USA,&bad).valid);
        assert!(eq::start_development(&mut w,N::USA,"Invalid family",bad,1.0).is_err());
        assert_eq!(spheres_sim::save(&w),original);
    }
    let mut legacy=eq::baseline_spec(); legacy.components.insert("armament".into(),"ground_mg_127".into());
    assert!(!eq::design_preview(&w,N::USA,&legacy).valid,"a legacy package cannot bypass family validation");
}

#[test]
fn component_research_follows_branches_and_never_retrofits_frozen_stock() {
    let mut w=world(); revision(&mut w,"ground_artillery","original");
    arsenal::deliver_design(w.nation_mut(N::USA),"original",10,30.0).unwrap();
    let frozen=eq::profile(w.nation(N::USA),"original").unwrap().clone();
    let stock=serde_json::to_string(&w.nation(N::USA).arsenal).unwrap();
    let macro_count=w.nation(N::USA).tech.count();
    let start=spheres_sim::save(&w);
    assert!(eq::start_research(&mut w,N::USA,"ground_guided_weapons").is_err());
    assert_eq!(spheres_sim::save(&w),start);
    let mut day=clock::absolute_day(&w);
    for id in ["ground_medium_weapons","tank_fire_control_1990","ground_guided_weapons","ground_sensor_fusion","ground_secure_radios","ground_battlefield_network"] {
        assert!(eq::research_available(w.nation(N::USA),id).is_ok(),"{id}");
        eq::start_research(&mut w,N::USA,id).unwrap();
        day+=1;
        assert!(eq::research_step(w.nation_mut(N::USA),eq::research(id).unwrap().points,1990,day));
        assert!(w.nation(N::USA).equipment.as_ref().unwrap().learned.contains(id));
    }
    let mut improved=eq::default_spec("ground_artillery");
    improved.components.insert("ammunition".into(),"ground_ammo_guided".into());
    assert!(!eq::design_preview(&w,N::USA,&improved).valid,"guided load also needs its installed digital control");
    improved.components.insert("fire_control".into(),"fcs_digital".into());
    let preview=eq::design_preview(&w,N::USA,&improved); assert!(preview.valid,"{:?}",preview.blockers);
    assert!(preview.profile.unwrap().ground_roles.unwrap().fire_support>frozen.ground_roles.unwrap().fire_support);
    assert_eq!(eq::profile(w.nation(N::USA),"original").unwrap(),&frozen);
    assert_eq!(serde_json::to_string(&w.nation(N::USA).arsenal).unwrap(),stock);
    assert_eq!(w.nation(N::USA).tech.count(),macro_count);
    eq::validate_state(w.nation(N::USA)).unwrap();
    let mut corrupt=w.clone(); corrupt.nation_mut(N::USA).equipment.as_mut().unwrap().learned.remove("ground_medium_weapons");
    assert!(spheres_sim::load(&spheres_sim::save(&corrupt)).is_err());
}

#[test]
fn specialists_need_fielded_supported_inventory_and_cannot_create_strategic_lift() {
    for platform in eq::PLATFORMS.iter().filter(|p|eq::is_ground_platform(p.id)) {
        let mut w=world(); revision(&mut w,platform.id,"specialist");
        let n=w.nation_mut(N::USA); n.arsenal.held.clear(); n.arsenal.orders.clear();
        let empty=operations::capabilities(n); assert_eq!(empty.ground_roles,eq::GroundRoles::default());
        arsenal::queue_design_order(n,"specialist",10,7,0.0).unwrap();
        assert_eq!(operations::capabilities(n).ground_roles,eq::GroundRoles::default(),"orders are not coverage");
        arsenal::deliver_design(n,"specialist",10,0.0).unwrap();
        let before=operations::capabilities(n);
        assert!(before.ground_roles.reconnaissance>0.0);
        assert_eq!(before.lift,empty.lift); assert_eq!(before.strike,empty.strike);
        let profile=&mut n.equipment.as_mut().unwrap().revisions.get_mut("specialist").unwrap().profile;
        profile.unit_cost_bn*=1000.0;
        assert_eq!(operations::capabilities(n).ground_roles,before.ground_roles,"price is not combat coverage");
        arsenal::reserve_refit(n,"specialist",10).unwrap();
        assert_eq!(operations::capabilities(n).ground_roles,eq::GroundRoles::default(),"reserved vehicles leave the battle");
        arsenal::release_refit(n,"specialist",10).unwrap();
        n.equipment.as_mut().unwrap().maintenance_fraction=0.0;
        assert_eq!(operations::capabilities(n).ground_roles,eq::GroundRoles::default(),"unsupported vehicles contribute no specialist effect");
    }
}

#[test]
fn specialist_stock_and_shared_deployments_continue_identically_after_reload() {
    let mut w=world();
    for p in ["ground_ifv","ground_apc","ground_recon","ground_artillery","ground_air_defense"] {
        revision(&mut w,p,p); arsenal::deliver_design(w.nation_mut(N::USA),p,100,12.0).unwrap();
    }
    w.conflicts=vec![conflict(&w,1,N::Iraq),conflict(&w,2,N::Vietnam)];
    let v=operations::view(&w,N::USA);
    assert!(v.deployed<=v.structure+1e-10); assert!(v.overseas_deployed<=v.overseas_limit+1e-10);
    let raw=spheres_sim::save(&w); let mut loaded=spheres_sim::load(&raw).unwrap(); assert_eq!(spheres_sim::save(&loaded),raw);
    for _ in 0..3 {war::tick(&mut w);war::tick(&mut loaded);clock::advance_date(&mut w);clock::advance_date(&mut loaded);}
    assert_eq!(spheres_sim::save(&w),spheres_sim::save(&loaded));
    eq::validate_state(w.nation(N::USA)).unwrap();
    assert!(w.nation(N::USA).arsenal.held.iter().filter(|h|h.design_id.is_some()).all(|h|h.units<=100.0 && h.units.fract()==0.0));
}

#[test]
fn old_tank_versions_keep_their_serialized_profiles_when_the_library_is_upgraded() {
    for (version,spec) in [(1,eq::baseline_spec()),(2,eq::tank_spec("tank_standard"))] {
        let mut w=world(); revision(&mut w,"tank_standard","old");
        let preview=eq::design_preview(&w,N::USA,&spec);
        let s=w.nation_mut(N::USA).equipment.as_mut().unwrap(); s.version=version;
        let r=s.revisions.get_mut("old").unwrap(); r.specification_key=eq::specification_key(&spec);r.spec=spec;r.profile=preview.profile.unwrap();
        let old=serde_json::to_string(r).unwrap(); assert!(!old.contains("ground_roles"));
        let mut loaded=spheres_sim::load(&spheres_sim::save(&w)).unwrap();
        eq::save_draft(&mut loaded,N::USA,"Scout",eq::default_spec("ground_recon")).unwrap();
        let s=loaded.nation(N::USA).equipment.as_ref().unwrap();assert_eq!(s.version,eq::VERSION);
        assert_eq!(serde_json::to_string(&s.revisions["old"]).unwrap(),old);
    }
}
