use shared::cooking::{Recipe, RecipeInput, RecipeStep, Source, Tool, Wanted};
use shared::id::{Id, stand_ins};
use shared::item::{Inventory, Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::livestock::Livestock;
use shared::potion::{BottleForm, Liquid, Water};

use super::super::bench::Bench;
use super::super::kitchen::KitchenNow;
use super::Ghost;

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let ids = &mut stand_ins();
    for item in items {
        (inventory.give(item, StackLimits::default(), ids)).unwrap();
    }
    inventory
}

fn eggs(count: u32) -> Item {
    let hen = Livestock::built_in().get("chicken").unwrap();
    shared::farmyard::new_egg(hen, count, Id::for_test(1), None).unwrap()
}

fn water_bottles(count: u32) -> Item {
    let mut bottle = Item::new(Id::for_test(2), ItemKind::Bottle);
    let water = Some(Liquid::Water(Water::Plain));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(water, 3)));
    bottle.count = count;
    bottle
}

/// Three eggs and 200 ml of water in a Bowl.
fn recipe() -> Recipe {
    let inputs = vec![
        RecipeInput::Food {
            foods: vec![Wanted::Any(Source::Egg)],
            count: 3,
        },
        RecipeInput::Water { ml: 200 },
    ];
    Recipe {
        rows: 1,
        sources: Vec::new(),
        steps: vec![RecipeStep {
            tool: Tool::Bowl,
            column: 0,
            row: 0,
            height: 1,
            inputs,
        }],
    }
}

#[test]
fn make_fills_from_what_is_carried_and_ghosts_the_rest() {
    let carried = carrying(vec![eggs(2), water_bottles(3)]);
    let mut bench = Bench::default();
    bench.load(&recipe(), &carried, &KitchenNow::default());
    let step = &bench.steps[0];
    assert_eq!((step.tool, step.column, step.row), (Tool::Bowl, 0, 0));
    assert_eq!(step.foods.iter().map(|f| f.1).sum::<u32>(), 2);
    assert_eq!(step.liquids.iter().map(|l| l.1).sum::<u32>(), 200);
    // One egg short: a ghost, and Start stays off.
    let short = Ghost {
        input: 0,
        missing: 1,
    };
    assert_eq!(step.ghosts(&carried), [short]);
    assert!(bench.missing_any(&carried));
    // Another egg picked up fills it.
    let more = carrying(vec![eggs(3), water_bottles(3)]);
    assert!(bench.top_up(&more, &KitchenNow::default()));
    assert!(!bench.missing_any(&more));
}

#[test]
fn water_faced_gives_water_and_a_ghost_can_be_given_up() {
    let carried = carrying(vec![eggs(1)]);
    let kitchen = KitchenNow {
        water_faced: true,
        ..KitchenNow::default()
    };
    let mut bench = Bench::default();
    bench.load(&recipe(), &carried, &kitchen);
    assert_eq!(bench.steps[0].faced, 200);
    assert!(bench.steps[0].liquids.is_empty());
    let ghosts = bench.steps[0].ghosts(&carried);
    assert_eq!(ghosts.len(), 1);
    // Cooking without the missing eggs.
    bench.give_up(&carried, (0, 0));
    assert!(!bench.missing_any(&carried));
    assert_eq!(bench.steps[0].wanted.len(), 1);
}

#[test]
fn a_stack_dropped_on_a_ghost_goes_in_as_far_as_is_missing() {
    let none = carrying(Vec::new());
    let mut bench = Bench::default();
    bench.load(&recipe(), &none, &KitchenNow::default());
    assert_eq!(bench.steps[0].ghosts(&none).len(), 2);
    let carried = carrying(vec![eggs(5)]);
    let egg = carried.items[0].item.id;
    assert!(bench.ghost_takes(&carried, (0, 0), egg));
    // Not the water's ghost.
    assert!(!bench.ghost_takes(&carried, (0, 1), egg));
    bench.drop_on_ghost(&carried, &KitchenNow::default(), (0, 0), egg);
    assert_eq!(bench.steps[0].foods, [(egg, 3)]);
    assert_eq!(bench.steps[0].ghosts(&carried).len(), 1);
}
