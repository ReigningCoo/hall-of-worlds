//! `BarCounter`: a single or double bar counter or a hinged flap, how long it
//! is, and where each of its vessels stands.

use serde::{Deserialize, Serialize};

/// Which kind of bar piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BarKind {
    /// A counter with a shelf underneath for its vessels.
    Single,
    /// A counter with a raised shelf above for its vessels, and room
    /// underneath for the barkeep to serve.
    Double,
    /// A hinged flap, 1 long, lifted to get behind the bar: holds nothing,
    /// and is walked through as open floor.
    Flap,
}

/// A bar piece `length` cells long (1 to 3; a flap 1), 1 deep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BarCounter {
    pub kind: BarKind,
    pub length: u8,
}

/// Where one of a counter's vessels stands: its middle `along` the counter in
/// twenty-fourths of a cell from its start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarPlace {
    pub along: u8,
}

/// How many vessels a cell of counter holds (Claude's first guess).
pub const VESSELS_PER_CELL: u8 = 4;

impl BarKind {
    pub const ALL: [Self; 3] = [Self::Single, Self::Double, Self::Flap];

    /// "Bar Counter", "Double Bar Counter" or "Bar Flap".
    pub fn name(self) -> &'static str {
        match self {
            Self::Single => "Bar Counter",
            Self::Double => "Double Bar Counter",
            Self::Flap => "Bar Flap",
        }
    }
}

impl BarCounter {
    /// The lengths counters come in (a flap only 1).
    pub const LENGTHS: [u8; 3] = [1, 2, 3];

    /// Every bar piece there is: each counter at each length, and the flap.
    pub fn all() -> Vec<Self> {
        let counters = [BarKind::Single, BarKind::Double].into_iter();
        let counters = counters.flat_map(|kind| Self::LENGTHS.map(|length| Self { kind, length }));
        let flap = Self {
            kind: BarKind::Flap,
            length: 1,
        };
        counters.chain([flap]).collect()
    }

    /// Whether it is a piece there is.
    pub fn is_well_made(self) -> bool {
        match self.kind {
            BarKind::Flap => self.length == 1,
            BarKind::Single | BarKind::Double => Self::LENGTHS.contains(&self.length),
        }
    }

    /// Its name: "Bar Counter", "Double Bar Counter", "Bar Flap".
    pub fn name(self) -> &'static str {
        self.kind.name()
    }

    /// Whether it is a counter (not the flap): something stands on it, and
    /// it holds vessels.
    pub fn is_counter(self) -> bool {
        self.kind != BarKind::Flap
    }

    /// Where each vessel stands, in the order they are filled, from the
    /// counter's start: `VESSELS_PER_CELL` a cell, evenly; none on a flap.
    pub fn places(self) -> Vec<BarPlace> {
        if !self.is_counter() {
            return Vec::new();
        }
        let count = VESSELS_PER_CELL * self.length;
        let step = 24 / VESSELS_PER_CELL;
        (0..count)
            .map(|j| BarPlace {
                along: step / 2 + step * j,
            })
            .collect()
    }

    /// How many vessels it holds.
    pub fn capacity(self) -> usize {
        self.places().len()
    }
}
