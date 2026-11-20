//! `Later`: effects laid on now that start later (a fungus's delayed poison,
//! fungi plan PR 6). Until its `start` an effect waits here, counting for
//! nothing; then it joins `Effects` (`Sheet::forget_ended` moves it, settling
//! Health and Mana at that tick). One not `known` is hidden from its player
//! (`Sheet::as_sent` leaves it out): it shows only once it starts.

use serde::{Deserialize, Serialize};

use super::effect::{Effect, Source};

/// An effect waiting to start, and whether its player knows it is coming.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pending {
    pub effect: Effect,
    /// Recognised (owner, fungi decision 8): sent to the client, which may
    /// show it before it starts. Never a guess: an unknown one is left out.
    pub known: bool,
}

/// Every effect waiting to start on one creature, in no order. Several may
/// share a source (a poison's phases), each starting after the one before
/// has ended.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Later(pub Vec<Pending>);

impl Later {
    /// Lays one on to start at its `start`.
    pub fn add(&mut self, effect: Effect, known: bool) {
        self.0.push(Pending { effect, known });
    }

    /// The earliest start at or before `now`, of one still waiting.
    pub fn first_due(&self, now: u64) -> Option<u64> {
        (self.0.iter())
            .map(|p| p.effect.start)
            .filter(|&start| start <= now)
            .min()
    }

    /// The earliest start after `now`, if any is waiting.
    pub fn next_start(&self, now: u64) -> Option<u64> {
        (self.0.iter())
            .map(|p| p.effect.start)
            .filter(|&start| start > now)
            .min()
    }

    /// Takes out those starting at or before `tick`, earliest first.
    pub fn take_due(&mut self, tick: u64) -> Vec<Pending> {
        let (mut due, waiting) = (std::mem::take(&mut self.0).into_iter())
            .partition::<Vec<_>, _>(|p| p.effect.start <= tick);
        self.0 = waiting;
        due.sort_by_key(|p| p.effect.start);
        due
    }

    /// Marks every one from `source` known.
    pub fn know(&mut self, source: Source) {
        (self.0.iter_mut())
            .filter(|p| p.effect.source == source)
            .for_each(|p| p.known = true);
    }

    /// Forgets every one from a source this says (dying cures poisons).
    pub fn forget(&mut self, which: impl Fn(Source) -> bool) {
        self.0.retain(|p| !which(p.effect.source));
    }

    /// Only those known: what its player is sent.
    pub fn known(&self) -> Self {
        Self(self.0.iter().filter(|p| p.known).cloned().collect())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
