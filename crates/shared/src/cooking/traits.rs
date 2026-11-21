//! The words an ingredient's culinary data is written in (`IngredientDef`):
//! where it comes from, its roles in a dish, how it is used and cooked, its
//! texture, what it does to a dish and how it tastes.

use serde::{Deserialize, Serialize};

/// Which table an ingredient's key is in. Meat, milk, eggs and honey are keyed
/// by the animal (`shared::livestock`'s keys), parts by the part (`parts.ron`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Source {
    Herb,
    Fruit,
    Vegetable,
    Fungus,
    Meat,
    Milk,
    Egg,
    Honey,
    Part,
    /// Kitchen staples made rather than grown, by key (`PANTRY`): salt,
    /// soy sauce (cooking plan PR 12).
    Pantry,
}

/// What an ingredient is in a dish. A dish is classified by its tool steps
/// and the mix of these (`docs/context/cooking.md`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Role {
    Starch,
    Protein,
    Vegetable,
    Fat,
    Sweet,
    Acid,
    Nut,
    Aromatic,
    Herb,
    Spice,
}

/// How much it is eaten, most to least.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Usage {
    Staple,
    Common,
    Occasional,
    Rare,
    /// Taken as a medicine, not as food: in a dish only as a bitter seasoning.
    MedicinalOnly,
    Inedible,
}

/// Whether it must be cooked to be eaten.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CookingNeed {
    /// Harmful or inedible raw.
    MustCook,
    BetterCooked,
    Either,
    RawBest,
    /// Not cooked at all (spices ground, inedible things).
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Texture {
    Soft,
    Juicy,
    Leafy,
    Firm,
    Chewy,
    Tough,
    Hard,
    Dry,
}

/// What it does to the rest of a dish.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Function {
    Thickens,
    Binds,
    Sets,
    MakesDough,
    Sweetens,
    Sours,
    Colours,
    Tenderises,
    Curdles,
}

/// The ways it is usually prepared.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Method {
    Raw,
    Cut,
    Grind,
    Mash,
    Boil,
    Stew,
    Steam,
    Fry,
    Roast,
    Bake,
    Infuse,
    Dry,
    Pickle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Flavour {
    Mild,
    Sweet,
    Sour,
    Bitter,
    Savoury,
    Earthy,
    Nutty,
    Fruity,
    Grassy,
    Floral,
    Citrus,
    Minty,
    Aniseed,
    Resinous,
    Pungent,
    Peppery,
    Fiery,
    Smoky,
}
