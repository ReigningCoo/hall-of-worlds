//! The player's own map marks (`common::MapMarks`) on the world map: each a
//! token with no base, near the player, carrying a flat disc that looks like
//! the mark on the maps, lying on the ground as a poi's base does (`mark_disc`). Hovering names it, as the full map does;
//! clicking or tapping it travels there.
//!
//! Like pois, marks only exist as entities while the world map is the current
//! view, and only within `pois::RANGE` of the player. They follow the
//! resource as the server adds and removes them.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::id::Id;
use shared::map_mark::{MapMark, MarkColour, MarkIcon};
use shared::wilderness::SCALE;

use super::pois::RANGE;
use crate::common::{
    Clickable, CurrentWorld, HoverName, MapMarks, MapPosition, SmoothPosition, Token, Wording,
    in_cells, mark_name, near_way_round, point_at,
};
use crate::player::{Player, TravelTo};
use crate::view3d::ClickedToken;

/// On a mark's token: what its disc shows.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(super) struct MarkToken {
    pub(super) look: (MarkIcon, MarkColour),
}

/// The mark tokens there are, by the mark's id.
#[derive(Resource, Default)]
pub(super) struct SpawnedMarks {
    entities: HashMap<Id, Entity>,
    /// Whether `entities` matches the marks and the player's place: false when
    /// the view has just become current.
    ready: bool,
}

/// The marks' tokens' places, to move them.
type Placed<'w, 's> = Query<
    'w,
    's,
    (&'static mut MapPosition, &'static mut SmoothPosition),
    (With<MarkToken>, Without<Player>),
>;

/// When the marks change, the player reaches a new point or the view has just
/// become current: spawns the marks now in range, moves those kept (the near
/// way round the planet from the player) and despawns the rest.
pub(super) fn sync_marks(
    mut commands: Commands,
    (marks, world, wording): (Res<MapMarks>, Res<CurrentWorld>, Res<Wording>),
    player: Single<Ref<MapPosition>, With<Player>>,
    mut spawned: ResMut<SpawnedMarks>,
    mut placed: Placed,
) {
    let changed = marks.is_changed() || world.is_changed() || wording.is_changed();
    if spawned.ready && !changed && !player.is_changed() {
        return;
    }
    spawned.ready = true;
    let width = world.surface.width();
    let near = |mark: &MapMark| {
        let (x, y) = shown_at(mark);
        let across = near_way_round(x - player.x as f64, width);
        let at = (player.x as f64 + across, y);
        let range = RANGE as f64;
        (across.abs() <= range && (y - player.y as f64).abs() <= range).then_some(at)
    };
    let wanted: HashMap<Id, (&MapMark, (f64, f64))> = (marks.0.iter())
        .filter_map(|mark| Some((mark.id, (mark, near(mark)?))))
        .collect();
    spawned.entities.retain(|id, entity| {
        // A mark whose look or name changed is spawned again.
        let keep = wanted.contains_key(id) && !changed;
        if !keep {
            commands.entity(*entity).despawn();
        }
        keep
    });
    for (id, (mark, (x, y))) in wanted {
        let (cell_x, cell_y) = point_at((x, y));
        let point = MapPosition {
            x: cell_x,
            y: cell_y,
        };
        let smooth = SmoothPosition { x, y };
        if let Some(entity) = spawned.entities.get(&id) {
            if let Ok((mut at, mut exact)) = placed.get_mut(*entity) {
                at.set_if_neq(point);
                exact.set_if_neq(smooth);
            }
            continue;
        }
        let entity = commands
            .spawn((
                MarkToken {
                    look: (mark.icon, mark.colour),
                },
                Clickable,
                HoverName(mark_name(&wording.0, mark)),
                point,
                smooth,
                Token {
                    no_base: true,
                    base: shared::creature::Size::Small,
                    ..default()
                },
            ))
            .id();
        spawned.entities.insert(id, entity);
    }
}

/// Where a mark is drawn on the world map, in points: exactly on its pile, if
/// it has one, or in the middle of its cell (`common::in_cells`).
fn shown_at(mark: &MapMark) -> (f64, f64) {
    let (x, y) = in_cells(mark);
    let scale = SCALE as f64;
    let along = |cell: f64| (cell + 0.5) / scale - 0.5;
    (along(x), along(y))
}

/// Leaving the view or the world: despawns every mark's token.
pub(super) fn despawn_marks(mut commands: Commands, mut spawned: ResMut<SpawnedMarks>) {
    for (_, entity) in spawned.entities.drain() {
        commands.entity(entity).despawn();
    }
    spawned.ready = false;
}

/// A click or tap on a mark's token travels to its point.
pub(super) fn travel_to_mark(
    clicked: Res<ClickedToken>,
    marks: Query<&MapPosition, With<MarkToken>>,
    mut travel: MessageWriter<TravelTo>,
) {
    if let Some(at) = clicked.0.and_then(|entity| marks.get(entity).ok()) {
        travel.write(TravelTo { x: at.x, y: at.y });
    }
}
