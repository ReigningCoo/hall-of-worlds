//! Bands: the groups mobs belong to. A wild pack or herd is one band; on a
//! site, each race is one band (on every level). A band rolls one of its race's
//! variants (`BandVariant`), each with its own allies, rivals and enemies, from
//! its number, so nothing about it is saved. Families within bands are reserved.

use serde::{Deserialize, Serialize};

use crate::creature::CreatureKind;
use crate::rng::Seed;

/// A band, by a number of its own (a hash): members share it.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Band(pub u64);

impl Band {
    /// The band of every `kind` on the site with this seed, whichever level.
    pub fn of_site(seed: Seed, kind: CreatureKind) -> Self {
        Band(seed.derive("band").derive(&format!("{kind:?}")).value())
    }
}

/// A family within a band: reserved for the more intelligent races (never
/// animals), and not used yet.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Family(pub u64);

/// Someone as the behaviour rules see them: their race and band.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Member {
    pub kind: CreatureKind,
    pub band: Band,
}

/// One kind of band a race can have, rolled per band by `weight`, with how it
/// takes to other races: those listed in none are `Relation::Neutral`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BandVariant {
    /// Its id for good: lower case, digits and `_`.
    pub key: String,
    /// How often it is rolled among its race's variants.
    pub weight: u32,
    #[serde(default)]
    pub allies: Vec<CreatureKind>,
    #[serde(default)]
    pub rivals: Vec<CreatureKind>,
    #[serde(default)]
    pub enemies: Vec<CreatureKind>,
}

impl BandVariant {
    /// A race's only variant when its data lists none.
    pub fn common() -> Self {
        BandVariant {
            key: "common".into(),
            weight: 1,
            allies: Vec::new(),
            rivals: Vec::new(),
            enemies: Vec::new(),
        }
    }

    /// The variant a band rolls among these (none: `None`).
    pub fn rolled(variants: &[BandVariant], band: Band) -> Option<&BandVariant> {
        let total: u64 = variants.iter().map(|v| u64::from(v.weight)).sum();
        let mut left = Seed::new(band.0).derive("variant").value() % total.max(1);
        variants.iter().find(|variant| {
            let here = left < u64::from(variant.weight);
            left = left.saturating_sub(u64::from(variant.weight));
            here
        })
    }
}
