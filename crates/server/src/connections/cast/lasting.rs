//! Lasting effects landing on creatures (magic plan PR 12): Earth's
//! paralysis and a Curse take hold only by the ratio contest (paralysis
//! against Will, a Curse against Aura, each with Warding), a poison's harm a
//! second is shared out by resisting as harm is, and Stoneskin, an Illusion
//! and Warding, which help, always lie on whoever they land on. Players keep
//! them on their sheet (`Sheet::add_effect`), paralysis spending the energy
//! it lasts as a drunken collapse does; mobs as `entities::lay_on_mob`.

use shared::bonus::Effect as Timed;
use shared::spell::{
    CURSED, PARALYSED, POISONED, Part, effect_power, lasting, poison, poison_per_second, resisted,
    takes_hold,
};

use super::super::Client;
use super::effects::Landing;
use crate::ServerWorld;
use crate::combat::luck;
use crate::entities::{Harmer, lay_on_mob};

/// `count` of `part` lie on each target they take hold on. Whether any did.
pub(super) fn lay_lasting(
    clients: &mut [Client],
    caster: usize,
    world: &mut ServerWorld,
    at: &Landing,
    targets: &[usize],
    (part, count): (Part, u32),
) -> bool {
    let me = &clients[caster];
    let skills = Some(&me.sheet.skills);
    let power = effect_power(part.effect, &me.bonused.attributes, skills);
    let per_second = poison_per_second(skills);
    let made = lasting(part, count, skills, at.tick);
    let by = Harmer {
        id: me.entity_id(),
        position: me.visit.as_ref().map_or((0, 0), |visit| visit.position()),
        kind: me.inventory.creature,
    };
    let mut changed = false;
    for &index in targets {
        let id = at.present.ids[index];
        let mut rng = luck(at.tick, by.id, id);
        let effect = match part {
            PARALYSED if !takes_hold(power, at.present.wills[index], &mut rng) => continue,
            CURSED if !takes_hold(power, at.present.resistances[index], &mut rng) => continue,
            POISONED => {
                let resistance = at.present.resistances[index];
                poison(resisted(per_second, power, resistance), count, at.tick)
            }
            _ => match &made {
                Some(effect) => effect.clone(),
                None => return changed,
            },
        };
        changed |= match at.present.players[index] {
            Some(player) => lay_on_player(&mut clients[player], effect, at.tick),
            None => {
                let poisoner = (part == POISONED).then_some((by, at.index));
                lay_on_mob(world, at.place, id, effect, poisoner, at.tick)
            }
        };
    }
    changed
}

/// A spell's `effect` lies on a player from `tick`.
fn lay_on_player(client: &mut Client, effect: Timed, tick: u64) -> bool {
    let paralysis = effect.source == shared::bonus::Source::Spell(PARALYSED);
    let ticks = effect.end.unwrap_or(tick).saturating_sub(tick);
    let (sheet, situation) = client.sheet_and_situation();
    sheet.add_effect(effect, situation, tick);
    client.refresh_attributes(tick);
    if paralysis {
        // Held still: no energy until it is over (as `alcohol.rs`'s collapse).
        let cost = u64::from(client.speed) * ticks;
        let cost = u32::try_from(cost).unwrap_or(u32::MAX);
        client.energy.spend(tick, client.speed, cost);
    }
    client.unsaved = true;
    client.send_sheet(tick);
    client.send_vitals(tick);
    true
}
