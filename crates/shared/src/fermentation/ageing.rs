//! Ageing in the cask (owner: oak raises quality over time up to a cap per
//! drink; cherry adds a little; pine taints; chestnut and acacia do nothing).

use crate::item::Quality;
use crate::material::Material;

use super::quality::stepped;
use super::{DrinkKind, GAME_DAY_SECS};

/// How long `kind` ages in oak for each step of quality, in real seconds:
/// ale 7 game days, cider and perry 14, wines 30. Cherry takes twice as
/// long, and pine taints at this pace.
pub fn ageing_step_secs(kind: DrinkKind) -> u64 {
    let days = match kind {
        DrinkKind::Ale => 7,
        DrinkKind::Cider | DrinkKind::Perry => 14,
        DrinkKind::Wine | DrinkKind::FruitWine => 30,
    };
    days * GAME_DAY_SECS
}

/// The most steps ageing in `wood` moves `kind`: oak up 1 for ale, cider and
/// perry, 2 for fruit wines, 3 for wine; cherry up 1; pine down 2; the rest
/// none.
pub fn ageing_cap(kind: DrinkKind, wood: Material) -> i32 {
    match wood {
        Material::Oak => match kind {
            DrinkKind::Ale | DrinkKind::Cider | DrinkKind::Perry => 1,
            DrinkKind::FruitWine => 2,
            DrinkKind::Wine => 3,
        },
        Material::Cherry => 1,
        Material::Pine => -2,
        _ => 0,
    }
}

/// `made` (the quality it fermented to) after ageing `secs` in a cask of
/// `wood`, from Poor to Legendary.
pub fn aged(made: Quality, kind: DrinkKind, wood: Material, secs: u64) -> Quality {
    let cap = ageing_cap(kind, wood);
    let step = match wood {
        Material::Cherry => ageing_step_secs(kind) * 2,
        _ => ageing_step_secs(kind),
    };
    let steps = (secs / step).min(u64::from(cap.unsigned_abs())) as i32;
    stepped(made, steps * cap.signum())
}

/// How long `kind` ages sealed in glass (a wine bottle or a bottle filled from
/// a cask) for each step of quality: twice oak's pace (Claude's first guess:
/// wine 60 game days a step).
pub fn glass_step_secs(kind: DrinkKind) -> u64 {
    ageing_step_secs(kind) * 2
}

/// The most steps ageing in glass raises `kind` (owner: sealed wine and
/// spirits age anywhere, as the neutral woods would but up, with no wood's
/// flavour; Claude's first guess: one fewer than oak, so wine 2, fruit wines
/// 1, ale, cider and perry none). There are no spirits yet.
pub fn glass_cap(kind: DrinkKind) -> i32 {
    (ageing_cap(kind, Material::Oak) - 1).max(0)
}

/// `bottled` (its quality when it went into the glass) after ageing `secs`
/// sealed in glass, up to Legendary.
pub fn glass_aged(bottled: Quality, kind: DrinkKind, secs: u64) -> Quality {
    let steps = (secs / glass_step_secs(kind)).min(glass_cap(kind) as u64) as i32;
    stepped(bottled, steps)
}
