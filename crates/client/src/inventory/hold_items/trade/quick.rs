//! A right-click (by touch, two taps) while trading (trading tweaks PR 3,
//! owner): in place of the usual quick move, something carried goes into the
//! trade (sold, or offered in the first free place on the own side), or out
//! of it again if it is in already; something on either side comes out of the
//! trade; and a shop's stock goes into the Buying box's first free place
//! (`TradeAsk::Buy` with no place, growing the box). The server says how it
//! stands after; nothing is shown before.

use shared::item::Inventory;
use shared::protocol::{ClientMessage, TradeAsk};

use super::super::super::hold::QuickFrom;
use super::super::Outbox;
use crate::common::{buying_box, own_side};
use crate::connection::ToServer;

/// Does the quick move in the trade, if one is open and this is for it:
/// `true` if it was (even when nothing could be done).
pub(in crate::inventory::hold_items) fn quick_in_trade(
    from: &QuickFrom,
    carried: &Inventory,
    out: &mut Outbox,
) -> bool {
    let trading = &out.trading;
    let Some(with) = trading.with else {
        return false;
    };
    let ask = match *from {
        QuickFrom::Ground(_) => return false,
        // Only what lies in a grid, not in the bin, not worn.
        QuickFrom::Carried(id) if carried.in_bin(id) || carried.placed(id).is_none() => {
            return false;
        }
        QuickFrom::Carried(id) if trading.is_selling(id) => Some(TradeAsk::Unsell { item: id }),
        QuickFrom::Carried(id) => match &trading.between {
            Some(between) => (between.mine.first_free(id, carried)).map(TradeAsk::Offer),
            // Still asking them: nothing to put it on yet.
            None if trading.asking => None,
            None => Some(TradeAsk::Sell { item: id }),
        },
        QuickFrom::Stock { with: from, item } if from == own_side() => {
            Some(TradeAsk::Unsell { item })
        }
        QuickFrom::Stock { with: from, item } if from == buying_box() => {
            Some(TradeAsk::Unbuy { item })
        }
        QuickFrom::Stock { with: from, item } if from == with && trading.between.is_none() => {
            Some(TradeAsk::Buy { item, at: None })
        }
        // Another grid (the receive box, a character's side): nothing.
        QuickFrom::Stock { .. } => None,
    };
    if let Some(ask) = ask {
        out.to_server.write(ToServer(ClientMessage::Trade(ask)));
    }
    true
}
