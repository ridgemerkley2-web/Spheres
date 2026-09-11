//! Read-only fiscal recovery presentation. Cash and forecasts come from the sim.
use serde_json::{json, Value};
use spheres_sim::{clock, fiscal_recovery, world::{NationId, WorldState}};

pub(crate) fn view(w: &WorldState, id: NationId) -> Value {
    let Some(n) = w.nation_opt(id).filter(|n| n.alive && w.player == Some(id)) else {
        return json!({"available":false,"reason":"Choose the current player government."});
    };
    let enabled = fiscal_recovery::enabled(w);
    let book = w.fiscal_recovery.nations.get(&id);
    let can_enable = !w.rules.fiscal_recovery && clock::is_daily(w);
    json!({
        "available":true,"nation":id.code(),"name":id.name(),"date":w.date_str(),
        "enabled":enabled,"enrolled":w.rules.fiscal_recovery,
        "assessment":fiscal_recovery::assessment(w,id),
        "balances":{"treasury_bn":n.treasury_bn,"debt_bn":n.debt_bn,"debt_gdp":n.debt_gdp},
        "last_observed_day":book.and_then(|f|f.last_day),
        "observed_month_fraction":book.map_or(0.0,|f|f.observed_month_fraction),
        "latest_closed_observation":book.and_then(|f|f.observations.last()),
        "current_partial_observation":book.and_then(|f|f.accumulating.as_ref()),
        "enrollment":{"allowed":can_enable,"command":can_enable.then(||json!({"kind":"enable_fiscal_recovery"})),
            "reason":if enabled {"Fiscal recovery is already enabled."}
                else if w.rules.fiscal_recovery {"Fiscal recovery requires the daily campaign calendar."}
                else if !clock::is_daily(w) {"Finish the daily-calendar transition before enabling fiscal recovery."}
                else {"Review adoption of fiscal recovery for this campaign."},
            "effect":"Enable daily fiscal observation for all living countries. Preserve existing cash and debt; unopened treasuries receive no cash and recognize debt from their current debt ratio. Actual interest and deficits remain payable. Monthly reviews may gradually affect government confidence after observed adjustment time. This changes campaign rules and does not enact taxes, service cuts or debt relief."},
        "actions":[{"id":"tax","label":"Review taxes"},{"id":"budget","label":"Review ministry spending"},
            {"id":"construction","label":"Review construction funding"},{"id":"plays","label":"Review Cabinet recovery actions"}],
        "accounting_note":"This is a reading of posted public cash flows. Company property remains separate. Construction and supplier invoices enter their existing fiscal receipt once; unused authorization is not spending. Forecasts are scenarios, not future bills or automatic orders."
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use spheres_sim::{init::world_1990, save, world::GameRules};

    #[test]
    fn view_is_current_player_only_and_never_enrolls_or_changes_money() {
        let mut w=world_1990(GameRules { daily_simulation:true,..Default::default() });
        w.player=Some(NationId::France);
        let before=save(&w);
        let first=view(&w,NationId::France);
        assert_eq!(first["enrollment"]["allowed"],true);
        assert!(first["assessment"].is_null());
        assert_eq!(view(&w,NationId::Japan)["available"],false);
        assert_eq!(view(&w,NationId::France),first);
        assert_eq!(save(&w),before);
        fiscal_recovery::enable(&mut w);
        let before=save(&w);
        let enabled=view(&w,NationId::France);
        assert_eq!(enabled["enrollment"]["allowed"],false);
        assert_eq!(enabled["assessment"],serde_json::to_value(fiscal_recovery::assessment(&w,NationId::France)).unwrap());
        assert_eq!(save(&w),before);
    }
}
