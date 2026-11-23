//! The inn's decor: something on most walls (every third cell along a
//! wall, sconces and hangings by turns, what hangs chosen by the room),
//! chandeliers over the main room and the back rooms, and rugs (under the
//! private rooms' tables, before the front fires, up the foyer, and runners
//! along upstairs' corridors).

use super::super::spots::*;
use super::super::{GROUND, InnRoom, RoomKind, UPSTAIRS};
use crate::decor::{ChandelierStyle, Decor, Hanging, Rug, RugColour, RugPattern};
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::furniture::Furniture;
use crate::rng::Seed;
use std::collections::BTreeSet;

/// Every third cell along a wall gets something.
const EVERY: i64 = 3;
const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];

/// Everything on floor `index` (already furnished: nothing hangs behind a
/// piece standing against its wall).
pub(super) fn decor(level: &Level, index: u32, rooms: &[InnRoom]) -> Vec<Decor> {
    let mut decor = hangings(level, rooms);
    match index {
        GROUND => {
            notice_board(&mut decor);
            decor.extend(ground_chandeliers());
            decor.extend(ground_rugs());
        }
        UPSTAIRS => decor.extend(runners()),
        _ => {}
    }
    decor
}

/// What may hang in a room of `kind` besides sconces.
fn hangings_for(kind: RoomKind) -> &'static [Hanging] {
    use Hanging::*;
    match kind {
        RoomKind::Kitchen => &[Hams, Onions, HerbBundles],
        RoomKind::Brewing | RoomKind::Ageing => &[HerbBundles, Hams, Onions, Shield],
        RoomKind::Treasury => &[Shield, CrossedWeapons, Tapestry],
        RoomKind::Bedroom { .. } | RoomKind::OwnRoom { .. } => &[Tapestry, CoatPegs],
        RoomKind::Corridor => &[Tapestry, Shield, Antlers],
        _ => &[
            Antlers,
            BoarHead,
            Shield,
            CrossedWeapons,
            Tapestry,
            CoatPegs,
        ],
    }
}

/// Whether a piece standing with its back to a wall covers that wall: racks,
/// benches, fireplaces, wardrobes and the bar.
fn covers_wall(furniture: Furniture) -> bool {
    matches!(
        furniture,
        Furniture::CraftingBench { .. }
            | Furniture::CaskRack { .. }
            | Furniture::BottleRack { .. }
            | Furniture::Fireplace { .. }
            | Furniture::Wardrobe { .. }
            | Furniture::BarCounter { .. }
    )
}

/// Along every room's walls, every `EVERY`th cell (by the wall's own
/// coordinate along it): a sconce or a hanging by turns, none behind a
/// piece covering its wall.
fn hangings(level: &Level, rooms: &[InnRoom]) -> Vec<Decor> {
    let seed = Seed::new(0x1A_4E_57).derive("inn hangings");
    let mut seen = BTreeSet::new();
    let mut decor = Vec::new();
    for room in rooms {
        let choices = hangings_for(room.kind);
        for rect in &room.rects {
            for x in rect.x..rect.x + rect.width {
                for y in rect.y..rect.y + rect.height {
                    for side in SIDES {
                        let wall = side.beside((x, y));
                        let face = side.opposite();
                        if !seen.insert((wall, face)) || level.cell(wall.0, wall.1) != Cell::Wall {
                            continue;
                        }
                        let along = match side {
                            Side::North | Side::South => x,
                            Side::East | Side::West => y,
                        };
                        if along % EVERY != 0 {
                            continue;
                        }
                        let covered = level.furniture_at(x, y);
                        if covered.is_some_and(|piece| covers_wall(piece.furniture)) {
                            continue;
                        }
                        let hanging = if along % (2 * EVERY) == 0 {
                            Hanging::Sconce
                        } else {
                            let hash = seed.at(&[wall.0, wall.1, face as i64]);
                            choices[(hash % choices.len() as u64) as usize]
                        };
                        decor.push(Decor::Wall {
                            wall,
                            face,
                            hanging,
                        });
                    }
                }
            }
        }
    }
    decor
}

/// By the archway, where everyone passes: a notice board and coat pegs, in
/// the place of what hung there.
fn notice_board(decor: &mut Vec<Decor>) {
    let (ax, ay) = ARCHWAY;
    for (x, hanging) in [(ax - 1, Hanging::NoticeBoard), (ax + 1, Hanging::CoatPegs)] {
        let board = Decor::Wall {
            wall: (x, ay),
            face: Side::North,
            hanging,
        };
        decor.retain(|other| other.slot() != board.slot());
        decor.push(board);
    }
}

/// Chandeliers: six over the main room's tables (none over the stairs),
/// one over the bar's gap, and one in each back room.
fn ground_chandeliers() -> Vec<Decor> {
    let [iron, wood, brass] = ChandelierStyle::ALL;
    let (sx, _) = STAIRS;
    [
        ((13, 21), brass),
        ((sx + 7, 21), brass),
        ((sx, 24), brass),
        ((13, 37), brass),
        ((sx + 7, 37), brass),
        ((sx, 40), brass),
        ((sx, 12), wood),
        ((4, 8), iron),
        ((17, 3), iron),
        ((25, 5), wood),
        ((35, 7), wood),
        ((10, 3), brass),
    ]
    .into_iter()
    .map(|(at, style)| Decor::Chandelier { at, style })
    .collect()
}

fn rug(at: (i64, i64), size: (u8, u8), pattern: RugPattern, colour: RugColour) -> Decor {
    Decor::Rug {
        at,
        size,
        rug: Rug { pattern, colour },
    }
}

/// Under each private room's table and chairs, before each front fire, up
/// the foyer and through the archway.
fn ground_rugs() -> Vec<Decor> {
    let colours = RugColour::ALL;
    let mut rugs: Vec<Decor> = (0..ROW_COUNT)
        .map(|n| {
            let at = (2, ROW_FIRST_Y + n * ROW_PITCH);
            let colour = colours[n as usize % colours.len()];
            rug(at, (4, 4), RugPattern::Medallion, colour)
        })
        .collect();
    for fire in FRONT_FIREPLACES {
        let at = (fire.x, fire.y - 2);
        rugs.push(rug(at, (3, 2), RugPattern::Braided, RugColour::Walnut));
    }
    let (ax, ay) = ARCHWAY;
    rugs.push(rug(
        (ax, ay),
        (1, 4),
        RugPattern::Bordered,
        RugColour::Madder,
    ));
    rugs
}

/// Runners along upstairs' legs and crossbar, 2 wide.
fn runners() -> Vec<Decor> {
    let mut rugs = Vec::new();
    for x in [9, 2 * MIDDLE_X - 10] {
        for y in [3, 11, 19, 35, 43] {
            rugs.push(rug((x, y), (2, 4), RugPattern::Striped, RugColour::Indigo));
        }
    }
    for x in [12, MIDDLE_X + 5] {
        rugs.push(rug((x, 28), (4, 2), RugPattern::Striped, RugColour::Indigo));
    }
    rugs
}
