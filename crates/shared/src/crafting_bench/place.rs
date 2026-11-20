//! Which sites have crafting benches and on which levels, and placing them:
//! in a room, back to a plain wall, never cutting anything off. All from
//! hashes of `seed.derive("crafting bench")`, after furniture.

use super::Craft;
use super::spot::spots;
use crate::furniture::place::reach::Reach;
use crate::furniture::place::rooms::{rooms_of, usable_cells};
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::SiteLevel;

/// A site has at most one bench of a craft per this many levels, rounded up.
const LEVELS_A_BENCH: u32 = 5;

/// In 100ths: how much less likely each step of ruin makes a bench to be
/// left (as furniture's).
const RUIN_LOSS_PERCENT: u64 = 15;

/// Purpose numbers for the hashes.
const HAS: i64 = -1;
const LEVEL: i64 = -2;
const LOST: i64 = -3;

/// How likely a site of `kind` is to have benches of `craft`, in percent
/// (Claude's first guesses): Alchemy in every herbalist's hut, 30% of stone
/// towers and 20% of castles.
pub fn bench_percent(craft: Craft, kind: PoiKind) -> u64 {
    match (craft, kind) {
        (Craft::Alchemy, PoiKind::HerbalistHut) => 100,
        (Craft::Alchemy, PoiKind::StoneTower) => 30,
        (Craft::Alchemy, PoiKind::Castle) => 20,
        _ => 0,
    }
}

/// The crafts whose bench is meant for level `index` of a site of `kind`
/// with `levels` levels: if the site has that craft's benches, one on each of
/// ⌈levels ÷ 5⌉ levels chosen by hash. A level with no room for it has none.
pub fn crafts_on_level(kind: PoiKind, seed: Seed, levels: u32, index: u32) -> Vec<Craft> {
    let seed = seed.derive("crafting bench");
    (Craft::ALL.into_iter())
        .filter(|&craft| {
            let c = craft.index() as i64;
            if seed.at(&[c, HAS]) % 100 >= bench_percent(craft, kind) {
                return false;
            }
            let mut order: Vec<u32> = (0..levels).collect();
            order.sort_by_key(|&level| (seed.at(&[c, LEVEL, i64::from(level)]), level));
            order.truncate(levels.div_ceil(LEVELS_A_BENCH) as usize);
            order.contains(&index)
        })
        .collect()
}

/// Places level `index`'s crafting benches (`crafts_on_level`), ruined by
/// `ruin` (0 to 5, each bench left at 100 − 15 × ruin %): the first spot by
/// hash that cuts nothing off, or none.
pub fn place_benches(
    site: &mut SiteLevel,
    kind: PoiKind,
    seed: Seed,
    levels: u32,
    index: u32,
    ruin: u8,
) {
    let crafts = crafts_on_level(kind, seed, levels, index);
    if crafts.is_empty() {
        return;
    }
    let seed = seed.derive("crafting bench");
    let kept = 100u64.saturating_sub(RUIN_LOSS_PERCENT * u64::from(ruin));
    let stairs: Vec<(i64, i64)> = site.stairs.iter().map(|s| (s.x, s.y)).collect();
    let arrivals: Vec<(i64, i64)> = site.entrance.into_iter().chain(stairs.clone()).collect();
    for craft in crafts {
        let c = craft.index() as i64;
        if seed.at(&[c, LOST, i64::from(index)]) % 100 >= kept {
            continue;
        }
        let level = &site.level;
        let rooms = rooms_of(level);
        let usable = usable_cells(level, &rooms, &stairs, &arrivals);
        let mut reach = Reach::new(level, arrivals.clone());
        let hash = |what: &[i64]| seed.at(&[&[c, i64::from(index)], what].concat());
        let placed = (spots(level, &rooms, &usable, craft, hash).into_iter())
            .find(|piece| reach.take(&piece.cells().collect::<Vec<_>>()));
        if let Some(piece) = placed {
            site.level.place_furniture(piece);
        }
    }
}
