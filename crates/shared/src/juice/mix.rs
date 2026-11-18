//! Cordial (owner, drinks plan decision 20): juice and water poured together
//! through the one Pour rule (`shared::pour`). Never more water than juice,
//! so a serving of juice makes at most two of cordial; salt water won't mix.

use crate::potion::{Liquid, Water};

/// What pouring one liquid onto another makes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mixed {
    /// Water poured into juice: cordial, taking no more water than the juice
    /// already there.
    WaterIntoJuice(Liquid),
    /// Juice poured into water: cordial, if at least as much juice as the
    /// water there goes in.
    JuiceIntoWater(Liquid),
}

/// What pouring `poured` into a container holding `held` makes, if the two
/// mix: a juice and a water other than salt water, either way round.
pub fn mix(poured: &Liquid, held: &Liquid) -> Option<Mixed> {
    let water = |liquid: &Liquid| matches!(liquid, Liquid::Water(water) if *water != Water::Salt);
    match (poured, held) {
        (water_poured, Liquid::Juice(fruit)) if water(water_poured) => {
            Some(Mixed::WaterIntoJuice(Liquid::Cordial(fruit.clone())))
        }
        (Liquid::Juice(fruit), water_held) if water(water_held) => {
            Some(Mixed::JuiceIntoWater(Liquid::Cordial(fruit.clone())))
        }
        _ => None,
    }
}
