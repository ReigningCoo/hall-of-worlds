//! Holders (owner, 2026-11-17): a keyring tied to the belt keeps its own grid,
//! takes only keys, ties and unties full, one of each kind at a time, makes
//! what is in it lighter by its quality, and is where keys from outside go first.

use super::*;
use crate::creature::CreatureKind;
use crate::id::Id;

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// A human with a 6 × 4 grid, carrying these kinds with ids from 1.
fn carrying(kinds: &[ItemKind]) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (6, 4),
        ..Inventory::default()
    };
    for (index, &kind) in kinds.iter().enumerate() {
        let item = Item::new(id(index as u128 + 1), kind);
        inventory
            .give(item, StackLimits::NONE, &mut crate::id::stand_ins())
            .expect("room");
    }
    inventory
}

/// A human wearing a Rare belt (1, three slots) with a keyring (2) tied to it,
/// and these kinds from 3.
fn with_keyring(kinds: &[ItemKind]) -> Inventory {
    let mut all = vec![ItemKind::Belt, ItemKind::Keyring];
    all.extend_from_slice(kinds);
    let mut inventory = carrying(&all);
    inventory.items[0].item.quality = Some(Quality::Rare);
    inventory.wear(id(1)).unwrap();
    inventory.tie_bag(id(2), 0).unwrap();
    inventory
}

fn key(n: u128) -> Item {
    Item::new(id(n), ItemKind::Key)
}

#[test]
fn a_tied_keyring_keeps_its_own_grid_and_adds_no_cells() {
    let inventory = with_keyring(&[]);
    assert!(inventory.is_tied(id(2)));
    assert!(!inventory.adds_to_grid(id(2)));
    assert_eq!(inventory.grid_cells(), 24, "the own grid as it was");
    assert_eq!(inventory.grid(Holder::Inside(id(2))), Some(&[][..]));
    let (width, _) = inventory.grid_size(Holder::Inside(id(2))).unwrap();
    assert_eq!(width, 6);
}

#[test]
fn a_keyring_takes_only_keys() {
    let mut inventory = with_keyring(&[ItemKind::Key, ItemKind::Ring]);
    let ring = Holder::Inside(id(2));
    assert_eq!(
        inventory.move_item(id(4), ring, 0, 0, Rotation::None),
        Err(MoveRefused::NotTaken)
    );
    assert_eq!(
        inventory.move_item(id(3), ring, 2, 1, Rotation::None),
        Ok(())
    );
    assert_eq!(inventory.placed(id(3)).map(|(at, _)| at), Some(ring));
    // And back out to a chosen cell.
    assert_eq!(
        inventory.move_item(id(3), Holder::Inventory, 5, 3, Rotation::None),
        Ok(())
    );
}

#[test]
fn a_keyring_is_tied_and_untied_with_its_keys() {
    let mut inventory = carrying(&[ItemKind::Belt, ItemKind::Keyring, ItemKind::Key]);
    inventory.wear(id(1)).unwrap();
    let ring = Holder::Inside(id(2));
    inventory
        .move_item(id(3), ring, 0, 0, Rotation::None)
        .unwrap();
    assert_eq!(inventory.tie_bag(id(2), 0), Ok(()));
    assert!(inventory.find(id(3)).is_some(), "the key came along");
    assert_eq!(
        inventory.move_item(id(2), Holder::Inventory, 0, 0, Rotation::None),
        Ok(())
    );
    assert_eq!(inventory.placed(id(3)).map(|(at, _)| at), Some(ring));
    // A pouch still has to be empty.
    let mut inventory = carrying(&[ItemKind::Belt, ItemKind::Pouch, ItemKind::Ring]);
    inventory.wear(id(1)).unwrap();
    let pouch = Holder::Inside(id(2));
    inventory
        .move_item(id(3), pouch, 0, 0, Rotation::None)
        .unwrap();
    assert_eq!(inventory.tie_bag(id(2), 0), Err(MoveRefused::NotEmpty));
}

