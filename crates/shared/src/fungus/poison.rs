//! What a poisonous fungus does (owner, 2026-11-19, decision 8): how bad, how
//! soon, its effects and how it harms. For now eating one raw harms at once
//! (`eating`); later onsets, hidden poisons, deadly drains (fungi plan PR 6)
//! and harm from handling (PR 9) use the rest.

use serde::{Deserialize, Serialize};

use crate::herb::Effect;

/// How bad a poison is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    Mild,
    Moderate,
    Severe,
    /// Kills unless healed (fungi plan PR 6).
    Deadly,
}

/// How soon it starts, scaled to game time (owner, decision 8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Onset {
    /// At once: real ones within an hour or two.
    Soon,
    /// About 10 game minutes: real ones after a few hours.
    Delayed,
    /// A game hour or more: real ones after half a day, days or weeks.
    Late,
}

/// What brings it on.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Trigger {
    /// Eating it.
    #[default]
    Eaten,
    /// Eating an old one (puffballs gone to powder).
    Old,
    /// Eating it often, meal after meal (brown roll-rim, chaga tea).
    Repeated,
    /// Drinking alcohol within 3 days after eating it (common ink cap).
    WithAlcohol,
    /// Touching it with bare skin.
    Touched,
    /// Breathing its spores or dust.
    Breathed,
}

/// A fungus's poison.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poison {
    pub severity: Severity,
    pub onset: Onset,
    /// Sickness that eases for a while before the worst (the death cap's).
    #[serde(default)]
    pub false_recovery: bool,
    /// What it does: 1 to 4 effects, each 1 to 3 either way.
    pub effects: Vec<Effect>,
    /// Only raw: cooking makes it safe (later).
    #[serde(default)]
    pub raw_only: bool,
    #[serde(default)]
    pub trigger: Trigger,
}

impl Poison {
    /// Whether eating a fresh one raw brings it on.
    pub fn when_eaten(&self) -> bool {
        self.trigger == Trigger::Eaten
    }

    /// Whether it makes sense: an error saying what is wrong.
    pub fn check(&self) -> Result<(), String> {
        if self.effects.is_empty() || self.effects.len() > 4 {
            return Err("a poison has 1 to 4 effects".to_owned());
        }
        if self
            .effects
            .iter()
            .any(|e| !(1..=3).contains(&e.strength()))
        {
            return Err("a poison's effects are 1 to 3 either way".to_owned());
        }
        Ok(())
    }
}
