//! Brew's pieces kept by Alchemy: the carried stacks, the bottle turned
//! into a potion, recipes noting what they brewed, and what brewing teaches.

use super::*;
use crate::herb::{HerbKnowledge, Herbs, Stat};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Contents, Dose, Liquid, Oil, Water};

fn contents(doses: &[(Stat, i16)]) -> Contents {
    Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
}

fn water(water: Water, count: u32, id: u128) -> Item {
    let mut bottle = Item::new(Id::for_test(id), ItemKind::Bottle);
    let liquid = Some(Liquid::Water(water));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 7)));
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

#[test]
fn carried_ointments_and_waters_are_offered() {
    let strong = contents(&[(Stat::Strength, 4), (Stat::Will, -1)]);
    let inventory = carrying(vec![
        new_ointment(strong.clone(), 2, Id::for_test(1)),
        water(Water::Plain, 3, 2),
        water(Water::Salt, 1, 3),
    ]);
    let ointments = inventory.ointment_stacks();
    assert_eq!(ointments.len(), 1);
    assert_eq!(ointments[0].count, 2);
    let waters = inventory.water_stacks();
    let kinds: Vec<Water> = waters.iter().map(|stack| stack.water).collect();
    assert_eq!(kinds, vec![Water::Plain, Water::Salt]);
}

#[test]
fn the_water_bottle_holds_the_potion_or_comes_back_empty() {
    let potion = contents(&[(Stat::Strength, 4)]);
    let bottle = brewed_into(water(Water::Moon, 1, 1), Some(potion.clone()));
    let form = bottle.bottle().unwrap();
    assert_eq!(form.liquid, Some(Liquid::Potion(potion.clone())));
    assert_eq!(form.look, 7, "the bottle keeps its look");
    assert_eq!(bottle.quality, Some(quality(&potion)));
    let wasted = brewed_into(water(Water::Moon, 1, 1), None);
    assert!(wasted.bottle().unwrap().is_empty());
    assert_eq!(wasted.quality, None);
}

#[test]
fn recipes_note_what_their_ointment_brewed_into() {
    let made = contents(&[(Stat::Strength, 4)]);
    let mut recipes = Recipes::default();
    recipes.remember(Recipe::new(["a".to_owned()], Oil::Nut, made.clone()));
    let brewed = BrewedWith {
        water: Water::Moon,
        potion: Some(made.clone()),
    };
    recipes.brewed(&contents(&[(Stat::Will, 4)]), brewed.clone());
    assert_eq!(recipes.iter().next().unwrap().brewed, None);
    recipes.brewed(&made, brewed.clone());
    assert_eq!(recipes.iter().next().unwrap().brewed, Some(brewed));
}

#[test]
fn a_potion_teaches_its_herbs_effects_on_its_stats() {
    let herbs = Herbs::built_in();
    let def = herbs.iter().find(|def| def.effects.len() >= 2).unwrap();
    let first = def.effects[0].stat;
    let others = def.effects.iter().skip(1).filter(|e| e.stat != first);
    let other = others.map(|effect| effect.stat).next();
    let ointment = contents(&[(first, 5)]);
    let mut recipes = Recipes::default();
    let recipe = Recipe::new([def.key.clone()], Oil::Seed, ointment.clone());
    recipes.remember(recipe);
    let mut known = HerbKnowledge::default();

    // An ointment no mix remembers teaches nothing.
    let stranger = contents(&[(first, 6)]);
    let learned = learn_from_potion(&mut known, &recipes, &herbs, &stranger, &stranger);
    assert!(!learned);

    let potion = contents(&[(first, 4)]);
    assert!(learn_from_potion(
        &mut known, &recipes, &herbs, &ointment, &potion
    ));
    assert!(known.knows_effect(&def.key, 0));
    if let Some(other) = other {
        let n = def.effects.iter().position(|e| e.stat == other).unwrap();
        assert!(
            !known.knows_effect(&def.key, n),
            "not on the potion's stats"
        );
    }
    // Nothing new the second time.
    assert!(!learn_from_potion(
        &mut known, &recipes, &herbs, &ointment, &potion
    ));
}
