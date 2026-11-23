//! Behind the back wall: the kitchen's cooking stations (Ranges, Hearths,
//! Stoves) and worktables, its back-left corner left empty
//! (`STAIR_DOWN_CORNER`) and the cells before the treasury's windows clear;
//! the treasury's chests (only to look at); the brewing room's Alchemy bench,
//! empty casks' cradles and racks, worktable and stores; the ageing room's
//! casks on cradles, stands and racks, and wine racks.

use super::super::spots::*;
use super::{GRANITE, OAK, PINE, WALNUT, piece};
use crate::bottle_rack::{BottleRack, BottleRackKind};
use crate::cask::CaskSize;
use crate::cask_rack::CaskRack;
use crate::chest::Chest;
use crate::crafting_bench::Craft;
use crate::door::Side;
use crate::furnishings::against_wall;
use crate::furniture::{Crates, Furniture, Piece, Sacks, TableStyle};

/// The kitchen's, treasury's, brewing and ageing rooms' pieces.
pub(super) fn pieces() -> Vec<Piece> {
    let mut pieces = kitchen();
    pieces.extend(treasury());
    pieces.extend(brewing());
    pieces.extend(ageing());
    pieces
}

/// A bench against the wall on its `wall` side: hearths of stone, the
/// Alchemy bench of wood.
fn bench(craft: Craft, at: (i64, i64), wall: Side) -> Piece {
    let furniture = Furniture::CraftingBench { craft };
    let material = if craft.is_hearth() { GRANITE } else { OAK };
    against_wall(furniture, at, (craft.length(), 1), wall, material)
}

fn worktable(at: (i64, i64), length: u8, facing: Side) -> Piece {
    let table = Furniture::Table {
        style: TableStyle::Trestle,
    };
    piece(table, at, (length, 1), facing, PINE)
}

fn cask_rack(size: CaskSize, length: u8, at: (i64, i64), wall: Side) -> Piece {
    let furniture = Furniture::CaskRack {
        rack: CaskRack { size, length },
    };
    against_wall(furniture, at, (length, 1), wall, OAK)
}

fn single(furniture: Furniture, at: (i64, i64)) -> Piece {
    piece(furniture, at, (1, 1), Side::South, PINE)
}

const CRATES: Furniture = Furniture::Crates {
    crates: Crates::Three,
};
const SACKS: Furniture = Furniture::Sacks { sacks: Sacks::Heap };

/// Down the west wall below the corner kept empty: a Range, a Hearth and a
/// Stove, two worktables beside them; along the north wall east of the
/// treasury a Range and a Hearth, a Stove on the east wall, a worktable; a
/// worktable and stores in the passage before the treasury.
fn kitchen() -> Vec<Piece> {
    vec![
        bench(Craft::Range, (1, 5), Side::West),
        bench(Craft::Hearth, (1, 9), Side::West),
        bench(Craft::Stove, (1, 12), Side::West),
        worktable((4, 5), 3, Side::South),
        worktable((4, 10), 2, Side::South),
        bench(Craft::Range, (14, 1), Side::North),
        bench(Craft::Hearth, (17, 1), Side::North),
        bench(Craft::Stove, (19, 4), Side::East),
        worktable((15, 4), 2, Side::East),
        worktable((15, 8), 3, Side::East),
        single(SACKS, (19, 9)),
        single(CRATES, (13, 9)),
    ]
}

/// Chests of gold along the treasury's walls, never opened (`look_only`).
fn treasury() -> Vec<Piece> {
    let (x0, y0) = (TREASURY.x, TREASURY.y);
    let x1 = x0 + TREASURY.width - 1;
    let chest = |chest| Furniture::Chest { chest };
    vec![
        against_wall(chest(Chest::Large), (x0, y0), (2, 1), Side::North, WALNUT),
        against_wall(
            chest(Chest::Small),
            (x0 + 2, y0),
            (1, 1),
            Side::North,
            WALNUT,
        ),
        against_wall(
            chest(Chest::Large),
            (x1 - 1, y0),
            (2, 1),
            Side::North,
            WALNUT,
        ),
        against_wall(
            chest(Chest::Large),
            (x0, y0 + 2),
            (2, 1),
            Side::West,
            WALNUT,
        ),
        against_wall(
            chest(Chest::Large),
            (x1, y0 + 2),
            (2, 1),
            Side::East,
            WALNUT,
        ),
    ]
}

/// The brewing room (east of the ageing room, and down beside the bar's
/// back): the Alchemy bench, cradles and racks for empty casks to ferment
/// in, a worktable, sacks of grain and crates.
fn brewing() -> Vec<Piece> {
    vec![
        bench(Craft::Alchemy, (32, 1), Side::North),
        cask_rack(CaskSize::Barrel, 3, (35, 1), Side::North),
        cask_rack(CaskSize::Hogshead, 1, (39, 1), Side::East),
        cask_rack(CaskSize::Kilderkin, 3, (39, 3), Side::East),
        cask_rack(CaskSize::Firkin, 2, (39, 7), Side::East),
        worktable((34, 5), 2, Side::East),
        cask_rack(CaskSize::Pin, 3, (35, 13), Side::South),
        single(SACKS, (39, 11)),
        single(SACKS, (39, 12)),
        single(CRATES, (31, 9)),
    ]
}

/// The ageing room (behind the bar, through the brewing room): Tuns on
/// their stands and Butts on cradles along the north wall, Puncheons and
/// Hogsheads down the west, Firkins along the south, wine racks on the
/// east either side of the door.
fn ageing() -> Vec<Piece> {
    let wine = Furniture::BottleRack {
        rack: BottleRack {
            kind: BottleRackKind::Wine,
            length: 3,
        },
    };
    let (door_y, east) = (AGEING_DOOR.1, AGEING_DOOR.0 - 1);
    vec![
        cask_rack(CaskSize::Tun, 2, (21, 1), Side::North),
        cask_rack(CaskSize::Tun, 2, (24, 1), Side::North),
        cask_rack(CaskSize::Butt, 2, (27, 1), Side::North),
        cask_rack(CaskSize::Puncheon, 3, (21, 3), Side::West),
        cask_rack(CaskSize::Hogshead, 3, (21, 6), Side::West),
        cask_rack(CaskSize::Firkin, 3, (23, 9), Side::South),
        cask_rack(CaskSize::Firkin, 3, (26, 9), Side::South),
        against_wall(wine, (east, door_y - 3), (3, 1), Side::East, OAK),
        against_wall(wine, (east, door_y + 1), (3, 1), Side::East, OAK),
    ]
}
