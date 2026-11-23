//! Once every client has had its turn: each spell's step that is due
//! resolves, then the next is due a second later (`spell::STEP_MILLIS`).
//! The first step lands as the Mana is paid, or fizzles.

use bevy::prelude::*;
use shared::magic::PRACTICE_PER_CAST;
use shared::protocol::{Landed, ServerMessage};
use shared::skill::SkillId;
use shared::spell::{Effect, STEP_MILLIS, Step, spell_cost};
use shared::vitals::Vital;

use super::super::Client;
use super::creatures::present;
use super::effects::{heal_amount, heal_targets, heals_in};
use super::step::{effect_targets, where_step};
use crate::{ServerWorld, ServerWorlds, TICKS_PER_SECOND};

/// Resolves every step that is due, for every client.
pub(in crate::connections) fn run_casts(clients: &mut [Client], worlds: &mut ServerWorlds) {
    for i in 0..clients.len() {
        let world = worlds.of(clients[i].world);
        run_due(clients, i, world);
    }
}

/// Resolves client `i`'s step if it is due on `world`, its world.
pub(in crate::connections) fn run_due(clients: &mut [Client], i: usize, world: &mut ServerWorld) {
    let tick = world.action_clock.ticks();
    let due = (clients[i].casting.running.as_ref()).is_some_and(|cast| cast.due <= tick);
    if due && !clients[i].gone {
        run_step(clients, i, world);
    }
}

/// Resolves client `i`'s next step, ending the spell after its last, when it
/// fizzles, or when the caster has left its level.
fn run_step(clients: &mut [Client], i: usize, world: &mut ServerWorld) {
    let Some(mut cast) = clients[i].casting.running.take() else {
        return;
    };
    let tick = world.action_clock.ticks();
    let Some(present) = present(clients, i, world, cast.place) else {
        debug!("A spell stopped: its caster left the level");
        return;
    };
    let first = cast.next == 0;
    let step = cast.spell.steps[cast.next].clone();
    let targets = match &step {
        Step::Where(place) => {
            let Some(level) = clients[i].visit.as_ref().map(|visit| visit.cells()) else {
                return;
            };
            let aimed = where_step(&cast, place, &present, level, clients[i].heading);
            if first && !aimed.iter().any(|aimed| aimed.creature.is_some()) {
                fizzle(&mut clients[i], cast.toward);
                return;
            }
            cast.forms.push(place.place.skill());
            cast.aimed = aimed;
            None
        }
        Step::Effects(_) => Some(effect_targets(&cast, &present)),
    };
    if first && !pay(&mut clients[i], &cast, tick) {
        fizzle(&mut clients[i], cast.toward);
        return;
    }
    if let (Step::Effects(effects), Some(targets)) = (&step, targets) {
        let count = heals_in(effects);
        let amount = heal_amount(&clients[i], count);
        let place = (&present, cast.place);
        if count > 0 && heal_targets(clients, i, world, place, &targets, amount) {
            let skills = cast.forms.iter().copied().chain([Effect::Heal.skill()]);
            practise(&mut clients[i], skills, tick);
        }
    }
    cast.next += 1;
    if cast.next < cast.spell.steps.len() {
        cast.due = tick + STEP_MILLIS * TICKS_PER_SECOND as u64 / 1000;
        clients[i].casting.running = Some(cast);
    }
}

/// Pays the spell's Mana as its first step lands, telling the caster.
/// Whether there was enough.
fn pay(client: &mut Client, cast: &super::Cast, tick: u64) -> bool {
    let limits = client.limits(tick);
    let cost = spell_cost(&cast.spell, Some(&client.sheet.skills));
    let played = client.activity.played;
    if !client.sheet.vitals.spend_mana(cost, limits, tick, played) {
        return false;
    }
    client.unsaved = true;
    let mana = client.sheet.vitals.at(Vital::Mana, limits, tick);
    debug!("A spell landed for {cost:?} Mana, {mana:?} left");
    client.send_vitals(tick);
    client.link.send(&ServerMessage::AbilityLanded {
        who: None,
        target: cast.toward,
        landed: Landed::Cast,
    });
    true
}

/// Tells the caster their spell fizzled: nothing paid.
fn fizzle(client: &mut Client, target: Option<shared::id::Id>) {
    client.link.send(&ServerMessage::AbilityLanded {
        who: None,
        target,
        landed: Landed::Fizzled,
    });
}

/// Practises the skills an effect that did something leant on: its own and
/// the Form skills of the wheres that carried it, each at most once in 5
/// seconds (`Windows`), as Heal was.
fn practise(client: &mut Client, skills: impl Iterator<Item = SkillId>, tick: u64) {
    let played = client.activity.played;
    for skill in skills {
        let practice = client.learning.skills.take(skill, PRACTICE_PER_CAST, tick);
        client.sheet.skills.practise(skill, practice);
        client.sheet.skills.used(skill, played);
    }
    client.unsaved = true;
    client.send_sheet(tick);
}
