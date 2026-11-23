//! The main room's scattered tables (owner's: about 30 groups up to 2 × 2,
//! never a grid, walking space round each): groups of a kind rolled from a
//! fixed seed, each put where it fits in `TABLES` with an empty cell between
//! it and the next group, the stairs kept clear. The same every time, as
//! nothing but the seed goes in.

use super::super::spots::{STAIRS, TABLES};
use super::{OAK, piece};
use crate::board_game::BoardGame;
use crate::door::Side;
use crate::furniture::{
    Barrel, Candles, ChairStyle, Furniture, Material, Piece, TableStyle, Wood, barrel_table,
    games_table,
};
use crate::rng::{Rng, Seed};
use crate::site::Rect;
use std::collections::BTreeSet;

/// How many groups the room gets at most, and how many places are tried.
pub(super) const GROUPS: usize = 30;
const TRIES: usize = 4000;
/// The cells round the stairs kept clear: the stairs and a ring 2 wide.
const STAIRS_CLEAR: i64 = 2;

/// The kinds of group, and how often each is rolled.
#[derive(Clone, Copy)]
enum Kind {
    /// A 1 × 1 table, a chair on two to four sides.
    Small,
    /// A 2 × 1 table east–west or north–south, chairs along both long sides
    /// and perhaps its ends.
    Long,
    /// A 2 × 1 table with a bench along each long side.
    Benches,
    /// A 2 × 2 table, two chairs a side.
    Square,
    /// A games table, a player each side.
    Games,
    /// A standing barrel with stools round it.
    Barrel,
}

const KINDS: [(Kind, u64); 6] = [
    (Kind::Small, 5),
    (Kind::Long, 6),
    (Kind::Benches, 3),
    (Kind::Square, 4),
    (Kind::Games, 2),
    (Kind::Barrel, 3),
];

/// Every side, as chairs may stand.
const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];

const WOODS: [(Wood, u64); 4] = [
    (Wood::Oak, 4),
    (Wood::Walnut, 2),
    (Wood::Elm, 2),
    (Wood::Cherry, 1),
];

/// Every group's pieces.
pub(super) fn pieces() -> Vec<Piece> {
    let mut rng = Seed::new(0x1A_4E_57).derive("inn tables").rng();
    let mut taken: BTreeSet<(i64, i64)> = BTreeSet::new();
    let (sx, sy) = STAIRS;
    for x in sx - STAIRS_CLEAR..=sx + STAIRS_CLEAR {
        for y in sy - STAIRS_CLEAR..=sy + STAIRS_CLEAR {
            taken.insert((x, y));
        }
    }
    let mut pieces = Vec::new();
    let mut groups = 0;
    for _ in 0..TRIES {
        if groups == GROUPS {
            break;
        }
        let kind = rng.pick_weighted(&KINDS);
        let wood = rng.pick_weighted(&WOODS);
        let group = group(kind, wood, &mut rng);
        let (w, h) = extent(&group);
        // Inside `TABLES` with a cell to spare all round, so its edges stay
        // clear for walking (before the bar's stools, the rows and the foyer).
        let (room_w, room_h) = (TABLES.width - 2 - w + 1, TABLES.height - 2 - h + 1);
        if room_w < 1 || room_h < 1 {
            continue;
        }
        let x = TABLES.x + 1 + rng.range_u64(0, room_w as u64) as i64;
        let y = TABLES.y + 1 + rng.range_u64(0, room_h as u64) as i64;
        let group: Vec<Piece> = group.into_iter().map(|p| p.moved((x, y))).collect();
        let rect = Rect {
            x,
            y,
            width: w,
            height: h,
        };
        let round = cells(rect, 1);
        if round.iter().any(|cell| taken.contains(cell)) {
            continue;
        }
        taken.extend(cells(rect, 0));
        pieces.extend(group);
        groups += 1;
    }
    pieces
}

/// The cells of `rect` grown by `by` each way.
fn cells(rect: Rect, by: i64) -> Vec<(i64, i64)> {
    let xs = rect.x - by..rect.x + rect.width + by;
    xs.flat_map(|x| (rect.y - by..rect.y + rect.height + by).map(move |y| (x, y)))
        .collect()
}

/// How far a group made at `(0, 0)` reaches east and south.
fn extent(group: &[Piece]) -> (i64, i64) {
    let cells = group.iter().flat_map(|piece| piece.cells());
    cells.fold((0, 0), |(w, h), (x, y)| (w.max(x + 1), h.max(y + 1)))
}

