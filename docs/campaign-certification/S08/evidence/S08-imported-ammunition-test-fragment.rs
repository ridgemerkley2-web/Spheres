// Representative foreign ammunition branches: basic ground rounds and guided
// aircraft stores. The existing domestic suite independently covers all23 recipes.
#[test]
fn s08_imported_rounds_and_guided_air_stores_keep_one_paid_receipt_without_research_grants(){
    for guided in [false,true] {
        let (mut w,d)=fixture();let c=establish(&mut w,&d,1.0);isolated_day(&mut w);
        let mut spec=equipment::default_spec(if guided{"air_light_attack"}else{"ground_apc"});
        if guided { // Explicit seller-only research endowment to exercise a locked foreign component.
            w.nation_mut(HOME).equipment.as_mut().unwrap().learned=equipment::RESEARCH.iter().map(|r|r.id.into()).collect();
            spec.components.insert("air_payload".into(),"air_payload_guided".into());spec.components.insert("air_avionics".into(),"air_avionics_digital".into());
        }
        let family=equipment::ammunition_family(&spec).unwrap().to_string();assert_eq!(family,if guided{"air_bomb_guided"}else{"mg_127"});
        let q=companies::development_quote(&w,HOME,c,"S08 ammunition interface",&spec,1.0,1);assert!(q.valid,"{:?}",q.reason);
        apply(&mut w,companies::CompanyOrder::Develop{company:c,name:"S08 ammunition interface".into(),spec,daily_budget_bn:1.0,stock_target:1,quote:q.token});
        let p=firm(&w,c).products.last().unwrap().id;until_stock(&mut w,c,p,1);apply(&mut w,companies::CompanyOrder::Inventory{company:c,product:p,stock_target:0});
        spheres_sim::apply_command(&mut w,&Command::Equipment{nation:HOME,order:spheres_sim::EquipmentOrder::Maintenance{daily_budget_bn:0.01}}).unwrap();
        let q=companies::ammo_supply_quote(&w,HOME,c,&family,12);assert!(q.valid,"{:?}",q.reason);apply(&mut w,companies::CompanyOrder::AmmoSupply{company:c,family:family.clone(),stock_target:12,quote:q.token});let ammo=firm(&w,c).ammunition_products.last().unwrap().id;
        for _ in 0..10 {if firm(&w,c).ammunition_products[0].stock==12{break;}isolated_day(&mut w);}assert_eq!(firm(&w,c).ammunition_products[0].stock,12);apply(&mut w,companies::CompanyOrder::AmmoInventory{company:c,product:ammo,stock_target:0});
        buyer(&mut w);let learned=w.nation(BUYER).equipment.as_ref().unwrap().learned.clone();assert!(!companies::import_purchase_quote(&w,BUYER,HOME,c,ammo,true,3).valid,"A compatible owned model and maintenance plan are prerequisites");book(&mut w,c,p,false,1);end_day(&mut w);let due=w.companies.imports.contracts[0].due_day.unwrap();while clock::absolute_day(&w)<=due{advance(&mut w);}
        spheres_sim::apply_command(&mut w,&Command::Equipment{nation:BUYER,order:spheres_sim::EquipmentOrder::Maintenance{daily_budget_bn:0.001}}).unwrap();advance(&mut w);advance(&mut w);
        let q=companies::import_purchase_quote(&w,BUYER,HOME,c,ammo,true,3);assert!(q.valid,"{:?}",q.reason);assert!(q.protected_maintenance_bn>=0.0);
        let cash=firm(&w,c).cash_bn;book(&mut w,c,ammo,true,3);assert_eq!(firm(&w,c).ammunition_products[0].stock,9);assert_eq!(companies::ammo_inbound_units(&w,BUYER,&family),3);
        let mut loaded=restored(&w);end_day(&mut w);end_day(&mut loaded);let due=w.companies.imports.contracts.last().unwrap().due_day.unwrap();while clock::absolute_day(&w)<=due{advance(&mut w);advance(&mut loaded);}assert_eq!(spheres_sim::save(&w),spheres_sim::save(&loaded));
        let stores=w.nation(BUYER).equipment.as_ref().unwrap().ammunition.as_ref().unwrap();assert_eq!(stores.stocks[&family],3.0);assert_eq!(stores.supplier_receipts.len(),1);assert!(stores.active_from_day.is_none());assert!(stores.orders.is_empty());assert_eq!(companies::ammo_inbound_units(&w,BUYER,&family),0);near(firm(&w,c).cash_bn,cash+q.cost_bn);assert_eq!(w.nation(BUYER).equipment.as_ref().unwrap().learned,learned);restored(&w);
    }
}
