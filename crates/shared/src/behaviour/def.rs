//! `BehaviourDef`: one race's behaviour, as `behaviours.ron` and mods write it.

use serde::{Deserialize, Serialize};

use super::{BandVariant, Relation, Temper};
use crate::creature::CreatureKind;

/// How strongly a race is driven to each thing, 0 to 100: what its choices
/// are scored by (mob behaviour plan PR 2 on).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Drives {
    /// Going for those it dislikes.
    pub aggression: u8,
    /// Standing its ground when hurt or outnumbered.
    pub courage: u8,
    /// Going to see what a noise was.
    pub curiosity: u8,
    /// Roaming when nothing else is going on.
    pub wander: u8,
    /// Keeping near its band.
    pub cohesion: u8,
    /// Keeping others off its ground.
    pub guarding: u8,
    /// Going for wanted things on the ground (`desires`).
    pub desire: u8,
}

impl Drives {
    /// Each drive with its name, for checking and showing.
    pub fn named(self) -> [(&'static str, u8); 7] {
        [
            ("aggression", self.aggression),
            ("courage", self.courage),
            ("curiosity", self.curiosity),
            ("wander", self.wander),
            ("cohesion", self.cohesion),
            ("guarding", self.guarding),
            ("desire", self.desire),
        ]
    }
}

/// Who it joins a fight for when they are harmed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Defends {
    /// Nobody but itself.
    Nobody,
    /// Its own band.
    #[default]
    Band,
    /// Its band and its band's allies.
    Allies,
}

/// The call it gives when hurt or alarmed, for its band to hear.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alarm {
    /// What the call is (lower case, digits and `_`): "howl", "bleat", "shout".
    pub call: String,
    /// How far it carries, in cells, for normal hearing in the open.
    pub loudness: u16,
}

/// How it runs away.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Flee {
    #[default]
    Straight,
    /// Dodging from side to side, as prey animals do.
    Zigzag,
}

/// Whether fences and gates stop it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fences {
    /// Over, under or through a gate, as it likes.
    #[default]
    Through,
    /// Nearly never (wolves).
    Seldom,
    /// Never.
    Never,
}

/// Things on the ground it goes for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Desire {
    Food,
    Coins,
    Weapons,
}

/// How one race behaves. Every field but `kind`, `temper` and `drives` may be
/// left out, taking its default.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehaviourDef {
    /// The race: its key, which a mod's entry replaces.
    pub kind: CreatureKind,
    pub temper: Temper,
    pub drives: Drives,
    #[serde(default)]
    pub defends: Defends,
    #[serde(default)]
    pub alarm: Option<Alarm>,
    /// The races it hunts as prey, wild ones only.
    #[serde(default)]
    pub hunts: Vec<CreatureKind>,
    #[serde(default)]
    pub flee: Flee,
    #[serde(default)]
    pub fences: Fences,
    /// Whether it keeps away from open fires (campfires, lit fireplaces).
    #[serde(default)]
    pub fears_fire: bool,
    #[serde(default)]
    pub desires: Vec<Desire>,
    /// How it takes to other bands of its own race.
    #[serde(default = "neutral")]
    pub other_bands: Relation,
    /// The kinds of band it has, rolled per band by weight.
    #[serde(default = "common")]
    pub bands: Vec<BandVariant>,
}

fn neutral() -> Relation {
    Relation::Neutral
}

fn common() -> Vec<BandVariant> {
    vec![BandVariant::common()]
}

/// Whether a key is lower case letters, digits and `_`, and not empty.
fn is_key(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

impl BehaviourDef {
    /// What is wrong with it, for a mod's author.
    pub fn check(&self) -> Result<(), String> {
        let race = self.kind;
        for (name, value) in self.drives.named() {
            if value > 100 {
                return Err(format!("{race:?}: {name} is {value}, more than 100"));
            }
        }
        if let Some(alarm) = &self.alarm
            && !is_key(&alarm.call)
        {
            return Err(format!(
                "{race:?}: the alarm call {:?} isn't a key",
                alarm.call
            ));
        }
        if self.hunts.contains(&race) {
            return Err(format!("{race:?} hunts its own race"));
        }
        if self.bands.is_empty() {
            return Err(format!("{race:?} has no band variants"));
        }
        for (at, variant) in self.bands.iter().enumerate() {
            let key = &variant.key;
            if !is_key(key) {
                return Err(format!("{race:?}: the band variant {key:?} isn't a key"));
            }
            if self.bands[..at].iter().any(|before| before.key == *key) {
                return Err(format!("{race:?}: two band variants are {key:?}"));
            }
            if variant.weight == 0 {
                return Err(format!("{race:?}: the band variant {key:?} has no weight"));
            }
            let lists = [&variant.allies, &variant.rivals, &variant.enemies];
            for (one, list) in lists.iter().enumerate() {
                for other in &lists[one + 1..] {
                    if let Some(both) = list.iter().find(|kind| other.contains(kind)) {
                        return Err(format!("{race:?}: {key:?} lists {both:?} twice"));
                    }
                }
            }
        }
        Ok(())
    }
}
