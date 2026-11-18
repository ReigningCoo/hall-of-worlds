//! What the land is like at a world map point, for what grows there: herbs'
//! beds, orchards and fields, and the fruit and vegetables shops sell and
//! mobs carry (`shared::produce`).

use shared::mob::Habitat;
use shared::poi::Poi;

use super::ServerWorld;

impl ServerWorld {
    /// The land at world map point `(x, y)` (`x` taken round the planet).
    pub(crate) fn habitat_at(&self, (x, y): (i64, i64)) -> Habitat {
        let wild = &self.wilderness.wild;
        wild.habitat_at(x.rem_euclid(wild.points()), y)
    }

    /// The land at a poi.
    pub(crate) fn habitat_of_poi(&self, poi: &Poi) -> Habitat {
        self.habitat_at((poi.x, poi.y))
    }
}
