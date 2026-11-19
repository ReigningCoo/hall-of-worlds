//! Collect Water (brewing plan, decision 5): which water a bottle is filled
//! with, where someone faces water near enough to fill from, when rain can be
//! caught instead (Alchemy plan), how long a bottle takes, and which empty
//! bottle is filled next. The server decides; the client predicts with the
//! same rules.

use super::{BottleForm, Water};
use crate::dungeon::{Cell, Level};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::facing::{EIGHTH, Heading, faced_cell};
use crate::herb::{cube_root, skill_percent, skill_value};
use crate::id::Id;
use crate::item::{Inventory, Item};
use crate::skill::Skills;
use crate::spot::Spot;

/// The Water skill's path in the skill tree, practised by Collect Water.
pub const WATER_SKILL: &str = "gathering/water";

/// How near a water cell's middle has to be, in units: as near as a shop's
/// door (`shop::DOOR_REACH`).
pub const COLLECT_REACH: u64 = crate::shop::DOOR_REACH;

/// How far to either side of straight ahead water still counts as faced: 45°.
pub const FACING_HALF: u16 = EIGHTH;

/// How long the first bottle takes untrained, in seconds.
const FIRST_SECONDS: u32 = 10;

/// Water practice for one bottle filled (as much as one herb picked gives
/// Foraging).
pub const COLLECT_PRACTICE: u32 = 8;

/// Where the water is drawn from, as far as which water it is goes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Source {
    /// Under the ground: a cave, or a basement.
    pub underground: bool,
    /// The sea, not a river or a pool.
    pub sea: bool,
    /// Out in the open with no sun in the sky and at least one moon.
    pub moonlit: bool,
}

/// The water a bottle is filled with: Cave Water underground, Salt Water from
/// the sea, Moon Water by moonlight, otherwise Water.
pub fn water_from(source: Source) -> Water {
    match source {
        Source {
            underground: true, ..
        } => Water::Cave,
        Source { sea: true, .. } => Water::Salt,
        Source { moonlit: true, .. } => Water::Moon,
        _ => Water::Plain,
    }
}

/// How hard it has to rain (`weather::WeatherSample::rain`, 0 to 1) for
/// Collect Water to catch Rain Water: steady rain, not a drizzle.
pub const RAIN_AT: f64 = 0.25;

/// Whether rain can be caught where someone stands: outside on their own cell
/// (not under a roof, not underground), and raining there at least `RAIN_AT`
/// warm enough not to be sleet or snow (`weather::LIGHTNING_WARMEST`, the
/// shaders' sleet line; `temperature` as `climate` gives it at sea level, as
/// the drawn rain is). Water faced still wins over rain: the caller asks this
/// only when `water_faced` finds none.
pub fn catches_rain(outside: bool, rain: f64, temperature: f64) -> bool {
    outside && rain >= RAIN_AT && temperature >= crate::weather::LIGHTNING_WARMEST
}

/// Whether a cell can be filled from: water, deep or shallow.
pub fn holds_water(cell: Cell) -> bool {
    matches!(cell, Cell::Water | Cell::Shallows)
}

/// The water cell someone at `at` facing `heading` would fill from, the
/// nearest: its middle within `COLLECT_REACH` and within `FACING_HALF` of
/// straight ahead (or underfoot, wading in it).
pub fn water_faced(level: &Level, at: Spot, heading: Heading) -> Option<(i64, i64)> {
    let water = |level: &Level, x, y| holds_water(level.cell(x, y));
    faced_cell(level, at, heading, COLLECT_REACH, FACING_HALF, water)
}

/// How long one bottle takes, in whole seconds: 10 less the cube root of the
/// Water skill's percent (as checks count it, `skill_value`), so 10 down to 6.
pub fn fill_seconds(skills: &Skills) -> u32 {
    FIRST_SECONDS - cube_root(skill_percent(skill_value(skills, WATER_SKILL)))
}

/// The same in action ticks.
pub fn fill_ticks(skills: &Skills) -> u64 {
    u64::from(fill_seconds(skills)) * ACTION_TICKS_PER_SECOND as u64
}

/// The bottle `item` filled with `water`, keeping its look.
pub fn filled(mut item: Item, water: Water) -> Item {
    let look = item.bottle().map_or(0, |bottle| bottle.look);
    let liquid = Some(super::Liquid::Water(water));
    item.form = Some(crate::item_form::ItemForm::Bottle(BottleForm::new(
        liquid, look,
    )));
    item
}

impl Inventory {
    /// The stack the next empty bottle is filled from: as `stack_to_use`, the
    /// smallest in the character's own grid, else the smallest in a bag.
    pub fn empty_bottle(&self) -> Option<Id> {
        let empty = |top: &Item| top.bottle().is_some_and(BottleForm::is_empty);
        let smallest = |own: bool| {
            self.stacks_where(empty)
                .filter(|&(_, in_own)| in_own == own)
                .min_by_key(|(stack, _)| stack.count())
                .map(|(stack, _)| stack.id)
        };
        smallest(true).or_else(|| smallest(false))
    }
}
