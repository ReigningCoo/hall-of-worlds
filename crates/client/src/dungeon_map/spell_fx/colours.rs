//! The colour of each effect as drawn: Fire orange, cold pale blue, Heal a
//! living green and so on. Bright, as everything here is added onto the
//! scene (`AlphaMode::Add`), so dark effects are deep, dim colours.

use bevy::prelude::*;
use shared::spell::{Look, Part};

/// How bright a glow is, times its colour.
const BRIGHT: f32 = 1.4;

/// A where carrying nothing: plain magic, pale violet.
const PLAIN: (f32, f32, f32) = (0.8, 0.7, 1.0);

/// The colour a part is drawn in, as `sRGB`.
fn srgb(part: Part) -> (f32, f32, f32) {
    match part.key() {
        "fire" => (1.0, 0.45, 0.1),
        "cold" => (0.6, 0.85, 1.0),
        "water" => (0.2, 0.45, 1.0),
        "ice" => (0.8, 0.95, 1.0),
        "air" => (0.85, 0.9, 0.9),
        "vacuum" => (0.35, 0.2, 0.5),
        "earth" => (0.7, 0.5, 0.2),
        "stoneskin" => (0.6, 0.6, 0.55),
        "heal" => (0.4, 1.0, 0.45),
        "harm" => (0.7, 0.05, 0.1),
        "poison" => (0.55, 0.8, 0.1),
        "cure" => (0.6, 1.0, 0.8),
        "light" => (1.0, 0.95, 0.7),
        "darkness" => (0.25, 0.1, 0.35),
        "haste" => (1.0, 0.9, 0.2),
        "slow" => (0.35, 0.4, 0.6),
        "calm" => (0.5, 0.7, 1.0),
        "fear" => (0.5, 0.05, 0.2),
        "blessing" => (1.0, 0.8, 0.3),
        "weakness" => (0.45, 0.35, 0.5),
        "curse" => (0.55, 0.1, 0.6),
        "uncurse" => (0.8, 0.7, 1.0),
        "illusion" => (1.0, 0.45, 0.85),
        "disillusion" => (0.9, 0.8, 0.9),
        "identify" => (0.3, 0.9, 0.85),
        "conceal" => (0.15, 0.35, 0.35),
        "warding" => (0.7, 0.8, 1.0),
        "dispel" => (1.0, 1.0, 1.0),
        "teleport" => (0.7, 0.3, 1.0),
        "anchor" => (0.4, 0.3, 0.5),
        "levitate" => (0.6, 0.85, 1.0),
        "grounding" => (0.55, 0.4, 0.25),
        "mana" => (0.25, 0.45, 1.0),
        "drain" => (0.15, 0.15, 0.45),
        _ => PLAIN,
    }
}

/// A glowing colour from `sRGB` values, ready to add.
fn glow((r, g, b): (f32, f32, f32)) -> LinearRgba {
    let linear = Color::srgb(r, g, b).to_linear();
    LinearRgba::rgb(linear.red, linear.green, linear.blue) * BRIGHT
}

/// The colours `look` is drawn in, first part first: never empty.
pub(super) fn look_colours(look: &Look) -> Vec<LinearRgba> {
    let colours: Vec<LinearRgba> = look.iter().map(|part| glow(srgb(part))).collect();
    if colours.is_empty() {
        vec![glow(PLAIN)]
    } else {
        colours
    }
}
