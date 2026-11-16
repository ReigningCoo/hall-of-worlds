//! The wedge in which nothing on the floors above the player's is drawn
//! (owner's, 2026-11-16): `OtherFloorLooks::wedge` degrees wide, seen from
//! above, with its point on the player, opening towards the camera, so
//! whatever of the storeys over them would stand between the camera and the
//! player is gone. Cut per pixel in the surface shader, so it turns smoothly
//! with the camera.

use bevy::prelude::*;

use super::super::CELL_SIZE;
use super::super::sight_lines::{SurfaceMaterial, Wedge};
use super::floors::Floors;
use crate::common::{OtherFloorLooks, SmoothPosition};
use crate::player::Player;
use crate::view3d::ViewCamera;

/// Points the floors above's wedge from the player towards the camera, every
/// frame it moves.
pub(in crate::dungeon_map) fn aim_wedge(
    floors: Res<Floors>,
    player: Single<&SmoothPosition, With<Player>>,
    camera: Single<&Transform, With<ViewCamera>>,
    looks: Res<OtherFloorLooks>,
    mut materials: ResMut<Assets<SurfaceMaterial>>,
    mut last: Local<Option<Wedge>>,
) {
    let wedge = Wedge {
        at: Vec2::new(player.x as f32, player.y as f32) * CELL_SIZE,
        towards: towards_camera(*camera),
        degrees: looks.wedge,
    };
    // Materials made since are aimed too.
    if *last == Some(wedge) && !floors.is_changed() {
        return;
    }
    *last = Some(wedge);
    for floor in &floors.above {
        for (_, _, handle) in floor.solid.iter().chain(&floor.memory) {
            if let Some(mut material) = materials.get_mut(handle) {
                material.extension.set_wedge(Some(wedge));
            }
        }
    }
}

/// The way from what the camera looks at towards the camera, level, of unit
/// length. Its up is added to where it looks, so a camera looking straight
/// down still has one (the bottom of the screen).
fn towards_camera(camera: &Transform) -> Vec2 {
    let back = -(camera.forward().as_vec3() + camera.up().as_vec3());
    Vec2::new(back.x, back.z).normalize_or(Vec2::Y)
}

#[cfg(test)]
mod tests;
