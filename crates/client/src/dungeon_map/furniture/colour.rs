//! Each wood's and stone's colour, which tints the grey picture.

use bevy::prelude::*;
use shared::furniture::{Material, Stone, Wood};

/// A material's colour (linear, for vertex colours), from its sRGB.
pub(super) fn of(material: Material) -> Vec3 {
    let (r, g, b) = match material {
        Material::Wood(wood) => match wood {
            Wood::Oak => (0.62, 0.45, 0.28),
            Wood::Walnut => (0.40, 0.27, 0.18),
            Wood::Ash => (0.86, 0.78, 0.62),
            Wood::Pine => (0.88, 0.72, 0.45),
            Wood::Elm => (0.58, 0.48, 0.38),
            Wood::Beech => (0.88, 0.70, 0.58),
            Wood::Cherry => (0.66, 0.36, 0.24),
            Wood::Ebony => (0.18, 0.15, 0.14),
        },
        Material::Stone(stone) => match stone {
            Stone::Granite => (0.72, 0.68, 0.68),
            Stone::Sandstone => (0.86, 0.70, 0.50),
            Stone::Limestone => (0.88, 0.85, 0.76),
            Stone::Marble => (0.96, 0.95, 0.93),
            Stone::Slate => (0.40, 0.43, 0.48),
        },
    };
    let linear = Color::srgb(r, g, b).to_linear();
    Vec3::new(linear.red, linear.green, linear.blue)
}
