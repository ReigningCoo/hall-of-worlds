use shared::bonus::{Effect, Source, food_effect, poison};
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::item::ItemKind;
use shared::lang::{Lang, Language};

use super::bar::EFFECT_BAR;
use super::looks::{gone, icon_path, is_harmful, timed};
use super::popup::EFFECT_POPUP;
use super::words::{bonus_lines, ends_early, ends_in, name};
use crate::common::ServerTick;

const SECOND: f64 = ACTION_TICKS_PER_SECOND;

#[test]
fn the_markup_is_good() {
    EFFECT_BAR.assert_good();
    EFFECT_POPUP.assert_good();
}

#[test]
fn only_effects_with_an_end_not_yet_reached_show() {
    let mut forever = poison(0);
    forever.end = None;
    let bread = food_effect(ItemKind::Bread, 0).unwrap();
    let end = bread.end.unwrap() as f64;
    let effects = vec![forever, bread];
    let shown = |now| timed(&effects, now).map(|e| e.source).collect::<Vec<_>>();
    assert_eq!(shown(0.0), vec![Source::Eaten(ItemKind::Bread)]);
    assert_eq!(shown(end - 1.0), vec![Source::Eaten(ItemKind::Bread)]);
    assert!(shown(end).is_empty());
}

#[test]
fn the_shade_grows_as_time_runs_out() {
    let effect = poison(100);
    let end = effect.end.unwrap() as f64;
    assert_eq!(gone(&effect, 100.0), 0.0);
    assert!((gone(&effect, (100.0 + end) / 2.0) - 0.5).abs() < 1e-6);
    assert_eq!(gone(&effect, end + 5.0), 1.0);
}

#[test]
fn poison_is_harmful_and_food_is_not() {
    assert!(is_harmful(&poison(0)));
    assert!(!is_harmful(&food_effect(ItemKind::Bread, 0).unwrap()));
    assert!(!is_harmful(&Effect::dev_bonus(5, 0, 10)));
    let bread = icon_path(Source::Eaten(ItemKind::Bread));
    let apple = shared::fruit::FruitTag::of("apple");
    assert_eq!(
        bread,
        icon_path(Source::EatenFruit(apple)),
        "every food shares one"
    );
}

#[test]
fn the_popup_says_what_it_is_what_ends_it_and_when() {
    let lang = Lang::new(Language::English);
    let bread = food_effect(ItemKind::Bread, 0).unwrap();
    assert_eq!(name(bread.source, &lang), "Eating: Bread");
    assert_eq!(name(Source::Poison, &lang), "Poisoned");
    let tag = shared::fruit::FruitTag::of;
    assert_eq!(
        name(Source::EatenFruit(tag("apple")), &lang),
        "Eating: Apple"
    );
    assert_eq!(
        name(Source::FruitEffects(tag("kola_nut")), &lang),
        "Kola nut"
    );
    let vegetable = shared::vegetable::VegetableTag::of;
    assert_eq!(
        name(Source::EatenVegetable(vegetable("carrot")), &lang),
        "Eating: Carrot"
    );
    assert_eq!(
        name(Source::VegetableEffects(vegetable("cassava")), &lang),
        "Cassava"
    );
    let fungus = shared::fungus::FungusTag::of;
    assert_eq!(
        name(Source::EatenFungus(fungus("morel")), &lang),
        "Eating: Morel"
    );
    assert_eq!(
        name(Source::FungusEffects(fungus("manna_lichen")), &lang),
        "Manna lichen"
    );
    assert_eq!(
        name(Source::FungusPoison(fungus("death_cap")), &lang),
        "Poisoned: Death cap"
    );
    assert_eq!(
        bonus_lines(&bread, &lang),
        vec!["Restores 1 Health a second"]
    );
    let ends = ends_early(&bread, &lang);
    assert_eq!(ends.as_deref(), Some("Ends if you move or are hurt"));
    assert_eq!(ends_early(&poison(0), &lang), None);
    assert_eq!(
        bonus_lines(&poison(0), &lang),
        vec!["\u{2212}1 Health a second"]
    );
    assert_eq!(ends_in(&bread, 0.0, &lang).unwrap(), "Ends in 45 s");
    assert_eq!(
        ends_in(&bread, 44.5 * SECOND, &lang).unwrap(),
        "Ends in 1 s"
    );
    let dev = Effect::dev_bonus(5, 0, (300.0 * SECOND) as u64);
    assert_eq!(
        ends_in(&dev, 30.0 * SECOND, &lang).unwrap(),
        "Ends in 4 min 30 s"
    );
}

#[test]
fn the_clock_counts_on_from_what_the_server_said() {
    let mut clock = ServerTick::default();
    assert_eq!(clock.now(5.0), None);
    clock.set(1000, 2.0);
    assert_eq!(clock.now(3.0), Some(1000.0 + SECOND));
    // Never back before what it said.
    assert_eq!(clock.now(1.0), Some(1000.0));
}
