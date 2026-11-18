//! Rolling a new cask, as Give Item and shops make them.

use super::{CaskForm, CaskSize, WOODS};
use crate::item_form::{Field, Refused, RollRequest, keep_only};
use crate::juice::juice_fruit;
use crate::material::Material;
use crate::potion::{Liquid, LiquidKind};
use crate::rng::Rng;

/// How often each size is made: the small ones far more.
const SIZE_WEIGHTS: [(CaskSize, u64); 8] = [
    (CaskSize::Pin, 8),
    (CaskSize::Firkin, 8),
    (CaskSize::Kilderkin, 6),
    (CaskSize::Barrel, 4),
    (CaskSize::Hogshead, 3),
    (CaskSize::Puncheon, 2),
    (CaskSize::Butt, 1),
    (CaskSize::Tun, 1),
];

/// How often each wood is used: oak most, cherry least.
pub fn cask_wood_weights() -> [(Material, u64); 5] {
    let weight = |wood: Material| match wood {
        Material::Oak => 6,
        Material::Chestnut => 3,
        Material::Pine => 2,
        Material::Acacia => 2,
        _ => 1,
    };
    WOODS.map(|wood| (wood, weight(wood)))
}

/// The cask a `RollRequest` asks for: its size (`cask_size`) and wood
/// (`material`, one of `WOODS`) rolled unless asked, empty unless asked for a
/// water, an oil, a juice or a cordial (then full, the fruit rolled). Refused for another material (`Field::Material`)
/// or a potion (`Field::Liquid`).
pub fn cask_to(rng: &mut Rng, request: &RollRequest) -> Result<CaskForm, Refused> {
    let mut sizes = SIZE_WEIGHTS;
    if !keep_only(&mut sizes, request.cask_size) {
        return Err(Refused(Field::CaskSize));
    }
    let mut woods = cask_wood_weights();
    if !keep_only(&mut woods, request.material) {
        return Err(Refused(Field::Material));
    }
    let size = rng.pick_weighted(&sizes);
    let wood = rng.pick_weighted(&woods);
    let cask = CaskForm::new(size, wood);
    match request.liquid {
        None => Ok(cask),
        Some(LiquidKind::Water(water)) => Ok(cask.full_of(Liquid::Water(water))),
        Some(LiquidKind::Oil(oil)) => Ok(cask.full_of(Liquid::Oil(oil))),
        Some(LiquidKind::Juice) => Ok(cask.full_of(Liquid::Juice(juice_fruit(rng)))),
        Some(LiquidKind::Cordial) => Ok(cask.full_of(Liquid::Cordial(juice_fruit(rng)))),
        Some(LiquidKind::Potion) => Err(Refused(Field::Liquid)),
    }
}
