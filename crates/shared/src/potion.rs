//! Bottles (owner, 2026-11-17): one item kind, `ItemKind::Bottle`, empty or
//! holding a `Liquid`: a water, an oil, or a potion's `Contents` (effects on
//! `herb::Stat`s, each a signed step); with a look seed the client picks the
//! bottle's shape, glass and stopper from. Bottles stack by liquid, potions
//! also by quality, whatever they look like. Press Oil fills them with
//! oil (`press.rs`); brewing and drinking are `shared::brewing`'s
//! (`docs/context/brewing.md`). A potion goes on the action bar by its
//! `PotionKey` (`key.rs`). See `docs/context/shared/potion.md`.

mod carried;
mod collect;
mod contents;
mod form;
mod key;
mod liquid;
mod press;
mod roll;

pub use collect::{
    COLLECT_PRACTICE, COLLECT_REACH, FACING_HALF, RAIN_AT, Source, WATER_SKILL, catches_rain,
    fill_seconds, fill_ticks, filled, holds_water, water_faced, water_from,
};
pub use contents::{Contents, Dose};
pub use form::{BottleForm, EMPTY_GRAMS, POTION_GRAMS};
pub use key::PotionKey;
pub use liquid::{Liquid, LiquidKind, Oil, PURE_STRENGTH, Water};
pub use press::{
    PRESS_PRACTICE, PressChoice, Pressing, grams_per_bottle, press_seconds, press_ticks, pressed,
};
pub use roll::{MOST_EFFECTS, bottle_to, roll_contents};

#[cfg(test)]
mod collect_tests;
#[cfg(test)]
mod key_tests;
#[cfg(test)]
mod liquid_tests;
#[cfg(test)]
mod press_tests;
#[cfg(test)]
mod tests;
