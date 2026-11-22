//! `BottleRack`: a wine rack or a spirit rack, how long it is, and where each
//! of its bottles stands or lies.

use serde::{Deserialize, Serialize};

/// Which kind of bottle rack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BottleRackKind {
    /// Full height against a wall: bottles lying in a diamond lattice.
    Wine,
    /// Three stepped shelves of upright bottles, standing on a table.
    Spirit,
}

/// A bottle rack `length` cells long (1 to 3), 1 deep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BottleRack {
    pub kind: BottleRackKind,
    pub length: u8,
}

/// Where one of a rack's bottles is: its middle `along` the rack in twelfths
/// of a cell from its start, and its `row` from the bottom (a wine rack's
/// lattice row; a spirit rack's shelf, 0 the lowest, at the front).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottlePlace {
    pub along: u8,
    pub row: u8,
}

/// How many rows of bottles a wine rack's lattice has (Claude's first guess).
pub const WINE_ROWS: u8 = 8;
/// How many shelves a spirit rack has (owner's: three, stepped).
pub const SPIRIT_SHELVES: u8 = 3;

impl BottleRackKind {
    pub const ALL: [Self; 2] = [Self::Wine, Self::Spirit];

    /// "Wine Rack" or "Spirit Rack".
    pub fn name(self) -> &'static str {
        match self {
            Self::Wine => "Wine Rack",
            Self::Spirit => "Spirit Rack",
        }
    }
}

impl BottleRack {
    /// The lengths every rack comes in.
    pub const LENGTHS: [u8; 3] = [1, 2, 3];

    /// Every rack there is: each kind at each length.
    pub fn all() -> Vec<Self> {
        (BottleRackKind::ALL.into_iter())
            .flat_map(|kind| Self::LENGTHS.map(|length| Self { kind, length }))
            .collect()
    }

    /// Whether it is a rack there is (1 to 3 long).
    pub fn is_well_made(self) -> bool {
        Self::LENGTHS.contains(&self.length)
    }

    /// "Wine Rack" or "Spirit Rack".
    pub fn name(self) -> &'static str {
        self.kind.name()
    }

    /// Where each bottle is, in the order they are filled: the bottom row
    /// first, from the rack's start. A wine rack's even rows hold 3 bottles a
    /// cell and its odd rows one fewer in all, each in the gap between those
    /// below and above, a diamond lattice (1 × 1 holds 20, 1 × 2 44, 1 × 3
    /// 68); a spirit rack's shelves 3 a cell (9, 18, 27).
    pub fn places(self) -> Vec<BottlePlace> {
        let length = self.length;
        let kind = self.kind;
        let rows = match kind {
            BottleRackKind::Wine => WINE_ROWS,
            BottleRackKind::Spirit => SPIRIT_SHELVES,
        };
        (0..rows)
            .flat_map(|row| {
                let between = kind == BottleRackKind::Wine && row % 2 == 1;
                let count = 3 * length - u8::from(between);
                let first = if between { 4 } else { 2 };
                (0..count).map(move |j| BottlePlace {
                    along: first + 4 * j,
                    row,
                })
            })
            .collect()
    }

    /// How many bottles it holds.
    pub fn capacity(self) -> usize {
        self.places().len()
    }
}
