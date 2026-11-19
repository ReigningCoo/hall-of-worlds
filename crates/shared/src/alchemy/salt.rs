//! Salt as an item (Alchemy plan PR 5): `ItemKind::Salt`, what an Alembic
//! leaves behind from Salt Water, one for each 100 ml distilled; counted, with
//! no quality. Its use as a Retort solid comes with PR 8.

use crate::id::Id;
use crate::item::{Item, ItemKind};

/// What one Salt weighs, in grams: about what 100 ml of sea water leaves.
pub const SALT_GRAMS: u32 = 4;

/// `count` Salt.
pub fn new_salt(count: u32, id: Id) -> Item {
    let mut item = Item::new(id, ItemKind::Salt);
    item.count = count.max(1);
    item
}
