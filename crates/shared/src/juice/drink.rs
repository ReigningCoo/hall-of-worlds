//! What drinking juice or cordial does (owner, drinks plan decisions 19 and
//! 20): Mana a second for a while, by how much is drunk. A serving of juice
//! (`vessel::SERVING_ML`, 250 ml) gives 1 Mana a second for 30 s; cordial,
//! half juice and half water, 1 a second for 20 s a serving, so the two
//! servings poured from one of juice give a little more than the juice.

use crate::bonus::{self, Bonus, Effect, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fruit::FruitTag;
use crate::item::Item;
use crate::potion::Liquid;
use crate::vessel::SERVING_ML;
use crate::vitals::{PER_POINT, Vital};

/// Mana restored a second while it lasts, in thousandths of a point.
pub const MANA_A_SECOND: u32 = PER_POINT;
/// How long a serving of juice restores Mana, in seconds.
pub const JUICE_SECONDS: u32 = 30;
/// How long a serving of cordial restores Mana, in seconds.
pub const CORDIAL_SECONDS: u32 = 20;
/// How many times juice's Mana a second a fermented drink gives.
pub const DRINK_TIMES: u32 = 2;

/// The effect of drinking `ml` of `liquid` from `now`: Mana restored for its
/// serving's seconds times the share of a serving drunk (a bottle, 100 ml of
/// juice: 12 s), in whole seconds rounded down. None for anything else (water
/// does nothing), or too little to last a second. Nothing ends it early, and
/// the latest drink's replaces any earlier one (`Source::Juice`, `Cordial`).
pub fn drink_effect(liquid: &Liquid, ml: u32, now: u64) -> Option<Effect> {
    let (source, seconds, per_second) = match liquid {
        Liquid::Juice(fruit) => (Source::Juice(FruitTag::of(fruit)), JUICE_SECONDS, 1),
        Liquid::Cordial(fruit) => (Source::Cordial(FruitTag::of(fruit)), CORDIAL_SECONDS, 1),
        // Twice its juice's effect (drinks decision 21), whatever it was made
        // from, plus its units (`alcohol::units_in`, the drinker's).
        Liquid::Drink(_) => (Source::Drink, JUICE_SECONDS, DRINK_TIMES),
        _ => return None,
    };
    let seconds = u64::from(seconds) * u64::from(ml) / u64::from(SERVING_ML);
    if seconds == 0 {
        return None;
    }
    Some(Effect {
        source,
        bonuses: vec![Bonus::flat(
            bonus::Stat::Restore(Vital::Mana),
            (MANA_A_SECOND * per_second) as i32,
        )],
        start: now,
        end: Some(now + seconds * ACTION_TICKS_PER_SECOND as u64),
        ends_on: EndsOn::NOTHING,
    })
}

impl Liquid {
    /// Whether it can be drunk: a water, a juice, a cordial or a fermented
    /// drink (potions are drunk their own way, `brewing::drunk`; oil and
    /// vinegar can't be).
    pub fn is_drink(&self) -> bool {
        matches!(
            self,
            Self::Water(_) | Self::Juice(_) | Self::Cordial(_) | Self::Drink(_)
        )
    }
}

impl Item {
    /// Whether this is a bottle of something drunk whole and given back
    /// empty, a water, juice, cordial or drink (not a potion).
    pub fn is_drinkable_bottle(&self) -> bool {
        self.bottle()
            .and_then(|bottle| bottle.liquid.as_ref())
            .is_some_and(Liquid::is_drink)
    }
}
