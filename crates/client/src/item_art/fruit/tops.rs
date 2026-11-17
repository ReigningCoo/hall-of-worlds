//! What is at a fruit's ends: a stalk and perhaps a leaf, a calyx, a star of
//! dry sepals, a pomegranate's crown, or a pineapple's leaves.

use bevy::prelude::*;

use super::super::herb::kit::Kit;
use super::body::Placed;
use super::colour::{Colours, colour};
use super::group::Top;
use super::variety::Variety;

/// Puts `top` on the fruit at `placed`, its `piece`th in the item.
pub(super) fn top(
    top: Top,
    placed: &Placed,
    colours: &Colours,
    variety: &Variety,
    piece: usize,
    kit: &mut Kit,
) {
    let size = placed.least();
    let leaf = kit.material(colour(colours.leaf), false);
    match top {
        Top::Bare => {}
        Top::Stalk => {
            if !variety.stalk && piece == 0 {
                return;
            }
            let stalk = kit.material(colour(colours.stalk), false);
            let (at, out) = placed.stalk_end();
            let bend = Vec3::new(variety.wobble(piece, 13), 0.0, variety.wobble(piece, 14)) * 0.4;
            // Towards the top of the picture, as the owner asked.
            let lean = Vec3::new(0.0, 0.0, -0.6);
            let tip = at + (out + bend + lean).normalize() * size * 0.55;
            kit.rod(&stalk, at - out * size * 0.05, tip, size * 0.05);
            if variety.leaf && piece == 0 {
                let a = variety.wobble(piece, 15) * std::f32::consts::PI;
                let across = Vec3::new(a.cos(), 0.0, a.sin());
                let middle = tip + across * size * 0.5 + Vec3::Y * size * 0.05;
                let scale = Vec3::new(size * 0.55, size * 0.04, size * 0.22);
                kit.blob(&leaf, middle, scale, (-a, 0.0));
            }
        }
        Top::Calyx => {
            let (at, out) = placed.stalk_end();
            for k in 0..4 {
                let a = k as f32 * std::f32::consts::FRAC_PI_2 + 0.3;
                let across = Vec3::new(a.cos(), 0.0, a.sin());
                let middle = at + across * size * 0.22 + out * size * 0.02;
                let scale = Vec3::new(size * 0.26, size * 0.05, size * 0.14);
                kit.blob(&leaf, middle, scale, (-a, 0.0));
            }
            let stalk = kit.material(colour(colours.stalk), false);
            kit.rod(&stalk, at, at + out * size * 0.2, size * 0.06);
        }
        Top::Sepals => {
            let dry = kit.material(colour(colours.skin * 0.45), false);
            let (at, out) = placed.blossom_end();
            for k in 0..5 {
                let a = k as f32 * std::f32::consts::TAU / 5.0;
                let across = Vec3::new(a.cos(), 0.0, a.sin());
                let tip = at + (out * 0.6 + across) * size * 0.2;
                kit.cone(&dry, at, tip, size * 0.06);
            }
        }
        Top::Crown => {
            let skin = kit.material(colour(colours.skin * 0.8), false);
            let (at, out) = placed.stalk_end();
            kit.rod(
                &skin,
                at - out * size * 0.05,
                at + out * size * 0.15,
                size * 0.18,
            );
            for k in 0..6 {
                let a = k as f32 * std::f32::consts::TAU / 6.0;
                let across = Vec3::new(a.cos(), 0.0, a.sin());
                let base = at + out * size * 0.12 + across * size * 0.12;
                kit.cone(
                    &skin,
                    base,
                    base + (out + across * 0.6) * size * 0.2,
                    size * 0.06,
                );
            }
        }
        Top::Leaves => {
            let (at, out) = placed.stalk_end();
            for k in 0..9 {
                let spread = (k as f32 - 4.0) / 4.0;
                let a = placed.yaw - spread * 0.9;
                let along = Vec3::new(a.cos(), 0.12 + spread.abs() * 0.1, -a.sin());
                let length = size * (0.9 - spread.abs() * 0.3);
                let middle = at + along * length + out * size * 0.05;
                let scale = Vec3::new(length, size * 0.04, size * 0.09);
                kit.blob(&leaf, middle, scale, (a, 0.0));
            }
        }
    }
}
