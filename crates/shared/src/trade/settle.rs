//! Settling a shop trade: what is due in copper, and what the player offers
//! to pay instead. More than is due is a tip; less is haggling, allowed up
//! to 5% short at full Trading skill and less with less skill, a fixed rule
//! with no dice so the client can say beforehand (owner).

use serde::{Deserialize, Serialize};

use super::Refusal;
use crate::value::{FINE_PER_COPPER, copper_shown};

/// The most short a haggle may be, at full skill, in percent of what is due.
pub const HAGGLE_PERCENT: u64 = 5;

/// Trading practice for a modest tip or an accepted haggle, and the negative
/// practice for a tip too big (Claude's, to balance).
pub const TRADE_PRACTICE: u32 = 8;

/// The largest tip that is practice, in percent of what is due: a bigger
/// one is negative practice (owner).
pub const TIP_PERCENT: u64 = 5;

/// What the player pays in copper, negative when the shop pays them, from
/// what the shop asks and pays in fine units. Rounded up when the player
/// pays and down when the shop does, so a pinch of something can never be
/// sold for a whole copper again and again (Claude's).
pub fn due(asks: u64, pays: u64) -> i64 {
    match asks >= pays {
        true => copper_shown(asks - pays) as i64,
        false => -(((pays - asks) / FINE_PER_COPPER) as i64),
    }
}

/// How many copper short of `due` (either way) a haggle may be with this much
/// Trading (a whole percentage).
pub fn haggle_room(due: i64, trading: u32) -> u64 {
    due.unsigned_abs() * HAGGLE_PERCENT * u64::from(trading.min(100)) / 10_000
}

/// How a trade was paid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Payment {
    /// What was due.
    Exact,
    /// This many copper more: `generous` past `TIP_PERCENT`.
    Tip { copper: u64, generous: bool },
    /// This many copper less, accepted.
    Haggled { copper: u64 },
}

/// What is paid (copper, negative when the shop pays) and how, when the
/// player offers to pay `paying` (none: what is due).
pub fn judge(due: i64, paying: Option<i64>, trading: u32) -> Result<(i64, Payment), Refusal> {
    let Some(paying) = paying else {
        return Ok((due, Payment::Exact));
    };
    let payment = match paying.cmp(&due) {
        std::cmp::Ordering::Equal => Payment::Exact,
        std::cmp::Ordering::Greater => {
            let copper = (paying - due).unsigned_abs();
            let generous = copper * 100 > TIP_PERCENT * due.unsigned_abs();
            Payment::Tip { copper, generous }
        }
        std::cmp::Ordering::Less => {
            let copper = (due - paying).unsigned_abs();
            if copper > haggle_room(due, trading) {
                return Err(Refusal::TooShort);
            }
            Payment::Haggled { copper }
        }
    };
    Ok((paying, payment))
}
