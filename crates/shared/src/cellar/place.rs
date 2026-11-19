//! Where a castle's or tower's cellar casks stand: one room of its cellar
//! level, against its walls, a cell apart, never by a door, on stairs or
//! where anyone arrives, and never under furniture. All from hashes.

use std::collections::BTreeSet;

use crate::cask::CaskSize;
use crate::dungeon::{Cell, Level};
use crate::furniture::place::rooms::{Room, rooms_of, usable_cells};
use crate::material::Material;
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::{Exit, SiteLevel};

/// Fewest and most casks in the room (fewer if the room has no room).
const FEWEST: u64 = 3;
const MOST: u64 = 8;

/// One cask standing in a cellar: its cell, size and wood.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellarCask {
    pub at: (i64, i64),
    pub size: CaskSize,
    pub wood: Material,
}

/// How likely a site of this kind is to have a cellar of casks, in percent
/// (Claude's first guesses): castles 80, stone towers 50, wooden ones 40.
pub fn cellar_percent(kind: PoiKind) -> u64 {
    match kind {
        PoiKind::Castle => 80,
        PoiKind::StoneTower => 50,
        PoiKind::WoodenTower => 40,
        _ => 0,
    }
}

/// Which level of a site of `kind` with `levels` levels holds its casks, if
/// it has any: its basement (the last level) if it has one, else level 0.
pub fn cellar_level(kind: PoiKind, seed: Seed, levels: u32, has_basement: bool) -> Option<u32> {
    let roll = seed.derive("cellar").at(&[0]) % 100;
    (roll < cellar_percent(kind) && levels > 0).then(|| if has_basement { levels - 1 } else { 0 })
}

/// The sizes cellars hold, by weight: castles' run to the Tun.
fn sizes(kind: PoiKind) -> &'static [(CaskSize, u64)] {
    match kind {
        PoiKind::Castle => &[
            (CaskSize::Barrel, 25),
            (CaskSize::Hogshead, 30),
            (CaskSize::Puncheon, 20),
            (CaskSize::Butt, 15),
            (CaskSize::Tun, 10),
        ],
        _ => &[
            (CaskSize::Kilderkin, 20),
            (CaskSize::Barrel, 30),
            (CaskSize::Hogshead, 30),
            (CaskSize::Puncheon, 15),
            (CaskSize::Butt, 5),
        ],
    }
}

/// The woods, by weight: mostly oak.
const WOODS: [(Material, u64); 5] = [
    (Material::Oak, 45),
    (Material::Chestnut, 20),
    (Material::Acacia, 10),
    (Material::Cherry, 10),
    (Material::Pine, 15),
];

/// The casks on `site`, level `index` of a site of `kind` from its map
/// `seed`, which is its cellar level (`cellar_level`): none if no room has a
/// wall to stand them against.
pub fn cellar_casks(site: &SiteLevel, kind: PoiKind, seed: Seed, index: u32) -> Vec<CellarCask> {
    let seed = seed.derive("cellar");
    let level = &site.level;
    let stairs: Vec<(i64, i64)> = site.stairs.iter().map(|s| (s.x, s.y)).collect();
    let arrivals = if index == 0 {
        vec![site.arrive(Exit::WorldMap)]
    } else {
        Vec::new()
    };
    let rooms = rooms_of(level);
    let usable = usable_cells(level, &rooms, &stairs, &arrivals);
    let hash = |(x, y): (i64, i64)| seed.at(&[i64::from(index), x, y]);
    // The room: of those with three places or more, the one hashed highest.
    let Some(places) = (rooms.iter())
        .map(|room| places(level, room, &usable))
        .filter(|places| places.len() >= FEWEST as usize)
        .max_by_key(|places| hash(places[0]))
    else {
        return Vec::new();
    };
    let wanted = FEWEST + seed.at(&[i64::from(index)]) % (MOST - FEWEST + 1);
    let mut order = places;
    order.sort_by_key(|&cell| (hash(cell), cell));
    let mut chosen: Vec<(i64, i64)> = Vec::new();
    for cell in order {
        let near = |&(x, y): &(i64, i64)| (x - cell.0).abs() <= 1 && (y - cell.1).abs() <= 1;
        if !chosen.iter().any(near) {
            chosen.push(cell);
        }
        if chosen.len() as u64 == wanted {
            break;
        }
    }
    chosen.sort();
    (chosen.into_iter())
        .map(|at| {
            let mut rng = Seed::new(hash(at)).rng();
            CellarCask {
                at,
                size: rng.pick_weighted(sizes(kind)),
                wood: rng.pick_weighted(&WOODS),
            }
        })
        .collect()
}

/// A room's cells a cask may stand on: usable floor against a plain wall,
/// with no furniture, row by row.
fn places(level: &Level, room: &Room, usable: &BTreeSet<(i64, i64)>) -> Vec<(i64, i64)> {
    let by_wall = |(x, y): (i64, i64)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .any(|&(dx, dy)| level.cell(x + dx, y + dy) == Cell::Wall)
    };
    (room.cells.iter().copied())
        .filter(|&(x, y)| {
            usable.contains(&(x, y)) && by_wall((x, y)) && level.furniture_at(x, y).is_none()
        })
        .collect()
}
