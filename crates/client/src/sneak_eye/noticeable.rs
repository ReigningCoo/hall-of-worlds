//! How noticeable the player is: the best chance any mob they can see has of
//! hearing their next step, per mille.

use shared::creature::CreatureKind;
use shared::dungeon::Level;
use shared::noise::{Footsteps, Hearer, Noise, chance};
use shared::rng::Seed;
use shared::spot::Spot;

/// The most likely of `mobs` (kind and place) to hear `steps` taken at `at`.
pub(super) fn noticeable(
    level: &Level,
    at: Spot,
    steps: Footsteps,
    mobs: impl IntoIterator<Item = (CreatureKind, Spot)>,
) -> u32 {
    let noise = Noise::footsteps(at, steps);
    (mobs.into_iter())
        .map(|(kind, spot)| {
            let hearer = Hearer {
                at: spot,
                hearing: kind.senses().hearing,
                // The chance needs no roll.
                seed: Seed::new(0),
            };
            chance(level, &noise, &hearer)
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests;
