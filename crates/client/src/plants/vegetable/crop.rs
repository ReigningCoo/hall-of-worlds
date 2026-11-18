//! `Crop`: flowers and what is picked, put on a plant once its shape is
//! built, with random numbers of their own, so a plant keeps its shape from
//! stage to stage whatever it shows. Fruit and pods are drawn in the colour
//! of the item's picture (`VegetableLook`), ears green before they ripen.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::herb::Rgb;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{METRE, between, colour};
use super::look::{Bears, Look, Show};

/// Dry pods' colour.
const DRY_POD: Rgb = Rgb(196, 172, 118);

/// Flowers and what is picked, for one plant.
pub(super) struct Crop<'a> {
    pub(super) look: &'a Look,
    pub(super) rng: Rng,
    /// Whether to draw them at all (tests build plants without).
    pub(super) dressed: bool,
}

impl Crop<'_> {
    /// A random number from `low` to `high`.
    pub(super) fn between(&mut self, low: f32, high: f32) -> f32 {
        between(&mut self.rng, low, high)
    }

    /// What shows, if anything is drawn besides leaves.
    pub(super) fn show(&self) -> Option<Show> {
        Some(self.look.show).filter(|&show| self.dressed && show != Show::Leaves)
    }

    /// What is picked's colour now: green before it ripens.
    pub(super) fn colour(&self) -> Vec3 {
        match (self.look.show, self.look.bears) {
            (Show::Unripe, _) => (self.look.leaf * 1.3).min(Vec3::ONE),
            (_, Bears::Pods { dry: true }) => colour(DRY_POD),
            _ => self.look.part,
        }
    }

    /// A flower at `at`, facing up.
    pub(super) fn flower(&mut self, pieces: &mut Pieces, at: Vec3) {
        let s = self.look.flower_size * self.between(0.85, 1.2);
        let turn = self.between(0.0, TAU);
        pieces.blob_turned(at, Vec3::new(s, s * 0.5, s), turn, self.look.flower);
    }

    /// What shows at `at` under a plant's leaves: a flower, or fruit or a pod
    /// hanging from it.
    pub(super) fn hang(&mut self, pieces: &mut Pieces, at: Vec3) {
        self.bear(pieces, at, false);
    }

    /// What shows at `at` on the ground: a flower a little up, or fruit or a
    /// pod lying there.
    pub(super) fn lie(&mut self, pieces: &mut Pieces, at: Vec3) {
        self.bear(pieces, at, true);
    }

    fn bear(&mut self, pieces: &mut Pieces, at: Vec3, lying: bool) {
        match self.show() {
            None => {}
            Some(Show::Flowers) => {
                let up = if lying { self.look.flower_size } else { 0.0 };
                self.flower(pieces, at + Vec3::Y * up);
            }
            Some(_) => match self.look.bears {
                Bears::Pods { .. } => self.pod(pieces, at, lying),
                _ => self.fruit(pieces, at, lying),
            },
        }
    }

    /// One fruit hanging from `at`, or lying there.
    fn fruit(&mut self, pieces: &mut Pieces, at: Vec3, lying: bool) {
        let size = self.look.part_size * self.between(0.85, 1.15);
        let (long, wide) = (size.x, size.y);
        let turn = self.between(0.0, TAU);
        let colour = self.colour();
        let (half, middle) = if lying && long > wide * 1.6 {
            (Vec3::new(long, wide, wide), at + Vec3::Y * wide)
        } else if lying {
            (Vec3::new(wide, long, wide), at + Vec3::Y * long)
        } else {
            (Vec3::new(wide, long, wide), at - Vec3::Y * long)
        };
        pieces.ball(middle, half, turn, colour);
    }

    /// A pod hanging from `at`, or lying there.
    fn pod(&mut self, pieces: &mut Pieces, at: Vec3, lying: bool) {
        let size = self.look.part_size * self.between(0.85, 1.15);
        let side = Vec3::new(self.between(-0.4, 0.4), 0.0, self.between(-0.4, 0.4));
        let way = if lying {
            let angle = self.between(0.0, TAU);
            Vec3::new(angle.cos(), 0.0, angle.sin())
        } else {
            (side - Vec3::Y).normalize()
        };
        let from = if lying { at + Vec3::Y * size.y } else { at };
        let colour = self.colour();
        pieces.stem(
            from,
            from + way * size.x * 2.0,
            (size.y, size.y * 0.5),
            colour,
        );
    }

    /// An ear or head from `at` along `way` (any length).
    pub(super) fn head(&mut self, pieces: &mut Pieces, at: Vec3, way: Vec3) {
        if self.show().is_none() {
            return;
        }
        let size = self.look.part_size * self.between(0.9, 1.1);
        let way = way.normalize_or(Vec3::Y);
        let colour = self.colour();
        let (wide, tip) = (size.y, size.y * 0.55);
        pieces.stem(at, at + way * size.x * 2.0, (wide, tip), colour);
    }

    /// A root's or bulb's shoulder at the foot, full grown: the top of it
    /// showing above the ground.
    pub(super) fn shoulder(&mut self, pieces: &mut Pieces) {
        if self.look.bears != Bears::Shoulder || self.show() != Some(Show::Ripe) {
            return;
        }
        // At least 6 cm across, and larger than life, so it shows.
        let wide = self.look.part_size.y.max(self.look.part_size.x * 0.35) * 1.3;
        let wide = wide.max(0.06 * METRE);
        let size = Vec3::new(wide, wide * 0.75, wide);
        let turn = self.between(0.0, 1.6);
        pieces.ball(Vec3::Y * wide * 0.4, size, turn, self.colour());
    }

    /// Shoots coming up round the foot, out to `reach`, full grown.
    pub(super) fn shoots(&mut self, pieces: &mut Pieces, reach: f32) {
        if self.look.bears != Bears::Shoots || self.show() != Some(Show::Ripe) {
            return;
        }
        let (tall, wide) = (self.look.part_size.x * 2.0, self.look.part_size.y);
        for i in 0..4 {
            let angle = i as f32 / 4.0 * TAU + self.between(-0.4, 0.4);
            let foot = Vec3::new(angle.cos(), 0.0, angle.sin()) * reach * self.between(0.3, 0.6);
            let colour = self.colour();
            pieces.stem(foot, foot + Vec3::Y * tall, (wide, wide * 0.6), colour);
        }
    }

    /// Flowers at `at`, for a plant whose crop is underground, full grown.
    pub(super) fn flowers_over_hidden(&mut self, pieces: &mut Pieces, at: Vec3) {
        if self.look.bears != Bears::Hidden || self.show() != Some(Show::Flowers) {
            return;
        }
        for _ in 0..3 {
            let angle = self.between(0.0, TAU);
            let side = Vec3::new(angle.cos(), 0.0, angle.sin()) * self.look.flower_size * 1.2;
            self.flower(pieces, at + side);
        }
    }
}
