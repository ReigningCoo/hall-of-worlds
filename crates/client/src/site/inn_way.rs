//! Walking off the Hall's edge at one of its hidden ways into The Way Inn
//! (`hall::inn_way_behind`): the keys (or the pad) held outward on the edge
//! tile in front of one ask the server to go in (`ClientMessage::UseExit`,
//! which in the Hall means that), which checks it. Nothing on screen marks the way; anywhere else the edge
//! stops the player as before.

use bevy::prelude::*;
use shared::hall::{self, Side};
use shared::protocol::{ClientMessage, LevelKind};
use shared::spot::{CELL_SPAN, HALF_SPAN};

use crate::common::{CurrentDungeon, GameView, SpotPosition};
use crate::connection::ToServer;
use crate::player::{Player, Walking};

/// How long to wait before asking again, in seconds, while still pushing
/// (the server may have refused, or not yet answered).
const ASK_AGAIN: f64 = 1.0;

/// Asks to go into the inn while the player pushes off the walkway's edge in
/// front of a way in: on the outer half of that tile, the keys more across the
/// walkway (outward) than along it.
pub(super) fn walk_into_inn(
    mut walking: MessageReader<Walking>,
    view: Res<State<GameView>>,
    dungeon: Res<CurrentDungeon>,
    player: Single<&SpotPosition, With<Player>>,
    time: Res<Time>,
    mut asked_at: Local<Option<f64>>,
    mut to_server: MessageWriter<ToServer>,
) {
    let Some(&Walking { x: across, y: out }) = walking.read().last() else {
        return;
    };
    let in_hall = dungeon.info.as_ref().map(|info| info.kind);
    let Some(LevelKind::Hall { origin }) = in_hall else {
        return;
    };
    if *view.get() != GameView::Dungeon {
        return;
    }
    let at = player.0;
    let (x, y) = at.cell();
    let Some((_, side)) = hall::inn_way_behind(x + origin, y) else {
        return;
    };
    let (outward, to_edge) = match side {
        Side::North => (-out, at.y - (y * CELL_SPAN - HALF_SPAN)),
        Side::South => (out, y * CELL_SPAN + HALF_SPAN - at.y),
    };
    if outward <= across.abs() || to_edge > HALF_SPAN {
        return;
    }
    let now = time.elapsed_secs_f64();
    if asked_at.is_some_and(|then| now - then < ASK_AGAIN) {
        return;
    }
    *asked_at = Some(now);
    to_server.write(ToServer(ClientMessage::UseExit));
}
