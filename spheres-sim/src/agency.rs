//! Player decisions remain decisions: a persistent diplomatic inbox and explicit
//! monetary commitments. No RNG is used here. Untouched legacy worlds serialize
//! no agency state; their AI-to-AI negotiations retain their existing rules.
use crate::{clock, commitment, statecraft, war, world::*};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Named game rules, not historical measurements. Dates are actual calendar days.
pub const TREATY_REPLY_DAYS: i32 = 30;
pub const DEFENSE_REPLY_DAYS: i32 = 7;
pub const BREAK_PEG_PC: f64 = 12.0;
pub const BREAK_PEG_STABILITY: f64 = 5.0;
pub const BREAK_PEG_INFLATION: f64 = 0.02;
const HISTORY_LIMIT: usize = 32;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponsePolicy { #[default] Review, Accept, Decline }

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StandingPolicy {
    pub defense_pacts: ResponsePolicy,
    pub trade_treaties: ResponsePolicy,
    pub calls_to_arms: ResponsePolicy,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MonetaryRegime { Floating, Pegged { rate: f64 } }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OfferKind {
    DefensePact,
    TradeTreaty,
    CallToArms { conflict: u32, attacker: NationId, year: i32, month: u32,
        rung: u8, guaranteed: bool },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Offer {
    pub id: u64,
    pub from: NationId,
    pub to: NationId,
    pub kind: OfferKind,
    pub issued_day: i32,
    pub expires_day: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Decision { pub offer: Offer, pub resolved_day: i32, pub outcome: String }

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Agency {
    pub next_id: u64,
    pub offers: Vec<Offer>,
    pub history: Vec<Decision>,
    pub policies: BTreeMap<NationId, StandingPolicy>,
    pub monetary: BTreeMap<NationId, MonetaryRegime>,
}
impl Agency {
    pub fn is_empty(&self) -> bool {
        self.next_id == 0 && self.offers.is_empty() && self.history.is_empty()
            && self.policies.is_empty() && self.monetary.is_empty()
    }
}

pub fn policy(w: &WorldState, nation: NationId) -> StandingPolicy {
    w.agency.policies.get(&nation).copied().unwrap_or_default()
}
pub fn set_policy(w: &mut WorldState, nation: NationId, policy: StandingPolicy) -> Result<(), String> {
    player(w, nation)?;
    w.agency.policies.insert(nation, policy);
    w.headline(format!("{} updates its standing diplomatic response policy. Existing requests still await their own answer.", nation.name()));
    Ok(())
}
fn player(w: &WorldState, nation: NationId) -> Result<(), String> {
    if w.player != Some(nation) { return Err("Only the player may direct this decision.".into()); }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) { return Err("The government no longer exists.".into()); }
    Ok(())
}

fn same_request(a: &OfferKind, b: &OfferKind) -> bool {
    match (a,b) {
        (OfferKind::CallToArms {conflict:a,..},OfferKind::CallToArms {conflict:b,..}) => a == b,
        _ => a == b,
    }
}
fn queue(w: &mut WorldState, from: NationId, to: NationId, kind: OfferKind) -> Result<u64,String> {
    if let Some(old) = w.agency.offers.iter().find(|o| o.from==from && o.to==to && same_request(&o.kind,&kind)) {
        return Err(format!("Request #{} already awaits a response.",old.id));
    }
    w.agency.next_id = w.agency.next_id.checked_add(1).ok_or("The request ledger is full.")?;
    let id=w.agency.next_id;
    let days=if matches!(kind,OfferKind::CallToArms{..}) {DEFENSE_REPLY_DAYS} else {TREATY_REPLY_DAYS};
    let offer=Offer{id,from,to,kind,issued_day:clock::absolute_day(w),expires_day:clock::absolute_day(w)+days};
    let title=title(&offer);
    let review=chosen_policy(w,to,&offer.kind)==ResponsePolicy::Review;
    w.agency.offers.push(offer);
    if review { w.headline(format!("ACTION REQUIRED: {} receives {} from {}. Reply within {} days in Decisions; an unanswered request is declined.",to.name(),title,from.name(),days)); }
    Ok(id)
}
fn record(w:&mut WorldState, offer:Offer, outcome:&str) {
    w.agency.offers.retain(|o|o.id!=offer.id);
    w.agency.history.push(Decision{offer,resolved_day:clock::absolute_day(w),outcome:outcome.into()});
    if w.agency.history.len()>HISTORY_LIMIT { w.agency.history.remove(0); }
}
fn chosen_policy(w:&WorldState,to:NationId,kind:&OfferKind)->ResponsePolicy {
    let p=policy(w,to);
    match kind { OfferKind::DefensePact=>p.defense_pacts,OfferKind::TradeTreaty=>p.trade_treaties,
        OfferKind::CallToArms{..}=>p.calls_to_arms }
}

/// Called only after the ordinary treaty's hard conditions passed. Returning
/// true means the human response policy owns the request, with no acceptance die.
pub fn offer_treaty(w:&mut WorldState,from:NationId,to:NationId,kind:OfferKind)->Result<bool,String> {
    if w.player!=Some(to) {return Ok(false);}
    let response=chosen_policy(w,to,&kind);
    let id=queue(w,from,to,kind)?;
    if response!=ResponsePolicy::Review {
        respond(w,to,id,response==ResponsePolicy::Accept)?;
    }
    Ok(true)
}

fn call_matches(c:&Conflict,o:&Offer)->bool {
    matches!(o.kind,OfferKind::CallToArms{conflict,attacker,year,month,..}
        if c.id==conflict && c.origin_attacker==attacker && c.start_year==year
        && c.start_month==month && c.side_b.contains(&o.from))
}
fn call_error(w:&WorldState,c:&Conflict,o:&Offer)->Option<String> {
    if !call_matches(c,o) {return Some("That conflict has ended or changed sides.".into());}
    if c.involves(o.to) {return Some("Your country is already a party to this conflict.".into());}
    let OfferKind::CallToArms{rung,guaranteed,..}=o.kind else {return Some("Not a defense request.".into())};
    if guaranteed && !w.allied(o.to,o.from) {return Some("The defense pact is no longer in force.".into());}
    if c.side_a.iter().any(|foe|crate::sovereignty::hostility_blocked(w,o.to,*foe)) {
        return Some("The request conflicts with your formal sphere.".into());
    }
    let mut proposed=c.clone();
    war::join_side(&mut proposed,o.to,false,1,Objective::Deny);
    commitment::rung_blocked(w,&proposed,o.to,rung)
}
fn resolve_call(w:&mut WorldState,c:&mut Conflict,o:&Offer,accept:bool)->Result<(),String> {
    if accept {
        if let Some(why)=call_error(w,c,o) {return Err(why);}
        let OfferKind::CallToArms{rung,guaranteed,..}=o.kind else {unreachable!()};
        war::join_side(c,o.to,false,rung,Objective::Deny);
        if guaranteed { w.shift_reputation(o.to,5.0); }
        w.shift_relation(o.to,o.from,10.0);
        w.headline(format!("{} accepts {}'s call and enters the war at rung {}.",o.to.name(),o.from.name(),rung));
    } else if matches!(o.kind,OfferKind::CallToArms{guaranteed:true,..}) && w.allied(o.to,o.from) {
        statecraft::decline_guarantee(w,o.to,o.from);
    } else {
        w.headline(format!("{} declines intervention in defense of {}.",o.to.name(),o.from.name()));
    }
    Ok(())
}

/// War temporarily owns its Conflict value outside WorldState. The request
/// therefore carries a durable reference; a policy response can use that value.
pub fn offer_call(w:&mut WorldState,c:&mut Conflict,to:NationId,rung:u8,guaranteed:bool) {
    let kind=OfferKind::CallToArms{conflict:c.id,attacker:c.origin_attacker,
        year:c.start_year,month:c.start_month,rung:rung.clamp(1,9),guaranteed};
    let response=chosen_policy(w,to,&kind);
    let Ok(id)=queue(w,c.defender(),to,kind) else {return};
    if response!=ResponsePolicy::Review {
        let o=w.agency.offers.iter().find(|o|o.id==id).unwrap().clone();
        match resolve_call(w,c,&o,response==ResponsePolicy::Accept) {
            Ok(())=>record(w,o,if response==ResponsePolicy::Accept {"accepted by standing policy"} else {"declined by standing policy"}),
            Err(why)=>w.headline(format!("ACTION REQUIRED: {} cannot apply its standing response: {}",to.name(),why)),
        }
    }
}

pub fn response_error(w:&WorldState,nation:NationId,id:u64,accept:bool)->Option<String> {
    if let Err(why)=player(w,nation) {return Some(why);}
    let Some(o)=w.agency.offers.iter().find(|o|o.id==id && o.to==nation) else {
        return Some("That diplomatic request is no longer pending.".into());
    };
    if clock::absolute_day(w)>=o.expires_day {return Some("The reply deadline has passed.".into());}
    if !w.nation_opt(o.from).is_some_and(|n|n.alive) {return Some("The requesting government no longer exists.".into());}
    if !accept {return None;}
    match o.kind {
        OfferKind::DefensePact=>statecraft::pact_error(w,o.from,o.to).err(),
        OfferKind::TradeTreaty=>statecraft::trade_error(w,o.from,o.to).err(),
        OfferKind::CallToArms{conflict,..}=>match w.conflict(conflict) {
            Some(c)=>call_error(w,c,o),None=>Some("That conflict has ended.".into())
        },
    }
}
pub fn respond(w:&mut WorldState,nation:NationId,id:u64,accept:bool)->Result<(),String> {
    if let Some(why)=response_error(w,nation,id,accept) {return Err(why);}
    let o=w.agency.offers.iter().find(|o|o.id==id).unwrap().clone();
    match o.kind {
        OfferKind::DefensePact if accept=>statecraft::sign_pact(w,o.from,o.to),
        OfferKind::TradeTreaty if accept=>statecraft::sign_trade(w,o.from,o.to),
        OfferKind::CallToArms{conflict,..}=>{
            if let Some(index)=w.conflicts.iter().position(|c|c.id==conflict) {
                let mut c=w.conflicts[index].clone();
                resolve_call(w,&mut c,&o,accept)?;
                w.conflicts[index]=c;
            }
        }
        _=>w.headline(format!("{} declines {} from {}.",o.to.name(),title(&o),o.from.name())),
    }
    record(w,o,if accept {"accepted"} else {"declined"});
    Ok(())
}

/// Call before reusing a conflict id, including a new quarrel in the same month.
pub fn retire_conflict(w:&mut WorldState,id:u32) {
    let old:Vec<_>=w.agency.offers.iter().filter(|o|matches!(o.kind,OfferKind::CallToArms{conflict,..} if conflict==id)).cloned().collect();
    for o in old {record(w,o,"closed: conflict ended");}
}
pub fn tick(w:&mut WorldState) {
    if w.agency.offers.is_empty() {return;}
    let today=clock::absolute_day(w);
    let pending=w.agency.offers.clone();
    for o in pending {
        let alive=[o.from,o.to].iter().all(|id|w.nation_opt(*id).is_some_and(|n|n.alive));
        let valid=alive && match o.kind {
            OfferKind::CallToArms{conflict,guaranteed,..}=>w.conflict(conflict).is_some_and(|c|call_matches(c,&o) && !c.involves(o.to)) && (!guaranteed || w.allied(o.to,o.from)),
            _=>true,
        };
        if !valid {record(w,o,"closed: circumstances changed");continue;}
        if today>=o.expires_day {
            if let OfferKind::CallToArms{conflict,..}=o.kind {
                if let Some(i)=w.conflicts.iter().position(|c|c.id==conflict) {
                    let mut c=w.conflicts[i].clone();
                    let _=resolve_call(w,&mut c,&o,false);
                    w.conflicts[i]=c;
                }
            }
            w.headline(format!("{}'s unanswered {} from {} expires and is declined.",o.to.name(),title(&o),o.from.name()));
            record(w,o,"expired: declined");
        }
    }
}

/// Legacy peg flags record an already-paid decision. Daily adoption freezes its
/// saved rate without replaying the inflation/stability benefit or changing cash.
pub fn pegged_rate(w:&WorldState,nation:NationId)->Option<f64> {
    match w.agency.monetary.get(&nation) {
        Some(MonetaryRegime::Pegged{rate})=>Some(*rate),
        Some(MonetaryRegime::Floating)=>None,
        None if (clock::is_daily(w)||w.player==Some(nation)) && w.has_flag(&format!("peg_{nation:?}"))=>w.nation_opt(nation).map(|n|n.interest_rate),
        None=>None,
    }
}
pub fn establish_peg(w:&mut WorldState,nation:NationId,rate:f64) {
    if clock::is_daily(w)||w.player==Some(nation) {
        w.agency.monetary.insert(nation,MonetaryRegime::Pegged{rate});
    }
}
pub fn break_peg(w:&mut WorldState,nation:NationId)->Result<(),String> {
    if pegged_rate(w,nation).is_none() {return Err("The currency is already floating.".into());}
    w.agency.monetary.insert(nation,MonetaryRegime::Floating);
    let n=w.nation_mut(nation);
    n.stability=(n.stability-BREAK_PEG_STABILITY).max(0.0);
    n.inflation=(n.inflation+BREAK_PEG_INFLATION).min(3.0);
    if w.player==Some(nation) {w.player_set_rate=false;}
    w.headline(format!("{} exits its currency peg: inflation rises two points, stability falls five, and the central bank resumes automatic policy.",nation.name()));
    Ok(())
}
pub fn resume_bank(w:&mut WorldState,nation:NationId)->Result<(),String> {
    player(w,nation)?;
    if pegged_rate(w,nation).is_some() {return Err("Exit the currency peg before returning to a floating automatic rate.".into());}
    w.player_set_rate=false;
    Ok(())
}

fn title(o:&Offer)->&'static str {
    match o.kind {OfferKind::DefensePact=>"a mutual-defense pact",OfferKind::TradeTreaty=>"a trade treaty",
        OfferKind::CallToArms{guaranteed:true,..}=>"a call to honor its defense pact",
        OfferKind::CallToArms{..}=>"an invitation to intervene"}
}
#[derive(Serialize)]
pub struct OfferView { pub id:u64,pub from:NationId,pub from_name:String,pub title:String,
    pub expires:String,pub days_remaining:i32,pub consequence:String,pub accept_blocked:Option<String> }
#[derive(Serialize)]
pub struct View {pub policy:StandingPolicy,pub offers:Vec<OfferView>,pub history:Vec<Decision>,
    pub monetary:MonetaryRegime,pub automatic_bank:bool,pub break_peg_pc:f64,pub expiry_rule:&'static str}
pub fn view(w:&WorldState,nation:NationId)->View {
    let offers=w.agency.offers.iter().filter(|o|o.to==nation).map(|o|{
        let (y,m,d)=clock::date_from_day(o.expires_day);
        let consequence=match o.kind {
            OfferKind::DefensePact=>"Both signatories pay 0.3% of GDP annually. A later call to arms needs your response; abandoning a guarantee has diplomatic costs.".into(),
            OfferKind::TradeTreaty=>"Opens market integration and long-term dependency. Ending the treaty later can damage both economies.".into(),
            OfferKind::CallToArms{rung,guaranteed:true,..}=>format!("Accept: enter the defending side at rung {rung}, subject to normal access, nuclear and sphere rules. Decline or expire: lose 25 reputation and 45 relations and end the pact."),
            OfferKind::CallToArms{rung,..}=>format!("Accept: enter the defending side at rung {rung}, subject to normal access, nuclear and sphere rules. Declining carries no treaty penalty."),
        };
        OfferView{id:o.id,from:o.from,from_name:o.from.name().into(),title:title(o).into(),expires:format!("{y:04}-{m:02}-{d:02}"),
            days_remaining:(o.expires_day-clock::absolute_day(w)).max(0),consequence,
            accept_blocked:response_error(w,nation,o.id,true)}
    }).collect();
    View{policy:policy(w,nation),offers,history:w.agency.history.iter().filter(|h|h.offer.to==nation).cloned().collect(),
        monetary:pegged_rate(w,nation).map_or(MonetaryRegime::Floating,|rate|MonetaryRegime::Pegged{rate}),
        automatic_bank:w.player==Some(nation)&&!w.player_set_rate&&pegged_rate(w,nation).is_none(),
        break_peg_pc:BREAK_PEG_PC,expiry_rule:"New requests follow the visible standing policy. Review is the default. Unanswered requests decline after their stated deadline; existing requests are not automatically answered by a policy change."}
}
