//! Eating, as dropping food on the clothes panel or using it from the action
//! bar does.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{BrpResult, Value, params, refused};
use shared::item::ItemKind;
use shared::protocol::ClientMessage;

use super::done;
use crate::connection::ToServer;
use crate::inventory::Carried;

#[derive(Deserialize)]
pub(super) struct EatItemParams {
    /// By name: `"Fruit"`, `"Bread"`, `"RawMeat"`, ...
    kind: ItemKind,
}

/// `client.eat_item { "kind": "Fruit" }`: eats one of the first stack of this
/// kind in the inventory (`ClientMessage::Consume`).
pub(super) fn eat_item(
    In(request): In<Option<Value>>,
    carried: Res<Carried>,
    mut to_server: MessageWriter<ToServer>,
) -> BrpResult {
    let EatItemParams { kind } = params(request)?;
    let found = carried
        .0
        .items
        .iter()
        .find(|placed| placed.item.kind == kind);
    let Some(placed) = found else {
        return Err(refused(format!("no {kind:?} in the inventory")));
    };
    let item = placed.item.id;
    to_server.write(ToServer(ClientMessage::Consume { item }));
    done()
}
