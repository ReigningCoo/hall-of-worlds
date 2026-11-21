//! Which kindling a fire is lit or fed with: the first carried, as Cut
//! Kindling's log (the own grid top to bottom, left to right, then bags).

use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind};
use crate::material::Material;

/// The stack of kindling Make Campfire takes a piece from, and its wood.
pub fn kindling_to_burn(carried: &Inventory) -> Option<(Id, Material)> {
    let is_kindling = |item: &Item| item.kind == ItemKind::Kindling;
    let own = (carried.items.iter())
        .filter(|placed| is_kindling(&placed.item))
        .min_by_key(|placed| (placed.y, placed.x))
        .map(|placed| &placed.item);
    let item = own.or_else(|| {
        (carried.stacks_where(is_kindling))
            .find(|(_, own)| !own)
            .map(|(item, _)| item)
    })?;
    // Kindling given without a form (older saves) burns as the middle.
    let wood = item.kindling().map_or(Material::Wood, |form| form.wood);
    Some((item.id, wood))
}
