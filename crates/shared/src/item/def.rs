//! What each kind of item is: its name, weight, cells, and how it is carried or worn.

use super::item_def::{HELD, plain, wear};
use super::{ItemDef, ItemKind, Shape, WearDef};
use crate::creature::{Layer, Location};

/// All but its top-right cell: an L.
const BOOMERANG: Shape = Shape::rectangle(2, 2).without(1 << 1);

impl ItemKind {
    pub const fn def(self) -> ItemDef {
        use Layer::{Middle, Outer, Over, Skin};
        use Location::{
            Arms, Back, Feet, Fingers, Forelegs, Hands, Head, Hindlegs, Legs, Neck, Shoulders,
            Torso, Waist,
        };
        let square = Shape::rectangle(1, 1);
        let def =
            match self {
                // Collars, gorgets, bevors, aventails and mantles: in `neck_def.rs`.
                Self::Aventail | Self::Bevor | Self::Collar | Self::Gorget | Self::Mantle => {
                    self.neck_def()
                }
                Self::Keepsake | Self::Parcel | Self::Remains => self.quest_def(), // `quest_def.rs`
                // Armour sets: in `set_def.rs`.
                Self::ArmourCoat | Self::Corslet | Self::FullArmour | Self::HalfArmour => {
                    self.set_def()
                }
                // Shoes, boots, sandals, clogs and slippers: in `feet_def.rs`.
                Self::Boots | Self::Clogs | Self::Sandals | Self::Shoes | Self::Slippers => {
                    self.feet_def()
                }
                Self::Fruit
                | Self::Vegetable
                | Self::Flour
                | Self::Yeast
                | Self::Bread
                | Self::DwarvenBread
                | Self::RawMeat => self.food_def(),
                Self::Log | Self::Kindling => self.wood_def(), // `wood_def.rs`
                // Bags, boxes and backpacks: in `bag_def.rs`.
                Self::Backpack
                | Self::Bag
                | Self::BagOfHolding
                | Self::HerbPouch
                | Self::Keyring
                | Self::Pouch
                | Self::SmallBox => self.bag_def(),
                // Both arms, or one; its form says its name, cells and weight, and whether
                // it is padded (kote).
                Self::ArmArmour => plain("Arm Armour", 2000, Shape::rectangle(2, 3)).worn(wear(
                    Outer,
                    &[Arms],
                    (0, 0),
                    false,
                )),
                // The rest of the head pieces' looks, cells and weights are their forms'.
                Self::ArmingCap => plain("Arming Cap", 250, Shape::rectangle(2, 1)).worn(wear(
                    Skin,
                    &[Head],
                    (1, 1),
                    true,
                )),
                // Under torso armour; lined armour takes its place (`Item::padded`).
                Self::ArmingVest => plain("Arming Vest", 1200, Shape::rectangle(2, 2)).worn(wear(
                    Middle,
                    &[Torso],
                    (1, 0),
                    false,
                )),
                // A blade's weight and shape are its form's: these are only for one without.
                Self::Axe => plain("Axe", 900, Shape::rectangle(1, 3)).worn(HELD),
                Self::Barding => plain("Barding", 15_000, Shape::rectangle(4, 4)).worn(wear(
                    Outer,
                    &[Neck, Torso, Forelegs, Hindlegs],
                    (0, 0),
                    false,
                )),
                // Its own plates guard the throat, as a gorget would.
                Self::BevoredHelm => plain("Bevored Helm", 4000, Shape::rectangle(2, 3))
                    .worn(wear(Outer, &[Head, Neck], (0, 0), false)),
                Self::Boomerang => plain("Boomerang", 400, BOOMERANG),
                // Cloth over barding, to below the knees, which doesn't count as the legs.
                Self::Caparison => plain("Caparison", 4000, Shape::rectangle(3, 3)).worn(wear(
                    Over,
                    &[Head, Neck, Torso],
                    (0, 1),
                    false,
                )),
                // Mail chausses with feet, over the shoes; its form says the rest.
                Self::Chausses => plain("Chausses", 3800, Shape::rectangle(2, 3)).worn(wear(
                    Outer,
                    &[Legs, Feet],
                    (1, 0),
                    false,
                )),
                Self::Cloak => plain("Cloak", 900, Shape::rectangle(2, 3)).worn(WearDef {
                    one_place: true,
                    ..wear(Outer, &[Back], (1, 1), true)
                }),
                Self::Coif => plain("Coif", 2000, Shape::rectangle(2, 2)).worn(wear(
                    Middle,
                    &[Head, Neck],
                    (1, 0),
                    false,
                )),
                // 100 of one make one of the next (`coin.rs`), held as a number.
                Self::CopperCoin => plain("Copper Coin", 1, square).without_quality().counted(),
                Self::SilverCoin => plain("Silver Coin", 3, square).without_quality().counted(),
                Self::GoldCoin => plain("Gold Coin", 5, square).without_quality().counted(),
                Self::Key => plain("Key", 30, square).without_quality(),
                // Counted with quality: a stack keys on its contents and quality
                // (`shared::potion`). What one weighs is its form's, full or empty.
                Self::Bottle => plain("Bottle", crate::potion::EMPTY_GRAMS, square).counted(),
                // Counted with quality: a stack keys on its herb and quality
                // (`shared::herb`). What one weighs is its form's, from its herb.
                Self::Herb => plain("Herb", 10, square).counted(),
                // Counted with quality: a stack keys on its contents and quality
                // (`shared::brewing`).
                Self::Ointment => {
                    plain("Ointment", crate::brewing::OINTMENT_GRAMS, square).counted()
                }
                // Laminated to the knee, wrapping the thighs: Waist and Legs (the owner's
                // decision). Its form says its cells and weight.
                Self::CuirassierTassets => plain(
                    "Cuirassier Tassets",
                    4000,
                    Shape::rectangle(2, 4),
                )
                .worn(wear(Outer, &[Waist, Legs], (0, 0), false)),
                Self::Dagger => plain("Dagger", 400, Shape::rectangle(1, 2)).worn(HELD),
                // Sabatons, mail shoes and foot guards, over shoes or boots; its form says
                // its name, cells and weight.
                Self::FootArmour => plain("Foot Armour", 800, Shape::rectangle(2, 2)).worn(wear(
                    Outer,
                    &[Feet],
                    (0, 0),
                    false,
                )),
                // Over the back of the hand and the wrist only, so rings stay free.
                Self::DemiGauntlets => plain("Demi-gauntlets", 450, Shape::rectangle(2, 1))
                    .worn(wear(Outer, &[Hands], (0, 0), false)),
                // The old kind, kept so saves load: gloves are now rolled fingerless.
                Self::FingerlessGloves => plain("Fingerless Gloves", 80, Shape::rectangle(1, 2))
                    .worn(wear(Middle, &[Hands], (1, 1), true)),
                // Over the shoulders too (the owner's decision), under pauldrons.
                Self::Gambeson => plain("Gambeson", 2500, Shape::rectangle(2, 3)).worn(wear(
                    Middle,
                    &[Shoulders, Torso, Arms],
                    (1, 0),
                    false,
                )),
                Self::Gauntlets => plain("Gauntlets", 1200, Shape::rectangle(2, 1)).worn(wear(
                    Outer,
                    &[Hands, Fingers],
                    (0, 0),
                    false,
                )),
                Self::Gem => plain("Gem", 5, square),
                Self::Helmet => plain("Helmet", 2000, Shape::rectangle(2, 2)).worn(wear(
                    Outer,
                    &[Head],
                    (0, 0),
                    false,
                )),
                Self::Knife => plain("Knife", 200, Shape::rectangle(1, 2)).worn(HELD),
                Self::Lantern => plain("Lantern", 800, Shape::rectangle(1, 2)),
                // Brewing's tools (`ability::Ability::tool`): weights first guesses.
                Self::OilPress => plain("Oil Press", 6000, Shape::rectangle(2, 2)),
                Self::MortarAndPestle => plain("Mortar and Pestle", 1500, square),
                Self::Cauldron => plain("Cauldron", 4000, Shape::rectangle(2, 2)),
                // Its form says its size, wood, cells and weight (`shared::cask`).
                Self::Cask => plain("Cask", 14_000, Shape::rectangle(3, 3)),
                Self::Tap => plain("Tap", 300, square),
                // Its form says which vessel, its cells and weight (`shared::vessel`).
                Self::Vessel => plain("Vessel", 450, square),
                // Both legs, or a single Classical greave; its form says its name, cells
                // and weight.
                Self::LegArmour => plain("Leg Armour", 3000, Shape::rectangle(2, 3)).worn(wear(
                    Outer,
                    &[Legs],
                    (0, 0),
                    false,
                )),
                Self::Longsword => plain("Longsword", 2000, Shape::rectangle(1, 6)).worn(HELD),
                Self::Necklace => {
                    plain("Necklace", 40, square).worn(wear(Skin, &[Neck], (5, 5), true))
                }
                // Quilted leggings under mail and plate, in place of trousers.
                Self::PaddedChausses => plain("Padded Chausses", 1200, Shape::rectangle(2, 3))
                    .worn(wear(Middle, &[Legs], (1, 0), false)),
                // Under gauntlets.
                Self::PaddedGloves => plain("Padded Gloves", 150, Shape::rectangle(2, 1))
                    .worn(wear(Middle, &[Hands, Fingers], (1, 0), false)),
                Self::Polearm => plain("Polearm", 3000, Shape::rectangle(1, 8)).worn(HELD),
                Self::Ring => plain("Ring", 10, square).worn(WearDef {
                    one_place: true,
                    ..wear(Skin, &[Fingers], (1, 1), true)
                }),
                Self::ShortSword => plain("Short Sword", 900, Shape::rectangle(1, 4)).worn(HELD),
                // A pair: its form says its kind, name, cells and weight.
                Self::ShoulderArmour => plain("Shoulder Armour", 2600, Shape::rectangle(3, 2))
                    .worn(wear(Outer, &[Shoulders], (0, 0), false)),
                Self::Spear => plain("Spear", 1500, Shape::rectangle(1, 8)).worn(HELD),
                Self::Stone => plain("Stone", 5000, Shape::rectangle(2, 2)).without_quality(),
                // Belted over armour, to the knee, which doesn't count as the legs.
                Self::Surcoat => plain("Surcoat", 800, Shape::rectangle(2, 3)).worn(wear(
                    Over,
                    &[Torso, Waist],
                    (1, 1),
                    true,
                )),
                Self::Sword => plain("Sword", 1400, Shape::rectangle(1, 5)).worn(HELD),
                // Open at the sides, so it fits more than one size.
                Self::Tabard => plain("Tabard", 500, Shape::rectangle(2, 3)).worn(wear(
                    Over,
                    &[Shoulders, Torso],
                    (1, 1),
                    true,
                )),
                // Its form says its name, cells and weight, and whether it is lined.
                Self::TorsoArmour => plain("Torso Armour", 5000, Shape::rectangle(3, 3))
                    .worn(wear(Outer, &[Torso], (0, 0), false)),
                // Faulds, tassets, skirts and the rest; its form says its name, cells and
                // weight. Never padded: Waist's Middle is the belt's.
                Self::WaistArmour => plain("Waist Armour", 2500, Shape::rectangle(2, 2))
                    .worn(wear(Outer, &[Waist], (0, 0), false)),
                // Tunics, robes, doublets, bodices, vests, trousers, skirts, belts, gloves,
                // hoods, caps, headcloths, headdresses, neckwear, shawls and hats: in
                // `clothing_def.rs`.
                _ if self.is_clothing() => self.clothing_def(),
                // Underclothes, and the old T-shirt: in `under_def.rs`.
                _ => self.under_def(),
            };
        // First guesses at how much work each is beside the usual.
        def.taking(match self {
            Self::Fruit
            | Self::Vegetable
            | Self::Flour
            | Self::Yeast
            | Self::Bread
            | Self::DwarvenBread
            | Self::RawMeat
            | Self::Stone => 10,
            _ if self.coin_value().is_some() => 10,
            Self::Ring
            | Self::Pouch
            | Self::Bag
            | Self::Belt
            | Self::Boomerang
            | Self::Keyring
            | Self::HerbPouch => 75,
            Self::Tabard => 50,
            _ if self.is_under() || self.is_clothing() => 50,
            Self::Gauntlets
            | Self::Helmet
            | Self::BevoredHelm
            | Self::Coif
            | Self::DemiGauntlets
            | Self::ShoulderArmour
            | Self::TorsoArmour
            | Self::ArmArmour
            | Self::LegArmour
            | Self::Chausses
            | Self::FootArmour
            | Self::Collar
            | Self::Gorget
            | Self::Bevor
            | Self::Aventail
            | Self::Mantle
            | Self::WaistArmour
            | Self::CuirassierTassets
            | Self::Backpack
            | Self::Necklace => 150,
            Self::Sword
            | Self::Axe
            | Self::Dagger
            | Self::Knife
            | Self::Longsword
            | Self::Polearm
            | Self::ShortSword
            | Self::Spear => 200,
            Self::FullArmour | Self::HalfArmour | Self::Barding => 400,
            Self::ArmourCoat | Self::Corslet => 300,
            _ => 100,
        })
    }
}
