//! An Alchemy order checked against what is carried, the Paste item, the
//! sub-skills learned on carrying an apparatus, and how long a run takes.

use super::*;
use crate::herb::{Herbs, Stat};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Contents, Dose, Liquid, Oil, Water};
use crate::skill::{SkillId, Skills};

fn herb(herbs: &Herbs, key: &str, count: u32, id: u128) -> Item {
    let def = herbs.get(key).expect("a herb");
    crate::herb::new_item(def, count, Id::for_test(id), None)
}

fn bottle(liquid: Liquid, count: u32, id: u128) -> Item {
    let mut bottle = Item::new(Id::for_test(id), ItemKind::Bottle);
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(Some(liquid), 3)));
    bottle.count = count;
    bottle
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        inventory
            .give(item, StackLimits::default(), stand_ins)
            .unwrap();
    }
    inventory
}

/// Two built-in herbs' keys.
fn two_herbs(herbs: &Herbs) -> (String, String) {
    let mut keys = herbs.iter().map(|def| def.key.clone());
    (keys.next().unwrap(), keys.next().unwrap())
}

/// Two herbs (3 and 1), Nut Oil, two Waters, and the apparatus.
fn carried(herbs: &Herbs) -> Inventory {
    let (a, b) = two_herbs(herbs);
    carrying(vec![
        herb(herbs, &a, 3, 1),
        herb(herbs, &b, 1, 2),
        bottle(Liquid::Oil(Oil::Nut), 1, 3),
        bottle(Liquid::Water(Water::Plain), 2, 4),
        Item::new(Id::for_test(5), ItemKind::MortarAndPestle),
        Item::new(Id::for_test(6), ItemKind::Retort),
    ])
}

fn stack_of(carried: &Inventory, key: &str) -> Id {
    let stacks = carried.herb_stacks();
    stacks.iter().find(|s| s.herb == key).unwrap().stack
}

#[test]
fn an_order_is_checked_against_what_is_carried() {
    let herbs = Herbs::built_in();
    let (a, b) = two_herbs(&herbs);
    let carried = carried(&herbs);
    let skills = AlchemySkills::default();
    let oil = carried.oil_stacks()[0].stack;
    let water = carried.water_stacks()[0].stack;
    let mortar = OrderStep::Mortar {
        herbs: vec![stack_of(&carried, &a), stack_of(&carried, &b)],
        oil: Some(oil),
    };
    let order = AlchemyOrder {
        source: None,
        steps: vec![
            mortar.clone(),
            OrderStep::Retort {
                liquids: vec![water, water],
            },
        ],
        count: 1,
    };
    let checked = carried.check_alchemy(&order, &herbs, skills).unwrap();
    assert_eq!(checked.herbs, vec![a.clone(), b.clone()]);
    assert_eq!(checked.planned.practised.len(), 2);
    // Three waters from a stack of two is too many.
    let thirsty = AlchemyOrder {
        steps: vec![
            mortar.clone(),
            OrderStep::Retort {
                liquids: vec![water, water, water],
            },
        ],
        ..order.clone()
    };
    let refused = carried.check_alchemy(&thirsty, &herbs, skills);
    assert_eq!(refused.unwrap_err(), OrderRefused::TooFew);
    let none = AlchemyOrder {
        count: 0,
        ..order.clone()
    };
    let refused = carried.check_alchemy(&none, &herbs, skills);
    assert_eq!(refused.unwrap_err(), OrderRefused::Count);
    let twice = AlchemyOrder {
        steps: vec![OrderStep::Mortar {
            herbs: vec![stack_of(&carried, &a), stack_of(&carried, &a)],
            oil: None,
        }],
        ..order.clone()
    };
    let refused = carried.check_alchemy(&twice, &herbs, skills);
    assert_eq!(refused.unwrap_err(), OrderRefused::Rules(Refused::SameHerb));
    let alembic = AlchemyOrder {
        steps: vec![mortar, OrderStep::Alembic],
        ..order
    };
    let refused = carried.check_alchemy(&alembic, &herbs, skills);
    let not_built = OrderRefused::Rules(Refused::NotBuilt(Apparatus::Alembic));
    assert_eq!(refused.unwrap_err(), not_built);
}

#[test]
fn an_apparatus_must_be_carried() {
    let herbs = Herbs::built_in();
    let (a, _) = two_herbs(&herbs);
    let carried = carrying(vec![herb(&herbs, &a, 1, 1)]);
    let order = AlchemyOrder {
        source: None,
        steps: vec![OrderStep::Mortar {
            herbs: vec![Id::for_test(1)],
            oil: None,
        }],
        count: 1,
    };
    let refused = carried.check_alchemy(&order, &herbs, AlchemySkills::default());
    let missing = OrderRefused::Apparatus(Apparatus::Mortar);
    assert_eq!(refused.unwrap_err(), missing);
}

#[test]
fn a_stored_paste_can_start_a_process() {
    let herbs = Herbs::built_in();
    let contents = Contents::new([Dose {
        stat: Stat::Strength,
        step: 8,
    }]);
    let paste = new_mixture(
        Form::Paste,
        contents.clone(),
        Quality::Rare,
        2,
        Id::for_test(7),
    );
    assert_eq!(paste.kind, ItemKind::Paste);
    assert_eq!(paste.quality, Some(Quality::Rare));
    assert_eq!(paste.mixture(), Some((Form::Paste, &contents)));
    let mut carried = carried(&herbs);
    carried
        .give(paste, StackLimits::default(), &mut crate::id::stand_ins())
        .unwrap();
    let water = carried.water_stacks()[0].stack;
    let order = AlchemyOrder {
        source: Some(Id::for_test(7)),
        steps: vec![OrderStep::Retort {
            liquids: vec![water],
        }],
        count: 1,
    };
    let checked = carried.check_alchemy(&order, &herbs, AlchemySkills::default());
    let planned = checked.unwrap().planned;
    assert!(matches!(
        planned.product,
        Product::Potion { bottles: 1, .. }
    ));
    // An herb stack isn't a source.
    let wrong = AlchemyOrder {
        source: Some(Id::for_test(1)),
        ..order
    };
    let refused = carried.check_alchemy(&wrong, &herbs, AlchemySkills::default());
    assert_eq!(refused.unwrap_err(), OrderRefused::Source);
}

#[test]
fn carrying_an_apparatus_teaches_its_sub_skill() {
    let mut skills = Skills::default();
    let grinding = SkillId::find(GRINDING).unwrap();
    let infusing = SkillId::find(INFUSING).unwrap();
    assert!(!learn_sub_skills(&Inventory::default(), &mut skills));
    let mortar = carrying(vec![Item::new(Id::for_test(1), ItemKind::MortarAndPestle)]);
    assert!(learn_sub_skills(&mortar, &mut skills));
    assert!(skills.knows(grinding));
    assert!(!skills.knows(infusing));
    assert!(!learn_sub_skills(&mortar, &mut skills), "learned once");
}

#[test]
fn a_run_takes_each_steps_time() {
    let untrained = AlchemySkills::default();
    let steps = [Apparatus::Mortar, Apparatus::Retort];
    assert_eq!(run_seconds(&steps, untrained), 25);
    let mut trained = untrained;
    trained.sub = [10_000; Apparatus::ALL.len()];
    assert_eq!(run_seconds(&steps, trained), 17);
    assert!(run_ticks(&steps, trained) < run_ticks(&steps, untrained));
}
