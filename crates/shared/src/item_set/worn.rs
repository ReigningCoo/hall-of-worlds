//! What sets give someone for what they wear: each set's bonuses for how
//! many of its different pieces are worn.

use std::collections::{BTreeMap, BTreeSet};

use super::ItemSet;
use crate::bonus::Bonus;
use crate::item::Item;

/// How many different pieces of each set are worn, by set number. A piece
/// counts only on an item of the kind it is, and twice the same piece once.
pub fn worn_pieces(worn: &[Item]) -> BTreeMap<u16, usize> {
    (pieces_of(worn).into_iter())
        .map(|(set, worn)| (set, worn.len()))
        .collect()
}

/// Which of set `set`'s pieces are worn (`SetPiece::piece`), each once and
/// only on an item of its kind, as `worn_pieces` counts them.
pub fn pieces_worn(worn: &[Item], set: u16) -> BTreeSet<u8> {
    pieces_of(worn).remove(&set).unwrap_or_default()
}

/// The different pieces of each set worn, by set number.
fn pieces_of(worn: &[Item]) -> BTreeMap<u16, BTreeSet<u8>> {
    let mut pieces: BTreeMap<u16, BTreeSet<u8>> = BTreeMap::new();
    for item in worn {
        let Some(piece) = item.set else {
            continue;
        };
        if piece.kind() == Some(item.kind) {
            pieces.entry(piece.set).or_default().insert(piece.piece);
        }
    }
    pieces
}

/// Every set bonus what is worn gives, set by set.
pub fn set_bonuses(worn: &[Item]) -> Vec<Bonus> {
    let mut bonuses = Vec::new();
    for (set, count) in worn_pieces(worn) {
        if count < 2 {
            continue;
        }
        bonuses.extend(ItemSet::of(set).bonuses_at(count).copied());
    }
    bonuses
}
