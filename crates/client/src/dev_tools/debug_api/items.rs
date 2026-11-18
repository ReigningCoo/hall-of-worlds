//! Giving items, putting them on, and changing the character's creature.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{BrpResult, Value, params, refused};
use shared::creature::CreatureKind;
use shared::item::{Favour, ItemKind, Quality};
use shared::protocol::ClientMessage;

use super::done;
use crate::connection::ToServer;
use crate::inventory::Carried;

#[derive(Deserialize)]
pub(super) struct GiveItemParams {
    /// By name: `"Ring"`, `"Backpack"`, ...
    kind: ItemKind,
    #[serde(default)]
    quality: Quality,
    #[serde(default)]
    magical: bool,
    #[serde(default)]
    set: bool,
    /// `"Blessed"` or `"Cursed"`.
    favour: Option<Favour>,
    /// For `"Herb"`: the herb's key (`"roseroot"`); any herb if missing.
    herb: Option<String>,
    /// For `"Fruit"`: the fruit's key (`"apple"`); any fruit if missing.
    fruit: Option<String>,
    /// For `"Vegetable"`: the vegetable's key (`"carrot"`); any if missing.
    vegetable: Option<String>,
    /// For `"Flour"`: the grain's key (`"rye"`); any grain's if missing.
    flour: Option<String>,
    /// For `"Bottle"`: what it holds, `{ "Water": "Moon" }` (`Plain`, `Cave`,
    /// `Moon`, `Salt`), `{ "Oil": "Nut" }` (`Vegetable`, `Nut`, `Seed`) or
    /// `"Potion"`; by `effects` if missing.
    liquid: Option<shared::potion::LiquidKind>,
    /// For `"Bottle"`: how many effects the potion holds; empty if missing
    /// and no liquid is asked for.
    effects: Option<u8>,
    /// For `"Vessel"`: which (`"Tankard"`, `"WineBottle"`, ...); any if missing.
    vessel: Option<shared::vessel::VesselKind>,
    /// How many, alike (1 if missing).
    count: Option<u32>,
}

/// `client.give_item { "kind": "Ring", "quality": "Rare", "magical": true }`
/// (or `{ "kind": "Herb", "herb": "roseroot", "count": 3 }`,
/// `{ "kind": "Fruit", "fruit": "watermelon" }`,
/// `{ "kind": "Vegetable", "vegetable": "carrot" }`,
/// `{ "kind": "Flour", "flour": "rye" }`, or
/// `{ "kind": "Bottle", "effects": 2 }` for a potion,
/// `{ "kind": "Vessel", "vessel": "Tankard", "liquid": { "Water": "Plain" } }`, or
/// `{ "kind": "Bottle", "liquid": { "Water": "Moon" } }`): into the
/// inventory, as Give Item does. Saved.
pub(super) fn give_item(
    In(request): In<Option<Value>>,
    mut to_server: MessageWriter<ToServer>,
) -> BrpResult {
    let GiveItemParams {
        kind,
        quality,
        magical,
        set,
        favour,
        herb,
        fruit,
        vegetable,
        flour,
        liquid,
        effects,
        vessel,
        count,
    } = params(request)?;
    to_server.write(ToServer(ClientMessage::DevGiveItem(
        shared::protocol::DevGive {
            kind,
            size: None,
            magical,
            quality,
            set,
            favour,
            seed: None,
            count: count.unwrap_or(1),
            identical: true,
            request: shared::item_form::RollRequest {
                liquid,
                effects,
                vessel,
                ..shared::item_form::RollRequest::NONE
            },
            herb: herb.or(fruit).or(vegetable).or(flour),
            to: None,
        },
    )));
    done()
}

#[derive(Deserialize)]
pub(super) struct SetCreatureParams {
    /// By name: `"Human"`, ...
    kind: CreatureKind,
    /// For good unless `"Temporary"`.
    #[serde(default)]
    how: shared::protocol::KindChange,
}

/// `client.set_creature { "kind": "Human", "how"?: "Permanent" | "Temporary" }`: as
/// Change Creature
/// (`HOW_DEBUG_CREATURE`): what the new body can't wear, or has no room for,
/// drops on the ground; refused on the world map and in the Hall.
pub(super) fn set_creature(
    In(request): In<Option<Value>>,
    mut to_server: MessageWriter<ToServer>,
) -> BrpResult {
    let SetCreatureParams { kind, how } = params(request)?;
    to_server.write(ToServer(ClientMessage::DevSetCreature { kind, how }));
    done()
}

#[derive(Deserialize)]
pub(super) struct WearItemParams {
    /// By name: `"FullArmour"`, `"Helmet"`, ...
    kind: ItemKind,
}

/// `client.wear_item { "kind": "FullArmour" }`: puts on the first thing of this kind
/// in the inventory, as dragging it onto the clothes panel does.
pub(super) fn wear_item(
    In(request): In<Option<Value>>,
    carried: Res<Carried>,
    mut to_server: MessageWriter<ToServer>,
) -> BrpResult {
    let WearItemParams { kind } = params(request)?;
    let Some(placed) = carried
        .0
        .items
        .iter()
        .find(|placed| placed.item.kind == kind)
    else {
        return Err(refused(format!("no {kind:?} in the inventory")));
    };
    let item = placed.item.id;
    to_server.write(ToServer(ClientMessage::WearItem { item }));
    done()
}
