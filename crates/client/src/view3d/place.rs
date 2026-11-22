//! Every frame: the camera on the player and the tokens on their owners. In
//! the Hall of Worlds the view rolls by the twist at the player's column
//! (`common::HallBend::tilt`), so the floor under them is level on screen
//! and they stand upright, while the Hall rolls away ahead and behind.

use bevy::prelude::*;

use super::token::{self, BaseTilt, IconAnchor, ModelAnchor, TokenAssets, TokenBase, TokenView};
use super::{OrbitCamera, ViewCamera, ViewGrid, clearance};
use crate::common::{
    DrawnGround, HallBend, MapPosition, Preferences, SmoothPosition, ViewFacing, ViewTurn,
};
use crate::player::Player;

/// How the whole view is rolled about the walkway's length: the Hall's twist
/// at the player's column, none elsewhere. The camera is turned by it round
/// the player, and tokens stand upright in it.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct ViewRoll(pub(super) Quat);

/// Every frame: the camera follows the player's exact position, so travelling
/// looks smooth. Tokens stand on the floor at their owners' exact positions,
/// with icon anchors facing the camera. Also tells other modules which way the
/// view is turned (`ViewFacing`).
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn place_camera_and_tokens(
    player: Single<(Option<&MapPosition>, &SmoothPosition), With<Player>>,
    preferences: Res<Preferences>,
    grid: Res<ViewGrid>,
    (heights, bend): (Res<DrawnGround>, Res<HallBend>),
    positions: Query<(Option<&MapPosition>, &SmoothPosition)>,
    (mut facing, mut turn, mut roll): (ResMut<ViewFacing>, ResMut<ViewTurn>, ResMut<ViewRoll>),
    time: Res<Time>,
    camera: Single<
        (&mut OrbitCamera, &mut Transform),
        (
            With<ViewCamera>,
            Without<TokenView>,
            Without<IconAnchor>,
            Without<ModelAnchor>,
        ),
    >,
    mut views: Query<
        (&mut TokenView, &mut Transform),
        (
            Without<IconAnchor>,
            Without<ModelAnchor>,
            Without<TokenBase>,
        ),
    >,
    mut anchors: Query<
        (&IconAnchor, &mut Transform),
        (
            Without<ViewCamera>,
            Without<TokenView>,
            Without<ModelAnchor>,
            Without<TokenBase>,
        ),
    >,
    mut models: Query<
        (&ModelAnchor, &mut Transform),
        (Without<ViewCamera>, Without<TokenView>, Without<TokenBase>),
    >,
    mut bases: Query<
        &mut Transform,
        (
            With<TokenBase>,
            Without<ViewCamera>,
            Without<TokenView>,
            Without<IconAnchor>,
            Without<ModelAnchor>,
        ),
    >,
) {
    let (mut orbit, mut camera) = camera.into_inner();
    let size = grid.cell_size;
    // Reduce Motion: on the cell, without gliding between.
    let snap = preferences.graphics.reduce_motion;
    let shown_at = |(cell, smooth): (Option<&MapPosition>, &SmoothPosition)| match cell {
        Some(cell) if snap => (cell.x as f32, cell.y as f32),
        _ => (smooth.x as f32, smooth.y as f32),
    };

    let (x, y) = shown_at(*player);
    // On raised ground the camera keeps the player's cell where it is on
    // screen, and the land moves under them.
    let target = grid.at(&heights, &bend, x, y);
    // Tilted up as far as a clear line to them over that land needs.
    let wanted = match grid.raised && heights.slopes() {
        true => {
            let ground = |cx: i64, cy: i64| {
                let floor = grid.floor + heights.drawn(cx, cy) * size;
                heights.covers(cx, cy).then_some(floor)
            };
            clearance::clearing_raise(&orbit, target, size, ground)
        }
        _ => 0.0,
    };
    let raise = clearance::eased(orbit.raise(), wanted, time.delta_secs(), snap);
    orbit.set_raise(raise);
    *camera = orbit.transform(target);
    // In the Hall, rolled round the player with the floor under them.
    let rolled = bend.tilt(x);
    camera.rotate_around(target, rolled);
    roll.set_if_neq(ViewRoll(rolled));
    // Only write when it changes, so change detection stays meaningful.
    facing.set_if_neq(ViewFacing(orbit.quarter_turns()));
    // The camera turning east (positive yaw) puts west at the top.
    turn.set_if_neq(ViewTurn(-orbit.yaw));

    // Token views turn only with the view's roll, so turning each icon anchor
    // about their vertical towards the camera makes it face the screen while
    // standing upright. Scaling the anchor scales its icon by the owner's
    // `TokenSize`.
    let rotation = TokenAssets::icon_rotation(rolled.inverse() * camera.rotation);
    let half = token::HALF_THICKNESS;
    for (mut view, mut transform) in &mut views {
        let Ok(position) = positions.get(view.owner) else {
            continue;
        };
        let (x, y) = shown_at(position);
        // Each base lies on the ground under it, tilted where it slopes, or
        // with the Hall's twist there; worked out in the rolled view.
        let middle = grid.at(&heights, &bend, x, y);
        let tilt = match grid.raised && heights.slopes() {
            true => BaseTilt::fit(view.radius, half, |dx, dz| {
                grid.floor_at(&heights, &bend, x + dx / size, y + dz / size)
            }),
            false => BaseTilt::on_plane(middle.y, rolled.inverse() * bend.tilt(x) * Vec3::Y),
        };
        // The root at the base's middle; what stands on it stands on the
        // middle of its top, `up` from there.
        let up = tilt.up(half);
        transform.translation = middle.with_y(tilt.bottom) + rolled * up;
        transform.rotation = rolled;
        view.tilt = rolled * tilt.rotation();
        if let Ok(mut base) = bases.get_mut(view.base) {
            base.rotation = tilt.rotation();
        }
        // The icon or model stays upright, its feet on the top's middle.
        if let Ok((anchor, mut transform)) = anchors.get_mut(view.anchor) {
            let scale = anchor.scale();
            transform.rotation = rotation;
            transform.scale = Vec3::splat(scale);
            transform.translation = TokenAssets::icon_offset(scale) + up - Vec3::Y * half;
        }
        // Turned by `turn_models`.
        if let Ok((_, mut transform)) = models.get_mut(view.model) {
            transform.translation = up;
            transform.scale = Vec3::splat(token::MODEL_SCALE);
        }
    }
}
