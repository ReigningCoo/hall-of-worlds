//! One piece of decor on a level: where it is and what it is.

use serde::{Deserialize, Serialize};

use super::{CHANDELIER_LIGHT_CELLS, ChandelierStyle, Hanging, RUG_MOST, Rug};
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::furniture::takes_furniture;

/// A piece of decor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Decor {
    /// Hanging on the face of the wall cell `wall` turned towards `face`,
    /// over the cell beside it on that side (`front`).
    Wall {
        wall: (i64, i64),
        face: Side,
        hanging: Hanging,
    },
    /// Hanging from the ceiling over a floor cell.
    Chandelier {
        at: (i64, i64),
        style: ChandelierStyle,
    },
    /// On the floor: `at` its north-west cell, `size` its cells east–west and
    /// north–south.
    Rug {
        at: (i64, i64),
        size: (u8, u8),
        rug: Rug,
    },
}

/// Where a piece of decor goes: one piece a place, a new one replacing the
/// old (rugs also replace any they overlap).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DecorSlot {
    /// A wall cell's face.
    Wall((i64, i64), Side),
    /// The ceiling over a cell.
    Ceiling((i64, i64)),
    /// A rug's north-west cell.
    Floor((i64, i64)),
}

impl Decor {
    pub fn slot(&self) -> DecorSlot {
        match *self {
            Self::Wall { wall, face, .. } => DecorSlot::Wall(wall, face),
            Self::Chandelier { at, .. } => DecorSlot::Ceiling(at),
            Self::Rug { at, .. } => DecorSlot::Floor(at),
        }
    }

    /// The cells it is sent and seen with: a wall's and the cell in front of
    /// it, a chandelier's, every one a rug covers.
    pub fn cells(&self) -> Vec<(i64, i64)> {
        match *self {
            Self::Wall { wall, face, .. } => vec![wall, face.beside(wall)],
            Self::Chandelier { at, .. } => vec![at],
            Self::Rug {
                at: (x, y), size, ..
            } => {
                let (width, height) = (i64::from(size.0), i64::from(size.1));
                (y..y + height)
                    .flat_map(|cy| (x..x + width).map(move |cx| (cx, cy)))
                    .collect()
            }
        }
    }

    /// The cell its light shines from and how far it reaches, if it gives
    /// any: a sconce's from the cell in front of its wall.
    pub fn light(&self) -> Option<((i64, i64), u32)> {
        match *self {
            Self::Wall {
                wall,
                face,
                hanging,
            } => Some((face.beside(wall), hanging.light_cells()?)),
            Self::Chandelier { at, .. } => Some((at, CHANDELIER_LIGHT_CELLS)),
            Self::Rug { .. } => None,
        }
    }

    /// Whether it can go on `level`: on a wall cell's face with floor or
    /// ground that takes furniture in front; a chandelier or rug only on
    /// indoor floor (`Cell::Floor`, under a ceiling), a rug 1 to `RUG_MOST`
    /// cells each way. Stairs are left to the caller.
    pub fn fits(&self, level: &Level) -> bool {
        let floor = |(x, y): (i64, i64)| level.contains(x, y) && level.cell(x, y) == Cell::Floor;
        match *self {
            Self::Wall { wall, face, .. } => {
                let (fx, fy) = face.beside(wall);
                level.contains(wall.0, wall.1)
                    && level.cell(wall.0, wall.1) == Cell::Wall
                    && level.contains(fx, fy)
                    && takes_furniture(level.cell(fx, fy))
            }
            Self::Chandelier { at, .. } => floor(at),
            Self::Rug { size, .. } => {
                let sized = |n: u8| (1..=RUG_MOST).contains(&n);
                sized(size.0) && sized(size.1) && self.cells().into_iter().all(floor)
            }
        }
    }

    /// The same moved by `(dx, dy)` cells.
    pub fn moved(self, (dx, dy): (i64, i64)) -> Self {
        let shift = |(x, y): (i64, i64)| (x + dx, y + dy);
        match self {
            Self::Wall {
                wall,
                face,
                hanging,
            } => Self::Wall {
                wall: shift(wall),
                face,
                hanging,
            },
            Self::Chandelier { at, style } => Self::Chandelier {
                at: shift(at),
                style,
            },
            Self::Rug { at, size, rug } => Self::Rug {
                at: shift(at),
                size,
                rug,
            },
        }
    }
}
