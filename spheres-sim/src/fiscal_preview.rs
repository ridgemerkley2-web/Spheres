//! Read-only fiscal quotes. Dispatch the proposed policy on a copy so opening
//! debt, ministry authority and rate decisions use the same rules as play.
use crate::{economy, programs, world::*, Command};
use serde::Serialize;

#[derive(Serialize)]
pub struct FiscalQuote {
    pub basis_gdp_bn: f64,
    pub opening_books: bool,
    pub revenue_gdp: f64,
    pub revenue_bn: f64,
    pub interest_gdp: f64,
    pub interest_bn: f64,
    pub effective_rate: f64,
    pub authorized_spend_bn: f64,
    pub total_at_full_use_bn: f64,
    pub balance_at_full_use_bn: f64,
    pub posted_spending_run_rate_bn: f64,
    pub total_political_cost: f64,
    pub affordable: bool,
}

pub fn with_budget(w: &WorldState, nation: NationId, policies: &[Command], budget: &Command)
    -> Result<(programs::ProgramPreview, FiscalQuote), String>
{
    if !matches!(budget, Command::SetProgramBudget { nation: id, .. } if *id == nation) {
        return Err("A complete department budget is required.".into());
    }
    if policies.len() > 32 { return Err("At most 32 policy orders can be quoted.".into()); }
    let mut copy=w.clone();
    let mut total_cost=0.0;
    let mut budget_cost=0.0;
    for command in policies.iter().chain(std::iter::once(budget)) {
        match command {
            Command::SetTaxRate { nation:id, rate } | Command::SetInterestRate { nation:id, rate }
                if *id==nation && rate.is_finite() => {},
            Command::SetProgramBudget { nation:id, .. } if *id==nation && command==budget => {},
            _ => return Err("Only this government's tax and rate orders belong in a budget preview.".into()),
        }
        if let Some(reason)=crate::world_refusal(&copy,command) { return Err(reason); }
        let cost=crate::price_of(&copy,command).unwrap_or(0.0);
        total_cost+=cost;
        if command==budget { budget_cost=cost; }
        // Affordability is reported separately; an unaffordable draft still
        // needs a useful quote. Validation and economic effects are unchanged.
        crate::dispatch(&mut copy,command)?;
    }
    let n=copy.nation(nation);
    let terms=economy::growth_terms(n,n.state_invest_gdp,n.interest_rate,&economy::Conditions::of(&copy,nation));
    let f=economy::Fiscal::of(n,&terms);
    let mut p=programs::preview(&copy,nation);
    p.political_cost=budget_cost;
    let revenue=f.revenue_gdp*n.gdp;
    let total=p.annual_authorized_bn+f.interest_bn;
    let quote=FiscalQuote {
        basis_gdp_bn:n.gdp,opening_books:!w.nation(nation).on_the_books(),
        revenue_gdp:f.revenue_gdp,revenue_bn:revenue,interest_gdp:f.interest_gdp,
        interest_bn:f.interest_bn,effective_rate:f.effective_rate,
        authorized_spend_bn:p.annual_authorized_bn,total_at_full_use_bn:total,
        balance_at_full_use_bn:revenue-total,posted_spending_run_rate_bn:f.spend_gdp*n.gdp,
        total_political_cost:total_cost,affordable:total_cost<=w.nation(nation).political_capital,
    };
    Ok((p,quote))
}
