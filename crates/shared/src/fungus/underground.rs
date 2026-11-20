//! Fungi underground (owner, 2026-11-19, decisions 3 to 5): a patch on about
//! one floor cell in `PATCH_ONE_IN` of a cave level, basement or cellar, each
//! a fungus suiting that cell (`place::suited`), from hashes of the site's seed
//! and the cell. What a cell offers:
//!
//! - caves: soil and dung anywhere, rock beside a wall, water within
//!   `WATER_REACH` of a pool, lava beside lava, remains in crypts and on the
//!   deepest level, earth to dig buried kinds from; near daylight (`dim`) on
//!   the top level within `DIM_REACH` of outside ground (a mouth, a skylight)
//!   or of stairs out, where roots and wood come through too;
//! - basements and cellars: wood and soil, `Deep` kinds only.
//!
//! The server's to place (the table is built in, but placing isn't part of any
//! generator's version), on the levels it makes and their cutouts.

use super::{Fungi, FungusPatch, Niche};
use super::{Spot, choose, suited};
use crate::dungeon::{Cell, Level};
use crate::mob::Habitat;
use crate::rng::Seed;
use crate::site::{Exit, SiteLevel};

/// About one floor cell in this many grows a patch.
pub const PATCH_ONE_IN: u64 = 60;

/// How near a pool a cell offers water, in cells.
pub const WATER_REACH: i64 = 2;

/// How near daylight a cell on a cave's top level is dim, in cells.
pub const DIM_REACH: i64 = 4;

/// What kind of underground level it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Underground {
    /// A cave level (`crypt` for crypts' dug ones), `depth` levels down of
    /// `levels`.
    Cave {
        depth: u32,
        levels: u32,
        crypt: bool,
    },
    /// A building's basement or cellar.
    Basement,
}

/// The fungus patches growing on `site_level`, an underground level of this
/// kind, with its land's `habitat` (its temperature counts, not its biome),
/// from `seed` (the site's).
pub fn grow_underground(
    site_level: &SiteLevel,
    fungi: &Fungi,
    kind: Underground,
    habitat: &Habitat,
    seed: Seed,
) -> Vec<((i64, i64), FungusPatch)> {
    let (cells, phases) = (seed.derive("fungus cells"), seed.derive("fungus phases"));
    let level = &site_level.level;
    let stairs: Vec<(i64, i64)> = (site_level.stairs.iter()).map(|s| (s.x, s.y)).collect();
    let daylight: Vec<(i64, i64)> = (site_level.stairs.iter())
        .filter(|s| s.to == Exit::WorldMap)
        .map(|s| (s.x, s.y))
        .collect();
    let (min_x, min_y, width, height) = level.bounds();
    let mut patches = Vec::new();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.cell(x, y) != Cell::Floor || stairs.contains(&(x, y)) {
                continue;
            }
            let mut rng = Seed::new(cells.at(&[x, y])).rng();
            if rng.range_u64(0, PATCH_ONE_IN) != 0 {
                continue;
            }
            let spot = spot_at(level, kind, &daylight, (x, y));
            let Some(def) = choose(&suited(fungi, &spot, habitat), spot.depth, &mut rng) else {
                continue;
            };
            let patch = FungusPatch {
                fungus: def.key.clone(),
                phase: phases.at(&[x, y]) as u32,
                picked: None,
            };
            patches.push(((x, y), patch));
        }
    }
    patches
}

/// What the floor cell `(x, y)` of `level` offers.
pub(super) fn spot_at(
    level: &Level,
    kind: Underground,
    daylight: &[(i64, i64)],
    (x, y): (i64, i64),
) -> Spot {
    let (depth, levels, crypt) = match kind {
        Underground::Basement => {
            return Spot {
                niches: vec![Niche::Wood, Niche::Soil],
                underground: true,
                ..Spot::default()
            };
        }
        Underground::Cave {
            depth,
            levels,
            crypt,
        } => (depth, levels, crypt),
    };
    let near = |reach: i64, wanted: &dyn Fn(Cell) -> bool| {
        (-reach..=reach).any(|dy| (-reach..=reach).any(|dx| wanted(level.cell(x + dx, y + dy))))
    };
    let deepest = depth + 1 == levels;
    let dim = depth == 0
        && (near(DIM_REACH, &Cell::is_outside)
            || (daylight.iter()).any(|&(sx, sy)| (sx - x).abs().max((sy - y).abs()) <= DIM_REACH));
    let mut niches = vec![Niche::Soil, Niche::Dung];
    let beside_wall = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .any(|&(dx, dy)| level.cell(x + dx, y + dy) == Cell::Wall);
    if beside_wall {
        niches.push(Niche::Rock);
    }
    if near(WATER_REACH, &|cell| cell == Cell::Water) {
        niches.push(Niche::Water);
    }
    if near(1, &|cell| cell == Cell::Lava) {
        niches.push(Niche::Lava);
    }
    if crypt || deepest {
        niches.push(Niche::Remains);
    }
    if dim {
        niches.extend([Niche::Roots, Niche::Wood]);
    }
    Spot {
        niches,
        dim,
        earth: true,
        underground: true,
        depth,
        deepest,
    }
}
