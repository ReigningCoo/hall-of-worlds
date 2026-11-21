//! A cooking order checked against what is carried and the kitchen, how long
//! a run takes, and liquids drawn.

use super::*;
use crate::crafting_bench::town_tools;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::ItemForm;
use crate::livestock::Livestock;
use crate::potion::{BottleForm, Liquid, Water};
use crate::pour::Held;
use crate::vessel::{VesselForm, VesselKind};

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        (inventory.give(item, StackLimits::default(), stand_ins)).unwrap();
    }
    inventory
}

fn eggs(count: u32) -> Item {
    let hen = Livestock::built_in().get("chicken").unwrap();
    crate::farmyard::new_egg(hen, count, Id::for_test(1), None).unwrap()
}

fn jug(ml: u32) -> Item {
    let mut jug = Item::new(Id::for_test(2), ItemKind::Vessel);
    let form = VesselForm::new(VesselKind::Jug).holding(Some(Liquid::Water(Water::Plain)), ml);
    jug.form = Some(ItemForm::Vessel(form));
    jug
}

fn bottles(count: u32) -> Item {
    let mut bottle = Item::new(Id::for_test(3), ItemKind::Bottle);
    let water = Some(Liquid::Water(Water::Plain));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(water, 3)));
    bottle.count = count;
    bottle
}

fn order(tool: Tool, inputs: Vec<OrderInput>) -> CookOrder {
    CookOrder {
        bench: None,
        rows: 1,
        sources: Vec::new(),
        steps: vec![OrderStep {
            tool,
            column: 0,
            row: 0,
            height: 1,
            inputs,
        }],
        culture: None,
        count: 1,
    }
}

fn town() -> Kitchen<'static> {
    Kitchen {
        heat: Heat::Hearth,
        oven: true,
        tools: town_tools(),
        storage: &[],
        water_faced: false,
    }
}

fn check(
    carried: &Inventory,
    order: &CookOrder,
    kitchen: Kitchen,
) -> Result<Checked, OrderRefused> {
    let skills = CookingSkills::default();
    carried.check_cooking(
        order,
        kitchen,
        Ingredients::built_in(),
        Dishes::built_in(),
        skills,
    )
}

fn egg_in(count: u32) -> OrderInput {
    OrderInput::Food {
        stack: Id::for_test(1),
        count,
    }
}

#[test]
fn an_egg_fries_in_a_towns_pan() {
    let carried = carrying(vec![eggs(2)]);
    let checked = check(&carried, &order(Tool::Pan, vec![egg_in(1)]), town()).unwrap();
    assert_eq!(checked.planned.outputs.len(), 1);
    assert_eq!(checked.planned.heat, Heat::Hearth);
    let Portion::Food { grams, .. } = &checked.planned.outputs[0].portions[0] else {
        panic!("an egg");
    };
    assert_eq!(*grams, carried.find(Id::for_test(1)).unwrap().own_weight());
}

#[test]
fn tools_and_heat_must_be_to_hand() {
    let carried = carrying(vec![eggs(2)]);
    let fry = order(Tool::Pan, vec![egg_in(1)]);
    let cold = Kitchen::default();
    assert_eq!(
        check(&carried, &fry, cold),
        Err(OrderRefused::Tool(Tool::Pan))
    );
    let with_pan = carrying(vec![eggs(2), Item::new(Id::for_test(9), ItemKind::Pan)]);
    let refused = check(&with_pan, &fry, cold);
    assert_eq!(
        refused,
        Err(OrderRefused::Rules(Refused::NoHeat { step: 0 }))
    );
    let fire = Kitchen {
        heat: Heat::Fire,
        ..cold
    };
    assert!(check(&with_pan, &fry, fire).is_ok());
    let bake = order(Tool::Oven, vec![egg_in(1)]);
    assert_eq!(
        check(&with_pan, &bake, fire),
        Err(OrderRefused::Tool(Tool::Oven))
    );
    assert!(check(&with_pan, &bake, town()).is_ok());
}

#[test]
fn what_is_used_must_be_carried() {
    let carried = carrying(vec![eggs(2)]);
    let three = order(Tool::Pan, vec![egg_in(3)]);
    assert_eq!(check(&carried, &three, town()), Err(OrderRefused::TooFew));
    let none = order(Tool::Pan, vec![egg_in(0)]);
    assert_eq!(check(&carried, &none, town()), Err(OrderRefused::Food));
    let mut never = order(Tool::Pan, vec![egg_in(1)]);
    never.count = 0;
    assert_eq!(check(&carried, &never, town()), Err(OrderRefused::Count));
}

