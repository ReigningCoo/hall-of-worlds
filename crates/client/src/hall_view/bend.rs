//! The Hall of Worlds drawn bent (`common::HallBend`): on while it is the
//! view shown, off anywhere else.

use bevy::prelude::*;
use shared::protocol::LevelKind;

use crate::common::{CurrentDungeon, GameView, HallBend};

/// Every frame, before anything is placed: the bend for the Hall's stretch
/// shown, or none. Only written when it changes, so views can redraw then.
pub(super) fn set_bend(
    dungeon: Res<CurrentDungeon>,
    view: Res<State<GameView>>,
    mut bend: ResMut<HallBend>,
) {
    let origin = match dungeon.info.as_ref().map(|info| info.kind) {
        Some(LevelKind::Hall { origin }) if *view.get() == GameView::Dungeon => Some(origin),
        _ => None,
    };
    bend.set_if_neq(origin.map_or_else(HallBend::default, HallBend::on));
}
