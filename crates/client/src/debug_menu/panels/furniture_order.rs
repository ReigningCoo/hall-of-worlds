//! What Place Furniture puts down (no Bevy systems, so it can be tested
//! alone): the panel's choices, and the piece a click makes. Clicking the same
//! cell again with the same choices gives the next variation (every style of
//! the kind that fits (a fireplace has one, of stone), chairs without and then with arms, a cask rack's every
//! cask size made at that length, a bar counter single, double, then a flap
//! at length 1, candles each way they stand, a games table's each game,
//! crates then sacks each way they stack: `variations.rs`), wrapping round; a
//! click elsewhere starts a new piece in the variation the last one ended on.
//! Booths, wall decorations, chandeliers and rugs are put down by
//! `decor.rs`.

use bevy::prelude::*;
use shared::door::Side;
use shared::furniture::{Material, Piece, footprint};

mod decor;
mod variations;

pub(super) use decor::Order;

/// Which kind of piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Table,
    Chair,
    Bench,
    /// A cask rack, cradle or stand (`shared::cask_rack`), of wood.
    CaskRack,
    /// A wine rack against a wall (`shared::bottle_rack`), of wood.
    WineRack,
    /// A spirit rack, of wood, only on a table (the server refuses it
    /// elsewhere).
    SpiritRack,
    /// A bar counter (single or double) or its flap (`shared::bar`), of
    /// wood.
    BarCounter,
    /// A fireplace against a wall (`shared::fireplace`), of stone.
    Fireplace,
    /// Candles, only on a table or bar counter (`furniture::Candles`).
    Candles,
    /// A 1 × 1 table with a chess or Hnefatafl board, of wood.
    GamesTable,
    /// A barrel standing as a table (`furniture::Barrel`), of wood.
    Barrel,
    /// Crates, then sacks (`furniture::Crates`, `Sacks`).
    Clutter,
    /// Two settles facing across a table (`furniture::booth`), of wood.
    Booth,
    /// Something hanging on a wall's face (`shared::decor::Hanging`).
    WallDecoration,
    /// A chandelier over the cell (`shared::decor::ChandelierStyle`).
    Chandelier,
    /// A rug on the floor (`shared::decor::Rug`).
    Rug,
}

impl Kind {
    pub(super) const ALL: [Self; 16] = [
        Self::Table,
        Self::Chair,
        Self::Bench,
        Self::CaskRack,
        Self::WineRack,
        Self::SpiritRack,
        Self::BarCounter,
        Self::Fireplace,
        Self::Candles,
        Self::GamesTable,
        Self::Barrel,
        Self::Clutter,
        Self::Booth,
        Self::WallDecoration,
        Self::Chandelier,
        Self::Rug,
    ];

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Table => "Table",
            Self::Chair => "Chair",
            Self::Bench => "Bench",
            Self::CaskRack => "Cask Rack",
            Self::WineRack => "Wine Rack",
            Self::SpiritRack => "Spirit Rack",
            Self::BarCounter => "Bar Counter",
            Self::Fireplace => "Fireplace",
            Self::Candles => "Candles",
            Self::GamesTable => "Games Table",
            Self::Barrel => "Barrel Table",
            Self::Clutter => "Crates and Sacks",
            Self::Booth => "Booth",
            Self::WallDecoration => "Wall Decoration",
            Self::Chandelier => "Chandelier",
            Self::Rug => "Rug",
        }
    }

    /// The sizes it comes in, long side by short.
    pub(super) fn sizes(self) -> &'static [(u8, u8)] {
        match self {
            Self::Table | Self::Booth => &[(1, 1), (2, 1), (3, 1), (2, 2), (3, 2)],
            Self::Chair
            | Self::Candles
            | Self::GamesTable
            | Self::Barrel
            | Self::Clutter
            | Self::WallDecoration
            | Self::Chandelier => &[(1, 1)],
            Self::Rug => &[(1, 1), (2, 1), (3, 2), (4, 3), (4, 4)],
            Self::Bench
            | Self::CaskRack
            | Self::WineRack
            | Self::SpiritRack
            | Self::BarCounter
            | Self::Fireplace => &[(1, 1), (2, 1), (3, 1)],
        }
    }

    /// A size as the dropdown says it: short side first, as "2 × 3".
    pub(super) fn size_name(self, (long, short): (u8, u8)) -> String {
        match self {
            Self::Bench
            | Self::CaskRack
            | Self::WineRack
            | Self::SpiritRack
            | Self::BarCounter
            | Self::Fireplace => long.to_string(),
            _ => format!("{short} × {long}"),
        }
    }
}

/// The panel's choices: kind, size, facing and material.
type Choices = (Kind, usize, Side, Option<Material>);

/// What the panel has chosen, and the last piece put down.
#[derive(Resource, Clone, Debug, PartialEq)]
pub(super) struct FurnitureOrder {
    pub(super) kind: Kind,
    /// Its place in `kind.sizes()`.
    pub(super) size: usize,
    pub(super) facing: Side,
    /// `None` for Random: rolled each click, of the style's kind.
    pub(super) material: Option<Material>,
    /// The cell last clicked and the choices then, to know a click again.
    last: Option<((i64, i64), Choices)>,
    /// Which variation the last piece was.
    variation: usize,
}

impl Default for FurnitureOrder {
    fn default() -> Self {
        Self {
            kind: Kind::Table,
            size: 4,
            facing: Side::East,
            material: None,
            last: None,
            variation: 0,
        }
    }
}

impl FurnitureOrder {
    /// The variation (of `count`) a click on `cell` puts down: the next one
    /// on clicking the same cell with the same choices.
    fn vary(&mut self, cell: (i64, i64), count: usize) -> usize {
        let choices = (self.kind, self.size, self.facing, self.material);
        let again = self.last == Some((cell, choices));
        self.variation = (self.variation + usize::from(again)) % count.max(1);
        self.last = Some((cell, choices));
        self.variation
    }

    /// The piece a click on `cell` puts down (its north-west cell), `roll`
    /// choosing a Random material.
    pub(super) fn click(&mut self, cell: (i64, i64), roll: u64) -> Piece {
        let sizes = self.kind.sizes();
        let (long, short) = sizes[self.size.min(sizes.len() - 1)];
        let variations = self.kind.variations((long, short), self.material);
        let furniture = variations[self.vary(cell, variations.len())];
        let facing = match self.kind {
            // A table runs east–west or north–south, east when square.
            Kind::Table if long > short && matches!(self.facing, Side::North | Side::South) => {
                Side::South
            }
            Kind::Table => Side::East,
            Kind::Chair
            | Kind::Bench
            | Kind::CaskRack
            | Kind::WineRack
            | Kind::SpiritRack
            | Kind::BarCounter
            | Kind::Fireplace
            | Kind::Candles
            | Kind::GamesTable
            | Kind::Barrel
            | Kind::Clutter
            | Kind::Booth
            | Kind::WallDecoration
            | Kind::Chandelier
            | Kind::Rug => self.facing,
        };
        let chosen = self
            .material
            .filter(|m| m.is_stone() == furniture.is_stone());
        let material = chosen.unwrap_or_else(|| {
            let of_kind = Material::ALL.into_iter();
            let of_kind: Vec<Material> = of_kind
                .filter(|m| m.is_stone() == furniture.is_stone())
                .collect();
            of_kind[(roll % of_kind.len() as u64) as usize]
        });
        Piece {
            at: cell,
            size: footprint(furniture, (long, short), facing),
            facing,
            material,
            furniture,
        }
    }
}

#[cfg(test)]
mod tests;
