//! What keeps a set's worn pieces together (owner, 2026-11-27): one layer
//! at different places (a suit of plate, or underclothes with a ring), or
//! one place at different layers (a linen coif, a hood and a helmet). Held
//! weapons have no layer and may join any set.

use std::sync::OnceLock;

use super::kinds::{wear, worn_kinds};
use crate::creature::{Layer, Location};
use crate::item::ItemKind;
use crate::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Theme {
    /// Every worn piece on this layer, each at different places.
    Layer(Layer),
    /// Every worn piece at this place, each on a different layer.
    Place(Location),
}

/// How often each layer is a set's (Claude's weights): only a surcoat or a
/// tabard is worn Over, so a set of it is one piece and weapons.
const LAYERS: [(Layer, u64); 4] = [
    (Layer::Skin, 3),
    (Layer::Middle, 3),
    (Layer::Outer, 3),
    (Layer::Over, 1),
];

impl Theme {
    /// One layer or one place, even odds; a place only where pieces are
    /// worn on two layers or more.
    pub(super) fn roll(rng: &mut Rng) -> Self {
        if rng.range_u64(0, 2) == 0 {
            return Self::Layer(rng.pick_weighted(&LAYERS));
        }
        let places = places();
        Self::Place(places[rng.range_u64(0, places.len() as u64) as usize])
    }

    /// The worn kinds suiting it, in `ItemKind::ALL`'s order.
    pub(super) fn kinds(self) -> Vec<ItemKind> {
        let suits = |kind: &&ItemKind| match self {
            Self::Layer(layer) => wear(**kind).layer == layer,
            Self::Place(place) => wear(**kind).locations.contains(&place),
        };
        worn_kinds().iter().filter(suits).copied().collect()
    }

    /// Whether `kind` (one of `kinds`) goes with the worn pieces so far:
    /// at places none of them is, on one layer, but for a second ring; on a
    /// layer none of them is, at one place.
    pub(super) fn fits(self, kinds: &[ItemKind], kind: ItemKind) -> bool {
        match self {
            Self::Layer(_) => {
                let rings = kinds.iter().filter(|&&each| each == ItemKind::Ring).count();
                if kind == ItemKind::Ring && rings == 1 {
                    return true;
                }
                let taken = || kinds.iter().flat_map(|&each| wear(each).locations);
                !wear(kind)
                    .locations
                    .iter()
                    .any(|place| taken().any(|each| each == place))
            }
            Self::Place(_) => kinds
                .iter()
                .all(|&each| wear(each).layer != wear(kind).layer),
        }
    }
}

/// The places worn pieces cover on two layers or more, in `Location`'s
/// order.
fn places() -> &'static [Location] {
    static PLACES: OnceLock<Vec<Location>> = OnceLock::new();
    PLACES.get_or_init(|| {
        let mut places: Vec<Location> = Vec::new();
        for &kind in worn_kinds() {
            for &place in wear(kind).locations {
                if !places.contains(&place) && layers_at(place) >= 2 {
                    places.push(place);
                }
            }
        }
        places.sort();
        places
    })
}

/// How many layers worn pieces cover `place` on.
fn layers_at(place: Location) -> usize {
    let layers = (worn_kinds().iter())
        .map(|&kind| wear(kind))
        .filter(|wear| wear.locations.contains(&place))
        .map(|wear| wear.layer);
    let mut layers: Vec<Layer> = layers.collect();
    layers.sort();
    layers.dedup();
    layers.len()
}
