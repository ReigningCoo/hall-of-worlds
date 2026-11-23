//! Once every client has had its turn: each spell's step that is due
//! resolves, then the next is due a second later (`spell::STEP_MILLIS`), or,
//! after a missile step, as soon as its missiles have landed. The first step
//! lands as the Mana is paid, or fizzles.

use bevy::prelude::*;
use shared::magic::PRACTICE_PER_CAST;
use shared::protocol::{Landed, ServerMessage, SpellNews};
use shared::skill::SkillId;
use shared::spell::{Look, STEP_MILLIS, Step, Where, acts_on_place, spell_cost};
use shared::vitals::Vital;

use super::super::Client;
use super::Aimed;
use super::creatures::{Present, present};
use super::effects::{Landing, land};
use super::flight::{fly, launch_missiles, missile_paths, tell_level};
use super::step::{effect_targets, landing_cells, where_step};
use crate::{ServerWorld, ServerWorlds, TICKS_PER_SECOND};

/// Resolves every step that is due, and flies every missile on, for every
/// client.
pub(in crate::connections) fn run_casts(clients: &mut [Client], worlds: &mut ServerWorlds) {
    for i in 0..clients.len() {
        let world = worlds.of(clients[i].world);
        run_due(clients, i, world);
    }
}

/// Resolves client `i`'s step if it is due on `world`, its world, or flies
/// its missiles on.
pub(in crate::connections) fn run_due(clients: &mut [Client], i: usize, world: &mut ServerWorld) {
    let tick = world.action_clock.ticks();
    let Some(cast) = clients[i].casting.running.as_ref() else {
        return;
    };
    let flying = !cast.flying.is_empty();
    if clients[i].gone || !(flying || cast.due <= tick) {
        return;
    }
    let Some(mut cast) = clients[i].casting.running.take() else {
        return;
    };
    let Some(present) = present(clients, i, world, cast.place) else {
        debug!("A spell stopped: its caster left the level");
        return;
    };
    if flying {
        if let Some(aimed) = fly(clients, &mut cast, &present, tick) {
            // The next step starts as the last missile lands.
            finish_where(&mut clients[i], cast, aimed, tick, 0);
        } else {
            clients[i].casting.running = Some(cast);
        }
        return;
    }
    run_step(clients, i, world, cast, &present);
}

/// Resolves client `i`'s next step, ending the spell after its last or when
/// it fizzles.
fn run_step(
    clients: &mut [Client],
    i: usize,
    world: &mut ServerWorld,
    mut cast: super::Cast,
    present: &Present,
) {
    let tick = world.action_clock.ticks();
    let first = cast.next == 0;
    let step = cast.spell.steps[cast.next].clone();
    let effects = match &step {
        Step::Where(place) => {
            let Some(level) = clients[i].visit.as_ref().map(|visit| visit.cells()) else {
                return;
            };
            let heading = clients[i].heading;
            let Some(part) = place.parts().nth(cast.part) else {
                return;
            };
            if part.place != Where::Missile {
                let (aimed, looks) = where_step(&cast, place, present, level, heading);
                // A first step that hits nobody fizzles: nothing to see.
                let last = cast.part + 1 == place.len();
                let hit = (aimed.iter().chain(&cast.gathered)).any(|a| a.creature.is_some());
                let on_place = !aimed.is_empty() && acts_on_place(&cast.spell);
                if !first || !last || hit || on_place {
                    for shown in looks {
                        tell_level(
                            clients,
                            present,
                            &ServerMessage::Spell(SpellNews::Where(shown)),
                        );
                    }
                }
                finish_where(&mut clients[i], cast, aimed, tick, STEP_TICKS);
                return;
            }
            let paths = missile_paths(&cast, (place, present, level), heading);
            if paths.is_empty() {
                // A first where aimed at nobody fizzles; a merged one with
                // nothing to start from ends its step.
                if cast.part == 0 {
                    fizzle(&mut clients[i], cast.toward);
                } else {
                    finish_where(&mut clients[i], cast, Vec::new(), tick, STEP_TICKS);
                }
                return;
            }
            launch_missiles(clients, i, &mut cast, (paths, present), tick);
            clients[i].casting.running = Some(cast);
            return;
        }
        Step::Effects(effects) => effects,
    };
    if first && !pay(&mut clients[i], &cast, tick) {
        fizzle(&mut clients[i], cast.toward);
        return;
    }
    let targets = effect_targets(&cast, present);
    let on = (targets.iter())
        .map(|&index| present.creatures[index].body.at)
        .collect();
    let look = Look::of_effects(effects);
    tell_level(
        clients,
        present,
        &ServerMessage::Spell(SpellNews::Landed { on, look }),
    );
    let Some(index) = clients[i].visit.as_ref().map(|visit| visit.index()) else {
        return;
    };
    let at = Landing {
        present,
        place: cast.place,
        index,
        tick,
    };
    let cells = landing_cells(&cast, present);
    let skills = land(clients, i, world, &at, effects, (&targets, &cells));
    if !skills.is_empty() {
        let skills = cast.forms.iter().copied().chain(skills);
        practise(&mut clients[i], skills, tick);
    }
    next_step(&mut clients[i], cast, tick + STEP_TICKS);
}

/// Action ticks between steps (`spell::STEP_MILLIS`).
const STEP_TICKS: u64 = STEP_MILLIS * TICKS_PER_SECOND as u64 / 1000;

/// A where has hit `aimed`. A merged where follows from those hits on the
/// next tick; after a step's last where, everything its wheres hit is the
/// next step's targets, due `after` ticks on. A first step that hit nobody,
/// or without the Mana as it lands, fizzles.
fn finish_where(
    client: &mut Client,
    mut cast: super::Cast,
    aimed: Vec<Aimed>,
    tick: u64,
    after: u64,
) {
    let Some(Step::Where(step)) = cast.spell.steps.get(cast.next) else {
        return;
    };
    let wheres = step.len();
    let form = step.parts().nth(cast.part).map(|part| part.place.skill());
    cast.forms.extend(form);
    cast.gathered.extend(aimed.iter().copied());
    if cast.part + 1 < wheres && !aimed.is_empty() {
        // The step's next where leads on from these hits.
        cast.part += 1;
        cast.aimed = aimed;
        cast.due = tick;
        client.casting.running = Some(cast);
        return;
    }
    let all = std::mem::take(&mut cast.gathered);
    cast.part = 0;
    let first = cast.next == 0;
    // Nobody hit is a fizzle, unless the spell does something to the place.
    let landed = all.iter().any(|aimed| aimed.creature.is_some())
        || (!all.is_empty() && acts_on_place(&cast.spell));
    if first && !landed {
        fizzle(client, cast.toward);
        return;
    }
    if first && !pay(client, &cast, tick) {
        fizzle(client, cast.toward);
        return;
    }
    cast.aimed = all;
    next_step(client, cast, tick + after);
}

/// On to the next step, due at `due`, if there is one.
fn next_step(client: &mut Client, mut cast: super::Cast, due: u64) {
    cast.next += 1;
    if cast.next < cast.spell.steps.len() {
        cast.due = due;
        client.casting.running = Some(cast);
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
