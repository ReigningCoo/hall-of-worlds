//! What things are made of. A first table for trying this out: the owner means to
//! flesh out materials later, with money values and more categories (fabric, wood).
//! See `docs/context/shared/item.md`.

use serde::{Deserialize, Serialize};

/// What sort of thing a material is, which decides what it can be used for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Bone,
    Gem,
    /// Hides: soft, hardened (cuir bouilli) or thick buff.
    Leather,
    Metal,
    /// Plant stuff other than timber: rattan, cork, straw, grass.
    Plant,
    /// Cloth and the fibres it is woven from: linen, hemp and ramie from plants,
    /// wool and haircloth (goat or horse hair) from animals, silk, cotton, felt.
    Textile,
    Wood,
}

/// A material. Saved by name, so new ones can go anywhere in the list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Material {
    Acacia,
    Adamantine,
    Alder,
    Amethyst,
    Apple,
    Aquamarine,
    Ash,
    Aspen,
    Avocado,
    Baobab,
    Beech,
    Birch,
    Bone,
    Brass,
    BrazilNut,
    Bronze,
    BuffLeather,
    Cashew,
    Cedar,
    Cherry,
    Chestnut,
    Chitin,
    Citrine,
    Citrus,
    Coir,
    Copper,
    Cork,
    Cotton,
    Cypress,
    Diamond,
    DouglasFir,
    DragonScale,
    Durian,
    Ebony,
    Elm,
    Emerald,
    Felt,
    Fig,
    Fir,
    Fur,
    Garnet,
    Gold,
    /// Grass and leaves, hung from a band (the grass skirt).
    Grass,
    Guava,
    Haircloth,
    HardenedLeather,
    Hemp,
    Hickory,
    HolmOak,
    Horn,
    Iron,
    Jackwood,
    Juniper,
    Kapok,
    Kola,
    Larch,
    Leather,
    Linden,
    Linen,
    Lychee,
    Macadamia,
    Mahogany,
    Mango,
    Maple,
    Marula,
    Mesquite,
    Mithril,
    Mopane,
    Mulberry,
    Oak,
    Olive,
    Onyx,
    Palm,
    Pear,
    Peridot,
    Persimmon,
    Pili,
    Pine,
    Pistachio,
    Plum,
    Ramie,
    Rattan,
    Rawhide,
    RedCedar,
    Ruby,
    Sandalwood,
    Sapphire,
    Shea,
    Silk,
    Silver,
    Spruce,
    Steel,
    Straw,
    Suede,
    Tamarind,
    Teak,
    Terminalia,
    Thorn,
    Topaz,
    Walnut,
    Whitewood,
    Willow,
    Wood,
    Wool,
}

impl Material {
    pub fn category(self) -> Category {
        match self {
            Self::Bone | Self::Chitin | Self::DragonScale | Self::Horn => Category::Bone,
            Self::BuffLeather
            | Self::Fur
            | Self::HardenedLeather
            | Self::Leather
            | Self::Rawhide
            | Self::Suede => Category::Leather,
            Self::Adamantine
            | Self::Brass
            | Self::Bronze
            | Self::Copper
            | Self::Gold
            | Self::Iron
            | Self::Mithril
            | Self::Silver
            | Self::Steel => Category::Metal,
            Self::Coir | Self::Cork | Self::Grass | Self::Rattan | Self::Straw => Category::Plant,
            Self::Cotton
            | Self::Felt
            | Self::Haircloth
            | Self::Hemp
            | Self::Linen
            | Self::Ramie
            | Self::Silk
            | Self::Wool => Category::Textile,
            Self::Wood => Category::Wood,
            _ if self.wood().is_some() => Category::Wood,
            _ => Category::Gem,
        }
    }
}

mod all;
mod name;
mod properties;
mod wood;

pub use wood::{WOODS, Wood};

#[cfg(test)]
mod tests;