/// A group of `kind` in `wood`, its north-west corner at `(0, 0)` (every
/// piece at or east and south of it).
fn group(kind: Kind, wood: Wood, rng: &mut Rng) -> Vec<Piece> {
    let material = Material::Wood(wood);
    match kind {
        Kind::Small => {
            let sides: Vec<Side> = (SIDES.into_iter())
                .filter(|_| rng.range_u64(0, 4) != 0)
                .collect();
            let sides = if sides.len() < 2 {
                vec![Side::West, Side::East]
            } else {
                sides
            };
            seated((1, 1), &sides, material, rng)
        }
        Kind::Long => {
            let ends = rng.range_u64(0, 2) == 0;
            if rng.range_u64(0, 2) == 0 {
                let mut sides = vec![Side::North, Side::South];
                if ends {
                    sides.extend([Side::West, Side::East]);
                }
                seated((2, 1), &sides, material, rng)
            } else {
                let mut sides = vec![Side::West, Side::East];
                if ends {
                    sides.extend([Side::North, Side::South]);
                }
                seated((1, 2), &sides, material, rng)
            }
        }
        Kind::Benches => benches(material, rng),
        Kind::Square => seated((2, 2), &SIDES, material, rng),
        Kind::Games => {
            let game = match rng.range_u64(0, 2) {
                0 => BoardGame::Chess,
                _ => BoardGame::Hnefatafl,
            };
            let facing = [Side::East, Side::South][rng.range_u64(0, 2) as usize];
            let at = match facing {
                Side::East => (1, 0),
                _ => (0, 1),
            };
            let style = ChairStyle::BackStool;
            games_table(at, game, facing, style, wood).to_vec()
        }
        Kind::Barrel => {
            let sides: Vec<Side> = (SIDES.into_iter())
                .filter(|_| rng.range_u64(0, 3) != 0)
                .collect();
            let sides = if sides.len() < 2 {
                vec![Side::West, Side::East]
            } else {
                sides
            };
            let mut pieces = barrel_table((1, 1), Barrel::Topped, &sides, wood);
            pieces.push(candles((1, 1), rng));
            pieces
        }
    }
}

/// A table `size` cells (east–west, north–south) with chairs all along each
/// of `sides`, candles on it most of the time; the table a cell in from
/// `(0, 0)` on each side with chairs, so the group's corner is `(0, 0)`.
fn seated((w, h): (i64, i64), sides: &[Side], material: Material, rng: &mut Rng) -> Vec<Piece> {
    let x0 = i64::from(sides.contains(&Side::West));
    let y0 = i64::from(sides.contains(&Side::North));
    let style = [
        TableStyle::Joined,
        TableStyle::Trestle,
        TableStyle::PlankLegs,
    ][rng.range_u64(0, 3) as usize];
    let style = if (w, h) == (1, 1) && rng.range_u64(0, 2) == 0 {
        TableStyle::Pedestal
    } else {
        style
    };
    let chairs = style.chairs();
    let chair = Furniture::Chair {
        style: chairs[rng.range_u64(0, chairs.len() as u64) as usize],
        arms: false,
    };
    let table = Furniture::Table { style };
    let facing = if h > w { Side::South } else { Side::East };
    let (long, short) = (w.max(h) as u8, w.min(h) as u8);
    let mut pieces = vec![piece(table, (x0, y0), (long, short), facing, material)];
    for &side in sides {
        let seats: Vec<(i64, i64)> = match side {
            Side::North => (0..w).map(|dx| (x0 + dx, y0 - 1)).collect(),
            Side::South => (0..w).map(|dx| (x0 + dx, y0 + h)).collect(),
            Side::West => (0..h).map(|dy| (x0 - 1, y0 + dy)).collect(),
            Side::East => (0..h).map(|dy| (x0 + w, y0 + dy)).collect(),
        };
        for at in seats {
            pieces.push(piece(chair, at, (1, 1), side.opposite(), material));
        }
    }
    if rng.range_u64(0, 5) != 0 {
        pieces.push(candles((x0, y0), rng));
    }
    pieces
}

/// A 2 × 1 table east–west with a bench along each long side.
fn benches(material: Material, rng: &mut Rng) -> Vec<Piece> {
    let style = [TableStyle::Trestle, TableStyle::Joined][rng.range_u64(0, 2) as usize];
    let bench = Furniture::Bench {
        style: style.benches()[0],
    };
    vec![
        piece(bench, (0, 0), (2, 1), Side::South, material),
        piece(
            Furniture::Table { style },
            (0, 1),
            (2, 1),
            Side::East,
            material,
        ),
        piece(bench, (0, 2), (2, 1), Side::North, material),
        candles((0, 1), rng),
    ]
}

/// Candles of a rolled kind on the table at `at`.
fn candles(at: (i64, i64), rng: &mut Rng) -> Piece {
    let candles = Candles::ALL[rng.range_u64(0, Candles::ALL.len() as u64) as usize];
    piece(Furniture::Candles { candles }, at, (1, 1), Side::South, OAK)
}