#[test]
fn water_comes_from_any_source() {
    let carried = carrying(vec![eggs(2), jug(500), bottles(2)]);
    let boil = |inputs: Vec<OrderInput>| order(Tool::Cauldron, [vec![egg_in(1)], inputs].concat());
    let jug_in = |ml| OrderInput::Liquid {
        stack: Id::for_test(2),
        ml,
    };
    let bottle_in = |ml| OrderInput::Liquid {
        stack: Id::for_test(3),
        ml,
    };
    assert!(check(&carried, &boil(vec![jug_in(300)]), town()).is_ok());
    let too_much = boil(vec![jug_in(300), jug_in(300)]);
    assert_eq!(
        check(&carried, &too_much, town()),
        Err(OrderRefused::TooFew)
    );
    assert!(check(&carried, &boil(vec![bottle_in(200)]), town()).is_ok());
    assert_eq!(
        check(&carried, &boil(vec![bottle_in(300)]), town()),
        Err(OrderRefused::TooFew)
    );
    assert_eq!(
        check(&carried, &boil(vec![bottle_in(50)]), town()),
        Err(OrderRefused::Liquid)
    );
    let faced = boil(vec![OrderInput::WaterFaced { ml: 400 }]);
    assert_eq!(
        check(&carried, &faced, town()),
        Err(OrderRefused::NoWaterFaced)
    );
    let at_water = Kitchen {
        water_faced: true,
        ..town()
    };
    assert!(check(&carried, &faced, at_water).is_ok());
}

#[test]
fn liquids_are_drawn_off() {
    let jug = drawn(&jug(500), 300);
    assert_eq!(Held::of(&jug).unwrap().ml, 200);
    let empty = drawn(&jug, 200);
    assert_eq!(Held::of(&empty).unwrap().liquid, None);
    let bottle = drawn(&bottles(1), 100);
    assert_eq!(Held::of(&bottle).unwrap().liquid, None);
}

#[test]
fn a_join_takes_half_again() {
    let skills = CookingSkills::default();
    let mut steps = order(Tool::Cauldron, Vec::new()).steps;
    assert_eq!(run_seconds(&steps, skills), 30);
    steps[0].height = 2;
    assert_eq!(run_seconds(&steps, skills), 45);
    steps.push(OrderStep {
        tool: Tool::Knife,
        column: 1,
        row: 0,
        height: 1,
        inputs: Vec::new(),
    });
    assert_eq!(run_seconds(&steps, skills), 55);
    assert_eq!(heat_seconds(&steps, skills), 45);
}

/// Wheat dough as an intermediate item, id 50.
fn dough() -> Item {
    let flour = Portion::Food {
        source: Source::Vegetable,
        key: "wheat".into(),
        grams: 300,
        quality: None,
    };
    let process = Process {
        rows: 1,
        sources: Vec::new(),
        steps: vec![Step::at(
            Tool::Bowl,
            0,
            0,
            vec![flour, Portion::Water { ml: 150 }],
        )],
        heat: Heat::Cold,
    };
    let ingredients = Ingredients::built_in();
    let skills = CookingSkills::default();
    let planned = plan(&process, ingredients, Dishes::built_in(), skills).unwrap();
    let quality = crate::item::Quality::Common;
    let item = crate::food::food_item(
        &planned.outputs[0],
        quality,
        None,
        ingredients,
        Id::for_test(50),
        None,
    );
    assert_eq!(item.kind, ItemKind::Intermediate);
    item
}

#[test]
fn a_row_starts_from_an_intermediate_carried_or_stored() {
    let mut cut = order(Tool::Knife, Vec::new());
    cut.sources = vec![Some(Id::for_test(50))];
    let nothing = carrying(Vec::new());
    assert_eq!(check(&nothing, &cut, town()), Err(OrderRefused::Source));
    let carried = carrying(vec![dough()]);
    let checked = check(&carried, &cut, town()).unwrap();
    assert!(
        checked.planned.outputs[0]
            .history
            .tools
            .starts_with(&[Tool::Bowl, Tool::Knife])
    );
    let stored = [dough()];
    let range = Kitchen {
        storage: &stored,
        ..town()
    };
    assert!(check(&nothing, &cut, range).is_ok());
}
