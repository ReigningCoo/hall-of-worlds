//! The Middle-layer clothes' rows of the table of kinds (`def.rs`): how each is worn.
//! Their names, cells and weights are their forms' (`clothing`); these are for items
//! made before forms. Long tunics and robes still only take the Torso and Arms: their
//! length is a property, so trousers, a skirt and a belt go with any of them (owner,
//! 2026-11-15). Skirts take the Legs, as trousers do, so the two clash.

use super::item_def::{plain, wear};
use super::{ItemDef, ItemKind, Shape};
use crate::creature::Layer::Middle;
use crate::creature::Location::{Arms, Fingers, Hands, Legs, Torso, Waist};

impl ItemKind {
    /// The clothes with forms, in `clothing::ClothingPiece`'s order.
    pub const ALL_CLOTHING: [Self; 9] = [
        Self::Tunic,
        Self::Robe,
        Self::Doublet,
        Self::Bodice,
        Self::Vest,
        Self::Trousers,
        Self::Skirt,
        Self::Belt,
        Self::Gloves,
    ];

    pub(super) const fn is_clothing(self) -> bool {
        matches!(
            self,
            Self::Tunic
                | Self::Robe
                | Self::Doublet
                | Self::Bodice
                | Self::Vest
                | Self::Trousers
                | Self::Skirt
                | Self::Belt
                | Self::Gloves
        )
    }

    pub(super) const fn clothing_def(self) -> ItemDef {
        let square = Shape::rectangle(2, 2);
        let long = Shape::rectangle(2, 3);
        let arms = &[Torso, Arms];
        match self {
            Self::Tunic => plain("Tunic", 600, long).worn(wear(Middle, arms, (1, 0), false)),
            // Wrapped and sashed, so it fits a size either way.
            Self::Robe => plain("Robe", 1200, long).worn(wear(Middle, arms, (1, 1), true)),
            Self::Doublet => plain("Doublet", 800, square).worn(wear(Middle, arms, (0, 0), false)),
            Self::Bodice => plain("Bodice", 400, square).worn(wear(Middle, arms, (1, 0), false)),
            Self::Vest => plain("Vest", 500, square).worn(wear(Middle, &[Torso], (1, 0), false)),
            Self::Trousers => {
                plain("Trousers", 500, long).worn(wear(Middle, &[Legs], (1, 0), false))
            }
            // Gathered, tied or wrapped, so it fits a size either way.
            Self::Skirt => plain("Skirt", 450, square).worn(wear(Middle, &[Legs], (1, 1), true)),
            // Every form has the belt's bag slots (`inventory/bag_slots.rs`).
            Self::Belt => plain("Belt", 200, Shape::rectangle(1, 3)).worn(wear(
                Middle,
                &[Waist],
                (2, 2),
                true,
            )),
            // Fingerless ones leave the Fingers bare (their form's `bare`).
            Self::Gloves => plain("Gloves", 100, Shape::rectangle(1, 2)).worn(wear(
                Middle,
                &[Hands, Fingers],
                (1, 0),
                false,
            )),
            _ => panic!("not clothing with a form"),
        }
    }
}
