//! Where a necklace lies on a body frame, in millimetres: `Skin`, the neck and the
//! front of the chest it rests on, and `Path`, the strand's middle line round the neck
//! by the angle from the front (towards the body's left, +x), as a hanging loop seen
//! from the front is a U: level round the back of the neck, dropping in front by its
//! length.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::body::Frame;
use shared::item_form::{NecklaceLength, Strand};

use super::{NecklaceLook, WORN_TOP, Wear};
use crate::item_art::body::trunk_half;

/// How far in front of the skin a worn strand's middle is, and how far out it reaches
/// either way from there: it lies flat, so it can't poke through what is over it.
const WORN_GAP: f32 = 0.6;
const WORN_HALF_DEEP: f32 = 0.5;
/// Worn strands are drawn at least this wide (half), to show on a token.
const WORN_HALF_WIDE: f32 = 3.5;
/// How far a strand hanging in front reaches, from the base of the neck down.
const DROP_COLLARBONE: f32 = 55.0;
const DROP_CHEST: f32 = 140.0;
const DROP_LONG: f32 = 250.0;
const DROP_TORC: f32 = 22.0;
/// How sharply the front falls away from the sides: higher is more of a V.
const FALL: f32 = 1.6;
/// How much wider than the neck a strand lying on the shoulders is, by half widths.
const SPREAD: f32 = 1.3;
/// A torc's opening at the front, either side, in radians.
pub(super) const TORC_GAP: f32 = 0.32;

/// The neck and the trunk in front of it.
pub(super) struct Skin<'a> {
    frame: &'a Frame,
}

impl<'a> Skin<'a> {
    pub fn of(frame: &'a Frame) -> Self {
        Self { frame }
    }

    pub fn neck(&self) -> f32 {
        self.frame.neck as f32
    }

    pub fn neck_base(&self) -> f32 {
        self.frame.neck_base as f32
    }

    pub fn chin(&self) -> f32 {
        self.frame.chin as f32
    }

    /// The skin in front at (`x`, `y`), and the way out from it: the neck's round
    /// above the trunk, the trunk's front where it stands further forwards; its
    /// slope both ways taken from points either side.
    pub fn front(&self, x: f32, y: f32) -> (Vec3, Vec3) {
        const STEP: f32 = 2.0;
        let z = self.depth(x, y);
        let across = (self.depth(x + STEP, y) - self.depth(x - STEP, y)) / (2.0 * STEP);
        let up = (self.depth(x, y + STEP) - self.depth(x, y - STEP)) / (2.0 * STEP);
        let out = Vec3::new(-across, -up, 1.0).normalize();
        (Vec3::new(x, y, z), out)
    }

    /// How far forwards the skin is at (`x`, `y`): on the neck, or the trunk's front.
    fn depth(&self, x: f32, y: f32) -> f32 {
        let neck = self.neck();
        let mut z = (neck * neck - x * x).max(0.0).sqrt();
        if y < self.neck_base() {
            let girth = trunk_half(self.frame, y);
            let (across, deep) = (girth.across.max(1.0), girth.deep(true).max(1.0));
            let share = (x / across).clamp(-0.99, 0.99);
            z = z.max(deep * (1.0 - share * share).sqrt());
        }
        z
    }
}

/// The strand's line, its thickness, and how it is worn.
pub(super) struct Path<'a> {
    pub skin: Skin<'a>,
    /// Half its thickness out from the skin, and half its width along it.
    pub half_deep: f32,
    pub half_wide: f32,
    /// Its middle line this far off the skin.
    pub off: f32,
    pub wear: Wear,
    /// Half as wide as the loop is, where it lies over the shoulders.
    spread: f32,
    /// The height it lies at round the back, and how far it falls in front.
    top: f32,
    drop: f32,
}

impl<'a> Path<'a> {
    pub fn new(look: &NecklaceLook, frame: &'a Frame, wear: Wear) -> Self {
        let skin = Skin::of(frame);
        let radius = super::strand::radius(&look.strand);
        let (half_deep, half_wide, off) = match wear {
            Wear::Alone => (radius, radius, radius + 0.5),
            Wear::Worn => (
                WORN_HALF_DEEP,
                radius.max(WORN_HALF_WIDE),
                WORN_GAP + WORN_HALF_DEEP,
            ),
        };
        debug_assert!(off + half_deep <= WORN_TOP || wear == Wear::Alone);
        let choker = look.length == NecklaceLength::Choker;
        let top = if choker {
            skin.neck_base() + (skin.chin() - skin.neck_base()) * 0.35
        } else {
            skin.neck_base() + 2.0 + off
        };
        let drop = match (look.strand, look.length) {
            (_, NecklaceLength::Choker) => 0.0,
            (Strand::Torc { .. }, _) => DROP_TORC,
            (_, NecklaceLength::Collarbone) => DROP_COLLARBONE,
            (_, NecklaceLength::Chest) => DROP_CHEST,
            (_, NecklaceLength::Long) => DROP_LONG,
        };
        let spread = if choker {
            skin.neck()
        } else {
            skin.neck() * SPREAD
        };
        Self {
            skin,
            half_deep,
            half_wide,
            off,
            wear,
            spread,
            top,
            drop,
        }
    }

    /// The middle of the strand this far round from the front, and the way out from
    /// the body there.
    pub fn at(&self, angle: f32) -> (Vec3, Vec3) {
        let (sin, cos) = angle.sin_cos();
        let x = self.spread * sin;
        if cos > 0.0 {
            let y = self.top - self.drop * cos.powf(FALL);
            let (skin, out) = self.skin.front(x, y);
            // Over the shoulders it rests on top of the trunk, beside the neck.
            let side = Vec3::new(x, y, self.skin.neck() * cos);
            let (point, out) = if skin.z >= side.z {
                (skin, out)
            } else {
                (side, Vec3::new(sin, 0.0, cos))
            };
            return (point + out * self.off, out);
        }
        // Round the back of the neck, level.
        let deep = self.skin.neck() + self.off;
        let point = Vec3::new(x + sin * self.off, self.top, deep * cos);
        (point, Vec3::new(sin, 0.0, cos).normalize())
    }

    /// The angles it is drawn at, `steps` of them, from the front all the way round;
    /// a torc's leave out its opening, from one end to the other.
    pub fn angles(&self, strand: &Strand, steps: usize) -> Vec<f32> {
        let (from, to, closed) = match strand {
            Strand::Torc { .. } => (TORC_GAP, TAU - TORC_GAP, false),
            _ => (0.0, TAU, true),
        };
        let last = if closed { steps } else { steps + 1 };
        (0..last)
            .map(|step| from + (to - from) * step as f32 / steps as f32)
            .collect()
    }

    /// How far round the line is, from the front, in millimetres.
    pub fn length(&self) -> f32 {
        let steps = 256;
        (0..steps)
            .map(|step| {
                let angle = |s: usize| s as f32 / steps as f32 * TAU;
                self.at(angle(step + 1)).0.distance(self.at(angle(step)).0)
            })
            .sum()
    }

    /// The lowest point in front, where things hang from.
    pub fn front(&self) -> (Vec3, Vec3) {
        self.at(0.0)
    }
}
