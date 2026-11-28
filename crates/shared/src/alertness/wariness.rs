//! `Wariness`: one creature's alertness from tick to tick, what it heard and
//! when it turns to it.

use super::{ALERT_TICKS, Alertness};
use crate::id::Id;
use crate::spot::Spot;

/// A creature's alertness, who it is about, and a noise it is about to turn to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Wariness {
    stage: Alertness,
    /// Who it heard or sees, when known.
    about: Option<Id>,
    /// Where the noise it will turn to was, and the tick it turns.
    turn: Option<(Spot, u64)>,
    /// The tick it is Unaware again, unless it hears or sees more.
    calm_at: u64,
}

impl Wariness {
    pub const fn stage(&self) -> Alertness {
        self.stage
    }

    pub const fn is_unaware(&self) -> bool {
        matches!(self.stage, Alertness::Unaware)
    }

    /// Caught off guard: Unaware, or Alert but not yet turned to what it heard
    /// (its reaction isn't over). A blow on it now is a critical.
    pub const fn off_guard(&self) -> bool {
        match self.stage {
            Alertness::Unaware => true,
            Alertness::Alert => self.turn.is_some(),
            Alertness::Aware => false,
        }
    }

    /// Who it is Alert to, if it is Alert and knows (a player's footsteps).
    pub fn alert_to(&self) -> Option<Id> {
        self.about.filter(|_| self.stage == Alertness::Alert)
    }

    /// Who it is Alert to or Aware of, if known.
    pub const fn about(&self) -> Option<Id> {
        self.about
    }

    /// It heard a noise at `at` (made by `by`, if known) on `tick`: unless it
    /// already sees someone, it is Alert, and turns to the first noise heard
    /// `reaction` ticks later.
    pub fn heard(&mut self, at: Spot, by: Option<Id>, tick: u64, reaction: u64) {
        if self.stage == Alertness::Aware {
            return;
        }
        self.stage = Alertness::Alert;
        self.calm_at = tick + ALERT_TICKS;
        if by.is_some() {
            self.about = by;
        }
        if self.turn.is_none() {
            self.turn = Some((at, tick + reaction));
        }
    }

    /// What it sees on `tick`: someone (`Some`), and it is Aware of them; no
    /// one, and if it was Aware it is Alert now, settling back to Unaware once
    /// `ALERT_TICKS` have gone by with nothing more.
    pub fn sees(&mut self, who: Option<Id>, tick: u64) {
        match who {
            Some(who) => {
                self.stage = Alertness::Aware;
                self.about = Some(who);
                self.turn = None;
                self.calm_at = tick + ALERT_TICKS;
            }
            None if self.stage == Alertness::Aware => {
                self.stage = Alertness::Alert;
                self.calm_at = tick + ALERT_TICKS;
            }
            None if self.stage == Alertness::Alert
                && self.turn.is_none()
                && tick >= self.calm_at =>
            {
                *self = Self::default();
            }
            None => {}
        }
    }

    /// Where it turns to now, once its reaction is over.
    pub fn turn_due(&mut self, tick: u64) -> Option<Spot> {
        let (at, when) = self.turn?;
        (tick >= when).then(|| {
            self.turn = None;
            at
        })
    }
}
