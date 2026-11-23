//! What spells leave on levels for a while (magic plan PR 11): water frozen
//! by ice (`Cell::Ice`, walked on, what was there put back after) and clouds
//! of steam (`Level::set_cloud`, blocking sight). Kept by cell where plants
//! are (`PlantKey`, so the wilderness and its cutouts agree), until an action
//! tick, never saved: a restart thaws everything. Every tick each player's
//! level is given them as they are now, its cells sent again where that
//! changed, and those over are forgotten.

use std::collections::BTreeMap;

use shared::dungeon::Cell;

use super::doors::changed_at;
use super::plants::PlantKey;
use super::update_sight;
use crate::ServerWorld;
use crate::connections::Client;

/// Ice and steam on levels, and until when.
#[derive(Default)]
pub(crate) struct SpellMarks {
    /// Frozen water: what the cell was, and the tick it thaws at.
    ice: BTreeMap<PlantKey, (Cell, u64)>,
    /// Clouds of steam: the tick each clears at.
    clouds: BTreeMap<PlantKey, u64>,
}

impl SpellMarks {
    /// Freezes the cell at `key`, which is `was`, until `until`; one frozen
    /// already keeps what it was, lasting the longer.
    pub(crate) fn freeze(&mut self, key: PlantKey, was: Cell, until: u64) {
        let kept = self.ice.entry(key).or_insert((was, until));
        kept.1 = kept.1.max(until);
    }

    /// A cloud over the cell at `key` until `until`.
    pub(crate) fn cloud(&mut self, key: PlantKey, until: u64) {
        let kept = self.clouds.entry(key).or_insert(until);
        *kept = (*kept).max(until);
    }
}

/// Sets every player's level's ice and clouds as they are now, then forgets
/// those over.
pub(crate) fn sync_spell_marks(clients: &mut [Client], world: &mut ServerWorld) {
    let marks = &world.spell_marks;
    if marks.ice.is_empty() && marks.clouds.is_empty() {
        return;
    }
    let tick = world.action_clock.ticks();
    for client in clients.iter_mut() {
        let Some(visit) = &mut client.visit else {
            continue;
        };
        let mut changed = false;
        for (&key, &(was, until)) in &marks.ice {
            let Some((x, y)) = visit.cell_of(key) else {
                continue;
            };
            let now = visit.cells().cell(x, y);
            let wanted = if until > tick { Cell::Ice } else { was };
            if now != wanted && (now == was || now == Cell::Ice) {
                visit.level.level.set(x, y, wanted);
                changed_at(visit, x, y);
                changed = true;
            }
        }
        for (&key, &until) in &marks.clouds {
            let Some((x, y)) = visit.cell_of(key) else {
                continue;
            };
            let wanted = until > tick;
            if visit.cells().cloud_at(x, y) != wanted {
                visit.level.level.set_cloud(x, y, wanted);
                changed_at(visit, x, y);
                changed = true;
            }
        }
        if changed {
            // Gone: noticed when its messages are next read.
            let _ = update_sight(client);
        }
    }
    let marks = &mut world.spell_marks;
    marks.ice.retain(|_, (_, until)| *until > tick);
    marks.clouds.retain(|_, until| *until > tick);
}
