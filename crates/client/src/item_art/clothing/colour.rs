//! How clothes look: cloth in its dye (natural silk ivory), velvet deeper and matte,
//! damask's sheen, a second colour for patterns, bands and linings, gold or coloured
//! thread, cord, fur and buttons.

use bevy::prelude::*;
use shared::armour::Dye;
use shared::clothing::{ClothingForm, Weave};
use shared::item::Quality;
use shared::material::Material;

use crate::item_art::armour::torso::dye_colour;
use crate::item_art::armour::{Stuff, colour_of, darker, matte};
use crate::item_art::ring::metal_material;

/// Cloth of this material in this dye.
fn cloth(dye: Dye, material: Material) -> Color {
    match (dye, material) {
        (Dye::Natural, Material::Silk) => Color::srgb(0.94, 0.9, 0.8),
        (dye, material) => dye_colour(dye, material),
    }
}

/// How rough it looks: silk has a sheen, wool and linen none.
fn roughness(material: Material) -> f32 {
    match material {
        Material::Silk => 0.45,
        Material::Leather | Material::Suede | Material::BuffLeather => 0.65,
        _ => 0.92,
    }
}

fn scaled(colour: Color, by: f32) -> Color {
    let c = colour.to_srgba();
    Color::srgb(
        (c.red * by).min(1.0),
        (c.green * by).min(1.0),
        (c.blue * by).min(1.0),
    )
}

/// The second colour: its own, or for bands on something with none, red (yellow on
/// red).
fn second(form: &ClothingForm) -> Color {
    let fallback = if form.dye == Dye::Red {
        Dye::Yellow
    } else {
        Dye::Red
    };
    cloth(form.contrast.unwrap_or(fallback), form.material)
}

pub(in crate::item_art) fn material_of(
    stuff: Stuff,
    form: &ClothingForm,
    quality: Option<Quality>,
) -> StandardMaterial {
    let colour = cloth(form.dye, form.material);
    let main = || match form.weave {
        Weave::Velvet => matte(scaled(colour, 0.78), 1.0),
        _ => matte(colour, roughness(form.material)),
    };
    match stuff {
        Stuff::Main => main(),
        Stuff::Shade => darker(main()),
        Stuff::Cloth => matte(second(form), roughness(form.material)),
        // Tartan's crossings: both colours, darker.
        Stuff::Charge => {
            let (a, b) = (colour.to_srgba(), second(form).to_srgba());
            let mixed = Color::srgb(
                (a.red + b.red) * 0.33,
                (a.green + b.green) * 0.33,
                (a.blue + b.blue) * 0.33,
            );
            matte(mixed, roughness(form.material))
        }
        // Damask's figures, in the cloth's own sheen.
        Stuff::Trim => matte(scaled(colour, 1.15), 0.3),
        Stuff::Field => {
            let fine = matches!(quality, Some(quality) if quality >= Quality::Rare);
            if form.weave == Weave::Brocade || fine {
                metal_material(Material::Gold, quality)
            } else {
                let thread = form.contrast.unwrap_or(Dye::Yellow);
                matte(cloth(thread, Material::Silk), 0.5)
            }
        }
        Stuff::Backing => matte(Color::srgb(0.3, 0.2, 0.12), 0.7),
        Stuff::Bone => matte(colour_of(Material::Fur), 1.0),
        _ => metal_material(Material::Brass, quality),
    }
}
