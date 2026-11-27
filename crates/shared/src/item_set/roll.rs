//! Rolling a set's quality and pieces from its number alone.

use super::{MAX_PIECES, MIN_PIECES};
use crate::creature::Location;
use crate::item::{ItemKind, Quality};
use crate::quest::PRIZE_KINDS;
use crate::rng::{Rng, Seed};

/// The pool's own seed: every set is rolled from it and its number, so all
/// planets and servers have the same sets.
const POOL_SEED: u64 = 0x5E7_1735;

/// How good a set is (Claude's weights, never Poor or Artifact): what a
/// planet's set item was before sets.
const SET_QUALITIES: [(Quality, u64); 5] = [
    (Quality::Common, 10),
    (Quality::Uncommon, 25),
    (Quality::Rare, 30),
    (Quality::Epic, 25),
    (Quality::Legendary, 10),
];

/// Tries at a piece before a set settles for fewer.
const TRIES: usize = 64;

/// The seed set `set` is rolled from, for a purpose.
pub(super) fn seed(set: u16, purpose: &str) -> Seed {
    Seed::new(Seed::new(POOL_SEED).at(&[i64::from(set)])).derive(purpose)
}

/// A set's quality and its pieces' kinds, in order: worn in different
/// places, but two rings may be in one set.
pub(super) fn pieces(set: u16) -> (Quality, Vec<ItemKind>) {
    let mut rng = seed(set, "pieces").rng();
    let quality = rng.pick_weighted(&SET_QUALITIES);
    let wanted = rng.range_u64(MIN_PIECES as u64, MAX_PIECES as u64 + 1) as usize;
    let mut kinds: Vec<ItemKind> = Vec::new();
    for _ in 0..TRIES {
        if kinds.len() == wanted {
            break;
        }
        let kind = pick(&mut rng);
        if fits(&kinds, kind) {
            kinds.push(kind);
        }
    }
    (quality, kinds)
}

/// Any of the kinds a set's piece may be.
fn pick(rng: &mut Rng) -> ItemKind {
    PRIZE_KINDS[rng.range_u64(0, PRIZE_KINDS.len() as u64) as usize]
}

/// Whether `kind` is worn where no piece so far is, but for a second ring
/// (gauntlets cover the fingers too, so never with a ring).
fn fits(kinds: &[ItemKind], kind: ItemKind) -> bool {
    let rings = kinds.iter().filter(|&&each| each == ItemKind::Ring).count();
    if kind == ItemKind::Ring && (1..2).contains(&rings) {
        return true;
    }
    let taken: Vec<Location> = kinds.iter().flat_map(|&each| locations(each)).collect();
    !locations(kind).iter().any(|place| taken.contains(place))
}

/// Where a kind is worn.
pub(super) fn locations(kind: ItemKind) -> Vec<Location> {
    kind.def()
        .wear
        .map_or(Vec::new(), |wear| wear.locations.to_vec())
}
