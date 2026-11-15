//! Trousers: the seat, then each leg to its length, as close or as full as they are
//! cut, gathered, buttoned at the knee or strapped under the foot; the waistband by
//! how they are held up, a fall front, and leg wraps. Worn, they are hidden under
//! whatever hangs over them, cuffed and buckled above hooves or tied above talons, and
//! slit at the back for a tail; a taur's upper body has only their waist.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use shared::body::Side;
use shared::clothing::{Fullness, LegEnds, TrouserFront, TrouserLength, TrouserWaist, Trousers};

use super::shape::{CLOSE, FITTED, FRONT, FULL, LOOSE, Wear};
use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::{blob, limb};

/// Round a leg: its front, outside, back and inside.
const LEG_FRONT: f32 = 0.0;
const LEG_OUTSIDE: f32 = FRAC_PI_2;
const LEG_INSIDE: f32 = -FRAC_PI_2;

impl Wear<'_> {
    pub(super) fn trousers(&self, model: &mut Model, trousers: &Trousers) {
        let (waist, crotch, knee, ankle) = (self.waist(), self.crotch(), self.knee(), self.ankle());
        let wearer = self.wearer();
        let hooves = wearer.is_some_and(|wearer| wearer.hooves);
        let talons = wearer.is_some_and(|wearer| wearer.talons);
        let ankle_length = trousers.length == TrouserLength::Ankle;
        let end = match trousers.length {
            TrouserLength::Knee => knee - 30.0,
            TrouserLength::Calf => Self::between(knee, ankle, 0.5),
            TrouserLength::Ankle if hooves || talons => ankle + 50.0,
            TrouserLength::Ankle => ankle + 25.0,
        };
        let ease = match trousers.fullness {
            Fullness::Fitted => CLOSE,
            Fullness::Loose => FITTED + 4.0,
            Fullness::VeryFull => FULL,
            Fullness::Pleated => LOOSE,
        };
        // Drawn in towards the end where gathered, buttoned, cuffed or tied.
        let drawn_in = matches!(trousers.ends, LegEnds::GatheredAtAnkle | LegEnds::AtKnee)
            || trousers.fullness == Fullness::VeryFull
            || (ankle_length && (hooves || talons));
        let seat = ease.min(FITTED + 6.0);
        let leg_ease = move |height: f32| {
            let near = ((height - end) / 120.0).clamp(0.0, 1.0);
            let full = match trousers.fullness {
                // The hakama widens down the leg.
                Fullness::Pleated => ease + (crotch - height).max(0.0) * 0.1,
                _ if drawn_in => Self::between(CLOSE + 2.0, ease, near.sqrt()),
                _ => ease,
            };
            // As close as the seat at the top of the leg, full further down.
            let down = ((crotch + 30.0 - height) / 150.0).clamp(0.0, 1.0);
            Self::between(seat.min(full), full, down)
        };
        // Hidden above whatever hangs over them.
        let under = wearer
            .and_then(|wearer| wearer.over_trousers())
            .map_or(f32::MAX, |hem| hem + 30.0);
        let high = if trousers.fullness == Fullness::Pleated {
            waist + 40.0
        } else {
            waist + 20.0
        };
        let top = high.min(under);
        let taur = wearer.is_some_and(|wearer| wearer.taur);
        let seat_bottom = if taur { self.hip() } else { crotch - 10.0 };
        if top > seat_bottom {
            self.trunk(model, Stuff::Main, &|_| top, &|_| seat_bottom, &|_, _| seat);
        }
        if !taur {
            let leg_top = (crotch + 30.0).min(under);
            for side in Side::BOTH {
                if leg_top > end {
                    self.leg(model, Stuff::Main, side, (leg_top, end), &leg_ease);
                }
                if end < under {
                    self.trouser_end(model, trousers, side, end, leg_ease(end));
                }
                if trousers.leg_wraps && knee - 40.0 < under {
                    self.leg_wraps(model, side, (knee - 40.0, end + 10.0), leg_ease(knee));
                }
            }
        }
        if top >= high {
            self.waistband(model, trousers, seat);
            if trousers.front == TrouserFront::FallFront {
                let flap = [(waist + 5.0, FRONT), (crotch + 30.0, FRONT)];
                self.strip(model, Stuff::Shade, &flap, &|_, _| seat, 70.0);
                for side in [-1.0, 1.0] {
                    let at = (waist + 2.0, FRONT + side * 0.22);
                    self.button(model, Stuff::Rivet, at, &|_, _| seat + 2.0, 3.5);
                }
            }
        }
    }

    /// How the waist is held up.
    fn waistband(&self, model: &mut Model, trousers: &Trousers, ease: f32) {
        let waist = self.waist();
        let (high, stuff) = match trousers.waist {
            TrouserWaist::Drawstring => (14.0, Stuff::Shade),
            TrouserWaist::Buttons => (30.0, Stuff::Main),
            TrouserWaist::Points => (24.0, Stuff::Main),
            TrouserWaist::Ties => (40.0, Stuff::Shade),
        };
        let at = waist + 20.0 - high / 2.0;
        self.band(model, stuff, at, high, ease);
        let on = |angle: f32| (at, angle);
        let over = move |_: f32, _: f32| ease + 2.0;
        match trousers.waist {
            TrouserWaist::Drawstring => {
                // The cord's two ends hanging at the front.
                for side in [-1.0, 1.0] {
                    let cord = [on(FRONT + side * 0.04), (at - 70.0, FRONT + side * 0.08)];
                    self.line(model, Stuff::Backing, &cord, &over, 2.5);
                }
            }
            TrouserWaist::Buttons => self.button(model, Stuff::Rivet, on(FRONT), &over, 4.5),
            TrouserWaist::Points => {
                // Tied to the doublet: pairs of laces all round.
                for at in 0..8 {
                    let angle = FRONT + at as f32 * PI / 4.0 + 0.2;
                    self.button(model, Stuff::Backing, on(angle), &over, 3.0);
                }
            }
            TrouserWaist::Ties => {
                // The hakama's straps, crossed and tied at the front.
                let tie = [on(FRONT - 0.3), (at - 60.0, FRONT + 0.05)];
                self.line(model, Stuff::Shade, &tie, &over, 6.0);
                self.button(model, Stuff::Shade, on(FRONT), &over, 7.0);
            }
        }
    }

    /// How one leg ends: gathered, buttoned at the knee, strapped under the foot, or
    /// cuffed above hooves and tied above talons.
    fn trouser_end(&self, model: &mut Model, trousers: &Trousers, side: Side, end: f32, ease: f32) {
        let wearer = self.wearer();
        let near_foot = trousers.length == TrouserLength::Ankle;
        if near_foot && wearer.is_some_and(|wearer| wearer.hooves) {
            // A leather cuff buckled snug above the hoof.
            self.leg_band(model, Stuff::Backing, side, (end + 18.0, 36.0), ease);
            let (point, out) = self.on_leg(side, (end + 18.0, LEG_OUTSIDE), ease, 4.0);
            blob(model.of(Stuff::Rivet), point + out * 2.0, Vec3::splat(6.0));
            return;
        }
        if near_foot && wearer.is_some_and(|wearer| wearer.talons) {
            // Cords tied round the shank.
            for at in [end + 8.0, end + 22.0] {
                self.leg_band(model, Stuff::Backing, side, (at, 4.0), ease);
            }
            return;
        }
        match trousers.ends {
            LegEnds::Open => {}
            LegEnds::GatheredAtAnkle => {
                self.leg_band(model, Stuff::Shade, side, (end + 8.0, 14.0), ease)
            }
            LegEnds::AtKnee => {
                self.leg_band(model, Stuff::Shade, side, (end + 12.0, 22.0), ease);
                for at in 0..3 {
                    let place = (end + 30.0 + at as f32 * 25.0, LEG_OUTSIDE);
                    let (point, out) = self.on_leg(side, place, ease, 2.0);
                    blob(model.of(Stuff::Rivet), point + out, Vec3::splat(3.5));
                }
            }
            LegEnds::Stirrups => {
                // A strap from the inside of the hem down under the foot and up.
                let low = 8.0;
                let inside = self.on_leg(side, (end, LEG_INSIDE), ease, 1.0).0;
                let outside = self.on_leg(side, (end, LEG_OUTSIDE), ease, 1.0).0;
                let under = (inside + outside) / 2.0;
                let under = Vec3::new(under.x, low, under.z - 15.0);
                let path = [(inside, 4.0), (under, 4.0), (outside, 4.0)];
                limb(model.of(Stuff::Shade), &path);
            }
        }
    }

    /// Cloth strips wound round from the ankle up to below the knee.
    fn leg_wraps(&self, model: &mut Model, side: Side, (top, bottom): (f32, f32), ease: f32) {
        let turns = ((top - bottom) / 45.0).max(1.0);
        let steps = (turns * 10.0) as usize;
        let path: Vec<_> = (0..=steps)
            .map(|step| {
                let t = step as f32 / steps as f32;
                let height = Self::between(bottom, top, t);
                let angle = LEG_FRONT + t * turns * 2.0 * PI * side.sign() as f32;
                self.on_leg(side, (height, angle), ease, 1.5)
            })
            .collect();
        self.ribbon(model, Stuff::Cloth, &path, 34.0);
    }
}
