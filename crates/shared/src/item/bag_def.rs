//! Bags' rows of the table of kinds (`def.rs`): pouches, bags, boxes and backpacks,
//! with the grids inside them. A Pouch and a Bag can also be tied to a belt
//! (`ContainerDef::on_belt`), adding their insides to the own grid; a Keyring and a
//! Herb Pouch are holders (`ContainerDef::holder`), tied with its own grid.

use super::item_def::{bag, wear};
use super::{ItemDef, ItemKind, Shape, WearDef};
use crate::creature::{Layer, Location};

/// A bag of holding's grid: as wide as the ground's, and up to this many rows.
const BAG_OF_HOLDING_WIDTH: u8 = 8;
const BAG_OF_HOLDING_ROWS: u8 = 60;
/// A holder's grid (`ContainerDef::holder`): this wide, and as many rows as a
/// grid can have, so it has no limit that matters.
const HOLDER_WIDTH: u8 = 6;
const HOLDER_ROWS: u8 = u8::MAX;

impl ItemKind {
    pub(super) const fn bag_def(self) -> ItemDef {
        match self {
            Self::Backpack => bag(
                "Backpack",
                1200,
                Shape::rectangle(3, 4),
                (10, 12),
                Some(25_000),
                None,
            )
            .worn(WearDef {
                one_place: true,
                ..wear(Layer::Outer, &[Location::Back], (1, 1), true)
            }),
            // A cloth drawstring bag: between a pouch and a backpack.
            Self::Bag => bag(
                "Bag",
                250,
                Shape::rectangle(3, 3),
                (6, 6),
                Some(6000),
                Some(2000),
            )
            .on_belt(),
            // A placeholder for trying out bags that grow: far more inside than out.
            Self::BagOfHolding => bag(
                "Bag of Holding",
                500,
                Shape::rectangle(3, 3),
                (BAG_OF_HOLDING_WIDTH, BAG_OF_HOLDING_ROWS),
                Some(100_000),
                None,
            )
            .growing(),
            // A holder for keys on the belt (owner, 2026-11-17): 6 wide, showing 4
            // rows at first and growing without limit as more goes in.
            Self::Keyring => bag(
                "Keyring",
                40,
                Shape::rectangle(2, 1),
                (HOLDER_WIDTH, HOLDER_ROWS),
                None,
                None,
            )
            .growing()
            .holding(&[ItemKind::Key]),
            // A holder for herbs on the belt (owner, 2026-11-17): like the Keyring,
            // 6 wide, showing 4 rows at first and growing without limit; herbs
            // stack in it by herb and quality.
            Self::HerbPouch => bag(
                "Herb Pouch",
                60,
                Shape::rectangle(2, 1),
                (HOLDER_WIDTH, HOLDER_ROWS),
                None,
                None,
            )
            .growing()
            .holding(&[ItemKind::Herb]),
            Self::SmallBox => bag(
                "Small Box",
                100,
                Shape::rectangle(2, 1),
                (3, 2),
                Some(300),
                None,
            ),
            _ => bag(
                "Pouch",
                80,
                Shape::rectangle(2, 2),
                (4, 4),
                Some(1500),
                Some(500),
            )
            .on_belt(),
        }
    }
}
