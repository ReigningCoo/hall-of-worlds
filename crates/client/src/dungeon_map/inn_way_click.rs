//! A click or tap in the Hall on one of its hidden ways into The Way Inn
//! (`shared::hall::inn_way`): the void tile itself, though nothing is drawn
//! there. The player travels to the walkway tile in front of it, and `site`
//! steps through on arriving (`InnWayClicked`). Clicks on the void anywhere
//! else still do nothing.

use bevy::prelude::*;
use shared::hall;
use shared::protocol::LevelKind;

use super::ground_hit::Ground;
use crate::common::CurrentDungeon;

/// A way into the inn was clicked: the walkway tile in front of it (this
/// stretch's cells), for `site` to step through from on arriving there.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct InnWayClicked {
    pub x: i64,
    pub y: i64,
}

/// The walkway tile in front of the way in the ray falls on, if it falls on
/// one. The Hall is flat, so the ray is followed down to height 0.
pub(super) fn inn_way_front(ray: Ray3d, dungeon: &CurrentDungeon) -> Option<(i64, i64)> {
    let Some(LevelKind::Hall { origin }) = dungeon.info.as_ref().map(|info| info.kind) else {
        return None;
    };
    let (x, y) = super::picking::floor_cell(ray, Ground(None))?;
    let side = hall::inn_way(x + origin, y)?;
    Some(hall::inn_way_front(x, side))
}

#[cfg(test)]
mod tests;
