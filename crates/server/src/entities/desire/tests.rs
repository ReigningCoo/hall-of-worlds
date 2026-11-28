//! Basic desire (`desire`): mobs going for wanted things on the ground and
//! taking them.

use shared::behaviour::Desire;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::spot::Spot;

use super::super::Doing;
use super::super::choice::Choice;
use super::super::tests::{beside_a_mob, player, run, the_mob};
use super::{bits, holds, take_score};
use crate::connections::tests::TestClient;
use crate::storage::SiteId;

#[test]
fn bits_and_what_a_bag_holds() {
    assert_eq!(bits(&[]), 0);
    assert_eq!(bits(&[Desire::Food]), 1);
    assert_eq!(bits(&[Desire::Weapons, Desire::Coins]), 0b110);
    let bag = [
        Item::new(Id::for_test(1), ItemKind::Stone),
        Item::new(Id::for_test(2), ItemKind::SilverCoin),
    ];
    assert_eq!(holds(&bag), 0b010);
    assert_eq!(holds(&bag[..1]), 0);
}

#[test]
fn nearer_and_more_wanted_scores_higher() {
    assert_eq!(take_score(80, 0), 60);
    assert!(take_score(80, 2) > take_score(80, 5));
    assert!(take_score(80, 2) > take_score(30, 2));
    assert!(take_score(0, 30) <= 0);
}

/// The level the player and the mob are on, and the ground there.
fn place(test: &TestClient) -> (SiteId, u32) {
    *test.world.populations.0.keys().next().unwrap()
}

/// Lays these down at `cell`'s middle.
fn lay(test: &mut TestClient, cell: (i64, i64), items: Vec<Item>) -> Spot {
    let (site, index) = place(test);
    let at = Spot::middle_of(cell);
    let level = test.world.site_mut(site).levels.entry(index).or_default();
    level.ground.entry(at).or_default().extend(items);
    at
}

fn lying(test: &mut TestClient, at: Spot) -> Vec<ItemKind> {
    let (site, index) = place(test);
    let level = test.world.site(site).levels.get(&index);
    let pile = level.and_then(|level| level.ground.get(&at));
    pile.map_or_else(Vec::new, |pile| pile.iter().map(|item| item.kind).collect())
}

/// A kobold (wanting food, coins and weapons) 4 cells east of a player it
/// pays no heed to (god mode), with a bag between them.
fn kobold_and_bag(items: Vec<Item>) -> (TestClient, Spot) {
    let mut test = beside_a_mob(CreatureKind::Kobold, 4);
    test.client.god_mode = true;
    let (x, y) = player(&test);
    let at = lay(&mut test, (x + 2, y), items);
    (test, at)
}

#[test]
fn a_mob_goes_for_what_it_wants_takes_it_and_goes_home() {
    let coins = Item::new(Id::for_test(7), ItemKind::GoldCoin);
    let stone = Item::new(Id::for_test(8), ItemKind::Stone);
    let (mut test, at) = kobold_and_bag(vec![stone, coins]);
    run(&mut test, 1);
    let mob = the_mob(&test);
    assert!(
        matches!(mob.doing, Doing::Taking { at: bag, .. } if bag == at),
        "{:?}",
        mob.doing
    );
    assert_eq!(mob.scores.chosen, Some(Choice::Take(at)));
    run(&mut test, 200);
    // The coin taken, the stone left.
    assert_eq!(lying(&mut test, at), [ItemKind::Stone]);
    let mob = the_mob(&test);
    let (site, index) = place(&test);
    let level = &test.world.site(site).levels[&index];
    let carried = &level.records[&mob.id].carried;
    assert!(carried.iter().any(|item| item.id == Id::for_test(7)));
    // Nothing more it wants: it goes for it no more.
    assert!(
        !matches!(mob.doing, Doing::Taking { .. }),
        "{:?}",
        mob.doing
    );
}

#[test]
fn a_bag_left_empty_goes() {
    let dagger = Item::new(Id::for_test(9), ItemKind::Dagger);
    let (mut test, at) = kobold_and_bag(vec![dagger]);
    run(&mut test, 200);
    let (site, index) = place(&test);
    let level = &test.world.site(site).levels[&index];
    assert!(!level.ground.contains_key(&at));
}

#[test]
fn nothing_wanted_is_left_alone() {
    let stone = Item::new(Id::for_test(8), ItemKind::Stone);
    let (mut test, at) = kobold_and_bag(vec![stone]);
    run(&mut test, 50);
    assert!(!matches!(the_mob(&test).doing, Doing::Taking { .. }));
    assert_eq!(lying(&mut test, at), [ItemKind::Stone]);
}

#[test]
fn an_enemy_in_sight_comes_first() {
    let coins = Item::new(Id::for_test(7), ItemKind::GoldCoin);
    let (mut test, at) = kobold_and_bag(vec![coins]);
    test.client.god_mode = false;
    run(&mut test, 1);
    let mob = the_mob(&test);
    assert!(matches!(mob.doing, Doing::Going { .. }), "{:?}", mob.doing);
    let take = mob
        .scores
        .options
        .iter()
        .find(|one| one.choice == Choice::Take(at));
    assert!(take.is_some(), "{:?}", mob.scores);
}
