//! The wood of every fruit and nut tree with a trunk, and how many logs a
//! grown one gives, by its fruit's key: the research's groups
//! (`docs/context/research-trees.md`), each sharing one wood.

use crate::material::Material::{self, *};

/// Each trunked fruit plant's wood and logs, by its fruit's key. Banana,
/// papaya and saguaro have trunks but no true wood: felled, they give
/// kindling instead (`FRUIT_KINDLING`), so they are left out.
#[rustfmt::skip]
pub(super) const FRUIT_WOODS: [(&str, Material, u8); 57] = [
    ("apple", Apple, 1), ("crab_apple", Apple, 1),
    ("pear", Pear, 3), ("quince", Pear, 1), ("medlar", Pear, 1), ("rowan", Pear, 2),
    ("cherry", Cherry, 5),
    ("plum", Plum, 2), ("apricot", Plum, 2), ("almond", Plum, 1),
    ("walnut", Walnut, 8),
    ("pecan", Hickory, 10), ("hickory_nut", Hickory, 7),
    ("sweet_chestnut", Chestnut, 12), ("chinquapin", Chestnut, 6),
    ("acorn", Oak, 11),
    ("beechnut", Beech, 10),
    ("olive", Olive, 5),
    ("orange", Citrus, 2), ("lemon", Citrus, 1),
    ("fig", Fig, 2),
    ("mulberry", Mulberry, 5),
    ("persimmon", Persimmon, 2), ("jackalberry", Persimmon, 5),
    ("pawpaw", Whitewood, 1), ("cacao_pod", Whitewood, 1),
    ("candlenut", Whitewood, 7), ("mongongo_nut", Whitewood, 7),
    ("date", Palm, 5), ("coconut", Palm, 5), ("acai", Palm, 1),
    ("mango", Mango, 10),
    ("jackfruit", Jackwood, 7), ("breadfruit", Jackwood, 7),
    ("avocado", Avocado, 5),
    ("lychee", Lychee, 5), ("rambutan", Lychee, 5), ("mangosteen", Lychee, 4),
    ("durian", Durian, 11),
    ("guava", Guava, 1),
    ("tamarind", Tamarind, 10), ("carob", Tamarind, 4), ("jujube", Tamarind, 2),
    ("cashew", Cashew, 4),
    ("brazil_nut", BrazilNut, 12),
    ("macadamia", Macadamia, 2),
    ("kola_nut", Kola, 6),
    ("pili_nut", Pili, 6),
    ("baobab", Baobab, 12),
    ("marula", Marula, 7),
    ("quandong", Sandalwood, 1),
    ("kakadu_plum", Terminalia, 2),
    ("shea_nut", Shea, 6),
    ("pistachio", Pistachio, 2),
    ("siberian_pine_nut", Pine, 10), ("stone_pine_nut", Pine, 7),
    ("juniper", Juniper, 1),
];

/// The trunks without true wood, felled for kindling instead of logs: each
/// one's wood and the pieces a grown one gives, 5 to 15 (owner, 2026-11-19;
/// a saguaro's ribs the most, a papaya's hollow stem the least).
#[rustfmt::skip]
pub(super) const FRUIT_KINDLING: [(&str, Material, u8); 3] = [
    ("banana", Banana, 10), ("papaya", Papaya, 5), ("saguaro_fruit", Saguaro, 15),
];

/// A trunked fruit plant's wood and grown logs, by its fruit's key; `None`
/// for one without true wood (banana, papaya, saguaro: `fruit_kindling`) or
/// no trunk.
pub fn fruit_wood(key: &str) -> Option<(Material, u8)> {
    (FRUIT_WOODS.iter())
        .find(|(fruit, ..)| *fruit == key)
        .map(|&(_, wood, logs)| (wood, logs))
}

/// A fruit plant without true wood (banana, papaya, saguaro): its wood and
/// the kindling a grown one gives, by its fruit's key; `None` for any other.
pub fn fruit_kindling(key: &str) -> Option<(Material, u8)> {
    (FRUIT_KINDLING.iter())
        .find(|(fruit, ..)| *fruit == key)
        .map(|&(_, wood, pieces)| (wood, pieces))
}
