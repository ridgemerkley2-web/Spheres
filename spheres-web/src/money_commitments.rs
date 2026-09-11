//! Saved commitments, grouped by ownership and payment stage. These categories
//! deliberately have no combined total: transfers, paid property and conditional
//! future work are different claims, often describing the same original order.
use serde_json::{json, Value};
use spheres_sim::{clock, equipment, industry, production, programs,
    world::{NationId, WorldState, BUDGET_MINISTRIES}};

const DISPLAY_LIMIT: usize = 8;
const MINISTRIES: [&str; BUDGET_MINISTRIES] = ["Health", "Education", "Housing", "Welfare",
    "Infrastructure", "Industry & Energy", "Science", "Defense", "Security", "Diplomacy"];

fn date(day: i32) -> String {
    let (year, month, date) = clock::date_from_day(day);
    format!("{year:04}-{month:02}-{date:02}")
}

fn item(id: String, label: String, amount: Option<f64>, status: &str,
    due: String, action: Option<&str>) -> Value {
    let mut row = json!({"id":id,"label":label,"amount_bn":amount,
        "status":status,"due_label":due});
    if let Some(action) = action { row["action"] = json!({"action":action}); }
    row
}

fn category(id: &str, title: &str, note: &str, mut items: Vec<Value>) -> Value {
    // An unknown saved amount is not zero and cannot produce a complete total.
    // Compute before truncation so eight visible rows never understate the book.
    let total = items.iter().try_fold(0.0, |sum, row|
        row["amount_bn"].as_f64().filter(|v|v.is_finite()).map(|v|sum+v));
    let count = items.len();
    items.truncate(DISPLAY_LIMIT);
    let note = if count > DISPLAY_LIMIT {
        format!("{note} Showing {DISPLAY_LIMIT} of {count} entries. The total covers all entries; it is unknown if any saved amount is unavailable.")
    } else if count == 0 { format!("{note} No entries in this category.") }
    else { note.to_owned() };
    json!({"id":id,"title":title,"amount_bn":total,"note":note,"items":items})
}

fn model_name(w: &WorldState, nation: NationId, revision: &str) -> String {
    w.nation_opt(nation).and_then(|n|n.equipment.as_ref())
        .and_then(|s|s.revisions.get(revision)).map_or_else(||revision.to_owned(),|r|r.name.clone())
}

