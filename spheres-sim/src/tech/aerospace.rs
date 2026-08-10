//! Aerospace & Military.
//!
//! OWNER: the aerospace domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: combat aircraft and their engines, stealth, precision munitions,
//! ballistic and cruise missiles and the defences against them, naval and
//! submarine systems, armour, sensors and electronic warfare, uncrewed systems,
//! military space, nuclear weapons engineering and its verification.
//!
//! Every id here must begin with `aero_`. Prerequisites may name other `aero_`
//! ids in this file, or any `core_` id from the foundation set in `tech/mod.rs`.
//! Nothing else — you cannot see the other domains and they cannot see you.
//!
//! Every entry is a real technology with a real history, and carries a comment
//! naming the first deployment its year floor is read off. Anything past the
//! present day is speculative and must say so in the comment.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

pub fn techs() -> Vec<TechDef> {
    vec![]
}
