//! The trade window's words: sums of coins, refusals, and how a trade ended.

use shared::protocol::TradeEnd;
use shared::trade::{Payment, Refusal};
use shared::value::Coins;

/// Copper as coins in words, none of a coin left out: "1 gold, 5 copper";
/// "nothing" for none.
pub(super) fn coins_words(copper: u64) -> String {
    let coins = Coins::from_copper(copper);
    let parts: Vec<String> = [
        (coins.gold, "gold"),
        (coins.silver, "silver"),
        (coins.copper, "copper"),
    ]
    .into_iter()
    .filter(|&(count, _)| count > 0)
    .map(|(count, metal)| format!("{count} {metal}"))
    .collect();
    match parts.is_empty() {
        true => "nothing".to_owned(),
        false => parts.join(", "),
    }
}

/// What the bottom right says of what is due (negative: to be paid).
pub(super) fn due_words(due: i64) -> String {
    match due {
        0 => "Nothing to pay".to_owned(),
        due if due > 0 => format!("You pay {}", coins_words(due as u64)),
        due => format!("You receive {}", coins_words(due.unsigned_abs())),
    }
}

/// Why a change, or completing, was refused.
pub(super) fn refusal_words(why: Refusal) -> &'static str {
    match why {
        Refusal::NotStocked => "They no longer have that.",
        Refusal::NotCarried => "You no longer have that.",
        Refusal::NotForSale => "They won't take that.",
        Refusal::AlreadyIn => "That is in the trade already.",
        Refusal::TooShort => "That is less than they will take.",
        Refusal::CantAfford => "You can't pay for that.",
        Refusal::NoRoom => "You have no room for it.",
        Refusal::NotHere => "You are too far away to trade.",
        Refusal::Nothing => "There is nothing to trade.",
    }
}

/// How a trade ended, if worth saying: what was paid or received, or that
/// the player walked away.
pub(super) fn ended_words(why: TradeEnd) -> Option<String> {
    match why {
        TradeEnd::Completed { paid, payment } => {
            let sum = match paid {
                0 => "Traded.".to_owned(),
                paid if paid > 0 => format!("Paid {}.", coins_words(paid as u64)),
                paid => format!("Received {}.", coins_words(paid.unsigned_abs())),
            };
            Some(match payment {
                Payment::Haggled { .. } => format!("{sum} They let you haggle."),
                Payment::Tip { .. } => format!("{sum} They thank you for the tip."),
                Payment::Exact => sum,
            })
        }
        TradeEnd::Closed => None,
        TradeEnd::OutOfReach => Some("You walked away: nothing was traded.".to_owned()),
    }
}
