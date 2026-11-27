//! Telling both sides of a trade with a character how it stands: each its
//! own side and the other's, with the other's things (`TradeStock`) first.
//! An NPC's side is empty and always locked in.

use shared::item::Item;
use shared::protocol::{ServerMessage, TradeNews};
use shared::trade::{Between, Side};

use crate::connections::Client;

/// Tells client `i`, and the other player `j` if any, the trade as each sees it.
pub(super) fn tell_pair(clients: &mut [Client], i: usize, j: Option<usize>) {
    tell_state(clients, i, j);
    if let Some(j) = j {
        tell_state(clients, j, Some(i));
    }
}

/// Tells client `i` its trade with `j` (none: an NPC) as it stands.
pub(super) fn tell_state(clients: &mut [Client], i: usize, j: Option<usize>) {
    let Some(dealing) = clients[i].trade.dealing.clone() else {
        return;
    };
    let (theirs, items) = match j.and_then(|j| Some((j, clients[j].trade.dealing.as_ref()?))) {
        Some((j, other)) => (other.side.clone(), offered(&clients[j], &other.side)),
        None => (npc_side(), Vec::new()),
    };
    let client = &mut clients[i];
    let items = client.as_seen(items);
    let with = dealing.with;
    let between = Between {
        with,
        mine: dealing.side,
        theirs,
    };
    client.link.send(&ServerMessage::TradeStock { with, items });
    client
        .link
        .send(&ServerMessage::Trade(TradeNews::Between(between)));
}

/// An NPC's side: nothing, locked in.
pub(super) fn npc_side() -> Side {
    Side {
        offers: Vec::new(),
        locked: true,
    }
}

/// The things offered on a side, as carried (with all in them).
fn offered(client: &Client, side: &Side) -> Vec<Item> {
    let found = side
        .offers
        .iter()
        .map(|offer| client.inventory.find(offer.item));
    found.flatten().cloned().collect()
}
