//! Which vegetables grow at a place (owner, 2026-11-17), by its biome alone
//! (the table has no temperatures): wild, the commoner more often, with crops
//! only farmed (`Abundance::VeryRare`) very rarely and wild-only plants as
//! often as herbs (`wilderness::vegetables`); in farms' fields, one crop each
//! from the cultivated ones (never `wild_only`), any equally.

use super::{Abundance, VegetableDef, Vegetables};
use crate::climate::Biome;
use crate::rng::Rng;

impl VegetableDef {
    /// Whether its plant grows wild in this biome.
    pub fn grows_wild(&self, biome: Biome) -> bool {
        self.biomes.contains(&biome)
    }
}

impl Abundance {
    /// How often it is chosen against others that grow wild at the same
    /// place.
    pub const fn wild_weight(self) -> u64 {
        match self {
            Self::Common => 9,
            Self::Uncommon => 3,
            Self::Rare => 1,
            // Crops only farmed: an escape now and then.
            Self::VeryRare => 1,
        }
    }
}

/// The farmed vegetables that grow wild in this biome (escapes, by their
/// table's order): never the wild-only ones, which `wild_only_vegetables` has.
pub fn wild_crops(vegetables: &Vegetables, biome: Biome) -> Vec<&VegetableDef> {
    (vegetables.iter())
        .filter(|def| !def.wild_only && def.grows_wild(biome))
        .collect()
}

/// The wild-only vegetables that grow in this biome, by their table's order.
pub fn wild_only_vegetables(vegetables: &Vegetables, biome: Biome) -> Vec<&VegetableDef> {
    (vegetables.iter())
        .filter(|def| def.wild_only && def.grows_wild(biome))
        .collect()
}

/// The crops a farm's fields grow in this biome: the cultivated ones (never
/// `wild_only`) growing there, else every cultivated one.
pub fn field_crops(vegetables: &Vegetables, biome: Biome) -> Vec<&VegetableDef> {
    let farmed = || vegetables.iter().filter(|def| !def.wild_only);
    let suited: Vec<_> = farmed().filter(|def| def.grows_wild(biome)).collect();
    if suited.is_empty() {
        farmed().collect()
    } else {
        suited
    }
}

/// One of `vegetables`, more often the more common it is wild
/// (`Abundance::wild_weight`); `None` if there are none.
pub fn choose_wild<'a>(vegetables: &[&'a VegetableDef], rng: &mut Rng) -> Option<&'a VegetableDef> {
    let options: Vec<(usize, u64)> = (vegetables.iter().enumerate())
        .map(|(i, def)| (i, def.abundance.wild_weight()))
        .collect();
    if options.is_empty() {
        return None;
    }
    Some(vegetables[rng.pick_weighted(&options)])
}
