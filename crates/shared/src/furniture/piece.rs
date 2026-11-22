//! One piece of furniture on a level: what it is, where, which way it faces
//! and what it is made of.

use serde::{Deserialize, Serialize};

use super::{BenchStyle, ChairStyle, Material, TableStyle};
use crate::cask_rack::CaskRack;
use crate::crafting_bench::Craft;
use crate::door::Side;

/// How much longer a move takes over a table than over open floor, in
/// thousandths: climbing over or ducking under it.
pub const TABLE_COST_PER_MILLE: u32 = 3000;

/// The same over a chair or bench.
pub const SEAT_COST_PER_MILLE: u32 = 1500;

/// What a piece is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Furniture {
    Table {
        style: TableStyle,
    },
    Chair {
        style: ChairStyle,
        arms: bool,
    },
    Bench {
        style: BenchStyle,
    },
    /// A crafting bench (`shared::crafting_bench`): a table-like piece, 1
    /// deep, back to a wall, holding its craft's tools on a raised shelf
    /// along its back.
    CraftingBench {
        craft: Craft,
    },
    /// A rack, cradle or stand for casks of one size (`shared::cask_rack`),
    /// 1 deep against a wall, facing away from it as a bench does.
    CaskRack {
        rack: CaskRack,
    },
}

impl Furniture {
    pub fn is_table(self) -> bool {
        matches!(self, Self::Table { .. })
    }

    /// How much longer a move over it takes, in thousandths of open floor's.
    pub fn cost_per_mille(self) -> u32 {
        match self {
            Self::Table { .. } | Self::CraftingBench { .. } | Self::CaskRack { .. } => {
                TABLE_COST_PER_MILLE
            }
            Self::Chair { .. } | Self::Bench { .. } => SEAT_COST_PER_MILLE,
        }
    }
}

/// A piece of furniture covering a rectangle of cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    /// Its north-west cell.
    pub at: (i64, i64),
    /// Cells east–west and north–south (a table 1 × 1 to 2 × 3 either way
    /// round, a chair 1 × 1, a bench 1 to 3 long and 1 deep, a crafting
    /// bench its craft's length and 1 deep, a cask rack its length and 1
    /// deep).
    pub size: (u8, u8),
    /// A seat: the way someone sitting on it faces (towards their table, or
    /// away from the wall behind). A table: the way its length runs from its
    /// head, east or south (east for a square one). A crafting bench: the way
    /// its front faces, away from the wall behind it (as a seat's); its shelf
    /// is along the opposite side. A cask rack: as a crafting bench.
    pub facing: Side,
    pub material: Material,
    pub furniture: Furniture,
}

impl Piece {
    /// The cells it covers.
    pub fn cells(&self) -> impl Iterator<Item = (i64, i64)> + use<> {
        let (x, y) = self.at;
        let (width, height) = (i64::from(self.size.0), i64::from(self.size.1));
        (y..y + height).flat_map(move |cy| (x..x + width).map(move |cx| (cx, cy)))
    }

    /// Whether it covers `(x, y)`.
    pub fn covers(&self, (x, y): (i64, i64)) -> bool {
        let (width, height) = (i64::from(self.size.0), i64::from(self.size.1));
        (self.at.0..self.at.0 + width).contains(&x) && (self.at.1..self.at.1 + height).contains(&y)
    }

    /// The same piece moved by `(dx, dy)` cells.
    pub fn moved(self, (dx, dy): (i64, i64)) -> Self {
        Self {
            at: (self.at.0 + dx, self.at.1 + dy),
            ..self
        }
    }
}
