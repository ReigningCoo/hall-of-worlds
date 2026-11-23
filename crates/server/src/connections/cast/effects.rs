//! What an effect step does to those it lands on. Heal so far (restoring
//! Health by the caster's Healing, `spell::heal_restores`); the other effects
//! come with the effects PRs, and do nothing yet. Whoever is healed is told,
//! and the caster hears of each.

use shared::id::Id;
use shared::protocol::{Landed, ServerMessage};
use shared::spell::{Effect, EffectStep, Outcome, Part, heal_restores, outcomes};
use shared::vitals::{Points, Vital};

use super::super::Client;
use super::creatures::Present;
use crate::ServerWorld;
use crate::combat::health_left;
use crate::entities::heal_mob;
use crate::storage::SiteId;

/// How many Heals a step holds, not reversed (Void's reversal, direct harm,
/// comes with the effects PRs).
pub(super) fn heals_in(step: &EffectStep) -> u32 {
    let heal = Outcome::Part(Part {
        effect: Effect::Heal,
        reversed: false,
    });
    (outcomes(step).into_iter())
        .filter(|(outcome, _)| *outcome == heal)
        .map(|(_, count)| count)
        .sum()
}

/// Heals each target by `amount`, telling them and the caster (client
/// `caster`). Whether anyone gained Health.
pub(super) fn heal_targets(
    clients: &mut [Client],
    caster: usize,
    world: &mut ServerWorld,
    (present, place): (&Present, (SiteId, u32)),
    targets: &[usize],
    amount: Points,
) -> bool {
    let tick = world.action_clock.ticks();
    let mut changed = false;
    for &index in targets {
        let id = present.ids[index];
        let healed = match present.players[index] {
            Some(player) => Some(heal_player(&mut clients[player], amount, tick)),
            None => heal_mob(world, place, id, amount, tick),
        };
        let Some((left, gained)) = healed else {
            continue;
        };
        changed |= gained;
        let target = (index != 0).then_some(id);
        tell_healed(&mut clients[caster], target, amount, left);
    }
    changed
}

/// A player gains this much Health, and is told of it: how much they have
/// now, per mille, and whether they gained any.
fn heal_player(client: &mut Client, amount: Points, tick: u64) -> (u16, bool) {
    let limits = client.limits(tick);
    let before = client.sheet.vitals.at(Vital::Health, limits, tick);
    client
        .sheet
        .vitals
        .restore(Vital::Health, amount, limits, tick);
    client.unsaved = true;
    let gained = client.sheet.vitals.at(Vital::Health, limits, tick) > before;
    client.send_vitals(tick);
    (health_left(&client.sheet.vitals, limits, tick), gained)
}

/// Tells the caster their spell healed `target` (`None`: themselves).
fn tell_healed(caster: &mut Client, target: Option<Id>, amount: Points, left: u16) {
    caster.link.send(&ServerMessage::AbilityLanded {
        who: None,
        target,
        landed: Landed::Healed { amount, left },
    });
}

/// How much `count` Heals from this caster restore.
pub(super) fn heal_amount(caster: &Client, count: u32) -> Points {
    heal_restores(count, Some(&caster.sheet.skills))
}
