//! A drinking horn: a cow horn curving from a wide, metal-rimmed mouth at the
//! top down to its tip, pale to dark, two cells tall.

use bevy::prelude::*;

use super::{HOLLOW, Kit, PEWTER};

/// How many pieces it is built of along its curve.
const PIECES: usize = 8;
/// The curve, a quadratic Bézier from the mouth to the tip.
const MOUTH: Vec3 = Vec3::new(0.15, 1.6, 0.0);
const BEND: Vec3 = Vec3::new(0.8, -0.7, 0.0);
const TIP: Vec3 = Vec3::new(-0.7, -2.0, 0.0);

/// Where along the curve `t` is, from the mouth (0) to the tip (1).
fn along(t: f32) -> Vec3 {
    let u = 1.0 - t;
    MOUTH * u * u + BEND * 2.0 * u * t + TIP * t * t
}

/// How wide it is there.
fn radius(t: f32) -> f32 {
    0.42 * (1.0 - t) + 0.05 * t
}

/// Its colour there: pale horn darkening to the tip.
fn colour(t: f32) -> [f32; 3] {
    let pale = [0.86, 0.78, 0.6];
    let dark = [0.25, 0.18, 0.12];
    [0, 1, 2].map(|k| pale[k] + (dark[k] - pale[k]) * t * t)
}

pub(super) fn horn(kit: &mut Kit) {
    for piece in 0..PIECES {
        let (wide, narrow) = (
            piece as f32 / PIECES as f32,
            (piece + 1) as f32 / PIECES as f32,
        );
        let (top, bottom) = (along(wide), along(narrow));
        let shade = kit.matte(colour((wide + narrow) / 2.0), 0.5);
        let span = top - bottom;
        let frustum = ConicalFrustum {
            radius_top: radius(wide),
            radius_bottom: radius(narrow),
            height: span.length(),
        };
        let turned = Quat::from_rotation_arc(Vec3::Y, span.normalize());
        let at = Transform::from_translation((top + bottom) / 2.0).with_rotation(turned);
        kit.add(frustum, &shade, at);
        kit.add(
            Sphere::new(radius(narrow)),
            &shade,
            Transform::from_translation(bottom),
        );
    }
    // The rim round the mouth, square to the curve there.
    let pewter = kit.metal(PEWTER);
    let facing = Quat::from_rotation_arc(Vec3::Y, (MOUTH - along(0.05)).normalize());
    let rim = Transform::from_translation(MOUTH).with_rotation(facing);
    kit.add(Torus::new(0.38, 0.48), &pewter, rim);
    let dark = kit.matte(HOLLOW, 1.0);
    kit.add(Cylinder::new(0.38, 0.02), &dark, rim);
}
