//! Fell Tree and Cut Kindling through a connected client: blows adding up to
//! a felled tree with a stump and a pile of logs, stopped by moving with the
//! damage kept, growing back by world time, and logs split into kindling.

use shared::ability::Ability;
use shared::blade::Hands;
use shared::creature::CreatureKind;
use shared::dungeon::Cell;
use shared::facing::Heading;
use shared::herb::MINUTES_PER_DAY;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::{ItemForm, ItemOrder, make_item};
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::rng::Seed;
use shared::skill::SkillId;
use shared::tree::TreeStage;
use shared::woodcutting::{WOODCUTTING_SKILL, fell_ticks};

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// A one-handed axe with id `n`.
fn axe(n: u128) -> Item {
    let order = ItemOrder::plain(ItemKind::Axe, Quality::Common);
    let mut new_id = || Id::for_test(1000 + n);
    let kind = CreatureKind::Human;
    let mut item = make_item(
        &order,
        kind,
        Id::for_test(n),
        Seed::new(1),
        None,
        &mut new_id,
    );
    if let Some(ItemForm::Blade(form)) = &mut item.form {
        form.hands = Hands::One;
    }
    item
}

/// A player in a castle facing a tree just east of them, holding a one-handed
/// axe: the tree's cell.
fn at_a_tree() -> (TestClient, (i64, i64)) {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let visit = test.client.visit.as_mut().unwrap();
    let (x, y) = visit.room_to_walk_east(2);
    visit.move_for_test((x, y));
    visit.set_cell_for_test((x + 1, y), Cell::Tree);
    let stand_ins = &mut shared::id::stand_ins();
    let inventory = &mut test.client.inventory;
    inventory
        .give(axe(7), StackLimits::NONE, stand_ins)
        .unwrap();
    inventory.wear(Id::for_test(7)).unwrap();
    test.client
        .sheet
        .abilities
        .learn_from(&test.client.inventory);
    test.send(&[ClientMessage::Face(Heading::EAST)]);
    (test, (x + 1, y))
}

fn use_ability(ability: Ability) -> ClientMessage {
    ClientMessage::UseAbility {
        ability,
        target: None,
    }
}

/// How long felling takes this player from nothing, in ticks.
fn whole(test: &TestClient) -> u64 {
    let axe = test.client.inventory.held_axe().unwrap();
    fell_ticks(&test.client.sheet.skills, axe)
}

/// Shares the world's tree changes with the player, as every tick does.
fn share(test: &mut TestClient) {
    crate::sites::grow_trees(std::slice::from_ref(&test.client), &mut test.world);
    crate::sites::share_trees(std::slice::from_mut(&mut test.client), &mut test.world);
}

/// The logs on the ground on the player's level.
fn logs_lying(test: &mut TestClient) -> usize {
    let visit = test.client.visit.as_ref().unwrap();
    let (site, index, _) = visit.plant_key(0, 0);
    let level = test.world.site(site).levels.get(&index).cloned();
    let ground = level.map(|level| level.ground).unwrap_or_default();
    ground
        .values()
        .flat_map(|bag| bag.iter())
        .filter(|item| item.kind == ItemKind::Log)
        .count()
}

fn felled_at(test: &TestClient, (x, y): (i64, i64)) -> Option<TreeStage> {
    test.client.visit.as_ref().unwrap().cells().felled_at(x, y)
}

