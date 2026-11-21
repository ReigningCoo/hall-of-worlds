//! Make Campfire on a visit's level: the fire faced to feed, else the cell
//! faced to light a new one on (`shared::campfire`), and doing it once the
//! wind-up lands, checked again then.

use shared::campfire::{Campfire, campfire_faced, can_hold_fire, place_faced};
use shared::facing::Heading;
use shared::material::Material;

use super::{fire_now, keep_campfire, saved_campfire};
use crate::ServerWorld;
use crate::sites::Visit;

/// What Make Campfire would do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FireTarget {
    /// Feed the fire burning at this cell.
    Feed((i64, i64)),
    /// Light a new fire at this cell.
    Light((i64, i64)),
}

impl FireTarget {
    /// The cell it is at.
    pub(crate) fn cell(self) -> (i64, i64) {
        match self {
            Self::Feed(cell) | Self::Light(cell) => cell,
        }
    }
}

/// Whether a new fire could go on `(x, y)` of the visit's level: ground that
/// holds one, no stairs and no fire there.
fn free_for_fire(visit: &Visit, (x, y): (i64, i64)) -> bool {
    let stairs = visit.level.stairs.iter().any(|s| (s.x, s.y) == (x, y));
    !stairs && visit.cells().campfire_at(x, y).is_none()
}

/// What Make Campfire does for someone on this visit facing `heading`: feed
/// the fire faced, else light one on the ground faced; `None` if neither.
pub(crate) fn fire_target(visit: &Visit, heading: Heading) -> Option<FireTarget> {
    let level = visit.cells();
    let burning = |x: i64, y: i64| level.campfire_at(x, y).is_some();
    if let Some(cell) = campfire_faced(level, visit.spot(), heading, burning) {
        return Some(FireTarget::Feed(cell));
    }
    let free = |x: i64, y: i64| free_for_fire(visit, (x, y));
    place_faced(level, visit.spot(), heading, free).map(FireTarget::Light)
}

/// Puts a piece of `wood`'s kindling on the fire at `target`, or lights one
/// there with it: the pieces it has left after, or `None` if it can't be done
/// now (the fire went out or is full, or the ground was taken).
pub(crate) fn light_or_feed(
    world: &mut ServerWorld,
    visit: &Visit,
    target: FireTarget,
    wood: Material,
) -> Option<usize> {
    let now = fire_now(world);
    let (x, y) = target.cell();
    let key = visit.plant_key(x, y);
    let kept = saved_campfire(world, key).filter(|fire| fire.is_lit(now));
    let fire = match (target, kept) {
        (FireTarget::Feed(_), Some(mut fire)) => {
            fire.feed(now, wood).ok()?;
            fire
        }
        (FireTarget::Light(_), None)
            if can_hold_fire(visit.cells(), x, y) && free_for_fire(visit, (x, y)) =>
        {
            Campfire::lit(now, wood)
        }
        _ => return None,
    };
    let pieces = fire.pieces_left(now);
    keep_campfire(world, key, Some(fire));
    Some(pieces)
}
