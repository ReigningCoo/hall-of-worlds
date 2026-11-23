//! The private rooms down the main room's west side and the booths down its
//! east (`ROW_COUNT` each, 6 × 4 inside): a private room has a 2 × 2 table
//! in its middle with two chairs at each end and candles on it, room to
//! walk all round; a booth two settles across
//! a table 3 × 2, set back from the main room so it is sat at from there.

use super::super::spots::*;
use super::{OAK, WALNUT, piece};
use crate::door::Side;
use crate::furniture::{Candles, ChairStyle, Furniture, Piece, TableStyle, Wood, booth};

/// The private rooms' west column and the booths'.
const ROOMS_X: i64 = 1;
const BOOTHS_X: i64 = 34;

/// Every private room's and booth's pieces.
pub(super) fn pieces() -> Vec<Piece> {
    let mut pieces = Vec::new();
    for n in 0..ROW_COUNT {
        let y = ROW_FIRST_Y + n * ROW_PITCH;
        pieces.extend(private_room((ROOMS_X, y), n));
        pieces.extend(booth_at((BOOTHS_X, y), n));
    }
    pieces
}

/// A private room whose north-west cell is `(x, y)`: the table in its
/// middle, two chairs at its west and east ends, its first and last rows
/// and outer columns left to walk round (its door is in its east wall).
fn private_room((x, y): (i64, i64), n: i64) -> Vec<Piece> {
    let material = if n % 2 == 0 { OAK } else { WALNUT };
    let table = Furniture::Table {
        style: TableStyle::Joined,
    };
    let chair = Furniture::Chair {
        style: ChairStyle::PanelBack,
        arms: false,
    };
    let (tx, ty) = (x + 2, y + 1);
    let mut pieces = vec![piece(table, (tx, ty), (2, 2), Side::East, material)];
    for d in 0..2 {
        pieces.push(piece(chair, (tx - 1, ty + d), (1, 1), Side::East, material));
        pieces.push(piece(chair, (tx + 2, ty + d), (1, 1), Side::West, material));
    }
    let candles = Furniture::Candles {
        candles: Candles::Candelabrum,
    };
    pieces.push(piece(candles, (tx, ty), (1, 1), Side::South, material));
    pieces
}

/// A booth whose north-west cell is `(x, y)`: settles along its north and
/// south walls across a table, a cell in from the main room's column.
fn booth_at((x, y): (i64, i64), n: i64) -> Vec<Piece> {
    let wood = if n % 2 == 0 { Wood::Oak } else { Wood::Walnut };
    let mut pieces = booth((x + 1, y), 3, 2, true, wood).to_vec();
    let candles = Furniture::Candles {
        candles: Candles::Pair,
    };
    pieces.push(piece(candles, (x + 2, y + 1), (1, 1), Side::South, OAK));
    pieces
}
