//! Harmful outcomes landing as Health lost (magic plan PR 11): fire, cold,
//! steam and harm (Heal reversed), by the caster's skill against each
//! target's Aura and Warding (`spell::resisted`). Fire does half to the wet
//! and dries them. Spells never take the caster or a friend below 1 Health
//! (`spell::spared_harm`), so players are never killed by them; mobs die as
//! by a blow. The caster hears of each.

use shared::bonus::Cancel;
use shared::id::Id;
use shared::protocol::{Landed, ServerMessage};
use shared::spell::{
    Effect, Outcome, Part, Side, WET, WET_FIRE, harm_before_resisting, resisted, spared_harm,
    spell_power,
};
use shared::vitals::{Points, Vital};

use super::super::Client;
use super::effects::Landing;
use crate::ServerWorld;
use crate::combat::{Harm, health_left};
use crate::entities::{Harmer, harm_mob};

/// `count` of `outcome` harm each target. Whether any lost Health or was
/// dried.
pub(super) fn harm_targets(
    clients: &mut [Client],
    caster: usize,
    world: &mut ServerWorld,
    at: &Landing,
    targets: &[usize],
    (outcome, count): (Outcome, u32),
) -> bool {
    let me = &clients[caster];
    let skills = Some(&me.sheet.skills);
    let before = harm_before_resisting(outcome, count, skills);
    let power = spell_power(outcome, &me.bonused.attributes, skills);
    let fire = outcome == Outcome::Part(Part::plain(Effect::Fire));
    let harmer = |client: &Client| Harmer {
        id: client.entity_id(),
        position: client
            .visit
            .as_ref()
            .map_or((0, 0), |visit| visit.position()),
        kind: client.inventory.creature,
        member: None,
    };
    let mut changed = false;
    for &index in targets {
        let id = at.present.ids[index];
        let harm = resisted(before, power, at.present.resistances[index]);
        let side = at.present.creatures[index].side;
        let done = match at.present.players[index] {
            Some(player) => harm_player(&mut clients[player], world, (harm, side, fire), at.tick),
            None => {
                let by = harmer(&clients[caster]);
                let place = (at.place, at.index);
                harm_mob(world, place, id, harm, by, at.tick).map(|hit| (hit.harm, hit.left, false))
            }
        };
        let Some((harm, left, dried)) = done else {
            continue;
        };
        changed |= harm != Points::ZERO || dried;
        let target = (index != 0).then_some(id);
        tell_harmed(&mut clients[caster], target, harm, left);
    }
    changed
}

/// A player takes `harm` from a spell (never below 1 Health, as the caster
/// or a friend), half from fire while wet, which dries them: the harm done,
/// Health left per mille, and whether they were dried.
fn harm_player(
    client: &mut Client,
    world: &mut ServerWorld,
    (harm, side, fire): (Points, Side, bool),
    tick: u64,
) -> Option<(Points, u16, bool)> {
    let wet = fire && client.sheet.under_spell(WET, tick);
    let harm = if wet {
        Points((u64::from(harm.milli()) * WET_FIRE / 1000) as u32)
    } else {
        harm
    };
    let dried = wet && client.sheet.end_spell(WET);
    let limits = client.limits(tick);
    let health = client.sheet.vitals.at(Vital::Health, limits, tick);
    let harm = spared_harm(harm, health, side);
    if harm != Points::ZERO {
        // Being hurt ends what it ends, as a blow does.
        client.cancel_effects(Cancel::Hurt, tick);
        let played = client.activity.played;
        client.sheet.vitals.hurt(harm, limits, tick, played);
    }
    let left = health_left(&client.sheet.vitals, limits, tick);
    if harm != Points::ZERO {
        let me = client.entity_id();
        world.harmed.push(Harm {
            id: me,
            left,
            told: me,
        });
    }
    client.unsaved = true;
    client.send_sheet(tick);
    client.send_vitals(tick);
    Some((harm, left, dried))
}

/// Tells the caster their spell harmed `target` (`None`: themselves).
fn tell_harmed(caster: &mut Client, target: Option<Id>, harm: Points, left: u16) {
    caster.link.send(&ServerMessage::AbilityLanded {
        who: None,
        target,
        landed: Landed::Hit { harm, left },
    });
}
