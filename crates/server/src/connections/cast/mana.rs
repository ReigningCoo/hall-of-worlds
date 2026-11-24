//! Mana landing on players (magic plan PR 13): given, half the spell's cost
//! up to all of it by the caster's Mana skill, shared between the targets
//! (`spell::mana_given`), so casting it on yourself never gains you any; or,
//! reversed, drained, each target losing Heal's line by Mana skill shared out
//! by resisting as harm is, the caster getting back what was drained up to
//! the spell's cost. Mobs keep no Mana to speak of, so it passes them by.

use shared::spell::{Effect, effect_power, mana_drained, mana_given, resisted};
use shared::vitals::{Points, Vital};

use super::super::Client;
use super::effects::Landing;

/// Mana given to (`gives`) or drained from each player among `targets`, by
/// `count`. Whether anyone's Mana changed.
pub(super) fn mana_targets(
    clients: &mut [Client],
    caster: usize,
    at: &Landing,
    targets: &[usize],
    (gives, count): (bool, u32),
) -> bool {
    let players: Vec<usize> = (targets.iter())
        .filter_map(|&index| at.present.players[index])
        .collect();
    if players.is_empty() {
        return false;
    }
    let me = &clients[caster];
    let skills = Some(&me.sheet.skills);
    if gives {
        let each = mana_given(at.cost, players.len(), skills);
        let mut changed = false;
        for player in players {
            changed |= change_mana(&mut clients[player], each, at.tick) != Points::ZERO;
        }
        return changed;
    }
    let power = effect_power(Effect::Mana, &me.bonused.attributes, skills);
    let each = mana_drained(count, skills);
    let mut drained = 0u32;
    for &index in targets {
        let Some(player) = at.present.players[index] else {
            continue;
        };
        let amount = resisted(each, power, at.present.resistances[index]);
        let client = &mut clients[player];
        let limits = client.limits(at.tick);
        let taken = (client.sheet.vitals).drain_mana(amount, limits, at.tick);
        client.unsaved = true;
        client.send_vitals(at.tick);
        drained = drained.saturating_add(taken.milli());
    }
    let back = Points(drained.min(at.cost.milli()));
    if back != Points::ZERO {
        change_mana(&mut clients[caster], back, at.tick);
    }
    drained > 0
}

/// Gives a player `amount` Mana, telling them: how much they gained.
fn change_mana(client: &mut Client, amount: Points, tick: u64) -> Points {
    let limits = client.limits(tick);
    let before = client.sheet.vitals.at(Vital::Mana, limits, tick);
    (client.sheet.vitals).restore(Vital::Mana, amount, limits, tick);
    let after = client.sheet.vitals.at(Vital::Mana, limits, tick);
    client.unsaved = true;
    client.send_vitals(tick);
    Points(after.milli().saturating_sub(before.milli()))
}
