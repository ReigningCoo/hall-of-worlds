//! What an effect step does once Void has been read: each effect either as
//! itself or reversed (`Part`), with how many times. Void reverses every effect
//! after it in its step, and a second Void puts them back; Void itself does
//! nothing, so it is never a part.

use serde::{Deserialize, Serialize};

use super::design::EffectStep;
use super::effect::Effect;

/// One effect as it acts: itself, or reversed by Void (Fire as cold).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Part {
    pub effect: Effect,
    pub reversed: bool,
}

impl Part {
    /// The effect as itself.
    pub const fn plain(effect: Effect) -> Self {
        Self {
            effect,
            reversed: false,
        }
    }

    /// The effect reversed.
    pub const fn reversed(effect: Effect) -> Self {
        Self {
            effect,
            reversed: true,
        }
    }

    /// Its key, as in language files: the effect's own, or what it is
    /// reversed (Fire reversed is `cold`).
    pub const fn key(self) -> &'static str {
        if !self.reversed {
            return self.effect.key();
        }
        match self.effect {
            Effect::Fire => "cold",
            Effect::Water => "ice",
            Effect::Air => "vacuum",
            Effect::Earth => "stoneskin",
            Effect::Void => "void",
            Effect::Identify => "conceal",
            Effect::Curse => "uncurse",
            Effect::Illusion => "disillusion",
            Effect::Warding => "dispel",
            Effect::Heal => "harm",
            Effect::Poison => "cure",
            Effect::Teleport => "anchor",
            Effect::Light => "darkness",
            Effect::Haste => "slow",
            Effect::Calm => "fear",
            Effect::Blessing => "weakness",
            Effect::Levitate => "grounding",
            Effect::Mana => "drain",
        }
    }
}

/// A step's parts in the order first written, the same part written twice (or
/// with a count) added up. Void is read and left out; a Void count of 2
/// reverses twice, back to normal.
pub fn parts(step: &EffectStep) -> Vec<(Part, u32)> {
    let mut reversed = false;
    let mut parts: Vec<(Part, u32)> = Vec::new();
    for &(effect, count) in &step.effects {
        if effect == Effect::Void {
            reversed ^= count % 2 == 1;
            continue;
        }
        if count == 0 {
            continue;
        }
        let part = Part { effect, reversed };
        match parts.iter_mut().find(|(seen, _)| *seen == part) {
            Some((_, total)) => *total += u32::from(count),
            None => parts.push((part, u32::from(count))),
        }
    }
    parts
}
