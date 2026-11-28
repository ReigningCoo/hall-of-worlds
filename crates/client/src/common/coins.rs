//! Sums of money drawn as coins (trading plan PR 1, moved here for the trade
//! window by trading tweaks PR 3): a coin picture touching its count, gold to
//! copper, any of none left out, with words between where wanted. A document
//! drawing them has the templates `coin_gold`, `coin_silver`, `coin_copper`
//! (a `span.joined` of `img.<metal>` and `<metal>_count`) and `value_words`
//! (a `p`), as the item popup's markup shows.

use shared::value::Coins;

use crate::dom::Dom;

/// Which coin a picture is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metal {
    Gold,
    Silver,
    Copper,
}

impl Metal {
    /// The template for its picture, in its colour, and its count.
    fn template(self) -> (&'static str, &'static str) {
        match self {
            Self::Gold => ("coin_gold", "gold_count"),
            Self::Silver => ("coin_silver", "silver_count"),
            Self::Copper => ("coin_copper", "copper_count"),
        }
    }
}

/// One thing along a row of coins.
#[derive(Debug, PartialEq, Eq)]
pub enum CoinPiece {
    /// A coin's picture and how many.
    Coin(Metal, u64),
    Words(String),
}

/// `copper` as coins, leaving out any of none (nothing at all for none).
pub fn coin_pieces(copper: u64) -> Vec<CoinPiece> {
    let coins = Coins::from_copper(copper);
    [
        (Metal::Gold, coins.gold),
        (Metal::Silver, coins.silver),
        (Metal::Copper, coins.copper),
    ]
    .into_iter()
    .filter(|&(_, count)| count > 0)
    .map(|(metal, count)| CoinPiece::Coin(metal, count))
    .collect()
}

/// Writes `pieces` into `form`'s `into` (a `span`), each a copy of its
/// template from `form`, hiding it when there is nothing. Copies are numbered
/// from `first`: several rows in one form each start far apart (0, 100, ...),
/// or their counts' ids would clash.
pub fn show_coins(dom: &mut Dom, form: &str, into: &str, first: usize, pieces: Vec<CoinPiece>) {
    let into = format!("{form}/{into}");
    dom.set_flag(&into, "hidden", pieces.is_empty());
    dom.empty(&into);
    for (at, piece) in (first..).zip(pieces) {
        match piece {
            CoinPiece::Coin(metal, count) => {
                let (template, count_id) = metal.template();
                dom.copy_template(&format!("{form}/{template}"), &into, at);
                dom.set_text(&format!("{form}/{count_id}.{at}"), count.to_string());
            }
            CoinPiece::Words(text) => {
                dom.copy_template(&format!("{form}/value_words"), &into, at);
                dom.set_text(&format!("{form}/value_words.{at}"), text);
            }
        }
    }
}
