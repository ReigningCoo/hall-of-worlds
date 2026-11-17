//! Picking herbs: a click (or tap) on a patch that can be harvested sends the
//! player there (`click::pick_on_click`, which sends `TravelTo`) and asks the
//! server to harvest it (`ClientMessage::PickHerb`) at once if it is beside
//! or under them, or else on arriving. The server says what came of it
//! (`HerbNews`) and sends the patch again.

use bevy::prelude::*;
use shared::protocol::ClientMessage;

use crate::common::{CurrentDungeon, HerbTable, MapPosition, ServerClock};
use crate::connection::ToServer;
use crate::player::{Arrived, Player, TakeStep, TravelTo, Walking};

/// The patch the player is going to pick.
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct HerbHeading(Option<(i64, i64)>);

/// Whether the patch at `(x, y)` can be harvested now.
fn ready(
    dungeon: &CurrentDungeon,
    table: &HerbTable,
    clock: &ServerClock,
    (x, y): (i64, i64),
) -> bool {
    let now = clock.minutes.max(0.0) as u64;
    dungeon
        .level
        .herb_at(x, y)
        .and_then(|patch| Some((patch, table.get(&patch.herb)?)))
        .is_some_and(|(patch, def)| patch.can_pick(def, now))
}

/// Whether `at` is beside or under the player: the server's reach.
fn in_reach(player: &MapPosition, (x, y): (i64, i64)) -> bool {
    (x - player.x).abs().max((y - player.y).abs()) <= 1
}

/// A journey to a patch ready to pick picks it, now if it is in reach, or
/// else on arriving; any other journey, or a step of the player's own, calls
/// that off.
#[allow(clippy::too_many_arguments)]
pub(in crate::dungeon_map) fn aim_at_herbs(
    mut journeys: MessageReader<TravelTo>,
    (mut steps, mut walking): (MessageReader<TakeStep>, MessageReader<Walking>),
    player: Single<&MapPosition, With<Player>>,
    (dungeon, table, clock): (Res<CurrentDungeon>, Res<HerbTable>, Res<ServerClock>),
    mut heading: ResMut<HerbHeading>,
    mut to_server: MessageWriter<ToServer>,
) {
    if steps.read().count() + walking.read().count() > 0 {
        heading.0 = None;
    }
    for &TravelTo { x, y } in journeys.read() {
        heading.0 = None;
        if !ready(&dungeon, &table, &clock, (x, y)) {
            continue;
        }
        if in_reach(&player, (x, y)) {
            to_server.write(ToServer(ClientMessage::PickHerb { x, y }));
        } else {
            heading.0 = Some((x, y));
        }
    }
}

/// Arriving by a patch the player set out to pick picks it, if it is still
/// ready.
pub(in crate::dungeon_map) fn pick_on_arrival(
    mut arrivals: MessageReader<Arrived>,
    (dungeon, table, clock): (Res<CurrentDungeon>, Res<HerbTable>, Res<ServerClock>),
    mut heading: ResMut<HerbHeading>,
    mut to_server: MessageWriter<ToServer>,
) {
    let Some(&Arrived { x, y }) = arrivals.read().last() else {
        return;
    };
    let Some(at) = heading.0.take() else {
        return;
    };
    let here = MapPosition { x, y };
    if in_reach(&here, at) && ready(&dungeon, &table, &clock, at) {
        to_server.write(ToServer(ClientMessage::PickHerb { x: at.0, y: at.1 }));
    }
}
