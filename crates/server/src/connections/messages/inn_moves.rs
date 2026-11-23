//! Into or out of the inn once a client's messages are read, and a dev tool
//! that was waiting for them to be out.

use super::super::{Client, answer};
use crate::accounts::Accounts;
use crate::{ServerOptions, ServerWorlds, sites};

/// Client `i` into or out of the inn, if it asked (`sites::move_inn`), then any
/// dev tool waiting for it to be out (`InnState::after`). `false` if gone.
pub(super) fn moved_in_or_out(
    clients: &mut [Client],
    i: usize,
    worlds: &mut ServerWorlds,
    accounts: &mut Accounts,
    options: &ServerOptions,
) -> bool {
    if !sites::move_inn(clients, i, worlds) {
        return false;
    }
    let client = &mut clients[i];
    match client.inn.after.take() {
        Some(message) => {
            let world = worlds.of(client.world);
            answer(client, message, world, accounts, options, None)
        }
        None => true,
    }
}
