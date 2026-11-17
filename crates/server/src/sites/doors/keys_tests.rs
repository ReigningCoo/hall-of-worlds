//! Keys remembering where they were found and the door they turned in.

use shared::door::note_found_keys;
use shared::id::Id;
use shared::item_form::{ItemForm, KeyForm, KeyFound, KeyPlace};
use shared::protocol::DoorAsk;

use super::tests::{ask, beside_a_door, lock_it};
use crate::connections::tests::TestClient;
use crate::sites::key_place;

/// The form of the key to the test lock the player carries.
fn carried_key(test: &TestClient) -> KeyForm {
    let mut carried = test.client.inventory.items.iter();
    carried
        .find_map(|placed| match &placed.item.form {
            Some(ItemForm::Key(key)) if key.lock == Id::for_test(77) => Some((**key).clone()),
            _ => None,
        })
        .expect("the key is carried")
}

#[test]
fn a_key_remembers_the_castle_it_was_found_in_and_its_door() {
    let mut test = TestClient::new();
    let (door, _) = beside_a_door(&mut test);
    lock_it(&mut test, door);
    ask(&mut test, DoorAsk::DevKey);

    // Noted where the player is: in the castle, at its poi, on this planet.
    let here = key_place(&test.client, &test.world);
    let KeyPlace::Planet {
        world, name, x, y, ..
    } = &here
    else {
        panic!("not on a planet: {here:?}");
    };
    assert_eq!(*world, test.world.seed.id());
    assert!(
        name.as_deref()
            .is_some_and(|name| name.starts_with("Castle "))
    );
    assert_eq!((*x, *y), test.client.world_position());
    assert!(note_found_keys(&mut test.client.inventory, &here));
    assert_eq!(carried_key(&test).found, KeyFound::At(here.clone()));

    // Unlocked and opened, then closed and locked: both noted, at that door.
    let (x, y) = door;
    ask(&mut test, DoorAsk::Open { x, y });
    let used = carried_key(&test).used.expect("it has been used");
    assert!(used.unlocked && !used.locked);
    assert_eq!(used.at, here);
    ask(&mut test, DoorAsk::CloseNear);
    ask(&mut test, DoorAsk::CloseNear);
    let used = carried_key(&test).used.expect("it has been used");
    assert!(used.unlocked && used.locked);
}
