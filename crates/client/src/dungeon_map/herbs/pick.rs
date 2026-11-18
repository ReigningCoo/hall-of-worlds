//! Picking herbs, fruit and vegetables: a click (or tap) on a patch or plant
//! that can be harvested sends the
//! player there (`click::pick_on_click`, which sends `TravelTo`) and asks the
//! server to harvest it (`ClientMessage::Pick`) at once if it is beside
//! or under them, or else on arriving. The server says what came of it
//! (`PickNews`) and sends the patch again. A fruit plant with a trunk can't be
//! walked onto, so the journey there goes on to the free cell beside it
//! nearest the player instead.

use bevy::prelude::*;
use shared::fruit::Fruits;
use shared::protocol::ClientMessage;
use shared::vegetable::Vegetables;

use crate::common::{CurrentDungeon, HerbTable, MapPosition, ServerClock};
use crate::connection::ToServer;
use crate::player::{Arrived, Player, TakeStep, TravelTo, Walking};

/// The patch or plant the player is going to pick, and the cell beside a
/// trunk they were sent on to instead, if they were.
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct HerbHeading {
    target: Option<(i64, i64)>,
    beside: Option<(i64, i64)>,
}

/// The seen cell beside the trunk at `at` that can be walked on, nearest the
/// player.
fn beside_trunk(
    dungeon: &CurrentDungeon,
    player: &MapPosition,
    at: (i64, i64),
) -> Option<(i64, i64)> {
    let level = &dungeon.level;
    let near = (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (at.0 + dx, at.1 + dy)));
    near.filter(|&(x, y)| level.is_walkable(x, y) && dungeon.remembered.is_seen(x, y))
        .min_by_key(|&(x, y)| (x - player.x).pow(2) + (y - player.y).pow(2))
}

/// Whether the herb patch, fruit plant or vegetable at `(x, y)` can be
/// harvested now.
fn ready(
    dungeon: &CurrentDungeon,
    table: &HerbTable,
    clock: &ServerClock,
    (x, y): (i64, i64),
) -> bool {
    let now = clock.minutes.max(0.0) as u64;
    let level = &dungeon.level;
    let herb = (level.herb_at(x, y))
        .and_then(|patch| Some((patch, table.get(&patch.herb)?)))
        .is_some_and(|(patch, def)| patch.can_pick(def, now));
    let fruit = (level.fruit_at(x, y))
        .and_then(|plant| Some((plant, Fruits::built_in().get(&plant.fruit)?)))
        .is_some_and(|(plant, def)| plant.can_pick(def, now));
    let vegetable = (level.vegetable_at(x, y))
        .and_then(|plant| Some((plant, Vegetables::built_in().get(&plant.vegetable)?)))
        .is_some_and(|(plant, def)| plant.can_pick(def, now));
    herb || fruit || vegetable
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
    mut commands: Commands,
) {
    if steps.read().count() + walking.read().count() > 0 {
        *heading = HerbHeading::default();
    }
    for &TravelTo { x, y } in journeys.read() {
        // The journey on beside a trunk, sent below: still going to pick.
        if heading.beside.take() == Some((x, y)) {
            continue;
        }
        heading.target = None;
        if !ready(&dungeon, &table, &clock, (x, y)) {
            continue;
        }
        if in_reach(&player, (x, y)) {
            to_server.write(ToServer(ClientMessage::Pick { x, y }));
            continue;
        }
        heading.target = Some((x, y));
        if dungeon.level.has_trunk(x, y)
            && let Some((bx, by)) = beside_trunk(&dungeon, &player, (x, y))
        {
            heading.beside = Some((bx, by));
            commands.write_message(TravelTo { x: bx, y: by });
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
    let Some(at) = heading.target.take() else {
        return;
    };
    let here = MapPosition { x, y };
    if in_reach(&here, at) && ready(&dungeon, &table, &clock, at) {
        to_server.write(ToServer(ClientMessage::Pick { x: at.0, y: at.1 }));
    }
}
