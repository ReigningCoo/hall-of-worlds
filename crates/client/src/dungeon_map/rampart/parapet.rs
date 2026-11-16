//! A parapet: a low wall along the outer edge of a wall's top, set out a
//! little past its face, with merlons standing on it and crenels between.
//! Heights are from the walk on top, where a walker's feet are: the crenels
//! at waist height, the merlons near head height. Kept apart from walls round
//! sites so the tops of towers can have them too.

use bevy::prelude::*;

use crate::dungeon_map::CELL_SIZE;

/// A metre: a cell is about a metre and a half across.
pub(super) const METRE: f32 = CELL_SIZE / 1.5;

/// How high the parapet stands between merlons (its crenels), and its
/// merlons, above the walk.
pub(super) const CRENEL_HEIGHT: f32 = METRE;
pub(super) const MERLON_HEIGHT: f32 = 1.5 * METRE;

/// How far the parapet stands out past the wall's face, and how thick it is.
pub(super) const OVERHANG: f32 = CELL_SIZE * 0.05;
const THICKNESS: f32 = CELL_SIZE * 0.12;

/// How far below the walk the parapet starts: a band round the wall's top,
/// so the part standing out past the face doesn't float.
const LIP: f32 = CELL_SIZE * 0.08;

/// Merlons: two to a cell, centred a quarter of a cell either side of each
/// cell's middle (so they line up from cell to cell), each half this long.
const MERLON_SPACING: f32 = CELL_SIZE / 2.0;
const MERLON_FIRST: f32 = CELL_SIZE / 4.0;
const MERLON_HALF: f32 = CELL_SIZE * 0.13;

/// One straight stretch of parapet, in one cell.
pub(super) struct Parapet {
    /// The middle of the cell at the height of the walk.
    pub foot: Vec3,
    /// The way out from the wall (one of the four sides, flat).
    pub outward: Vec3,
    /// How far the wall's face is from the cell's middle that way.
    pub face: f32,
    /// How far the parapet runs from the cell's middle either way along the
    /// wall: towards west or north (as a negative number), and east or south.
    pub span: (f32, f32),
}

impl Parapet {
    /// The way along the wall, east or south.
    fn axis(&self) -> Vec3 {
        Vec3::new(self.outward.z.abs(), 0.0, self.outward.x.abs())
    }

    /// Its boxes, each as its smallest and largest corners: the low run, then
    /// the merlons on it.
    pub fn boxes(&self) -> Vec<(Vec3, Vec3)> {
        let axis = self.axis();
        let outer = self.face + OVERHANG;
        let inner = outer - THICKNESS;
        let piece = |(a, b): (f32, f32), (low, high): (f32, f32)| {
            let p = self.foot + self.outward * inner + axis * a + Vec3::Y * low;
            let q = self.foot + self.outward * outer + axis * b + Vec3::Y * high;
            (p.min(q), p.max(q))
        };
        let mut boxes = vec![piece(self.span, (-LIP, CRENEL_HEIGHT))];
        // Where the merlons go is worked out along the whole map, so they keep
        // their rhythm from one cell to the next.
        let start = self.foot.dot(axis);
        let (from, to) = (start + self.span.0, start + self.span.1);
        let first = ((from - MERLON_FIRST) / MERLON_SPACING).floor() as i64;
        let last = ((to - MERLON_FIRST) / MERLON_SPACING).ceil() as i64;
        for m in first..=last {
            let middle = MERLON_FIRST + m as f32 * MERLON_SPACING;
            let a = (middle - MERLON_HALF).max(from);
            let b = (middle + MERLON_HALF).min(to);
            // Only merlons at least half there: none cut to a sliver at an end.
            if b - a >= MERLON_HALF {
                boxes.push(piece(
                    (a - start, b - start),
                    (CRENEL_HEIGHT, MERLON_HEIGHT),
                ));
            }
        }
        boxes
    }
}
