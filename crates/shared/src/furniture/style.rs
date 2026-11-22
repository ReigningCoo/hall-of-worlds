//! How a piece is built, per kind of piece (only for its model). A room's seats
//! match its table: each table style has the seat styles that go with it.

use serde::{Deserialize, Serialize};

/// How a table is built.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TableStyle {
    /// Boards on trestles.
    Trestle,
    /// Four legs joined by stretchers.
    Joined,
    /// A board on two upright planks.
    PlankLegs,
    /// One central column (only 1 × 1 and 1 × 2).
    Pedestal,
    /// Stone: a slab on blocks.
    StoneSlab,
    /// Stone: one solid block.
    StoneBlock,
}

/// How a chair is built.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ChairStyle {
    Stool,
    BackStool,
    LadderBack,
    PanelBack,
    XFrame,
    /// Stone: a block to sit on.
    StoneBlock,
    /// A tall stool with a foot ring, for along a bar counter (tavern
    /// furniture plan PR 3); no table's chairs.
    BarStool,
}

/// How a bench is built.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BenchStyle {
    /// A plank on splayed legs.
    SplayedPlank,
    /// A seat between two end boards.
    BoardEnded,
    /// A long stool with a rail between its legs.
    LongStool,
    /// Stone: a slab on blocks.
    StoneSlab,
    /// A high-backed settle (tavern furniture plan PR 5): a seat boxed in by
    /// solid ends, its back as tall as a person sitting, the walls of a
    /// booth. Never among a table's benches, so furnishing never makes one.
    Settle,
}

impl TableStyle {
    pub const ALL: [Self; 6] = [
        Self::Trestle,
        Self::Joined,
        Self::PlankLegs,
        Self::Pedestal,
        Self::StoneSlab,
        Self::StoneBlock,
    ];
    pub const WOOD: [Self; 4] = [Self::Trestle, Self::Joined, Self::PlankLegs, Self::Pedestal];
    pub const STONE: [Self; 2] = [Self::StoneSlab, Self::StoneBlock];

    /// Whether a table of `size` (east–west, north–south) can be built so: a
    /// pedestal only 1 × 1 or 1 × 2.
    pub fn fits(self, (width, height): (u8, u8)) -> bool {
        self != Self::Pedestal || width.min(height) == 1 && width.max(height) <= 2
    }

    /// The chairs that go with it.
    pub fn chairs(self) -> &'static [ChairStyle] {
        use ChairStyle::*;
        match self {
            Self::Trestle => &[Stool, BackStool],
            Self::Joined => &[LadderBack, PanelBack, BackStool],
            Self::PlankLegs => &[Stool, LadderBack],
            Self::Pedestal => &[PanelBack, XFrame, LadderBack],
            Self::StoneSlab | Self::StoneBlock => &[StoneBlock],
        }
    }

    /// The benches that go with it.
    pub fn benches(self) -> &'static [BenchStyle] {
        use BenchStyle::*;
        match self {
            Self::Trestle => &[SplayedPlank, LongStool],
            Self::Joined => &[BoardEnded, LongStool],
            Self::PlankLegs => &[SplayedPlank, BoardEnded],
            Self::Pedestal => &[BoardEnded],
            Self::StoneSlab | Self::StoneBlock => &[StoneSlab],
        }
    }
}

impl ChairStyle {
    pub const ALL: [Self; 7] = [
        Self::Stool,
        Self::BackStool,
        Self::LadderBack,
        Self::PanelBack,
        Self::XFrame,
        Self::StoneBlock,
        Self::BarStool,
    ];
}

impl BenchStyle {
    pub const ALL: [Self; 5] = [
        Self::SplayedPlank,
        Self::BoardEnded,
        Self::LongStool,
        Self::StoneSlab,
        Self::Settle,
    ];
}
