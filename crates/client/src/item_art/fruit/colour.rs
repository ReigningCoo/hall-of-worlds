//! A fruit picture's colours: its ripe colour from the table, moved a little
//! towards its unripe colour or deeper by variety, a blush for fruits that
//! have one, and the leaf and stalk.

use bevy::prelude::*;
use shared::fruit::{FruitDef, Skin};
use shared::herb::Rgb;

use super::variety::Variety;

/// Fruits whose skin blushes on the side the sun reached, and the blush's colour.
const BLUSHES: [(&str, Rgb); 6] = [
    ("apple", Rgb(200, 45, 40)),
    ("crab_apple", Rgb(205, 50, 40)),
    ("apricot", Rgb(215, 70, 40)),
    ("mango", Rgb(210, 60, 50)),
    ("pear", Rgb(190, 60, 40)),
    ("jujube", Rgb(130, 45, 30)),
];

/// What a fruit's picture is drawn in.
#[derive(Clone)]
pub(super) struct Colours {
    pub(super) skin: Vec3,
    pub(super) blush: Option<Vec3>,
    /// A dusty bloom or down: matte.
    pub(super) matte: bool,
    pub(super) leaf: Vec3,
    pub(super) stalk: Vec3,
}

impl Colours {
    pub(super) fn of(def: &FruitDef, variety: &Variety) -> Self {
        let ripe = rgb(def.look.ripe);
        let unripe = rgb(def.look.unripe);
        let skin = match variety.tint {
            tint if tint > 0.0 => ripe.lerp(unripe, tint * 0.2),
            tint => ripe * (1.0 + tint * 0.15),
        };
        let skin = match def.look.skin {
            // A bloom dusts it paler.
            Skin::Bloom => skin.lerp(Vec3::new(0.6, 0.65, 0.75), 0.18),
            _ => skin,
        };
        let blush = BLUSHES
            .iter()
            .find(|(key, _)| *key == def.key)
            .filter(|_| variety.blush > 0.15)
            .map(|(_, colour)| rgb(*colour));
        Self {
            skin,
            blush,
            matte: matches!(
                def.look.skin,
                Skin::Bloom | Skin::Downy | Skin::Husk | Skin::Shell
            ),
            leaf: Vec3::new(0.25, 0.45, 0.16),
            stalk: Vec3::new(0.36, 0.26, 0.14),
        }
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
