//! Identify landing on players (magic plan PR 13): what everything they wear
//! hides, whether it is magical and whether it is blessed or cursed, is found
//! out, for everyone from then on (`item::Known`); reversed (Conceal), it is
//! hidden again. Mobs show nothing of theirs yet, and there are no secret
//! doors or traps to find yet, so it passes them and the ground by.

use super::super::Client;
use super::effects::Landing;

/// Finds out (`reveal`) or hides what the worn things of each player among
/// `targets` hide. Whether anything was found out or hidden.
pub(super) fn identify_targets(
    clients: &mut [Client],
    at: &Landing,
    targets: &[usize],
    reveal: bool,
) -> bool {
    let mut changed = false;
    for &index in targets {
        let Some(player) = at.present.players[index] else {
            continue;
        };
        let client = &mut clients[player];
        if know_worn(client, reveal) {
            client.unsaved = true;
            client.send_inventory();
            changed = true;
        }
    }
    changed
}

/// Sets whether everything a player wears is known to be magical or not and
/// blessed, cursed or neither. Whether any changed.
fn know_worn(client: &mut Client, known: bool) -> bool {
    let mut changed = false;
    for item in &mut client.inventory.worn {
        let can_hide = item.can_be_magical() || item.favour.is_some();
        // Conceal hides only what there is to hide: plain things stay known
        // to be plain.
        let wanted = shared::item::Known {
            magic: known || (!can_hide && item.known.magic),
            favour: known || (!can_hide && item.known.favour),
        };
        if item.known != wanted {
            item.known = wanted;
            changed = true;
        }
    }
    changed
}
