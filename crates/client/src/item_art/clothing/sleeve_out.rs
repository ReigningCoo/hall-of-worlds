//! How far a piece's sleeves stand out from the upper arm, measured off its model, so
//! capes, veils and shawls draped over the arms keep outside them.

use bevy::prelude::*;
use shared::body::{Frame, Joint, Side};

use crate::item_art::armour::Model;

/// Places down the upper arm a sleeve is measured at, from the shoulder to the elbow.
const STEPS: usize = 5;
/// The most a sleeve is taken to stand off the skin: further out are panels hanging
/// from it, not the sleeve round the arm.
const MOST_EASE: f32 = 50.0;

/// Millimetres in each step a sleeve's distance out is kept in.
const UNIT: f32 = 2.0;

/// How far sleeves stand out from the middle of the upper arm, at most, at evenly
/// spaced places from the shoulder (first) to the elbow (last), in steps of `UNIT`
/// rounded up: kept small, as a copy goes with every worn piece in its `Wearer`. All
/// nothing when no sleeves are worn.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(in crate::item_art) struct SleeveOut([u8; STEPS]);

impl SleeveOut {
    /// How far out at `along` (0 at the shoulder, 1 at the elbow), between the places
    /// measured.
    pub(in crate::item_art) fn at(&self, along: f32) -> f32 {
        let at = along.clamp(0.0, 1.0) * (STEPS - 1) as f32;
        let (low, share) = (at.floor() as usize, at.fract());
        let high = (low + 1).min(STEPS - 1);
        let (low, high) = (self.0[low] as f32, self.0[high] as f32);
        (low + (high - low) * share) * UNIT
    }

    /// The further out of two at each place.
    pub(super) fn max(self, other: Self) -> Self {
        Self(std::array::from_fn(|step| self.0[step].max(other.0[step])))
    }
}

/// The upper arm on one side: from the shoulder joint to the elbow.
pub(in crate::item_art) fn upper_arm(frame: &Frame, side: Side) -> (Vec3, Vec3) {
    let joint = |joint: Joint| {
        let [x, y, z] = frame.joint(joint);
        Vec3::new(x as f32, y as f32, z as f32)
    };
    (joint(Joint::Shoulder(side)), joint(Joint::Elbow(side)))
}

/// How far `point` is from the upper arm `(from, to)`, and how far along it
/// (0 at the shoulder, 1 at the elbow, not clamped).
pub(in crate::item_art) fn from_arm(point: Vec3, (from, to): (Vec3, Vec3)) -> (f32, f32) {
    let along = (point - from).dot(to - from) / (to - from).length_squared();
    let nearest = from.lerp(to, along.clamp(0.0, 1.0));
    ((point - nearest).length(), along)
}

/// How far the model stands from the middle of either upper arm on its outside (away
/// from the body), down from the shoulder to the elbow: its sleeves, if any.
pub(super) fn measure(model: &Model, frame: &Frame) -> SleeveOut {
    let reach = frame.upper_arm as f32 + MOST_EASE;
    let mut most = [0.0f32; STEPS];
    for side in Side::BOTH {
        let arm = upper_arm(frame, side);
        let sign = side.sign() as f32;
        let corners = model
            .parts
            .iter()
            .flat_map(|(_, triangles)| triangles.iter().flatten());
        for &corner in corners {
            let (away, along) = from_arm(corner, arm);
            let outside = (corner.x - arm.0.x) * sign > 0.0;
            if outside && (0.0..=1.0).contains(&along) && away < reach {
                let step = (along * (STEPS - 1) as f32).round() as usize;
                most[step] = most[step].max(away);
            }
        }
    }
    SleeveOut(most.map(|away| (away / UNIT).ceil().min(u8::MAX as f32) as u8))
}
