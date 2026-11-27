//! Walking into reach of a creature before acting on it (an Attack, a Kick, a
//! spell chosen by hand): routes towards where they stand, routing again as
//! they move, and stops as soon as they are within reach (owner, 2026-11-27).

use bevy::prelude::*;
use shared::combat::{Body, distance, within};
use shared::dungeon::Level;
use shared::id::Id;
use shared::spot::Spot;

use super::{Player, TakeStep, Travel, TravelTo, Walking};
use crate::common::{CurrentDungeon, DrawnGround, GameView, Other, PlayerSize, SpotPosition};

/// How far the one approached may move from where the route was found to
/// before it is found again, in units: half a cell.
const REROUTE: u64 = 120;

/// Walk towards `target` until within `reach` of them: units between the two
/// middles in 3D, with a clear line (`combat::within`), as a blow or a spell
/// is measured. Replaces any approach under way. Ends with `Approached`.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Approach {
    pub target: Id,
    pub reach: u64,
}

/// How an approach ended: `reached` once within reach; otherwise given up (the
/// target gone from sight or the level, no way nearer) or called off (keys,
/// the pad, a click elsewhere, `StopApproach`).
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Approached {
    pub target: Id,
    pub reached: bool,
}

/// Calls off any approach under way where the player stands (Esc).
#[derive(Message, Clone, Copy, Debug, Default)]
pub struct StopApproach;

/// An approach under way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Approaching {
    target: Id,
    reach: u64,
    /// Where the target stood when the route was last found.
    routed_to: Option<Spot>,
    /// How far apart the two were then: a route that ends no nearer gives up.
    apart: u64,
}

/// What an approach does next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Next {
    /// In reach: stop here.
    Reached,
    /// The target is gone, or the last route came no nearer.
    GiveUp,
    /// Find a route to where the target stands now.
    Route(Spot),
    /// Keep walking the route.
    Walk,
}

/// What to do with `going`, the player standing as `me` and the target as
/// `them` (`None`: no longer seen), with a route still being walked or not.
pub(super) fn next(
    level: &Level,
    going: &Approaching,
    me: &Body,
    them: Option<&Body>,
    route_left: bool,
) -> Next {
    let Some(them) = them else {
        return Next::GiveUp;
    };
    if within(level, me, them, going.reach) {
        return Next::Reached;
    }
    match going.routed_to {
        None => Next::Route(them.at),
        Some(to) if route_left => match to.distance(them.at) > REROUTE {
            true => Next::Route(them.at),
            false => Next::Walk,
        },
        // The route has ended out of reach: again only if it came nearer.
        Some(_) if distance(me, them) < going.apart => Next::Route(them.at),
        Some(_) => Next::GiveUp,
    }
}

/// The player and their journey.
type Walker<'w, 's> = Single<'w, 's, (&'static SpotPosition, &'static mut Travel), With<Player>>;

/// Keeps an approach going (in `Travel`, before `travel_on_level`): routes to
/// the target as `travel_on_level` routes journeys (without `Arrived` or a
/// Teleport), stops the journey once in reach, and reports how it ended.
/// Called off by anything else moving the player, or `StopApproach`.
#[allow(clippy::too_many_arguments)]
pub(super) fn approach(
    view: Res<State<GameView>>,
    (mut asked, mut stops): (MessageReader<Approach>, MessageReader<StopApproach>),
    (mut journeys, mut walking): (MessageReader<TravelTo>, MessageReader<Walking>),
    mut steps: MessageReader<TakeStep>,
    mut approached: MessageWriter<Approached>,
    (dungeon, drawn, size): (Res<CurrentDungeon>, Res<DrawnGround>, Res<PlayerSize>),
    others: Query<(&Other, &SpotPosition), Without<Player>>,
    player: Walker,
) {
    let (spot, mut travel) = player.into_inner();
    let mut end = |travel: &mut Travel, reached: bool| {
        if let Some(going) = travel.approach.take() {
            approached.write(Approached {
                target: going.target,
                reached,
            });
        }
    };
    if let Some(&Approach { target, reach }) = asked.read().last() {
        end(&mut travel, false);
        travel.approach = Some(Approaching {
            target,
            reach,
            routed_to: None,
            apart: u64::MAX,
        });
    }
    // Whatever moves the player now has its way.
    let moved = journeys.read().count() + walking.read().count() + steps.read().count() > 0;
    let stopped = stops.read().count() > 0;
    if moved || stopped || *view.get() != GameView::Dungeon {
        if stopped && travel.approach.is_some() {
            stop(&mut travel);
        }
        end(&mut travel, false);
        return;
    }
    let Some(mut going) = travel.approach else {
        return;
    };
    let relief = drawn.relief.as_ref();
    let me = Body::on(relief, spot.0, size.0);
    let them = (others.iter())
        .find(|(other, _)| other.id == going.target)
        .map(|(other, at)| Body::on(relief, at.0, other.kind.def().size));
    let route_left = travel.asked || !travel.spots.is_empty() || travel.landing.is_some();
    match next(&dungeon.level, &going, &me, them.as_ref(), route_left) {
        Next::Reached => {
            stop(&mut travel);
            end(&mut travel, true);
        }
        Next::GiveUp => {
            stop(&mut travel);
            end(&mut travel, false);
        }
        Next::Route(to) => {
            going.routed_to = Some(to);
            going.apart = them.map_or(u64::MAX, |them| distance(&me, &them));
            travel.goal = to.cell();
            travel.asked = true;
            travel.rerouted = false;
            travel.approach = Some(going);
        }
        Next::Walk => {}
    }
}

/// Ends the journey where the player is: the stretch walked is sent by
/// `travel_on_level`, and they step out of anyone squeezed past.
fn stop(travel: &mut Travel) {
    travel.spots.clear();
    travel.asked = false;
}

#[cfg(test)]
mod tests;
