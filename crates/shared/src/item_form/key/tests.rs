use uuid::Uuid;

use super::*;
use crate::door::{has_unfound_keys, note_found_keys, note_key_turned};
use crate::item::{Inventory, Item, ItemKind};
use crate::item_form::ItemForm;

fn place(x: i64) -> KeyPlace {
    KeyPlace::Planet {
        world: WorldId(Uuid::from_u128(9)),
        runes: Some(GateCode([0, 1, 2, 3, 4, 5, 6])),
        name: Some("Ashford".into()),
        x,
        y: -388,
    }
}

fn key_item(n: u128, lock: Id) -> Item {
    let mut key = Item::new(Id::for_test(n), ItemKind::Key);
    key.form = Some(ItemForm::key(lock));
    key
}

fn form(item: &Item) -> &KeyForm {
    match &item.form {
        Some(ItemForm::Key(key)) => key,
        other => panic!("not a key: {other:?}"),
    }
}

#[test]
fn where_it_was_found_is_kept_from_the_first_time() {
    let mut key = KeyForm::new(Id::for_test(1));
    assert!(key.note_found(&place(1)));
    assert!(!key.note_found(&place(2)));
    assert_eq!(key.found, KeyFound::At(place(1)));
    // An old save's key never gets a place it wasn't found at.
    let mut old = KeyForm {
        found: KeyFound::Unknown,
        ..KeyForm::new(Id::for_test(1))
    };
    assert!(!old.note_found(&place(1)));
    assert_eq!(old.found, KeyFound::Unknown);
}

#[test]
fn the_first_door_is_kept_and_each_way_it_turned() {
    let mut key = KeyForm::new(Id::for_test(1));
    assert!(key.note_turned(&place(1), Turned::Unlocked));
    assert!(!key.note_turned(&place(2), Turned::Unlocked));
    assert!(key.note_turned(&place(2), Turned::Locked));
    let used = key.used.unwrap();
    assert_eq!(used.at, place(1));
    assert!(used.locked && used.unlocked);
}

#[test]
fn keys_carried_anywhere_are_found_and_turned() {
    let lock = Id::for_test(50);
    let mut bag = Item::new(Id::for_test(1), ItemKind::Pouch);
    let mut inventory = Inventory::default();
    bag.contents.push(crate::item::Placed {
        item: key_item(2, lock),
        x: 0,
        y: 0,
        rotation: crate::item::Rotation::None,
    });
    inventory.worn.push(bag);
    assert!(has_unfound_keys(&inventory));
    assert!(note_found_keys(&mut inventory, &place(1)));
    assert!(!has_unfound_keys(&inventory));
    assert!(!note_found_keys(&mut inventory, &place(2)));
    assert!(note_key_turned(
        &mut inventory,
        lock,
        &place(3),
        Turned::Locked
    ));
    assert!(!note_key_turned(
        &mut inventory,
        Id::for_test(51),
        &place(3),
        Turned::Locked
    ));
    let key = form(&inventory.worn[0].contents[0].item);
    assert_eq!(key.found, KeyFound::At(place(1)));
    assert_eq!(key.used.as_ref().map(|used| used.locked), Some(true));
}

#[test]
fn an_old_saved_key_loads_as_found_somewhere_unknown() {
    let lock = Id::for_test(77);
    let old = format!("Key(\"{lock}\")");
    let form: ItemForm = ron::from_str(&old).unwrap();
    let expected = KeyForm {
        found: KeyFound::Unknown,
        ..KeyForm::new(lock)
    };
    assert_eq!(form, ItemForm::Key(Box::new(expected)));
}

#[test]
fn a_key_saves_and_sends_what_it_remembers() {
    let mut key = KeyForm::new(Id::for_test(77));
    key.note_found(&place(1));
    key.note_turned(&KeyPlace::Hall, Turned::Unlocked);
    let mut item = key_item(1, Id::for_test(77));
    item.form = Some(ItemForm::Key(Box::new(key)));
    let saved = ron::to_string(&item).unwrap();
    assert_eq!(ron::from_str::<Item>(&saved).unwrap(), item);
    let sent = postcard::to_allocvec(&item).unwrap();
    assert_eq!(postcard::from_bytes::<Item>(&sent).unwrap(), item);
    // A new key that remembers nothing yet, too.
    let fresh = key_item(2, Id::for_test(78));
    let saved = ron::to_string(&fresh).unwrap();
    assert_eq!(ron::from_str::<Item>(&saved).unwrap(), fresh);
}
