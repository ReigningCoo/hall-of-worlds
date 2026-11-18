//! Where each piece of an item lies (`Place`): one alone, long ones side by
//! side, round ones heaped; and how many pieces a handful shows.

use bevy::prelude::*;
use shared::vegetable::{Unit, VegetableDef};

use super::group::{Drawing, Pod, Root, Shoot};
use super::variety::Variety;

/// Where one piece lies: its middle, its turn about the up axis (its length
/// along its own +x) and its size.
#[derive(Clone, Copy)]
pub(super) struct Place {
    pub(super) at: Vec3,
    pub(super) yaw: f32,
    pub(super) size: f32,
}

impl Place {
    pub(super) const ONE: Self = Self {
        at: Vec3::ZERO,
        yaw: 0.0,
        size: 1.0,
    };

    /// A point of the piece's own (its length along x) where it lies.
    pub(super) fn p(&self, local: Vec3) -> Vec3 {
        self.at + Quat::from_rotation_y(self.yaw) * (local * self.size)
    }

    /// A way of the piece's own, turned as it lies.
    pub(super) fn turn(&self, yaw: f32) -> f32 {
        self.yaw + yaw
    }
}

/// How many pieces one item shows: a handful by its kind, else one.
pub(super) fn count(def: &VegetableDef, drawing: Drawing) -> usize {
    if def.unit != Unit::Handful {
        return 1;
    }
    match drawing {
        Drawing::Seed(_) => 18,
        Drawing::Pod(Pod::Shell) => 7,
        Drawing::Pod(Pod::Ridged) | Drawing::Root(Root::Globe) => 5,
        Drawing::Pod(_) => 3,
        Drawing::Shoot(Shoot::Spear) => 6,
        Drawing::Shoot(Shoot::Vine) => 3,
        Drawing::Leaves(_) => 5,
        _ => 5,
    }
}

/// Long pieces `across` apart side by side along z, each a little turned and
/// shifted.
pub(super) fn side_by_side(count: usize, across: f32, variety: &Variety) -> Vec<Place> {
    (0..count)
        .map(|n| {
            let z = (n as f32 - (count as f32 - 1.0) / 2.0) * across;
            let x = variety.wobble(n, 30) * across * 0.8;
            Place {
                at: Vec3::new(x, (n % 2) as f32 * across * 0.25, z),
                yaw: variety.wobble(n, 31) * 0.12,
                size: 1.0 - variety.share(n, 32) * 0.12,
            }
        })
        .collect()
}

/// Round pieces of about `radius` heaped: a pair, three, or more spiralling out
/// (a sunflower's pattern), those in the middle lying on the others.
pub(super) fn heaped(count: usize, radius: f32, variety: &Variety) -> Vec<Place> {
    (0..count)
        .map(|n| {
            let spot = match count {
                1 => Vec2::ZERO,
                2 => Vec2::new(n as f32 * 2.0 - 1.0, 0.0) * 0.95,
                _ => {
                    let a = n as f32 * 2.399_963;
                    Vec2::new(a.cos(), a.sin()) * (n as f32 + 0.5).sqrt() * 1.15
                }
            } * radius;
            let lift = if count > 6 && n * 3 < count {
                radius * 0.9
            } else {
                0.0
            };
            Place {
                at: Vec3::new(spot.x, lift, spot.y),
                yaw: variety.wobble(n, 33) * std::f32::consts::PI,
                size: 1.0 - variety.share(n, 34) * 0.15,
            }
        })
        .collect()
}
