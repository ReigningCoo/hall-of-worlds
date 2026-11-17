//! Which herbs grow at a place: by its biome and yearly average temperature,
//! more often the more common they are.

use super::{Abundance, HerbDef, Herbs};
use crate::climate::Biome;
use crate::mob::Habitat;
use crate::rng::Rng;

/// Where a herb of marshes alone grows until the climate has marshes (owner:
/// marsh herbs grow in their other biomes; these have none): the wettest.
pub const MARSH_STAND_INS: [Biome; 4] = [
    Biome::BorealForest,
    Biome::SeasonalForest,
    Biome::TemperateRainforest,
    Biome::TropicalRainforest,
];

impl HerbDef {
    /// Whether it grows wild in `biome`.
    pub fn grows_in(&self, biome: Biome) -> bool {
        self.biomes.contains(&biome)
            || (self.biomes.is_empty() && self.marsh && MARSH_STAND_INS.contains(&biome))
    }

    /// Whether it grows wild at a place like this.
    pub fn suits(&self, habitat: &Habitat) -> bool {
        let celsius = habitat.celsius.clamp(i16::MIN.into(), i16::MAX.into()) as i16;
        self.grows_in(habitat.biome) && self.temperature.suits(celsius)
    }
}

impl Abundance {
    /// How often it is chosen against others that grow at the same place,
    /// in the wild.
    pub const fn wild_weight(self) -> u64 {
        match self {
            Self::Common => 9,
            Self::Uncommon => 3,
            Self::Rare => 1,
        }
    }

    /// The same in a herbalist's beds, where rarer herbs are sought out.
    pub const fn bed_weight(self) -> u64 {
        match self {
            Self::Common => 3,
            Self::Uncommon => 2,
            Self::Rare => 1,
        }
    }
}

/// The herbs that grow wild at a place like this, by their table's order.
pub fn wild_herbs<'a>(herbs: &'a Herbs, habitat: &Habitat) -> Vec<&'a HerbDef> {
    herbs.iter().filter(|def| def.suits(habitat)).collect()
}

/// The herbs a herbalist grows at a place like this: those that grow wild
/// there, else those its warmth suits, else those nearest its warmth.
pub fn bed_herbs<'a>(herbs: &'a Herbs, habitat: &Habitat) -> Vec<&'a HerbDef> {
    let wild = wild_herbs(herbs, habitat);
    if !wild.is_empty() {
        return wild;
    }
    let off = |def: &HerbDef| {
        let (min, max) = (
            i32::from(def.temperature.min),
            i32::from(def.temperature.max),
        );
        (min - habitat.celsius).max(habitat.celsius - max).max(0)
    };
    let nearest = herbs.iter().map(off).min().unwrap_or(0);
    herbs.iter().filter(|def| off(def) == nearest).collect()
}

/// One of `herbs`, more often the more common it is by `weight`.
pub fn choose<'a>(
    herbs: &[&'a HerbDef],
    weight: fn(Abundance) -> u64,
    rng: &mut Rng,
) -> Option<&'a HerbDef> {
    if herbs.is_empty() {
        return None;
    }
    let options: Vec<(usize, u64)> = (herbs.iter().enumerate())
        .map(|(i, def)| (i, weight(def.abundance)))
        .collect();
    Some(herbs[rng.pick_weighted(&options)])
}
