//! Clothes on the Middle layer: tunics, robes, doublets, bodices and vests, trousers,
//! skirts, belts and gloves so far.
//! Each is built in millimetres in place on a body frame, so its picture and a dressed
//! character use the same parts. Alone it is drawn as loose as it is cut; worn
//! (`Fit::Worn`) it lies over the underclothes and inside any armour over it, with
//! openings for whoever wears it. Clothes for the head and neck aren't drawn yet.

mod belt;
mod belt_ends;
mod bodice;
mod colour;
mod doublet;
mod gloves;
mod legs;
mod neck;
mod pattern;
mod robe;
mod shape;
mod skirt;
mod sleeves;
mod strips;
mod trousers;
mod tunic;
mod vest;
mod wearer;

#[cfg(test)]
mod tests;

pub(in crate::item_art) use self::colour::material_of;
pub(in crate::item_art) use self::shape::Fit;
pub(in crate::item_art) use self::wearer::Wearer;
use bevy::prelude::*;
use shared::body::Frame;
use shared::clothing::{BeltForm, ClothingForm, ClothingPiece, Garment, RobeDecoration, VestMake};
use shared::creature::CreatureKind;
use shared::item::Quality;

use self::shape::Wear;
use super::armour::{Model, hands};
use crate::item_art::body::mesh_of;
use crate::item_art::{CAMERA_AT, CAMERA_UP, OUTLINE_PIXELS, Part, VIEW_WIDTH};

/// How many of a picture's pixels are one cell, for the margin's sake.
const PIXELS_PER_CELL: f32 = 64.0;
/// Pixels kept clear round it, for its black edge.
const MARGIN_PIXELS: f32 = OUTLINE_PIXELS + 2.0;
/// Turned a little to its right and seen a little from above.
const TURN: f32 = 0.45;
const TILT: f32 = 0.12;
const BELT_TILT: f32 = 0.7;
/// How thick cloth, and leather or padding, are drawn.
const CLOTH: f32 = 1.3;
const HEAVY: f32 = 2.4;

/// Whether pieces of this kind are drawn yet: those worn on the torso, the legs, the
/// waist and the hands.
pub(in crate::item_art) fn drawn(piece: ClothingPiece) -> bool {
    matches!(
        piece,
        ClothingPiece::Tunic
            | ClothingPiece::Robe
            | ClothingPiece::Doublet
            | ClothingPiece::Bodice
            | ClothingPiece::Vest
            | ClothingPiece::Trousers
            | ClothingPiece::Skirt
            | ClothingPiece::Belt
            | ClothingPiece::Gloves
    )
}

/// What can be seen of a piece of clothing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClothingLook {
    pub(in crate::item_art) form: ClothingForm,
    /// What it takes up, wide and long, not turned.
    pub(in crate::item_art) cells: (u8, u8),
    /// Fingers on each hand, for gloves.
    pub(in crate::item_art) digits: u8,
}

impl ClothingLook {
    /// A piece taking up these cells, gloves made for five fingers.
    pub fn new(form: ClothingForm, cells: (u8, u8)) -> Self {
        Self {
            form,
            cells,
            digits: 5,
        }
    }

    /// The same, gloves made for this many fingers on each hand.
    pub fn with_digits(self, digits: u8) -> Self {
        Self { digits, ..self }
    }
}

/// How thick a piece's cloth is drawn.
fn thickness(form: &ClothingForm) -> f32 {
    let heavy = match form.garment {
        Garment::Vest(vest) => vest.make != VestMake::Cloth,
        Garment::Doublet(doublet) => {
            doublet.arming() || form.material.category() == shared::material::Category::Leather
        }
        Garment::Belt(belt) => matches!(belt.form, BeltForm::Strap | BeltForm::Plates),
        Garment::Trousers(_) => form.material.category() == shared::material::Category::Leather,
        _ => false,
    };
    if heavy { HEAVY } else { CLOTH }
}

/// The piece on this frame, as cut or worn (gloves for five fingers).
pub(in crate::item_art) fn build(form: &ClothingForm, frame: &Frame, fit: Fit) -> Model {
    build_for(form, frame, fit, 5)
}

