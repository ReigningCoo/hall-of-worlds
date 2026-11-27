//! Who to trade with: the dropdown at the top lists the shops whose doors are
//! within 5 cells (`shared::trade::shop_in_reach`), nearest first, and says
//! which the trade is with. It can't be changed while anything is in the
//! trade, either side. Choosing one asks the server to trade with it; the
//! window opening chooses the nearest.

use bevy::prelude::*;
use shared::protocol::ClientMessage;
use shared::spot::Spot;
use shared::trade::shop_in_reach;

use super::state::{Shop, TradeWindow};
use crate::common::{CurrentDungeon, SpotPosition, Trading};
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};
use crate::player::Player;
use crate::widgets::UiId;

/// What the dropdown last showed: the shops, which is chosen, and whether it
/// can be used.
type Shown = (Vec<Shop>, Option<usize>, bool);

/// The dropdown's words when no shop is near enough.
const NOBODY: &str = "Nobody near enough";

/// The shops in reach of `at`, nearest first, named by their kind (with a
/// number when two are alike).
pub(super) fn shops_near(
    at: Spot,
    dungeon: &CurrentDungeon,
    keepers: &std::collections::BTreeMap<u16, shared::id::Id>,
) -> Vec<Shop> {
    let mut near: Vec<_> = (dungeon.buildings.iter())
        .filter_map(|look| {
            let &keeper = keepers.get(&look.id)?;
            shop_in_reach(at, look.door).then_some((look, keeper))
        })
        .collect();
    near.sort_by_key(|(look, _)| at.distance(Spot::middle_of(look.door)));
    let mut shops: Vec<Shop> = Vec::new();
    for (look, keeper) in near {
        let name = look.kind.name();
        let alike = shops
            .iter()
            .filter(|shop| shop.name.starts_with(name))
            .count();
        let name = match alike {
            0 => name.to_owned(),
            n => format!("{name} {}", n + 1),
        };
        shops.push(Shop { keeper, name });
    }
    shops
}

/// Keeps the dropdown's list, choice and whether it can be used up to date,
/// and on opening chooses the nearest shop.
pub(super) fn show_shops(
    player: Single<&SpotPosition, With<Player>>,
    dungeon: Res<CurrentDungeon>,
    trading: Res<Trading>,
    mut window: ResMut<TradeWindow>,
    mut to_server: MessageWriter<ToServer>,
    mut dom: Dom,
    mut shown: Local<Option<Shown>>,
) {
    if window.document.is_none() {
        *shown = None;
        return;
    }
    let shops = shops_near(player.0, &dungeon, &window.keepers);
    if window.just_opened {
        window.just_opened = false;
        if let Some(nearest) = shops.first().filter(|_| trading.with.is_none()) {
            let with = nearest.keeper;
            to_server.write(ToServer(ClientMessage::OpenTrade { with }));
        }
    }
    let chosen = (shops.iter()).position(|shop| Some(shop.keeper) == trading.with);
    let usable = !shops.is_empty() && !trading.is_busy();
    let now = (shops.clone(), chosen, usable);
    if shown.as_ref() == Some(&now) {
        return;
    }
    let mut options: Vec<(String, String)> = (shops.iter().enumerate())
        .map(|(at, shop)| (at.to_string(), shop.name.clone()))
        .collect();
    if chosen.is_none() {
        // Nobody chosen (or no one near): a first line saying so.
        let words = if shops.is_empty() {
            NOBODY
        } else {
            "Choose a shop"
        };
        options.insert(0, ("none".to_owned(), words.to_owned()));
    }
    dom.set_options("trade/with", options);
    let value = chosen.map_or("none".to_owned(), |at| at.to_string());
    dom.set("trade/with", "value", &value);
    dom.set_flag("trade/with", "disabled", !usable);
    window.shops = shops;
    *shown = Some(now);
}

/// Choosing a shop in the dropdown asks to trade with it.
pub(super) fn choose_shop(
    mut events: MessageReader<UiEvent>,
    mut window: ResMut<TradeWindow>,
    trading: Res<Trading>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        if event.id != UiId::new("trade", "with") {
            continue;
        }
        let UiValue::Choice { value, .. } = &event.value else {
            continue;
        };
        let shop = value
            .parse::<usize>()
            .ok()
            .and_then(|at| window.shops.get(at));
        if let Some(shop) = shop.filter(|shop| Some(shop.keeper) != trading.with) {
            let with = shop.keeper;
            // What was said of the last trade has gone with it.
            window.said = None;
            window.dirty = true;
            to_server.write(ToServer(ClientMessage::OpenTrade { with }));
        }
    }
}
