use super::*;
use crate::cooking::{
    CookingSkills, Dishes, Heat, Ingredients, Portion, Process, Source, Step, Tool, plan,
};
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::rng::Seed;
use crate::yeast::Strain;

const HOUR: u32 = 3600;

fn food(source: Source, key: &str, grams: u32) -> Portion {
    Portion::Food {
        source,
        key: key.into(),
        grams,
        quality: None,
    }
}

fn made(steps: Vec<(Tool, Vec<Portion>)>, quality: Quality, culture: Option<Culture>) -> Item {
    let process = Process {
        rows: 1,
        sources: Vec::new(),
        steps: (steps.into_iter().enumerate())
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
    food_item(output, quality, culture, ingredients, Id::NOBODY, None)
}

fn kind_of(item: &Item) -> Option<Rest> {
    Some(item.food()?.rest.as_ref()?.kind)
}

fn warm_milk(culture: Option<Culture>) -> Item {
    let milk = food(Source::Milk, "cattle", 500);
    made(vec![(Tool::Cauldron, vec![milk])], Quality::Common, culture)
}

#[test]
fn every_sample_is_its_kind() {
    for kind in [ItemKind::Dish, ItemKind::Intermediate] {
        for seed in 0..40 {
            let form = dev_food(kind, Quality::Common, Seed::new(seed)).expect("cooks");
            assert_eq!(
                form.rest.is_some(),
                kind == ItemKind::Intermediate,
                "{form:?}"
            );
        }
    }
}

#[test]
fn outputs_become_intermediates_or_dishes() {
    let flour = || food(Source::Vegetable, "wheat", 300);
    let water = || Portion::Water { ml: 150 };
    let dough = made(
        vec![(Tool::Bowl, vec![flour(), water()])],
        Quality::Common,
        None,
    );
    assert_eq!(
        (dough.kind, kind_of(&dough)),
        (ItemKind::Intermediate, Some(Rest::Dough))
    );
    assert_eq!(dough.food().unwrap().name(), "Wheat Dough");
    let noodles = vec![(Tool::Bowl, vec![flour(), water()]), (Tool::Knife, vec![])];
    let noodles = made(noodles, Quality::Common, None);
    assert_eq!(kind_of(&noodles), Some(Rest::FreshNoodles));
    assert_eq!(kind_of(&warm_milk(None)), Some(Rest::WarmMilk));
    let butter = made(
        vec![(Tool::Bowl, vec![food(Source::Milk, "cattle", 500)])],
        Quality::Common,
        None,
    );
    assert_eq!(kind_of(&butter), Some(Rest::Butter));
    let egg = made(
        vec![(Tool::Pan, vec![food(Source::Egg, "chicken", 55)])],
        Quality::Uncommon,
        None,
    );
    assert_eq!(
        (egg.kind, egg.count, egg.quality),
        (ItemKind::Dish, 1, Some(Quality::Uncommon))
    );
    assert_eq!(egg.food().unwrap().name(), "Fried Egg");
}

#[test]
fn a_big_dish_comes_in_servings() {
    let stew = made(
        vec![
            (
                Tool::Knife,
                vec![
                    food(Source::Meat, "cattle", 400),
                    food(Source::Vegetable, "carrot", 150),
                    food(Source::Vegetable, "onion", 100),
                ],
            ),
            (Tool::Cauldron, vec![Portion::Water { ml: 600 }]),
        ],
        Quality::Common,
        None,
    );
    assert_eq!(stew.kind, ItemKind::Dish);
    assert!(stew.count >= 3, "{}", stew.count);
    let one = stew.food().unwrap().grams();
    assert!((250..=400).contains(&one), "{one}");
    assert!(servings(1) == 1 && servings(600) == 2);
}

#[test]
fn eating_gives_well_fed() {
    let egg = dev_food(ItemKind::Dish, Quality::Common, Seed::new(0)).unwrap();
    let effect = eating(&egg, Quality::Rare, 10, Ingredients::built_in());
    assert_eq!(effect.source, crate::bonus::Source::WellFed);
    assert!(effect.end.unwrap() > 10);
}

#[test]
fn warmth_sets_the_pace() {
    assert_eq!(warmth_percent(-5), 0);
    assert_eq!(warmth_percent(10), 50);
    assert_eq!(warmth_percent(CARRIED_CELSIUS), 100);
    assert_eq!(warmth_percent(30), 150);
    assert_eq!(warmth_percent(60), 200);
}

#[test]
fn dough_proves_then_over_proves() {
    let mut dough = made(
        vec![(Tool::Bowl, vec![food(Source::Vegetable, "wheat", 300)])],
        Quality::Common,
        None,
    );
    let step = |dough: &mut Item, secs: u32, celsius| dough.rest_food(secs, 0, celsius);
    assert!(!step(&mut dough, HOUR / 2, CARRIED_CELSIUS).changed);
    assert!(step(&mut dough, HOUR, CARRIED_CELSIUS).changed);
    assert_eq!(dough.quality, Some(Quality::Uncommon));
    // In the cold nothing moves.
    assert!(!step(&mut dough, 100 * HOUR, 0).seen());
    step(&mut dough, 3 * HOUR, CARRIED_CELSIUS);
    assert_eq!(dough.quality, Some(Quality::Common));
    // Hot, over-proving comes twice as soon.
    step(&mut dough, 4 * HOUR, 40);
    assert_eq!(dough.quality, Some(Quality::Poor));
}

#[test]
fn noodles_dry() {
    let mut noodles = made(
        vec![
            (
                Tool::Bowl,
                vec![
                    food(Source::Vegetable, "wheat", 300),
                    Portion::Water { ml: 100 },
                ],
            ),
            (Tool::Knife, vec![]),
        ],
        Quality::Common,
        None,
    );
    noodles.rest_food(13 * HOUR, 0, CARRIED_CELSIUS);
    assert_eq!(kind_of(&noodles), Some(Rest::DriedNoodles));
    assert_eq!(noodles.food().unwrap().name(), "Wheat Dried Noodles");
    assert_eq!(noodles.quality, Some(Quality::Common));
}

#[test]
fn warm_milk_sets_wild_or_cultured() {
    let culture = |strain, quality| Some(Culture { strain, quality });
    let mut wild = warm_milk(None);
    wild.rest_food(9 * HOUR, 0, CARRIED_CELSIUS);
    assert_eq!(kind_of(&wild), Some(Rest::WarmMilk), "wild is slower");
    wild.rest_food(8 * HOUR, 0, CARRIED_CELSIUS);
    assert_eq!(
        (kind_of(&wild), wild.quality),
        (Some(Rest::Yoghurt), Some(Quality::Poor))
    );
    let mut cultured = warm_milk(culture(Strain::Yoghurt, Quality::Rare));
    cultured.rest_food(8 * HOUR, 0, CARRIED_CELSIUS);
    assert_eq!(kind_of(&cultured), Some(Rest::Yoghurt));
    assert_eq!(cultured.quality, Some(Quality::Uncommon));
    let mut yeasted = warm_milk(culture(Strain::Bread, Quality::Rare));
    yeasted.rest_food(11 * HOUR, 0, CARRIED_CELSIUS);
    assert_eq!(kind_of(&yeasted), Some(Rest::WarmMilk));
    yeasted.rest_food(HOUR, 0, CARRIED_CELSIUS);
    assert_eq!(
        (kind_of(&yeasted), yeasted.quality),
        (Some(Rest::Yoghurt), Some(Quality::Common))
    );
    // A culture taken from it is a yoghurt strain of its quality.
    let sample = culture_from(&cultured, Id::NOBODY, None).unwrap();
    let yeast = sample.yeast().unwrap();
    assert_eq!(
        (yeast.strain, sample.quality),
        (Strain::Yoghurt, Some(Quality::Uncommon))
    );
    assert!(!yeast.feedable(), "a new one is taken from each batch");
    assert!(culture_from(&warm_milk(None), Id::NOBODY, None).is_none());
}

#[test]
fn ground_food_rests_by_real_time_and_the_lands_warmth() {
    let mut dough = dev_food(ItemKind::Intermediate, Quality::Common, Seed::new(0)).unwrap();
    dough.rest = Some(RestClock::new(Rest::Dough, Quality::Common, None));
    let mut item = Item::new(Id::NOBODY, ItemKind::Intermediate);
    item.form = Some(crate::item_form::ItemForm::Food(Box::new(dough)));
    item.lay_food_down(1000);
    item.settle_lying_food(1000 + 2 * u64::from(HOUR), 10);
    assert_eq!(item.food().unwrap().rest.as_ref().unwrap().seconds(), HOUR);
    assert_eq!(item.quality, Some(Quality::Uncommon));
}

#[test]
fn tinted_by_foods_and_browned() {
    let ingredients = Ingredients::built_in();
    let milk = warm_milk(None);
    let form = milk.food().unwrap();
    let pale = tint(
        &form.portions,
        &crate::cooking::History::default(),
        ingredients,
    );
    let fried = tint(&form.portions, &form.history, ingredients);
    assert_eq!(pale, [245, 240, 225]);
    let mut history = form.history.clone();
    history.tools = vec![Tool::Pan];
    let browned = tint(&form.portions, &history, ingredients);
    assert!(browned[0] < pale[0] && fried == pale);
}

#[test]
fn accent_is_the_second_colour_by_grams() {
    let ingredients = Ingredients::built_in();
    let history = crate::cooking::History::default();
    let milk = food(Source::Milk, "cattle", 200);
    assert_eq!(
        accent(std::slice::from_ref(&milk), &history, ingredients),
        None
    );
    let eggs = food(Source::Egg, "chicken", 100);
    let mixed = [milk, eggs];
    assert_eq!(accent(&mixed, &history, ingredients), Some([240, 200, 90]));
}
