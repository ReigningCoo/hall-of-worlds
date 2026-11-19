//! Drinking alcohol through a connected client: a tankard of ale gives units,
//! Mana and a stage that slows; the stage changes as the units wear off; and
//! becoming very drunk collapses, spending the energy it lasts.

use shared::alcohol::{COLLAPSE_TICKS, Stage, UNIT_TICKS};
use shared::bonus::Source;
use shared::fermentation::{Base, Drink, Sweetness};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::Liquid;
use shared::protocol::ClientMessage;
use shared::vessel::{VesselForm, VesselKind};

use crate::connections::tests::TestClient;

fn ale(strength: u32) -> Liquid {
    Liquid::Drink(Box::new(Drink {
        base: Base::Grain("barley".into()),
        strength,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    }))
}

/// A waterskin (2 litres) of `liquid`, given to the test client as `id`.
fn give_skin(test: &mut TestClient, id: u128, liquid: Liquid) {
    let skin = VesselForm::new(VesselKind::Waterskin).full_of(liquid);
    let item = Item {
        form: Some(ItemForm::Vessel(skin)),
        ..Item::new(Id::for_test(id), ItemKind::Vessel)
    };
    let new_id = &mut shared::id::stand_ins();
    assert!(
        test.client
            .inventory
            .give(item, StackLimits::NONE, new_id)
            .is_ok()
    );
}

fn drink(test: &mut TestClient, id: u128, servings: usize) {
    for _ in 0..servings {
        test.send(&[ClientMessage::Consume {
            item: Id::for_test(id),
        }]);
    }
}

#[test]
fn ale_gives_units_mana_and_a_stage_that_slows() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let speed = test.client.speed;
    give_skin(&mut test, 1, ale(60));
    // Two servings of 6% ale: 3 units, tipsy for a 70 kg human.
    drink(&mut test, 1, 2);
    assert_eq!(test.client.sheet.alcohol.milli, 3000);
    assert_eq!(test.client.sheet.alcohol_stage(), Some(Stage::Tipsy));
    assert_eq!(test.client.speed, speed * 67 / 100);
    let sources: Vec<Source> = test
        .client
        .sheet
        .effects
        .0
        .iter()
        .map(|e| e.source)
        .collect();
    assert!(sources.contains(&Source::Drink));
    // A unit wears off: no longer tipsy.
    test.world.action_clock.advance(UNIT_TICKS);
    let now = test.world.action_clock.ticks();
    assert!(test.client.keep_alcohol(now));
    assert_eq!(test.client.sheet.alcohol_stage(), None);
    assert_eq!(test.client.speed, speed);
}

#[test]
fn becoming_very_drunk_collapses() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give_skin(&mut test, 1, ale(100));
    // 10% ale, 2.5 units a serving: the fifth reaches 12.5 units.
    drink(&mut test, 1, 5);
    assert_eq!(test.client.sheet.alcohol_stage(), Some(Stage::Collapsed));
    let now = test.world.action_clock.ticks();
    assert!(!test.client.energy.ready(now, test.client.speed));
    let over = now + COLLAPSE_TICKS;
    assert!(test.client.energy.ready(over, test.client.speed));
    test.world.action_clock.advance(COLLAPSE_TICKS);
    assert!(test.client.keep_alcohol(over));
    assert_eq!(test.client.sheet.alcohol_stage(), Some(Stage::VeryDrunk));
}
