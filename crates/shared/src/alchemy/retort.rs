//! The Retort step (Alchemy plan): each 100 ml unit of liquid, in order,
//! applies its effect to what came before at full amount (a water as
//! `brewing::watered` does, an oil its row's boost), and adds its volume;
//! drinks and pure alcohol add their alcohol, Salt Water its salt.

use super::batch::Batch;
use super::{Mix, RETORT_UNITS, Refused};
use crate::brewing::oil_suits;
use crate::potion::{Liquid, PURE_STRENGTH, Water};

/// Millilitres in a unit.
pub(super) const UNIT_ML: u32 = 100;

/// Applies these liquids to `batch`, adding their units.
pub(super) fn retort(batch: &mut Batch, liquids: &[Liquid]) -> Result<(), Refused> {
    if liquids.is_empty() {
        return Err(Refused::NoLiquid);
    }
    if liquids.len() > RETORT_UNITS {
        return Err(Refused::TooMuchLiquid);
    }
    for liquid in liquids {
        match liquid {
            Liquid::Water(water) => apply_water(&mut batch.mix, *water),
            Liquid::Oil(oil) => {
                if let Some(row) = oil_suits(*oil) {
                    batch.mix.boost_row(row);
                }
            }
            Liquid::Drink(_) | Liquid::Alcohol => {}
            _ => return Err(Refused::Liquid),
        }
        batch.alcohol += UNIT_ML * liquid.alcohol_strength() / PURE_STRENGTH;
        batch.salt += u32::from(*liquid == Liquid::Water(Water::Salt));
    }
    batch.units += liquids.len() as u32;
    batch.distilled = false;
    Ok(())
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
