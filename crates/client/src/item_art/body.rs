//! A plain body to dress, built from its frame in millimetres: everything worn is made in
//! place on the same frame, so it is drawn on this. Smooth shapes, a little turned and
//! seen slightly from above, as a token shows a character.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;
use shared::body::{Hair, Layout, Role};

use super::colours::{Colours, FLESH, material};
use super::{CAMERA_AT, CAMERA_UP, OUTLINE_PIXELS, Part, VIEW_WIDTH};

mod antlers;
mod arms;
mod coat_hair;
mod coat_shape;
mod extras;
mod fan;
mod features;
mod fit;
mod flesh;
mod fringe;
mod hair;
mod head;
mod horns;
mod humps;
mod legs;
mod marks;
mod morph;
mod mouth;
mod shapes;
mod sickle;
mod trunk;

pub(super) use coat_hair::coat_hair;
pub(super) use features::{tail_path, wings};
use fit::fitted;
pub(super) use fit::turn_of;
pub(super) use flesh::flesh;
pub(super) use hair::{Cover, hair};
pub(super) use head::{HeadAxes, crown};
pub(super) use horns::horns_of;
pub(super) use marks::split as split_marks;
#[cfg(test)]
pub(super) use morph::placed as morph_placed;
pub(super) use morph::{morph_parts, pieces as bare_pieces};
pub(super) use shapes::{blob, limb, loft};
#[cfg(test)]
pub(super) use trunk::Girth;
pub(super) use trunk::{neck_root, trunk_depth, trunk_half};

/// What an upright body's picture takes up, wide and long: only the proportions
/// matter. An animal's is square, as its token is.
pub(super) const CELLS: (u8, u8) = (1, 2);
const BEAST_CELLS: (u8, u8) = (2, 2);

/// What a body's picture takes up.
pub(super) fn cells(layout: &Layout) -> (u8, u8) {
    if layout.frame().is_some() {
        CELLS
    } else {
        BEAST_CELLS
    }
}
/// How many of a picture's pixels are one cell, for the margin's sake.
const PIXELS_PER_CELL: f32 = 64.0;
/// Pixels kept clear round it, for its black edge.
const MARGIN_PIXELS: f32 = OUTLINE_PIXELS + 2.0;
/// Turned a little to its right, so its sides show.
const TURN: f32 = 0.35;
/// Seen a little from above, about as a token is.
const TILT: f32 = 0.25;
/// Points round anything round, and rings along a head.
pub(super) const ROUND: usize = 12;
const HEAD_RINGS: usize = 8;

pub(super) type Triangle = [Vec3; 3];

fn point(mm: [i32; 3]) -> Vec3 {
    Vec3::new(mm[0] as f32, mm[1] as f32, mm[2] as f32)
}

/// The body, in millimetres, standing as its layout says, with whatever it has beyond
/// a person's.
pub(super) fn build(layout: &Layout) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    trunk_and_coat(&mut triangles, layout);
    neck_and_head(&mut triangles, layout);
    limbs(&mut triangles, layout);
    features::rear(&mut triangles, layout);
    triangles
}

/// The trunk, with any fleece round it and humps on its back.
fn trunk_and_coat(triangles: &mut Vec<Triangle>, layout: &Layout) {
    trunk::build(triangles, layout);
    coat_shape::fleece(triangles, layout);
    humps::build(triangles, layout);
}

/// Each limb, by its role.
fn limbs(triangles: &mut Vec<Triangle>, layout: &Layout) {
    for limb in layout.limbs() {
        match limb.role {
            Role::Arm | Role::WingArm => arms::build(triangles, layout, limb),
            Role::Leg | Role::Foreleg | Role::Hindleg | Role::JointedLeg => {
                legs::build(triangles, layout, limb, layout.parts.feet)
            }
        }
    }
}

/// The neck from the top of the spine, and the head on it with its snout, ears, horns
/// and tusks.
fn neck_and_head(triangles: &mut Vec<Triangle>, layout: &Layout) {
    // A measured head: an animal's on a neck carrying on from its trunk, a beast
    // people's on a person's neck, which neck armour is made to fit.
    if let Some(shape) = layout.head.shape {
        if layout.frame().is_some() {
            neck(triangles, layout);
        } else {
            head::neck(triangles, layout);
        }
        head::build(triangles, layout, &shape);
        extras::build(triangles, layout);
        return;
    }
    neck(triangles, layout);
    let head = layout.head;
    blob(
        triangles,
        features::skull(layout),
        Vec3::new(
            head.half.0 as f32,
            head.length as f32 / 2.0,
            head.half.1 as f32,
        ),
    );
    features::head(triangles, layout, &layout.parts);
}

/// A person's neck: round, from a little into the trunk to a little into the head.
fn neck(triangles: &mut Vec<Triangle>, layout: &Layout) {
    let axes = trunk::Axes::of(layout);
    let neck_base = trunk::neck_root(layout);
    let chin = point(layout.chin());
    let up_neck = (chin - neck_base).normalize_or(Vec3::Y);
    let neck = layout.neck.radius as f32;
    limb(
        triangles,
        &[
            (neck_base - axes.along * 20.0, neck),
            (chin + up_neck * 20.0, neck),
        ],
    );
}

/// The body's skin, and apart from it the coat of a taur's lower body, which may be
/// another colour.
pub(super) fn build_coated(layout: &Layout) -> (Vec<Triangle>, Vec<Triangle>) {
    let Some(upper) = layout.upper_body() else {
        return (build(layout), Vec::new());
    };
    // The lower body: trunk, legs and tail, its neck and head left to the upper body,
    // unless it still has one of its own, as when shapeshifting into or out of a taur.
    let mut coat = Vec::new();
    trunk_and_coat(&mut coat, layout);
    if layout.head.length > 0 {
        neck_and_head(&mut coat, layout);
    }
    limbs(&mut coat, layout);
    features::rear(&mut coat, layout);
    (build(&upper), coat)
}

/// A flat-faced mesh of these triangles, as weapons' are.
pub(super) fn mesh_of(triangles: &[Triangle]) -> Mesh {
    let mut positions = Vec::with_capacity(triangles.len() * 3);
    let mut normals = Vec::with_capacity(triangles.len() * 3);
    for [a, b, c] in triangles {
        let normal = (*b - *a).cross(*c - *a).normalize_or(Vec3::Z).to_array();
        for corner in [a, b, c] {
            positions.push(corner.to_array());
            normals.push(normal);
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
}

/// A bare body standing as this layout says, as tall as `height` beside a person, in
/// these colours, with this hair.
pub(super) fn body_parts(
    layout: &Layout,
    height: i32,
    colours: Colours,
    hair: Hair,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    // The picture's plane faces the camera, as weapons' do.
    let camera = Transform::from_translation(CAMERA_AT).looking_at(Vec3::ZERO, CAMERA_UP);
    let [skin, marked, coat, feathers, hair, mane, flesh] =
        fitted(layout, height, hair, colours.marks);
    [
        (skin, colours.skin),
        (
            marked,
            colours.marks.map_or(colours.skin, |marks| marks.colour),
        ),
        (coat, colours.coat),
        (feathers, colours.feathers),
        (hair, colours.hair),
        (mane, colours.mane),
        (flesh, FLESH),
    ]
    .into_iter()
    .filter(|(triangles, _)| !triangles.is_empty())
    .map(|(triangles, colour)| Part {
        mesh: meshes.add(mesh_of(&triangles)),
        material: materials.add(material(colour)),
        transform: Transform::from_rotation(camera.rotation),
    })
    .collect()
}

#[cfg(test)]
mod parts_tests;
#[cfg(test)]
mod tests;
