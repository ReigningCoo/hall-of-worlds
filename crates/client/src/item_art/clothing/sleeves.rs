//! Sleeves: cloth down an arm, close, belled or puffed, cuffs, rolls at the shoulder,
//! the lacing of sleeves laced on, and long panels hanging from the arm. Wing-arms
//! take none, so a worn piece leaves them bare.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::body::Side;

use super::shape::Wear;
use crate::item_art::armour::arms::Arm;
use crate::item_art::armour::surface::{grid, ring_grid};
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::blob;

/// Points round an arm.
const ROUND: usize = 24;

impl Wear<'_> {
    pub(super) fn arm(&self, side: Side) -> Arm<'_> {
        Arm {
            frame: self.frame,
            side,
            bend: 0.0,
        }
    }

    /// Whether this piece's sleeves are drawn: not on wing-arms.
    pub fn sleeved(&self) -> bool {
        !self.wearer().is_some_and(|wearer| wearer.wings)
    }

    /// How far down the arm the elbow and wrist are.
    pub fn elbow(&self) -> f32 {
        self.frame.elbow_length as f32
    }

    pub fn wrist(&self) -> f32 {
        (self.frame.elbow_length + self.frame.wrist_length) as f32
    }

    /// Both sleeves from `from` to `to` millimetres down the arm from the shoulder
    /// joint, `ease(down)` off the skin.
    pub fn sleeves(
        &self,
        model: &mut Model,
        stuff: Stuff,
        (from, to): (f32, f32),
        ease: &dyn Fn(f32) -> f32,
    ) {
        if !self.sleeved() {
            return;
        }
        for side in Side::BOTH {
            let arm = self.arm(side);
            let surface = |u: f32, v: f32| {
                let down = Self::between(from, to, v);
                arm.at(down, u * TAU, self.sleeve_ease(ease(down)))
            };
            let rows = (((to - from) / 35.0).ceil() as usize).max(2);
            plate(
                model.of(stuff),
                &ring_grid(&surface, rows, ROUND),
                self.thick,
                true,
            );
        }
    }

    /// A band round both sleeves at `down`, `high` long, over cloth `ease` off: cuffs,
    /// the seam of a sleeve laced on.
    pub fn cuffs(&self, model: &mut Model, stuff: Stuff, down: f32, high: f32, ease: f32) {
        let over = move |_| ease + self.thick + 0.8;
        self.sleeves(model, stuff, (down - high / 2.0, down + high / 2.0), &over);
    }

    /// A roll or wing round the top of each sleeve.
    pub fn rolls(&self, model: &mut Model, stuff: Stuff, ease: f32) {
        if !self.sleeved() {
            return;
        }
        for side in Side::BOTH {
            let arm = self.arm(side);
            for at in 0..10 {
                let angle = at as f32 / 10.0 * TAU;
                if (angle - std::f32::consts::PI).abs() < 0.9 {
                    continue; // Not between the arm and the body.
                }
                let (point, out) = arm.at(25.0, angle, self.sleeve_ease(ease) + 8.0);
                blob(model.of(stuff), point, Vec3::splat(9.0) + out.abs() * 4.0);
            }
        }
    }

    /// Ties where each sleeve is laced on at the shoulder.
    pub fn sleeve_ties(&self, model: &mut Model, ease: f32) {
        if !self.sleeved() {
            return;
        }
        for side in Side::BOTH {
            let arm = self.arm(side);
            for angle in [0.6, 0.0, -0.6] {
                let (point, out) = arm.at(10.0, angle, self.sleeve_ease(ease) + 3.0);
                blob(
                    model.of(Stuff::Backing),
                    point,
                    Vec3::splat(4.0) + out.abs() * 2.0,
                );
            }
        }
    }

    /// A panel hanging from each arm at `down`, `long` and `wide`, behind the arm and
    /// a little out from it: hanging sleeves and a kimono's deep sleeves.
    pub fn hanging(&self, model: &mut Model, stuff: Stuff, down: f32, long: f32, wide: f32) {
        if !self.sleeved() {
            return;
        }
        for side in Side::BOTH {
            let arm = self.arm(side);
            let outward = Vec3::X * side.sign() as f32;
            let start = arm.centre(down) + outward * (arm.skin(down) + self.sleeve_ease(10.0));
            let surface = |u: f32, v: f32| {
                let point = start + Vec3::new(0.0, -long * v, (u - 0.6) * wide);
                (point, outward)
            };
            plate(model.of(stuff), &grid(&surface, 6, 4), self.thick, false);
        }
    }
}
