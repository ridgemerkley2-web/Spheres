//! Presentation of recorded public money. Quotes and annual authorizations
//! never become payments here; reading this module cannot initialize a book.
use serde_json::{json, Value};
use spheres_sim::{clock, fiscal_journal::{self, CashCause, MoneyDay, PolicyDecision, PolicySnapshot},
    fiscal_recovery::Observation, programs, world::{NationId, WorldState}};

fn date(day: i32) -> String {
    let (y,m,d)=clock::date_from_day(day);
    format!("{y:04}-{m:02}-{d:02}")
}
fn cause(cause: CashCause) -> (&'static str, &'static str) {
    use CashCause::*;
    match cause {
        BudgetSettlement => ("Government budget settlement", "Revenue, ministry spending and net interest settle together. Included supplier payments and prepaid authority are not additional charges."),
        ResourceMarket => ("Resource market", "Actual resource-market payments and receipts."),
        ResourceTransfer => ("Resource transfers", "Recorded resource purchases or transfers."),
        GoodsImportEscrow => ("Goods import deposits", "Cash reserved for accepted goods contracts; this is held money until dispatch or refund."),
        GoodsExportReceipt => ("Goods export receipts", "Payment received when contracted export cargo is dispatched; arrival is tracked separately."),
        GoodsRefund => ("Goods contract refunds", "Previously reserved goods money returned to the public account."),
        FreightService => ("Freight services", "Recorded transport charges."),
        PactUpkeep => ("Pact upkeep", "Recorded treaty and pact obligations."),
        ForeignAid => ("Foreign aid programmes", "Recorded patron expense supporting a client country's economy and forces; this is not a direct deposit into its treasury."),
        CovertOperation => ("Covert operations", "Recorded intelligence-operation payments."),
        Patronage => ("Political patronage", "Recorded patronage expense."),
        SupplierInputs => ("Supplier input receipts", "Company payments for public warehouse stock or utility services. Company balances remain separate."),
        RefitRefund => ("Refit refunds", "Unused prepaid refit money returned to the public account."),
        DebtRestructuring => ("Debt restructuring", "Noncash debt relief from approved restructuring. Cancelled debt is not cash revenue."),
        AssetSale => ("Public asset sales", "Actual public proceeds from the approved sale."),
        LegacyMineConstruction => ("Existing mine construction", "Recorded construction expense under the campaign's existing rules."),
        Other => ("Other recorded transactions", "A recorded public payment without a more specific category."),
    }
}
fn receipt(row: &MoneyDay, current_day: i32) -> Value {
    let mut entries:Vec<Value>=row.flows.iter().map(|(&key,flow)| {
        let (label,note)=cause(key);
        let mut entry=json!({"id":key,"label":label,"payment_bn":flow.outflow_bn-flow.inflow_bn,
            "inflow_bn":flow.inflow_bn,"outflow_bn":flow.outflow_bn,
            "cash_change_bn":flow.treasury_delta_bn,"debt_change_bn":flow.debt_delta_bn,
            "count":flow.postings,"note":note});
        if key==CashCause::DebtRestructuring {
            entry["payment_bn"]=Value::Null;entry["inflow_bn"]=Value::Null;entry["outflow_bn"]=Value::Null;
        }
        if key==CashCause::BudgetSettlement {
            if let Some(f)=&row.fiscal {
                entry["fiscal"]=json!(f);
                entry["note"]=json!(format!("Revenue ${:.6} bn; ministry spending ${:.6} bn; net interest ${:.6} bn (negative means a credit). Construction ${:.6} bn is included in spending. Previously paid authority used: ${:.6} bn, excluded from fresh spending. Supplier receipts included in this settlement: ${:.6} bn; these may include that prepaid authority and are not another bill.",
                    f.revenue_bn,f.spending_bn,f.interest_bn,f.construction_bn,f.prepaid_used_bn,f.supplier_inclusions_bn.values().sum::<f64>()));
            }
        }
        entry
    }).collect();
    if row.unrecorded_cash_delta_bn!=0.0 || row.unrecorded_debt_delta_bn!=0.0 {
        entries.push(json!({"id":"unrecorded","label":"Balance change outside recorded payments",
            "payment_bn":null,"inflow_bn":null,"outflow_bn":null,"count":0,
            "cash_change_bn":row.unrecorded_cash_delta_bn,"debt_change_bn":row.unrecorded_debt_delta_bn,
            "note":"These balances changed outside a tagged payment. The difference is shown explicitly; its cause is not reconstructed."}));
    }
    let cash=row.closing_treasury_bn-row.opening_treasury_bn;
    let debt=row.closing_debt_bn-row.opening_debt_bn;
    json!({"day":row.day,"label":date(row.day),"period_note":if row.day==current_day {"Today so far"}else{"Recorded day"},
        "opening_treasury_bn":row.opening_treasury_bn,"opening_debt_bn":row.opening_debt_bn,
        "closing_treasury_bn":row.closing_treasury_bn,"closing_debt_bn":row.closing_debt_bn,
        "cash_change_bn":cash,"debt_change_bn":debt,"net_change_bn":cash-debt,"entries":entries})
}

