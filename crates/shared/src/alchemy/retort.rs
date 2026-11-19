//! The Retort step (Alchemy plan): each 100 ml unit of liquid, in order,
//! applies its effect to what came before at full amount (a water as
//! `brewing::watered` does, an oil its row's boost), and adds its volume;
//! drinks and pure alcohol add their alcohol, Salt Water its salt. Then the
//! solid, if any (`solid`).

use super::batch::Batch;
use super::solid::apply_solid;
use super::transform::transformation;
use super::{Mix, RETORT_UNITS, Refused, Solid};
use crate::brewing::oil_suits;
use crate::potion::{Liquid, PURE_STRENGTH, Water};

/// Millilitres in a unit.
pub(super) const UNIT_ML: u32 = 100;

/// Applies these liquids to `batch`, adding their units, then the solid.
pub(super) fn retort(
    batch: &mut Batch,
    liquids: &[Liquid],
    solid: Option<Solid>,
) -> Result<(), Refused> {
    if liquids.is_empty() && (solid.is_none() || batch.is_empty()) {
        return Err(Refused::NoLiquid);
    }
    if let Some(Solid::Vegetable(tag)) = solid
        && transformation(tag).is_none()
    {
        return Err(Refused::Solid);
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
    if let Some(solid) = solid {
        apply_solid(&mut batch.mix, solid);
    }
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
