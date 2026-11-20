//! Who may live in a wilderness block, and how likely each is: the livestock
//! table's wild forms and feral herds by biome, rarity, place and warmth;
//! wolves by biome; highland cattle's own share by cold and height.

use super::wild::Habitat;
use crate::climate::Biome;
use crate::creature::CreatureKind;
use crate::livestock::{Livestock, Place, Rarity, Wild};

/// What a block has round it, beyond its biome: what some animals need.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Around {
    /// Water in it: the sea, shallows or a river.
    pub water: bool,
    /// A road in it, or a place's cutout: people about.
    pub people: bool,
}

/// What sort of animal one is, out there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Form {
    /// A kept animal's wild form (an aurochs is cattle in the wild).
    Wild,
    /// A kept animal gone feral.
    Feral,
    /// Wolves and highland cattle, as before the table.
    Native,
}

/// One kind that may live in a block, as what, and where in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Resident {
    pub kind: CreatureKind,
    pub form: Form,
    pub place: Place,
}

/// A wild form's weight by its rarity: each step about twice the last.
fn rarity_weight(rarity: Rarity) -> u64 {
    match rarity {
        Rarity::Never => 0,
        Rarity::VeryRare => 1,
        Rarity::Rare => 3,
        Rarity::Uncommon => 8,
        Rarity::Common => 16,
        Rarity::VeryCommon => 32,
    }
}

/// How many times a wild form's weight is a feral herd's of the same rarity:
/// feral herds are fewer than wild ones away from people.
const WILD_TIMES_FERAL: u64 = 4;

/// Wolves' weight by biome, as a wild form's (very common in forests and
/// tundra, as before the table, fewer in woodland, a few on grassland and
/// savanna, none in desert).
fn wolf_weight(biome: Biome) -> u64 {
    match biome {
        Biome::Desert => 0,
        Biome::Grassland | Biome::Savanna => 4,
        Biome::Woodland => 24,
        _ => 48,
    }
}

/// How well a place in a block suits, in thousandths: by height for
/// mountain animals, none without water for water birds, and so on.
fn place_fit(place: Place, land: Habitat, around: Around) -> u64 {
    // From 0 at the sea to 1000 at 1500 m and above.
    let high = (land.metres.clamp(0, 1500) * 1000 / 1500) as u64;
    match place {
        Place::Anywhere => 1000,
        Place::Mountains => 250 + 750 * high / 1000,
        Place::High => high,
        Place::ByWater => u64::from(around.water) * 500,
        Place::NearPeople => u64::from(around.people) * 500,
        Place::Cliffs => {
            let cliffs = around.water || around.people || land.metres >= 1000;
            u64::from(cliffs) * 500
        }
    }
}

/// A wild form's or feral herd's weight here (`biomes` empty for feral herds:
/// they live where the animal is kept).
fn weight(wild: &Wild, biomes: &[Biome], land: Habitat, around: Around) -> u64 {
    let biomes = if wild.biomes.is_empty() {
        biomes
    } else {
        &wild.biomes
    };
    if !biomes.contains(&land.biome) || !wild.warmth.suits(land.celsius) {
        return 0;
    }
    rarity_weight(wild.rarity) * place_fit(wild.place, land, around)
}

/// Who may live on this land with what weight, highland cattle's share
/// (`coo_percent`) of the whole first and the others shrunk to make room.
/// The weights are relative: only their shares of the sum matter.
pub(super) fn shares(land: Habitat, around: Around) -> Vec<(Resident, u64)> {
    let mut others: Vec<(Resident, u64)> = Vec::new();
    let mut add = |kind, form, place, weight: u64| {
        if weight > 0 {
            others.push((Resident { kind, form, place }, weight));
        }
    };
    add(
        CreatureKind::Wolf,
        Form::Native,
        Place::Anywhere,
        wolf_weight(land.biome) * WILD_TIMES_FERAL * 1000,
    );
    for def in Livestock::built_in().iter() {
        let Some(kind) = def.kind.filter(|&kind| kind != CreatureKind::Coo) else {
            continue;
        };
        if let Some(wild) = &def.wild {
            let here = weight(wild, &[], land, around) * WILD_TIMES_FERAL;
            add(kind, Form::Wild, wild.place, here);
        }
        let feral = weight(&def.feral, &def.kept, land, around);
        add(kind, Form::Feral, def.feral.place, feral);
    }
    let coo = coo_percent(land);
    let total: u64 = others.iter().map(|&(_, weight)| weight).sum();
    if coo == 0 || total == 0 {
        return others;
    }
    let coos = Resident {
        kind: CreatureKind::Coo,
        form: Form::Native,
        place: Place::Anywhere,
    };
    let rest = others
        .into_iter()
        .map(|(resident, weight)| (resident, weight * (100 - coo)));
    std::iter::once((coos, coo * total)).chain(rest).collect()
}

/// Highland cattle's share of a block's animals, in percent: none where it is
/// hot (savanna, deserts, forests too warm or wet), about 15 in low, mild
/// grassland, rising with cold and height to 40 on cold high ground.
pub(super) fn coo_percent(land: Habitat) -> u64 {
    use Biome::{BorealForest, Grassland, Tundra, Woodland};
    if !matches!(land.biome, Tundra | BorealForest | Woodland | Grassland) {
        return 0;
    }
    // Each from 0 to 1000: full cold at 0 °C and below, none from 15 °C;
    // full height from 1500 m.
    let cold = ((15 - land.celsius).clamp(0, 15) * 1000 / 15) as u64;
    let high = (land.metres.clamp(0, 1500) * 1000 / 1500) as u64;
    15 + 25 * (2 * cold + high) / 3000
}

#[cfg(test)]
mod tests;
