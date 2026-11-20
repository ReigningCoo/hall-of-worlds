//! `Produce`: what a living animal gives, at game rates (owner, decision 3:
//! per game day, at about half of modern breeds' yields).

use serde::{Deserialize, Serialize};

/// What is given. Each is counted in its own unit (`Product::unit`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Product {
    Milk,
    Wool,
    Fleece,
    AngoraWool,
    Egg,
    Honey,
    Cocoons,
}

impl Product {
    /// The unit `Produce::amount` is in.
    pub const fn unit(self) -> &'static str {
        match self {
            Self::Milk => "ml",
            Self::Wool | Self::Fleece | Self::AngoraWool | Self::Honey => "g",
            Self::Egg => "eggs",
            Self::Cocoons => "trays",
        }
    }
}

/// When it is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Per {
    /// Each game day (with `Produce::chance`).
    Day,
    /// Each shearing (how often: later, with shearing).
    Shearing,
    /// Each game day in the flowering months (a hive's honey).
    FloweringDay,
    /// Once, when grown (a rearing tray's cocoons).
    Grown,
}

/// One thing a living animal gives.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Produce {
    pub product: Product,
    /// How much, in `Product::unit`.
    pub amount: u32,
    pub per: Per,
    /// The percent chance each time: an egg "most game days" or "every 3 game
    /// days". 100 when written without.
    #[serde(default = "always")]
    pub chance: u8,
    /// What one egg weighs, in grams; 0 for anything else.
    #[serde(default)]
    pub grams: u32,
    /// When it isn't given every day: milk only after giving birth, eggs only
    /// in spring.
    #[serde(default)]
    pub note: String,
}

fn always() -> u8 {
    100
}

impl Produce {
    pub(super) fn check(&self) -> Result<(), String> {
        if self.amount == 0 || !(1..=100).contains(&self.chance) {
            return Err(format!("{:?}: none given", self.product));
        }
        if (self.product == Product::Egg) != (self.grams > 0) {
            return Err(format!("{:?}: only an egg has a weight", self.product));
        }
        Ok(())
    }
}
