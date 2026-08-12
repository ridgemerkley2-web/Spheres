//! The 1990 start, assembled from data.
//!
//! This file used to *be* the roster: twenty-four calls to an eighteen-argument
//! positional constructor, plus a hundred relation tuples. Every figure in it
//! is now a named field in `spheres-sim/data/`, with the 1990 sourcing note
//! travelling beside the numbers it justifies rather than as a Rust comment
//! nothing could read. See `crate::data` for the schema, the two passes of
//! validation, and the argument for embedding the canonical set.
//!
//! Nothing here is a fallback. If the content is broken this panics with every
//! error in it at once, because a world built from half-valid data would be a
//! silently wrong timeline, and the whole project rests on the timeline being
//! reproducible.

use crate::data;
use crate::world::{GameRules, WorldState};

/// Transcribed-not-invented: approximate 1990 historical starting conditions.
pub fn world_1990(rules: GameRules) -> WorldState {
    data::load_world(data::EMBEDDED_NATIONS, &data::EMBEDDED_RELATIONS, rules)
        .unwrap_or_else(|e| panic!("{}", data::render_errors(&e)))
}
