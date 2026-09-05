//! Optional campaign aims observe the model, never reward it with invented output.
//! Targets freeze at selection; evaluation runs once per fully settled daily date.
use crate::{clock, domination, resources, tech, world::*};
use serde::{Deserialize, Serialize};

pub const PROSPERITY_GAIN: f64 = 0.20;
pub const SCIENCE_DISCOVERIES: usize = 8;
pub const PARTNERSHIP_GAIN: usize = 2;
pub const SUSTAIN_DAYS: u32 = 90;
pub const STABILITY_DAYS: u32 = 365;
const HISTORY_LIMIT: usize = 24;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Aim { Prosperity, Science, Stability, Supply, DiplomaticLeadership, Domination }
impl Aim {
    pub const ALL: [Self;6] = [Self::Prosperity,Self::Science,Self::Stability,Self::Supply,Self::DiplomaticLeadership,Self::Domination];
    pub fn title(self)->&'static str {match self {
        Self::Prosperity=>"Shared prosperity",Self::Science=>"Scientific advancement",
        Self::Stability=>"A stable generation of government",Self::Supply=>"Reliable strategic supply",
        Self::DiplomaticLeadership=>"Trusted diplomatic leadership",Self::Domination=>"World domination",
    }}
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Goal {
    pub nation: NationId, pub aim: Aim, pub chosen_day:i32, pub baseline:f64,
    pub target:f64, pub debt_limit:f64, pub starting_technologies:Vec<String>,
    pub hold_days:u32, pub held_days:u32, pub last_day:Option<i32>,
    pub completed_day:Option<i32>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Record { pub goal:Goal,pub ended_day:i32,pub outcome:String }
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CampaignAims {pub active:Option<Goal>,pub history:Vec<Record>}
impl CampaignAims {pub fn is_empty(&self)->bool {self.active.is_none() && self.history.is_empty()}}

fn known(w:&WorldState,nation:NationId)->Vec<String> {
    w.nation(nation).tech.known.iter().filter_map(|i|tech::registry().get(*i as usize)).map(|t|t.id.into()).collect()
}
fn partnerships(w:&WorldState,nation:NationId)->usize {
    w.nations.iter().filter(|other| other.alive && other.id!=nation && w.relation(nation,other.id)>=55.0
        && !w.is_sanctioning(nation,other.id) && !w.is_sanctioning(other.id,nation)
        && (w.allied(nation,other.id) || w.trade_depth(nation,other.id)>0.0
            || w.domination.compacts.iter().any(|c|(c.patron==nation && c.partner==other.id)||(c.partner==nation && c.patron==other.id)))).count()
}
fn gdp_per_capita(w:&WorldState,nation:NationId)->f64 {
    let n=w.nation(nation); n.gdp*1000.0/n.population.max(1e-9)
}
fn prepare(w:&WorldState,nation:NationId,aim:Aim)->Goal {
    let n=w.nation(nation);
    let technologies=known(w,nation);
    let (baseline,target,hold_days)=match aim {
        Aim::Prosperity=>{let value=gdp_per_capita(w,nation);(value,value*(1.0+PROSPERITY_GAIN),SUSTAIN_DAYS)},
        Aim::Science=>(0.0,SCIENCE_DISCOVERIES.min(tech::registry().len().saturating_sub(technologies.len())) as f64,1),
        Aim::Stability=>(n.stability,(n.stability+10.0).clamp(75.0,90.0),STABILITY_DAYS),
        Aim::Supply=>(0.0,1.0,SUSTAIN_DAYS),
        Aim::DiplomaticLeadership=>{let count=partnerships(w,nation);let available=w.nations.iter().filter(|o|o.alive && o.id!=nation).count();
            (count as f64,(count+PARTNERSHIP_GAIN).min(available) as f64,SUSTAIN_DAYS)},
        Aim::Domination=>(domination::status(w,nation).progress,1.0,1),
    };
    Goal{nation,aim,chosen_day:clock::absolute_day(w),baseline,target,debt_limit:n.debt_gdp+0.20,
        starting_technologies:if aim==Aim::Science {technologies} else {vec![]},hold_days,held_days:0,last_day:None,completed_day:None}
}
fn availability(w:&WorldState,g:&Goal)->Option<String> {
    if !clock::is_daily(w) {return Some("Campaign aims require daily simulation. Resume daily play before choosing an aim.".into());}
    if g.aim==Aim::Science && g.target<=0.0 {return Some("Every technology in the registry is already known.".into());}
    if g.aim==Aim::Supply && !(w.rules.resource_market && w.rules.resource_gates) {
        return Some("Enable the strategic resource market and physical resource gates for this aim.".into());
    }
    if g.aim==Aim::DiplomaticLeadership && g.target<=g.baseline {return Some("Every other surviving country is already a trusted partner.".into());}
    None
}
fn player(w:&WorldState,nation:NationId)->Result<(),String> {
    if w.player!=Some(nation) || !w.nation_opt(nation).is_some_and(|n|n.alive) {return Err("A living player government must choose its own campaign aim.".into());} Ok(())
}
pub fn choose(w:&mut WorldState,nation:NationId,aim:Aim)->Result<(),String> {
    player(w,nation)?;
    if w.campaign_aims.active.is_some() {return Err("Continue in sandbox to close the current aim before choosing another.".into());}
    let goal=prepare(w,nation,aim);
    if let Some(why)=availability(w,&goal) {return Err(why);}
    w.campaign_aims.active=Some(goal);
    w.headline(format!("{} chooses the campaign aim: {}. The target is fixed from today's model.",nation.name(),aim.title()));
    Ok(())
}
pub fn continue_sandbox(w:&mut WorldState,nation:NationId)->Result<(),String> {
    player(w,nation)?;
    let Some(goal)=w.campaign_aims.active.as_ref() else {return Ok(())};
    if goal.nation!=nation {return Err("This campaign aim belongs to another government.".into());}
    let goal=w.campaign_aims.active.take().unwrap();
    let outcome=if goal.completed_day.is_some() {"achieved"} else {"set aside"};
    w.campaign_aims.history.push(Record{goal,ended_day:clock::absolute_day(w),outcome:outcome.into()});
    if w.campaign_aims.history.len()>HISTORY_LIMIT {w.campaign_aims.history.remove(0);}
    w.headline(format!("{} continues in sandbox. Recorded achievements remain in its campaign history.",nation.name()));
    Ok(())
}

#[derive(Serialize)]
pub struct Evaluation {pub value:f64,pub target:f64,pub metric:&'static str,pub eligible:bool,pub blockers:Vec<String>,pub progress:f64}
pub fn evaluate(w:&WorldState,g:&Goal)->Evaluation {
    let mut blockers=vec![];
    let Some(n)=w.nation_opt(g.nation).filter(|n|n.alive) else {
        return Evaluation{value:0.0,target:g.target,metric:"government",eligible:false,blockers:vec!["The government no longer exists.".into()],progress:0.0};
    };
    if let Some(why)=availability(w,g) {blockers.push(why);}
    if g.aim!=Aim::Domination && w.at_war(g.nation) {blockers.push("Be at peace.".into());}
    let (value,metric)=match g.aim {
        Aim::Prosperity=>{
            if n.debt_gdp>g.debt_limit {blockers.push(format!("Debt must be at most {:.1}% of GDP.",g.debt_limit*100.0));}
            if n.inflation>0.10 {blockers.push("Inflation must be at most 10%.".into());}
            if n.stability<45.0 {blockers.push("Stability must be at least 45.".into());}
            (gdp_per_capita(w,g.nation),"1990 dollars per person")
        },
        Aim::Science=>(known(w,g.nation).iter().filter(|id|!g.starting_technologies.contains(id)).count() as f64,"new technologies"),
        Aim::Stability=>{
            if n.inflation>0.08 {blockers.push("Inflation must be at most 8%.".into());}
            (n.stability,"stability")
        },
        Aim::Supply=>{
            let demand=resources::tick_draw(w,g.nation).iter().any(|x|*x>1e-9);
            let covered=!resources::action_stalled_mask(w,g.nation).iter().any(|x|*x);
            if !demand {blockers.push("Maintain positive scheduled strategic raw-input demand; stopping all work does not count.".into());}
            if !covered {blockers.push("Cover the next scheduled strategic raw-input bundle.".into());}
            ((demand && covered) as u8 as f64,"scheduled raw-input coverage")
        },
        Aim::DiplomaticLeadership=>{
            if w.reputation(g.nation)<70.0 {blockers.push("Reputation must be at least 70.".into());}
            (partnerships(w,g.nation) as f64,"trusted partner countries")
        },
        Aim::Domination=>{
            let status=domination::status(w,g.nation);
            if !status.victory {blockers.extend(status.incomplete_conditions);}
            (status.progress,"formal world control")
        },
    };
    if value+1e-9<g.target {blockers.push(format!("Reach {:.2} {}; current {:.2}.",g.target,metric,value));}
    let eligible=blockers.is_empty();
    let distance=if g.target>g.baseline {(value-g.baseline)/(g.target-g.baseline)} else {value/g.target.max(1e-9)};
    Evaluation{value,target:g.target,metric,eligible,blockers,progress:distance.clamp(0.0,1.0)}
}
/// Observe the settled day, once. Never count a skipped or repeated date as a
/// sustained day and never alter any economy, RNG, treasury or sovereignty field.
pub fn tick(w:&mut WorldState) {
    let Some(g)=w.campaign_aims.active.as_ref() else {return};
    if g.completed_day.is_some() {return;}
    let today=clock::absolute_day(w);
    if g.last_day==Some(today) {return;}
    let eligible=evaluate(w,g).eligible;
    let g=w.campaign_aims.active.as_mut().unwrap();
    let continuous=g.last_day.is_none_or(|day|today==day+1);
    g.held_days=if eligible {if continuous {g.held_days+1} else {1}} else {0};
    g.last_day=Some(today);
    if g.held_days>=g.hold_days {
        g.completed_day=Some(today);
        let (nation,title)=(g.nation,g.aim.title());
        w.headline(format!("CAMPAIGN AIM ACHIEVED: {} fulfills {}. Review the result in Decisions and continue in sandbox whenever you wish.",nation.name(),title));
    }
}
#[derive(Serialize)]
pub struct AimView {pub aim:Aim,pub title:&'static str,pub description:String,pub target:f64,pub hold_days:u32,pub unavailable:Option<String>}
#[derive(Serialize)]
pub struct View {pub offers:Vec<AimView>,pub active:Option<Goal>,pub evaluation:Option<Evaluation>,pub history:Vec<Record>,pub note:&'static str}
pub fn view(w:&WorldState,nation:NationId)->View {
    let offers=Aim::ALL.into_iter().map(|aim|{
        let g=prepare(w,nation,aim);
        let description=match aim {
            Aim::Prosperity=>format!("Grow real GDP per person 20% from selection to {:.0} (1990 dollars). Hold at peace for 90 days with inflation ≤10%, stability ≥45 and debt no more than 20 GDP percentage points above selection.",g.target),
            Aim::Science=>format!("Acquire {} technologies beyond those known on selection, through the existing research system. Finish at peace.",g.target),
            Aim::Stability=>format!("Maintain stability ≥{:.0}, inflation ≤8% and peace for 365 consecutive days.",g.target),
            Aim::Supply=>"Cover positive scheduled strategic raw-input demand for 90 consecutive peaceful days. This reads the existing resource bundle, not household demand or every manufactured good.".into(),
            Aim::DiplomaticLeadership=>format!("Reach {} distinct trusted partners (relations ≥55 plus an active pact, integrated trade treaty or voluntary compact, without mutual sanctions). Hold peace and reputation ≥70 for 90 days.",g.target),
            Aim::Domination=>domination::VICTORY_RULE.into(),
        };
        AimView{aim,title:aim.title(),description,target:g.target,hold_days:g.hold_days,unavailable:availability(w,&g)}
    }).collect();
    let active=w.campaign_aims.active.as_ref().filter(|g|g.nation==nation).cloned();
    let evaluation=active.as_ref().map(|g|evaluate(w,g));
    View{offers,active,evaluation,history:w.campaign_aims.history.iter().filter(|r|r.goal.nation==nation).cloned().collect(),
        note:"Optional aims grant no bonuses and never end the simulation. Targets freeze when selected; eligible days are consecutive settled calendar dates. Continue in sandbox retains completed results. Existing domination and sovereignty mechanics remain available."}
}
