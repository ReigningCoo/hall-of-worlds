//! An item's value at its popup's bottom right (trading plan PR 1): coin
//! pictures each touching its count, gold to copper, any of none left out; a stack's whole
//! value, then what one is worth in brackets ("each") when there are more
//! than one. Values come from `shared::value`, rounded up to copper only
//! here, as shown. Nothing shows for what is worth nothing (quest items).

use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::value::{Coins, copper_shown, each, total};

use crate::common::written;
use crate::dom::Dom;

/// Which coin a picture is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Metal {
    Gold,
    Silver,
    Copper,
}

impl Metal {
    /// The popup's template for its picture, in its colour, and its count.
    fn template(self) -> (&'static str, &'static str) {
        match self {
            Self::Gold => ("coin_gold", "gold_count"),
            Self::Silver => ("coin_silver", "silver_count"),
            Self::Copper => ("coin_copper", "copper_count"),
        }
    }
}

/// One thing along the value's row.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Piece {
    /// A coin's picture and how many.
    Coin(Metal, u64),
    Words(String),
}

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

/// `copper` as coins, leaving out any of none.
fn coin_pieces(copper: u64) -> Vec<Piece> {
    let coins = Coins::from_copper(copper);
    [
        (Metal::Gold, coins.gold),
        (Metal::Silver, coins.silver),
        (Metal::Copper, coins.copper),
    ]
    .into_iter()
    .filter(|&(_, count)| count > 0)
    .map(|(metal, count)| Piece::Coin(metal, count))
    .collect()
}

/// Writes the row into popup `form`'s `value`, a coin a copy of its template
/// (its picture and count together), hiding it when there is nothing.
pub(super) fn show_value(dom: &mut Dom, form: &str, pieces: Vec<Piece>) {
    let into = format!("{form}/value");
    dom.set_flag(&into, "hidden", pieces.is_empty());
    dom.empty(&into);
    let mut at = 0;
    let words = |dom: &mut Dom, at: &mut usize, text: String| {
        dom.copy_template(&format!("{form}/value_words"), &into, *at);
        dom.set_text(&format!("{form}/value_words.{at}"), text);
        *at += 1;
    };
    for piece in pieces {
        match piece {
            Piece::Coin(metal, count) => {
                let (template, count_id) = metal.template();
                dom.copy_template(&format!("{form}/{template}"), &into, at);
                dom.set_text(&format!("{form}/{count_id}.{at}"), count.to_string());
                at += 1;
            }
            Piece::Words(text) => words(dom, &mut at, text),
        }
    }
}

#[cfg(test)]
mod tests;
