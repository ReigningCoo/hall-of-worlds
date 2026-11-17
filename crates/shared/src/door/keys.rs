//! Keys: whether someone carries the one for a lock, and what keys carried
//! remember (`item_form::KeyForm`): where they were found and the door turned.

use crate::id::Id;
use crate::item::{Inventory, Item};
use crate::item_form::{ItemForm, KeyForm, KeyFound, KeyPlace, Turned};

/// Whether `item`, or anything inside it or stacked on it, is the key to `lock`.
fn is_or_holds_key(item: &Item, lock: Id) -> bool {
    item.form.as_ref().and_then(ItemForm::lock) == Some(lock)
        || item
            .contents
            .iter()
            .any(|placed| is_or_holds_key(&placed.item, lock))
        || item.above.iter().any(|above| is_or_holds_key(above, lock))
}

/// Whether the key to `lock` is carried, in bags (a tied keyring too), or worn
/// (not binned).
pub fn carries_key(inventory: &Inventory, lock: Id) -> bool {
    let carried = inventory.items.iter().map(|placed| &placed.item);
    carried
        .chain(&inventory.worn)
        .chain(&inventory.bag_slots)
        .any(|item| is_or_holds_key(item, lock))
}

/// Calls `each` with every key carried or worn, in bags and stacks too, until
/// it says to stop (`true`).
fn each_key(inventory: &mut Inventory, each: &mut impl FnMut(&mut KeyForm) -> bool) -> bool {
    let carried = inventory.items.iter_mut().map(|placed| &mut placed.item);
    let worn = inventory.worn.iter_mut().chain(&mut inventory.bag_slots);
    for item in carried.chain(worn) {
        if in_item(item, each) {
            return true;
        }
    }
    false
}

/// `each_key` for one item and everything in it or on it.
fn in_item(item: &mut Item, each: &mut impl FnMut(&mut KeyForm) -> bool) -> bool {
    if let Some(ItemForm::Key(key)) = &mut item.form
        && each(key)
    {
        return true;
    }
    let inside = item.contents.iter_mut().map(|placed| &mut placed.item);
    for item in inside.chain(&mut item.above) {
        if in_item(item, each) {
            return true;
        }
    }
    false
}

/// Whether anything carried is a key no character has had before.
pub fn has_unfound_keys(inventory: &Inventory) -> bool {
    let carried = inventory.items.iter().map(|placed| &placed.item);
    carried
        .chain(&inventory.worn)
        .chain(&inventory.bag_slots)
        .any(unfound_in)
}

/// Whether `item`, or anything in it or on it, is a key not had before.
fn unfound_in(item: &Item) -> bool {
    matches!(&item.form, Some(ItemForm::Key(key)) if key.found == KeyFound::NotYet)
        || item.contents.iter().any(|placed| unfound_in(&placed.item))
        || item.above.iter().any(unfound_in)
}

/// Records `place` on every key carried that no character has had before:
/// whether any was.
pub fn note_found_keys(inventory: &mut Inventory, place: &KeyPlace) -> bool {
    let mut noted = false;
    each_key(inventory, &mut |key| {
        noted |= key.note_found(place);
        false
    });
    noted
}

/// Records the key to `lock` (the first carried) turning in its door at
/// `place`: whether that changed it.
pub fn note_key_turned(
    inventory: &mut Inventory,
    lock: Id,
    place: &KeyPlace,
    turned: Turned,
) -> bool {
    let mut changed = false;
    each_key(inventory, &mut |key| {
        if key.lock != lock {
            return false;
        }
        changed = key.note_turned(place, turned);
        true
    });
    changed
}
