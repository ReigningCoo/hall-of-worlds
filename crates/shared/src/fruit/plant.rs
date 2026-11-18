//! A fruit plant: one cell where a fruit grows (owner, 2026-11-17), and the
//! stage it has grown to at any time. Plants grow by the world's clock on
//! their own: young for a round, then round after round of leaves, flowers and
//! fruit (unripe, then ripe), for `LIFE_ROUNDS` rounds, when a young one takes
//! its place. Each starts somewhere of its own in that life (`phase`), and
//! picking it starts a round again from its leaves, so only the time it was
//! last picked needs keeping.

use serde::{Deserialize, Serialize};

use super::FruitDef;
use crate::herb::MINUTES_PER_DAY;

/// How many rounds of fruit a plant bears before a young one takes its place.
pub const LIFE_ROUNDS: u64 = 8;

/// The stage a fruit plant is at. Fruiting is the last half of each round:
/// unripe, then ripe for its last quarter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FruitStage {
    Young,
    Leafy,
    Flowering,
    Fruiting { ripe: bool },
}

impl FruitStage {
    /// Its number, 1 to 4 (fruiting either way is 4).
    pub const fn number(self) -> u8 {
        match self {
            Self::Young => 1,
            Self::Leafy => 2,
            Self::Flowering => 3,
            Self::Fruiting { .. } => 4,
        }
    }
}

/// One cell's fruit plant.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FruitPlant {
    /// The fruit's key (`FruitDef::key`).
    pub fruit: String,
    /// Where it starts in its life, in game minutes (any number: taken round
    /// its life's length).
    pub phase: u32,
    /// The game minute it was last picked, if ever.
    #[serde(default)]
    pub picked: Option<u64>,
}

/// Game minutes in one round of leaves, flowers and fruit for `def`.
pub fn round_minutes(def: &FruitDef) -> u64 {
    u64::from(def.growth().days()) * MINUTES_PER_DAY
}

impl FruitPlant {
    /// The stage it has grown to at game minute `now`.
    pub fn stage(&self, def: &FruitDef, now: u64) -> FruitStage {
        let round = round_minutes(def);
        let into_round = match self.picked {
            // Picked: its round starts again from its leaves.
            Some(picked) => now.saturating_sub(picked) % round,
            None => {
                let life = round * (1 + LIFE_ROUNDS);
                let at = (now + u64::from(self.phase)) % life;
                if at < round {
                    return FruitStage::Young;
                }
                (at - round) % round
            }
        };
        match 4 * into_round / round {
            0 => FruitStage::Leafy,
            1 => FruitStage::Flowering,
            2 => FruitStage::Fruiting { ripe: false },
            _ => FruitStage::Fruiting { ripe: true },
        }
    }

    /// Whether it can be picked at game minute `now`: only when ripe.
    pub fn can_pick(&self, def: &FruitDef, now: u64) -> bool {
        self.stage(def, now) == FruitStage::Fruiting { ripe: true }
    }
}
