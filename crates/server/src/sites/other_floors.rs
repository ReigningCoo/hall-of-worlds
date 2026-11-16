//! What a player sees of a building's other floors (`shared::floor_sight`): the
//! outside in vertical slices, and down through holes. Remembered like
//! anything else seen, and sent as it comes into view, as `RememberedLevel`s
//! the client adds to what it has. Mobs and players there stay unsent (later).

use shared::dungeon::{Level, Sight};
use shared::floor_sight;
use shared::protocol::{RememberedLevel, SeenStairs, ServerMessage, level_chunk};

use super::levels::level_of;
use super::remembered::mirror_of_first;
use super::{Mirror, NewVisit, Visit, VisitDoors, beside, door_looks, level_chunks};
use crate::ServerWorld;
use crate::connections::Client;
use crate::storage::CellMasks;

/// Another floor of the building the player is in, kept while they are on
/// theirs.
pub(crate) struct OtherFloor {
    index: u32,
    level: Level,
    stairs: Vec<SeenStairs>,
    doors: VisitDoors,
    /// Its cells the client has as seen: what the character remembered on
    /// arriving, and what they have seen since.
    sent: CellMasks,
    /// Its cells sent only to shape those (`beside`), not as seen.
    shaped: CellMasks,
    /// Its cells that are another map's too (the ground floor's, in the
    /// wilderness).
    mirrors: Vec<Mirror>,
}

/// The floors of a building other than level `index` (from the ground up, no
/// basement), for a visit to it; none for any other site, or a basement.
pub(super) fn floors_of(
    client: &Client,
    world: &mut ServerWorld,
    new: &NewVisit,
    index: u32,
) -> Vec<OtherFloor> {
    let site = &new.site;
    let storeys = site.levels() - u32::from(site.has_basement());
    if !site.is_building() || site.kind().is_town() || index >= storeys {
        return Vec::new();
    }
    let memory = client.character.memory.get(&new.id);
    (0..storeys)
        .filter(|&other| other != index)
        .map(|other| {
            let (level, stairs, doors) = level_of(world, new, other);
            let mirrors = mirror_of_first(world, new, other, &level);
            let sent = memory.and_then(|levels| levels.get(&other)).cloned();
            OtherFloor {
                index: other,
                level: level.level,
                stairs,
                doors,
                sent: sent.unwrap_or_default(),
                shaped: CellMasks::new(),
                mirrors,
            }
        })
        .collect()
}

/// Works out what the player sees of the other floors from where they are,
/// given what they see on theirs, remembers it and sends what is new.
/// `false` if the client has gone.
pub(super) fn see_other_floors(client: &mut Client, visit: &mut Visit, seen: &Sight) -> bool {
    if visit.floors.is_empty() {
        return true;
    }
    let mut storeys: Vec<&Level> = visit.floors.iter().map(|floor| &floor.level).collect();
    storeys.insert(visit.index as usize, &visit.level.level);
    let found = floor_sight::other_floors(&storeys, visit.index as usize, visit.position(), seen);
    let place = visit.id;
    for floor in &mut visit.floors {
        let sight = &found[floor.index as usize];
        let new = remember(client, place, floor, sight);
        if !new.is_empty() && !send(client, place, floor, new) {
            return false;
        }
    }
    true
}

/// Remembers what is seen of a floor (and of any map it is set into), and
/// gives the cells the client doesn't have as seen yet.
fn remember(
    client: &mut Client,
    place: crate::storage::SiteId,
    floor: &OtherFloor,
    sight: &Sight,
) -> CellMasks {
    let mut new = CellMasks::new();
    let mut learnt = false;
    let (min_x, min_y, width, height) = floor.level.bounds();
    let seen = (min_y..min_y + height)
        .flat_map(|y| (min_x..min_x + width).map(move |x| (x, y)))
        .filter(|&(x, y)| sight.is_seen(x, y));
    let memory = client.character.memory.entry(place).or_default();
    let memory = memory.entry(floor.index).or_default();
    let mut cells = Vec::new();
    for (x, y) in seen {
        let (chunk, bit) = level_chunk(x, y);
        let mask = memory.entry(chunk).or_default();
        learnt |= *mask & (1 << bit) == 0;
        *mask |= 1 << bit;
        if floor
            .sent
            .get(&chunk)
            .is_none_or(|sent| sent & (1 << bit) == 0)
        {
            *new.entry(chunk).or_default() |= 1 << bit;
        }
        cells.push((x, y));
    }
    for mirror in &floor.mirrors {
        let memory = client.character.memory.entry(mirror.id).or_default();
        let memory = memory.entry(mirror.index).or_default();
        for (x, y) in cells.iter().filter_map(|&(x, y)| mirror.of(x, y)) {
            let (chunk, bit) = level_chunk(x, y);
            *memory.entry(chunk).or_default() |= 1 << bit;
        }
    }
    client.unsaved |= learnt;
    new
}

/// Sends a floor's cells `new` as seen, and their neighbours that shape them.
fn send(
    client: &Client,
    place: crate::storage::SiteId,
    floor: &mut OtherFloor,
    new: CellMasks,
) -> bool {
    let level = &floor.level;
    let is_new = |x, y| {
        let (chunk, bit) = level_chunk(x, y);
        new.get(&chunk).is_some_and(|m| m & (1 << bit) != 0)
    };
    let mut done = floor.sent.clone();
    super::add_masks(&mut done, floor.shaped.clone());
    let neighbours = beside(level, is_new, &done);
    // As they are now: the player is looking at them.
    let looks = |x, y| {
        let cell = level.cell(x, y);
        door_looks(
            client,
            &floor.doors,
            (place, floor.index),
            cell,
            (x, y),
            (x, y),
            false,
        )
    };
    let truesight = client.truesight;
    let mut chunks = level_chunks(level, &floor.stairs, &new, true, truesight, &looks);
    let shaping = level_chunks(level, &floor.stairs, &neighbours, false, truesight, &looks);
    chunks.extend(shaping);
    let message = ServerMessage::RememberedLevel(RememberedLevel {
        index: floor.index,
        bounds: level.bounds(),
        chunks,
    });
    super::add_masks(&mut floor.sent, new);
    super::add_masks(&mut floor.shaped, neighbours);
    client.link.send(&message)
}
