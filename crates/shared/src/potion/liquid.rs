//! What a full bottle holds: a `Liquid`, a water, an oil or a potion (brewing
//! plan, owner, 2026-11-17: `docs/context/brewing.md`), and `LiquidKind`, the
//! same without a potion's contents, for asking for one.

use serde::{Deserialize, Serialize};

use super::Contents;

/// A water, as the server decides where a bottle is filled. What each does to
/// a brew comes with brewing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Water {
    /// Anywhere else: changes nothing.
    Plain,
    /// Underground: strengthens harmful doses.
    Cave,
    /// Outside with no sun up and a moon: strengthens helpful doses.
    Moon,
    /// From the sea: takes away the strongest helpful dose.
    Salt,
    /// Caught from the rain, outside: takes away the strongest harmful dose
    /// (Alchemy plan, owner, 2026-11-19).
    Rain,
    // New waters go at the end: messages carry one by its place in this list.
}

impl Water {
    pub const ALL: [Water; 5] = [Self::Plain, Self::Cave, Self::Moon, Self::Salt, Self::Rain];

    /// "Water", "Cave Water", "Moon Water", "Salt Water" or "Rain Water".
    pub fn name(self) -> &'static str {
        match self {
            Self::Plain => "Water",
            Self::Cave => "Cave Water",
            Self::Moon => "Moon Water",
            Self::Salt => "Salt Water",
            Self::Rain => "Rain Water",
        }
    }
}

/// An oil, pressed from vegetables, grain, nuts or seeds; kept generic. Each
/// but Neutral suits one row of the attributes: Grain Body, Nut Mind, Seed
/// Spirit (Alchemy plan, owner, 2026-11-19).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Oil {
    /// From vegetables other than grain and seeds: no effect at all. Saves
    /// holding the old Vegetable Oil load as this (same place, so messages
    /// too).
    #[serde(alias = "Vegetable")]
    Neutral,
    Nut,
    Seed,
    /// From grain: boosts the Body row.
    Grain,
    // New oils go at the end: messages carry one by its place in this list.
}

impl Oil {
    pub const ALL: [Oil; 4] = [Self::Neutral, Self::Grain, Self::Nut, Self::Seed];

    /// "Neutral Oil", "Grain Oil", "Nut Oil" or "Seed Oil".
    pub fn name(self) -> &'static str {
        match self {
            Self::Neutral => "Neutral Oil",
            Self::Grain => "Grain Oil",
            Self::Nut => "Nut Oil",
            Self::Seed => "Seed Oil",
        }
    }
}

/// What a full bottle holds. A potion's contents are never empty: an empty
/// bottle holds no liquid at all (`BottleForm::liquid` is `None`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Liquid {
    Water(Water),
    Oil(Oil),
    Potion(Contents),
    /// A fruit's juice, by the fruit's key (`shared::juice`, drinks plan PR 3).
    Juice(String),
    /// A fruit's juice and as much water or less (`shared::juice`'s `mix`).
    Cordial(String),
    /// A fermented drink: ale, cider, perry, wine (`shared::fermentation`,
    /// drinks plan PR 6). Boxed, so every item doesn't grow by it.
    Drink(Box<crate::fermentation::Drink>),
    /// What an opened drink turns into once past its best (drinks decision
    /// 13), by what the drink was made from. Boxed as `Drink` is.
    Vinegar(Box<crate::fermentation::Base>),
    /// Pure alcohol, distilled in the Alembic (`shared::alchemy`, Alchemy plan
    /// PR 5): a Mortar's solvent for a tincture, or fed to an Alembic.
    Alcohol,
    /// An animal's milk, by `livestock`'s animal key (`shared::farmyard`,
    /// cooking plan PR 3).
    Milk(String),
    /// Honey, by the animal that made it (`livestock`'s key: `honeybee`).
    Honey(String),
    /// Soy sauce, pressed from moromi (`shared::food`, cooking plan PR 12).
    SoySauce,
    // New liquids go at the end: messages carry one by its place in this list.
}

