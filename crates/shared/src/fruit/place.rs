//! Which fruits grow at a place: wild by its biome and yearly average
//! temperature, as herbs do, more often the more common they are; in orchards
//! by its warmth alone, the cultivated ones (`Abundance::OrchardOnly`) most.

use super::{Abundance, FruitDef, Fruits};
use crate::mob::Habitat;
use crate::rng::Rng;

impl FruitDef {
    /// Whether its plant grows wild at a place like this: never for orchard
    /// fruits or a herb's fruit (the herb bears it).
    pub fn grows_wild(&self, habitat: &Habitat) -> bool {
        self.is_item()
            && self.abundance != Abundance::OrchardOnly
            && self.biomes.contains(&habitat.biome)
            && self.suits_warmth(habitat.celsius)
    }

    /// Whether a place this warm (yearly average, °C) suits it.
    pub fn suits_warmth(&self, celsius: i32) -> bool {
        let celsius = celsius.clamp(i16::MIN.into(), i16::MAX.into()) as i16;
        self.temperature.suits(celsius)
    }

    /// How far a place this warm is outside its temperatures, in °C.
    fn off_warmth(&self, celsius: i32) -> i32 {
        let (min, max) = (
            i32::from(self.temperature.min),
            i32::from(self.temperature.max),
        );
        (min - celsius).max(celsius - max).max(0)
    }
}

impl Abundance {
    /// How often it is chosen against others that grow wild at the same
    /// place: never for orchard fruits.
    pub const fn wild_weight(self) -> u64 {
        match self {
            Self::Common => 9,
            Self::Uncommon => 3,
            Self::Rare => 1,
            Self::OrchardOnly => 0,
        }
    }

    /// The same in an orchard, where the cultivated ones are grown most.
    pub const fn orchard_weight(self) -> u64 {
        match self {
            Self::OrchardOnly => 4,
            Self::Common => 3,
            Self::Uncommon => 2,
            Self::Rare => 1,
        }
    }
}

/// The fruits that grow wild at a place like this, by their table's order.
pub fn wild_fruits<'a>(fruits: &'a Fruits, habitat: &Habitat) -> Vec<&'a FruitDef> {
    fruits
        .items()
        .filter(|def| def.grows_wild(habitat))
        .collect()
}

/// The fruits an orchard grows at a place this warm: those its warmth suits,
/// else those nearest its warmth.
pub fn orchard_fruits(fruits: &Fruits, celsius: i32) -> Vec<&FruitDef> {
    let nearest = (fruits.items())
        .map(|def| def.off_warmth(celsius))
        .min()
        .unwrap_or(0);
    (fruits.items())
        .filter(|def| def.off_warmth(celsius) == nearest)
        .collect()
}

/// One of `fruits`, more often the more common it is by `weight`; `None` if
/// there are none, or none weighs anything.
pub fn choose<'a>(
    fruits: &[&'a FruitDef],
    weight: fn(Abundance) -> u64,
    rng: &mut Rng,
) -> Option<&'a FruitDef> {
    let options: Vec<(usize, u64)> = (fruits.iter().enumerate())
        .map(|(i, def)| (i, weight(def.abundance)))
        .filter(|&(_, weight)| weight > 0)
        .collect();
    if options.is_empty() {
        return None;
    }
    Some(fruits[rng.pick_weighted(&options)])
}
