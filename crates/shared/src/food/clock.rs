//! Intermediates' rest clocks (owner, 2026-11-21): as yeast's and casks',
//! playtime while carried and real time on the ground, each second counted
//! at the pace its warmth gives (`warmth_percent`). The server adds carried
//! time as it counts playtime, stamps food put down, and settles what lies
//! in a player's reach at the land's warmth.

use serde::{Deserialize, Serialize};

use super::{CARRIED_CELSIUS, Culture, Curve, Rest, set_percent, turned_quality, warmth_percent};
use crate::item::{Inventory, Item, Quality};
use crate::item_form::ItemForm;
use crate::yeast::SECONDS_PER_HOUR;

/// An intermediate's clock.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestClock {
    pub kind: Rest,
    /// Its quality as made (or as it turned), which its curve moves.
    pub base: Quality,
    /// Rest so far in hundredths of a second at the usual pace: seconds ×
    /// `warmth_percent`.
    pub rested: u64,
    /// When it was put on the ground (seconds since 1970), while it lies
    /// there.
    #[serde(default)]
    pub lying_since: Option<u64>,
    /// The culture put into it, if it ferments.
    #[serde(default)]
    pub culture: Option<Culture>,
}

/// What passing time did to some food: whether its player needs telling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rested {
    /// Some food's quality changed, or it turned into another.
    pub changed: bool,
    /// Some changing food's clock passed a whole hour, which its popup shows.
    pub hour_passed: bool,
}

impl Rested {
    /// Whether anything changed that its player sees.
    pub fn seen(self) -> bool {
        self.changed || self.hour_passed
    }

    fn and(self, other: Self) -> Self {
        Self {
            changed: self.changed || other.changed,
            hour_passed: self.hour_passed || other.hour_passed,
        }
    }
}

impl RestClock {
    /// A clock just started for `kind` made at `base`, with `culture` in it.
    pub fn new(kind: Rest, base: Quality, culture: Option<Culture>) -> Self {
        Self {
            kind,
            base,
            rested: 0,
            lying_since: None,
            culture,
        }
    }

    /// Whole seconds of rest at the usual pace.
    pub fn seconds(&self) -> u32 {
        (self.rested / 100).min(u64::from(u32::MAX)) as u32
    }

    /// Seconds at the usual pace until it turns into another; `None` if it
    /// never does.
    pub fn turns_in(&self) -> Option<u32> {
        match self.kind.curve() {
            Curve::Turns { after, .. } => {
                Some(self.turn_after(after).saturating_sub(self.seconds()))
            }
            _ => None,
        }
    }

    /// When it turns: a ferment at its culture's pace (`set_percent`).
    fn turn_after(&self, after: u32) -> u32 {
        match self.kind.culture() {
            Some(wants) => after * set_percent(self.culture, Some(wants)) / 100,
            None => after,
        }
    }

    /// Its quality now.
    pub fn quality(&self) -> Quality {
        self.kind.curve().quality(self.base, self.seconds())
    }

    /// Lets `secs` pass at `celsius`, setting `quality` to what it now is.
    fn pass(&mut self, quality: &mut Option<Quality>, secs: u64, celsius: i32) -> Rested {
        let curve = self.kind.curve();
        if !curve.changes() {
            return Rested::default();
        }
        let before = self.seconds();
        let was = (*quality, self.kind);
        self.rested = self
            .rested
            .saturating_add(secs * u64::from(warmth_percent(celsius)));
        if let Curve::Turns { after, into } = curve
            && self.seconds() >= self.turn_after(after)
        {
            if let Some(wants) = self.kind.culture() {
                self.base = turned_quality(self.base, self.culture, Some(wants));
            }
            self.kind = into;
            self.rested = 0;
            self.culture = None;
        }
        *quality = Some(self.quality());
        Rested {
            changed: (*quality, self.kind) != was,
            hour_passed: before / SECONDS_PER_HOUR != self.seconds() / SECONDS_PER_HOUR,
        }
    }
}

impl Item {
    /// Every resting food in it: itself, those stacked on it, those inside.
    fn each_rest(&mut self, act: &mut impl FnMut(&mut Option<Quality>, &mut RestClock)) {
        if let Some(ItemForm::Food(food)) = &mut self.form
            && let Some(rest) = &mut food.rest
        {
            act(&mut self.quality, rest);
        }
        for above in &mut self.above {
            above.each_rest(act);
        }
        for placed in &mut self.contents {
            placed.item.each_rest(act);
        }
    }

    /// Carried food in it rests `secs` of playtime at `celsius`; one picked up
    /// from the ground rests the real time it lay there, until `now`, instead.
    pub fn rest_food(&mut self, secs: u32, now: u64, celsius: i32) -> Rested {
        let mut rested = Rested::default();
        self.each_rest(&mut |quality, rest| {
            let passed = match rest.lying_since.take() {
                Some(since) => now.saturating_sub(since),
                None => u64::from(secs),
            };
            rested = rested.and(rest.pass(quality, passed, celsius));
        });
        rested
    }

    /// Food in it put on the ground at `now`: its clock runs in real time.
    pub fn lay_food_down(&mut self, now: u64) {
        self.each_rest(&mut |_, rest| {
            rest.lying_since.get_or_insert(now);
        });
    }

    /// Food in it lying on the ground rests the real time until `now` at
    /// `celsius`, counting from then on.
    pub fn settle_lying_food(&mut self, now: u64, celsius: i32) -> Rested {
        let mut rested = Rested::default();
        self.each_rest(&mut |quality, rest| {
            if let Some(since) = rest.lying_since {
                rested = rested.and(rest.pass(quality, now.saturating_sub(since), celsius));
                rest.lying_since = Some(since.max(now));
            }
        });
        rested
    }
}

impl Inventory {
    /// Everything carried and worn rests `secs` of playtime at
    /// `CARRIED_CELSIUS` (the bin's is left behind).
    pub fn rest_food(&mut self, secs: u32, now: u64) -> Rested {
        let carried = self.items.iter_mut().map(|placed| &mut placed.item);
        (carried.chain(self.worn.iter_mut())).fold(Rested::default(), |rested, item| {
            rested.and(item.rest_food(secs, now, CARRIED_CELSIUS))
        })
    }
}
