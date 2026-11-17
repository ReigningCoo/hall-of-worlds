//! Bottles (potion bottles PR 2): many shapes in glass of various shades, always
//! stopped, all picked on the client from the bottle's look seed. The liquid
//! inside comes from its contents alone (PR 3), drawn between the glass's far
//! and near sides.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::{CAMERA_AT, CAMERA_UP, Part};

// Split into files (`cargo xtask lint` keeps files small).
mod front;
mod glass;
mod heart;
mod liquid;
mod liquid_body;
mod liquid_colour;
mod liquid_features;
mod liquid_parts;
mod look;
mod profiles;
mod spheres;
mod stopper;
mod surface;

pub use liquid::LiquidLook;
pub use look::{BottleLook, Glass, Shape, Stopper};
use profiles::profile;
use surface::Surface;

/// How far the bottle's top is tipped towards the camera, so it is seen a
/// little from above, the stopper's top showing.
const TILT: f32 = 0.6;
/// How far the bottle reaches from the picture's middle, against `VIEW_WIDTH`'s
/// 1.25, leaving room for the black edge.
const FIT: f32 = 0.98;

/// A bottle's parts (`Look::Bottle`): the glass's far side and the stopper,
/// solid, then the liquid if there is one, then the glass's near side, all three
/// see-through.
pub(super) fn bottle_parts(
    look: &BottleLook,
    liquid: Option<&LiquidLook>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let turn = Quat::from_rotation_x(-FRAC_PI_2 + TILT);
    let view = turn.inverse() * CAMERA_AT.normalize();
    let outline = profile(look.shape);
    let mut surfaces = vec![(Surface::lathe(&outline), outline.shoulder)];
    if look.shape == Shape::Heart {
        surfaces.push((heart::heart_body(), f32::INFINITY));
    }
    let (stopper, reach) =
        stopper::stopper_parts(look.stopper, look.glass, &outline, meshes, materials);
    let mut points: Vec<Vec3> = surfaces
        .iter()
        .flat_map(|(surface, _)| surface.rows.iter().flatten().copied())
        .collect();
    points.extend((0..8).map(|i| {
        let angle = i as f32 * FRAC_PI_2 / 2.0;
        Vec3::new(angle.sin(), 0.0, angle.cos()) * reach.radius + Vec3::Y * reach.top
    }));
    let fitted = fit(&points, turn);
    let (far, near) = glass::materials(look.glass, liquid.is_some(), materials);
    let top = liquid.map(|_| liquid_body::levels(look.shape).1);
    let mut solid = Vec::new();
    let mut clear = Vec::new();
    for (surface, neck) in &surfaces {
        let (far_mesh, near_mesh) = glass::glass_meshes(surface, look.glass, view, *neck, top);
        solid.push(Part {
            mesh: meshes.add(far_mesh),
            material: far.clone(),
            transform: fitted,
        });
        clear.push(Part {
            mesh: meshes.add(near_mesh),
            material: near.clone(),
            transform: fitted,
        });
    }
    let stopper = stopper.into_iter().map(|part| Part {
        transform: fitted * part.transform,
        ..part
    });
    solid.extend(stopper);
    if let Some(liquid) = liquid {
        solid.extend(liquid_parts::liquid_parts(
            look.shape, liquid, view, fitted, meshes, materials,
        ));
    }
    solid.extend(clear);
    solid
}

/// Turned to face the camera, scaled to `FIT` in the picture and centred.
pub(super) fn fit(points: &[Vec3], turn: Quat) -> Transform {
    let (right, up) = picture_axes();
    let (mut low, mut high) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
    for point in points {
        let at = turn * *point;
        let on = Vec2::new(at.dot(right), at.dot(up));
        low = low.min(on);
        high = high.max(on);
    }
    let scale = 2.0 * FIT / (high - low).max_element();
    let middle = (low + high) / 2.0 * scale;
    Transform {
        translation: -(right * middle.x + up * middle.y),
        rotation: turn,
        scale: Vec3::splat(scale),
    }
}

/// The picture's right and up, in the world.
pub(super) fn picture_axes() -> (Vec3, Vec3) {
    let toward = CAMERA_AT.normalize();
    let up = (CAMERA_UP - toward * CAMERA_UP.dot(toward)).normalize();
    (up.cross(toward), up)
}

#[cfg(test)]
mod tests;
