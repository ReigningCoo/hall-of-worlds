//! What shapes a piece may have: the sizes of each kind, which way each may
//! face, and stone styles of stone. The furnishing step only makes such
//! pieces; the server checks Place Furniture's (dev tools) by the same rules.

use super::{BenchStyle, ChairStyle, Furniture, Piece, TableStyle};
use crate::door::Side;

impl Furniture {
    /// Whether its style is a stone one (made of stone, not wood).
    pub fn is_stone(self) -> bool {
        match self {
            Self::Table { style } => TableStyle::STONE.contains(&style),
            Self::Chair { style, .. } => style == ChairStyle::StoneBlock,
            Self::Bench { style } => style == BenchStyle::StoneSlab,
            // Hearths are of stone, other benches of wood.
            Self::CraftingBench { craft } => craft.is_hearth(),
            Self::CaskRack { .. }
            | Self::BottleRack { .. }
            | Self::BarCounter { .. }
            | Self::Candles { .. }
            | Self::GamesTable { .. }
            | Self::Barrel { .. }
            | Self::Crates { .. }
            | Self::Sacks { .. } => false,
            Self::Fireplace { .. } => true,
        }
    }
}

/// The cells east–west and north–south a piece covers, from its length and
/// depth (long side first) and which way it faces: a table's length runs the
/// way it faces (east or south), a seat's across the way its sitter looks.
pub fn footprint(furniture: Furniture, (long, short): (u8, u8), facing: Side) -> (u8, u8) {
    let along_x = match furniture {
        Furniture::Table { .. } => facing == Side::East,
        Furniture::Chair { .. }
        | Furniture::Bench { .. }
        | Furniture::CraftingBench { .. }
        | Furniture::CaskRack { .. }
        | Furniture::BottleRack { .. }
        | Furniture::BarCounter { .. }
        | Furniture::Fireplace { .. }
        | Furniture::Candles { .. }
        | Furniture::GamesTable { .. }
        | Furniture::Barrel { .. }
        | Furniture::Crates { .. }
        | Furniture::Sacks { .. } => matches!(facing, Side::North | Side::South),
    };
    if along_x {
        (long, short)
    } else {
        (short, long)
    }
}

impl Piece {
    /// Whether it is a piece the furnishing step could make: a table 1 × 1 to
    /// 2 × 3 facing east or south (east when square) along its length, a
    /// pedestal only where it fits; a chair 1 × 1; a bench 1 to 3 long and 1
    /// deep across the way it faces; a crafting bench its craft's length and 1
    /// deep across the way it faces, of wood (a cask, wine or spirit rack or a
    /// bar counter its length, of wood; a fireplace its length, of stone;
    /// candles, a games table, barrel, crates or sacks 1 × 1, of wood); stone styles of stone and wooden
    /// ones of wood, and arms only on wooden chairs, never a bar stool.
    pub fn is_well_made(&self) -> bool {
        let (width, height) = self.size;
        let (long, short) = (width.max(height), width.min(height));
        let sized = match self.furniture {
            Furniture::Table { style } => {
                // East or south, and east when square.
                let facing =
                    self.facing == Side::East || self.facing == Side::South && long > short;
                (1..=3).contains(&long)
                    && (1..=2).contains(&short)
                    && facing
                    && style.fits(self.size)
            }
            Furniture::Chair { style, arms } => {
                long == 1 && !(arms && (self.material.is_stone() || style == ChairStyle::BarStool))
            }
            Furniture::Bench { .. } => (1..=3).contains(&long) && short == 1,
            Furniture::CraftingBench { craft } => long == craft.length() && short == 1,
            Furniture::CaskRack { rack } => {
                rack.is_well_made() && long == rack.length && short == 1
            }
            Furniture::BottleRack { rack } => {
                rack.is_well_made() && long == rack.length && short == 1
            }
            Furniture::BarCounter { counter } => {
                counter.is_well_made() && long == counter.length && short == 1
            }
            Furniture::Fireplace { fireplace } => {
                fireplace.is_well_made() && long == fireplace.length && short == 1
            }
            Furniture::Candles { .. }
            | Furniture::GamesTable { .. }
            | Furniture::Barrel { .. }
            | Furniture::Crates { .. }
            | Furniture::Sacks { .. } => long == 1,
        };
        sized
            && footprint(self.furniture, (long, short), self.facing) == self.size
            && self.furniture.is_stone() == self.material.is_stone()
    }
}

#[cfg(test)]
mod tests;