impl Liquid {
    /// Its kind, without a potion's contents.
    pub fn kind(&self) -> LiquidKind {
        match self {
            Self::Water(water) => LiquidKind::Water(*water),
            Self::Oil(oil) => LiquidKind::Oil(*oil),
            Self::Potion(_) => LiquidKind::Potion,
            Self::Juice(_) => LiquidKind::Juice,
            Self::Cordial(_) => LiquidKind::Cordial,
            Self::Drink(_) => LiquidKind::Drink,
            Self::Vinegar(_) => LiquidKind::Vinegar,
            Self::Alcohol => LiquidKind::Alcohol,
            Self::Milk(_) => LiquidKind::Milk,
            Self::Honey(_) => LiquidKind::Honey,
            Self::SoySauce => LiquidKind::SoySauce,
        }
    }

    /// How much alcohol it holds, in tenths of a percent: a drink's strength,
    /// pure alcohol's 1000, nothing else any (vinegar's has turned).
    pub fn alcohol_strength(&self) -> u32 {
        match self {
            Self::Drink(drink) => drink.strength,
            Self::Alcohol => PURE_STRENGTH,
            _ => 0,
        }
    }

    /// A juice's or cordial's fruit, by key.
    pub fn fruit(&self) -> Option<&str> {
        match self {
            Self::Juice(fruit) | Self::Cordial(fruit) => Some(fruit),
            _ => None,
        }
    }

    /// A potion's contents; none for a water or an oil.
    pub fn contents(&self) -> Option<&Contents> {
        match self {
            Self::Potion(contents) => Some(contents),
            _ => None,
        }
    }

    /// What a bottle of it is called: "Water", "Nut Oil", "Potion", "Apple
    /// Juice", "Apple Cordial" (just "Juice" for a fruit the table lacks).
    pub fn name(&self) -> String {
        match self {
            Self::Drink(drink) => return drink.name(),
            Self::Vinegar(base) => return base.vinegar_name().to_owned(),
            Self::Milk(animal) => return crate::farmyard::milk_name(animal),
            Self::Honey(animal) => return crate::farmyard::honey_name(animal),
            _ => {}
        }
        let kind = self.kind().name();
        let fruit = self
            .fruit()
            .and_then(|key| crate::fruit::Fruits::built_in().get(key));
        match fruit {
            Some(def) => format!("{} {kind}", def.name),
            None => kind.to_owned(),
        }
    }
}

/// Pure alcohol's strength in tenths of a percent.
pub const PURE_STRENGTH: u32 = 1000;

/// A kind of liquid, a potion without its contents: what Give Item asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiquidKind {
    Water(Water),
    Oil(Oil),
    Potion,
    /// Some fruit's juice: Give Item rolls the fruit.
    Juice,
    /// Some fruit's cordial.
    Cordial,
    /// A fermented drink: Give Item rolls a fruit's.
    Drink,
    /// Some vinegar.
    Vinegar,
    /// Pure alcohol.
    Alcohol,
    /// Some animal's milk: Give Item rolls the animal.
    Milk,
    /// Honey.
    Honey,
    /// Soy sauce.
    SoySauce,
}

impl LiquidKind {
    /// Every kind: the waters, the oils, a potion, juice and cordial.
    pub fn all() -> Vec<LiquidKind> {
        let waters = Water::ALL.map(Self::Water);
        let oils = Oil::ALL.map(Self::Oil);
        waters
            .into_iter()
            .chain(oils)
            .chain([
                Self::Potion,
                Self::Juice,
                Self::Cordial,
                Self::Drink,
                Self::Vinegar,
                Self::Alcohol,
                Self::Milk,
                Self::Honey,
                Self::SoySauce,
            ])
            .collect()
    }

    /// "Water", "Seed Oil", "Potion".
    pub fn name(self) -> &'static str {
        match self {
            Self::Water(water) => water.name(),
            Self::Oil(oil) => oil.name(),
            Self::Potion => "Potion",
            Self::Juice => "Juice",
            Self::Cordial => "Cordial",
            Self::Drink => "Drink",
            Self::Vinegar => "Vinegar",
            Self::Alcohol => "Pure Alcohol",
            Self::Milk => "Milk",
            Self::Honey => "Honey",
            Self::SoySauce => "Soy Sauce",
        }
    }
}
