//! Turbans: wraps of cloth wound round the head from the brow up, crossing at the
//! front, each a roll on a tipped ring, with the top filled; more and thicker wraps
//! the more cloth there is. Under a helmet only a close wrap fits.

use bevy::prelude::*;
use shared::clothing::{Headcloth, TurbanSize};

use super::lofted::{oval, roll, solid};
use super::shape::Wear;
use super::skull::OFF_HEAD;
use crate::item_art::armour::{Model, Stuff};

/// Where the lowest wrap goes round: about the brow.
const BROW: f32 = 0.42;

/// How many wraps a turban of this size has, and how thick each is.
pub(super) fn wraps(size: TurbanSize) -> (usize, f32) {
    match size {
        TurbanSize::Small => (3, 11.0),
        TurbanSize::Full => (5, 15.0),
        TurbanSize::Great => (6, 22.0),
    }
}

impl Wear<'_> {
    pub fn turban(&self, model: &mut Model, cloth: &Headcloth) {
        let off = OFF_HEAD;
        // Over the crown first, under the wraps.
        self.over_head(model, Stuff::Main, (0.0, BROW), &|_, _| off, &|_| 0.0);
        if self.helmeted() {
            return;
        }
        let (count, tube) = wraps(cloth.turban.unwrap_or(TurbanSize::Full));
        let base = self.head_y(BROW);
        let half = self.head_half(base, off + tube - 4.0);
        let crown = self.head_y(0.0) + off;
        let mut top = base;
        for wrap in 0..count {
            let share = wrap as f32 / count.max(2) as f32;
            let y = base + tube * 0.9 + wrap as f32 * tube * 1.35;
            // Narrowing towards the top as the head does, and swelling outward
            // as the wraps pile up.
            let scale = 1.0 + share * 0.25 - (share * share) * 0.45;
            // Tipped either way in turn, so they cross over the brow.
            let tip = if wrap % 2 == 0 { 0.22 } else { -0.22 };
            let centre = Vec3::new(0.0, y, -6.0 * share);
            let ring = oval(centre, (half.0 * scale, half.1 * scale), tip);
            roll(model, Stuff::Main, &ring, tube);
            top = top.max(y);
        }
        // The top filled over the crown, its underside clear of it.
        let lid = top.max(crown - tube).max(crown + 4.0 + tube * 0.5);
        let rings = [
            oval(
                Vec3::Y * (lid - tube * 0.5),
                (half.0 * 0.85, half.1 * 0.85),
                0.0,
            ),
            oval(
                Vec3::Y * (lid + tube * 0.6),
                (half.0 * 0.6, half.1 * 0.6),
                0.0,
            ),
            oval(
                Vec3::Y * (lid + tube * 0.9),
                (half.0 * 0.2, half.1 * 0.2),
                0.0,
            ),
        ];
        solid(model, Stuff::Main, &rings);
    }
}

/// How far out a turban reaches at its brow wrap, off the head.
pub(super) fn turban_reach(size: TurbanSize) -> f32 {
    let (count, tube) = wraps(size);
    OFF_HEAD + tube * 2.0 + count as f32 * 4.0
}
