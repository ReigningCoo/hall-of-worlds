//! The long processes (cooking plan PR 12): koji, moromi, miso, cheese, their
//! ageing, and moromi's soy sauce.

use super::*;
use crate::cooking::{
    CookingSkills, Dishes, Heat, Ingredients, Portion, Process, Source, Step, Tool, plan,
};
use crate::fermentation::GAME_DAY_SECS;
use crate::id::Id;
use crate::item::{Item, Quality};
use crate::potion::{Liquid, Pressing};
use crate::yeast::Strain;

const DAY: u32 = GAME_DAY_SECS as u32;

fn food(source: Source, key: &str, grams: u32) -> Portion {
    Portion::Food {
        source,
        key: key.into(),
        grams,
        quality: None,
    }
}

fn salt(grams: u32) -> Portion {
    food(Source::Pantry, "salt", grams)
}

/// What one step of `tool` with `added` makes, started from `from` if given.
fn cook(from: Option<&Item>, tools: Vec<(Tool, Vec<Portion>)>, culture: Option<Culture>) -> Item {
    let process = Process {
        rows: 1,
        sources: vec![from.map(|item| item.food().unwrap().carried())],
        steps: (tools.into_iter().enumerate())
            .map(|(at, (tool, portions))| Step::at(tool, at as u8, 0, portions))
            .collect(),
        heat: Heat::Hearth,
    };
    let ingredients = Ingredients::built_in();
    let planned = plan(
        &process,
        ingredients,
        Dishes::built_in(),
        CookingSkills::default(),
    );
    let output = &planned.unwrap().outputs[0];
    food_item(
        output,
        Quality::Common,
        culture,
        ingredients,
        Id::NOBODY,
        None,
    )
}

fn kind_of(item: &Item) -> Option<Rest> {
    Some(item.food()?.rest.as_ref()?.kind)
}

fn cooked_soybeans(culture: Option<Culture>) -> Item {
    let boil = vec![
        food(Source::Vegetable, "soybean", 1000),
        Portion::Water { ml: 1500 },
    ];
    cook(
        None,
        vec![(Tool::Cauldron, boil), (Tool::Sieve, vec![])],
        culture,
    )
}

fn koji() -> Item {
    let koji = Some(Culture {
        strain: Strain::Koji,
        quality: Quality::Rare,
    });
    let mut beans = cooked_soybeans(koji);
    beans.rest_food(KOJI_GROWS, 0, CARRIED_CELSIUS);
    beans
}

#[test]
fn soybeans_grow_koji_best_with_a_koji_culture() {
    let mut wild = cooked_soybeans(None);
    assert_eq!(kind_of(&wild), Some(Rest::CookedSoybeans));
    wild.rest_food(KOJI_GROWS, 0, CARRIED_CELSIUS);
    assert_eq!(kind_of(&wild), Some(Rest::CookedSoybeans), "wild is slower");
    wild.rest_food(KOJI_GROWS, 0, CARRIED_CELSIUS);
    assert_eq!(
        (kind_of(&wild), wild.quality),
        (Some(Rest::Koji), Some(Quality::Poor))
    );
    let bread = Some(Culture {
        strain: Strain::Bread,
        quality: Quality::Rare,
    });
    let mut yeasted = cooked_soybeans(bread);
    yeasted.rest_food(KOJI_GROWS * 3 / 2, 0, CARRIED_CELSIUS);
    assert_eq!(
        (kind_of(&yeasted), yeasted.quality),
        (Some(Rest::Koji), Some(Quality::Common)),
        "ordinary yeast works"
    );
    let koji = koji();
    assert_eq!(
        (kind_of(&koji), koji.quality),
        (Some(Rest::Koji), Some(Quality::Uncommon))
    );
    // A koji culture can be taken from it.
    let sample = culture_from(&koji, Id::NOBODY, None).unwrap();
    assert_eq!(sample.yeast().unwrap().strain, Strain::Koji);
}

