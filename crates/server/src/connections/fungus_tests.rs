//! Fungi through a connected client: Give Item's fungus, eating one raw,
//! cutting a piece into servings, and a poison eaten.

use shared::bonus::Source;
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::fermentation::{Base, Drink, Sweetness};
use shared::fungus::FungusTag;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::potion::Liquid;
use shared::protocol::{ClientMessage, ServerMessage};

use super::combat::die_of_poison;
use super::tests::TestClient;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

fn give(key: &str, count: u32) -> ClientMessage {
    ClientMessage::DevGiveItem(shared::protocol::DevGive {
        kind: ItemKind::Fungus,
        size: None,
        magical: false,
        quality: Quality::Common,
        set: false,
        favour: None,
        seed: Some(3),
        count,
        identical: true,
        request: shared::item_form::RollRequest::NONE,
        herb: Some(key.to_owned()),
        to: None,
    })
}

fn carried(test: &TestClient) -> Vec<&Item> {
    let items = test.client.inventory.items.iter();
    items
        .map(|placed| &placed.item)
        .filter(|item| item.kind == ItemKind::Fungus)
        .collect()
}

fn has(test: &TestClient, source: Source) -> bool {
    let effects = test.client.sheet.effects.0.iter();
    effects.into_iter().any(|effect| effect.source == source)
}

fn eat_first(test: &mut TestClient) -> Id {
    let item: Id = carried(test)[0].id;
    test.send(&[ClientMessage::Consume { item }]);
    item
}

#[test]
fn eating_a_fungus_raw_heals_and_one_goes() {
    let mut test = TestClient::new();
    test.send(&[give("button_mushroom", 2)]);
    let found = carried(&test)[0].fungus().map(|form| form.fungus.clone());
    assert_eq!(found.as_deref(), Some("button_mushroom"));
    let stack = eat_first(&mut test);
    assert_eq!(test.client.inventory.find(stack).map_or(0, Item::count), 1);
    assert!(has(
        &test,
        Source::EatenFungus(FungusTag::of("button_mushroom"))
    ));
}

#[test]
fn a_giant_puffball_is_cut_and_its_servings_kept() {
    let mut test = TestClient::new();
    test.send(&[give("giant_puffball", 1)]);
    let puffball = eat_first(&mut test);
    assert!(test.client.inventory.find(puffball).is_none(), "cut");
    let left = carried(&test);
    assert_eq!(left.len(), 1);
    assert!(left[0].fungus().is_some_and(|form| form.serving));
    assert_eq!(left[0].count, 9);
}

#[test]
fn a_death_cap_poisons_later_unseen_and_kills() {
    let mut test = TestClient::new();
    test.send(&[give("death_cap", 1)]);
    eat_first(&mut test);
    let tag = FungusTag::of("death_cap");
    assert!(!has(&test, Source::FungusPoison(tag)), "not yet");
    assert!(!has(&test, Source::EatenFungus(tag)), "never eaten raw");
    assert_eq!(test.client.sheet.later.0.len(), 2);
    // No Healing and never eaten before: hidden from the player.
    assert!(test.client.sheet.as_sent().later.is_empty());
    // The worst comes after the sickness and the easing, and kills.
    let worst = (test.client.sheet.later.0.iter())
        .map(|p| p.effect.start)
        .max()
        .unwrap();
    let now = test.world.action_clock.ticks();
    test.world.action_clock.advance(worst - now + 60 * SECOND);
    let now = test.world.action_clock.ticks();
    assert!(test.client.keep_vitals(now));
    assert!(test.client.sheet.fungi.knows_poison("death_cap"));
    assert!(die_of_poison(&mut test.client, &mut test.world, now));
    assert!(!has(&test, Source::FungusPoison(tag)), "dying cures it");
    assert!(test.client.sheet.later.is_empty());
    let told: Vec<_> = std::iter::from_fn(|| test.end.receive().unwrap()).collect();
    assert!(told.contains(&ServerMessage::Killed { permanent: false }));
}

#[test]
fn an_ink_cap_then_ale_brings_sickness() {
    let mut test = TestClient::new();
    test.send(&[give("common_ink_cap", 1)]);
    eat_first(&mut test);
    let tag = FungusTag::of("common_ink_cap");
    assert!(has(&test, Source::FungusWindow(tag)));
    let now = test.world.action_clock.ticks();
    test.client.drink_units(&ale(), 500, now);
    assert!(has(&test, Source::FungusPoison(tag)));
}

/// 5% ale.
fn ale() -> Liquid {
    Liquid::Drink(Box::new(Drink {
        base: Base::Grain("barley".into()),
        strength: 50,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    }))
}
