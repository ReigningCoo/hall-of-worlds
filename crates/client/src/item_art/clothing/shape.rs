//! What every piece of clothing is built from, in millimetres on a body frame: its
//! levels, how far it stands off the skin, and cloth round the trunk. Alone a piece is
//! as loose as it is cut. Worn (`Fit::Worn`) it clears the underclothes and, wherever
//! armour is worn over it, stays inside armour's 15 mm; a tail comes out through a
//! slit at the back, and a taur's upper body has no legs for a skirt to cover.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;
use shared::body::Frame;

use super::wearer::Wearer;
use crate::item_art::armour::surface::{grid, ring_grid};
use crate::item_art::armour::waist::Hips;
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::limb;

/// Round the trunk: the body's left, its front, its right and its back.
pub(super) const LEFT: f32 = 0.0;
pub(super) const FRONT: f32 = FRAC_PI_2;
pub(super) const RIGHT: f32 = 2.0 * FRAC_PI_2;
pub(super) const BACK: f32 = 3.0 * FRAC_PI_2;
/// Points round the trunk.
const ROUND: usize = 40;
/// Ease off the skin as cut, in millimetres: close, fitted, loose and full.
pub(super) const CLOSE: f32 = 5.0;
pub(super) const FITTED: f32 = 8.0;
pub(super) const LOOSE: f32 = 18.0;
pub(super) const FULL: f32 = 32.0;
/// Worn, the least it stands off the skin, outside the underclothes (2.5 mm on the
/// body, 12 mm hanging below the hips), and the most under armour (its 15 mm).
pub(super) const OVER: f32 = 5.0;
pub(super) const OVER_HANGING: f32 = 14.0;
pub(super) const UNDER_ARMOUR: f32 = 12.0;
/// How much further out cloth may hang under armour for each millimetre below the
/// crotch, as waist and leg armour stand further out down the legs.
pub(super) const UNDER_ARMOUR_FLARE: f32 = 0.15;
/// The lowest a hem falls: just off the ground.
const FLOOR: f32 = 15.0;

/// Whether a piece is drawn alone, as cut, or worn by someone.
#[derive(Clone, Copy, Debug)]
pub(in crate::item_art) enum Fit {
    Alone,
    Worn(Wearer),
}

/// One piece being built on a frame.
pub(super) struct Wear<'a> {
    pub frame: &'a Frame,
    pub fit: Fit,
    /// How thick its cloth or leather is drawn.
    pub thick: f32,
}

