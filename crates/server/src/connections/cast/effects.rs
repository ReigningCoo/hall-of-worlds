//! What an effect step does where it lands, outcome by outcome
//! (`spell::outcomes`, Void read and mixtures made): Heal restores
//! (`heal.rs`); fire, cold, steam and harm take Health (`harm.rs`); Water
//! wets and ice chills (`conditions.rs`); and what they do to the cells
//! themselves (`place.rs`). Effects of the later effects PRs do nothing yet.

use shared::skill::SkillId;
use shared::spell::{Effect, EffectStep, Mixture, Outcome, Part, acts, outcomes};

use super::super::Client;
use super::conditions::{chill_targets, wet_targets};
use super::creatures::Present;
use super::harm::harm_targets;
use super::heal::{heal_amount, heal_targets};
use super::place::act_on_cells;
use crate::ServerWorld;
use crate::storage::SiteId;

/// Where an effect step lands: who is about, the level, the caster's own
/// level index there, and the tick.
pub(super) struct Landing<'a> {
    pub present: &'a Present,
    pub place: (SiteId, u32),
    pub index: u32,
    pub tick: u64,
}

/// Lands `step` on `targets` (by their place in `present`) and on `cells`.
/// The effects' skills to practise: those of every outcome that changed
/// something (Void's too, for a reversed one).
pub(super) fn land(
    clients: &mut [Client],
    caster: usize,
    world: &mut ServerWorld,
    at: &Landing,
    step: &EffectStep,
    (targets, cells): (&[usize], &[(i64, i64)]),
) -> Vec<SkillId> {
    let mut practised = Vec::new();
    for (outcome, count) in outcomes(step) {
        let does = acts(outcome);
        let mut did = false;
        if outcome == Outcome::Part(Part::plain(Effect::Heal)) {
            let amount = heal_amount(&clients[caster], count);
            let place = (at.present, at.place);
            did |= heal_targets(clients, caster, world, place, targets, amount);
        }
        if does.harms {
            did |= harm_targets(clients, caster, world, at, targets, (outcome, count));
        }
        if does.wets {
            did |= wet_targets(clients, at, targets, count);
        }
        if does.chills {
            did |= chill_targets(clients, caster, world, at, targets, count);
        }
        if does.on_place() {
            did |= act_on_cells(clients, caster, world, at, cells, (does, count));
        }
        if did {
            practised.extend(skills_of(outcome));
        }
    }
    practised.sort_unstable();
    practised.dedup();
    practised
}

/// The skills an outcome practises: its effect's, and Void's when reversed;
/// steam Fire's and Water's.
fn skills_of(outcome: Outcome) -> Vec<SkillId> {
    match outcome {
        Outcome::Part(part) if part.reversed => vec![part.effect.skill(), Effect::Void.skill()],
        Outcome::Part(part) => vec![part.effect.skill()],
        Outcome::Mixture(Mixture::Steam) => vec![Effect::Fire.skill(), Effect::Water.skill()],
    }
}
