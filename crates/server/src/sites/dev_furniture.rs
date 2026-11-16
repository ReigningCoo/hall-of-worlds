//! Developer tools: Place Furniture puts a piece on the player's site level by
//! hand, replacing any it overlaps, for everyone on that level and its mobs.
//! Not kept: the level is made afresh, as furnished, when next loaded.

use bevy::prelude::*;
use shared::dungeon::Cell;
use shared::furniture::Piece;

use super::doors::changed_at;
use super::update_sight;
use crate::ServerWorld;
use crate::connections::Client;
use crate::storage::SiteId;

/// A piece put down by hand on a site level, for `share_furniture`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FurnitureChange {
    pub(crate) place: (SiteId, u32),
    pub(crate) piece: Piece,
}

/// Whether furniture may be put on a cell: floor or open ground, not a
/// doorway, a gate, water or a bridge.
fn takes_furniture(cell: Cell) -> bool {
    cell.is_walkable()
        && !matches!(
            cell,
            Cell::Door | Cell::FieldGate | Cell::WorldGate | Cell::Shallows | Cell::Bridge
        )
}

/// Checks the piece (a shape furnishing could make, on the level's floor, off
/// its stairs) and has it shared at the end of the tick. Refusals are only
/// logged. Whether the client is still welcome.
pub(crate) fn place_furniture(client: &mut Client, world: &mut ServerWorld, piece: Piece) -> bool {
    let Some(visit) = &client.visit else {
        return true;
    };
    let site = &visit.level;
    let on_floor = |(x, y): (i64, i64)| {
        site.level.contains(x, y)
            && takes_furniture(site.level.cell(x, y))
            && !site.stairs.iter().any(|s| (s.x, s.y) == (x, y))
    };
    if visit.is_wilderness() || !piece.is_well_made() || !piece.cells().all(on_floor) {
        info!("Refused placing furniture at {:?}", piece.at);
        return true;
    }
    let place = (visit.id, visit.index);
    world
        .furniture_changes
        .push(FurnitureChange { place, piece });
    true
}

/// Every tick, before doors are shared: puts each piece placed by hand on the
/// level of everyone on it (sending the cells it and what it replaced cover
/// again), and on its mobs'.
pub(crate) fn share_furniture(clients: &mut [Client], world: &mut ServerWorld) {
    let changes = std::mem::take(&mut world.furniture_changes);
    for change in &changes {
        if let Some(population) = world.populations.0.get_mut(&change.place) {
            population.place_furniture(change.piece);
        }
    }
    for client in clients.iter_mut() {
        let Some(visit) = &mut client.visit else {
            continue;
        };
        let here = (visit.id, visit.index);
        let wild = visit.is_wilderness();
        let mine = changes
            .iter()
            .filter(|change| change.place == here && !wild);
        let mut any = false;
        for change in mine {
            let level = &visit.level.level;
            let replaced: Vec<(i64, i64)> = (change.piece.cells())
                .filter_map(|(x, y)| level.furniture_at(x, y))
                .flat_map(Piece::cells)
                .collect();
            visit.level.level.place_furniture(change.piece);
            for (x, y) in change.piece.cells().chain(replaced) {
                changed_at(visit, x, y);
            }
            any = true;
        }
        if any && !update_sight(client) {
            // Gone: noticed when its messages are next read.
            continue;
        }
    }
}

#[cfg(test)]
mod tests;
