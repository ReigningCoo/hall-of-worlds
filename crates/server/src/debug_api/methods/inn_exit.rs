//! `server.inn_exit`: where a player in The Way Inn comes out into the Hall
//! (`CharacterWorldSave::inn_return`), read, set or rerolled, to test leaving
//! as a new character does (`sites::inn::first_exit`; `HOW_DEBUG_INN_EXIT`
//! sets it for every new character).

use bevy::prelude::*;
use serde::Deserialize;
use shared::rng::Seed;
use shared::spot::Spot;

use super::super::{BrpResult, Value, json, params, refused};
use crate::Connections;
use crate::sites::inn::first_exit::{rolled, tile_from};

#[derive(Deserialize)]
pub(super) struct InnExitParams {
    /// Which connected player, in `server.state`'s order.
    #[serde(default)]
    player: usize,
    /// A Hall tile to come out on, `"x"` or `"x,y"` as the variable takes it.
    set: Option<String>,
    /// Somewhere else along the walkway, rolled from this (any number).
    reroll: Option<u64>,
}

/// `server.inn_exit { "player": 0, "set": "1000,1" }` or `{ "reroll": 7 }`:
/// the Hall tile the player comes out on when they leave the inn, after
/// setting or rerolling it. Only while they are in the inn: it is kept only
/// until they leave.
pub(super) fn inn_exit(
    In(request): In<Option<Value>>,
    mut connections: ResMut<Connections>,
) -> BrpResult {
    let InnExitParams {
        player,
        set,
        reroll,
    } = params(request)?;
    let client = connections
        .clients_mut()
        .get_mut(player)
        .ok_or_else(|| refused(format!("no player {player}")))?;
    if !crate::sites::in_inn(client) {
        return Err(refused("not in the inn".to_string()));
    }
    let tile = match (set, reroll) {
        (Some(text), _) => {
            Some(tile_from(&text).ok_or_else(|| refused(format!("not a Hall tile: {text}")))?)
        }
        (None, Some(seed)) => Some(rolled(Seed::new(seed).derive("inn exit reroll"))),
        (None, None) => None,
    };
    if let Some(tile) = tile {
        client.character.inn_return = Some(Spot::middle_of(tile));
        client.unsaved = true;
    }
    let exit = client.character.inn_return.map(Spot::cell);
    Ok(json!({
        "character": client.character_id,
        "exit": exit.map(|(x, y)| json!({ "x": x, "y": y })),
    }))
}
