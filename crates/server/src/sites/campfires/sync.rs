//! Every tick: each player's level given its campfires as they are now (how
//! many pieces each has left), its cells sent again where that changed; fires
//! found burnt out forgotten, their Ash left on the ground where they burnt.

use std::collections::BTreeMap;

use shared::campfire::Campfire;
use shared::creature::Size;
use shared::item::{Item, ItemKind};
use shared::spot::Spot;

use super::{fire_now, keep_campfire};
use crate::ServerWorld;
use crate::connections::Client;
use crate::ids::new_id;
use crate::sites::doors::changed_at;
use crate::sites::{Visit, update_sight};

/// Sets every player's level's campfires as they are now.
pub(crate) fn sync_campfires(clients: &mut [Client], world: &mut ServerWorld) {
    let now = fire_now(world);
    for client in clients.iter_mut() {
        let Some(visit) = &mut client.visit else {
            continue;
        };
        let wanted = fires_here(world, visit, now);
        let had: BTreeMap<(i64, i64), u8> = visit.cells().campfires().collect();
        if wanted == had {
            continue;
        }
        let cells: Vec<(i64, i64)> = had.keys().chain(wanted.keys()).copied().collect();
        for (x, y) in cells {
            let pieces = wanted.get(&(x, y)).copied();
            if visit.cells().campfire_at(x, y) != pieces {
                visit.level.level.set_campfire(x, y, pieces);
                changed_at(visit, x, y);
            }
        }
        // Gone: noticed when its messages are next read.
        let _ = update_sight(client);
    }
}

/// The fires burning on the visit's level now, by cell, with the pieces each
/// has left; any found burnt out are put out (`burn_out`).
fn fires_here(world: &mut ServerWorld, visit: &Visit, now: u64) -> BTreeMap<(i64, i64), u8> {
    let places: Vec<_> = std::iter::once((visit.id, visit.index))
        .chain(visit.mirrors.iter().map(|mirror| (mirror.id, mirror.index)))
        .collect();
    let mut here = BTreeMap::new();
    for (site, index) in places {
        let kept: Vec<((i64, i64), Campfire)> = (world.site(site).levels.get(&index))
            .map(|level| level.campfires.clone().into_iter().collect())
            .unwrap_or_default();
        for (at, fire) in kept {
            let key = (site, index, at);
            let Some(cell) = visit.cell_of(key) else {
                continue;
            };
            if fire.is_lit(now) {
                let pieces = fire.pieces_left(now).min(usize::from(u8::MAX)) as u8;
                here.insert(cell, pieces);
            } else {
                keep_campfire(world, key, None);
                burn_out(world, visit, cell, &fire);
            }
        }
    }
    here
}

/// Leaves a burnt-out fire's Ash on the ground of the visit's level, at its
/// cell (into a bag already in reach there, as anything dropped).
fn burn_out(world: &mut ServerWorld, visit: &Visit, cell: (i64, i64), fire: &Campfire) {
    let mut ash = Item::new(new_id(), ItemKind::Ash);
    ash.count = fire.ash().max(1);
    let level = world
        .site_mut(visit.id)
        .levels
        .entry(visit.index)
        .or_default();
    crate::ground::drop_at(
        &mut level.ground,
        Size::Medium,
        Spot::middle_of(cell),
        [ash],
    );
}
