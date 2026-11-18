//! Collect Water through a connected client: learned on a first empty bottle,
//! filling bottles one after another at water faced, and stopped by moving.

use shared::ability::Ability;
use shared::dungeon::Cell;
use shared::facing::Heading;
use shared::item::{Item, ItemKind, Quality};
use shared::potion::{Liquid, WATER_SKILL};
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::skill::SkillId;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a bottle takes untrained, and a few more.
const ONE_BOTTLE: u64 = 10 * 20 + 2;

/// A player in a castle with water just east of them, facing it, carrying
/// `count` empty bottles.
fn at_the_water(count: u32) -> TestClient {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let visit = test.client.visit.as_mut().unwrap();
    let (x, y) = visit.room_to_walk_east(2);
    visit.move_for_test((x, y));
    visit.set_cell_for_test((x + 1, y), Cell::Water);
    test.send(&[
        ClientMessage::Face(Heading::EAST),
        ClientMessage::DevGiveItem(shared::protocol::DevGive {
            kind: ItemKind::Bottle,
            size: None,
            magical: false,
            quality: Quality::Common,
            set: false,
            favour: None,
            seed: Some(3),
            count,
            identical: true,
            request: shared::item_form::RollRequest::NONE,
            herb: None,
            to: None,
        }),
    ]);
    test
}

/// The bottles carried, by what they hold, with how many of each.
fn bottles(test: &TestClient) -> Vec<(Option<Liquid>, u32)> {
    let items = test
        .client
        .inventory
        .items
        .iter()
        .map(|placed| &placed.item);
    items
        .filter(|item| item.kind == ItemKind::Bottle)
        .map(|item: &Item| (item.bottle().unwrap().liquid.clone(), item.count()))
        .collect()
}

fn collect() -> ClientMessage {
    ClientMessage::UseAbility {
        ability: Ability::CollectWater,
        target: None,
    }
}

#[test]
fn bottles_are_filled_one_after_another_and_practise_water() {
    let mut test = at_the_water(2);
    assert!(test.client.sheet.abilities.knows(Ability::CollectWater));
    test.send(&[collect()]);
    run(&mut test, ONE_BOTTLE, false);
    let after_one = bottles(&test);
    assert!(after_one.contains(&(None, 1)), "{after_one:?}");
    run(&mut test, ONE_BOTTLE, false);
    let after_two = bottles(&test);
    assert_eq!(after_two.len(), 1, "{after_two:?}");
    assert!(matches!(after_two[0], (Some(Liquid::Water(_)), 2)));
    let filled = received(&mut test).iter().any(|message| {
        matches!(
            message,
            ServerMessage::AbilityLanded {
                landed: Landed::Filled(_),
                ..
            }
        )
    });
    assert!(filled);
    let water = SkillId::find(WATER_SKILL).unwrap();
    assert!(test.client.sheet.skills.practice(water) > 0);
    assert!(test.client.fight.pending.is_none(), "no empty bottle left");
}

#[test]
fn moving_stops_it() {
    let mut test = at_the_water(1);
    test.send(&[collect()]);
    assert!(test.client.fight.pending.is_some());
    // As the client does: calling it off, then the step.
    test.send(&[ClientMessage::CancelAbility]);
    test.steps(&[(-1, 0)]);
    assert!(test.client.fight.pending.is_none());
    run(&mut test, ONE_BOTTLE, false);
    assert_eq!(bottles(&test), vec![(None, 1)]);
}

#[test]
fn being_harmed_stops_it() {
    let mut test = at_the_water(1);
    test.send(&[collect()]);
    assert!(test.client.fight.pending.is_some());
    test.send(
        &[ClientMessage::DevVitals(shared::protocol::DevVitals::Hurt(
            1,
        ))],
    );
    assert!(test.client.fight.pending.is_none());
    run(&mut test, ONE_BOTTLE, false);
    assert_eq!(bottles(&test), vec![(None, 1)]);
}

#[test]
fn nothing_is_filled_facing_away() {
    let mut test = at_the_water(1);
    test.send(&[ClientMessage::Face(Heading::WEST), collect()]);
    assert!(test.client.fight.pending.is_none());
}
