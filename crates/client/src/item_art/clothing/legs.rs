//! Cloth down the legs, for trousers, and a belt's cinch: whatever is under a worn
//! belt is drawn in at the waist, keeping the order it was in.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::body::Side;

use super::shape::{OVER, UNDER_ARMOUR, UNDER_ARMOUR_FLARE, Wear};
use crate::item_art::armour::legs::Leg;
use crate::item_art::armour::surface::ring_grid;
use crate::item_art::armour::{Model, Stuff, plate};

/// Points round a leg.
const ROUND: usize = 28;
/// Under a belt, how much of the ease off the skin is left: cloth keeps its order,
/// the looser outside the closer.
const CINCH: f32 = 0.12;
/// How far above and below a belt's edge cloth is drawn in, easing out again.
const CINCH_REACH: f32 = 60.0;
/// How far off the skin a worn belt is: outside whatever it cinches.
pub(super) const BELT_EASE: f32 = 12.0;

impl Wear<'_> {
    /// `ease` at `height`, drawn in by a worn belt.
    pub(super) fn cinched(&self, ease: f32, height: f32) -> f32 {
        let Some(high) = self.wearer().and_then(|wearer| wearer.belt) else {
            return ease;
        };
        let from_belt = ((height - self.belt_height()).abs() - high / 2.0).max(0.0);
        let share = (from_belt / CINCH_REACH).min(1.0);
        // Smoothly from fully cinched at the belt to as cut beyond its reach.
        let share = share * share * (3.0 - 2.0 * share);
        let cinched = OVER + (ease - OVER).max(0.0) * CINCH;
        Self::between(cinched, ease, share).min(ease)
    }

    /// Where a belt's middle is worn: at the waist.
    pub(super) fn belt_height(&self) -> f32 {
        self.waist()
    }

    /// How far off a leg's skin cloth cut `ease` off it stands at `height`: worn, over
    /// the hose and inside any leg or waist armour.
    pub(super) fn leg_ease(&self, ease: f32, height: f32) -> f32 {
        let Some(wearer) = self.wearer() else {
            return ease;
        };
        let most = match wearer.covered.skirt {
            true => UNDER_ARMOUR + (self.crotch() - height).max(0.0) * UNDER_ARMOUR_FLARE,
            false => f32::MAX,
        };
        ease.max(OVER).min(most)
    }

    /// Cloth round one leg from `top` down to `bottom`, `ease(height)` off it.
    pub(super) fn leg(
        &self,
        model: &mut Model,
        stuff: Stuff,
        side: Side,
        (top, bottom): (f32, f32),
        ease: &dyn Fn(f32) -> f32,
    ) {
        let leg = Leg {
            frame: self.frame,
            side,
        };
        let surface = |u: f32, v: f32| {
            let height = Self::between(top, bottom, v);
            leg.at(height, u * TAU, self.leg_ease(ease(height), height))
        };
        let rows = (((top - bottom) / 35.0).ceil() as usize).max(2);
        plate(
            model.of(stuff),
            &ring_grid(&surface, rows, ROUND),
            self.thick,
            true,
        );
    }

    /// A band round a leg at `height`, `high` tall, over cloth `ease` off it.
    pub(super) fn leg_band(
        &self,
        model: &mut Model,
        stuff: Stuff,
        side: Side,
        (height, high): (f32, f32),
        ease: f32,
    ) {
        let over = self.leg_ease(ease, height) + self.thick + 0.8;
        let band = (height + high / 2.0, height - high / 2.0);
        self.leg(model, stuff, side, band, &|_| over);
    }

    /// A point on a leg, `lift` over cloth `ease` off it, with the way out.
    pub(super) fn on_leg(
        &self,
        side: Side,
        (height, angle): (f32, f32),
        ease: f32,
        lift: f32,
    ) -> (Vec3, Vec3) {
        let leg = Leg {
            frame: self.frame,
            side,
        };
        let lift = self.leg_ease(ease, height) + self.thick + lift;
        leg.at(height, angle, lift)
    }
}
