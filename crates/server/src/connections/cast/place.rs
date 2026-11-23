//! What outcomes do to the cells they land on (magic plan PR 11): Fire
//! lights fireplaces, Water and steam put fires out, ice freezes water round
//! them and steam leaves clouds blocking sight (`spell::Acts`). The cells are
//! those the last where hit, creature or not, or the caster's own.

use std::collections::BTreeSet;

use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::protocol::{ServerMessage, SpellNews};
use shared::spell::{Acts, ICE_SECONDS, STEAM_SECONDS, cloud_cells, frozen_cells};

use super::super::Client;
use super::effects::Landing;
use super::flight::tell_level;
use crate::ServerWorld;
use crate::sites::{douse, light_by_spell};

/// `count` of an outcome that `acts` so on `cells`. Whether it changed
/// anything there.
pub(super) fn act_on_cells(
    clients: &mut [Client],
    caster: usize,
    world: &mut ServerWorld,
    at: &Landing,
    cells: &[(i64, i64)],
    (acts, count): (Acts, u32),
) -> bool {
    let Some(visit) = clients[caster].visit.as_ref() else {
        return false;
    };
    let ticks = |seconds: u64| seconds * u64::from(count) * ACTION_TICKS_PER_SECOND as u64;
    let mut changed = false;
    let mut clouds = BTreeSet::new();
    for &cell in cells {
        if acts.lights {
            changed |= light_by_spell(world, visit, cell);
        }
        if acts.douses {
            changed |= douse(world, visit, cell);
        }
        if acts.freezes {
            let until = at.tick + ticks(ICE_SECONDS);
            for (x, y) in frozen_cells(visit.cells(), cell, count) {
                let was = visit.cells().cell(x, y);
                world.spell_marks.freeze(visit.plant_key(x, y), was, until);
                changed = true;
            }
        }
        if acts.clouds {
            let until = at.tick + ticks(STEAM_SECONDS);
            for (x, y) in cloud_cells(visit.cells(), cell, count) {
                world.spell_marks.cloud(visit.plant_key(x, y), until);
                clouds.insert((x, y));
            }
        }
    }
    if !clouds.is_empty() {
        let millis = (STEAM_SECONDS * 1000 * u64::from(count)).min(u64::from(u32::MAX)) as u32;
        let news = SpellNews::Clouds {
            cells: clouds.into_iter().collect(),
            millis,
        };
        tell_level(clients, at.present, &ServerMessage::Spell(news));
    }
    changed
}
