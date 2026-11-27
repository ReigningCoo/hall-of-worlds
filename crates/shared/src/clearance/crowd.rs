//! Creatures in each other's way (owner, 2026-11-10): players pass through
//! players, creatures that aren't enemies can be squeezed past slowly ("excuse
//! me"), and enemies block.

use super::blocker::point_to_segment;
use super::{MOST_SQUEEZE_PER_MILLE, Passage, Shape, check_segment, squeeze_per_mille};
use crate::creature::Size;
use crate::dungeon::Level;
use crate::spot::Spot;

/// What another creature is to the one moving.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crowding {
    /// Walked through freely: another player, to a player.
    Pass,
    /// Squeezed past, more slowly the closer: anyone who isn't an enemy.
    Squeeze,
    /// In the way: an enemy.
    Block,
}

/// Another creature on the level, as far as a move goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Other {
    pub at: Spot,
    pub size: Size,
    pub crowding: Crowding,
}

/// How much slower a creature of `size` moves from `from` to `to` past
/// `others`, in thousandths (1000: not at all), or `None` if an enemy is in
/// the way.
///
/// Circles overlapping are what counts. Squeezing past someone goes by the
/// room left round the mover's middle, as for a gap (`squeeze_per_mille`),
/// but never stops it: at worst it is `MOST_SQUEEZE_PER_MILLE`; a move away
/// from someone already overlapped (its nearest point to them is its start)
/// costs no squeeze from them. An enemy's circle can't be entered; one
/// already overlapping (arriving by stairs, a shove) may only be moved away
/// from.
pub fn crowd_per_mille(size: Size, from: Spot, to: Spot, others: &[Other]) -> Option<u32> {
    let radius = size.radius();
    let mut slowest = 1000;
    for other in others {
        if other.crowding == Crowding::Pass {
            continue;
        }
        let body = Shape::Post {
            at: other.at,
            radius: other.size.radius(),
        };
        let passing = body.clearance(from, to);
        if passing >= radius {
            continue;
        }
        // Moving away from someone already overlapped: the nearest the move
        // comes to them is where it starts.
        let away = point_to_segment(other.at, from, to) >= other.at.distance_squared(from);
        match other.crowding {
            Crowding::Pass => {}
            // Away from them costs nothing: stepping out isn't slowed.
            Crowding::Squeeze if away => {}
            Crowding::Squeeze => {
                let squeeze = squeeze_per_mille(radius, passing).unwrap_or(MOST_SQUEEZE_PER_MILLE);
                slowest = slowest.max(squeeze);
            }
            // Only away from where it already overlaps.
            Crowding::Block if away => {}
            Crowding::Block => return None,
        }
    }
    Some(slowest)
}

/// `check_segment` with other creatures in the way too: the slower of the two
/// squeezes, or `None` if either refuses.
pub fn check_among(
    level: &Level,
    size: Size,
    from: Spot,
    to: Spot,
    others: &[Other],
) -> Option<Passage> {
    let passage = check_segment(level, size, from, to)?;
    let crowd = crowd_per_mille(size, from, to, others)?;
    Some(Passage {
        squeeze_per_mille: passage.squeeze_per_mille.max(crowd),
        ..passage
    })
}

/// Whether a creature of `size` standing at `at` overlaps anyone in `others`
/// it can't pass through freely: for picking where to stand, not for moving.
pub fn crowded_at(size: Size, at: Spot, others: &[Other]) -> bool {
    others.iter().any(|other| {
        other.crowding != Crowding::Pass
            && at.distance(other.at) < (size.radius() + other.size.radius()) as u64
    })
}
