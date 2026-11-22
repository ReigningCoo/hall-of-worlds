//! Carried items by kind (`Inventory::count_of`, `stack_to_use`).

use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Placed, Rotation};
use crate::poi::ChunkPos;
use crate::quest::{QuestId, Slot};

fn food(n: u128, count: u32) -> Item {
    let mut item = Item::new(Id::for_test(n), ItemKind::Bread);
    item.count = count;
    item
}

fn at(item: Item, x: u8) -> Placed {
    Placed {
        item,
        x,
        y: 0,
        rotation: Rotation::None,
    }
}

/// Bread in the own grid (3 and 5), in a backpack (2, and 1 in a pouch in it)
/// and in a worn pouch (4).
fn carrying() -> Inventory {
    let mut pouch = Item::new(Id::for_test(20), ItemKind::Pouch);
    pouch.contents.push(at(food(5, 1), 0));
    let mut backpack = Item::new(Id::for_test(10), ItemKind::Backpack);
    backpack.contents.push(at(food(3, 2), 0));
    backpack.contents.push(at(pouch, 2));
    let mut worn = Item::new(Id::for_test(30), ItemKind::Pouch);
    worn.contents.push(at(food(4, 4), 0));
    Inventory {
        items: vec![at(food(1, 5), 0), at(food(2, 3), 2), at(backpack, 4)],
        worn: vec![worn],
        ..Inventory::default()
    }
}

#[test]
fn every_one_carried_is_counted_however_deep() {
    let inventory = carrying();
    assert_eq!(inventory.count_of(ItemKind::Bread), 5 + 3 + 2 + 1 + 4);
    assert_eq!(inventory.count_of(ItemKind::Fruit), 0);
}

#[test]
fn the_smallest_stack_in_the_own_grid_is_used_first() {
    let mut inventory = carrying();
    assert_eq!(
        inventory.stack_to_use(ItemKind::Bread),
        Some(Id::for_test(2))
    );
    // With none in the own grid, the smallest in any bag.
    inventory
        .items
        .retain(|placed| placed.item.kind != ItemKind::Bread);
    assert_eq!(
        inventory.stack_to_use(ItemKind::Bread),
        Some(Id::for_test(5))
    );
    assert_eq!(inventory.stack_to_use(ItemKind::Fruit), None);
}

#[test]
fn quest_items_are_never_counted_or_used() {
    let mut inventory = Inventory::default();
    let mut quest = food(1, 1);
    quest.quest = Some(QuestId {
        town: ChunkPos { x: 1, y: 0 },
        slot: Slot::Feeder,
    });
    inventory.items.push(at(quest, 0));
    assert_eq!(inventory.count_of(ItemKind::Bread), 0);
    assert_eq!(inventory.stack_to_use(ItemKind::Bread), None);
}

#[test]
fn the_best_quality_carried() {
    use crate::item::Quality;
    let mut inv = Inventory::default();
    assert_eq!(inv.best_quality_of(|k| k == ItemKind::Pan), None);
    let mut poor = Item::new(Id::for_test(1), ItemKind::Pan);
    poor.quality = Some(Quality::Poor);
    let mut rare = Item::new(Id::for_test(2), ItemKind::Pan);
    rare.quality = Some(Quality::Rare);
    inv.items.push(at(poor, 0));
    inv.items.push(at(rare, 2));
    inv.items
        .push(at(Item::new(Id::for_test(3), ItemKind::Bowl), 4));
    assert_eq!(
        inv.best_quality_of(|k| k == ItemKind::Pan),
        Some(Quality::Rare)
    );
    // None counts as Common.
    assert_eq!(
        inv.best_quality_of(|k| k == ItemKind::Bowl),
        Some(Quality::Common)
    );
}
