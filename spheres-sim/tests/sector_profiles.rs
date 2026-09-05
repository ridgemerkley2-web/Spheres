use spheres_sim::{clock,init::world_1990,province_economy,sector_profiles,starting_industry,world::*,save,load};
#[test]
fn profiles_reconcile_and_enrichment_changes_only_frozen_accounts(){
    let mut w=world_1990(GameRules::default());clock::enable_daily_play(&mut w);
    starting_industry::enable_new_world(&mut w).unwrap();
    let before=w.clone();starting_industry::enrich_new_world(&mut w).unwrap();
    assert_eq!(serde_json::to_value(&w.nations).unwrap(),serde_json::to_value(&before.nations).unwrap());assert_eq!(w.production,before.production);
    assert_eq!(w.resources,before.resources);assert_eq!(w.rng,before.rng);
    for (&id,p) in sector_profiles::data(){
        assert!((p.shares.iter().sum::<f64>()-1.0).abs()<1e-12);
        assert_eq!(p.shares[2],w.starting_industry.as_ref().unwrap().profiles[&id].manufacturing_share);
    }
    assert_ne!(sector_profiles::data()[&NationId::Japan].shares,sector_profiles::data()[&NationId::Kuwait].shares);
    province_economy::enable(&mut w);
    for n in w.nations.iter().filter(|n|n.alive){let s=province_economy::snapshot(&w,n.id).unwrap();assert!((s.provinces.iter().map(|p|p.total_gdp_bn).sum::<f64>()+s.unallocated_gdp_bn-n.gdp).abs()<1e-8);}
    let restored=load(&save(&w)).unwrap();assert_eq!(save(&restored),save(&w));
    assert!(starting_industry::enrich_new_world(&mut w).is_err());
}
#[test]
fn old_frozen_accounts_are_not_backfilled_on_read_or_load(){
    let mut w=world_1990(GameRules::default());clock::enable_daily_play(&mut w);starting_industry::enable_new_world(&mut w).unwrap();province_economy::enable(&mut w);
    let saved=save(&w);let restored=load(&saved).unwrap();
    assert!(restored.starting_industry.as_ref().unwrap().broad_profiles.is_empty());
    assert_eq!(saved,save(&restored));
}
