//! The Way Inn (`shared::inn`) as a site: into it by walking off the Hall's
//! edge at a hidden way-in, arriving inside its entrance door whichever side
//! they came from; out by stepping onto that door, back to the walkway tile
//! they stepped off from (`CharacterWorldSave::inn_return`).
//!
//! One inn for every world, in instances (`SiteId::InnInstance`) of at most
//! `MOST_IN_ONE` players kept by the home world (`moving.rs`); no fighting
//! anywhere in it (`peace.rs`); things left lying in its public rooms cleared
//! after a while (`litter.rs`); players' own rooms upstairs each their own
//! copy (`own_room.rs`). Going in and out are asked for here and done
//! once a client's messages have been read (`move_inn`), as they need every
//! world and everyone else in the inn.

use bevy::prelude::*;
use shared::hall::{self, GatePlace, INN_WAY_OFFSET};
use shared::inn;
use shared::poi::Poi;
use shared::protocol::ClientMessage;
use shared::spot::Spot;
use shared::world::WorldId;

use super::enter::dev_poi;
use super::{NewVisit, place_name_of};
use crate::connections::Client;
use crate::storage::VisitSave;
use crate::{ServerWorld, hall as server_hall};
use shared::protocol::LevelKind;

mod litter;
mod moving;
mod own_room;
mod peace;

pub(crate) use litter::{Litter, clear_litter};
pub(crate) use moving::{close_empty_inns, move_inn, out_of_inn};
pub(crate) use own_room::{close_own_room, hidden_in_own_room, keep_own_room};
pub(crate) use peace::refuses_harm;

/// Where a client is with the inn.
#[derive(Debug, Default)]
pub(crate) struct InnState {
    /// Into or out of it, done once its messages this tick have been read, as
    /// that needs every world and everyone else in the inn (`move_inn`).
    pub(crate) moving: Option<InnMove>,
    /// While in it, kept by the home world (`Client::world` then): the world
    /// they came into the Hall from, where they are saved and go back to.
    pub(crate) from: Option<WorldId>,
    /// A dev tool asked for in it, answered once they are out in their own
    /// world (`answer_dev`).
    pub(crate) after: Option<ClientMessage>,
    /// What is in their own rooms upstairs (`own_room.rs`), as saved with
    /// the character: kept by the home world while they are in the inn.
    pub(crate) room: crate::storage::LevelSave,
}

/// Going into or out of the inn, done by `move_inn`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InnMove {
    /// Into an instance: inside its entrance, or (`at`, coming back to a
    /// save or a debug report's place) onto this floor and spot; leaving
    /// goes back to `back` in the Hall.
    In { back: Spot, at: Option<(u32, Spot)> },
    /// Out, back to the Hall where they came in from.
    Out,
}

/// The made-up poi standing for the inn, which is behind none: where the
/// player stands on the world map, of `inn::KIND`.
pub(crate) fn inn_poi(client: &Client) -> Poi {
    dev_poi(client, inn::KIND, 0)
}

/// What a level of `new` is told as, and its place's name: the inn's own
/// kind with none, else its poi's.
pub(super) fn kind_and_name(world: &ServerWorld, new: &NewVisit) -> (LevelKind, Option<String>) {
    if new.id.is_inn() {
        return (LevelKind::Inn, None);
    }
    (
        LevelKind::Poi(new.site.kind()),
        place_name_of(world, &new.poi),
    )
}

/// Walking off the Hall's edge (`ClientMessage::UseExit` there): into the inn if
/// they stand in front of a way in (`hall::inn_way_behind`), else refused.
pub(crate) fn enter_from_hall(client: &mut Client) -> bool {
    let Some(at) = client.character.hall.filter(|_| client.visit.is_none()) else {
        warn!("A client asked to go into the inn from outside the Hall");
        return true;
    };
    let (x, y) = at.cell();
    if hall::inn_way_behind(x, y).is_none() {
        info!("Refused going into the inn from the Hall at ({x}, {y})");
        return true;
    }
    let back = Spot::middle_of((x, y));
    client.inn.moving = Some(InnMove::In { back, at: None });
    true
}

/// Developer tools: into the inn from anywhere (onto `at`, a floor and spot,
/// if given). Leaving goes back in front of the nearest way in from where they
/// are in the Hall, or (anywhere else) beside the gate `DevToHall` would take
/// them to; already inside, where leaving was going to take them.
pub(crate) fn dev_to_inn(
    client: &mut Client,
    world: &mut ServerWorld,
    at: Option<(u32, Spot)>,
) -> bool {
    let in_inn = in_inn(client);
    let hall_spot = client.character.hall.filter(|_| client.visit.is_none());
    let back = match (in_inn, client.character.inn_return, hall_spot) {
        (true, Some(back), _) => back,
        (_, _, Some(at)) => {
            let (x, y) = at.cell();
            let way = x - (x - INN_WAY_OFFSET).rem_euclid(hall::GATE_SPACING);
            let side = if y < hall::WALKWAY_ROWS / 2 {
                hall::Side::North
            } else {
                hall::Side::South
            };
            Spot::middle_of(hall::inn_way_front(way, side))
        }
        _ => {
            let place = server_hall::gate_for(client, world, None, None);
            Spot::middle_of(way_beside(place))
        }
    };
    client.inn.moving = Some(InnMove::In { back, at });
    true
}

/// Coming back to a save in the inn (`sites::resume`): into an instance, on
/// that floor and spot.
pub(super) fn back_into_inn(client: &mut Client, saved: VisitSave) -> bool {
    let back = (client.character.inn_return).unwrap_or_else(|| Spot::middle_of(first_way()));
    let at = Some((saved.level, saved.position));
    client.inn.moving = Some(InnMove::In { back, at });
    true
}

/// The front of the way in just past the Hall's first gate: where leaving
/// goes with nowhere kept to go back to.
fn first_way() -> (i64, i64) {
    way_beside(GatePlace::from_number(0))
}

/// The front of the way in just past a gate, on its side.
fn way_beside(place: GatePlace) -> (i64, i64) {
    hall::inn_way_front(place.x() + INN_WAY_OFFSET, place.side)
}

/// Whether they are in the inn.
pub(crate) fn in_inn(client: &Client) -> bool {
    client.visit.as_ref().is_some_and(|visit| visit.id.is_inn())
}

/// After a move in a site: on the inn's entrance door, out of it.
pub(crate) fn follow_inn(client: &mut Client) -> bool {
    let out = client
        .visit
        .as_ref()
        .is_some_and(|visit| visit.id.is_inn() && inn::is_way_out(visit.index, visit.position()));
    if out {
        return leave_inn(client);
    }
    true
}

/// Out of the inn, back to the walkway tile they came in from (`move_inn`).
pub(crate) fn leave_inn(client: &mut Client) -> bool {
    client.inn.moving = Some(InnMove::Out);
    true
}

#[cfg(test)]
mod tests;
