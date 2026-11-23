//! Spells never kill the caster or a friend (magic plan, Effects): harm from
//! a spell stops at 1 Health for them. Players can't kill each other with
//! spells while there is no PvP.

use crate::vitals::Points;

use super::hit::Side;

/// The harm a spell may do to someone on `side` with `health` left: all of
/// it to enemies and the neutral, never more than leaves 1 Health to the
/// caster and friends.
pub fn spared_harm(harm: Points, health: Points, side: Side) -> Points {
    match side {
        Side::Caster | Side::Friend => {
            let most = health.milli().saturating_sub(Points::whole(1).milli());
            Points(harm.milli().min(most))
        }
        Side::Enemy | Side::Neutral => harm,
    }
}
