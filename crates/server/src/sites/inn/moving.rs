//! Into and out of The Way Inn's instances. There is one inn for every world:
//! its instances (`SiteId::InnInstance`) are kept by the home world, so while
//! someone is in one their client counts as in the home world
//! (`Client::world`), and `InnState::from` keeps the world they came into
//! the Hall from, where they are saved and go back to.

use shared::door::Swing;
use shared::inn::{self, spots};
use shared::protocol::ServerMessage;
use shared::site::Exit;
use shared::spot::Spot;
use std::collections::{BTreeMap, BTreeSet};

use super::{InnMove, inn_poi};
use crate::connections::Client;
use crate::sites::doors::{DoorKey, keep_door};
use crate::sites::{Arrival, enter_site, leave, resume};
use crate::storage::{CharacterWorldSave, SiteId, VisitSave};
use crate::{ServerWorld, ServerWorlds, hall as server_hall};

/// The most players in one instance of the inn (owner).
pub(crate) const MOST_IN_ONE: usize = 30;

/// Does client `i`'s `InnState::moving`, if it asked for one. `false` if the
/// client has gone.
pub(crate) fn move_inn(clients: &mut [Client], i: usize, worlds: &mut ServerWorlds) -> bool {
    let Some(going) = clients[i].inn.moving.take() else {
        return true;
    };
    match going {
        InnMove::In { back, at } => {
            let (instance, fresh) = instance_for(clients, i);
            go_in(&mut clients[i], worlds, instance, back, at, fresh)
        }
        InnMove::Out => out_of_inn(&mut clients[i], worlds),
    }
}

/// The instance someone is in, if any.
pub(super) fn instance_of(client: &Client) -> Option<u32> {
    match client.visit.as_ref()?.id {
        SiteId::InnInstance(number) => Some(number),
        _ => None,
    }
}

/// The instance client `i` goes into: the one they are in already, else
/// `choose`'s from how many everyone else's has; and whether it is new.
fn instance_for(clients: &[Client], i: usize) -> (u32, bool) {
    if let Some(number) = instance_of(&clients[i]) {
        return (number, false);
    }
    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    for (j, other) in clients.iter().enumerate() {
        if j != i
            && !other.is_gone()
            && let Some(number) = instance_of(other)
        {
            *counts.entry(number).or_default() += 1;
        }
    }
    let number = choose(&counts);
    (number, !counts.contains_key(&number))
}

/// Of the instances in use, by how many are in each: the fullest with room
/// (the lowest number of those as full), else a new one numbered the lowest
/// not in use.
pub(super) fn choose(counts: &BTreeMap<u32, usize>) -> u32 {
    let with_room = counts.iter().filter(|&(_, &count)| count < MOST_IN_ONE);
    let fullest = with_room.max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)));
    match fullest {
        Some((&number, _)) => number,
        None => (1..)
            .find(|number| !counts.contains_key(number))
            .unwrap_or(1),
    }
}

/// Into instance `number` of the inn, kept by the home world: inside its
/// entrance, or onto `at`; to go back to `back` in the Hall on leaving. A
/// `fresh` one starts as made, whatever an instance of that number left.
fn go_in(
    client: &mut Client,
    worlds: &mut ServerWorlds,
    number: u32,
    back: Spot,
    at: Option<(u32, Spot)>,
    fresh: bool,
) -> bool {
    let id = SiteId::InnInstance(number);
    leave(client, worlds.of(client.world));
    client.visit = None;
    client.character.inn_return = Some(back);
    client.unsaved = true;
    client.inn.from = client.inn.from.or(client.world);
    client.world = Some(worlds.home_id());
    let world = worlds.home_mut();
    if fresh {
        world.close_inn(number);
    }
    carry_knowledge(&mut client.character, id);
    let poi = inn_poi(client);
    match at {
        Some((level, position)) => {
            let save = VisitSave {
                site: id,
                level,
                position,
                from_wilderness: false,
            };
            resume(client, world, save)
        }
        None => {
            close_entrance(world, id);
            let arrival = Arrival::From(Exit::WorldMap);
            enter_site(client, world, poi, id, inn::GROUND, arrival, false)
        }
    }
}

/// What they remember of the inn's layout goes with them into another
/// instance; what they remember of its doors and bags doesn't.
fn carry_knowledge(character: &mut CharacterWorldSave, id: SiteId) {
    let others: Vec<SiteId> = (character.memory.keys().copied())
        .filter(|other| other.is_inn() && *other != id)
        .collect();
    for other in others {
        if let Some(memory) = character.memory.remove(&other) {
            character.memory.entry(id).or_insert(memory);
        }
        character.site_versions.remove(&other);
        character.left_at.remove(&other);
    }
    character.bags.retain(|other, _| !other.is_inn());
    character.doors.retain(|other, _| !other.is_inn());
}

/// The entrance door shut behind whoever comes in, so walking on doesn't take
/// them straight back out. Nobody stands in it: stepping onto it leaves.
fn close_entrance(world: &mut ServerWorld, id: SiteId) {
    if world.open_site(id).is_err() {
        return;
    }
    let ground = world.site(id).levels.get(&inn::GROUND);
    let door = ground.and_then(|level| level.doors.get(&spots::ENTRANCE).copied());
    if let Some(mut door) = door.filter(|door| door.swing != Swing::Closed) {
        door.swing = Swing::Closed;
        let at = spots::ENTRANCE;
        let key = DoorKey {
            site: id,
            index: inn::GROUND,
            at,
        };
        keep_door(world, key, door);
    }
}

/// Out of the inn, back to the walkway tile they came in from (or, with none
/// kept, beside the Hall's first gate), in the world they came from.
pub(crate) fn out_of_inn(client: &mut Client, worlds: &mut ServerWorlds) -> bool {
    leave(client, worlds.of(client.world));
    client.visit = None;
    if let Some(from) = client.inn.from.take() {
        client.world = Some(from);
    }
    let world = worlds.of(client.world);
    let back = client.character.inn_return.take();
    let back = back.unwrap_or_else(|| Spot::middle_of(super::first_way()));
    client.unsaved = true;
    // The home world's clock may not be theirs.
    client.link.send(&ServerMessage::WorldClock(world.time()))
        && server_hall::enter(client, world, back)
}

/// Closes the instances nobody is in any more: what was left in them goes.
pub(crate) fn close_empty_inns(clients: &[Client], worlds: &mut ServerWorlds) {
    let in_use: BTreeSet<u32> = (clients.iter())
        .filter(|client| !client.is_gone())
        .filter_map(instance_of)
        .collect();
    let world = worlds.home_mut();
    for number in world.open_inns() {
        if !in_use.contains(&number) {
            world.close_inn(number);
        }
    }
}

#[cfg(test)]
mod tests;
