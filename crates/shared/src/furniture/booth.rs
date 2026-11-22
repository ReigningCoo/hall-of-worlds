//! Booths (tavern furniture plan PR 5): two high-backed settles facing each
//! other across a table, as three ordinary pieces, so a booth needs nothing
//! of its own on a level. For code placing them (Place Furniture, a tavern's
//! layout) to make the three alike.

use super::{BenchStyle, Furniture, Material, Piece, TableStyle, Wood};
use crate::door::Side;

/// A booth whose north-west cell is `at`: a settle `length` long (1 to 3), a
/// table `length` by `depth` (1 or 2), then a settle facing the first,
/// running east–west if `east_west` (settles north and south of the table),
/// else north–south (settles west and east). Of `wood`, the table joined.
pub fn booth(at: (i64, i64), length: u8, depth: u8, east_west: bool, wood: Wood) -> [Piece; 3] {
    let material = Material::Wood(wood);
    let (x, y) = at;
    // `n` cells across the booth from its first settle.
    let across = |n: i64| if east_west { (x, y + n) } else { (x + n, y) };
    let (first_faces, last_faces) = if east_west {
        (Side::South, Side::North)
    } else {
        (Side::East, Side::West)
    };
    let settle = |at: (i64, i64), facing: Side| Piece {
        at,
        size: if east_west { (length, 1) } else { (1, length) },
        facing,
        material,
        furniture: Furniture::Bench {
            style: BenchStyle::Settle,
        },
    };
    let size = if east_west {
        (length, depth)
    } else {
        (depth, length)
    };
    // A table's length runs east or south, and east when square.
    let table = Piece {
        at: across(1),
        size,
        facing: if size.1 > size.0 {
            Side::South
        } else {
            Side::East
        },
        material,
        furniture: Furniture::Table {
            style: TableStyle::Joined,
        },
    };
    [
        settle(across(0), first_faces),
        table,
        settle(across(1 + i64::from(depth)), last_faces),
    ]
}
