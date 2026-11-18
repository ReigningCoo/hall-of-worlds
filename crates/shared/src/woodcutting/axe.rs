//! The axe Fell Tree and Cut Kindling are done with: a one-handed axe held,
//! not just carried (owner).

use crate::blade::{Family, Hands};
use crate::creature::Location;
use crate::item::{Inventory, Item, Quality};
use crate::item_form::ItemForm;

/// What a held one-handed axe brings to felling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeldAxe {
    pub quality: Quality,
    /// A hatchet: any other axe takes half as long again (owner).
    pub hatchet: bool,
}

/// What a hatchet is called, by its own row or its generic cell
/// (`blade::table`'s axes, `blade::cells`).
const HATCHET: &str = "Hatchet";

impl HeldAxe {
    /// The axe this item is, if it is a one-handed one.
    pub fn of(item: &Item) -> Option<Self> {
        let Some(ItemForm::Blade(blade)) = &item.form else {
            return None;
        };
        (blade.family == Family::Axe && blade.hands == Hands::One).then(|| Self {
            quality: item.quality.unwrap_or_default(),
            hatchet: blade.name() == HATCHET,
        })
    }
}

impl Inventory {
    /// The one-handed axe held, if any: of two, a hatchet first, then the
    /// better.
    pub fn held_axe(&self) -> Option<HeldAxe> {
        let held = |item: &&Item| {
            (item.kind.def().wear).is_some_and(|wear| wear.locations.contains(&Location::Held))
        };
        (self.worn.iter().filter(held))
            .filter_map(HeldAxe::of)
            .max_by_key(|axe| (axe.hatchet, axe.quality))
    }
}
