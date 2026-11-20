//! Reshaping a body without changing what it is: longer legs, a broader or
//! slimmer trunk, a shorter back, a bigger head. For a young animal's
//! proportions and a castrated one's build (`beasts/stage.rs`). Each takes
//! thousandths (`WHOLE` leaves the body as it is), and keeps it on the ground.

use super::blend::WHOLE;
use super::layout::{Girdle, Layout, Role};
use super::rings::Ring;

/// `value` this many thousandths of itself.
fn times(value: i32, share: i32) -> i32 {
    value * share / WHOLE
}

fn ring_times(ring: Ring, share: i32) -> Ring {
    Ring {
        along: times(ring.along, share),
        half: times(ring.half, share),
        belly: times(ring.belly, share),
        back: times(ring.back, share),
    }
}

impl Layout {
    /// Its legs this many thousandths as long, the body raised (or lowered) to
    /// stand on them.
    pub fn with_legs(&self, share: i32) -> Layout {
        let mut layout = *self;
        let mut raised = None;
        for limb in layout.limbs.iter_mut().flatten() {
            if !matches!(limb.role, Role::Leg | Role::Foreleg | Role::Hindleg) {
                continue;
            }
            let mut gained = 0;
            for bone in limb.bones.iter_mut().flatten() {
                let span = times(bone.span, share);
                // A leg's bones all go down a metre a metre (`build::leg`).
                gained += span - bone.span;
                bone.span = span;
            }
            raised.get_or_insert(gained);
        }
        layout.spine.base[1] += raised.unwrap_or(0);
        layout
    }

    /// Its trunk this many thousandths as wide and deep, and its legs as thick.
    pub fn with_girth(&self, share: i32) -> Layout {
        let mut layout = *self;
        let spine = &mut layout.spine;
        for station in [&mut spine.hips, &mut spine.waist, &mut spine.chest] {
            station.half = (times(station.half.0, share), times(station.half.1, share));
        }
        for ring in &mut spine.rings {
            *ring = Ring {
                along: ring.along,
                ..ring_times(*ring, share)
            };
        }
        for limb in layout.limbs.iter_mut().flatten() {
            limb.apart = times(limb.apart, share);
            for bone in limb.bones.iter_mut().flatten() {
                bone.radius = times(bone.radius, share);
                bone.end_radius = times(bone.end_radius, share);
            }
        }
        layout.neck.radius = times(layout.neck.radius, share);
        layout
    }

    /// Its trunk thickened by this many millimetres all round: a downy chick's
    /// or a woolly calf's coat.
    pub fn with_fluff(&self, mm: i32) -> Layout {
        let mut layout = *self;
        for ring in &mut layout.spine.rings {
            ring.half += mm;
            ring.belly += mm;
            ring.back += mm;
        }
        layout
    }

    /// Its back, from the hips to the neck, this many thousandths as long.
    pub fn with_back(&self, share: i32) -> Layout {
        let mut layout = *self;
        let spine = &mut layout.spine;
        for station in [&mut spine.hips, &mut spine.waist, &mut spine.chest] {
            station.along = times(station.along, share);
        }
        spine.shoulders = times(spine.shoulders, share);
        spine.neck_base = times(spine.neck_base, share);
        for ring in &mut spine.rings {
            ring.along = times(ring.along, share);
        }
        for limb in layout.limbs.iter_mut().flatten() {
            if let Girdle::Along(along) = limb.girdle {
                limb.girdle = Girdle::Along(times(along, share));
            }
        }
        // A fourfoot's spine starts behind the middle: the middle stays put.
        let back = self.spine.neck_base - layout.spine.neck_base;
        if self.spine.pitch == 90 {
            layout.spine.base[2] += back / 2;
        }
        layout
    }

    /// Its neck this many thousandths as long.
    pub fn with_neck(&self, share: i32) -> Layout {
        let mut layout = *self;
        layout.neck.length = times(layout.neck.length, share);
        layout
    }

    /// Its head this many thousandths of its size.
    pub fn with_head(&self, share: i32) -> Layout {
        let mut layout = *self;
        let head = &mut layout.head;
        head.length = times(head.length, share);
        head.half = (times(head.half.0, share), times(head.half.1, share));
        if let Some(shape) = &mut head.shape {
            shape.rings = shape.rings.map(|ring| ring_times(ring, share));
        }
        layout
    }
}

#[cfg(test)]
mod tests;
