//! Time passing on casks (owner's decision 2: world time on the ground,
//! playtime in the pack, as yeast's clock): what is fermenting ferments, a
//! sealed drink ages by its wood, an opened drink counts down to vinegar
//! (with a warning in its last tenth) and opened juice or cordial ferments
//! wild into a Poor drink. The server works it out whenever a cask is looked
//! at: carried ones as playtime is counted, ground ones in a player's reach.

use crate::cask::CaskForm;
use crate::fermentation::{
    Base, Drink, GAME_DAY_SECS, JUICE_TURNS_SECS, Mash, WILD_TOLERANCE, aged, best_before_secs,
    ferment, notice_due,
};
use crate::fruit::Fruits;
use crate::item::{Inventory, Item, Quality};
use crate::item_form::ItemForm;
use crate::potion::Liquid;

/// What passing time did to some casks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CaskSettled {
    /// Something its player sees changed: a game day passed on a clock the
    /// popup shows, it finished fermenting, aged a step, turned.
    pub seen: bool,
    /// The names of casks whose drink just came near its end: their player is
    /// told it turns to vinegar soon.
    pub warned: Vec<String>,
}

impl CaskSettled {
    fn and(mut self, other: Self) -> Self {
        self.seen |= other.seen;
        self.warned.extend(other.warned);
        self
    }
}

/// Whether `before` and `after` seconds are in different game days.
fn new_day(before: u64, after: u64) -> bool {
    before / GAME_DAY_SECS != after / GAME_DAY_SECS
}

/// Whole game days left of `secs`, rounded up: what popups show.
pub fn days_left(secs: u64) -> u64 {
    secs.div_ceil(GAME_DAY_SECS)
}

impl CaskForm {
    /// Lets `secs` pass on its clocks.
    pub fn pass(&mut self, secs: u64) -> CaskSettled {
        let mut settled = CaskSettled::default();
        let mut secs = secs;
        if self.time.is_fermenting() {
            let before = self.time.fermenting;
            let used = secs.min(before);
            self.time.fermenting -= used;
            secs -= used;
            settled.seen |= new_day(before, self.time.fermenting) || self.time.fermenting == 0;
        }
        if secs == 0 {
            return settled;
        }
        match self.liquid.clone() {
            Some(Liquid::Drink(drink)) if !self.opened => self.age(*drink, secs, &mut settled),
            Some(Liquid::Drink(drink)) => self.sour(*drink, secs, &mut settled),
            Some(Liquid::Juice(fruit) | Liquid::Cordial(fruit)) if self.opened => {
                self.turn_wild(&fruit, secs, &mut settled)
            }
            _ => {}
        }
        settled
    }

    /// A sealed drink ageing in its wood.
    fn age(&mut self, mut drink: Drink, secs: u64, settled: &mut CaskSettled) {
        let Some(made) = self.time.made else {
            return;
        };
        self.time.aged = self.time.aged.saturating_add(secs);
        let now = aged(made, drink.base.kind(), self.wood, self.time.aged);
        if now != drink.quality {
            drink.quality = now;
            self.liquid = Some(Liquid::Drink(Box::new(drink)));
            settled.seen = true;
        }
    }

    /// An opened drink counting down to vinegar.
    fn sour(&mut self, drink: Drink, secs: u64, settled: &mut CaskSettled) {
        let Some(best) = best_before_secs(drink.strength, drink.vinegar) else {
            return;
        };
        let before = self.time.opened_for;
        self.time.opened_for = before.saturating_add(secs);
        if self.time.opened_for >= best {
            self.liquid = Some(Liquid::Vinegar(Box::new(drink.base)));
            self.time.opened_for -= best;
            self.time.warned = false;
            settled.seen = true;
            return;
        }
        settled.seen |= new_day(before, self.time.opened_for);
        if !self.time.warned && notice_due(best, self.time.opened_for) {
            self.time.warned = true;
            settled.seen = true;
            settled.warned.push(self.name());
        }
    }

