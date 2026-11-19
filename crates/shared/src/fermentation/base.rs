//! What a drink is made from (`Base`) and so what it is called (`DrinkKind`,
//! owner: ale from grain, cider from apples, perry from pears, wine from
//! grapes, "<fruit> wine" otherwise).

use serde::{Deserialize, Serialize};

use crate::fruit::Fruits;
use crate::vegetable::Vegetables;

/// What a drink was fermented from, by the item's key.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Base {
    /// A fruit's key (`shared::fruit`).
    Fruit(String),
    /// A grain's key (`shared::vegetable`, a `Grain`).
    Grain(String),
}

/// The kind of drink, which sets its fermenting time and how it ages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DrinkKind {
    Ale,
    Cider,
    Perry,
    Wine,
    /// Wine of any fruit but grapes, apples and pears.
    FruitWine,
}

impl DrinkKind {
    pub const ALL: [Self; 5] = [
        Self::Ale,
        Self::Cider,
        Self::Perry,
        Self::Wine,
        Self::FruitWine,
    ];

    /// What `base` ferments into.
    pub fn of(base: &Base) -> Self {
        match base {
            Base::Grain(_) => Self::Ale,
            Base::Fruit(key) => match key.as_str() {
                "apple" => Self::Cider,
                "pear" => Self::Perry,
                "grape" => Self::Wine,
                _ => Self::FruitWine,
            },
        }
    }
}

impl Base {
    pub fn kind(&self) -> DrinkKind {
        DrinkKind::of(self)
    }

    /// The drink's name in English: "Cider", "Perry", "Wine", "Cherry Wine",
    /// "Ale" from barley and "Wheat Ale" from any other grain. A key not in
    /// the tables keeps the plain name of its kind.
    pub fn name(&self) -> String {
        match (self.kind(), self) {
            (DrinkKind::Cider, _) => "Cider".into(),
            (DrinkKind::Perry, _) => "Perry".into(),
            (DrinkKind::Wine, _) => "Wine".into(),
            (_, Base::Grain(key)) if key == "barley" => "Ale".into(),
            (_, Base::Grain(key)) => match Vegetables::built_in().get(key) {
                Some(def) => format!("{} Ale", def.name),
                None => "Ale".into(),
            },
            (_, Base::Fruit(key)) => match Fruits::built_in().get(key) {
                Some(def) => format!("{} Wine", def.name),
                None => "Wine".into(),
            },
        }
    }
}
