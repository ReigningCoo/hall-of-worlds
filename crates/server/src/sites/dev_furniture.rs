//! Developer tools: Place Furniture puts a piece on the player's site level by
//! hand, replacing any it overlaps, for everyone on that level and its mobs.
//! Decor (`shared::decor`) too, by `place_decor`, replacing what was in its
//! place. Not kept: the level is made afresh, as furnished, when next loaded;
//! except pieces with places (cask, wine and spirit racks, benches:
//! `furniture::Holder`), kept as
//! Create Crafting Bench's are, so what is put on them isn't lost.

use bevy::prelude::*;
use shared::decor::Decor;
use shared::furniture::{Holder, Piece, takes_furniture};

use super::doors::changed_at;
use super::update_sight;
use crate::ServerWorld;
use crate::connections::Client;
use crate::storage::SiteId;

/// A piece or decor put down by hand on a site level, for `share_furniture`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FurnitureChange {
    pub(crate) place: (SiteId, u32),
    what: Placed,
}

/// What was put down.
#[derive(Clone, Copy, Debug)]
enum Placed {
    Piece(Piece),
    Decor(Decor),
}

impl FurnitureChange {
    pub(crate) fn piece(place: (SiteId, u32), piece: Piece) -> Self {
        Self {
            place,
            what: Placed::Piece(piece),
        }
    }
}

/// Checks the piece (a shape furnishing could make, on the level's floor, off
/// its stairs; one standing on furniture on a table) and has it shared at the end of the tick. Refusals are only
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
    let on_top = piece.furniture.stands_on_furniture();
    let stands = !on_top || site.level.can_stand_on_furniture(&piece);
    if visit.is_wilderness() || !piece.is_well_made() || !piece.cells().all(on_floor) || !stands {
        info!("Refused placing furniture at {:?}", piece.at);
        return true;
    }
    let place = (visit.id, visit.index);
    // Pieces made by hand that it replaces go (on its own layer, and those on
    // top of a floor piece it replaces); one with places is kept.
    let saved = world
        .site_mut(visit.id)
        .levels
        .entry(visit.index)
        .or_default();
    let overlaps = |made: &Piece| {
        let layer = made.furniture.stands_on_furniture() == on_top || !on_top;
        layer && made.cells().any(|cell| piece.covers(cell))
    };
    saved.made_benches.retain(|made| !overlaps(made));
    if Holder::of(piece.furniture).is_some() {
        saved.made_benches.push(piece);
    }
    world
        .furniture_changes
        .push(FurnitureChange::piece(place, piece));
    true
}

/// Checks the decor fits the level (`Decor::fits`), off its stairs, and has
/// it shared at the end of the tick. Refusals are only logged. Whether the
/// client is still welcome.
pub(crate) fn place_decor(client: &mut Client, world: &mut ServerWorld, decor: Decor) -> bool {
    let Some(visit) = &client.visit else {
        return true;
    };
    let site = &visit.level;
    let on_stairs =
        (decor.cells().into_iter()).any(|cell| site.stairs.iter().any(|s| (s.x, s.y) == cell));
    if visit.is_wilderness() || on_stairs || !decor.fits(&site.level) {
        info!("Refused placing decor at {:?}", decor.slot());
        return true;
    }
    let place = (visit.id, visit.index);
    world.furniture_changes.push(FurnitureChange {
        place,
        what: Placed::Decor(decor),
    });
    true
}

/// Every tick, before doors are shared: puts each piece placed by hand on the
/// level of everyone on it (sending the cells it and what it replaced cover
/// again), and on its mobs'.
pub(crate) fn share_furniture(clients: &mut [Client], world: &mut ServerWorld) {
    let changes = std::mem::take(&mut world.furniture_changes);
    for change in &changes {
        if let Placed::Piece(piece) = change.what
            && let Some(population) = world.populations.0.get_mut(&change.place)
        {
            population.place_furniture(piece);
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
            let piece = match change.what {
                Placed::Piece(piece) => piece,
                Placed::Decor(decor) => {
                    // The client replaces what it replaces as the server
                    // does (`Level::place_decor`).
                    visit.level.level.place_decor(decor);
                    for (x, y) in decor.cells() {
                        changed_at(visit, x, y);
                    }
                    any = true;
                    continue;
                }
            };
            let level = &visit.level.level;
            let replaced: Vec<(i64, i64)> = (piece.cells())
                .filter_map(|(x, y)| match piece.furniture.stands_on_furniture() {
                    true => level.furniture_on_top_at(x, y),
                    false => level.furniture_at(x, y),
                })
                .flat_map(Piece::cells)
                .collect();
            visit.level.level.place_furniture(piece);
            for (x, y) in piece.cells().chain(replaced) {
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