#[test]
fn salted_koji_is_moromi_when_wet_and_miso_when_not() {
    let koji = koji();
    let wet = vec![salt(200), Portion::Water { ml: 1200 }];
    let moromi = cook(Some(&koji), vec![(Tool::Bowl, wet)], None);
    assert_eq!(kind_of(&moromi), Some(Rest::Moromi));
    let miso = cook(Some(&koji), vec![(Tool::Mortar, vec![salt(120)])], None);
    assert_eq!(kind_of(&miso), Some(Rest::Miso));
    // Too little salt, or heated: neither.
    let bland = cook(Some(&koji), vec![(Tool::Mortar, vec![salt(10)])], None);
    assert!(!matches!(kind_of(&bland), Some(Rest::Miso | Rest::Moromi)));
    let soup = vec![salt(120), Portion::Water { ml: 1000 }];
    let hot = cook(Some(&koji), vec![(Tool::Cauldron, soup)], None);
    assert!(!matches!(kind_of(&hot), Some(Rest::Miso | Rest::Moromi)));
}

#[test]
fn salted_fresh_cheese_ages_into_cheese() {
    let curd = vec![
        food(Source::Milk, "cattle", 1000),
        food(Source::Fruit, "lemon", 30),
    ];
    let warm = cook(
        None,
        vec![(Tool::Cauldron, curd), (Tool::Sieve, vec![])],
        None,
    );
    assert_eq!(kind_of(&warm), Some(Rest::FreshCheese));
    let mut cheese = cook(Some(&warm), vec![(Tool::Bowl, vec![salt(60)])], None);
    assert_eq!(kind_of(&cheese), Some(Rest::Cheese));
    let made = cheese.quality.unwrap();
    cheese.rest_food(15 * DAY, 0, CARRIED_CELSIUS);
    assert_eq!(cheese.quality.unwrap() as u8, made as u8 + 1);
    // Unsalted, it is still fresh cheese.
    let plain = cook(Some(&warm), vec![(Tool::Bowl, vec![])], None);
    assert_ne!(kind_of(&plain), Some(Rest::Cheese));
}

#[test]
fn ageing_raises_quality_up_to_three_steps() {
    let ages = Rest::Moromi.curve();
    assert_eq!(ages.quality(Quality::Common, 30 * DAY - 1), Quality::Common);
    assert_eq!(ages.quality(Quality::Common, 30 * DAY), Quality::Uncommon);
    assert_eq!(ages.quality(Quality::Common, 900 * DAY), Quality::Epic);
    let koji = koji();
    let wet = vec![salt(200), Portion::Water { ml: 1200 }];
    let mut moromi = cook(Some(&koji), vec![(Tool::Bowl, wet)], None);
    let made = moromi.quality.unwrap();
    // Cold slows it: at 10 °C half the pace.
    moromi.rest_food(30 * DAY, 0, 10);
    assert_eq!(moromi.quality, Some(made));
    moromi.rest_food(60 * DAY, 0, CARRIED_CELSIUS);
    assert!(moromi.quality.unwrap() as u8 >= made as u8 + 2);
}

#[test]
fn moromi_presses_into_soy_sauce_by_its_soybeans() {
    let koji = koji();
    let wet = vec![salt(200), Portion::Water { ml: 1200 }];
    let moromi = cook(Some(&koji), vec![(Tool::Bowl, wet)], None);
    assert_eq!(soy_sauce_bottles(&moromi), 1000 / SOY_GRAMS_PER_BOTTLE);
    assert_eq!(soy_sauce_bottles(&koji), 0, "only moromi");
    let herbs = crate::herb::Herbs::built_in();
    let pressing = Pressing::of(&moromi, &herbs);
    assert_eq!(pressing, Some(Pressing::Moromi));
    assert_eq!(pressing.unwrap().liquid(&herbs), Some(Liquid::SoySauce));
    // Soy sauce seasons a dish as a food of its own.
    let ingredients = Ingredients::built_in();
    let def = ingredients.of_liquid(&Liquid::SoySauce).unwrap();
    assert_eq!(
        (def.source, def.key.as_str()),
        (Source::Pantry, "soy_sauce")
    );
}