/// The same, gloves made for `digits` fingers on each hand.
pub(in crate::item_art) fn build_for(
    form: &ClothingForm,
    frame: &Frame,
    fit: Fit,
    digits: u8,
) -> Model {
    let mut model = Model::default();
    // A belt doesn't cinch itself.
    let fit = match (fit, &form.garment) {
        (Fit::Worn(wearer), Garment::Belt(_)) => Fit::Worn(Wearer {
            belt: None,
            ..wearer
        }),
        (fit, _) => fit,
    };
    let wear = Wear {
        frame,
        fit,
        thick: thickness(form),
    };
    let mut all_over = false;
    match &form.garment {
        Garment::Tunic(tunic) => wear.tunic(&mut model, tunic),
        Garment::Robe(robe) => {
            all_over = robe.decoration == RobeDecoration::AllOver;
            wear.robe(&mut model, robe);
        }
        Garment::Doublet(doublet) => wear.doublet(&mut model, doublet),
        Garment::Bodice(bodice) => wear.bodice(&mut model, bodice),
        Garment::Vest(vest) => wear.vest(&mut model, vest),
        Garment::Trousers(trousers) => wear.trousers(&mut model, trousers),
        Garment::Skirt(skirt) => wear.skirt(&mut model, skirt),
        Garment::Belt(belt) => wear.belt(&mut model, belt),
        Garment::Gloves(gloves) => wear.gloves(&mut model, gloves, digits),
        // Drawn by later changes.
        _ => {}
    }
    pattern::apply(&mut model, form, all_over);
    model
}

/// The piece built on an ordinary body for its picture, turned and tipped back, then
/// fitted to its cells.
fn fitted(look: &ClothingLook) -> Model {
    let frame = Frame::of(CreatureKind::Human).expect("people have a frame");
    let mut picture = match &look.form.garment {
        // Laid side by side, as gauntlets are.
        Garment::Gloves(gloves) => {
            let build = |side| {
                let mut model = Model::default();
                let wear = Wear {
                    frame: &frame,
                    fit: Fit::Alone,
                    thick: thickness(&look.form),
                };
                wear.glove(&mut model, gloves, side, look.digits);
                pattern::apply(&mut model, &look.form, false);
                model
            };
            hands::lay_pair(&frame, look.digits, &build)
        }
        // A strap or cord lies folded down its picture.
        Garment::Belt(belt) if !matches!(belt.form, BeltForm::Sash | BeltForm::Obi) => {
            build(&look.form, &frame, Fit::Alone)
        }
        garment => {
            let mut picture = build(&look.form, &frame, Fit::Alone);
            // A sash or obi is seen from higher up, so its round shows.
            let tilt = match garment {
                Garment::Belt(_) => BELT_TILT,
                _ => TILT,
            };
            let view = Quat::from_rotation_x(tilt) * Quat::from_rotation_y(TURN);
            picture.map(|point| view * point);
            picture
        }
    };
    let (low, high) = picture.bounds();
    let (wide, long) = (look.cells.0 as f32, look.cells.1 as f32);
    let world_per_pixel = VIEW_WIDTH / (wide.max(long) * PIXELS_PER_CELL);
    let room = |cells: f32| (cells * PIXELS_PER_CELL - 2.0 * MARGIN_PIXELS) * world_per_pixel;
    let size = high - low;
    let scale = (room(long) / size.y).min(room(wide) / size.x);
    let middle = (low + high) / 2.0;
    picture.map(|point| (point - middle) * scale);
    picture
}

/// A piece of clothing filling its picture.
pub(in crate::item_art) fn clothing_parts(
    look: &ClothingLook,
    quality: Option<Quality>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let camera = Transform::from_translation(CAMERA_AT).looking_at(Vec3::ZERO, CAMERA_UP);
    fitted(look)
        .parts
        .into_iter()
        .filter(|(_, triangles)| !triangles.is_empty())
        .map(|(stuff, triangles)| Part {
            mesh: meshes.add(mesh_of(&triangles)),
            material: materials.add(colour::material_of(stuff, &look.form, quality)),
            transform: Transform::from_rotation(camera.rotation),
        })
        .collect()
}
