use super::*;
use crate::ability::{Ability, Learned};
use crate::blade::Hands;
use crate::creature::CreatureKind;
use crate::dungeon::{Cell, Level};
use crate::facing::Heading;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::item_form::{ItemForm, ItemOrder, RollRequest, give_form_to, make_item};
use crate::material::Material;
use crate::rng::Seed;
use crate::skill::{SkillId, Skills};
use crate::spot::Spot;

/// A one-handed axe with id `n`, renamed a hatchet or not by its handle.
fn axe(n: u128, quality: Quality) -> Item {
    let order = ItemOrder::plain(ItemKind::Axe, quality);
    let mut new_id = || Id::for_test(1000 + n);
    let seed = Seed::new(n as u64);
    let mut item = make_item(
        &order,
        CreatureKind::Human,
        Id::for_test(n),
        seed,
        None,
        &mut new_id,
    );
    if let Some(ItemForm::Blade(form)) = &mut item.form {
        form.hands = Hands::One;
    }
    item
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (40, 40),
        ..Inventory::default()
    };
    for item in items {
        inventory
            .give(item, StackLimits::NONE, &mut crate::id::stand_ins())
            .expect("room");
    }
    inventory
}

fn skilled(percent: u32) -> Skills {
    let mut skills = Skills::default();
    skills.set_percent(SkillId::find(WOODCUTTING_SKILL).unwrap(), percent);
    skills
}

#[test]
fn only_a_held_one_handed_axe_teaches_and_fells() {
    let mut inventory = carrying(vec![axe(1, Quality::Common)]);
    assert_eq!(inventory.held_axe(), None, "carried, not held");
    assert!(!Ability::FellTree.has_tool(&inventory));
    inventory.wear(Id::for_test(1)).unwrap();
    assert!(inventory.held_axe().is_some());
    assert!(Ability::FellTree.has_tool(&inventory) && Ability::CutKindling.has_tool(&inventory));
    let mut learned = Learned::default();
    assert!(learned.learn_from(&inventory));
    assert!(learned.knows(Ability::FellTree) && learned.knows(Ability::CutKindling));
    assert!(!learned.learn_from(&inventory), "nothing new");

    let mut two_handed = axe(2, Quality::Common);
    if let Some(ItemForm::Blade(form)) = &mut two_handed.form {
        form.hands = Hands::Two;
    }
    assert_eq!(HeldAxe::of(&two_handed), None);
}

#[test]
fn felling_takes_a_minute_down_to_half_and_longer_without_a_hatchet() {
    let ticks_per_second = crate::energy::ACTION_TICKS_PER_SECOND as u64;
    let basic = HeldAxe {
        quality: Quality::Common,
        hatchet: true,
    };
    assert_eq!(fell_ticks(&Skills::default(), basic), 60 * ticks_per_second);
    let poor = HeldAxe {
        quality: Quality::Poor,
        ..basic
    };
    assert_eq!(fell_seconds(&Skills::default(), poor), 60);
    let best = HeldAxe {
        quality: Quality::Artifact,
        hatchet: true,
    };
    assert_eq!(fell_ticks(&skilled(100), best), 30 * ticks_per_second);
    let other = HeldAxe {
        hatchet: false,
        ..basic
    };
    assert_eq!(fell_seconds(&Skills::default(), other), 90);
    assert_eq!(
        fell_seconds(
            &skilled(100),
            HeldAxe {
                hatchet: false,
                ..best
            }
        ),
        45
    );
    let middling = fell_seconds(
        &skilled(10),
        HeldAxe {
            quality: Quality::Rare,
            ..basic
        },
    );
    assert!((35..55).contains(&middling), "{middling}");
}

#[test]
fn damage_is_kept_a_day_then_gone() {
    let whole = 1200;
    let damage = Damage::default().chopped(100, 600, whole);
    assert_eq!(damage.per_mille, 500);
    assert_eq!(damage.ticks_left(100, whole), 600);
    assert_eq!(damage.left(100 + DAMAGE_KEPT_MINUTES - 1), 500);
    assert_eq!(damage.left(100 + DAMAGE_KEPT_MINUTES), 0);
    let more = damage.chopped(200, 600, whole);
    assert!(more.is_felled() && more.ticks_left(200, whole) == 0);
    let late = damage.chopped(100 + DAMAGE_KEPT_MINUTES, 300, whole);
    assert_eq!(late.per_mille, 250, "the old damage had gone");
}

