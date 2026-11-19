//! The Retort step (Alchemy plan): each 100 ml unit of liquid, in order,
//! applies its effect to what came before at full amount (a water as
//! `brewing::watered` does, an oil its row's boost), and adds its volume.

use super::{Mix, RETORT_UNITS, Refused};
use crate::brewing::oil_suits;
use crate::potion::{Liquid, Water};

/// Applies these liquids to `mix`: the units they add.
pub(super) fn retort(mix: &mut Mix, liquids: &[Liquid]) -> Result<u32, Refused> {
    if liquids.is_empty() {
        return Err(Refused::NoLiquid);
    }
    if liquids.len() > RETORT_UNITS {
        return Err(Refused::TooMuchLiquid);
    }
    for liquid in liquids {
        match liquid {
            Liquid::Water(water) => apply_water(mix, *water),
            Liquid::Oil(oil) => {
                if let Some(row) = oil_suits(*oil) {
                    mix.boost_row(row);
                }
            }
            _ => return Err(Refused::Liquid),
        }
    }
    Ok(liquids.len() as u32)
}

/// One unit of a water (brewing decision 2 and the Alchemy plan's Rain Water).
fn apply_water(mix: &mut Mix, water: Water) {
    match water {
        Water::Plain => {}
        Water::Cave => mix.boost_way(false),
        Water::Moon => mix.boost_way(true),
        Water::Salt => mix.remove_strongest(true),
        Water::Rain => mix.remove_strongest(false),
    }
}
