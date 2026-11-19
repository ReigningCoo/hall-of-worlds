//! Yeast's clock (owner, 2026-11-18): it goes by playtime while carried
//! (paused while logged out) and by real time while on the ground. The server
//! adds playtime to what is carried as it counts it, stamps yeast put on the
//! ground with the time, and counts the time it lay there once it is carried
//! again.

use crate::item::{Inventory, Item, Quality};

use super::form::{WEAKENS_EVERY, YeastForm};

/// Seconds in an hour: the client is told of a carried yeast's clock each hour.
pub const SECONDS_PER_HOUR: u32 = 3600;

/// What passing time did to some yeast: whether its player needs telling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Settled {
    /// Some yeast lost a quality step.
    pub weakened: bool,
    /// Some yeast's clock passed a whole hour, which its popup shows.
    pub hour_passed: bool,
}

impl Settled {
    /// Whether anything changed that its player sees.
    pub fn seen(self) -> bool {
        self.weakened || self.hour_passed
    }

    fn and(self, other: Self) -> Self {
        Self {
            weakened: self.weakened || other.weakened,
            hour_passed: self.hour_passed || other.hour_passed,
        }
    }
}

impl YeastForm {
    /// Lets `secs` pass unfed, losing a quality step each `WEAKENS_EVERY`.
    fn pass(&mut self, quality: &mut Option<Quality>, secs: u64) -> Settled {
        // A starter still developing counts that down first.
        let developing = u64::from(self.developing).min(secs);
        self.developing -= developing as u32;
        let secs = secs - developing;
        let ready = developing > 0 && self.developing == 0;
        let before = self.unfed;
        self.unfed = (u64::from(before) + secs).min(u64::from(u32::MAX)) as u32;
        let steps = self.unfed / WEAKENS_EVERY - before / WEAKENS_EVERY;
        let was = *quality;
        for _ in 0..steps {
            *quality = quality.map(Quality::worse);
        }
        Settled {
            weakened: *quality != was,
            hour_passed: ready
                || self.unfed / SECONDS_PER_HOUR != before / SECONDS_PER_HOUR
                || (developing > 0 && passed_hour(self.developing, developing as u32)),
        }
    }
}

/// Whether counting `by` down to `left` passed a whole hour of what was left.
fn passed_hour(left: u32, by: u32) -> bool {
    (left + by) / SECONDS_PER_HOUR != left / SECONDS_PER_HOUR
}

impl Item {
    /// Every yeast in it: itself, those stacked on it, and those inside it at
    /// any depth.
    fn each_yeast(&mut self, act: &mut impl FnMut(&mut Option<Quality>, &mut YeastForm)) {
        if let Some(crate::item_form::ItemForm::Yeast(yeast)) = &mut self.form {
            act(&mut self.quality, yeast);
        }
        for above in &mut self.above {
            above.each_yeast(act);
        }
        for placed in &mut self.contents {
            placed.item.each_yeast(act);
        }
    }

    /// Carried yeast in it lets `secs` of playtime pass. One picked up from the
    /// ground first lets the real time pass that it lay there, until `now`
    /// (seconds since 1970), instead.
    pub fn age_yeast(&mut self, secs: u32, now: u64) -> Settled {
        let mut settled = Settled::default();
        self.each_yeast(&mut |quality, yeast| {
            let passed = match yeast.lying_since.take() {
                Some(since) => now.saturating_sub(since),
                None => u64::from(secs),
            };
            settled = settled.and(yeast.pass(quality, passed));
        });
        settled
    }

    /// Yeast in it put on the ground at `now` (seconds since 1970): its clock
    /// runs in real time from then. One already lying keeps its time.
    pub fn lay_yeast_down(&mut self, now: u64) {
        self.each_yeast(&mut |_, yeast| {
            yeast.lying_since.get_or_insert(now);
        });
    }

    /// Yeast in it lying on the ground lets the real time pass until `now`
    /// (seconds since 1970), counting from then on: for the ground's piles,
    /// whenever they are looked at.
    pub fn settle_lying_yeast(&mut self, now: u64) -> Settled {
        let mut settled = Settled::default();
        self.each_yeast(&mut |quality, yeast| {
            if let Some(since) = yeast.lying_since {
                settled = settled.and(yeast.pass(quality, now.saturating_sub(since)));
                yeast.lying_since = Some(since.max(now));
            }
        });
        settled
    }
}

impl Inventory {
    /// Everything carried and worn lets `secs` of playtime pass for its yeast
    /// (`Item::age_yeast`). The bin's is left behind, as it weighs nothing.
    pub fn age_yeast(&mut self, secs: u32, now: u64) -> Settled {
        let carried = self.items.iter_mut().map(|placed| &mut placed.item);
        (carried.chain(self.worn.iter_mut())).fold(Settled::default(), |settled, item| {
            settled.and(item.age_yeast(secs, now))
        })
    }
}