    /// Opened juice or cordial fermenting wild once left long enough (drinks
    /// decision 14), always Poor; the time past that counts towards vinegar.
    fn turn_wild(&mut self, fruit: &str, secs: u64, settled: &mut CaskSettled) {
        self.time.opened_for = self.time.opened_for.saturating_add(secs);
        if self.time.opened_for < JUICE_TURNS_SECS {
            return;
        }
        let Some(def) = Fruits::built_in().get(fruit) else {
            return;
        };
        let mash = match self.liquid {
            Some(Liquid::Cordial(_)) => Mash::cordial(def, self.ml),
            _ => Mash::juice(def, self.ml),
        };
        // Too little sugar to ferment: it stays as it is.
        let Some(made) = mash.and_then(|mash| ferment(&mash, WILD_TOLERANCE)) else {
            return;
        };
        let base = Base::Fruit(fruit.to_owned());
        self.liquid = Some(Liquid::Drink(Box::new(Drink::new(
            base,
            &made,
            Quality::Poor,
        ))));
        let past = self.time.opened_for - JUICE_TURNS_SECS;
        self.time.opened_for = 0;
        settled.seen = true;
        let more = self.pass(past);
        settled.seen |= more.seen;
        settled.warned.extend(more.warned);
    }

    /// Seconds until an opened drink turns to vinegar; none if it is sealed,
    /// keeps, or isn't a drink.
    pub fn turns_in(&self) -> Option<u64> {
        let Some(Liquid::Drink(drink)) = &self.liquid else {
            return None;
        };
        let best = best_before_secs(drink.strength, drink.vinegar)?;
        self.opened
            .then(|| best.saturating_sub(self.time.opened_for))
    }
}

impl Item {
    /// Every cask in it: itself and those inside it at any depth (casks never
    /// stack).
    fn each_cask(&mut self, act: &mut impl FnMut(&mut CaskForm)) {
        if let Some(ItemForm::Cask(cask)) = &mut self.form {
            act(cask);
        }
        for placed in &mut self.contents {
            placed.item.each_cask(act);
        }
    }

    /// Carried casks in it let `secs` of playtime pass; one picked up from the
    /// ground first lets the real time pass that it lay there, until `now`
    /// (seconds since 1970), instead.
    pub fn age_casks(&mut self, secs: u32, now: u64) -> CaskSettled {
        let mut settled = CaskSettled::default();
        self.each_cask(&mut |cask| {
            let passed = match cask.time.lying_since.take() {
                Some(since) => now.saturating_sub(since),
                None => u64::from(secs),
            };
            settled = std::mem::take(&mut settled).and(cask.pass(passed));
        });
        settled
    }

    /// Casks in it put on the ground at `now`: their clocks go by real time
    /// from then. One already lying keeps its time.
    pub fn lay_casks_down(&mut self, now: u64) {
        self.each_cask(&mut |cask| {
            cask.time.lying_since.get_or_insert(now);
        });
    }

    /// Casks in it lying on the ground let the real time pass until `now`,
    /// counting from then on.
    pub fn settle_lying_casks(&mut self, now: u64) -> CaskSettled {
        let mut settled = CaskSettled::default();
        self.each_cask(&mut |cask| {
            if let Some(since) = cask.time.lying_since {
                let passed = cask.pass(now.saturating_sub(since));
                settled = std::mem::take(&mut settled).and(passed);
                cask.time.lying_since = Some(since.max(now));
            }
        });
        settled
    }
}

impl Inventory {
    /// Everything carried and worn lets `secs` of playtime pass for its casks
    /// (`Item::age_casks`).
    pub fn age_casks(&mut self, secs: u32, now: u64) -> CaskSettled {
        let carried = self.items.iter_mut().map(|placed| &mut placed.item);
        (carried.chain(self.worn.iter_mut())).fold(CaskSettled::default(), |settled, item| {
            settled.and(item.age_casks(secs, now))
        })
    }
}
