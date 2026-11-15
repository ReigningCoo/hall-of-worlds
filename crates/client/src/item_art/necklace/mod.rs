//! Necklaces: the strand (a cord, a chain or a torc) as one tube swept along a line
//! round the neck, its links faked by shading its faces, and what hangs from it in the
//! strand's metal (a pendant, an amulet's medallion, stations, a charm), with gems
//! placed as on rings. Built in millimetres in place on a body frame, so its picture
//! and a dressed character use the same parts. Worn, it lies flat on the skin
//! (`Wear::Worn`), so whatever is worn over it hides it.

mod charm;
mod colour;
mod ornament;
mod path;
mod shapes;
mod strand;

#[cfg(test)]
mod tests;

use bevy::prelude::*;
use shared::body::Frame;
use shared::creature::{CreatureKind, Size};
use shared::item::Quality;
use shared::item_form::{NecklaceForm, NecklaceLength, Ornament, Strand};
use shared::material::Material;

use super::armour::Model;
use super::armour::set::Coloured;
use super::body::mesh_of;
use super::gem::{gem_colour, gem_parts};
use super::look::{SettingLook, setting_looks};
use super::{CAMERA_AT, CAMERA_UP, OUTLINE_PIXELS, Part, VIEW_WIDTH};

pub(in crate::item_art) use self::ornament::GemAt;

/// How many of a picture's pixels are one cell, for the margin's sake.
const PIXELS_PER_CELL: f32 = 64.0;
/// Pixels kept clear round it, for its black edge.
const MARGIN_PIXELS: f32 = OUTLINE_PIXELS + 2.0;
/// Seen from the front and above, so the loop round the neck shows as a loop.
const TILT: f32 = 0.3;
const TURN: f32 = 0.0;

/// What can be seen of a necklace. Setting sizes are against the necklace's own.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NecklaceLook {
    pub(in crate::item_art) metal: Material,
    pub(in crate::item_art) strand: Strand,
    pub(in crate::item_art) length: NecklaceLength,
    pub(in crate::item_art) ornament: Ornament,
    pub(in crate::item_art) settings: Vec<SettingLook>,
}

impl NecklaceLook {
    /// A necklace of this form, made in this size (what its settings are against).
    pub fn new(form: &NecklaceForm, size: Size) -> Self {
        Self {
            metal: form.metal,
            strand: form.strand,
            length: form.length,
            ornament: form.ornament,
            settings: setting_looks(&form.settings, size),
        }
    }
}

/// How it is built: as made, for its picture, or worn flat on the skin, wider than
/// it is so it shows at a token's size, never more than `WORN_TOP` off the skin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::item_art) enum Wear {
    Alone,
    Worn,
}

/// The most anything worn stands off the skin: inside underclothes' 2.5 mm.
pub(in crate::item_art) const WORN_TOP: f32 = 2.0;

/// The necklace on this frame: its metal and cord in `Stuff`s, and its gems apart.
pub(in crate::item_art) fn build(
    look: &NecklaceLook,
    frame: &Frame,
    wear: Wear,
) -> (Model, Vec<GemAt>) {
    let path = path::Path::new(look, frame, wear);
    let mut model = Model::default();
    strand::sweep(&mut model, look, &path);
    let gems = ornament::hang(&mut model, look, &path);
    (model, gems)
}

/// Worn on a character: every part in its own colours, gems as flat stones.
pub(in crate::item_art) fn worn(
    look: &NecklaceLook,
    frame: &Frame,
    quality: Option<Quality>,
) -> Coloured {
    let (model, gems) = build(look, frame, Wear::Worn);
    let mut all: Coloured = model
        .parts
        .into_iter()
        .map(|(stuff, triangles)| (triangles, colour::material_of(stuff, look, quality)))
        .collect();
    for gem in gems {
        let colour = gem_colour(gem.gem.material);
        let stone = StandardMaterial {
            base_color: colour,
            perceptual_roughness: 0.25,
            reflectance: 0.8,
            ..default()
        };
        all.push((gem.stone, stone));
    }
    all
}

/// The necklace as made, on an ordinary person's neck (not drawn), seen from the
/// front and above, fitted to its picture; gems drawn as rings' are.
pub(in crate::item_art) fn necklace_parts(
    look: &NecklaceLook,
    quality: Option<Quality>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let frame = Frame::of(CreatureKind::Human).expect("people have a frame");
    let (mut model, mut gems) = build(look, &frame, Wear::Alone);
    let view = Quat::from_rotation_x(TILT) * Quat::from_rotation_y(TURN);
    model.map(|point| view * point);
    for gem in &mut gems {
        gem.turn(view);
    }
    let (low, high) = model.bounds();
    let world_per_pixel = VIEW_WIDTH / PIXELS_PER_CELL;
    let room = (PIXELS_PER_CELL - 2.0 * MARGIN_PIXELS) * world_per_pixel;
    let size = high - low;
    let scale = room / size.x.max(size.y);
    let middle = (low + high) / 2.0;
    let fit = |point: Vec3| (point - middle) * scale;
    model.map(fit);
    let camera = Transform::from_translation(CAMERA_AT).looking_at(Vec3::ZERO, CAMERA_UP);
    let facing = Transform::from_rotation(camera.rotation);
    let mut parts: Vec<Part> = model
        .parts
        .into_iter()
        .filter(|(_, triangles)| !triangles.is_empty())
        .map(|(stuff, triangles)| Part {
            mesh: meshes.add(mesh_of(&triangles)),
            material: materials.add(colour::material_of(stuff, look, quality)),
            transform: facing,
        })
        .collect();
    for gem in gems {
        let transform = facing
            * Transform::from_translation(fit(gem.at))
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, gem.out))
                .with_scale(Vec3::splat(gem.width * scale));
        for (mesh, material) in gem_parts(&gem.gem, gem.quality, meshes, materials) {
            parts.push(Part {
                mesh,
                material,
                transform,
            });
        }
    }
    parts
}
