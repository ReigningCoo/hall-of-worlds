//! One piece of furniture on a level: what it is, where, which way it faces
//! and what it is made of.

use serde::{Deserialize, Serialize};

use super::{Barrel, Bed, BenchStyle, Candles, ChairStyle, Crates, Material, Sacks, TableStyle};
use crate::bar::BarCounter;
use crate::board_game::BoardGame;
use crate::bottle_rack::{BottleRack, BottleRackKind};
use crate::cask_rack::CaskRack;
use crate::chest::{Chest, Wardrobe};
use crate::crafting_bench::Craft;
use crate::door::Side;
use crate::fireplace::Fireplace;

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
    /// A wine rack (full height, 1 deep against a wall, facing away from it)
    /// or a spirit rack (standing on a table, facing the way its bottles are
    /// seen from): `shared::bottle_rack`.
    BottleRack {
        rack: BottleRack,
    },
    /// A bar counter or its flap (`shared::bar`), 1 deep, facing the way
    /// it is served across (towards the drinkers, the barkeep behind).
    BarCounter {
        counter: BarCounter,
    },
    /// A stone fireplace against a wall (`shared::fireplace`), 1 deep,
    /// facing away from the wall (its chimney breast) as a bench does.
    Fireplace {
        fireplace: Fireplace,
    },
    /// Candles standing on a table or bar counter (tavern furniture plan PR
    /// 5), 1 × 1, of wood, always lit: a fixed light
    /// (`decor::CANDLE_LIGHT_CELLS`).
    Candles {
        candles: Candles,
    },
    /// A 1 × 1 wooden table whose top is a board for `game` (tavern furniture
    /// plan PR 6), its men drawn where they start; facing the way the light
    /// player looks across it. Seats and candles go with it as a table's.
    GamesTable {
        game: BoardGame,
    },
    /// A barrel standing on end as a 1 × 1 table: stools round it, candles
    /// on it.
    Barrel {
        barrel: Barrel,
    },
    /// Crates stacked on one cell, in the way.
    Crates {
        crates: Crates,
    },
    /// Sacks on one cell, in the way.
    Sacks {
        sacks: Sacks,
    },
    /// A wooden bed (tavern furniture plan PR 7), `Bed::LENGTH` long and its
    /// width across, facing from its head (against a wall) to its foot; its
    /// model smaller than its cells.
    Bed {
        bed: Bed,
    },
    /// A wooden chest (`shared::chest`), its length and 1 deep, facing the
    /// way its lid opens towards, away from the wall.
    Chest {
        chest: Chest,
    },
    /// A wooden wardrobe (`shared::chest`), its length and 1 deep against a
    /// wall, facing away from it.
    Wardrobe {
        wardrobe: Wardrobe,
    },
}

impl Furniture {
    pub fn is_table(self) -> bool {
        matches!(self, Self::Table { .. })
    }

    /// Whether it stands on other furniture instead of the floor: a spirit
    /// rack or candles on a table or a bar counter. Such a piece is kept on a
    /// level apart from what it stands on (`dungeon::Level::furniture_on_top`).
    pub fn stands_on_furniture(self) -> bool {
        match self {
            Self::BottleRack { rack } => rack.kind == BottleRackKind::Spirit,
            Self::Candles { .. } => true,
            _ => false,
        }
    }

    /// Whether a piece standing on furniture may stand on this: a table, a
    /// games table, a standing barrel or a bar counter (not its flap).
    pub fn holds_furniture(self) -> bool {
        match self {
            Self::BarCounter { counter } => counter.is_counter(),
            Self::GamesTable { .. } | Self::Barrel { .. } => true,
            _ => self.is_table(),
        }
    }

    /// How much longer a move over it takes, in thousandths of open floor's.
    pub fn cost_per_mille(self) -> u32 {
        match self {
            Self::Table { .. }
            | Self::CraftingBench { .. }
            | Self::CaskRack { .. }
            | Self::BottleRack { .. }
            | Self::Fireplace { .. }
            | Self::Candles { .. }
            | Self::GamesTable { .. }
            | Self::Barrel { .. }
            | Self::Crates { .. }
            | Self::Bed { .. }
            | Self::Chest { .. }
            | Self::Wardrobe { .. } => TABLE_COST_PER_MILLE,
            // A counter as a table's; its flap lifts, as open floor.
            Self::BarCounter { counter } if counter.is_counter() => TABLE_COST_PER_MILLE,
            Self::BarCounter { .. } => 1000,
            Self::Chair { .. } | Self::Bench { .. } | Self::Sacks { .. } => SEAT_COST_PER_MILLE,
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
    /// bench its craft's length and 1 deep, a cask rack, bottle rack or bar
    /// counter its length and 1 deep, a fireplace its length and 1 deep, a
    /// games table, barrel, crates or sacks 1 × 1, a bed 2 × 1 or 2 × 2, a
    /// chest or wardrobe its length and 1 deep).
    pub size: (u8, u8),
    /// A seat: the way someone sitting on it faces (towards their table, or
    /// away from the wall behind). A table: the way its length runs from its
    /// head, east or south (east for a square one). A crafting bench: the way
    /// its front faces, away from the wall behind it (as a seat's); its shelf
    /// is along the opposite side. A cask rack or wine rack: as a crafting
    /// bench. A spirit rack: the way its front (lowest shelf) faces. Candles:
    /// the way a pair stands side by side across. A bar
    /// counter: the side drinkers stand on, the barkeep behind it. A
    /// fireplace: as a crafting bench, its hearth stone to the front. A games
    /// table: the way the light player looks across the board. A bed: from
    /// its head to its foot. A chest or wardrobe: as a crafting bench. A barrel,
    /// crates or sacks: only which way the model turns.
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
