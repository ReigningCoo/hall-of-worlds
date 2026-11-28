//! Noises on a level (`shared::noise`): collected each tick (blows, spells,
//! doors, footsteps), heard by mobs by the shared rule, and the sneaking players
//! a mob could have heard but didn't, who practise Sneaking.

use shared::dungeon::Level;
use shared::id::Id;
use shared::noise::{Hearer, Noise, Source, chance, heard};
use shared::spot::Spot as Units;

use super::{Doing, Mob};
use crate::ServerWorld;
use crate::storage::SiteId;

/// A noise this tick, and the sneaking player whose footsteps it is.
#[derive(Clone, Copy, Debug)]
pub(super) struct Made {
    pub(super) noise: Noise,
    pub(super) sneaker: Option<Id>,
}

/// For each noise made this tick: whether any mob heard it, and whether one
/// that could have (it was in reach) didn't.
#[derive(Debug, Default)]
pub(super) struct Overheard(Vec<(bool, bool)>);

impl Overheard {
    pub(super) fn of(made: &[Made]) -> Self {
        Self(vec![(false, false); made.len()])
    }

    /// The sneakers whose steps went unheard by a mob in reach, and by every
    /// other.
    pub(super) fn unheard(&self, made: &[Made]) -> Vec<Id> {
        (self.0.iter().zip(made))
            .filter(|((heard, missed), _)| !heard && *missed)
            .filter_map(|(_, made)| made.sneaker)
            .collect()
    }
}

/// A noise made at `at` on `place`, for its mobs to hear when they next run.
/// Nothing if no mob is there.
pub(crate) fn make_noise(world: &mut ServerWorld, place: (SiteId, u32), source: Source, at: Units) {
    if let Some(population) = world.populations.0.get_mut(&place) {
        population.noises.push(Noise::new(source, at));
    }
}

/// Every noise this tick: those kept since last tick (doors, spells), blows
/// being wound up (players `fighting`, and mobs), and `steps` taken.
pub(super) fn made_here(
    kept: Vec<Noise>,
    fighting: &[Units],
    mobs: &[Mob],
    steps: Vec<Made>,
) -> Vec<Made> {
    let striking = (mobs.iter())
        .filter(|mob| mob.fight.pending.is_some())
        .map(|mob| mob.spot);
    let blows = (fighting.iter().copied().chain(striking)).map(|at| Noise::new(Source::Blow, at));
    (kept.into_iter().chain(blows))
        .map(|noise| Made {
            noise,
            sneaker: None,
        })
        .chain(steps)
        .collect()
}

impl Mob {
    /// It listens: each noise is heard or not by the shared rule (noted in
    /// `overheard`), and at rest it turns to the nearest heard, through walls
    /// too: then it may see who made it.
    pub(super) fn listen(
        &mut self,
        level: &Level,
        made: &[Made],
        tick: u64,
        overheard: &mut Overheard,
    ) {
        let hearer = Hearer {
            at: self.spot,
            hearing: self.kind.senses().hearing,
            seed: self.id.seed(),
        };
        let mut nearest: Option<Units> = None;
        for (index, made) in made.iter().enumerate() {
            let noise = &made.noise;
            if heard(level, noise, &hearer, tick) {
                overheard.0[index].0 = true;
                let nearer = nearest.is_none_or(|was| {
                    self.spot.distance_squared(noise.at) < self.spot.distance_squared(was)
                });
                if nearer {
                    nearest = Some(noise.at);
                }
            } else if chance(level, noise, &hearer) > 0 {
                overheard.0[index].1 = true;
            }
        }
        if self.doing == Doing::Idle
            && let Some(at) = nearest
        {
            self.face(at);
        }
    }
}
