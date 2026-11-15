//! Words for a ring's or necklace's name: its gems counted and in the plural.

use super::{ItemForm, Setting};
use crate::item::Item;
use crate::material::Material;

/// `base`, then what is in the settings as they are now: "Gold Ring with a Ruby",
/// "... with Two Rubies and an Empty Setting". Just `base` without settings.
pub(super) fn with_stones(base: String, settings: &[Setting]) -> String {
    let mut stones: Vec<(Material, usize)> = Vec::new();
    let mut empty = 0;
    for setting in settings {
        match setting.gem.as_ref().and_then(gem_material) {
            Some(stone) => match stones.iter_mut().find(|(other, _)| *other == stone) {
                Some((_, count)) => *count += 1,
                None => stones.push((stone, 1)),
            },
            None => empty += 1,
        }
    }
    let mut parts: Vec<String> = stones
        .into_iter()
        .map(|(stone, count)| counted(count, stone.name(), plural(stone)))
        .collect();
    if empty > 0 {
        parts.push(counted(empty, "Empty Setting", "Empty Settings".to_owned()));
    }
    match parts.split_last() {
        None => base,
        Some((only, [])) => format!("{base} with {only}"),
        Some((last, others)) => format!("{base} with {} and {last}", others.join(", ")),
    }
}

pub(super) fn gem_material(item: &Item) -> Option<Material> {
    match &item.form {
        Some(ItemForm::Gem(gem)) => Some(gem.material),
        _ => None,
    }
}

pub(super) fn counted(count: usize, one: &str, many: String) -> String {
    let a = if one.starts_with(['A', 'E']) {
        "an"
    } else {
        "a"
    };
    match count {
        1 => format!("{a} {one}"),
        2 => format!("Two {many}"),
        3 => format!("Three {many}"),
        4 => format!("Four {many}"),
        5 => format!("Five {many}"),
        _ => format!("{count} {many}"),
    }
}

pub(super) fn plural(stone: Material) -> String {
    match stone {
        Material::Ruby => "Rubies".to_owned(),
        Material::Topaz => "Topazes".to_owned(),
        _ => format!("{}s", stone.name()),
    }
}
