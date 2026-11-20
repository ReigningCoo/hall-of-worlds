//! Fungi on felled trees (owner, 2026-11-19, decision 4): wood-feeders suiting
//! the land grow on some wild trees' cells (`Cell::Tree`), shown only while
//! the felled tree is cleared (a day after felling until a sapling comes up,
//! `dungeon::Level::fungus_at`). Rolled for every tree from hashes of its
//! cell, so felling one needs nothing more kept.

use super::choose;
use super::{Fungi, FungusPatch};
use crate::mob::Habitat;
use crate::rng::Seed;

/// The share of felled trees, in percent, that grow a fungus once cleared.
pub const FELLED_PERCENT: u64 = 40;

/// The patch that grows on the tree at `(x, y)` once it is felled and
/// cleared, if any: a wood-feeder growing above ground in `habitat`'s biome
/// and warmth (wood-feeders found underground too), from `seed`.
pub fn felled_patch(
    fungi: &Fungi,
    habitat: &Habitat,
    seed: Seed,
    (x, y): (i64, i64),
) -> Option<FungusPatch> {
    let mut rng = Seed::new(seed.derive("felled fungus").at(&[x, y])).rng();
    if rng.range_u64(0, 100) >= FELLED_PERCENT {
        return None;
    }
    let celsius = habitat.celsius.clamp(i16::MIN.into(), i16::MAX.into()) as i16;
    let suited: Vec<_> = (fungi.iter())
        .filter(|def| {
            def.feeds_on_wood()
                && def.biomes.contains(&habitat.biome)
                && def.temperature.suits(celsius)
        })
        .collect();
    let def = choose(&suited, 0, &mut rng)?;
    Some(FungusPatch {
        fungus: def.key.clone(),
        phase: seed.derive("felled fungus phases").at(&[x, y]) as u32,
        picked: None,
    })
}
