//! Which fungi grow at a place (owner, 2026-11-19, decisions 2 to 5): above
//! ground by biome and the land's yearly temperature, as herbs; underground by
//! temperature alone (a cave is at its land's), by what a cell offers (its
//! niches), by depth (the rarer likelier deeper) and by `Where`.

use super::{Fungi, FungusDef, Group, Niche, Where};
use crate::climate::Biome;
use crate::mob::Habitat;
use crate::rng::Rng;
use crate::vegetable::Abundance;

/// Fungi that grow only on a cave's deepest level (owner: Dreamcap).
pub const DEEPEST_ONLY: [&str; 1] = ["dreamcap"];

/// What one cell offers a fungus, underground or above.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Spot {
    /// What it can grow on or by there.
    pub niches: Vec<Niche>,
    /// Near daylight underground (`Where::Dim`).
    pub dim: bool,
    /// Earth that buried fungi can be dug from (`Where::Dug`).
    pub earth: bool,
    /// Underground (a cave level, basement or cellar), so `Where::Deep` and
    /// `Dim` kinds, rather than above ground (`Surface`, and `Dug` in its
    /// biomes).
    pub underground: bool,
    /// How many levels down, from 0.
    pub depth: u32,
    /// On a cave's deepest level.
    pub deepest: bool,
}

impl FungusDef {
    /// Whether it may grow on `spot` at `celsius` (the land's yearly average),
    /// in `biome` above ground.
    pub fn suits_spot(&self, spot: &Spot, celsius: i32, biome: Biome) -> bool {
        let celsius = celsius.clamp(i16::MIN.into(), i16::MAX.into()) as i16;
        let place = match self.grows {
            Where::Deep => spot.underground,
            Where::Dim => spot.underground && spot.dim,
            Where::Dug if spot.underground => spot.earth,
            Where::Dug | Where::Surface => !spot.underground && self.biomes.contains(&biome),
        };
        let deepest = !DEEPEST_ONLY.contains(&self.key.as_str()) || spot.deepest;
        place
            && deepest
            && self.temperature.suits(celsius)
            && self.niches.iter().any(|niche| spot.niches.contains(niche))
    }

    /// Whether it feeds on wood, so grows on felled trees.
    pub fn feeds_on_wood(&self) -> bool {
        self.niches.contains(&Niche::Wood)
    }

    /// Whether it is a lichen (common in Tundra and Boreal forest).
    pub fn is_lichen(&self) -> bool {
        self.group == Group::Lichen
    }
}

/// How often a fungus this common is chosen against others for the same spot
/// `depth` levels down: commoner ones more often, rarer ones gaining a step a
/// level (very rare ones never on the top level).
pub fn weight(abundance: Abundance, depth: u32) -> u64 {
    let depth = u64::from(depth);
    match abundance {
        Abundance::Common => 9,
        Abundance::Uncommon => 3 + depth,
        Abundance::Rare => 1 + depth,
        Abundance::VeryRare => depth,
    }
}

/// The fungi that may grow on `spot` (`FungusDef::suits_spot`), by the table's
/// order.
pub fn suited<'a>(fungi: &'a Fungi, spot: &Spot, habitat: &Habitat) -> Vec<&'a FungusDef> {
    (fungi.iter())
        .filter(|def| def.suits_spot(spot, habitat.celsius, habitat.biome))
        .collect()
}

/// One of `fungi`, more often the more common it is `depth` levels down
/// (`weight`); `None` if none can be.
pub fn choose<'a>(fungi: &[&'a FungusDef], depth: u32, rng: &mut Rng) -> Option<&'a FungusDef> {
    let options: Vec<(usize, u64)> = (fungi.iter().enumerate())
        .map(|(i, def)| (i, weight(def.abundance, depth)))
        .filter(|&(_, weight)| weight > 0)
        .collect();
    if options.is_empty() {
        return None;
    }
    Some(fungi[rng.pick_weighted(&options)])
}
