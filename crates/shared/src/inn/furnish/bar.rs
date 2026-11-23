//! The main room's back and front: the drink storage against the back wall
//! either side of the back fireplace (wine racks, spirit racks on tables,
//! small casks' racks), the bar counter across the room with its flap,
//! stools and candles; the two fireplaces either side of the entrance with
//! chairs facing them; and the foyer's furniture.

use super::super::spots::*;
use super::{GRANITE, OAK, PINE, WALNUT, piece};
use crate::bar::{BarCounter, BarKind};
use crate::bottle_rack::{BottleRack, BottleRackKind};
use crate::cask::CaskSize;
use crate::cask_rack::CaskRack;
use crate::chest::Chest;
use crate::door::Side;
use crate::fireplace::Fireplace;
use crate::furnishings::against_wall;
use crate::furniture::{Candles, ChairStyle, Furniture, Piece, TableStyle};

/// The main room's pieces at its back and front, and the foyer's.
pub(super) fn pieces() -> Vec<Piece> {
    let mut pieces = storage();
    pieces.extend(counter());
    pieces.extend(front_fires());
    pieces.extend(foyer());
    pieces
}

fn bottle_rack(kind: BottleRackKind) -> Furniture {
    Furniture::BottleRack {
        rack: BottleRack { kind, length: 3 },
    }
}

fn cask_rack(size: CaskSize, length: u8) -> Furniture {
    Furniture::CaskRack {
        rack: CaskRack { size, length },
    }
}

/// Along the back wall behind the bar, west to east: two wine racks, a
/// spirit rack on a table, a Pin rack, the fireplace, a Pin rack, a spirit
/// rack on a table, a Firkin rack and a Kilderkin rack.
fn storage() -> Vec<Piece> {
    let y = STORAGE.y;
    let wine = bottle_rack(BottleRackKind::Wine);
    let spirit = bottle_rack(BottleRackKind::Spirit);
    let table = Furniture::Table {
        style: TableStyle::Joined,
    };
    let back = |furniture, x: i64, length: u8| {
        against_wall(furniture, (x, y), (length, 1), Side::North, OAK)
    };
    let fireplace = Furniture::Fireplace {
        fireplace: Fireplace {
            length: BACK_FIREPLACE.width as u8,
        },
    };
    vec![
        back(wine, 8, 3),
        back(wine, 11, 3),
        piece(table, (14, y), (3, 1), Side::East, PINE),
        piece(spirit, (14, y), (3, 1), Side::South, PINE),
        back(cask_rack(CaskSize::Pin, 2), 17, 2),
        against_wall(
            fireplace,
            (BACK_FIREPLACE.x, y),
            (BACK_FIREPLACE.width as u8, 1),
            Side::North,
            GRANITE,
        ),
        back(cask_rack(CaskSize::Pin, 2), 22, 2),
        piece(table, (24, y), (3, 1), Side::East, PINE),
        piece(spirit, (24, y), (3, 1), Side::South, PINE),
        back(cask_rack(CaskSize::Firkin, 3), 27, 3),
        back(cask_rack(CaskSize::Kilderkin, 3), 30, 3),
    ]
}

/// The counter wall to wall, served across to the south, its flap
/// (`BAR_FLAP`) the way behind; doubles at its ends, candles along it, and
/// a stool before every cell but the flap's and the one beside it.
fn counter() -> Vec<Piece> {
    let y = BAR.y;
    let (flap_x, end_x) = (BAR_FLAP.0, BAR.x + BAR.width - 1);
    let runs = [
        (BAR.x, BarKind::Double, 3),
        (11, BarKind::Single, 3),
        (14, BarKind::Single, 3),
        (17, BarKind::Single, 3),
        (20, BarKind::Single, 3),
        (23, BarKind::Single, 3),
        (26, BarKind::Double, 3),
        (29, BarKind::Single, 1),
        (flap_x, BarKind::Flap, 1),
        (flap_x + 1, BarKind::Double, (end_x - flap_x) as u8),
    ];
    let mut pieces: Vec<Piece> = (runs.into_iter())
        .map(|(x, kind, length)| {
            let counter = Furniture::BarCounter {
                counter: BarCounter { kind, length },
            };
            piece(counter, (x, y), (length, 1), Side::South, OAK)
        })
        .collect();
    let stool = Furniture::Chair {
        style: ChairStyle::BarStool,
        arms: false,
    };
    pieces.extend(
        (BAR.x + 1..flap_x - 1).map(|x| piece(stool, (x, y + 1), (1, 1), Side::North, OAK)),
    );
    for (x, candles) in [
        (12, Candles::Pair),
        (18, Candles::Candelabrum),
        (24, Candles::Pair),
    ] {
        let candles = Furniture::Candles { candles };
        pieces.push(piece(candles, (x, y), (1, 1), Side::South, OAK));
    }
    pieces
}

/// Each front fireplace against the south wall, two armchairs a cell back
/// from it facing it.
fn front_fires() -> Vec<Piece> {
    let chair = Furniture::Chair {
        style: ChairStyle::PanelBack,
        arms: true,
    };
    let mut pieces = Vec::new();
    for fire in FRONT_FIREPLACES {
        let length = fire.width as u8;
        let fireplace = Furniture::Fireplace {
            fireplace: Fireplace { length },
        };
        let at = (fire.x, fire.y);
        pieces.push(against_wall(
            fireplace,
            at,
            (length, 1),
            Side::South,
            GRANITE,
        ));
        for x in [fire.x, fire.x + fire.width - 1] {
            pieces.push(piece(chair, (x, fire.y - 2), (1, 1), Side::South, WALNUT));
        }
    }
    pieces
}

/// The foyer's west side: a chair facing a 1 × 1 table, then a small chest,
/// along the wall; its east side a chest and a small chest end to end (a
/// 1 × 3 chest). Only to look at (`look_only`).
fn foyer() -> Vec<Piece> {
    let (x, y) = (FOYER_WEST.x, FOYER_WEST.y);
    let table = Furniture::Table {
        style: TableStyle::Pedestal,
    };
    let chair = Furniture::Chair {
        style: ChairStyle::XFrame,
        arms: true,
    };
    let chest = |chest| Furniture::Chest { chest };
    let (ex, ey) = (FOYER_EAST.x, FOYER_EAST.y);
    vec![
        piece(chair, (x, y), (1, 1), Side::South, WALNUT),
        piece(table, (x, y + 1), (1, 1), Side::East, WALNUT),
        against_wall(chest(Chest::Small), (x, y + 2), (1, 1), Side::West, WALNUT),
        against_wall(chest(Chest::Large), (ex, ey), (2, 1), Side::East, WALNUT),
        against_wall(
            chest(Chest::Small),
            (ex, ey + 2),
            (1, 1),
            Side::East,
            WALNUT,
        ),
        piece(
            Furniture::Candles {
                candles: Candles::Candlestick,
            },
            (x, y + 1),
            (1, 1),
            Side::South,
            WALNUT,
        ),
    ]
}
