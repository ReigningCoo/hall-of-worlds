//! Item Pictures' Vegetables page: every vegetable, pulse and grain item as it
//! is first drawn, each variant of a few (their other colours among them), and
//! slices of those cut into servings.

use bevy::prelude::*;
use shared::rng::Seed;
use shared::vegetable::{VegetableForm, Vegetables};

use super::Shown;
use super::armour::spawn_row;
use crate::item_art::{Look, VEGETABLE_VARIANTS, VegetableItemLook};

/// How many pictures a row holds.
const ROW: usize = 8;
/// Vegetables shown in every variant.
const VARIED: [&str; 4] = ["carrot", "potato", "kidney_bean", "maize"];

/// One vegetable item, by its place in the table.
#[derive(Clone, Copy)]
pub(in crate::debug_menu) struct VegetableShown {
    index: u16,
    serving: bool,
    variant: u8,
}

impl VegetableShown {
    pub(super) fn look(self) -> Option<Look> {
        let def = Vegetables::built_in().iter().nth(usize::from(self.index))?;
        let form = match self.serving {
            true => VegetableForm::serving(def, Seed::new(0)),
            false => VegetableForm::new(def, Seed::new(0)),
        };
        let form = VegetableForm {
            look: u64::from(self.variant),
            ..form
        };
        Some(Look::Vegetable(VegetableItemLook::of(&form)))
    }
}

/// The place of the vegetable with this key.
fn index(key: &str) -> u16 {
    let place = Vegetables::built_in().iter().position(|def| def.key == key);
    place.unwrap_or_default() as u16
}

fn shown(index: u16, serving: bool, variant: u8) -> Shown {
    Shown::Vegetable(VegetableShown {
        index,
        serving,
        variant,
    })
}

/// The rows of the Vegetables page.
pub(super) fn spawn_vegetable_rows(grid: &mut ChildSpawnerCommands) {
    let count = Vegetables::built_in().len();
    for start in (0..count).step_by(ROW) {
        let label = if start == 0 { "Every one" } else { "" };
        let row = (start..count.min(start + ROW)).map(|n| shown(n as u16, false, 0));
        spawn_row(grid, label, row);
    }
    for key in VARIED {
        let vegetable = index(key);
        for half in 0..VEGETABLE_VARIANTS / ROW as u8 {
            let label = if half == 0 { key } else { "" };
            let variants =
                (0..ROW as u8).map(move |n| shown(vegetable, false, half * ROW as u8 + n));
            spawn_row(grid, label, variants);
        }
    }
    let sliced = Vegetables::built_in().iter().enumerate();
    let slices = sliced
        .filter(|(_, def)| def.servings > 1)
        .map(|(n, _)| shown(n as u16, true, 0));
    spawn_row(grid, "Slices", slices);
}
