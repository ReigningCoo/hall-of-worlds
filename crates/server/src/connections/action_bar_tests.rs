//! The action bar through a connected client: told on joining, changed by the
//! rules, put right when refused, saved.

use shared::ability::{Ability, ActionBar, BarChange, BarSlot};
use shared::item::ItemKind;
use shared::protocol::{ClientMessage, ServerMessage};
use shared::world::WorldSeed;

use super::tests::TestClient;
use crate::ServerWorld;
use crate::storage::{Storage, tests::TestFolder};

/// The last `ActionBar` among these answers.
fn bar(answers: &[ServerMessage]) -> ActionBar {
    answers
        .iter()
        .rev()
        .find_map(|message| match message {
            ServerMessage::ActionBar(bar) => Some(bar.clone()),
            _ => None,
        })
        .expect("an action bar among the answers")
}

fn change(change: BarChange) -> ClientMessage {
    ClientMessage::ActionBar(change)
}

#[test]
fn a_new_character_is_told_heal_then_attack() {
    assert_eq!(bar(&TestClient::new().welcome), ActionBar::default());
}

#[test]
fn changes_are_made_by_the_rules_and_refusals_put_right() {
    let mut test = TestClient::new();
    let moved = test.send(&[
        change(BarChange::Swap { a: 1, b: 2 }),
        change(BarChange::ToggleAuto { slot: 2 }),
    ]);
    let attack = Some(BarSlot {
        thing: Ability::Attack.into(),
        auto: true,
    });
    assert_eq!(bar(&moved).slots[2], attack);
    // A human knows no Night Vision, and it is passive anyway.
    let refused = test.send(&[change(BarChange::Place {
        slot: 0,
        ability: Ability::NightVision,
    })]);
    assert_eq!(
        bar(&refused).slots[0].and_then(BarSlot::ability),
        Some(Ability::Heal)
    );
    assert_eq!(bar(&refused).slots[2], attack);
}

#[test]
fn kept_in_the_save() {
    let folder = TestFolder::new();
    let seed = WorldSeed::new(1);
    let open = || ServerWorld::open(seed, Storage::in_folder(&folder.0, seed.id())).unwrap();
    let mut test = TestClient::of(open());
    test.send(&[
        change(BarChange::Swap { a: 1, b: 9 }),
        change(BarChange::PlaceItem {
            slot: 4,
            item: ItemKind::Bread,
        }),
    ]);
    test.leave();
    let again = TestClient::of(open());
    let slots = bar(&again.welcome).slots;
    assert_eq!(slots[9].and_then(BarSlot::ability), Some(Ability::Attack));
    assert_eq!(slots[4], Some(BarSlot::new(ItemKind::Bread)));
}

#[test]
fn food_goes_on_the_bar_but_never_by_itself_and_nothing_else_does() {
    let mut test = TestClient::new();
    let placed = test.send(&[
        change(BarChange::PlaceItem {
            slot: 3,
            item: ItemKind::DwarvenBread,
        }),
        change(BarChange::ToggleAuto { slot: 3 }),
    ]);
    // Placed for a human too, but never used by itself.
    assert_eq!(
        bar(&placed).slots[3],
        Some(BarSlot::new(ItemKind::DwarvenBread))
    );
    let refused = test.send(&[change(BarChange::PlaceItem {
        slot: 5,
        item: ItemKind::Ring,
    })]);
    assert_eq!(bar(&refused).slots[5], None);
}
