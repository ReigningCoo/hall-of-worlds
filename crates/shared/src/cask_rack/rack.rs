//! `CaskRack`: which cask size a piece is made for and how long it is; what
//! kind of piece that makes, how many casks it holds and where each lies.

use serde::{Deserialize, Serialize};

use crate::cask::CaskSize;

/// A rack, cradle or stand for casks of one size, `length` cells along the
/// wall (1 to 3; a Tun stand 2), 1 deep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaskRack {
    pub size: CaskSize,
    pub length: u8,
}

/// What kind of piece a `CaskRack` is, from its size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RackKind {
    /// Small casks in a pyramid `layers` high, two a cell on the bottom.
    Rack { layers: u8 },
    /// One cask a cell, each on a cradle of its own.
    Cradle,
    /// One Tun lying along the wall, installed.
    Stand,
}

/// Where one of a piece's casks lies: its middle `along` the wall in
/// quarters of a cell from the piece's start, and its `layer` (0 the bottom).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RackPlace {
    pub along: u8,
    pub layer: u8,
}

impl RackKind {
    /// The kind of piece made for casks of `size` (owner's: Pins and Firkins
    /// three layers, Kilderkins two, Barrels to Butts cradles, a Tun a stand).
    pub fn of(size: CaskSize) -> Self {
        match size {
            CaskSize::Pin | CaskSize::Firkin => Self::Rack { layers: 3 },
            CaskSize::Kilderkin => Self::Rack { layers: 2 },
            CaskSize::Tun => Self::Stand,
            _ => Self::Cradle,
        }
    }
}

impl CaskRack {
    /// The lengths a piece for `size` comes in: 1 to 3 cells, a Tun stand 2.
    pub fn lengths(size: CaskSize) -> &'static [u8] {
        match size {
            CaskSize::Tun => &[2],
            _ => &[1, 2, 3],
        }
    }

    /// Every piece there is: each size at each of its lengths.
    pub fn all() -> Vec<Self> {
        (CaskSize::ALL.into_iter())
            .flat_map(|size| (Self::lengths(size).iter()).map(move |&length| Self { size, length }))
            .collect()
    }

    pub fn kind(self) -> RackKind {
        RackKind::of(self.size)
    }

    /// Whether it is a piece there is (its length one its size comes in).
    pub fn is_well_made(self) -> bool {
        Self::lengths(self.size).contains(&self.length)
    }

    /// "Pin Rack", "Hogshead Cradle", "Tun Stand".
    pub fn name(self) -> String {
        let kind = match self.kind() {
            RackKind::Rack { .. } => "Rack",
            RackKind::Cradle => "Cradle",
            RackKind::Stand => "Stand",
        };
        format!("{} {kind}", self.size.name())
    }

    /// Where each of its casks lies, in the order they are filled: the
    /// bottom row first, from the piece's start. A rack's layer `l` has
    /// `2 × length − l` casks, each in a groove of the row below: 1 × 1 holds
    /// 3, 1 × 2 holds 9 (three layers) or 7 (two), 1 × 3 15 or 11.
    pub fn places(self) -> Vec<RackPlace> {
        let length = self.length;
        match self.kind() {
            RackKind::Rack { layers } => (0..layers)
                .flat_map(|layer| {
                    let row = (2 * length).saturating_sub(layer);
                    (0..row).map(move |j| RackPlace {
                        along: 2 * j + 1 + layer,
                        layer,
                    })
                })
                .collect(),
            RackKind::Cradle => (0..length)
                .map(|cell| RackPlace {
                    along: 4 * cell + 2,
                    layer: 0,
                })
                .collect(),
            RackKind::Stand => vec![RackPlace {
                along: 2 * length,
                layer: 0,
            }],
        }
    }

    /// How many casks it holds.
    pub fn capacity(self) -> usize {
        self.places().len()
    }
}
