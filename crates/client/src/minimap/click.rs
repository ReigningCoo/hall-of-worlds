//! A click or tap on the open full map of the world map or the wilderness,
//! told to `map_marks`.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::mark_paint::MARK_PIXELS;
use super::{FullMap, ICON_PIXELS, Room, hovered_poi, shows_known, spot_on_full_map};
use crate::common::{CurrentDungeon, CurrentWorld, GameView, SmoothPosition};
use crate::player::Player;
use crate::view3d::OrbitCamera;

/// A click or tap (not a drag) on the full map's picture, on the world map or
/// in the wilderness: `minimap` sends it, and the marks' popup opens from it.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct FullMapClick {
    /// Where: on the world map in points, in the wilderness (`wild`) in its cells.
    pub point: (f64, f64),
    /// How far round it a mark's picture reaches there, in the same.
    pub reach: f64,
    /// On the wilderness's full map, so `point` and `reach` are in its cells.
    pub wild: bool,
    /// Where on the screen, in interface pixels, for the popup.
    pub at: Vec2,
    /// Whether it was on a poi's picture.
    pub on_poi: bool,
    /// Whether the land there is known.
    pub known: bool,
}

/// Turns a click `take_input` noticed on the full map's picture (a press let go
/// without dragging) into a `FullMapClick`, on the world map or in the
/// wilderness only: marks are never in sites or the Hall (owner).
#[allow(clippy::too_many_arguments)]
pub(super) fn report_click(
    window: Single<&Window, With<PrimaryWindow>>,
    ui_scale: Res<UiScale>,
    view: Res<State<GameView>>,
    camera: Single<&OrbitCamera>,
    player: Single<&SmoothPosition, With<Player>>,
    world: Option<Res<CurrentWorld>>,
    dungeon: Res<CurrentDungeon>,
    mut full: ResMut<FullMap>,
    mut clicks: MessageWriter<FullMapClick>,
) {
    let Some(at) = full.clicked.take() else {
        return;
    };
    let view = *view.get();
    let wild = view == GameView::Dungeon && dungeon.is_wilderness();
    if view != GameView::WorldMap && !wild {
        return;
    }
    let window = Room::of(&window, &ui_scale);
    let centre = (player.x, player.y);
    let Some((point, units_per_pixel)) =
        spot_on_full_map(&window, at, view, &camera, &full, centre)
    else {
        return;
    };
    let world = world.as_deref();
    let icon_reach = ICON_PIXELS * full.zoom / 2.0;
    clicks.write(FullMapClick {
        point,
        reach: f64::from(MARK_PIXELS * full.zoom / 2.0) * units_per_pixel,
        wild,
        at,
        on_poi: !wild && hovered_poi(world, point, icon_reach, units_per_pixel).is_some(),
        known: shows_known(view, world, &dungeon, point, (0.0, 0.0)),
    });
}
