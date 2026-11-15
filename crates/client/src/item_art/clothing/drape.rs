//! Cloth hanging from round the neck or head over the shoulders: hoods' capes, veils,
//! shawls, capelets and falling bands. Each falls from a ring at its top to heights in
//! front and behind, and at the sides onto the shoulders or down over the arms, kept
//! clear of the body, outside the clothes on the trunk and inside torso armour.

use shared::armour::neck::Hem;

use super::shape::{OVER, UNDER_ARMOUR, Wear};
use crate::item_art::armour::neck::{Curtain, Profile, dir, sideness};
use crate::item_art::armour::{Model, Stuff};
use bevy::prelude::*;

/// Where a drape falls to.
#[derive(Clone, Copy)]
pub(super) struct Fall {
    /// Heights in front and behind.
    pub front: f32,
    pub back: f32,
    /// At the sides: down the outside of the arm to this height, or (`None`) onto the
    /// top of the shoulder.
    pub side: Option<f32>,
}

impl Wear<'_> {
    /// How far a drape stands off the body: outside the clothes on the trunk, and
    /// inside torso armour worn over it.
    pub fn drape_off(&self) -> f32 {
        let Some(wearer) = self.wearer() else {
            return 10.0;
        };
        let out = wearer.chest_out.map_or(OVER, |out| out.max(OVER)) + 5.0;
        if wearer.covered.torso {
            out.min(UNDER_ARMOUR)
        } else {
            out
        }
    }

    /// The profile of a drape at an angle (from the front towards the left), from
    /// `top`, `off` the body.
    fn fall_at(&self, fall: &Fall, angle: f32, top: Vec3, off: f32) -> Profile {
        let around = self.around();
        let behind = (1.0 - angle.cos()) / 2.0;
        let height = Self::between(fall.front, fall.back, behind);
        let front = around.trunk(angle, height, off);
        let (side, crest) = match fall.side {
            Some(low) => {
                let out = around.shoulder_half() + around.upper_arm() + off + 6.0;
                let bottom = dir(angle) * out + Vec3::Y * low;
                let crest = around.over_shoulder(angle, around.shoulder_half(), off + 10.0);
                (bottom, crest)
            }
            None => {
                let bottom = around.over_shoulder(angle, around.base_across() + 40.0, off);
                (bottom, (top + bottom) / 2.0)
            }
        };
        let s = sideness(angle);
        let bottom = front.lerp(side, s);
        Profile {
            top,
            middle: ((top + bottom) / 2.0).lerp(crest, s),
            bottom,
        }
    }

    /// A drape from angle `from` to `to` (right round is -π to π), hanging from
    /// `top(angle)`, its hem cut `depth` deep.
    #[allow(clippy::too_many_arguments)]
    pub fn drape(
        &self,
        model: &mut Model,
        stuff: Stuff,
        (from, to): (f32, f32),
        top: &dyn Fn(f32) -> Vec3,
        fall: &Fall,
        (hem, depth): (Hem, f32),
        off: f32,
    ) {
        let around = self.around();
        // Under torso armour only what is above it shows.
        let armoured = self.wearer().is_some_and(|wearer| wearer.covered.torso);
        let fall = &if armoured {
            let least = self.shoulder() - 40.0;
            Fall {
                front: fall.front.max(least),
                back: fall.back.max(least),
                ..*fall
            }
        } else {
            *fall
        };
        let profile = |angle: f32| self.fall_at(fall, angle, top(angle), off);
        let curtain = Curtain {
            from,
            to,
            profile: &profile,
            hem,
            depth,
            clear: (&around, off),
        };
        curtain.sheet(model, stuff, 0.0, self.thick);
    }

    /// A point round the neck's base, `off` out from the neck, a little above the
    /// trunk's top: where drapes round the neck hang from.
    pub fn neck_top(&self, angle: f32, off: f32) -> Vec3 {
        let radius = self.neck() + off;
        dir(angle) * radius + Vec3::Y * (self.neck_base() + 12.0)
    }
}
