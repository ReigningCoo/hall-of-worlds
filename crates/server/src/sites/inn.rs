//! The Way Inn (`shared::inn`) as a site (`SiteId::Inn`): into it by walking
//! off the Hall's edge at a hidden way-in, arriving inside its entrance door
//! whichever side they came from; out by stepping onto that door, back to the
//! walkway tile they stepped off from (`CharacterWorldSave::inn_return`).

use bevy::prelude::*;
use shared::door::Swing;
use shared::hall::{self, GatePlace, INN_WAY_OFFSET};
use shared::inn::{self, spots};
use shared::poi::Poi;
use shared::site::Exit;
use shared::spot::Spot;

use super::enter::dev_poi;
use super::{Arrival, NewVisit, enter_site, leave, place_name_of};
use crate::connections::Client;
use crate::storage::SiteId;
use crate::{ServerWorld, hall as server_hall};
use shared::protocol::LevelKind;

/// The made-up poi standing for the inn, which is behind none: where the
/// player stands on the world map, of `inn::KIND`.
pub(crate) fn inn_poi(client: &Client) -> Poi {
    dev_poi(client, inn::KIND, 0)
}

/// What a level of `new` is told as, and its place's name: the inn's own
/// kind with none, else its poi's.
pub(super) fn kind_and_name(world: &ServerWorld, new: &NewVisit) -> (LevelKind, Option<String>) {
    match new.id {
        SiteId::Inn => (LevelKind::Inn, None),
        _ => (
            LevelKind::Poi(new.site.kind()),
            place_name_of(world, &new.poi),
        ),
    }
}

/// Walking off the Hall's edge (`ClientMessage::UseExit` there): into the inn if
/// they stand in front of a way in (`hall::inn_way_behind`), else refused.
/// `false` if the client has gone.
pub(crate) fn enter_from_hall(client: &mut Client, world: &mut ServerWorld) -> bool {
    let Some(at) = client.character.hall.filter(|_| client.visit.is_none()) else {
        warn!("A client asked to go into the inn from outside the Hall");
        return true;
    };
    let (x, y) = at.cell();
    if hall::inn_way_behind(x, y).is_none() {
        info!("Refused going into the inn from the Hall at ({x}, {y})");
        return true;
    }
    enter(client, world, Spot::middle_of((x, y)))
}

/// Developer tools: into the inn from anywhere. Leaving goes back in front
/// of the nearest way in from where they are in the Hall, or (anywhere else)
/// beside the gate `DevToHall` would take them to; already inside, where
/// leaving was going to take them.
pub(crate) fn dev_to_inn(client: &mut Client, world: &mut ServerWorld) -> bool {
    let in_inn = client
        .visit
        .as_ref()
        .is_some_and(|visit| visit.id == SiteId::Inn);
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
    enter(client, world, back)
}

/// The front of the way in just past a gate, on its side.
fn way_beside(place: GatePlace) -> (i64, i64) {
    hall::inn_way_front(place.x() + INN_WAY_OFFSET, place.side)
}

/// Into the inn's ground floor inside its entrance, to go back to `back` (the
/// Hall's own units) on leaving.
fn enter(client: &mut Client, world: &mut ServerWorld, back: Spot) -> bool {
    leave(client, world);
    client.visit = None;
    client.character.inn_return = Some(back);
    client.unsaved = true;
    close_entrance(world);
    let poi = inn_poi(client);
    let arrival = Arrival::From(Exit::WorldMap);
    enter_site(client, world, poi, SiteId::Inn, inn::GROUND, arrival, false)
}

/// The entrance door shut behind whoever comes in, so walking on doesn't take
/// them straight back out. Nobody stands in it: stepping onto it leaves.
fn close_entrance(world: &mut ServerWorld) {
    let ground = world.site_mut(SiteId::Inn).levels.get_mut(&inn::GROUND);
    let door = ground.and_then(|level| level.doors.get_mut(&spots::ENTRANCE));
    if let Some(door) = door {
        door.swing = Swing::Closed;
    }
}

/// After a move in a site: on the inn's entrance door, out of it. `false` if
/// the client has gone.
pub(crate) fn follow_inn(client: &mut Client, world: &mut ServerWorld) -> bool {
    let out = client.visit.as_ref().is_some_and(|visit| {
        visit.id == SiteId::Inn && inn::is_way_out(visit.index, visit.position())
    });
    if out {
        return leave_inn(client, world);
    }
    true
}

/// Out of the inn, back to the walkway tile they came in from (or, with none
/// kept, beside the Hall's first gate). `false` if the client has gone.
pub(crate) fn leave_inn(client: &mut Client, world: &mut ServerWorld) -> bool {
    leave(client, world);
    client.visit = None;
    let back = client.character.inn_return.take();
    let back = back.unwrap_or_else(|| Spot::middle_of(way_beside(GatePlace::from_number(0))));
    server_hall::enter(client, world, back)
}

#[cfg(test)]
mod tests;
