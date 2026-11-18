//! `Dress`: flowers and fruit put on a plant once its shape is built, with
//! random numbers of their own, so a plant keeps its shape from stage to
//! stage whatever it shows. Fruit is drawn by its shape (`FruitShape`) in its
//! ripe or unripe colour, the same colours as its item's picture.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::fruit::{FruitPosition, FruitShape};
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::look::{Look, Show};

/// A fruit's stalk, darker than the leaves.
const STALK: Vec3 = Vec3::new(0.2, 0.17, 0.08);

/// Flowers and fruit for one plant.
pub(super) struct Dress<'a> {
    pub(super) look: &'a Look,
    pub(super) rng: Rng,
    /// Whether fruit sits on what bears it rather than hanging (cacti, a
    /// pineapple): set by the family's builder.
    pub(super) sits: bool,
}

impl Dress<'_> {
    /// A random number from `low` to `high`.
    pub(super) fn between(&mut self, low: f32, high: f32) -> f32 {
        between(&mut self.rng, low, high)
    }

    /// Whether anything shows besides leaves.
    pub(super) fn bare(&self) -> bool {
        self.look.show == Show::Leaves
    }

    /// One fruit hanging from `at`, or sitting on it if it sits or lies on
    /// the ground.
    pub(super) fn fruit(&mut self, pieces: &mut Pieces, at: Vec3, colour: Vec3) {
        let s = self.look.fruit_size * self.between(0.85, 1.15);
        let turn = self.between(0.0, TAU);
        let ground = self.look.fruits == FruitPosition::Ground;
        let lying = ground || self.sits;
        let (size, drop) = match self.look.fruit_shape {
            FruitShape::Round | FruitShape::Aggregate | FruitShape::Nut => (Vec3::splat(s), s),
            FruitShape::Oval => (Vec3::new(s * 0.8, s * 1.1, s * 0.8), s * 1.1),
            FruitShape::Long if ground => (Vec3::new(s * 1.2, s * 0.7, s * 0.7), s * 0.7),
            FruitShape::Long => (Vec3::new(s * 0.32, s, s * 0.32), s),
            FruitShape::Pod => (Vec3::new(s * 0.32, s, s * 0.12), s),
            FruitShape::Pear if lying => (Vec3::new(s * 0.85, s, s * 0.85), s),
            FruitShape::Pear => {
                pieces.ball(at - Vec3::Y * s * 0.5, Vec3::splat(s * 0.55), turn, colour);
                (Vec3::splat(s * 0.85), s * 1.4)
            }
        };
        let middle = if lying {
            at + Vec3::Y * size.y
        } else {
            at - Vec3::Y * drop
        };
        pieces.ball(middle, size, turn, colour);
    }

    /// Fruit at `at` as its plant bears it: a bunch of a few, or one.
    pub(super) fn fruit_at(&mut self, pieces: &mut Pieces, at: Vec3, colour: Vec3) {
        if self.look.fruits != FruitPosition::Clusters || self.sits {
            self.fruit(pieces, at, colour);
            return;
        }
        let count = self.rng.range_u64(3, 6) as usize;
        let s = self.look.fruit_size;
        let low = at - Vec3::Y * s * 1.2;
        pieces.stem(at, low, (s * 0.15, s * 0.1), STALK);
        for i in 0..count {
            let angle = i as f32 / count as f32 * TAU + self.between(-0.4, 0.4);
            let side = Vec3::new(angle.cos(), 0.0, angle.sin()) * s * 0.9;
            let down = Vec3::Y * s * (i % 2) as f32 * 0.9;
            self.fruit(pieces, low + side - down, colour);
        }
    }

    /// A flower at `at`, facing up.
    pub(super) fn flower(&mut self, pieces: &mut Pieces, at: Vec3) {
        let s = self.look.flower_size * self.between(0.85, 1.2);
        let turn = self.between(0.0, TAU);
        pieces.blob_turned(at, Vec3::new(s, s * 0.5, s), turn, self.look.flower);
    }

    /// What shows at `at` on the plant's outside: a flower or fruit, by stage.
    pub(super) fn at(&mut self, pieces: &mut Pieces, at: Vec3) {
        match self.look.show {
            Show::Leaves => {}
            Show::Flowers => self.flower(pieces, at),
            Show::Fruit(colour) => self.fruit_at(pieces, at, colour),
        }
    }

    /// `count` flowers or fruit over a mass of leaves at `centre`, `size` its
    /// half widths: fruit hanging below its middle more than over its top,
    /// flowers over its top; at its tips, round its top.
    pub(super) fn over(&mut self, pieces: &mut Pieces, centre: Vec3, size: Vec3, count: usize) {
        if self.bare() {
            return;
        }
        let flowers = self.look.show == Show::Flowers;
        let (low, high) = match self.look.fruits {
            _ if flowers => (0.0, 0.85),
            FruitPosition::Tips => (0.35, 0.85),
            _ => (-0.35, 0.6),
        };
        for _ in 0..count {
            let angle = self.between(0.0, TAU);
            let rise = self.between(low, high);
            let flat = (1.0 - rise * rise).sqrt();
            let out = Vec3::new(angle.cos() * flat, rise, angle.sin() * flat);
            // Fruit hangs from just inside the leaves' edge; flowers sit on it.
            let depth = if flowers { 0.97 } else { 0.9 };
            self.at(pieces, centre + out * size * depth);
        }
    }
}
