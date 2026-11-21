//! An egg (cooking plan PR 3): one egg lying on its side, its broad end
//! towards the bottom of the picture, coloured by the animal that laid it (a
//! hen's brown, a duck's pale green, a goose's white) and speckled for those
//! whose eggs are (turkeys, guineafowl, quail), smaller or larger by its
//! weight. One cell; a stack's count is the inventory's.

use bevy::prelude::*;
use shared::herb::Rgb;
use shared::item::Item;

use super::Part;

/// How an egg looks: its shell, any speckles, and its size in percent of a
/// hen's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EggLook {
    pub shell: Rgb,
    pub speckles: Option<Rgb>,
    pub size: u8,
}

impl EggLook {
    /// An egg item's look, or `None` for anything else.
    pub fn of(item: &Item) -> Option<Self> {
        let egg = item.egg()?;
        let (shell, speckles) = colours(&egg.animal);
        // Its length grows as the cube root of its weight, kept in the cell.
        let size = (egg.grams as f32 / HEN_GRAMS).cbrt() * 100.0;
        Some(Self {
            shell,
            speckles,
            size: size.clamp(60.0, 120.0).round() as u8,
        })
    }
}

/// A hen's egg's weight, which is drawn at full size.
const HEN_GRAMS: f32 = 55.0;

/// An animal's eggs' shell and speckles; plain white for one not here.
fn colours(animal: &str) -> (Rgb, Option<Rgb>) {
    match animal {
        "chicken" => (Rgb(196, 140, 92), None),
        "duck" => (Rgb(212, 226, 212), None),
        "muscovy_duck" => (Rgb(240, 236, 222), None),
        "goose" => (Rgb(246, 244, 238), None),
        "turkey" => (Rgb(234, 216, 188), Some(Rgb(150, 100, 70))),
        "guineafowl" => (Rgb(214, 178, 140), Some(Rgb(160, 120, 90))),
        "quail" => (Rgb(222, 206, 170), Some(Rgb(60, 45, 35))),
        _ => (Rgb(244, 242, 236), None),
    }
}

/// Where speckles sit on the upper side, as (across, along) the egg from -1
/// to 1, and how big each is.
const SPECKLES: [(f32, f32, f32); 9] = [
    (-0.35, -0.5, 0.08),
    (0.2, -0.35, 0.06),
    (-0.05, -0.1, 0.1),
    (0.38, 0.05, 0.07),
    (-0.4, 0.2, 0.06),
    (0.1, 0.3, 0.09),
    (-0.15, 0.5, 0.07),
    (0.3, 0.55, 0.05),
    (0.0, -0.7, 0.05),
];

/// The egg's parts (`Look::Egg`), against the picture's `VIEW_WIDTH` of 2.5.
/// The pictures' camera looks down from above with -z up the picture.
pub(super) fn egg_parts(
    look: EggLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let mut colour = |Rgb(red, green, blue): Rgb| {
        materials.add(StandardMaterial {
            base_color: Color::srgb_u8(red, green, blue),
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            ..default()
        })
    };
    let shell = colour(look.shell);
    let scale = f32::from(look.size) / 100.0;
    let sphere = meshes.add(Sphere::new(1.0));
    let part = |material: &Handle<StandardMaterial>, transform: Transform| Part {
        mesh: sphere.clone(),
        material: material.clone(),
        transform: Transform::from_scale(Vec3::splat(scale)) * transform,
    };
    // Two ellipsoids overlapping: the broad end below, the narrower above.
    let mut parts = vec![
        part(
            &shell,
            Transform::from_xyz(0.0, 0.0, 0.15).with_scale(Vec3::new(0.6, 0.58, 0.62)),
        ),
        part(
            &shell,
            Transform::from_xyz(0.0, 0.0, -0.2).with_scale(Vec3::new(0.5, 0.49, 0.66)),
        ),
    ];
    if let Some(speckles) = look.speckles {
        let speckle = colour(speckles);
        for (across, along, size) in SPECKLES {
            // On the shell's upper side, flattened against it.
            let x = across * 0.5;
            let z = along * 0.7;
            let top = 0.56
                * (1.0 - (x / 0.62).powi(2) - (z / 0.8).powi(2))
                    .max(0.0)
                    .sqrt();
            let at = Transform::from_xyz(x, top, z).with_scale(Vec3::new(size, size * 0.3, size));
            parts.push(part(&speckle, at));
        }
    }
    parts
}
