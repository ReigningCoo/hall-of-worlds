//! The stage a creature is at, kept as one effect (`Source::Alcohol`) ending
//! where the stage changes, so the effect bar shows it and its penalties count
//! like any bonus. Whoever keeps the sheet (the server) swaps it as the stage
//! changes (`Sheet::keep_alcohol`); speed goes by the one there
//! (`Sheet::alcohol_stage`).

use super::{Drinker, Stage};
use crate::bonus::{Effect, EndsOn, Source};
use crate::creature::CreatureKind;
use crate::fungus::{Eater, with_alcohol};
use crate::sheet::{Sheet, Situation};

/// The effect of being at `stage` from `start` until `end`.
pub fn stage_effect(stage: Stage, start: u64, end: Option<u64>) -> Effect {
    Effect {
        source: Source::Alcohol(stage),
        bonuses: stage.bonuses(),
        start,
        end,
        ends_on: EndsOn::NOTHING,
    }
}

impl Sheet {
    /// The stage its effects say it is at, ended or not: what speed and
    /// stumbles go by, kept up to date by `keep_alcohol`.
    pub fn alcohol_stage(&self) -> Option<Stage> {
        self.effects
            .0
            .iter()
            .find_map(|effect| match effect.source {
                Source::Alcohol(stage) => Some(stage),
                _ => None,
            })
    }

    /// Drinks `milli` thousandths of a unit at `now`, as a creature of `kind`,
    /// and keeps its stage's effect up to date. Within a fungus's window
    /// (the common ink cap's, `fungus::with_alcohol`) it brings on that
    /// fungus's poison (no deadly one comes this way, so Easy isn't asked).
    pub fn drink_alcohol(
        &mut self,
        milli: u32,
        kind: CreatureKind,
        situation: Situation,
        now: u64,
    ) {
        let drinker = Drinker::of(self, kind);
        self.alcohol.drink(milli, now, drinker);
        // A stage reached again starts a fresh effect, ending where it now does.
        self.change_effects(situation, now, |effects| {
            effects
                .0
                .retain(|effect| !matches!(effect.source, Source::Alcohol(_)));
        });
        self.keep_alcohol(kind, situation, now);
        let eater = Eater {
            most_health: self.limits(situation, now).health,
            easy: false,
        };
        let poisons = with_alcohol(&self.effects, now, eater);
        if !poisons.is_empty() {
            self.lay_on(poisons, true, situation, now);
        }
    }

    /// Swaps its stage's effect for the stage it is at `now`, if that has
    /// changed (or its effect has ended): whether it did.
    pub fn keep_alcohol(&mut self, kind: CreatureKind, situation: Situation, now: u64) -> bool {
        let drinker = Drinker::of(self, kind);
        let stage = self.alcohol.stage(now, drinker);
        let lasting = self
            .effects
            .0
            .iter()
            .find_map(|effect| match effect.source {
                Source::Alcohol(stage) if effect.lasts(now) => Some(stage),
                _ => None,
            });
        let present = self.alcohol_stage().is_some();
        if lasting == stage && (stage.is_some() || !present) {
            return false;
        }
        let end = self.alcohol.next_change(now, drinker);
        self.change_effects(situation, now, |effects| {
            effects
                .0
                .retain(|effect| !matches!(effect.source, Source::Alcohol(_)));
            if let Some(stage) = stage {
                effects.add(stage_effect(stage, now, end));
            }
        });
        true
    }
}
