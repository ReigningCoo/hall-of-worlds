//! `ItemKind`: every kind of item. What each is, is the table in `def.rs`.

use serde::{Deserialize, Serialize};

use crate::blade::Family;

/// A kind of item. Saved by name, so new ones can go anywhere in the list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemKind {
    /// A fruit or nut (`shared::fruit`), counted: which one is in its form.
    /// Saved apples load as fruit, given the apple's form (`item::Saved`).
    #[serde(alias = "Apple")]
    Fruit,
    /// A vegetable, pulse or grain (`shared::vegetable`), counted: which one
    /// is in its form.
    Vegetable,
    /// A mushroom, fungus or lichen (`shared::fungus`), counted: which one is
    /// in its form.
    Fungus,
    /// A sack of flour milled from a grain (`shared::flour`), counted: which
    /// grain is in its form.
    Flour,
    /// A log of a tree's wood, felled (`shared::woodcutting`): which wood is in
    /// its form.
    Log,
    /// Kindling split from a log (`shared::woodcutting`), counted: which wood
    /// is in its form.
    Kindling,
    /// A yeast (`shared::yeast`): each its own item, with its own clock.
    Yeast,
    Bread,
    DwarvenBread,
    RawMeat,
    ArmArmour,
    ArmingCap,
    ArmingVest,
    ArmourCoat,
    Aventail,
    Axe,
    Backpack,
    Bag,
    BagOfHolding,
    Barding,
    Belt,
    BevoredHelm,
    Bevor,
    Boomerang,
    Boots,
    /// A glass bottle, empty or a potion (`shared::potion`), counted: what it
    /// holds is in its form.
    Bottle,
    /// A wooden cask holding a liquid by the litre (`shared::cask`): its
    /// size, wood and what is in it are in its form.
    Cask,
    /// A brass tap, knocked into a cask to draw from it (`shared::pour`).
    Tap,
    /// A drinking vessel or a jug, holding a liquid by the millilitre
    /// (`shared::vessel`): which one and what is in it are in its form.
    Vessel,
    /// An iron pot, kept for cooking later (no longer an alchemy tool).
    Cauldron,
    Caparison,
    Chausses,
    Cloak,
    Clogs,
    Coif,
    Collar,
    /// Saved items of the old `Coin` load as copper.
    #[serde(alias = "Coin")]
    CopperCoin,
    GoldCoin,
    SilverCoin,
    Corslet,
    CuirassierTassets,
    Dagger,
    DemiGauntlets,
    /// Old fingerless gloves: no form, no longer made (gloves are rolled fingerless).
    FingerlessGloves,
    FootArmour,
    FullArmour,
    Gambeson,
    Gauntlets,
    Gem,
    Gloves,
    Gorget,
    HalfArmour,
    Hat,
    Helmet,
    /// A herb (`shared::herb`), counted: which one is in its form.
    Herb,
    /// A holder for herbs on the belt (`bag_def.rs`).
    HerbPouch,
    /// Something someone lost, found for a quest: a letter, a book, a toy
    /// (owner: one picture for all, named for what it is).
    Keepsake,
    Key,
    Keyring,
    Knife,
    Lantern,
    LegArmour,
    Longsword,
    Mantle,
    /// The Mortar apparatus (`shared::alchemy`): herbs ground.
    MortarAndPestle,
    Necklace,
    /// Herbs and an oil ground together (`shared::brewing`), counted: its
    /// doses are in its form.
    Ointment,
    /// A small screw press for oil (`ability::Ability::PressOil`).
    OilPress,
    PaddedChausses,
    PaddedGloves,
    /// A quest's delivery: wrapped in brown paper and tied with string (owner).
    Parcel,
    /// Herbs ground dry (`shared::alchemy`), counted: doses in its form.
    Paste,
    Polearm,
    /// A part of a creature brought back for a quest ("kobold scales"),
    /// dropped only for the quester (owner).
    Remains,
    /// The Retort apparatus (`shared::alchemy`): liquids infused.
    Retort,
    /// The Alembic apparatus (`shared::alchemy`): liquids distilled.
    Alembic,
    /// The Crucible apparatus (`shared::alchemy`): anything calcined into a
    /// Powder.
    Crucible,
    /// The Filter apparatus (`shared::alchemy`): filtered through a Powder.
    Filter,
    /// The Pelican apparatus (`shared::alchemy`): circulation, raising a
    /// product's purity.
    Pelican,
    /// A Crucible's Powder (`shared::alchemy`), counted like a Paste.
    Powder,
    /// What distilling Salt Water leaves (`shared::alchemy`), counted: a
    /// Retort solid leaning a mixture to the Body row.
    Salt,
    /// A Retort solid (`shared::alchemy`) leaning a mixture to the Mind row;
    /// sold only, counted.
    Sulphur,
    /// Quicksilver in a little stoppered phial: a Retort solid
    /// (`shared::alchemy`) leaning a mixture to the Spirit row; sold only,
    /// counted.
    Mercury,
    Pouch,
    Ring,
    Sandals,
    Shoes,
    ShortSword,
    ShoulderArmour,
    Slippers,
    SmallBox,
    Socks,
    Spear,
    Stone,
    Surcoat,
    Sword,
    TShirt,
    Tabard,
    TorsoArmour,
    Trousers,
    WaistArmour,
    // Underclothes (`underclothes`), Skin layer.
    Braies,
    BreastBand,
    Hose,
    LinenCoif,
    Loincloth,
    Petticoat,
    Shirt,
    Wimple,
    // Clothes with forms (`clothing`), Middle layer.
    Bodice,
    Doublet,
    Robe,
    Tunic,
    Vest,
    // Trousers, Belt, Gloves and Hat (above) have forms too.
    Skirt,
    Hood,
    Cap,
    Headcloth,
    Headdress,
    Neckwear,
    Shawl,
}

impl ItemKind {
    /// Whether it can be made at other sizes: what is worn, gems and blades. Everything
    /// else is always `Medium`, whoever carries it.
    pub const fn comes_in_sizes(self) -> bool {
        self.def().wear.is_some() || matches!(self, Self::Gem) || self.blade_family().is_some()
    }

    /// The family of weapon this makes, for those rolled as blades.
    pub const fn blade_family(self) -> Option<Family> {
        match self {
            Self::Knife => Some(Family::Knife),
            Self::Dagger => Some(Family::Dagger),
            Self::ShortSword => Some(Family::ShortSword),
            Self::Sword => Some(Family::Sword),
            Self::Longsword => Some(Family::LongSword),
            Self::Axe => Some(Family::Axe),
            Self::Spear => Some(Family::Spear),
            Self::Polearm => Some(Family::Polearm),
            _ => None,
        }
    }
}
