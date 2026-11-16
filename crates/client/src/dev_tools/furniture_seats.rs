//! The seats `HOW_DEBUG_FURNITURE=show` puts beside its tables: some round
//! the first two tables (to see them reach under the edge), and every chair
//! and bench style in rows south of the tables.

use shared::door::Side;
use shared::furniture::{BenchStyle, ChairStyle, Furniture, Material, Piece, footprint};

/// The seats shown, from where the player first stood.
pub(super) fn shown_seats((x, y): (i64, i64)) -> Vec<Piece> {
    let wood = |n: usize| Material::ALL[n % 8];
    let stone = |n: usize| Material::ALL[8 + n % 5];
    let piece = |at, long, facing, material, furniture| Piece {
        at,
        size: footprint(furniture, (long, 1), facing),
        facing,
        material,
        furniture,
    };
    let chair = |style, arms| Furniture::Chair { style, arms };
    let bench = |style| Furniture::Bench { style };
    let mut seats = vec![
        // Round the trestle table (3 × 2 at x + 1, y + 3).
        piece(
            (x + 1, y + 2),
            3,
            Side::South,
            wood(0),
            bench(BenchStyle::SplayedPlank),
        ),
        piece(
            (x + 1, y + 5),
            3,
            Side::North,
            wood(0),
            bench(BenchStyle::LongStool),
        ),
        piece(
            (x + 4, y + 3),
            1,
            Side::West,
            wood(0),
            chair(ChairStyle::BackStool, true),
        ),
        // Round the joined table (2 × 3 at x + 6, y + 3).
        piece(
            (x + 5, y + 3),
            3,
            Side::East,
            wood(1),
            bench(BenchStyle::BoardEnded),
        ),
        piece(
            (x + 6, y + 6),
            1,
            Side::North,
            wood(1),
            chair(ChairStyle::PanelBack, true),
        ),
    ];
    for at in [(x + 8, y + 3), (x + 8, y + 4), (x + 8, y + 5)] {
        seats.push(piece(
            at,
            1,
            Side::West,
            wood(1),
            chair(ChairStyle::LadderBack, false),
        ));
    }
    // Every chair facing south, then the wooden ones with arms facing north.
    for (i, style) in ChairStyle::ALL.into_iter().enumerate() {
        let material = if style == ChairStyle::StoneBlock {
            stone(i)
        } else {
            wood(i)
        };
        let at = (x + 1 + 2 * i as i64, y + 14);
        seats.push(piece(at, 1, Side::South, material, chair(style, false)));
        if style != ChairStyle::StoneBlock {
            let at = (at.0, y + 16);
            seats.push(piece(at, 1, Side::North, material, chair(style, true)));
        }
    }
    // Every bench, 3, 2 and 1 long, facing south, north and east.
    let mut along = x + 1;
    for (i, style) in BenchStyle::ALL.into_iter().enumerate() {
        let material = if style == BenchStyle::StoneSlab {
            stone(i)
        } else {
            wood(i + 3)
        };
        seats.push(piece(
            (along, y + 18),
            3,
            Side::South,
            material,
            bench(style),
        ));
        seats.push(piece(
            (along, y + 20),
            2,
            Side::North,
            material,
            bench(style),
        ));
        seats.push(piece(
            (along, y + 22),
            1,
            Side::East,
            material,
            bench(style),
        ));
        along += 4;
    }
    seats
}
