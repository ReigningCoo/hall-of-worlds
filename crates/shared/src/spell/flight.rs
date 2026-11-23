//! A missile in flight (magic plan PR 6): its way, worked out once as it is
//! launched (`launch`: to the spot aimed at, stopped by walls), then flown a
//! stretch at a time (`Path::meets`), so whoever steps into its way while it
//! flies is hit and whoever steps out of it is missed. The server flies
//! missiles by this; `missile` is the same flight all at once.

use serde::{Deserialize, Serialize};

use crate::dungeon::Level;
use crate::facing::Heading;
use crate::spot::Spot;

use super::aim::aim_first;
use super::design::WhereStep;
use super::filters::Filters;
use super::hit::{Creature, Hit, Origin};
use super::hits::later_filters;
use super::line::{ahead, entry_along, wall_stop};
use super::missile::{MISSILE_RANGE, flight_millis, missile_speed};
use super::place::Where;
use super::toward::Toward;

/// A missile's way: from where it was launched to where it stops if it meets
/// nobody (a wall, the spot aimed at or the end of its range).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Path {
    pub from: Spot,
    pub to: Spot,
    /// The ground's height where it was launched, in units.
    pub height: i32,
    pub heading: Heading,
    /// Units from `from` to `to`.
    pub length: u64,
    /// The missile's count: faster for each (`missile_speed`).
    pub count: u8,
    /// Whom it may hit (`Side::allowed_by`): it passes through the rest.
    pub filters: Filters,
}

/// A missile from `origin` at `toward`, by a where step's count and filters:
/// to the spot aimed at (no further than `MISSILE_RANGE`, along the same way
/// if that is further), or straight ahead that far, stopped by the first wall.
/// A first step's filters choose its aim (`aim_first`); `None` when Friend or
/// Enemy find nobody of theirs in reach (a fizzle).
pub fn launch(
    level: &Level,
    step: &WhereStep,
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
) -> Option<Path> {
    let (origin, toward) = aim_first(level, step, origin, toward, creatures)?;
    Some(path(
        level,
        &origin,
        toward,
        step.count,
        step.filters,
        creatures,
    ))
}

/// A missile of `step`'s merged where at `index` (1 or more), from
/// `origin`, a hit of the where before it, going on ahead; filtered as
/// `part_hits` is, never aimed. `None` when that where isn't a missile.
pub fn launch_part(
    level: &Level,
    step: &WhereStep,
    index: usize,
    origin: &Origin,
    creatures: &[Creature],
) -> Option<Path> {
    let part = step.parts().nth(index).filter(|_| index > 0)?;
    (part.place == Where::Missile).then(|| {
        let filters = later_filters(step.filters);
        path(level, origin, Toward::Ahead, part.count, filters, creatures)
    })
}

/// `launch` without the aim: the way itself.
pub(super) fn path(
    level: &Level,
    origin: &Origin,
    toward: Toward,
    count: u8,
    filters: Filters,
    creatures: &[Creature],
) -> Path {
    let heading = toward.heading(origin, creatures);
    let end = match toward.spot(creatures) {
        Some(spot) if origin.at.distance(spot) <= MISSILE_RANGE => spot,
        _ => ahead(origin.at, heading, MISSILE_RANGE),
    };
    let to = wall_stop(level, origin.at, end);
    Path {
        from: origin.at,
        to,
        height: origin.height,
        heading,
        length: origin.at.distance(to),
        count,
        filters,
    }
}

impl Path {
    /// How fast it flies, in units a second.
    pub fn speed(&self) -> u64 {
        missile_speed(self.count)
    }

    /// How long the whole way takes, in milliseconds.
    pub fn millis(&self) -> u64 {
        flight_millis(self.length, self.count)
    }

    /// How far along it is `millis` after launching, in units, at most its
    /// length.
    pub fn along_after(&self, millis: u64) -> u64 {
        (millis.saturating_mul(self.speed()) / 1000).min(self.length)
    }

    /// The spot `along` units from `from`.
    pub fn at(&self, along: u64) -> Spot {
        if self.length == 0 {
            return self.from;
        }
        let (along, length) = (along.min(self.length) as i128, self.length as i128);
        let part = |d: i64| (i128::from(d) * along / length) as i64;
        self.from
            .offset(part(self.to.x - self.from.x), part(self.to.y - self.from.y))
    }

    /// The first creature its filters allow (never `except`, the one it was
    /// launched from) whose circle it meets flying from `start` to `end`
    /// units along: how far along, and its place in `creatures`.
    pub fn meets(
        &self,
        (start, end): (u64, u64),
        creatures: &[Creature],
        except: Option<usize>,
    ) -> Option<(u64, usize)> {
        let (from, to) = (self.at(start), self.at(end));
        creatures
            .iter()
            .enumerate()
            .filter(|(index, other)| Some(*index) != except && other.side.allowed_by(self.filters))
            .filter_map(|(index, other)| {
                let radius = other.body.size.radius().unsigned_abs();
                entry_along(from, to, other.body.at, radius).map(|along| (start + along, index))
            })
            .min()
    }

    /// Landing on `creatures[index]`, facing away from where it came from.
    pub fn hit(&self, creatures: &[Creature], index: usize) -> Hit {
        Hit::on(creatures, index, self.from, self.heading)
    }

    /// Landing at the end of its way, having met nobody.
    pub fn end(&self) -> Hit {
        Hit {
            creature: None,
            at: self.to,
            height: self.height,
            heading: self.heading,
        }
    }
}
