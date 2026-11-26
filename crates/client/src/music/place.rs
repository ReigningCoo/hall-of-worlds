//! The places music follows (music plan PR 4): the wilderness by climate
//! band and by day or night, the world map, the Hall, The Way Inn, and the
//! kinds of site. Playlists name them on tunes and ensembles, where `Any`
//! band or time takes in every one; where the player is, they are never
//! `Any` (`place_now`).

use serde::Deserialize;
use shared::climate::Biome;
use shared::poi::PoiKind;
use shared::protocol::{LevelInfo, LevelKind};

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Place {
    Wilderness(Band, Daylight),
    WorldMap,
    Hall,
    Inn,
    /// Hamlets and villages.
    Village,
    Town,
    City,
    /// Castles and forts.
    Castle,
    Tower,
    /// Caves, crypts and every cellar or basement.
    Cave,
    Farm,
    Herbalist,
    Oasis,
}

/// The wilderness's climate bands, from the biome.
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Band {
    Any,
    Cold,
    Temperate,
    Dry,
    /// Hot and wet: savanna and rainforest.
    Hot,
}

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Daylight {
    Any,
    Day,
    Night,
}

impl Place {
    /// Whether a place a playlist names takes in this one.
    pub fn within(self, listed: Place) -> bool {
        match (self, listed) {
            (Self::Wilderness(band, time), Self::Wilderness(in_band, in_time)) => {
                (in_band == Band::Any || in_band == band)
                    && (in_time == Daylight::Any || in_time == time)
            }
            _ => self == listed,
        }
    }

    /// The place a level is, given the wilderness's band and time there for
    /// sites the music doesn't tell apart (huts, ruins, tents and the like).
    pub fn of_level(info: &LevelInfo, outside: Place) -> Place {
        let kind = match info.kind {
            LevelKind::Wilderness => return outside,
            LevelKind::Hall { .. } => return Self::Hall,
            LevelKind::Inn => return Self::Inn,
            LevelKind::Poi(kind) => kind,
        };
        if info.storey(info.index) < 0 {
            return Self::Cave;
        }
        match kind {
            PoiKind::Hamlet | PoiKind::Village => Self::Village,
            PoiKind::Town => Self::Town,
            PoiKind::City => Self::City,
            PoiKind::Castle | PoiKind::Fort => Self::Castle,
            PoiKind::StoneTower | PoiKind::WoodenTower => Self::Tower,
            PoiKind::Farm => Self::Farm,
            PoiKind::HerbalistHut => Self::Herbalist,
            PoiKind::Oasis => Self::Oasis,
            PoiKind::GrassyCave
            | PoiKind::EarthCave
            | PoiKind::RockCave
            | PoiKind::IceCave
            | PoiKind::Crypt
            | PoiKind::MossyCrypt
            | PoiKind::BloodyCrypt
            | PoiKind::Mausoleum
            | PoiKind::StonePyramid
            | PoiKind::SandPyramid => Self::Cave,
            _ if !info.building => Self::Cave,
            _ => outside,
        }
    }
}

impl Band {
    pub fn of(biome: Biome) -> Band {
        match biome {
            Biome::Tundra | Biome::BorealForest => Self::Cold,
            Biome::Grassland
            | Biome::Woodland
            | Biome::SeasonalForest
            | Biome::TemperateRainforest => Self::Temperate,
            Biome::Desert => Self::Dry,
            Biome::Savanna | Biome::TropicalRainforest => Self::Hot,
        }
    }
}

#[cfg(test)]
mod tests;
