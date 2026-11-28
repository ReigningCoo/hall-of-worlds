//! `client.trade`: the trade window and the trade as the client holds them
//! (open, who the dropdown lists, who the trade is with, what is sold and
//! bought with where it goes, the sums, what the window says; with a
//! character both sides, asking, being asked, the receive box and the
//! setting), and every shop on the level with its door, for walking or
//! teleporting to one. See
//! `docs/context/debug-api.md`.

use bevy::prelude::*;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json};

use super::show::middle;
use super::sides::mode;
use super::state::TradeWindow;
use crate::common::{CurrentDungeon, TradeSetting, Trading};
use crate::inventory::Carried;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.trade", trade);
}

fn trade(
    In(_): In<Option<Value>>,
    (window, trading): (Res<TradeWindow>, Res<Trading>),
    (dungeon, carried): (Res<CurrentDungeon>, Res<Carried>),
    setting: Res<TradeSetting>,
) -> BrpResult {
    let side = |side: &shared::trade::Side| {
        json!({
            "locked": side.locked,
            "offers": (side.offers.iter())
                .map(|offer| json!({"item": offer.item.to_string(), "x": offer.x, "y": offer.y}))
                .collect::<Vec<_>>(),
        })
    };
    let between = (trading.between.as_ref())
        .map(|between| json!({"mine": side(&between.mine), "theirs": side(&between.theirs)}));
    let listed: Vec<Value> = (window.shops.iter())
        .map(|one| json!({"name": one.name, "with": one.with.to_string(), "never": one.never}))
        .collect();
    let shops: Vec<Value> = (dungeon.buildings.iter())
        .filter_map(|look| {
            let keeper = window.keepers.get(&look.id)?;
            Some(json!({
                "kind": look.kind.name(),
                "door": look.door,
                "front": (look.door.0 + look.faces.0, look.door.1 + look.faces.1),
                "keeper": keeper.to_string(),
            }))
        })
        .collect();
    let buying: Vec<Value> = (trading.buying.iter())
        .map(|(buy, item)| {
            json!({
                "item": item.id.to_string(),
                "kind": format!("{:?}", item.kind),
                "to": format!("{:?}", buy.to),
                "x": buy.x,
                "y": buy.y,
            })
        })
        .collect();
    let said = middle(&trading, &carried, &window.stock);
    Ok(json!({
        "open": window.document.is_some(),
        "listed": listed,
        "mode": format!("{:?}", mode(&window, &trading)),
        "between": between,
        "asking": trading.asking,
        "asked": window.asked.as_ref().map(|(by, name)| json!({"by": by.to_string(), "name": name})),
        "received": (window.received.iter())
            .map(|item| json!({"item": item.id.to_string(), "kind": format!("{:?}", item.kind)}))
            .collect::<Vec<_>>(),
        "setting": format!("{:?}", setting.0),
        "with": trading.with.map(|id| id.to_string()),
        "selling": trading.selling.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
        "buying": buying,
        "stock": window.stock.len(),
        "shown_stock": window.theirs.as_ref().map(|grid| grid.items().count()),
        "own_shown": window.own.items().count(),
        "asks": trading.totals.asks,
        "pays": trading.totals.pays,
        "due": trading.totals.due,
        "coins": carried.0.coins_carried(),
        "middle": [said.own, said.theirs, said.gain],
        "said": window.said,
        "shops": shops,
    }))
}