#[cfg(test)]
mod tests {
    use super::*;
    use spheres_sim::{apply_command, economy, fiscal_recovery, init::world_1990, load, save, tick_day, Command, world::GameRules};
    fn world()->WorldState {
        let mut w=world_1990(GameRules{daily_simulation:true,..Default::default()});
        w.player=Some(NationId::France);w
    }
    fn close(a:f64,b:f64) {assert!((a-b).abs()<1e-10,"{a} != {b}");}
    #[test]
    fn old_accounts_and_other_countries_are_not_created_by_reads() {
        let mut w=world();let before=save(&w);
        assert_eq!(view(&w,NationId::France)["available"],false);
        assert_eq!(view(&w,NationId::Japan)["available"],false);
        assert_eq!(save(&w),before);
        fiscal_recovery::enable(&mut w);let before=save(&w);
        assert_eq!(view(&w,NationId::France)["available"],false);
        assert_eq!(save(&w),before);
    }
    #[test]
    fn payments_reconcile_actual_cash_and_debt_and_survive_reload_without_preview_writes() {
        let mut w=world();let id=NationId::France;fiscal_recovery::enable(&mut w);
        w.nation_mut(id).treasury_bn=Some(1.0);w.nation_mut(id).debt_bn=Some(2.0);
        economy::charge_for(&mut w,id,1.5,0.0,CashCause::GoodsImportEscrow);
        economy::charge_for(&mut w,id,-3.0,0.0,CashCause::GoodsRefund);
        let saved=save(&w);let reading=view(&w,id);let r=&reading["reconciliation"];
        close(r["cash_change_bn"].as_f64().unwrap(),-0.5);
        close(r["debt_change_bn"].as_f64().unwrap(),-2.0);
        close(r["net_change_bn"].as_f64().unwrap(),1.5);
        let entries=r["entries"].as_array().unwrap();assert_eq!(entries.len(),2);
        close(entries.iter().map(|e|e["cash_change_bn"].as_f64().unwrap()).sum(),-0.5);
        close(entries.iter().map(|e|e["debt_change_bn"].as_f64().unwrap()).sum(),-2.0);
        assert_eq!(save(&w),saved);let loaded=load(&saved).unwrap();
        assert_eq!(view(&loaded,id),reading);assert_eq!(save(&loaded),saved);
        // A balance edit is disclosed by a pure reading, never hidden as tax or
        // appended to history by the UI endpoint.
        w.nation_mut(id).treasury_bn=Some(0.75);let edited=save(&w);let edited_view=view(&w,id);
        let unknown=edited_view["reconciliation"]["entries"].as_array().unwrap().iter().find(|e|e["id"]=="unrecorded").unwrap();
        close(unknown["cash_change_bn"].as_f64().unwrap(),0.25);
        assert!(unknown["payment_bn"].is_null());assert_eq!(save(&w),edited);
    }
    #[test]
    fn approved_correction_records_settings_and_only_later_observed_outcomes() {
        let mut w=world();let id=NationId::France;fiscal_recovery::enable(&mut w);
        let before=w.nation(id).tax_rate;
        apply_command(&mut w,&Command::SetTaxRate{nation:id,rate:0.28}).unwrap();
        let reading=view(&w,id);let decision=&reading["decisions"][0];
        assert_eq!(decision["before_label"],format!("Tax rate {:.2}%",before*100.0));
        assert_eq!(decision["after_label"],"Tax rate 28.00%");
        assert!(decision["cash_change_bn"].is_null());
        assert!(decision["observed_label"].as_str().unwrap().contains("Awaiting"));
        tick_day(&mut w,&[]);
        let reading=view(&w,id);assert!(reading["decisions"][0]["debt_change_bn"].is_number());
        assert!(reading["decisions"][0]["note"].as_str().unwrap().contains("all events"));
        assert_eq!(view(&load(&save(&w)).unwrap(),id),reading);
    }
    #[test]
    fn fiscal_details_keep_signed_interest_and_inclusions_out_of_cash_legs() {
        use spheres_sim::fiscal_journal::{FiscalDetail, MoneyFlow, SupplierInclusion};
        let row=MoneyDay{day:0,opening_treasury_bn:1.0,opening_debt_bn:0.0,
            closing_treasury_bn:1.5,closing_debt_bn:0.0,unrecorded_cash_delta_bn:0.0,unrecorded_debt_delta_bn:0.0,
            flows:[(CashCause::BudgetSettlement,MoneyFlow{postings:1,inflow_bn:0.5,outflow_bn:0.0,treasury_delta_bn:0.5,debt_delta_bn:0.0})].into(),
            fiscal:Some(FiscalDetail{revenue_bn:3.0,spending_bn:3.5,interest_bn:-1.0,prepaid_used_bn:0.2,construction_bn:0.3,
                supplier_inclusions_bn:[(SupplierInclusion::Equipment,0.4)].into(),..Default::default()})};
        let r=receipt(&row,1);assert_eq!(r["entries"].as_array().unwrap().len(),1);
        assert_eq!(r["entries"][0]["inflow_bn"],0.5);assert_eq!(r["entries"][0]["fiscal"]["interest_bn"],-1.0);
        assert!(r["entries"][0]["note"].as_str().unwrap().contains("not another bill"));
    }
}
const MINISTRIES: [&str;10]=["Health","Education","Housing","Pensions","Infrastructure","Industry","Science","Defense","Security","Diplomacy"];
fn policy_setting(kind:&str,p:&PolicySnapshot)->String {
    match kind {
        "tax_rate" => format!("Tax rate {:.2}%",p.tax_rate*100.0),
        "interest_rate" => format!("Policy interest rate {:.2}%",p.interest_rate*100.0),
        "construction_budget" => p.construction_daily_budget_bn.map_or("No separate construction limit".into(),|v|format!("Construction limit ${v:.6} bn/day")),
        "debt_restructuring"|"asset_sale" => format!("Cash ${:.6} bn; debt ${:.6} bn",p.treasury_bn,p.debt_bn),
        "program_budget" => "Departmental shares of ministry authority".into(),
        _ => format!("Ministry authority {:.2}% of GDP",p.allocations.iter().sum::<f64>()*100.0),
    }
}
fn observation(o:&Observation)->String {
    let y=1990+o.month.div_euclid(12);let m=o.month.rem_euclid(12)+1;
    format!("{y:04}-{m:02}: observed {:.3} of a year; revenue ${:.6} bn, ministry spending ${:.6} bn, net interest ${:.6} bn; closing cash ${:.6} bn and debt ${:.6} bn",
        o.year_fraction,o.revenue_bn,o.spending_bn,o.interest_bn,o.closing_treasury_bn,o.closing_debt_bn)
}
fn decision(d:&PolicyDecision)->Value {
    let label=match d.kind.as_str() {
        "tax_rate"=>"Tax rate reviewed","interest_rate"=>"Interest rate reviewed",
        "construction_budget"=>"Construction funding reviewed","program_budget"=>"Department funding reviewed",
        "debt_restructuring"=>"Debt restructuring approved","asset_sale"=>"Asset sale approved",
        "fiscal_consolidation"=>"Fiscal consolidation approved",
        _=>"Government funding reviewed",
    };
    let mut changes=Vec::new();
    if d.before.tax_rate!=d.after.tax_rate {
        changes.push(format!("Tax rate: {:.2}% → {:.2}%",d.before.tax_rate*100.0,d.after.tax_rate*100.0));
    }
    if d.before.interest_rate!=d.after.interest_rate {
        changes.push(format!("Policy interest rate: {:.2}% → {:.2}%",d.before.interest_rate*100.0,d.after.interest_rate*100.0));
    }
    for (i,name) in MINISTRIES.iter().enumerate() {
        if d.before.allocations[i]!=d.after.allocations[i] {
            changes.push(format!("{name}: {:.2}% → {:.2}% of GDP",d.before.allocations[i]*100.0,d.after.allocations[i]*100.0));
        }
    }
    if let (Some(before),Some(after))=(&d.before.departments,&d.after.departments) {
        for m in 0..10 {for k in 0..programs::DEPARTMENTS {
            if before[m][k]!=after[m][k] {changes.push(format!("{} / {}: {:.2}% → {:.2}% of ministry authority",MINISTRIES[m],programs::NAMES[m][k],before[m][k] as f64/100.0,after[m][k] as f64/100.0));}
        }}
    }
    let outcome=d.latest_outcome.as_ref();
    let immediate_cash=d.after.treasury_bn-d.before.treasury_bn;
    let immediate_debt=d.after.debt_bn-d.before.debt_bn;
    let outcome_label=outcome.map_or("Awaiting the next daily settlement.".into(),|o|format!("Through {}: cash ${:.6} bn; debt ${:.6} bn.",date(o.day),o.treasury_bn,o.debt_bn));
    let monthly=d.latest_observation.as_ref().map(observation).unwrap_or_else(||"No later closed monthly observation yet.".into());
    json!({"id":d.id,"label":label,"date":date(d.day),
        "detail":if changes.is_empty(){"Approved settings recorded. Future spending still follows available authority and actual work.".into()}else{changes.join("; ")},
        "before_label":policy_setting(&d.kind,&d.before),"after_label":policy_setting(&d.kind,&d.after),
        "baseline_label":d.baseline_observation.as_ref().map(observation).unwrap_or_else(||"No closed monthly observation at the time of this decision.".into()),
        "observed_label":format!("{outcome_label} {monthly}"),
        "cash_change_bn":outcome.map(|o|o.treasury_bn-d.before.treasury_bn),
        "debt_change_bn":outcome.map(|o|o.debt_bn-d.before.debt_bn),
        "note":format!("Immediate cash change ${immediate_cash:.6} bn; immediate debt change ${immediate_debt:.6} bn. Balance changes include all events since this choice. Monthly totals cover their full recorded period, which can include days before this decision. These are observed outcomes, not a forecast or an estimate of this policy's effect alone.")})
}
pub(crate) fn view(w:&WorldState,me:NationId)->Value {
    if w.player!=Some(me) {return json!({"available":false,"reason":"Select your own country's money account.","commitments":[],"decisions":[],"recent_days":[],"reconciliation":null});}
    let commitments=crate::money_commitments::view(w,me);
    let Some(mut journal)=fiscal_journal::view(w,me) else {
        return json!({"available":false,"reason":if spheres_sim::fiscal_recovery::enabled(w){"Dated money recording starts with the next actual payment or approved funding decision."}else{"Adopt the connected economy to begin dated money recording."},
            "coverage_note":"Older cash movements are not reconstructed. Reviewing this page does not open accounts, spend money or change campaign rules.",
            "commitments":commitments,"decisions":[],"recent_days":[],"reconciliation":null});
    };
    // A load or external balance edit may leave a live difference since the last
    // posting. Disclose it prospectively instead of claiming the old close is
    // today's cash. This synthetic presentation is never persisted by a read.
    if journal.unrecorded_cash_delta_bn!=0.0 || journal.unrecorded_debt_delta_bn!=0.0 {
        let last=journal.days.last().unwrap();
        if last.day!=journal.current_day {
            journal.days.push(MoneyDay{day:journal.current_day,opening_treasury_bn:last.closing_treasury_bn,
                opening_debt_bn:last.closing_debt_bn,closing_treasury_bn:last.closing_treasury_bn,
                closing_debt_bn:last.closing_debt_bn,unrecorded_cash_delta_bn:0.0,unrecorded_debt_delta_bn:0.0,
                flows:Default::default(),fiscal:None});
        }
        let last=journal.days.last_mut().unwrap();
        last.unrecorded_cash_delta_bn+=journal.unrecorded_cash_delta_bn;
        last.unrecorded_debt_delta_bn+=journal.unrecorded_debt_delta_bn;
        last.closing_treasury_bn=journal.current_treasury_bn;last.closing_debt_bn=journal.current_debt_bn;
    }
    let recent:Vec<_>=journal.days.iter().rev().take(fiscal_journal::RETAINED_DAYS).map(|day|{
        let r=receipt(day,journal.current_day);
        json!({"day":day.day,"label":date(day.day),"cash_change_bn":r["cash_change_bn"],"debt_change_bn":r["debt_change_bn"],"reconciliation":r})
    }).collect();
    json!({"available":true,"reason":"","coverage_note":format!("Recording began {}. Up to {} dated days and {} approved funding decisions are retained. Earlier transactions are not reconstructed. Inflows repay debt before accumulating cash; outflows use cash before adding debt.",date(journal.started_day),fiscal_journal::RETAINED_DAYS,fiscal_journal::RETAINED_DECISIONS),
        "reconciliation":recent.first().map(|r|&r["reconciliation"]),"recent_days":recent,
        "commitments":commitments,"decisions":journal.decisions.iter().rev().map(decision).collect::<Vec<_>>()})
}
