//! Rolling a new vessel, as Give Item and shops make them.

use super::{VesselForm, VesselKind};
use crate::item_form::{Field, Refused, RollRequest};
use crate::potion::{Liquid, LiquidKind};
use crate::rng::Rng;
use crate::site::BuildingKind;

/// The vessel a `RollRequest` asks for: which one (`vessel`) rolled unless
/// asked, empty unless asked for a water or an oil (then full). Refused for a
/// potion (`Field::Liquid`): potions stay in bottles.
pub fn vessel_to(rng: &mut Rng, request: &RollRequest) -> Result<VesselForm, Refused> {
    let vessel = match request.vessel {
        Some(vessel) => vessel,
        None => VesselKind::ALL[rng.range_u64(0, VesselKind::ALL.len() as u64) as usize],
    };
    let empty = VesselForm::new(vessel);
    match request.liquid {
        None => Ok(empty),
        Some(LiquidKind::Water(water)) => Ok(empty.full_of(Liquid::Water(water))),
        Some(LiquidKind::Oil(oil)) => Ok(empty.full_of(Liquid::Oil(oil))),
        Some(LiquidKind::Potion) => Err(Refused(Field::Liquid)),
    }
}

/// The vessels a shop makes (`shop::stock_request`): carpenters' wooden
/// tankards and cups, leatherworkers' leather ones and horns, jewellers'
/// pewter goblets, and herbalists' wine bottles and jugs.
pub fn vessel_for_shop(shop: BuildingKind) -> &'static [VesselKind] {
    use VesselKind::*;
    match shop {
        BuildingKind::Carpenter => &[Tankard, Cup],
        BuildingKind::Leatherworker => &[Flask, Waterskin, DrinkingHorn],
        BuildingKind::Jeweller => &[Goblet],
        BuildingKind::Herbalist => &[WineBottle, Jug],
        _ => &[],
    }
}