#[test]
fn blows_fell_the_tree_leaving_a_stump_and_logs() {
    let (mut test, tree) = at_a_tree();
    assert!(test.client.sheet.abilities.knows(Ability::FellTree));
    test.send(&[use_ability(Ability::FellTree)]);
    let whole = whole(&test);
    run(&mut test, whole / 2, false);
    share(&mut test);
    let chopped = received(&mut test).iter().any(|message| {
        matches!(message, ServerMessage::TreeChopped { done, .. } if *done > 0 && *done < 1000)
    });
    assert!(chopped, "onlookers are told of each blow");
    assert_eq!(felled_at(&test, tree), None, "still standing");
    run(&mut test, whole / 2 + 50, false);
    share(&mut test);
    assert_eq!(felled_at(&test, tree), Some(TreeStage::Stump));
    assert!(test.client.fight.pending.is_none(), "done");
    assert!(logs_lying(&mut test) > 0, "a pile of logs by the feller");
    let woodcutting = SkillId::find(WOODCUTTING_SKILL).unwrap();
    assert!(test.client.sheet.skills.practice(woodcutting) > 0);
    let visit = test.client.visit.as_ref().unwrap();
    assert!(
        visit.cells().is_walkable(tree.0, tree.1),
        "the stump is walked over"
    );
    let landed = received(&mut test).iter().any(|message| {
        matches!(
            message,
            ServerMessage::AbilityLanded {
                landed: Landed::Chopped { done: 1000 },
                ..
            }
        )
    });
    assert!(landed);
}

#[test]
fn moving_stops_it_and_the_damage_is_kept() {
    let (mut test, (x, y)) = at_a_tree();
    test.send(&[use_ability(Ability::FellTree)]);
    run(&mut test, 3 * 40 + 2, false);
    test.send(&[ClientMessage::CancelAbility]);
    test.steps(&[(-1, 0)]);
    assert!(test.client.fight.pending.is_none());
    let key = test.client.visit.as_ref().unwrap().plant_key(x, y);
    let damage = crate::sites::saved_tree(&mut test.world, key)
        .damage
        .unwrap();
    assert!(damage.per_mille > 0, "{damage:?}");
    let whole = whole(&test);
    run(&mut test, whole, false);
    assert_eq!(felled_at(&test, (x, y)), None);
}

#[test]
fn a_felled_tree_grows_back_by_world_time_and_is_forgotten_once_grown() {
    let (mut test, tree) = at_a_tree();
    test.send(&[use_ability(Ability::FellTree)]);
    let whole = whole(&test);
    run(&mut test, whole + 50, false);
    share(&mut test);
    let felled = test.world.minutes();
    let day = MINUTES_PER_DAY as f64;
    for (days, stage) in [
        (2.0, Some(TreeStage::Cleared)),
        (6.0, Some(TreeStage::Sapling)),
        (13.0, Some(TreeStage::Young)),
        (20.0, Some(TreeStage::HalfGrown)),
        (27.0, None),
    ] {
        test.world.set_time(Some(felled + days * day), None);
        share(&mut test);
        assert_eq!(felled_at(&test, tree), stage, "after {days} days");
    }
    let key = test
        .client
        .visit
        .as_ref()
        .unwrap()
        .plant_key(tree.0, tree.1);
    let kept = crate::sites::saved_tree(&mut test.world, key);
    assert_eq!(kept, Default::default(), "the seed's tree again");
}

#[test]
fn a_log_is_split_into_kindling() {
    let (mut test, _) = at_a_tree();
    test.send(&[use_ability(Ability::FellTree)]);
    let whole = whole(&test);
    run(&mut test, whole + 50, false);
    // Taken from the pile into the pack, as the inventory would.
    let visit = test.client.visit.as_ref().unwrap();
    let (site, index, _) = visit.plant_key(0, 0);
    let ground = test.world.site(site).levels[&index].ground.clone();
    let log = ground
        .values()
        .flat_map(|bag| bag.iter())
        .find(|item| item.kind == ItemKind::Log)
        .unwrap()
        .id;
    test.send(&[ClientMessage::UseAbility {
        ability: Ability::CutKindling,
        target: Some(log),
    }]);
    assert!(
        test.client.fight.pending.is_some(),
        "a log on the ground named"
    );
    run(&mut test, 20 * 20 + 2, false);
    let kindling = (test.client.inventory.items.iter())
        .find(|placed| placed.item.kind == ItemKind::Kindling)
        .map(|placed| placed.item.count());
    assert!(kindling.is_some_and(|count| count >= 30), "{kindling:?}");
}
