//! An item's value at its popup's bottom right (trading plan PR 1): coin
//! pictures each touching its count, gold to copper, any of none left out; a stack's whole
//! value, then what one is worth in brackets ("each") when there are more
//! than one. Values come from `shared::value`, rounded up to copper only
//! here, as shown. Nothing shows for what is worth nothing (quest items).
//! The coins themselves are `common`'s (`coin_pieces`, `show_coins`).

use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::value::{copper_shown, each, total};

use crate::common::{CoinPiece as Piece, coin_pieces, show_coins, written};
use crate::dom::Dom;

/// What the row shows for the item (a whole stack); nothing when it is
/// worth nothing.
pub(super) fn value_pieces(item: &Item, lang: &Lang) -> Vec<Piece> {
    let all = copper_shown(total(item));
    if all == 0 {
        return Vec::new();
    }
    let mut pieces = coin_pieces(all);
    if item.count() > 1 {
        let words = |key| Piece::Words(written(lang.render(key, &Args::new())));
        pieces.push(words("item-value-each-start"));
        pieces.extend(coin_pieces(copper_shown(each(item.top()))));
        pieces.push(words("item-value-each-end"));
    }
    pieces
}

/// Writes the row into popup `form`'s `value`, hiding it when there is
/// nothing.
pub(super) fn show_value(dom: &mut Dom, form: &str, pieces: Vec<Piece>) {
    show_coins(dom, form, "value", 0, pieces);
}

#[cfg(test)]
mod tests;
