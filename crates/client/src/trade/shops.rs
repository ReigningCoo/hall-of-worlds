//! Who to trade with: the dropdown at the top lists the players near enough,
//! the shops whose doors are within 5 cells (`shared::trade::shop_in_reach`)
//! and the NPCs near enough (`people`), each nearest first, and says which
//! the trade is with; anyone set never to trade is greyed out. It can't be
//! changed while anything is in the trade, either side, or the receive box
//! holds anything. Choosing one asks the server to trade with them; the
//! window opening chooses the nearest shop.

use bevy::prelude::*;
use shared::protocol::ClientMessage;
use shared::spot::Spot;
use shared::trade::shop_in_reach;

use super::people::{Seen, numbered, people_near};
use super::state::{Partner, TradeWindow};
use crate::common::{CurrentDungeon, HoverName, Other, SpotPosition, Trading};
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};
use crate::player::Player;
use crate::widgets::UiId;

/// What the dropdown last showed: who, which is chosen, and whether it can
/// be used.
type Shown = (Vec<Partner>, Option<usize>, bool);

/// The dropdown's words when no one is near enough.
const NOBODY: &str = "Nobody near enough";

/// The shops in reach of `at`, nearest first, named by their kind (with a
/// number when two are alike).
pub(super) fn shops_near(
    at: Spot,
    dungeon: &CurrentDungeon,
    keepers: &std::collections::BTreeMap<u16, shared::id::Id>,
) -> Vec<Partner> {
    let mut near: Vec<_> = (dungeon.buildings.iter())
        .filter_map(|look| {
            let &keeper = keepers.get(&look.id)?;
            shop_in_reach(at, look.door).then_some((look, keeper))
        })
        .collect();
    near.sort_by_key(|(look, _)| at.distance(Spot::middle_of(look.door)));
    let mut shops: Vec<Partner> = Vec::new();
    for (look, keeper) in near {
        let name = numbered(&shops, look.kind.name());
        shops.push(Partner {
            with: keeper,
            name,
            never: false,
        });
    }
    shops
}

/// Keeps the dropdown's list, choice and whether it can be used up to date,
/// and on opening chooses the nearest shop.
#[allow(clippy::too_many_arguments)]
pub(super) fn show_shops(
    player: Single<&SpotPosition, With<Player>>,
    others: Query<(&Other, &SpotPosition, &HoverName)>,
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
    let seen: Vec<Seen> = (others.iter())
        .map(|(other, at, name)| Seen {
            id: other.id,
            kind: other.kind,
            feeling: other.feeling,
            player: other.player,
            never_trades: other.never_trades,
            name: name.0.clone(),
            at: at.0,
        })
        .collect();
    let (players, npcs) = people_near(player.0, &seen);
    let shops = shops_near(player.0, &dungeon, &window.keepers);
    if window.just_opened {
        window.just_opened = false;
        let free = trading.with.is_none() && !window.receiving();
        if let Some(nearest) = shops.first().filter(|_| free) {
            let with = nearest.with;
            to_server.write(ToServer(ClientMessage::OpenTrade { with }));
        }
    }
    let listed: Vec<Partner> = [players, shops, npcs].concat();
    let chosen = (listed.iter()).position(|one| Some(one.with) == trading.with);
    let usable = !listed.is_empty() && !trading.is_busy() && !window.receiving();
    let now = (listed.clone(), chosen, usable);
    if shown.as_ref() == Some(&now) {
        return;
    }
    let mut options: Vec<(String, String)> = (listed.iter().enumerate())
        .map(|(at, one)| (at.to_string(), one.name.clone()))
        .collect();
    let mut greyed: Vec<bool> = listed.iter().map(|one| one.never).collect();
    if chosen.is_none() {
        // Nobody chosen (or no one near): a first line saying so.
        let words = match listed.is_empty() {
            true => NOBODY,
            false => "Choose someone",
        };
        options.insert(0, ("none".to_owned(), words.to_owned()));
        greyed.insert(0, false);
    }
    dom.set_options("trade/with", options);
    dom.set_options_greyed("trade/with", greyed);
    let value = chosen.map_or("none".to_owned(), |at| at.to_string());
    dom.set("trade/with", "value", &value);
    dom.set_flag("trade/with", "disabled", !usable);
    window.shops = listed;
    *shown = Some(now);
}

/// Choosing someone in the dropdown asks to trade with them.
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
        let one = value
            .parse::<usize>()
            .ok()
            .and_then(|at| window.shops.get(at));
        let new = |one: &&Partner| Some(one.with) != trading.with && !one.never;
        if let Some(one) = one.filter(new) {
            let with = one.with;
            // What was said of the last trade has gone with it.
            window.said = None;
            window.dirty = true;
            to_server.write(ToServer(ClientMessage::OpenTrade { with }));
        }
    }
}