#[test]
fn a_tree_is_felled_only_when_faced_and_near() {
    let mut level = Level::new(0, 0, 10, 10);
    for y in 0..10 {
        for x in 0..10 {
            level.set(x, y, Cell::Ground);
        }
    }
    level.set(5, 5, Cell::Tree);
    let beside = Spot::middle_of((4, 5));
    assert_eq!(tree_faced(&level, beside, Heading::EAST), Some((5, 5)));
    assert_eq!(tree_faced(&level, beside, Heading::WEST), None, "behind");
    let far = Spot::middle_of((2, 5));
    assert_eq!(tree_faced(&level, far, Heading::EAST), None, "too far");
}

#[test]
fn logs_weigh_their_wood_and_split_by_the_kilogram() {
    let oak = LogForm {
        wood: Material::Oak,
    };
    assert_eq!((oak.name(), oak.grams()), ("Oak Log".to_owned(), 63_000));
    assert_eq!(oak.shape(), crate::item::Shape::rectangle(8, 3));
    assert_eq!(
        LogForm {
            wood: Material::Pine
        }
        .grams(),
        45_000
    );
    assert_eq!(kindling_from(&oak), 63);
    let log = new_log(Material::Oak, Id::for_test(1), None);
    assert_eq!(
        (log.name(), log.total_weight()),
        ("Oak Log".to_owned(), 63_000)
    );
    let kindling = new_kindling(Material::HolmOak, 3, Id::for_test(2), None);
    assert_eq!(kindling.name(), "Holm Oak Kindling");
    assert_eq!(kindling.total_weight(), 3000);
}

#[test]
fn the_first_carried_log_is_cut_else_the_first_on_the_ground() {
    let ground = [
        new_kindling(Material::Oak, 1, Id::for_test(9), None),
        new_log(Material::Ash, Id::for_test(8), None),
    ];
    let nothing = carrying(vec![]);
    assert_eq!(log_to_cut(&nothing, None), None, "the ground isn't shown");
    assert_eq!(
        log_to_cut(&nothing, Some(&ground)),
        Some(LogAt::Ground(Id::for_test(8)))
    );
    let logs = vec![
        new_log(Material::Oak, Id::for_test(1), None),
        new_log(Material::Pine, Id::for_test(2), None),
    ];
    let carried = carrying(logs);
    assert_eq!(
        log_to_cut(&carried, Some(&ground)),
        Some(LogAt::Carried(Id::for_test(1)))
    );
}

#[test]
fn cutting_a_log_takes_twenty_seconds_down_to_sixteen() {
    assert_eq!(cut_seconds(&Skills::default()), 20);
    assert_eq!(cut_seconds(&skilled(100)), 16);
}

#[test]
fn give_item_makes_logs_and_kindling_of_any_tree_wood() {
    let give = |kind: ItemKind, material: Option<Material>| {
        let mut item = Item::new(Id::for_test(1), kind);
        let request = RollRequest {
            material,
            ..RollRequest::NONE
        };
        let mut new_id = crate::id::stand_ins();
        give_form_to(&mut item, None, Seed::new(4), &request, &mut new_id).map(|()| item)
    };
    let log = give(ItemKind::Log, Some(Material::Mahogany)).unwrap();
    assert_eq!(log.log().map(|log| log.wood), Some(Material::Mahogany));
    let kindling = give(ItemKind::Kindling, None).unwrap();
    assert!(
        kindling
            .kindling()
            .is_some_and(|form| form.wood.wood().is_some())
    );
    assert!(give(ItemKind::Log, Some(Material::Steel)).is_err());
    assert!(
        give(ItemKind::Log, Some(Material::Wood)).is_err(),
        "not a tree's"
    );
}