/// No enrollment, quotes that open ledgers, work planning, or economic mutation.
/// The session/route owns access control; every record is filtered to `me`.
pub(crate) fn view(w: &WorldState, me: NationId) -> Vec<Value> {
    let Some(nation) = w.nation_opt(me).filter(|n|n.alive) else { return vec![]; };
    let mut pending = vec![];
    if let Some(p) = nation.program_budget.as_ref().filter(|p|
        p.day.is_some() && p.day != p.settled_day) {
        for (m, departments) in p.spent_today_bn.iter().enumerate() {
            for (d, &amount) in departments.iter().enumerate() {
                if amount <= 0.0 { continue; }
                pending.push(item(format!("program:{m}:{d}"),
                    format!("{}: {}", MINISTRIES[m], programs::NAMES[m][d]), Some(amount),
                    "awaiting_fiscal_close", format!("Fiscal close of {}", date(p.day.unwrap())), Some("budget")));
            }
        }
    }

    let mut transfers = vec![];
    let mut refits = vec![];
    let mut development = vec![];
    for company in w.companies.firms.iter().filter(|c|c.nation==me) {
        for receipt in &company.receivables {
            let purpose = match receipt.kind.as_str() {
                "capitalization" => "Company capital",
                "development" => "Completed development work",
                "sale" => "Equipment purchase",
                "ammo_sale" => "Ammunition purchase",
                "refit_advance" => "Refit deposit",
                other => other,
            };
            transfers.push(item(format!("supplier:{}:{}",company.id,receipt.id),
                format!("{}: {purpose}",company.name),Some(receipt.amount_bn),"awaiting_supplier_transfer",
                format!("After fiscal close of {}",date(receipt.day)),None));
        }
        for contract in company.refits.iter().filter(|p|p.settled_day.is_some() && p.escrow_bn>0.0) {
            refits.push(item(format!("refit:{}:{}",company.id,contract.id),
                format!("{}: {}",company.name,model_name(w,me,&contract.target_revision)),
                Some(contract.escrow_bn),&contract.status,
                "Held until completed conversions earn payment or an eligible refund settles".into(),None));
        }
        for product in company.products.iter().filter(|p|
            p.certified_day.is_none() && p.cancelled_day.is_none()) {
            let remaining = equipment::profile(nation,&product.revision_id)
                .map(|p|(p.development_cost_bn-product.development_spent_bn).max(0.0));
            development.push(item(format!("development:{}:{}",company.id,product.id),
                format!("{}: {}",company.name,model_name(w,me,&product.revision_id)),
                remaining,&product.status,
                "Conditional on future R&D funding and qualified work; no fixed payment date".into(),None));
        }
    }

    let mut imports = vec![];
    for contract in w.companies.imports.contracts.iter().filter(|d|d.buyer==me) {
        let label=if let Some(family)=&contract.family {
            format!("{} rounds: {} from {}",contract.quantity,family,contract.seller.name())
        } else {
            format!("{} × {} from {}",contract.quantity,contract.source_revision.name,contract.seller.name())
        };
        if contract.settled_day.is_none() {
            transfers.push(item(format!("import-transfer:{}",contract.id),label,
                Some(contract.total_price_bn),&contract.status,
                if contract.cancelled_day.is_some() {
                    format!("Original payment settles after fiscal close of {}; its exact refund then settles once",date(contract.purchased_day))
                } else {
                    format!("Import payment enters protected escrow after fiscal close of {}",date(contract.purchased_day))
                },None));
        } else if contract.escrow_bn>0.0 {
            let due=contract.due_day.map_or_else(||"No shipping date recorded".into(),
                |day|format!("Scheduled arrival {}; holds can delay delivery",date(day)));
            imports.push(item(format!("import-escrow:{}",contract.id),label,Some(contract.escrow_bn),
                &contract.status,format!("{due}. {}",contract.reason),None));
        }
    }

    let mut deliveries = vec![];
    for delivery in w.companies.deliveries.iter().filter(|d|
        d.buyer==me && d.settled_day.is_some() && d.delivered_day.is_none()) {
        deliveries.push(item(format!("equipment-delivery:{}",delivery.id),
            format!("{} × {}",delivery.quantity,model_name(w,me,&delivery.revision_id)),
            Some(delivery.total_price_bn),&delivery.status,
            delivery.due_day.map_or_else(||"No shipping date recorded".into(),
                |day|format!("Scheduled arrival {}; holds can delay delivery",date(day))),None));
    }
    for delivery in w.companies.ammunition_deliveries.iter().filter(|d|
        d.buyer==me && d.settled_day.is_some() && d.delivered_day.is_none()) {
        deliveries.push(item(format!("ammunition-delivery:{}",delivery.id),
            format!("{} rounds: {}",delivery.quantity,delivery.family),
            Some(delivery.total_price_bn),&delivery.status,
            delivery.due_day.map_or_else(||"No shipping date recorded".into(),
                |day|format!("Scheduled arrival {}; holds can delay delivery",date(day))),None));
    }

    let mut goods_escrow = vec![];
    let mut goods_cargo = vec![];
    if let Some(commerce) = &w.commerce {
        for contract in commerce.contracts.iter().filter(|c|c.buyer==me && c.escrow_bn>0.0) {
            goods_escrow.push(item(format!("goods-escrow:{}",contract.id),
                format!("{} from {}",contract.good.name(),contract.seller.name()),
                Some(contract.escrow_bn),&contract.status,
                format!("Dispatch window ends {}; this is not an arrival date",date(contract.expires_day)),Some("trade")));
        }
        for cargo in commerce.cargo.iter().filter(|c|c.buyer==me) {
            goods_cargo.push(item(format!("goods-cargo:{}",cargo.id),
                format!("{} from {} (shipment {})",cargo.good.name(),cargo.seller.name(),cargo.id),None,
                if cargo.hold_reason.is_some(){"held_in_transit"}else{"in_transit"},
                format!("Scheduled arrival {}; holds can delay delivery",date(cargo.due_day)),Some("trade")));
        }
    }

    let construction = production::projects_for(w,me).map(|p| {
        // Match industry::funding_for without building the expensive global
        // project_plans map for every visible row. Old paid progress remains paid.
        let remaining = clock::is_daily(w).then(|| {
            let spent = w.production.industry.projects.get(&p.id).map_or_else(
                ||industry::project_cost_bn(p)*p.progress_fraction(),|f|f.spent_bn);
            (industry::contract_cost_bn(w,p)-spent).max(0.0)
        });
        item(format!("construction:{}",p.id),format!("{}: {}",p.district,production::catalog(p.kind).name),
            remaining,p.status.key(),"Conditional on future construction funding; no fixed payment date".into(),Some("construction"))
    }).collect();

    let public_work = nation.equipment.as_ref().map_or_else(Vec::new,|state|
        state.projects.iter().filter(|p|!matches!(p.status,
            equipment::ProjectStatus::Complete|equipment::ProjectStatus::Cancelled)).map(|p| {
            let status = match p.status {
                equipment::ProjectStatus::Working => "working",
                equipment::ProjectStatus::Paused => "paused",
                equipment::ProjectStatus::Blocked => "blocked",
                equipment::ProjectStatus::Complete => "complete",
                equipment::ProjectStatus::Cancelled => "cancelled",
            };
            item(format!("public-equipment:{}",p.id),model_name(w,me,&p.revision_id),
                Some((p.cost_bn-p.spent_bn).max(0.0)),status,
                "Conditional on future department funding and required inputs; no fixed payment date".into(),None)
        }).collect());

    vec![
        category("pending_fiscal_close","Approved spending awaiting fiscal close",
            "Fresh departmental expense already recorded on the open funding day. Prepaid funds are excluded. Tax revenue and interest determine the eventual net cash/debt movement. Supplier transfers below may describe this same expense; do not add them again.",pending),
        category("supplier_transfers","Supplier transfers awaiting settlement",
            "Accepted invoices already consume the approved programme ledger. Some can use previously paid funds. These transfers are not an additional Treasury bill and have no saved per-invoice split between fresh and prepaid money.",transfers),
        category("paid_equipment_deliveries","Paid domestic equipment awaiting delivery",
            "Historical purchase prices of settled domestic equipment and ammunition still in transit. Already paid; arrival does not charge this price again. Unsettled purchases are listed only under supplier transfers; foreign lots appear under paid import escrow. Company-owned unsold stock is excluded.",deliveries),
        category("paid_import_escrow","Paid imports: owned stock and held funds",
            "The buyer owns these reserved foreign equipment or ammunition lots. Their original payment is held in protected escrow until delivery earns the seller's payment or an eligible cancellation refunds it. This is already paid, not another Treasury bill. Delivered/refunded lots and unsettled purchases are excluded.",imports),
        category("refit_escrow","Paid refit deposits still held",
            "Unearned public money already paid into locked service deposits. Company working capital is a separate pool and is excluded. Completed work earns these deposits; only eligible cancelled units can refund them.",refits),
        category("goods_escrow","Paid goods money awaiting dispatch",
            "Cash already removed from the Treasury for undispatched manufactured-goods purchases. Dispatch transfers this escrow to the seller; it does not charge the buyer again. Dispatch deadlines are not delivery promises.",goods_escrow),
        category("paid_goods_cargo","Paid goods awaiting arrival",
            "Buyer-owned goods already dispatched; the seller was paid at dispatch. The save has no separate paid-value field for each shipment, so no cash amount is invented. This cargo is excluded from undispatched escrow and is not a future bill.",goods_cargo),
        category("construction_remaining","Building work still to fund",
            "Remaining frozen base contract prices for active building projects, including paused work. Paid progress is excluded. This is conditional work over each project's lifetime, not a bill due now; prospective contractor fees are extra and are not fixed here. Legacy monthly projects have no quoted cash total.",construction),
        category("development_remaining","Supplier development still to fund",
            "Remaining frozen public R&D fees for unfinished, uncancelled models. Already invoiced work is excluded even before transfer. Future funding and work are conditional. Company tooling, stock production and optional later purchases are separate.",development),
        category("public_equipment_remaining","Public equipment work still to fund",
            "Remaining base payments on retained public development, production and refit jobs. These are conditional lifetime commitments, not invoices. Prospective contractor fees and separately acquired inputs are excluded; paid deliveries and company refit deposits are not counted here.",public_work),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use spheres_sim::{apply_command, companies, commerce, init::world_1990, logistics, save, Command,
        world::{GameRules,BUDGET_DEFENSE}};

    fn fixture() -> WorldState {
        let mut w=world_1990(GameRules{daily_simulation:true,..Default::default()});
        w.player=Some(NationId::France);w
    }
    fn firm() -> companies::Company {
        companies::Company{id:1,nation:NationId::France,name:"Test Works".into(),ownership:"private".into(),
            district:"FR-IDF".into(),established_day:0,cash_bn:100.0,capital_received_bn:100.0,
            development_revenue_bn:0.0,sales_revenue_bn:0.0,development_expense_bn:0.0,
            tooling_expense_bn:0.0,materials_expense_bn:0.0,fabrication_expense_bn:0.0,
            receivables:vec![],products:vec![],ammunition_products:vec![],refits:vec![],
            refit_advances_received_bn:0.0,refit_revenue_bn:0.0,refit_refunds_bn:0.0,
            transactions:vec![],last_tick_day:None}
    }
    fn product() -> companies::Product {
        companies::Product{id:1,design_nation:NationId::France,revision_id:"missing".into(),
            status:"blocked".into(),reason:"Missing revision".into(),daily_budget_bn:0.0,stock_target:12,
            development_work_days:0.0,development_spent_bn:0.0,tooling_work_days:0.0,tooling_spent_bn:0.0,
            unit_work_days:0.0,unit_spent_bn:0.0,unit_material_cost_bn:0.0,unit_inputs:[0.0;12],
            stock:12,stock_cost_bn:99.0,produced_units:12,sold_units:0,started_day:0,
            certified_day:None,cancelled_day:None}
    }
    fn group<'a>(rows:&'a [Value],id:&str)->&'a Value {
        rows.iter().find(|row|row["id"]==id).unwrap()
    }
    fn delivery(id:u32,settled:bool,arrived:bool)->companies::Delivery {
        companies::Delivery{id,company:1,buyer:NationId::France,product:1,revision_id:"model-a".into(),
            district:"FR-IDF".into(),quantity:2,unit_price_bn:1.5,total_price_bn:3.0,cost_basis_bn:2.0,
            purchased_day:0,settled_day:settled.then_some(0),due_day:settled.then_some(7),
            delivered_day:arrived.then_some(7),status:if settled{"in_transit"}else{"awaiting_settlement"}.into(),reason:String::new()}
    }
    #[test]
    fn accepted_supplier_invoice_is_not_added_to_fresh_fiscal_cash_or_paid_delivery() {
        let mut w=fixture();let me=NationId::France;
        let allocations=w.nation(me).budget_for(w.year).allocations;
        apply_command(&mut w,&Command::SetProgramBudget{nation:me,fiscal_year:1990,
            allocations,departments:programs::default_departments()}).unwrap();
        let p=w.nation_mut(me).program_budget.as_mut().unwrap();
        p.day=Some(0);p.settled_day=None;p.spent_today_bn=programs::ZERO;
        p.spent_today_bn[BUDGET_DEFENSE][3]=0.4;p.prepaid_used_today_bn[BUDGET_DEFENSE][3]=4.6;
        p.available_bn[BUDGET_DEFENSE][3]=100.0;p.prepaid_bn[BUDGET_DEFENSE][3]=50.0;
        let mut c=firm();c.receivables.push(companies::Receivable{id:2,day:0,kind:"sale".into(),
            amount_bn:5.0,product:Some(1),delivery:Some(3),refit:None});
        let mut foreign=c.clone();foreign.id=2;foreign.nation=NationId::Japan;
        w.companies.firms=vec![c,foreign];w.companies.deliveries.push(delivery(3,false,false));
        let saved=save(&w);let rows=view(&w,me);
        assert_eq!(group(&rows,"pending_fiscal_close")["amount_bn"],0.4);
        assert_eq!(group(&rows,"supplier_transfers")["amount_bn"],5.0);
        assert_eq!(group(&rows,"paid_equipment_deliveries")["amount_bn"],0.0);
        assert!(group(&rows,"supplier_transfers")["items"][0].get("action").is_none());
        assert_eq!(view(&w,me),rows);assert_eq!(save(&w),saved);
        w.nation_mut(me).program_budget.as_mut().unwrap().settled_day=Some(0);
        assert_eq!(group(&view(&w,me),"pending_fiscal_close")["amount_bn"],0.0);
    }
    #[test]
    fn imported_commitments_follow_original_payment_escrow_and_release_without_an_extra_bill() {
        // Read-model fixture only: exact synthetic ownership stages exercise
        // categorisation; native S08 lifecycle tests own payment conservation.
        let mut w=fixture();let me=NationId::France;
        let spec=equipment::baseline_spec();
        let revision=equipment::DesignRevision{id:"foreign-model".into(),name:"Foreign model".into(),
            specification_key:equipment::specification_key(&spec),profile:equipment::design_preview(&w,me,&spec).profile.unwrap(),spec,created_day:0,certified_day:Some(0)};
        let route=logistics::RoutePlan{mode:"sea".into(),nodes:vec![],segments:vec![],distance_km:100,
            estimated_days:14,months:1,capacity_tonnes:50.0,bottleneck:"Test corridor".into(),chokepoints:vec![],dispatch_note:None};
        let mut d=companies::ImportContract{id:1,seller:NationId::Japan,buyer:me,company:1,product:2,ammunition:false,
            quantity:2,district:"JP-test".into(),source_revision:revision,buyer_revision:Some("import-1-2".into()),family:None,
            route,transit_days:14,unit_price_bn:1.5,total_price_bn:3.0,cost_basis_bn:3.0/1.15,purchased_day:0,
            settled_day:None,due_day:None,delivered_day:None,cancelled_day:None,refunded_day:None,escrow_bn:0.0,
            refunded_bn:0.0,status:"awaiting_settlement".into(),reason:"Awaiting fiscal close".into()};
        w.companies.imports.contracts.push(d.clone());
        let saved=save(&w);let rows=view(&w,me);
        assert_eq!(group(&rows,"supplier_transfers")["amount_bn"],3.0);
        assert_eq!(group(&rows,"pending_fiscal_close")["amount_bn"],0.0,"No extra fresh bill is inferred from a foreign invoice");
        assert_eq!(group(&rows,"paid_import_escrow")["amount_bn"],0.0);
        assert_eq!(group(&view(&w,NationId::Tonga),"supplier_transfers")["amount_bn"],0.0);
        assert_eq!(save(&w),saved);
        d.settled_day=Some(0);d.due_day=Some(20);d.escrow_bn=3.0;
        d.status="blocked".into();d.reason="Original shipping province unavailable".into();
        w.companies.imports.contracts[0]=d.clone();
        let saved=save(&w);let rows=view(&w,me);
        assert_eq!(group(&rows,"supplier_transfers")["amount_bn"],0.0);
        assert_eq!(group(&rows,"paid_equipment_deliveries")["amount_bn"],0.0,"The same foreign lot is not counted again as domestic delivery");
        assert_eq!(group(&rows,"paid_import_escrow")["amount_bn"],3.0);
        assert!(group(&rows,"paid_import_escrow")["items"][0]["due_label"].as_str().unwrap().contains("Original shipping province unavailable"));
        assert_eq!(save(&w),saved);
        for refund in [false,true] {
            let mut ended=d.clone();ended.escrow_bn=0.0;
            if refund {ended.cancelled_day=Some(5);ended.refunded_day=Some(5);ended.refunded_bn=3.0;}
            else {ended.delivered_day=Some(20);}
            w.companies.imports.contracts[0]=ended;
            let rows=view(&w,me);
            assert_eq!(group(&rows,"paid_import_escrow")["amount_bn"],0.0);
            assert_eq!(group(&rows,"supplier_transfers")["amount_bn"],0.0);
        }
    }
    #[test]
    fn paid_property_is_separate_from_fresh_bills_and_cargo_does_not_invent_a_price() {
        let mut w=fixture();let me=NationId::France;
        w.companies.deliveries=vec![delivery(1,true,false),delivery(2,true,true),delivery(3,false,false)];
        let mut c=firm();
        c.refits.push(companies::RefitContract{id:8,product:1,source_revision:"old".into(),target_revision:"new".into(),
            quantity:1,completed_units:0,cancelled_units:0,unit_days:2,unit_fabrication_cost_bn:0.1,
            unit_material_quote_bn:0.1,unit_price_bn:0.7,total_price_bn:0.7,recipe_per_unit:[0.0;12],
            advance_received_bn:0.7,escrow_bn:0.7,earned_revenue_bn:0.0,refunded_bn:0.0,
            working_capital_locked_bn:50.0,unit_started_day:None,unit_work_days:0.0,unit_inputs:[0.0;12],
            unit_material_cost_bn:0.0,materials_expense_bn:0.0,fabrication_expense_bn:0.0,resources_used:[0.0;12],
            booked_day:0,settled_day:Some(0),cancelled_day:None,refund_day:None,closed_day:None,
            last_work_day:None,status:"queued".into(),reason:"Waiting".into()});
        w.companies.firms.push(c);
        let mut book=commerce::Commerce::default();
        book.contracts.push(commerce::Contract{id:1,buyer:me,seller:NationId::Japan,good:commerce::Good::Intermediates,
            quantity:5.0,unit_price_bn:1.0,remaining_quantity:2.0,escrow_bn:2.0,delivered_quantity:0.0,
            cancelled_quantity:0.0,paid_bn:3.0,accepted_day:0,expires_day:30,status:"partially_dispatched".into(),reason:None});
        let route=logistics::RoutePlan{mode:"sea".into(),nodes:vec![],segments:vec![],distance_km:100,
            estimated_days:7,months:1,capacity_tonnes:50.0,bottleneck:"Test corridor".into(),
            chokepoints:vec![],dispatch_note:None};
        book.cargo.push(commerce::Cargo{id:2,contract:1,buyer:me,seller:NationId::Japan,
            good:commerce::Good::Intermediates,quantity:3.0,route,dispatched_day:0,due_day:7,hold_reason:Some("Closed corridor".into())});
        w.commerce=Some(book);
        let rows=view(&w,me);
        assert_eq!(group(&rows,"paid_equipment_deliveries")["amount_bn"],3.0);
        assert_eq!(group(&rows,"refit_escrow")["amount_bn"],0.7);
        assert_eq!(group(&rows,"goods_escrow")["amount_bn"],2.0);
        assert_eq!(group(&rows,"pending_fiscal_close")["amount_bn"],0.0);
        assert!(group(&rows,"paid_goods_cargo")["amount_bn"].is_null());
        assert_eq!(group(&rows,"paid_goods_cargo")["items"][0]["status"],"held_in_transit");
        assert!(group(&rows,"paid_goods_cargo")["note"].as_str().unwrap().contains("seller was paid at dispatch"));
    }
    #[test]
    fn building_totals_use_frozen_paid_terms_and_include_hidden_paused_projects() {
        let mut w=fixture();let me=NationId::France;
        for id in 1..=10 {
            w.production.projects.push(production::Project{id,nation:me,district:format!("province-{id}"),
                kind:production::ProjectKind::Infrastructure,priority:production::Priority::Normal,
                status:production::ProjectStatus::Paused,reason:None,progress_days:50.0,total_days:100,
                resources_used:[0.0;12],capacity_micros:None,started_day:Some(0)});
            w.production.industry.projects.insert(id,industry::ProjectFunding{spent_bn:2.0,contract_cost_bn:Some(10.0),..Default::default()});
        }
        let saved=save(&w);let rows=view(&w,me);let c=group(&rows,"construction_remaining");
        assert_eq!(c["amount_bn"],80.0);assert_eq!(c["items"].as_array().unwrap().len(),8);
        assert!(c["note"].as_str().unwrap().contains("8 of 10"));assert_eq!(save(&w),saved);
        w.production.projects.truncate(1);w.production.industry.projects.clear();
        let remaining=industry::project_cost_bn(&w.production.projects[0])*0.5;
        assert_eq!(group(&view(&w,me),"construction_remaining")["amount_bn"],remaining);
        w.rules.daily_simulation=false;
        assert!(group(&view(&w,me),"construction_remaining")["amount_bn"].is_null(),
            "An inherited monthly project's work fraction is not a saved financial contract");
    }
    #[test]
    fn missing_saved_model_terms_are_unknown_and_cancelled_or_certified_work_is_excluded() {
        let mut w=fixture();let mut c=firm();
        let p=product();
        c.products.push(p.clone());let mut cancelled=p.clone();cancelled.id=2;cancelled.cancelled_day=Some(0);c.products.push(cancelled);
        let mut certified=p;certified.id=3;certified.certified_day=Some(0);c.products.push(certified);w.companies.firms.push(c);
        let rows=view(&w,NationId::France);let dev=group(&rows,"development_remaining");
        assert!(dev["amount_bn"].is_null());assert_eq!(dev["items"].as_array().unwrap().len(),1);
        assert_eq!(group(&rows,"paid_equipment_deliveries")["amount_bn"],0.0,"Unsold company stock is not a public purchase");
    }
    #[test]
    fn unfinished_model_and_public_work_use_saved_base_prices_without_repricing_or_counting_fees_twice() {
        let mut w=fixture();let me=NationId::France;
        let spec=equipment::baseline_spec();
        let mut profile=equipment::design_preview(&w,me,&spec).profile.unwrap();
        // A saved historical contract need not match today's compiled price.
        profile.development_cost_bn=8.0;
        let revision=equipment::DesignRevision{id:"saved-model".into(),name:"Saved model".into(),
            specification_key:equipment::specification_key(&spec),spec,profile,created_day:0,certified_day:None};
        let state=w.nation_mut(me).equipment.get_or_insert_with(Default::default);
        state.revisions.insert(revision.id.clone(),revision);
        let job=equipment::EquipmentProject{id:10,kind:equipment::ProjectKind::Production,
            revision_id:"saved-model".into(),source_design_id:None,district:Some("FR-IDF".into()),
            quantity:2,completed_units:0,priority:production::Priority::Normal,
            status:equipment::ProjectStatus::Paused,paused:true,reason:"Paused".into(),started_day:0,
            completed_day:None,minimum_days:10,work_days:4.0,cost_bn:10.0,spent_bn:4.0,
            tooling_days:0,tooling_cost_bn:0.0,daily_budget_bn:0.0,recipe_per_unit:[0.0;12],
            resources_used:[0.0;12],company_inputs_saved:[0.0;12],company_fees_bn:0.5,
            last_spent_bn:0.0,last_day:None};
        state.projects.push(job.clone());
        for (id,status) in [(11,equipment::ProjectStatus::Complete),(12,equipment::ProjectStatus::Cancelled)] {
            let mut ended=job.clone();ended.id=id;ended.status=status;state.projects.push(ended);
        }
        let mut c=firm();let mut p=product();p.revision_id="saved-model".into();p.development_spent_bn=3.0;
        c.products.push(p);c.receivables.push(companies::Receivable{id:9,day:0,kind:"development".into(),
            amount_bn:3.0,product:Some(1),delivery:None,refit:None});w.companies.firms.push(c);
        let before=save(&w);let rows=view(&w,me);
        assert_eq!(group(&rows,"development_remaining")["amount_bn"],5.0,
            "Invoiced work is removed from future development even before supplier transfer");
        assert_eq!(group(&rows,"supplier_transfers")["amount_bn"],3.0);
        assert_eq!(group(&rows,"public_equipment_remaining")["amount_bn"],6.0);
        assert_eq!(group(&rows,"public_equipment_remaining")["items"].as_array().unwrap().len(),1);
        assert_eq!(save(&w),before);
    }
}
