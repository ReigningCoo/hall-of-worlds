//! What sets give someone for what they wear: each set's bonuses for how
//! many of its different pieces are worn.

use std::collections::BTreeMap;

use super::ItemSet;
use crate::bonus::Bonus;
use crate::item::Item;

/// How many different pieces of each set are worn, by set number. A piece
/// counts only on an item of the kind it is, and twice the same piece once.
pub fn worn_pieces(worn: &[Item]) -> BTreeMap<u16, usize> {
    let mut pieces: BTreeMap<u16, Vec<u8>> = BTreeMap::new();
    for item in worn {
        let Some(piece) = item.set else {
            continue;
        };
        if piece.kind() != Some(item.kind) {
            continue;
        }
        let worn = pieces.entry(piece.set).or_default();
        if !worn.contains(&piece.piece) {
            worn.push(piece.piece);
        }
    }
    (pieces.into_iter())
        .map(|(set, worn)| (set, worn.len()))
        .collect()
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
