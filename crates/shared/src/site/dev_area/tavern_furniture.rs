//! The Tavern Dev Area's furniture, in the building's own cells (`Shell`):
//! the back-bar along the taproom's north wall (wine racks, spirit racks on
//! tables, a Firkin rack and a Tun stand), the bar in front of it with bar
//! stools, an inglenook with a settle on the west, games tables, a standing
//! barrel and tables in the middle, and booths along the south wall; the
//! bedrooms' are `tavern_bedrooms.rs`.

use super::tavern::Shell;
use crate::bar::{BarCounter, BarKind};
use crate::board_game::BoardGame;
use crate::bottle_rack::{BottleRack, BottleRackKind};
use crate::cask::CaskSize;
use crate::cask_rack::CaskRack;
use crate::door::Side;
use crate::dungeon::Level;
use crate::fireplace::Fireplace;
use crate::furnishings::against_wall;
use crate::furniture::{
    Barrel, BenchStyle, Candles, ChairStyle, Crates, Furniture, Material, Piece, Sacks, Stone,
    TableStyle, Wood, barrel_table, booth, footprint, games_table,
};

pub(super) const OAK: Material = Material::Wood(Wood::Oak);
pub(super) const PINE: Material = Material::Wood(Wood::Pine);
pub(super) const WALNUT: Material = Material::Wood(Wood::Walnut);

/// A piece at the building's cell `at`, `(long, short)` as `footprint`
/// takes them.
fn piece(
    shell: Shell,
    furniture: Furniture,
    at: (i64, i64),
    (long, short): (u8, u8),
    facing: Side,
    material: Material,
) -> Piece {
    Piece {
        at: shell.at(at),
        size: footprint(furniture, (long, short), facing),
        facing,
        material,
        furniture,
    }
}

/// A piece against the wall on its `wall` side.
pub(super) fn walled(
    shell: Shell,
    furniture: Furniture,
    at: (i64, i64),
    size: (u8, u8),
    wall: Side,
    material: Material,
) -> Piece {
    against_wall(furniture, shell.at(at), size, wall, material)
}

/// Puts every piece on `level`, those on the floor first.
pub(super) fn furnish(level: &mut Level, shell: Shell) {
    let mut pieces = back_bar(shell);
    pieces.extend(bar(shell));
    pieces.extend(taproom(shell));
    pieces.extend(super::tavern_bedrooms::bedrooms(shell));
    let (on_top, floor): (Vec<Piece>, Vec<Piece>) = pieces
        .into_iter()
        .partition(|p| p.furniture.stands_on_furniture());
    for piece in floor.into_iter().chain(on_top) {
        level.place_furniture(piece);
    }
}

/// Along the north wall: two wine racks, two tables with spirit racks on
/// them, a Firkin rack and a Tun on its stand, and crates at the end.
fn back_bar(shell: Shell) -> Vec<Piece> {
    let wine = Furniture::BottleRack {
        rack: BottleRack {
            kind: BottleRackKind::Wine,
            length: 3,
        },
    };
    let spirit = Furniture::BottleRack {
        rack: BottleRack {
            kind: BottleRackKind::Spirit,
            length: 3,
        },
    };
    let table = Furniture::Table {
        style: TableStyle::Joined,
    };
    let firkins = Furniture::CaskRack {
        rack: CaskRack {
            size: CaskSize::Firkin,
            length: 3,
        },
    };
    let tun = Furniture::CaskRack {
        rack: CaskRack {
            size: CaskSize::Tun,
            length: 2,
        },
    };
    vec![
        walled(shell, wine, (0, 0), (3, 1), Side::North, OAK),
        walled(shell, wine, (3, 0), (3, 1), Side::North, OAK),
        piece(shell, table, (6, 0), (3, 1), Side::East, PINE),
        piece(shell, spirit, (6, 0), (3, 1), Side::South, PINE),
        piece(shell, table, (9, 0), (3, 1), Side::East, PINE),
        piece(shell, spirit, (9, 0), (3, 1), Side::South, PINE),
        walled(shell, firkins, (12, 0), (3, 1), Side::North, OAK),
        walled(shell, tun, (15, 0), (2, 1), Side::North, OAK),
        piece(
            shell,
            Furniture::Crates {
                crates: Crates::Three,
            },
            (17, 1),
            (1, 1),
            Side::West,
            PINE,
        ),
    ]
}

