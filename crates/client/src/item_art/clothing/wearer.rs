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
use super::worn_head::WornHead;
use crate::item_art::Look;
use crate::item_art::body::{tail_path, trunk_half};

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
    /// How far clothes on the torso stand out in front of the chest, at most: what
    /// drapes over them (capes, shawls, scarves' ends) lies outside it.
    pub chest_out: Option<f32>,
    /// A shawl is worn: a hood's cape goes over it.
    pub shawl: bool,
    /// The head, for what is worn on it.
    pub head: WornHead,
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
        let (mut chest_out, mut shawl) = (None, false);
        let frame = layout.worn_frame().map(|(frame, _)| frame);
        for look in worn {
            match look {
                Look::Clothing { clothing, .. } => {
                    let form = &clothing.form;
                    match (&form.garment, &frame) {
                        (Garment::Belt(worn), _) => belt = Some(super::belt::height(worn)),
                        (Garment::Skirt(_), Some(frame)) => {
                            skirt_hem = lowest(skirt_hem, hem_of(form, frame).0)
                        }
                        (Garment::Shawl(_), _) => shawl = true,
                        (garment, _) if !on_trunk(garment) => {}
                        (_, Some(frame)) => {
                            let (hem, out) = hem_of(form, frame);
                            torso_hem = lowest(torso_hem, hem);
                            chest_out = Some(chest_out.map_or(out, |have: f32| have.max(out)));
                        }
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
            chest_out,
            shawl,
            head: WornHead::of(layout, worn),
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

/// Whether a piece is worn on the trunk, hanging from the shoulders or the waist: not
/// on the head, the neck, the hands or the legs.
fn on_trunk(garment: &Garment) -> bool {
    matches!(
        garment,
        Garment::Tunic(_)
            | Garment::Robe(_)
            | Garment::Doublet(_)
            | Garment::Bodice(_)
            | Garment::Vest(_)
    )
}

/// How low a piece hangs at the body's middle, front or back (not its sleeves), and
/// how far out in front of the chest it stands.
fn hem_of(form: &shared::clothing::ClothingForm, frame: &Frame) -> (f32, f32) {
    let model = super::build(form, frame, Fit::Alone);
    let middle = frame.hips_half.0 as f32 * 0.5;
    let corners = || {
        model
            .parts
            .iter()
            .flat_map(|(_, triangles)| triangles.iter().flatten())
            .filter(|corner| corner.x.abs() < middle)
    };
    let hem = corners().map(|corner| corner.y).fold(f32::MAX, f32::min);
    let chest = frame.chest as f32;
    let out = corners()
        .filter(|corner| corner.z > 0.0 && (corner.y - chest).abs() < 40.0)
        .map(|corner| corner.z - trunk_half(frame, corner.y).deep(true))
        .fold(0.0, f32::max);
    (hem, out)
}
