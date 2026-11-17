//! A herb picture's colours: its plant's leaves and flowers (`PlantLook`), the
//! colour its description gives its berries, fruit, seeds or pods ("dark purple
//! berries"), and earthy browns for roots and bark, each a shade lighter or
//! darker by variant.

use bevy::prelude::*;
use shared::herb::{HerbDef, Part, Rgb};

/// Colour words a description may use, as a herb's picture shows them.
const WORDS: [(&str, (f32, f32, f32)); 13] = [
    ("red", (0.72, 0.1, 0.1)),
    ("scarlet", (0.8, 0.12, 0.08)),
    ("crimson", (0.62, 0.05, 0.12)),
    ("black", (0.12, 0.08, 0.12)),
    ("purple", (0.38, 0.12, 0.42)),
    ("blue", (0.2, 0.25, 0.6)),
    ("orange", (0.9, 0.48, 0.1)),
    ("yellow", (0.9, 0.78, 0.2)),
    ("white", (0.92, 0.9, 0.84)),
    ("brown", (0.45, 0.3, 0.16)),
    ("green", (0.36, 0.55, 0.25)),
    ("golden", (0.85, 0.65, 0.2)),
    ("pink", (0.88, 0.5, 0.6)),
];

/// What a herb's picture is drawn in.
pub(super) struct Palette {
    pub(super) leaf: Color,
    pub(super) leaf_dark: Color,
    pub(super) flower: Color,
    /// Berries, fruit, seeds or pods: from the description, or a guess by part.
    pub(super) fruit: Color,
    pub(super) earth: Color,
    pub(super) earth_dark: Color,
    /// Cut faces and the insides of roots and bulbs.
    pub(super) pale: Color,
}

impl Palette {
    pub(super) fn of(def: &HerbDef, variant: u8) -> Self {
        // One variant a shade lighter, one darker.
        let shade = [0.9, 1.0, 1.1][usize::from(variant % 3)];
        let tint = |(red, green, blue): (f32, f32, f32), by: f32| {
            Color::srgb(
                (red * by).min(1.0),
                (green * by).min(1.0),
                (blue * by).min(1.0),
            )
        };
        let plant = &def.plant;
        let leaf = rgb(plant.leaf);
        let flower = plant.flower.map_or(lighter(leaf), rgb);
        let guess = match def.part {
            Part::Berries => (0.5, 0.08, 0.15),
            Part::Fruit => (0.85, 0.45, 0.12),
            Part::Resin => (0.82, 0.5, 0.1),
            Part::Pods => darker(leaf),
            _ => (0.42, 0.3, 0.18),
        };
        let fruit = said_colour(&plant.description, def.part).unwrap_or(guess);
        let earth = said_colour(&plant.description, Part::Root)
            .filter(|_| matches!(def.part, Part::Root | Part::Rhizome | Part::Tuber))
            .unwrap_or((0.55, 0.4, 0.26));
        Self {
            leaf: tint(leaf, shade),
            leaf_dark: tint(darker(leaf), shade),
            flower: tint(flower, shade),
            fruit: tint(fruit, shade),
            earth: tint(earth, shade),
            earth_dark: tint(darker(earth), shade),
            pale: tint((0.9, 0.85, 0.7), shade),
        }
    }
}

fn rgb(Rgb(red, green, blue): Rgb) -> (f32, f32, f32) {
    let unit = |c: u8| f32::from(c) / 255.0;
    (unit(red), unit(green), unit(blue))
}

fn darker((red, green, blue): (f32, f32, f32)) -> (f32, f32, f32) {
    (red * 0.6, green * 0.6, blue * 0.6)
}

fn lighter((red, green, blue): (f32, f32, f32)) -> (f32, f32, f32) {
    let up = |c: f32| c + (1.0 - c) * 0.4;
    (up(red), up(green), up(blue))
}

/// The colour a description gives the part ("glossy black berries"): a colour
/// word among the three words before the part's own word, in the same clause.
pub(super) fn said_colour(description: &str, part: Part) -> Option<(f32, f32, f32)> {
    let nouns: &[&str] = match part {
        Part::Berries => &["berr"],
        Part::Fruit => &["fruit"],
        Part::Seeds => &["seed"],
        Part::Pods => &["pod"],
        Part::Root | Part::Rhizome | Part::Tuber => &["root", "rhizome", "tuber"],
        _ => return None,
    };
    let lower = description.to_lowercase();
    // Within one clause: "yellow flowers; berries" says nothing of the berries.
    lower.split([',', ';', '.', ':']).find_map(|clause| {
        let words: Vec<&str> = clause
            .split(|c: char| !c.is_alphabetic())
            .filter(|word| !word.is_empty())
            .collect();
        let at = words
            .iter()
            .position(|word| nouns.iter().any(|noun| word.starts_with(noun)))?;
        words[at.saturating_sub(3)..at]
            .iter()
            .rev()
            .find_map(|word| {
                let word = word.strip_suffix("ish").unwrap_or(word);
                WORDS
                    .iter()
                    .find(|(name, _)| *name == word)
                    .map(|(_, colour)| *colour)
            })
    })
}
