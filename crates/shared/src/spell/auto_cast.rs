//! Whether an outcome helps or harms what it lands on, and from that when a
//! spell on the action bar's automatic slots is cast by itself (magic plan,
//! Casting): helpful effects only when the first step's target is low, harmful
//! only at a targeted enemy in reach, anything else by hand.

use serde::{Deserialize, Serialize};

use super::combination::{Mixture, Outcome, outcomes};
use super::design::Spell;
use super::effect::Effect;

/// What an outcome does to what it lands on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aim {
    Helpful,
    Harmful,
    /// Neither: wetting, light, finding secrets, teleporting, dispelling all.
    Neutral,
}

impl Outcome {
    /// Whether it helps or harms.
    pub const fn aim(self) -> Aim {
        use Aim::{Harmful, Helpful, Neutral};
        let part = match self {
            Self::Mixture(Mixture::Steam) => return Harmful,
            Self::Part(part) => part,
        };
        match (part.effect, part.reversed) {
            (Effect::Fire | Effect::Air | Effect::Curse | Effect::Poison, false) => Harmful,
            (Effect::Earth | Effect::Calm, false) => Harmful,
            (Effect::Water | Effect::Identify | Effect::Teleport | Effect::Light, false) => Neutral,
            (Effect::Void, _) => Neutral,
            (Effect::Illusion | Effect::Warding | Effect::Heal | Effect::Haste, false) => Helpful,
            (Effect::Blessing | Effect::Levitate | Effect::Mana, false) => Helpful,
            // Reversed: cold, ice, vacuum, harm, slow, fear, weakness,
            // grounding, drain and stopping teleports harm...
            (Effect::Fire | Effect::Water | Effect::Air | Effect::Heal | Effect::Haste, true) => {
                Harmful
            }
            (Effect::Calm | Effect::Blessing | Effect::Levitate | Effect::Mana, true) => Harmful,
            (Effect::Teleport | Effect::Illusion, true) => Harmful,
            // ...stoneskin, lifting curses and curing poison help...
            (Effect::Earth | Effect::Curse | Effect::Poison, true) => Helpful,
            // ...and concealing, darkness and removing every effect are neither.
            (Effect::Identify | Effect::Light | Effect::Warding, true) => Neutral,
        }
    }
}

/// When a spell in an automatic slot is cast by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutoCast {
    /// Helpful effects only: when the first step's target (the caster, or a
    /// friend) is at or below a quarter of their Health (`magic::needs_healing`).
    WhenLow,
    /// Harmful effects only: when an enemy is targeted and in reach.
    AtEnemy,
    /// Mixed, neutral or nothing: never by itself.
    ByHand,
}

/// When this spell is cast automatically, from every outcome of every step.
pub fn auto_cast(spell: &Spell) -> AutoCast {
    let aims: Vec<Aim> = spell
        .effect_steps()
        .flat_map(outcomes)
        .map(|(outcome, _)| outcome.aim())
        .collect();
    if aims.is_empty() {
        AutoCast::ByHand
    } else if aims.iter().all(|aim| *aim == Aim::Helpful) {
        AutoCast::WhenLow
    } else if aims.iter().all(|aim| *aim == Aim::Harmful) {
        AutoCast::AtEnemy
    } else {
        AutoCast::ByHand
    }
}
