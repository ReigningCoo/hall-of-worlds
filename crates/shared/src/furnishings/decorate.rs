//! Decorating: things hanging on rooms' walls by what each room is, rugs in
//! the great hall and bedrooms, and where people live, sconces among the
//! hangings, a chandelier over the great hall and candles on tables.

use super::Setting;
use super::furnisher::Furnisher;
use crate::decor::{ChandelierStyle, Decor, Hanging, Rug, RugColour, RugPattern};
use crate::door::Side;
use crate::dungeon::{Cell, RoomId};
use crate::furniture::{Candles, Furniture, Material, Piece};

/// What the level's rooms are.
pub(super) struct Roles<'a> {
    pub(super) hall: Option<RoomId>,
    pub(super) cellar: Option<RoomId>,
    pub(super) bedrooms: &'a [RoomId],
    /// Whether its lights burn (people live there).
    pub(super) lit: bool,
}

/// The fewest cells a great hall needs for a chandelier.
const CHANDELIER_CELLS: usize = 30;
/// In percent: how likely a table is to have candles, a bedroom or the
/// great hall a rug.
const CANDLES_PERCENT: u64 = 35;
const RUG_PERCENT: u64 = 50;
/// Purpose numbers for the hashes.
const FACE: i64 = 30;
const HANG: i64 = 31;
const CANDLE: i64 = 32;
const RUG: i64 = 33;
const STYLE: i64 = 34;

/// Hangs, lays and lights the level's decor.
pub(super) fn decorate(furnisher: &mut Furnisher, setting: &Setting, roles: Roles) {
    let rooms: Vec<(RoomId, Vec<(i64, i64)>)> = (furnisher.rooms.iter())
        .map(|room| (room.id, room.cells.clone()))
        .collect();
    for (id, cells) in &rooms {
        let hangings = hangings(&roles, *id);
        let count = cells.len().div_ceil(setting.cells_per_hanging);
        hang(furnisher, *id, cells, &hangings, count);
        let rugged = Some(*id) == roles.hall || roles.bedrooms.contains(id);
        if rugged && furnisher.roll(&[i64::from(id.0), RUG]) % 100 < RUG_PERCENT {
            lay_rug(furnisher, *id, cells);
        }
    }
    if !roles.lit {
        return;
    }
    if let Some(hall) = roles.hall
        && let Some((_, cells)) = rooms.iter().find(|(id, _)| *id == hall)
        && cells.len() >= CHANDELIER_CELLS
    {
        hang_chandelier(furnisher, cells);
    }
    light_tables(furnisher);
}

/// What hangs in a room, by weight: sconces only where people live.
fn hangings(roles: &Roles, id: RoomId) -> Vec<(Hanging, u64)> {
    let mut weights = if Some(id) == roles.hall {
        vec![
            (Hanging::Antlers, 3),
            (Hanging::BoarHead, 2),
            (Hanging::Shield, 3),
            (Hanging::CrossedWeapons, 3),
            (Hanging::Tapestry, 3),
        ]
    } else if Some(id) == roles.cellar {
        vec![
            (Hanging::Hams, 3),
            (Hanging::Onions, 3),
            (Hanging::HerbBundles, 2),
        ]
    } else if roles.bedrooms.contains(&id) {
        vec![
            (Hanging::Tapestry, 3),
            (Hanging::CoatPegs, 3),
            (Hanging::Shield, 1),
        ]
    } else {
        vec![
            (Hanging::Shield, 2),
            (Hanging::Tapestry, 2),
            (Hanging::CoatPegs, 1),
            (Hanging::NoticeBoard, 1),
            (Hanging::HerbBundles, 1),
        ]
    };
    if roles.lit {
        weights.push((Hanging::Sconce, 4));
    }
    weights
}

/// Whether something may hang on a wall over `cell`: never behind a tall
/// piece standing against it.
fn clear_in_front(furnisher: &Furnisher, (x, y): (i64, i64)) -> bool {
    let piece = furnisher.site.level.furniture_at(x, y);
    !piece.is_some_and(|piece| {
        matches!(
            piece.furniture,
            Furniture::Fireplace { .. }
                | Furniture::BottleRack { .. }
                | Furniture::CaskRack { .. }
                | Furniture::Wardrobe { .. }
                | Furniture::CraftingBench { .. }
        )
    })
}

