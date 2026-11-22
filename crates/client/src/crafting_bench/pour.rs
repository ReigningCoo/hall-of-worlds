//! Pouring at a bar (tavern furniture plan PR 3): a bar counter's panel also
//! lists every drink stored in its room (`shared::bar::room_storage`, from
//! what the server told of each piece, `common::Benches`), each with Pour,
//! and a dropdown of the carried vessels to fill. Pour asks the server
//! (`BenchAsk::Pour`), which checks everything by the pour rule; the list
//! follows what it tells next. First guesses for the owner to tune.

use bevy::prelude::*;
use shared::bar::{drinks_in, room_storage, vessels_to_fill};
use shared::furniture::Holder;
use shared::id::Id;
use shared::item::Item;
use shared::protocol::{BenchAsk, ClientMessage};

use super::BenchPanel;
use crate::common::{Benches, CurrentDungeon};
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};
use crate::inventory::Carried;

/// What the open bar panel lists: for which panel, the drinks (by id, with
/// their rows' words) and the vessels to fill, and which is chosen.
#[derive(Resource, Default)]
pub(super) struct BarList {
    panel: Option<Entity>,
    drinks: Vec<(Id, String)>,
    vessels: Vec<(Id, String)>,
    chosen: usize,
}

/// Millilitres as litres to a tenth: "19.5 L".
fn litres(ml: u32) -> String {
    format!("{}.{} L", ml / 1000, ml % 1000 / 100)
}

/// A row's words for a container: its name, and how much is in it.
fn words(item: &Item) -> String {
    match shared::pour::Held::of(item) {
        Some(held) if held.ml > 0 => format!("{}, {}", item.name(), litres(held.ml)),
        _ => item.name(),
    }
}

/// Lists the drinks and vessels again whenever they change.
pub(super) fn show_drinks(
    panel: Res<BenchPanel>,
    (benches, dungeon, carried): (Res<Benches>, Res<CurrentDungeon>, Res<Carried>),
    mut list: ResMut<BarList>,
    mut dom: Dom,
) {
    let Some(open) = &panel.0 else {
        list.panel = None;
        return;
    };
    if !matches!(Holder::of(open.piece.furniture), Some(Holder::Bar(_))) {
        return;
    }
    let fresh = list.panel != Some(open.panel);
    let changed = benches.is_changed() || dungeon.is_changed() || carried.is_changed();
    if !fresh && !changed {
        return;
    }
    let level = &dungeon.level;
    let stored = room_storage(level, &open.piece);
    let drinks: Vec<(Id, String)> = (stored.iter())
        .filter_map(|piece| benches.0.get(&piece.at))
        .flat_map(|held| {
            drinks_in(held)
                .map(|item| (item.id, words(item)))
                .collect::<Vec<_>>()
        })
        .collect();
    let carried_vessels = vessels_to_fill(&carried.0);
    let vessels: Vec<(Id, String)> = (carried_vessels.iter())
        .map(|item| (item.id, words(item)))
        .collect();
    // The first with room, until one is chosen.
    let room = |item: &&Item| shared::pour::Held::of(item).is_some_and(|held| held.room > 0);
    let first_with_room = carried_vessels.iter().position(room).unwrap_or(0);
    if fresh {
        list.panel = Some(open.panel);
        list.drinks.clear();
        list.vessels.clear();
        list.chosen = 0;
        dom.set_flag("bench/bar", "hidden", false);
    }
    if vessels != list.vessels {
        let kept = list.vessels.get(list.chosen).map(|(id, _)| *id);
        let kept_at = kept.and_then(|id| vessels.iter().position(|(v, _)| *v == id));
        list.chosen = kept_at.unwrap_or(first_with_room);
        let rows: Vec<String> = match vessels.is_empty() {
            true => vec!["No vessel carried".to_owned()],
            false => vessels.iter().map(|(_, row)| row.clone()).collect(),
        };
        let options = rows.into_iter().enumerate();
        dom.set_options("bench/into", options.map(|(at, row)| (at.to_string(), row)));
        dom.set("bench/into", "value", &list.chosen.to_string());
        list.vessels = vessels;
    }
    if drinks != list.drinks {
        dom.empty("bench/drinks");
        dom.set_flag("bench/no_drinks", "hidden", !drinks.is_empty());
        dom.set_flag("bench/drinks_scroll", "hidden", drinks.is_empty());
        for (at, (_, row)) in drinks.iter().enumerate() {
            dom.copy_template("bench/drink", "bench/drinks", at);
            dom.set_text(&format!("bench/name.{at}"), row);
        }
        list.drinks = drinks;
    }
}

/// The vessel chosen, and Pour asking the server.
pub(super) fn answer_drinks(
    mut events: MessageReader<UiEvent>,
    panel: Res<BenchPanel>,
    mut list: ResMut<BarList>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("bench", name) = event.id.parts() else {
            continue;
        };
        match (name, &event.value) {
            ("into", UiValue::Choice { index, .. }) => list.chosen = *index,
            ("pour", UiValue::Pressed) => {
                let Some(open) = &panel.0 else {
                    continue;
                };
                let from = (event.id.index()).and_then(|at| list.drinks.get(at));
                let into = list.vessels.get(list.chosen);
                if let (Some((from, _)), Some((into, _))) = (from, into) {
                    let ask = BenchAsk::Pour {
                        bench: open.piece.at,
                        from: *from,
                        into: *into,
                    };
                    to_server.write(ToServer(ClientMessage::Bench(ask)));
                }
            }
            _ => {}
        }
    }
}
