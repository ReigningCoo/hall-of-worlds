//! Who clothes are worn by, as far as their shape goes: where a tail comes out, whether
//! the arms are wings, whether it is a taur's upper body, and which armour is worn over
//! them. There is no rule for tails and wings: the model cuts the opening for whoever
//! wears it (decision 14 of the clothing research). It also knows what other clothes
//! are worn with a piece: a belt cinching it, and how low long clothes hang over
//! trousers and skirts, which are hidden under them.

use bevy::prelude::*;
use shared::body::{Feet, Frame, Layout, Role};
use shared::clothing::Garment;

use super::Fit;
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
    /// Hooves or talons: trouser legs end in a cuff or a tie above them.
    pub hooves: bool,
    pub talons: bool,
    /// A worn belt, how tall it is: it cinches what is under it at the waist.
    pub belt: Option<f32>,
    /// The lowest hem of clothes on the torso, and of a skirt, at the body's middle:
    /// what is under them is hidden above it.
    pub torso_hem: Option<f32>,
    pub skirt_hem: Option<f32>,
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
        let mut belt = None;
        let (mut torso_hem, mut skirt_hem) = (None, None);
        let frame = layout.worn_frame().map(|(frame, _)| frame);
        for look in worn {
            match look {
                Look::Clothing { clothing, .. } => {
                    let form = &clothing.form;
                    match (&form.garment, &frame) {
                        (Garment::Belt(worn), _) => belt = Some(super::belt::height(worn)),
                        (Garment::Skirt(_), Some(frame)) => {
                            skirt_hem = lowest(skirt_hem, hem_of(form, frame))
                        }
                        (Garment::Trousers(_) | Garment::Gloves(_), _) => {}
                        (_, Some(frame)) => torso_hem = lowest(torso_hem, hem_of(form, frame)),
                        _ => {}
                    }
                }
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
            hooves: layout.parts.feet == Feet::Hooves,
            talons: layout.parts.feet == Feet::Talons,
            belt,
            torso_hem,
            skirt_hem,
        }
    }

    /// How low what is worn over trousers hangs: they are hidden above it.
    pub(super) fn over_trousers(&self) -> Option<f32> {
        match (self.torso_hem, self.skirt_hem) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }
}

fn lowest(have: Option<f32>, new: f32) -> Option<f32> {
    Some(have.map_or(new, |have| have.min(new)))
}

/// How low a piece hangs at the body's middle, front or back (not its sleeves).
fn hem_of(form: &shared::clothing::ClothingForm, frame: &Frame) -> f32 {
    let model = super::build(form, frame, Fit::Alone);
    let middle = frame.hips_half.0 as f32 * 0.5;
    model
        .parts
        .iter()
        .flat_map(|(_, triangles)| triangles.iter().flatten())
        .filter(|corner| corner.x.abs() < middle)
        .map(|corner| corner.y)
        .fold(f32::MAX, f32::min)
}
