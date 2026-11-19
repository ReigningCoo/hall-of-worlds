//! Give Item's drink and vinegar: a juicable fruit's, fermented with one
//! Common yeast, the same on both sides from the same seed.

use crate::fruit::Fruits;
use crate::item::Quality;
use crate::juice::juice_fruit;
use crate::potion::Liquid;
use crate::rng::Rng;

use super::{Base, Drink, Mash, ferment, tolerance};

/// A drink of a rolled fruit, as its juice ferments with a Common yeast
/// (Common); its plain juice if that is too weak to ferment.
pub fn rolled_drink(rng: &mut Rng) -> Liquid {
    let fruit = juice_fruit(rng);
    let made = (Fruits::built_in().get(&fruit))
        .and_then(|def| Mash::juice(def, 1000))
        .and_then(|mash| ferment(&mash, tolerance(Quality::Common)));
    match made {
        Some(made) => Liquid::Drink(Box::new(Drink::new(
            Base::Fruit(fruit),
            &made,
            Quality::Common,
        ))),
        None => Liquid::Juice(fruit),
    }
}

/// A rolled fruit's vinegar.
pub fn rolled_vinegar(rng: &mut Rng) -> Liquid {
    Liquid::Vinegar(Box::new(Base::Fruit(juice_fruit(rng))))
}
