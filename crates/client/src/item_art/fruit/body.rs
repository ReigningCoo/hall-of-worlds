//! One fruit's body: a squashed sphere (or two, for a pear) where it lies, with
//! its blush, and `Placed`: where its skin is, for what is put on it.

use bevy::prelude::*;

use super::super::herb::kit::Kit;
use super::colour::{Colours, colour};
use super::group::{Body, Style};
use super::variety::Variety;

/// Where one fruit lies: its middle, half sizes (x its length) and turn.
pub(super) struct Placed {
    pub(super) centre: Vec3,
    pub(super) half: Vec3,
    pub(super) yaw: f32,
    pub(super) body: Body,
}

impl Placed {
    /// A fruit of `style` reaching `reach` from `centre` along its length.
    pub(super) fn new(style: &Style, centre: Vec3, reach: f32, yaw: f32) -> Self {
        let across = reach / style.length;
        let half = Vec3::new(reach, across * 0.92, across);
        Self {
            centre: centre + Vec3::Y * half.y,
            half,
            yaw,
            body: style.body,
        }
    }

    pub(super) fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw)
    }

    /// The point on its skin in direction `d` of its own (a unit vector).
    pub(super) fn on(&self, d: Vec3) -> Vec3 {
        self.centre + self.rotation() * (self.half * d)
    }

    /// The way out of its skin at direction `d` of its own.
    pub(super) fn normal(&self, d: Vec3) -> Vec3 {
        self.rotation() * (d / self.half).normalize()
    }

    /// Where the stalk is: on top of a round fruit, at +x of a long one.
    pub(super) fn stalk_end(&self) -> (Vec3, Vec3) {
        let d = match self.body {
            Body::Round => Vec3::new(0.0, 1.0, -0.45).normalize(),
            _ => Vec3::X,
        };
        (self.on(d), self.normal(d))
    }

    /// The other end, where the flower was.
    pub(super) fn blossom_end(&self) -> (Vec3, Vec3) {
        let d = match self.body {
            Body::Round => Vec3::new(-0.2, 0.98, 0.1).normalize(),
            _ => Vec3::NEG_X,
        };
        (self.on(d), self.normal(d))
    }

    /// The smallest half size.
    pub(super) fn least(&self) -> f32 {
        self.half.min_element()
    }
}

/// Builds the fruit's body, and its blush.
pub(super) fn body(placed: &Placed, colours: &Colours, variety: &Variety, kit: &mut Kit) {
    let skin = kit.material(colour(colours.skin), !colours.matte);
    let (centre, half, yaw) = (placed.centre, placed.half, placed.yaw);
    let rotation = placed.rotation();
    match placed.body {
        Body::Pear => {
            // A wide bottom at -x, a narrower neck rising towards +x.
            let bottom = centre + rotation * Vec3::new(-half.x * 0.32, 0.0, 0.0);
            kit.blob(&skin, bottom, half * Vec3::new(0.68, 1.0, 1.0), (yaw, 0.0));
            let neck = centre + rotation * Vec3::new(half.x * 0.45, -half.y * 0.15, 0.0);
            kit.blob(&skin, neck, half * Vec3::new(0.55, 0.7, 0.66), (yaw, 0.0));
        }
        Body::Lemon => {
            kit.blob(&skin, centre, half * Vec3::new(0.86, 1.0, 1.0), (yaw, 0.0));
            for end in [-1.0, 1.0] {
                let at = centre + rotation * Vec3::new(end * half.x * 0.85, 0.0, 0.0);
                kit.blob(&skin, at, Vec3::splat(half.z * 0.3), (yaw, 0.0));
            }
        }
        Body::Round | Body::Oval => fine_blob(kit, &skin, centre, half, yaw),
    }
    if let Some(blush) = colours.blush {
        // A wide wash half way to the blush, and a smaller patch of it inside:
        // each a sphere poking out of the skin on the blushed side.
        let wash = kit.material(colour(colours.skin.lerp(blush, 0.7)), !colours.matte);
        let blush = kit.material(colour(blush), !colours.matte);
        let a = variety.blush_angle;
        let towards = Vec3::new(a.cos(), 0.6, a.sin()).normalize();
        let (middle, shape) = match placed.body {
            Body::Pear => (
                centre + rotation * Vec3::new(-half.x * 0.32, 0.0, 0.0),
                half * Vec3::new(0.68, 1.0, 1.0),
            ),
            _ => (centre, half),
        };
        let least = shape.min_element();
        let layers = [
            (&wash, 0.9, 0.12 + variety.blush * 0.05),
            (&blush, 0.5, 0.55 + variety.blush * 0.05),
        ];
        for (material, scale, shift) in layers {
            let at = middle + rotation * (towards * least * shift);
            fine_blob(kit, material, at, shape * scale, yaw);
        }
    }
}

/// A smoother squashed sphere than `Kit::blob`'s, so where a blush meets the
/// skin the edge is a curve, not steps.
fn fine_blob(kit: &mut Kit, material: &Handle<StandardMaterial>, at: Vec3, half: Vec3, yaw: f32) {
    let transform = Transform::from_translation(at)
        .with_rotation(Quat::from_rotation_y(yaw))
        .with_scale(half);
    kit.mesh(Sphere::new(1.0).mesh().uv(40, 24), material, transform);
}
