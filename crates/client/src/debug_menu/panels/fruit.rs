//! Item Pictures' Fruit page: every fruit and nut item as it is first drawn,
//! each variant of a few, and slices of the big fruits.

use bevy::prelude::*;
use shared::fruit::{FruitForm, Fruits};

use super::Shown;
use super::armour::spawn_row;
use crate::item_art::{FRUIT_VARIANTS, FruitItemLook, Look};

/// How many pictures a row holds.
const ROW: usize = 8;
/// Fruits shown in every variant.
const VARIED: [&str; 4] = ["apple", "cherry", "rambutan", "walnut"];
/// Fruits cut into servings shown as slices, with two that never are.
const SLICED: [&str; 8] = [
    "watermelon",
    "melon",
    "pineapple",
    "durian",
    "jackfruit",
    "breadfruit",
    "papaya",
    "coconut",
];

/// One fruit item, by its place among the table's fruit items.
#[derive(Clone, Copy)]
pub(in crate::debug_menu) struct FruitShown {
    index: u16,
    serving: bool,
    variant: u8,
}

impl FruitShown {
    pub(super) fn look(self) -> Option<Look> {
        let def = Fruits::built_in().items().nth(usize::from(self.index))?;
        let servings = u32::from(def.servings.max(1));
        let grams = match self.serving {
            true => def.grams.div_ceil(servings),
            false => def.grams,
        };
        let form = FruitForm {
            fruit: def.key.clone(),
            serving: self.serving,
            look: u64::from(self.variant),
            grams,
        };
        Some(Look::Fruit(FruitItemLook::of(&form)))
    }
}

/// The place of the fruit item with this key.
fn index(key: &str) -> u16 {
    let place = Fruits::built_in().items().position(|def| def.key == key);
    place.unwrap_or_default() as u16
}

fn shown(index: u16, serving: bool, variant: u8) -> Shown {
    Shown::Fruit(FruitShown {
        index,
        serving,
        variant,
    })
}

/// The rows of the Fruit page.
pub(super) fn spawn_fruit_rows(grid: &mut ChildSpawnerCommands) {
    let count = Fruits::built_in().items().count();
    for start in (0..count).step_by(ROW) {
        let label = if start == 0 { "Every one" } else { "" };
        let row = (start..count.min(start + ROW)).map(|n| shown(n as u16, false, 0));
        spawn_row(grid, label, row);
    }
    for key in VARIED {
        let fruit = index(key);
        for half in 0..FRUIT_VARIANTS / ROW as u8 {
            let label = if half == 0 { key } else { "" };
            let variants = (0..ROW as u8).map(move |n| shown(fruit, false, half * ROW as u8 + n));
            spawn_row(grid, label, variants);
        }
    }
    let slices = SLICED.iter().map(|key| shown(index(key), true, 0));
    spawn_row(grid, "Slices", slices);
}
