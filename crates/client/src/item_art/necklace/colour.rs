//! How a necklace looks: its metal (any of them, as rings'), darker in its shading,
//! a cord's leather, and the dark of an empty seat.

use bevy::prelude::*;
use shared::item::Quality;
use shared::item_form::Strand;
use shared::material::Material;

use super::NecklaceLook;
use crate::item_art::armour::{Stuff, colour_of, darker, matte};
use crate::item_art::ring::metal_material;

fn leather(look: &NecklaceLook) -> Color {
    match look.strand {
        Strand::Cord {
            leather: Material::Suede,
            ..
        } => Color::srgb(0.6, 0.45, 0.3),
        Strand::Cord {
            leather: Material::Rawhide,
            ..
        } => Color::srgb(0.78, 0.68, 0.5),
        Strand::Cord { leather, .. } => colour_of(leather),
        _ => colour_of(Material::Leather),
    }
}

pub(super) fn material_of(
    stuff: Stuff,
    look: &NecklaceLook,
    quality: Option<Quality>,
) -> StandardMaterial {
    let metal = || metal_material(look.metal, quality);
    match stuff {
        Stuff::Cloth => matte(leather(look), 0.7),
        Stuff::Trim => darker(matte(leather(look), 0.7)),
        Stuff::Shade => darker(metal()),
        Stuff::Backing => matte(Color::srgb(0.05, 0.05, 0.06), 1.0),
        _ => metal(),
    }
}
