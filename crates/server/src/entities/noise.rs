//! Noises on a level (`shared::noise`): collected each tick (blows, spells,
//! doors, footsteps), heard by mobs by the shared rule, and the sneaking players
//! a mob could have heard but didn't, who practise Sneaking.

use shared::behaviour::Behaviours;
use shared::dungeon::Level;
use shared::id::Id;
use shared::noise::{Hearer, Noise, Source, chance, heard};
use shared::spot::Spot as Units;

use super::choice::Other;
use super::curious::kin_call;
use super::{Doing, Mob};
use crate::ServerWorld;
use crate::storage::SiteId;

/// A noise this tick, the player whose footsteps it is (`by`), and whether
/// they sneak (`sneaker`).
#[derive(Clone, Copy, Debug)]
pub(super) struct Made {
    pub(super) noise: Noise,
    pub(super) by: Option<Id>,
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
        population.noises.push(Made {
            noise: Noise::new(source, at),
            by: None,
            sneaker: None,
        });
    }
}

/// Every noise this tick: those kept since last tick (doors, spells), blows
/// being wound up (players `fighting`, and mobs), and `steps` taken.
pub(super) fn made_here(
    kept: Vec<Made>,
    fighting: &[Units],
    mobs: &[Mob],
    steps: Vec<Made>,
) -> Vec<Made> {
    let striking = (mobs.iter())
        .filter(|mob| mob.fight.pending.is_some())
        .map(|mob| mob.spot);
    let blows = (fighting.iter().copied().chain(striking)).map(|at| Made {
        noise: Noise::new(Source::Blow, at),
        by: None,
        sneaker: None,
    });
    (kept.into_iter().chain(blows)).chain(steps).collect()
}

impl Mob {
    /// It listens: each noise is heard or not by the shared rule (noted in
    /// `overheard`), and the nearest heard makes it Alert (`alert.rs`): once
    /// its reaction is over, at rest, it turns to it, through walls too, and
    /// may see who made it, or else go to look (`curious.rs`). A call from its
    /// own band it answers at once. Its own noises it doesn't listen to.
    /// Whether it went from rest to doing something.
    pub(super) fn listen(
        &mut self,
        level: &Level,
        made: &[Made],
        (others, behaviours): (&[Other], &Behaviours),
        tick: u64,
        overheard: &mut Overheard,
    ) -> bool {
        let hearer = Hearer {
            at: self.spot,
            hearing: self.kind.senses().hearing,
            seed: self.id.seed(),
        };
        let mut heard_now = Vec::new();
        for (index, made) in made.iter().enumerate() {
            if made.by == Some(self.id) {
                continue;
            }
            let noise = &made.noise;
            if heard(level, noise, &hearer, tick) {
                overheard.0[index].0 = true;
                heard_now.push(index);
            } else if chance(level, noise, &hearer) > 0 {
                overheard.0[index].1 = true;
            }
        }
        let nearest = (heard_now.iter().map(|&index| &made[index]))
            .min_by_key(|made| self.spot.distance_squared(made.noise.at));
        if let Some(made) = nearest {
            self.wariness
                .heard(made.noise.at, made.by, tick, self.reaction());
        }
        let at_rest = self.doing == Doing::Idle;
        if let Some(call) = kin_call(self, made, &heard_now, (others, behaviours)) {
            return self.answer(call.noise.at, tick) && at_rest;
        }
        if let Some(at) = self.wariness.turn_due(tick)
            && at_rest
        {
            self.face(at);
            let curiosity = behaviours.of(self.kind).drives.curiosity;
            return self.wonder(level, at, curiosity, tick);
        }
        false
    }
}
