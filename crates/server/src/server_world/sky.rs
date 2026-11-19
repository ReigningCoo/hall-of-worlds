//! The world's moons as quests see them: how many, and whether one is in the
//! sky now (`shared::quest::Need::Moon`); whether it is moonlit; and whether
//! rain can be caught at a place (Rain Water).

use shared::celestial::ColourName;

use super::ServerWorld;
use shared::world::MINUTES_PER_HOUR;

impl ServerWorld {
    /// How many moons the world has.
    pub(crate) fn moon_count(&self) -> u8 {
        self.celestial.moon_count().min(usize::from(u8::MAX)) as u8
    }

    /// Whether moon `moon` is above the horizon now.
    pub(crate) fn moon_up(&self, moon: u8) -> bool {
        let hours = self.minutes() / MINUTES_PER_HOUR;
        let index = self.celestial.sun_count() + usize::from(moon);
        (self.celestial.at(hours).get(index)).is_some_and(|now| now.is_up())
    }

    /// Whether no sun is in the sky now and at least one moon is: Moon Water
    /// is drawn outside (`shared::potion::Source::moonlit`).
    pub(crate) fn moonlit(&self) -> bool {
        let hours = self.minutes() / MINUTES_PER_HOUR;
        let now = self.celestial.at(hours);
        let (suns, moons) = now.split_at(self.celestial.sun_count().min(now.len()));
        !suns.iter().any(|sun| sun.is_up()) && moons.iter().any(|moon| moon.is_up())
    }

    /// Whether someone `outside` at this world map point catches Rain Water
    /// (`shared::potion::catches_rain`), by the weather now there, worked out
    /// as clients work out the rain they draw (at sea level's temperature).
    pub(crate) fn catches_rain(&self, x: i64, y: i64, outside: bool) -> bool {
        if !outside {
            return false;
        }
        let hours = self.minutes() / MINUTES_PER_HOUR;
        let rain = self.weather.sample(x, y, hours).rain;
        let temperature = self.climate.sample(x, y, self.params.sea_level);
        shared::potion::catches_rain(outside, rain, temperature.temperature)
    }

    /// Moon `moon`'s colour, by name, if it has one.
    pub(crate) fn moon_colour(&self, moon: u8) -> Option<ColourName> {
        let index = self.celestial.sun_count() + usize::from(moon);
        (self.celestial.bodies().get(index)).and_then(|body| body.colour_name)
    }
}