/// Hangs up to `count` things on the room's wall faces, two cells apart.
fn hang(
    furnisher: &mut Furnisher,
    id: RoomId,
    cells: &[(i64, i64)],
    weights: &[(Hanging, u64)],
    count: usize,
) {
    let room = i64::from(id.0);
    let level = &furnisher.site.level;
    let mut faces: Vec<(u64, (i64, i64), Side)> = Vec::new();
    for &cell in cells {
        for side in [Side::North, Side::East, Side::South, Side::West] {
            let wall = side.beside(cell);
            if level.cell(wall.0, wall.1) == Cell::Wall && clear_in_front(furnisher, cell) {
                let hash = furnisher.roll(&[room, FACE, wall.0, wall.1, side as i64]);
                faces.push((hash, wall, side.opposite()));
            }
        }
    }
    faces.sort();
    let mut hung: Vec<(i64, i64)> = Vec::new();
    for (hash, wall, face) in faces {
        if hung.len() >= count {
            break;
        }
        let near = |&(x, y): &(i64, i64)| (x - wall.0).abs() <= 1 && (y - wall.1).abs() <= 1;
        if hung.iter().any(near) {
            continue;
        }
        hung.push(wall);
        if !furnisher.is_left(&[HANG, wall.0, wall.1]) {
            continue;
        }
        let mut rng = crate::rng::Seed::new(hash).rng();
        let hanging = rng.pick_weighted(weights);
        let decor = Decor::Wall {
            wall,
            face,
            hanging,
        };
        if decor.fits(&furnisher.site.level) {
            furnisher.site.level.place_decor(decor);
        }
    }
}

/// A rug 2 × 2 or 3 × 2 on free floor of the room, the first that fits.
fn lay_rug(furnisher: &mut Furnisher, id: RoomId, cells: &[(i64, i64)]) {
    let room = i64::from(id.0);
    if !furnisher.is_left(&[RUG, room]) {
        return;
    }
    let roll = furnisher.roll(&[room, RUG, STYLE]);
    let pattern = RugPattern::ALL[(roll % RugPattern::ALL.len() as u64) as usize];
    let colour = RugColour::ALL[(roll / 8 % RugColour::ALL.len() as u64) as usize];
    let size = if (roll / 64).is_multiple_of(2) {
        (3, 2)
    } else {
        (2, 2)
    };
    let mut order: Vec<(u64, (i64, i64))> = (cells.iter())
        .map(|&(x, y)| (furnisher.roll(&[room, RUG, x, y]), (x, y)))
        .collect();
    order.sort();
    for (_, at) in order {
        let covered = (0..i64::from(size.1))
            .flat_map(|dy| (0..i64::from(size.0)).map(move |dx| (at.0 + dx, at.1 + dy)));
        let open = covered.clone().all(|cell| {
            cells.contains(&cell) && furnisher.site.level.furniture_at(cell.0, cell.1).is_none()
        });
        let rug = Decor::Rug {
            at,
            size,
            rug: Rug { pattern, colour },
        };
        if open && rug.fits(&furnisher.site.level) {
            furnisher.site.level.place_decor(rug);
            return;
        }
    }
}

/// A chandelier over the floor cell nearest the hall's middle.
fn hang_chandelier(furnisher: &mut Furnisher, cells: &[(i64, i64)]) {
    let count = cells.len() as i64;
    let (sx, sy) = cells
        .iter()
        .fold((0, 0), |(sx, sy), &(x, y)| (sx + x, sy + y));
    let middle = (sx / count, sy / count);
    let Some(&at) = cells
        .iter()
        .min_by_key(|&&(x, y)| ((x - middle.0).pow(2) + (y - middle.1).pow(2), (x, y)))
    else {
        return;
    };
    let styles = ChandelierStyle::ALL;
    let style = styles[(furnisher.roll(&[STYLE, at.0, at.1]) % styles.len() as u64) as usize];
    let chandelier = Decor::Chandelier { at, style };
    if chandelier.fits(&furnisher.site.level) {
        furnisher.site.level.place_decor(chandelier);
    }
}

/// Candles on some of the level's tables.
fn light_tables(furnisher: &mut Furnisher) {
    let tables: Vec<Piece> = (furnisher.site.level.furniture())
        .filter(|piece| piece.furniture.is_table())
        .copied()
        .collect();
    for table in tables {
        let (x, y) = table.at;
        if furnisher.roll(&[CANDLE, x, y]) % 100 >= CANDLES_PERCENT {
            continue;
        }
        let candles = Candles::ALL[(furnisher.roll(&[CANDLE, STYLE, x, y]) % 3) as usize];
        let wood = match furnisher.wood(&[CANDLE, x, y]) {
            Material::Wood(wood) => wood,
            Material::Stone(_) => continue,
        };
        let piece = Piece {
            at: table.at,
            size: (1, 1),
            facing: table.facing,
            material: Material::Wood(wood),
            furniture: Furniture::Candles { candles },
        };
        furnisher.site.level.place_furniture(piece);
    }
}
