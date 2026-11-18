//! A vegetable picture's colours: the variety's skin from the table's look, a
//! little lighter or darker by variant, its accent and inside, and the greens,
//! stalks and earth every picture shares.

use bevy::prelude::*;
use shared::herb::Rgb;
use shared::vegetable::VegetableDef;

use super::variety::Variety;

#[derive(Clone)]
pub(super) struct Colours {
    pub(super) skin: Vec3,
    /// The look's accent, if it has one.
    pub(super) accent: Option<Vec3>,
    /// The accent where only the usual variety has it (a bean's mottling).
    pub(super) spots: Option<Vec3>,
    pub(super) inside: Vec3,
    /// Fresh leaves and stems.
    pub(super) leaf: Vec3,
    /// Dry straw, husks and twine.
    pub(super) straw: Vec3,
    /// Rootlets and earth.
    pub(super) earth: Vec3,
}

impl Colours {
    pub(super) fn of(def: &VegetableDef, variety: &Variety) -> Self {
        let look = &def.look;
        let chosen = look.colours.get(variety.colour).or(look.colours.first());
        let skin = chosen.map_or(Vec3::splat(0.6), |c| rgb(*c));
        let skin = skin * (1.0 + variety.shade * 0.06);
        let accent = look.accent.map(rgb);
        Self {
            skin,
            accent,
            spots: accent.filter(|_| variety.colour == 0),
            inside: rgb(look.inside),
            leaf: Vec3::new(0.26, 0.47, 0.18),
            straw: Vec3::new(0.82, 0.72, 0.48),
            earth: Vec3::new(0.42, 0.32, 0.22),
        }
    }

    /// The accent, or this if there is none.
    pub(super) fn accent_or(&self, other: Vec3) -> Vec3 {
        self.accent.unwrap_or(other)
    }
}

/// A table colour as sRGB parts, 0 to 1.
pub(super) fn rgb(Rgb(red, green, blue): Rgb) -> Vec3 {
    Vec3::new(red.into(), green.into(), blue.into()) / 255.0
}

/// A colour from sRGB parts.
pub(super) fn colour(parts: Vec3) -> Color {
    let parts = parts.clamp(Vec3::ZERO, Vec3::ONE);
    Color::srgb(parts.x, parts.y, parts.z)
}
