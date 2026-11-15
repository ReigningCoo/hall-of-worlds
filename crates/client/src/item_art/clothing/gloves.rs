//! Gloves: five fingers, mittens or fingerless, sewn, knitted or nalbound in rows,
//! felted or fur, each on its own hand as the body hangs it; a cuff at the wrist, a
//! flared gauntlet cuff or one to the elbow, perhaps embroidered or fringed, and a fur
//! lining showing at its top. Alone they lie side by side as gauntlets do.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::body::Side;
use shared::clothing::{Cuff, Fingers, GloveDecoration, GloveMake, Gloves};

use super::shape::{UNDER_ARMOUR, Wear};
use crate::item_art::armour::hands::{GloveCut, cloth_glove};
use crate::item_art::armour::surface::ring_grid;
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::limb;

/// Points round a cuff.
const ROUND: usize = 24;

impl Wear<'_> {
    /// Both gloves.
    pub(super) fn gloves(&self, model: &mut Model, gloves: &Gloves, digits: u8) {
        for side in Side::BOTH {
            self.glove(model, gloves, side, digits);
        }
    }

    /// One glove, in place on its hand.
    pub(super) fn glove(&self, model: &mut Model, gloves: &Gloves, side: Side, digits: u8) {
        let (stuff, swell, rows) = match gloves.make {
            GloveMake::Sewn => (Stuff::Main, 1.5, None),
            GloveMake::Knitted => (Stuff::Main, 2.2, Some(5.0)),
            GloveMake::Nalbound => (Stuff::Main, 3.0, Some(7.0)),
            GloveMake::Felted => (Stuff::Main, 3.5, None),
            GloveMake::Fur => (Stuff::Main, 5.0, None),
        };
        let cut = GloveCut {
            mitten: gloves.fingers == Fingers::Mitten,
            fingerless: gloves.fingers == Fingers::Fingerless,
            swell,
            stuff,
            rows,
        };
        // The cuff first, so a glove's parts start with its own cloth.
        self.glove_cuff(model, gloves, side, stuff, swell);
        cloth_glove(model, self.frame, side, digits, &cut);
    }

    /// The cuff up the forearm, its decoration and lining.
    fn glove_cuff(&self, model: &mut Model, gloves: &Gloves, side: Side, stuff: Stuff, swell: f32) {
        let (wrist, elbow) = (self.wrist(), self.elbow());
        let (top, flare) = match gloves.cuff {
            Cuff::Wrist => (wrist - 55.0, 0.0),
            Cuff::Gauntlet => (wrist - 110.0, 24.0),
            Cuff::Elbow => (elbow + 15.0, 0.0),
        };
        let bottom = wrist + 8.0;
        let ease = move |down: f32| {
            let up = ((bottom - down) / (bottom - top)).clamp(0.0, 1.0);
            self.cuff_ease(swell + 1.5 + flare * up * up)
        };
        self.forearm(model, stuff, side, (top, bottom), &ease);
        let over = |at: f32| ease(at) + self.thick + 0.8;
        if gloves.fur_lined {
            self.forearm(model, Stuff::Bone, side, (top - 6.0, top + 14.0), &|d| {
                over(d) + 1.5
            });
        }
        match gloves.decoration {
            GloveDecoration::EmbroideredCuff => {
                let band = (top + 12.0, top + 32.0);
                self.forearm(model, Stuff::Field, side, band, &over);
            }
            GloveDecoration::FringedCuff => {
                let arm = self.arm(side);
                for at in 0..18 {
                    let angle = at as f32 / 18.0 * TAU;
                    let (from, out) = arm.at(top + 4.0, angle, over(top));
                    let to = from + out * 4.0 + Vec3::NEG_Y * 22.0;
                    limb(model.of(Stuff::Shade), &[(from, 1.4), (to, 1.0)]);
                }
            }
            // Perfumed leather looks like any other.
            GloveDecoration::Perfumed | GloveDecoration::None => {}
        }
    }

    /// How far off the forearm a cuff stands: inside any arm armour worn over it.
    fn cuff_ease(&self, ease: f32) -> f32 {
        match self.wearer() {
            Some(wearer) if wearer.covered.arms => ease.min(UNDER_ARMOUR),
            _ => ease,
        }
    }

    /// Cloth round one forearm from `from` to `to` down the arm, `ease(down)` off it.
    fn forearm(
        &self,
        model: &mut Model,
        stuff: Stuff,
        side: Side,
        (from, to): (f32, f32),
        ease: &dyn Fn(f32) -> f32,
    ) {
        let arm = self.arm(side);
        let surface = |u: f32, v: f32| {
            let down = Self::between(from, to, v);
            arm.at(down, u * TAU, ease(down))
        };
        let rows = (((to - from) / 30.0).ceil() as usize).max(2);
        plate(
            model.of(stuff),
            &ring_grid(&surface, rows, ROUND),
            self.thick,
            true,
        );
    }
}
