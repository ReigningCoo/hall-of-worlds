//! Mobs' choices (mob behaviour plan PR 2): one scorer in place of separate
//! rules. Each creature a mob sees gives it options (attack, come over, guard
//! against, keep away), scored by its race's drives (`shared::behaviour`);
//! morale adds fleeing from enemies when hurt or outnumbered; the best wins,
//! but only by a margin over what it is doing already. Every mob's choice is
//! worked out from where everyone stood at the start of the tick, before any
//! of them moves.

mod choose;
mod fire;
mod morale;
mod option;
mod score;
mod seen;
mod ways;
mod zigzag;

pub(super) use choose::{after_harm, choose};
pub(super) use fire::{fires_by_place, near_fire, with_fire};
pub(crate) use option::{Choice, Scored, Scores};
pub(super) use seen::{Other, friends_seen, others_of};
#[cfg(test)]
pub(super) use seen::{Wants, wants};
pub(crate) use ways::Ways;
pub(super) use zigzag::zigzag;

#[cfg(test)]
mod tests;
