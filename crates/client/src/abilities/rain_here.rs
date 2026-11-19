//! Whether Collect Water can be done where the player stands, as the server
//! checks: facing water, or, facing none, outside in rain heavy enough to
//! catch Rain Water (`shared::potion::catches_rain`), the rain worked out at
//! the site's place on the world map as `dungeon_map` draws it.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use shared::facing::Heading;
use shared::potion::{catches_rain, water_faced};
use shared::spot::Spot;

use crate::common::{CurrentDungeon, CurrentWorld, ServerClock, WorldReturn};

/// What Collect Water's check reads.
#[derive(SystemParam)]
pub(super) struct WaterHere<'w> {
    dungeon: Res<'w, CurrentDungeon>,
    world: Option<Res<'w, CurrentWorld>>,
    clock: Res<'w, ServerClock>,
    place: Res<'w, WorldReturn>,
}

impl WaterHere<'_> {
    /// Whether someone at `at` facing `heading` has water to collect.
    pub(super) fn can_collect(&self, at: Spot, heading: Heading) -> bool {
        let level = &self.dungeon.level;
        if water_faced(level, at, heading).is_some() {
            return true;
        }
        let Some(world) = &self.world else {
            return false;
        };
        let (x, y) = at.cell();
        let outside = level.cell(x, y).is_outside();
        let (px, py) = (self.place.0.x, self.place.0.y);
        let rain = world.weather.sample(px, py, self.clock.hours()).rain;
        let temperature = world.climate.sample(px, py, world.params.sea_level);
        catches_rain(outside, rain, temperature.temperature)
    }
}
