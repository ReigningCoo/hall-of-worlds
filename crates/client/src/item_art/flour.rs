//! A sack of flour (milling PR 1): an open hessian sack standing upright, its
//! top rolled down into a thick cuff, flour heaped above the rim and spilt
//! over the front, its bottom slumped where it sits (owner, 2026-11-18: a
//! traditional open flour sack). Tipped a little so the heap
//! shows. The hessian is plain; the flour is wholemeal: the grain's inside
//! from the table (`VegetableLook::inside`) with some of its skin, the bran
//! (`BRAN`), so maize's is yellow, buckwheat's grey and teff's brown. One cell.

use bevy::prelude::*;
use shared::herb::Rgb;
use shared::item::Item;
use shared::vegetable::Vegetables;
use std::f32::consts::FRAC_PI_2;

use super::Part;
use mesh::{BODY, CUFF, HEAP, rings_mesh};

mod mesh;

/// The colour of a sack's flour, by its grain: what tells sacks apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FlourLook(pub Rgb);

impl FlourLook {
    /// A flour item's look, or `None` for anything else. A grain the table
    /// lacks gets plain white flour.
    pub fn of(item: &Item) -> Option<Self> {
        let flour = item.flour()?;
        let Some(def) = Vegetables::built_in().get(&flour.grain) else {
            return Some(Self(Rgb(240, 236, 226)));
        };
        let inside = def.look.inside;
        let skin = def.look.colours.first().copied().unwrap_or(inside);
        let mix = |inside: u8, skin: u8| {
            let (inside, skin) = (f32::from(inside), f32::from(skin));
            (inside + (skin - inside) * BRAN).round() as u8
        };
        Some(Self(Rgb(
            mix(inside.0, skin.0),
            mix(inside.1, skin.1),
            mix(inside.2, skin.2),
        )))
    }
}

/// How much of the grain's skin colour wholemeal flour takes on.
const BRAN: f32 = 0.35;
/// How far the sack is tipped towards the camera, so the heap shows.
const TIP: f32 = 0.55;

/// Hessian's colour.
const HESSIAN: Vec3 = Vec3::new(0.7, 0.56, 0.38);

/// The sack's parts (`Look::Flour`), against the picture's `VIEW_WIDTH` of
/// 2.5.
pub(super) fn flour_parts(
    look: FlourLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let Rgb(red, green, blue) = look.0;
    let flour = Vec3::new(red.into(), green.into(), blue.into()) / 255.0;
    let matte = |parts: Vec3| StandardMaterial {
        base_color: Color::srgb(parts.x, parts.y, parts.z),
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    };
    let hessian = materials.add(matte(HESSIAN));
    let cuff = materials.add(matte(HESSIAN * 0.82));
    let dusted = materials.add(matte(HESSIAN.lerp(flour, 0.6)));
    let powder = materials.add(matte(flour));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let parts = vec![
        // The body, its sides pushed out by the flour, and the cuff
        // (`mesh.rs`).
        part(meshes.add(rings_mesh(&BODY)), &hessian, Transform::IDENTITY),
        part(meshes.add(rings_mesh(&CUFF)), &cuff, Transform::IDENTITY),
        // The flour heaped above the rim, a lump to one side.
        part(meshes.add(rings_mesh(&HEAP)), &powder, Transform::IDENTITY),
        part(
            meshes.add(Sphere::new(0.22)),
            &powder,
            Transform::from_xyz(-0.22, 0.66, 0.02).with_scale(Vec3::new(1.3, 0.6, 1.0)),
        ),
        // Flour spilt over the front of the cuff.
        part(
            meshes.add(Sphere::new(0.2)),
            &dusted,
            Transform::from_xyz(0.18, 0.36, 0.47).with_scale(Vec3::new(1.6, 0.5, 0.2)),
        ),
    ];
    // Built standing up, tipped, then turned to face the pictures' camera,
    // which looks down from above with -z up the picture.
    let facing = Transform::from_rotation(Quat::from_rotation_x(TIP - FRAC_PI_2));
    parts
        .into_iter()
        .map(|part| Part {
            transform: facing * part.transform,
            ..part
        })
        .collect()
}
