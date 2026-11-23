//! Removing effects landing on creatures (magic plan PR 12): Poison reversed
//! cures, Curse reversed lifts a Curse and the curses on what a player wears,
//! Illusion reversed ends Illusions, Warding reversed dispels every spell's
//! effect. Never resisted: taking a spell away needs no contest.

use shared::item::Favour;
use shared::spell::Ends;

use super::super::Client;
use super::effects::Landing;
use crate::ServerWorld;
use crate::entities::end_mob_spells;

/// `ends` takes its effects off each target. Whether any were.
pub(super) fn end_on_targets(
    clients: &mut [Client],
    world: &mut ServerWorld,
    at: &Landing,
    targets: &[usize],
    ends: Ends,
) -> bool {
    let mut changed = false;
    for &index in targets {
        changed |= match at.present.players[index] {
            Some(player) => end_on_player(&mut clients[player], ends, at.tick),
            None => end_mob_spells(world, at.place, at.present.ids[index], ends, at.tick),
        };
    }
    changed
}

/// Ends `ends`'s effects on a player (a chill dispelled, their speed back),
/// and lifts curses from what they wear. Whether anything was.
fn end_on_player(client: &mut Client, ends: Ends, tick: u64) -> bool {
    let (sheet, situation) = client.sheet_and_situation();
    let mut changed = sheet.end_effects(ends, situation, tick);
    if ends == Ends::Curse {
        changed |= lift_curses(client);
    }
    if !changed {
        return false;
    }
    client.refresh_attributes(tick);
    client.update_speed(tick);
    client.unsaved = true;
    client.send_sheet(tick);
    client.send_vitals(tick);
    true
}

/// Takes the curse off everything a player wears, now known not to be
/// cursed. Whether any was.
fn lift_curses(client: &mut Client) -> bool {
    let mut lifted = false;
    for item in &mut client.inventory.worn {
        if item.favour == Some(Favour::Cursed) {
            item.favour = None;
            item.known.favour = true;
            lifted = true;
        }
    }
    if lifted {
        client.send_inventory();
    }
    lifted
}
