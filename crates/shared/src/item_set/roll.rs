//! Rolling a set's quality and pieces from its number alone, kept once
//! rolled: every set's pieces are asked for often (`SetPiece::kind`).

use std::sync::OnceLock;

use super::kinds::weapons;
use super::theme::Theme;
use super::{MAX_PIECES, MIN_PIECES, POOL};
use crate::creature::Location;
use crate::item::{ItemKind, Quality};
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

/// How many weapons a set asks for (Claude's weights); it takes up to
/// `MAX_WEAPONS` when its worn pieces fall short.
const WEAPONS_WANTED: [(usize, u64); 3] = [(0, 6), (1, 3), (2, 1)];
/// The most weapons in a set: one for each hand.
const MAX_WEAPONS: usize = 2;

/// Tries at a piece before a set settles for fewer.
const TRIES: usize = 64;

/// The seed set `set` is rolled from, for a purpose.
pub(super) fn seed(set: u16, purpose: &str) -> Seed {
    Seed::new(Seed::new(POOL_SEED).at(&[i64::from(set)])).derive(purpose)
}

/// A set's quality and its pieces' kinds, in order: its worn pieces (one
/// layer at different places, or one place at different layers), then its
/// weapons. `set` is below `POOL`.
pub(super) fn pieces(set: u16) -> (Quality, &'static [ItemKind]) {
    static POOLED: OnceLock<Vec<(Quality, Vec<ItemKind>)>> = OnceLock::new();
    let pooled = POOLED.get_or_init(|| (0..POOL).map(roll).collect());
    let (quality, kinds) = &pooled[usize::from(set % POOL)];
    (*quality, kinds)
}

/// Set `set`'s quality and pieces, rolled.
fn roll(set: u16) -> (Quality, Vec<ItemKind>) {
    let mut rng = seed(set, "pieces").rng();
    let quality = rng.pick_weighted(&SET_QUALITIES);
    let wanted = rng.range_u64(MIN_PIECES as u64, MAX_PIECES as u64 + 1) as usize;
    let theme = Theme::roll(&mut rng);
    let armed = rng.pick_weighted(&WEAPONS_WANTED).min(wanted);
    let mut kinds: Vec<ItemKind> = Vec::new();
    let worn = theme.kinds();
    for _ in 0..TRIES {
        if kinds.len() == wanted - armed {
            break;
        }
        let kind = pick(&mut rng, &worn);
        if theme.fits(&kinds, kind) {
            kinds.push(kind);
        }
    }
    // Weapons, different ones, up to the pieces wanted.
    let worn_count = kinds.len();
    for _ in 0..TRIES {
        if kinds.len() == wanted || kinds.len() - worn_count == MAX_WEAPONS {
            break;
        }
        let kind = pick(&mut rng, weapons());
        if !kinds.contains(&kind) {
            kinds.push(kind);
        }
    }
    (quality, kinds)
}

/// Any of `kinds`.
fn pick(rng: &mut Rng, kinds: &[ItemKind]) -> ItemKind {
    kinds[rng.range_u64(0, kinds.len() as u64) as usize]
}

/// Where a kind is worn.
pub(super) fn locations(kind: ItemKind) -> Vec<Location> {
    kind.def()
        .wear
        .map_or(Vec::new(), |wear| wear.locations.to_vec())
}
