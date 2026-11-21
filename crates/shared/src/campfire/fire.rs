//! One campfire's fuel: the pieces of kindling fed to it, burning one after
//! another by the server's clock (real seconds, stopped while paused).

use serde::{Deserialize, Serialize};

use super::{MAX_KINDLING, burn_seconds};
use crate::material::Material;

/// A campfire: when the first piece still in `burns` began burning, and how
/// long each piece fed burns, in order. Pieces burnt away are dropped from
/// `burns` and counted in `burnt` (`settle`), so a fire fed for hours stays
/// small.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Campfire {
    /// Server seconds when `burns[0]` caught.
    pub since: u64,
    /// Seconds each piece burns (`burn_seconds`), the one burning first.
    pub burns: Vec<u32>,
    /// Pieces burnt away before `burns[0]`.
    #[serde(default)]
    pub burnt: u32,
}

/// Why a piece of kindling wasn't put on a fire.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedRefused {
    /// It has burnt out.
    Out,
    /// It holds `MAX_KINDLING` unburnt already.
    Full,
}

impl Campfire {
    /// A fire lit at `now` with a piece of `wood`'s kindling.
    pub fn lit(now: u64, wood: Material) -> Self {
        Self {
            since: now,
            burns: vec![burn_seconds(wood)],
            burnt: 0,
        }
    }

    /// When it burns out (or burnt out), in server seconds.
    pub fn ends(&self) -> u64 {
        self.since + self.burns.iter().map(|&s| u64::from(s)).sum::<u64>()
    }

    /// Whether it is still burning at `now`.
    pub fn is_lit(&self, now: u64) -> bool {
        now < self.ends()
    }

    /// How many pieces haven't burnt away by `now`, the one burning included.
    pub fn pieces_left(&self, now: u64) -> usize {
        let mut end = self.since;
        let mut burnt = 0;
        for &burn in &self.burns {
            end += u64::from(burn);
            if end <= now {
                burnt += 1;
            }
        }
        self.burns.len() - burnt
    }

    /// Puts a piece of `wood`'s kindling on, burning once the rest have: only
    /// while lit, and while it holds fewer than `MAX_KINDLING`.
    pub fn feed(&mut self, now: u64, wood: Material) -> Result<(), FeedRefused> {
        if !self.is_lit(now) {
            return Err(FeedRefused::Out);
        }
        if self.pieces_left(now) >= MAX_KINDLING {
            return Err(FeedRefused::Full);
        }
        self.settle(now);
        self.burns.push(burn_seconds(wood));
        Ok(())
    }

    /// Drops the pieces burnt away by `now` from `burns`, counting them in
    /// `burnt`; the last stays, so `ends` is kept.
    pub fn settle(&mut self, now: u64) {
        while self.burns.len() > 1 {
            let first = u64::from(self.burns[0]);
            if self.since + first > now {
                break;
            }
            self.since += first;
            self.burns.remove(0);
            self.burnt += 1;
        }
    }

    /// How many Ash items it leaves: one for every piece fed.
    pub fn ash(&self) -> u32 {
        self.burnt + self.burns.len() as u32
    }
}