impl Wear<'_> {
    fn f(value: i32) -> f32 {
        value as f32
    }

    pub fn wearer(&self) -> Option<&Wearer> {
        match &self.fit {
            Fit::Alone => None,
            Fit::Worn(wearer) => Some(wearer),
        }
    }

    pub fn hips(&self) -> Hips<'_> {
        Hips { frame: self.frame }
    }

    pub fn shoulder(&self) -> f32 {
        Self::f(self.frame.shoulder)
    }

    pub fn neck_base(&self) -> f32 {
        Self::f(self.frame.neck_base)
    }

    pub fn chest(&self) -> f32 {
        Self::f(self.frame.chest)
    }

    pub fn waist(&self) -> f32 {
        Self::f(self.frame.waist)
    }

    pub fn hip(&self) -> f32 {
        Self::f(self.frame.hip)
    }

    pub fn crotch(&self) -> f32 {
        self.hips().crotch()
    }

    pub fn knee(&self) -> f32 {
        Self::f(self.frame.knee)
    }

    pub fn ankle(&self) -> f32 {
        Self::f(self.frame.ankle)
    }

    pub fn neck(&self) -> f32 {
        Self::f(self.frame.neck)
    }

    /// `share` of the way from `a` to `b`.
    pub fn between(a: f32, b: f32, share: f32) -> f32 {
        a + (b - a) * share
    }

    /// Where a hem meant for `height` falls: never on the ground, and at the hips of a
    /// taur's upper body, which has a beast's body below it.
    pub fn hem(&self, height: f32) -> f32 {
        let taur = self.wearer().is_some_and(|wearer| wearer.taur);
        let height = height.max(FLOOR);
        if taur { height.max(self.hip()) } else { height }
    }

    /// How far off the skin cloth cut `ease` off it stands at `height` on the trunk.
    pub fn ease(&self, ease: f32, height: f32) -> f32 {
        let Some(wearer) = self.wearer() else {
            return ease;
        };
        let ease = self.cinched(ease, height);
        let crotch = self.crotch();
        let (least, most) = if height < crotch {
            let most = match wearer.covered.skirt {
                true => OVER_HANGING + (crotch - height) * UNDER_ARMOUR_FLARE,
                false => f32::MAX,
            };
            (OVER_HANGING, most)
        } else {
            let most = if wearer.covered.torso {
                UNDER_ARMOUR
            } else {
                f32::MAX
            };
            (OVER, most)
        };
        ease.max(least).min(most)
    }

    /// The same for a sleeve.
    pub fn sleeve_ease(&self, ease: f32) -> f32 {
        match self.wearer() {
            None => ease,
            Some(wearer) if wearer.covered.arms => ease.clamp(OVER, UNDER_ARMOUR),
            Some(_) => ease.max(OVER),
        }
    }

    /// A point on cloth round the trunk at this height and angle, with the way out.
    pub fn at(&self, height: f32, angle: f32, ease: f32) -> (Vec3, Vec3) {
        self.hips().at(height, angle, self.ease(ease, height))
    }

    /// Cloth round the trunk from `top` down to `bottom` (each by the angle round),
    /// `ease(height, angle)` off the skin. Worn by someone with a tail, it is slit at
    /// the back from just above the tail down to the hem, bound in a darker shade.
    pub fn trunk(
        &self,
        model: &mut Model,
        stuff: Stuff,
        top: &dyn Fn(f32) -> f32,
        bottom: &dyn Fn(f32) -> f32,
        ease: &dyn Fn(f32, f32) -> f32,
    ) {
        let lowest = (0..ROUND)
            .map(|at| bottom(at as f32 / ROUND as f32 * TAU))
            .fold(f32::MAX, f32::min);
        let slit = self
            .wearer()
            .and_then(|wearer| wearer.tail)
            .filter(|&(root, _)| root.y > lowest);
        let Some((root, radius)) = slit else {
            self.round(model, stuff, top, bottom, ease, None);
            return;
        };
        let slit_top = root.y + radius;
        let split = |angle: f32| slit_top.max(bottom(angle)).min(top(angle));
        // Only cloth above the slit's top goes right round.
        let highest = (0..ROUND)
            .map(|at| top(at as f32 / ROUND as f32 * TAU))
            .fold(f32::MIN, f32::max);
        if highest > slit_top {
            self.round(model, stuff, top, &split, ease, None);
        }
        let gap = self.slit_gap(radius);
        self.round(model, stuff, &split, bottom, ease, Some((BACK, gap)));
        for side in [-1.0, 1.0] {
            let angle = BACK + side * gap;
            let edge = |share: f32| {
                let height = Self::between(split(angle), bottom(angle), share);
                let (point, _) = self.at(height, angle, ease(height, angle) + 1.0);
                (point, 2.0)
            };
            limb(model.of(Stuff::Shade), &[edge(0.0), edge(0.5), edge(1.0)]);
        }
    }

    /// Cloth round the trunk as `trunk`, but open down the front, `gap` either side of
    /// it: a vest worn open.
    pub fn open_front(
        &self,
        model: &mut Model,
        stuff: Stuff,
        top: &dyn Fn(f32) -> f32,
        bottom: &dyn Fn(f32) -> f32,
        ease: &dyn Fn(f32, f32) -> f32,
        gap: f32,
    ) {
        self.round(model, stuff, top, bottom, ease, Some((FRONT, gap)));
    }

    /// How far either side of the back a tail's slit is open.
    fn slit_gap(&self, radius: f32) -> f32 {
        (radius + 12.0) / Self::f(self.frame.hips_half.1).max(40.0)
    }

    /// Whether this place on the trunk is in a tail's slit, where nothing is drawn.
    pub fn in_slit(&self, height: f32, angle: f32) -> bool {
        let Some((root, radius)) = self.wearer().and_then(|wearer| wearer.tail) else {
            return false;
        };
        let from_back = ((angle - BACK + PI).rem_euclid(TAU) - PI).abs();
        height < root.y + radius && from_back < self.slit_gap(radius)
    }

    /// Right round, or open `gap.1` either side of the angle `gap.0`.
    fn round(
        &self,
        model: &mut Model,
        stuff: Stuff,
        top: &dyn Fn(f32) -> f32,
        bottom: &dyn Fn(f32) -> f32,
        ease: &dyn Fn(f32, f32) -> f32,
        gap: Option<(f32, f32)>,
    ) {
        let angle = |u: f32| match gap {
            Some((at, gap)) => at + gap + u * (TAU - 2.0 * gap),
            None => u * TAU,
        };
        let surface = |u: f32, v: f32| {
            let angle = angle(u);
            let height = Self::between(top(angle), bottom(angle), v);
            self.at(height, angle, ease(height, angle))
        };
        let tallest = (0..ROUND)
            .map(|at| {
                let angle = angle(at as f32 / ROUND as f32);
                top(angle) - bottom(angle)
            })
            .fold(0.0, f32::max);
        let rows = ((tallest / 35.0).ceil() as usize).max(2);
        let points = match gap {
            Some(_) => grid(&surface, rows, ROUND),
            None => ring_grid(&surface, rows, ROUND),
        };
        plate(model.of(stuff), &points, self.thick, gap.is_none());
    }

    /// A band right round the trunk at `height`, `high` tall, just over cloth `ease`
    /// off: hems, waistbands, sashes and stripes.
    pub fn band(&self, model: &mut Model, stuff: Stuff, height: f32, high: f32, ease: f32) {
        let ease = move |height: f32, _| self.ease(ease, height) + self.thick + 0.5;
        self.trunk(
            model,
            stuff,
            &|_| height + high / 2.0,
            &|_| height - high / 2.0,
            &ease,
        );
    }
}