#[test]
fn untying_a_keyring_needs_room_for_it() {
    let mut inventory = with_keyring(&[]);
    // Fill the own grid with stones, then nothing is left for the keyring.
    let mut n = 10;
    while inventory
        .give(
            Item::new(id(n), ItemKind::Stone),
            StackLimits::NONE,
            &mut crate::id::stand_ins(),
        )
        .is_ok()
    {
        n += 1;
    }
    assert_eq!(
        inventory.move_item(id(2), Holder::Inventory, 0, 0, Rotation::None),
        Err(MoveRefused::NoRoom)
    );
}

#[test]
fn only_one_keyring_is_tied_at_a_time() {
    let mut inventory = with_keyring(&[ItemKind::Keyring, ItemKind::Pouch]);
    assert_eq!(inventory.tie_bag(id(3), 1), Err(MoveRefused::HolderTied));
    assert_eq!(
        inventory.tie_bag(id(4), 1),
        Ok(()),
        "other bags still go on"
    );
    // On the keyring's own slot it swaps, the old one coming out with its keys.
    assert_eq!(inventory.tie_bag(id(3), 0), Ok(()));
    assert!(inventory.is_tied(id(3)));
    assert!(!inventory.is_tied(id(2)));
}

#[test]
fn keys_from_outside_go_into_the_keyring_first() {
    let mut inventory = with_keyring(&[]);
    let ring = Holder::Inside(id(2));
    assert_eq!(inventory.route_for(&key(5), StackLimits::NONE), ring);
    inventory
        .give(key(5), StackLimits::NONE, &mut crate::id::stand_ins())
        .unwrap();
    assert_eq!(inventory.placed(id(5)).map(|(at, _)| at), Some(ring));
    // Anything else goes in the own grid.
    let stone = Item::new(id(6), ItemKind::Stone);
    assert_eq!(
        inventory.route_for(&stone, StackLimits::NONE),
        Holder::Inventory
    );
    // Without a keyring tied, keys go in the own grid too.
    let loose = carrying(&[ItemKind::Keyring]);
    assert_eq!(
        loose.route_for(&key(5), StackLimits::NONE),
        Holder::Inventory
    );
}

#[test]
fn a_keyring_grows_without_limit() {
    let mut inventory = with_keyring(&[]);
    for n in 0..200 {
        inventory
            .give(key(100 + n), StackLimits::NONE, &mut crate::id::stand_ins())
            .unwrap();
    }
    let ring = Holder::Inside(id(2));
    assert_eq!(inventory.grid(ring).map(<[Placed]>::len), Some(200));
    assert!(inventory.find(id(299)).is_some());
}

#[test]
fn keys_in_a_keyring_weigh_less_by_its_quality() {
    let lightened: Vec<u32> = Quality::ALL.map(|q| q.lightened(600)).to_vec();
    assert_eq!(lightened, [600, 500, 400, 300, 200, 100, 0]);
    let mut inventory = with_keyring(&[]);
    for n in 0..10 {
        inventory
            .give(key(100 + n), StackLimits::NONE, &mut crate::id::stand_ins())
            .unwrap();
    }
    let keys = 10 * ItemKind::Key.def().weight;
    let keyring = ItemKind::Keyring.def().weight;
    let belt = inventory.worn[0].total_weight();
    // A Common keyring: a sixth off.
    assert_eq!(
        inventory.total_weight(),
        belt + keyring + Quality::Common.lightened(keys)
    );
    inventory.bag_slots[0].quality = Some(Quality::Artifact);
    assert_eq!(inventory.total_weight(), belt + keyring);
}

#[test]
fn keys_on_a_tied_keyring_open_doors_and_are_noted() {
    use crate::door::{carries_key, has_unfound_keys};
    use crate::item_form::ItemForm;
    let lock = id(77);
    let mut inventory = with_keyring(&[]);
    let mut door_key = key(5);
    door_key.form = Some(ItemForm::key(lock));
    inventory
        .give(door_key, StackLimits::NONE, &mut crate::id::stand_ins())
        .unwrap();
    assert_eq!(
        inventory.placed(id(5)).map(|(at, _)| at),
        Some(Holder::Inside(id(2)))
    );
    assert!(carries_key(&inventory, lock));
    assert!(has_unfound_keys(&inventory));
}
