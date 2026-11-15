//! Who clothes are worn by, as far as their shape goes: where a tail comes out, whether
//! the arms are wings, whether it is a taur's upper body, and which armour is worn over
//! them. There is no rule for tails and wings: the model cuts the opening for whoever
//! wears it (decision 14 of the clothing research).

use bevy::prelude::*;
use shared::body::{Layout, Role};

use crate::item_art::Look;
use crate::item_art::body::tail_path;

/// Where armour worn over clothes covers them, so the cloth keeps inside it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(in crate::item_art) struct Covered {
    pub torso: bool,
    pub arms: bool,
    /// Below the waist: waist or leg armour.
    pub skirt: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(in crate::item_art) struct Wearer {
    /// Where the tail leaves the body, on the frame things are worn on, and how thick
    /// it is there.
    pub tail: Option<(Vec3, f32)>,
    /// Wing-arms take no sleeves.
    pub wings: bool,
    /// A taur's upper body: nothing hangs below its hips.
    pub taur: bool,
    pub covered: Covered,
}

impl Wearer {
    /// A body of this layout wearing these (pieces' own looks).
    pub(in crate::item_art) fn of(layout: &Layout, worn: &[Look]) -> Self {
        let taur = layout.upper.is_some();
        let tail = (!taur && layout.parts.tail > 0).then(|| {
            let (root, radius) = tail_path(layout, layout.parts.tail as f32)[0];
            (root, radius)
        });
        let mut covered = Covered::default();
        for look in worn {
            match look {
                Look::Torso { .. } => covered.torso = true,
                Look::Arms { .. } => covered.arms = true,
                Look::Waist { .. } | Look::Legs { .. } => covered.skirt = true,
                Look::Set { .. } => {
                    covered = Covered {
                        torso: true,
                        arms: true,
                        skirt: true,
                    }
                }
                _ => {}
            }
        }
        Self {
            tail,
            wings: layout.limbs().any(|limb| limb.role == Role::WingArm),
            taur,
            covered,
        }
    }
}