/// The bar a cell in front of the back-bar, facing the drinkers with a flap
/// in its middle, candles on it and a stool at every cell.
fn bar(shell: Shell) -> Vec<Piece> {
    let counter = |kind, length| Furniture::BarCounter {
        counter: BarCounter { kind, length },
    };
    let runs = [
        (2, BarKind::Double, 3),
        (5, BarKind::Single, 3),
        (8, BarKind::Flap, 1),
        (9, BarKind::Single, 3),
        (12, BarKind::Double, 3),
    ];
    let mut pieces: Vec<Piece> = (runs.into_iter())
        .map(|(x, kind, length)| {
            piece(
                shell,
                counter(kind, length),
                (x, 2),
                (length, 1),
                Side::South,
                OAK,
            )
        })
        .collect();
    let stool = Furniture::Chair {
        style: ChairStyle::BarStool,
        arms: false,
    };
    pieces.extend(
        (2..=14)
            .filter(|&x| x != 8)
            .map(|x| piece(shell, stool, (x, 3), (1, 1), Side::North, OAK)),
    );
    let candles = Furniture::Candles {
        candles: Candles::Pair,
    };
    pieces.push(piece(shell, candles, (6, 2), (1, 1), Side::South, OAK));
    pieces.push(piece(shell, candles, (10, 2), (1, 1), Side::South, OAK));
    pieces
}

/// The inglenook and its settle, the games tables, a standing barrel, two
/// tables with candles, booths along the south wall and sacks in a corner.
fn taproom(shell: Shell) -> Vec<Piece> {
    let inglenook = Furniture::Fireplace {
        fireplace: Fireplace { length: 3 },
    };
    let settle = Furniture::Bench {
        style: BenchStyle::Settle,
    };
    let mut pieces = vec![
        walled(
            shell,
            inglenook,
            (0, 6),
            (3, 1),
            Side::West,
            Material::Stone(Stone::Granite),
        ),
        piece(shell, settle, (2, 6), (3, 1), Side::West, OAK),
    ];
    pieces.extend(games_table(
        shell.at((6, 7)),
        BoardGame::Chess,
        Side::East,
        ChairStyle::BackStool,
        Wood::Oak,
    ));
    pieces.extend(games_table(
        shell.at((11, 7)),
        BoardGame::Hnefatafl,
        Side::East,
        ChairStyle::BackStool,
        Wood::Walnut,
    ));
    pieces.extend(barrel_table(
        shell.at((15, 7)),
        Barrel::Topped,
        &[Side::West, Side::East],
        Wood::Oak,
    ));
    let table = Furniture::Table {
        style: TableStyle::Joined,
    };
    let chair = Furniture::Chair {
        style: ChairStyle::LadderBack,
        arms: false,
    };
    let lights = [Candles::Candlestick, Candles::Candelabrum];
    for (x, candles) in [5, 11].into_iter().zip(lights) {
        pieces.push(piece(shell, table, (x, 10), (2, 1), Side::East, OAK));
        pieces.push(piece(
            shell,
            Furniture::Candles { candles },
            (x, 10),
            (1, 1),
            Side::South,
            OAK,
        ));
        for dx in 0..2 {
            pieces.push(piece(shell, chair, (x + dx, 9), (1, 1), Side::South, OAK));
            pieces.push(piece(shell, chair, (x + dx, 11), (1, 1), Side::North, OAK));
        }
    }
    for x in [1, 8, 14] {
        pieces.extend(booth(shell.at((x, 12)), 2, 1, true, Wood::Oak));
    }
    pieces.push(piece(
        shell,
        Furniture::Sacks { sacks: Sacks::Heap },
        (17, 14),
        (1, 1),
        Side::West,
        PINE,
    ));
    pieces
}
