//! Cloth over the head, in millimetres on a body frame: the head is an ellipsoid (the
//! frame's, which beast peoples' skulls keep inside), cloth stands `off` it over the
//! hair and any linen coif, and keeps inside a helmet's bowl worn over it. Places on
//! it go by `down`, 0 at the crown, 1 at the chin and on down the neck, and an angle
//! from the front towards the body's left. Horns come out through bound slits.

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use super::shape::Wear;
use crate::item_art::armour::neck::Around;
use crate::item_art::armour::surface::{grid, ring_grid};
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::limb;

/// Off the head: clear of hair and of a linen coif (11 mm).
pub(super) const OFF_HEAD: f32 = 15.0;
/// Under a helmet's bowl: inside its shell, 10 mm off the head.
const UNDER_HELMET: f32 = 7.0;
/// Points round the head.
const ROUND: usize = 36;

/// How far either side of the front a face opening reaches, by `down`.
pub(super) type Gap<'a> = &'a dyn Fn(f32) -> f32;

impl Wear<'_> {
    pub fn around(&self) -> Around<'_> {
        Around { frame: self.frame }
    }

    pub fn chin(&self) -> f32 {
        self.frame.chin as f32
    }

    /// From the chin to the crown.
    pub fn head_height(&self) -> f32 {
        (self.frame.height - self.frame.chin) as f32
    }

    /// The height `down` from the crown: half a turn round the ellipsoid to the chin,
    /// then straight on down the neck a head's height for each 1 more.
    pub fn head_y(&self, down: f32) -> f32 {
        let high = self.head_height();
        if down <= 1.0 {
            self.chin() + high / 2.0 * (1.0 + (down * PI).cos())
        } else {
            self.chin() - (down - 1.0) * high
        }
    }

    /// `down` at this height.
    pub fn down_at(&self, y: f32) -> f32 {
        let high = self.head_height();
        if y >= self.chin() {
            let cos = ((y - self.chin()) / (high / 2.0) - 1.0).clamp(-1.0, 1.0);
            cos.acos() / PI
        } else {
            1.0 + (self.chin() - y) / high
        }
    }

    /// The head's half widths at this height, across and front to back, `off` out: the
    /// ellipsoid's above its middle, never narrower than the neck below it.
    pub fn head_half(&self, y: f32, off: f32) -> (f32, f32) {
        let high = self.head_height() / 2.0;
        let centre = self.chin() + high;
        let shrink = (1.0 - ((y - centre) / high).powi(2)).max(0.0).sqrt();
        let (across, deep) = (self.frame.head.0 as f32, self.frame.head.1 as f32);
        let neck = if y < centre { self.neck() } else { 0.0 };
        (
            (across * shrink).max(neck) + off,
            (deep * shrink).max(neck) + off,
        )
    }

    /// How far cloth meant `off` the head stands at this place: inside a helmet worn
    /// over it, above the helmet's edge.
    pub fn head_off(&self, angle: f32, y: f32, off: f32) -> f32 {
        let rim = self
            .wearer()
            .and_then(|wearer| wearer.head.helmet_at(angle));
        match rim {
            Some(rim) if y > rim - 10.0 => off.min(UNDER_HELMET),
            _ => off,
        }
    }

    /// Whether a helmet is worn over what is on the head.
    pub fn helmeted(&self) -> bool {
        self.wearer()
            .is_some_and(|wearer| wearer.head.helmet.is_some())
    }

    /// A point on cloth over the head `down` from the crown at this angle, `off` it,
    /// and the way out there.
    pub fn on_head(&self, angle: f32, down: f32, off: f32) -> (Vec3, Vec3) {
        let y = self.head_y(down);
        let high = self.head_height() / 2.0;
        let centre = self.chin() + high;
        let (across, deep) = self.head_half(y, 0.0);
        let (sin, cos) = angle.sin_cos();
        let skin = Vec3::new(across * sin, y, deep * cos);
        // Square to the ellipsoid above its middle, straight out below it.
        let out = if y > centre {
            let t = ((y - centre) / high).min(1.0);
            let shrink = (1.0 - t * t).max(0.0).sqrt();
            let (a, d) = (self.frame.head.0 as f32, self.frame.head.1 as f32);
            Vec3::new(shrink * sin / a, t / high, shrink * cos / d)
        } else {
            Vec3::new(sin / across.max(1.0), 0.0, cos / deep.max(1.0))
        }
        .normalize_or(Vec3::Y);
        let off = self.head_off(angle, y, off);
        (skin + out * off, out)
    }

    /// Cloth over the head from `top` down to `bottom` (each a `down`), `off` it, open
    /// at the front `gap(down)` either side.
    #[allow(clippy::too_many_arguments)]
    pub fn over_head(
        &self,
        model: &mut Model,
        stuff: Stuff,
        (top, bottom): (f32, f32),
        off: &dyn Fn(f32, f32) -> f32,
        gap: Gap,
    ) {
        let rows = (((bottom - top) * 16.0).ceil() as usize).max(2);
        let open =
            (0..=rows).any(|row| gap(Self::between(top, bottom, row as f32 / rows as f32)) > 0.0);
        let surface = |u: f32, v: f32| {
            let down = Self::between(top, bottom, v);
            let gap = gap(down);
            let angle = gap + u * (TAU - 2.0 * gap);
            self.on_head(angle, down, off(angle, down))
        };
        let points = if open {
            grid(&surface, rows, ROUND)
        } else {
            ring_grid(&surface, rows, ROUND)
        };
        plate(model.of(stuff), &points, self.thick, !open);
    }

    /// Bound slits where the wearer's horns come out through cloth `off` the head.
    pub fn horn_slits(&self, model: &mut Model, off: f32) {
        let Some(horns) = self.wearer().and_then(|wearer| wearer.head.horns) else {
            return;
        };
        let high = self.head_height() / 2.0;
        let centre = Vec3::Y * (self.chin() + high);
        let (across, deep) = (self.frame.head.0 as f32, self.frame.head.1 as f32);
        for (root, way, thick) in horns {
            // Out along the horn until it is beyond the cloth.
            let outside = |at: Vec3| {
                let from = at - centre;
                let scaled = Vec3::new(
                    from.x / (across + off),
                    from.y / (high + off),
                    from.z / (deep + off),
                );
                scaled.length() >= 1.0
            };
            let Some(step) = (0..60).find(|&step| outside(root + way * step as f32 * 2.0)) else {
                continue;
            };
            let at = root + way * step as f32 * 2.0;
            let side = way.cross(Vec3::Z).normalize_or(Vec3::X);
            let other = way.cross(side);
            let ring: Vec<(Vec3, f32)> = (0..=16)
                .map(|point| {
                    let turn = point as f32 / 16.0 * TAU;
                    let reach = thick + 3.0;
                    (at + (side * turn.cos() + other * turn.sin()) * reach, 2.5)
                })
                .collect();
            limb(model.of(Stuff::Shade), &ring);
        }
    }
}
