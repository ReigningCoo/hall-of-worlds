use super::*;
use crate::bonus::{self, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::Stat;
use crate::id::{Id, stand_ins};
use crate::item::{ItemKind, StackLimits};
use crate::rng::Seed;
use crate::vegetable::Raw;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// Someone with 20 Health at most, on Normal.
pub(super) const EATER: Eater = Eater {
    most_health: 20,
    easy: false,
};

fn def(key: &str) -> &'static FungusDef {
    Fungi::built_in().get(key).expect("a built-in fungus")
}

#[test]
fn the_table_has_every_fungus() {
    let fungi = Fungi::built_in();
    assert_eq!(fungi.len(), 111);
    assert_eq!(fungi.iter().filter(|def| def.invented).count(), 12);
    assert_eq!(fungi.iter().filter(|def| !def.herb.is_empty()).count(), 36);
    let count = |grows| fungi.iter().filter(|def| def.grows == grows).count();
    assert_eq!(count(Where::Surface), 52);
    assert_eq!(count(Where::Deep), 43);
    assert_eq!(count(Where::Dug), 13);
    assert_eq!(count(Where::Dim), 3);
    assert_eq!(fungi.iter().filter(|def| def.glow.is_some()).count(), 8);
    assert!(fungi.get("corn_smut").is_none(), "dropped (owner)");
}

#[test]
fn tags_find_their_fungus() {
    for fungus in Fungi::built_in().iter() {
        let found = Fungi::built_in().by_tag(FungusTag::of(&fungus.key));
        assert_eq!(found.map(|def| &def.key), Some(&fungus.key));
    }
}

#[test]
fn lookalikes_are_in_the_table_and_checked() {
    assert!(
        def("death_cap")
            .lookalikes
            .contains(&"straw_mushroom".to_owned())
    );
    let text = r#"[(key: "a", name: "A", group: Mushroom, where: Deep, niches: [Soil],
        temperature: (min: 0, max: 10), abundance: Common, unit: One, grams: 10, kcal: 3,
        raw: Never, growth: Fast, regrow: Pick, lookalikes: ["nothing"],
        look: (form: Cup, colours: [(1, 2, 3)], smallest: 1, largest: 2))]"#;
    let error = Fungi::read(text).expect_err("no such lookalike");
    assert!(error.contains("nothing"), "{error}");
}

#[test]
fn raw_healing_is_as_vegetables() {
    let button = def("button_mushroom");
    assert_eq!(button.raw, Raw::Commonly);
    // 22 kcal: 5.5 s cooked, 1.375 s raw, rounded to 1.
    assert_eq!(effect_ticks(button), 5 * SECOND);
    assert_eq!(healing(button), 1);
    let effects = eating(button, 100, EATER);
    assert_eq!(effects.len(), 1);
    let tag = FungusTag::of("button_mushroom");
    assert_eq!(effects[0].source, Source::EatenFungus(tag));
    assert_eq!(effects[0].end, Some(100 + SECOND));
    assert!(effects[0].ends_on.moving && effects[0].ends_on.hurt);
    // Rarely eaten raw: nothing.
    assert!(eating(def("oyster_mushroom"), 0, EATER).is_empty());
}

#[test]
fn only_poisons_brought_on_by_eating_apply() {
    // The common ink cap harms only with alcohol (fungi plan PR 6).
    let ink_cap = def("common_ink_cap");
    assert_eq!(
        ink_cap.poison.as_ref().unwrap().trigger,
        Trigger::WithAlcohol
    );
    let window = eating(ink_cap, 0, EATER);
    assert_eq!(window.len(), 1);
    assert_eq!(
        window[0].source,
        Source::FungusWindow(FungusTag::of("common_ink_cap"))
    );
    // The cellar cup's spores harm when breathed, not eaten (PR 9).
    let cellar_cup = def("cellar_cup");
    assert_eq!(
        cellar_cup.poison.as_ref().unwrap().trigger,
        Trigger::Breathed
    );
    assert!(eating(cellar_cup, 0, EATER).is_empty());
    // Morels are poisonous raw only, and raw is all there is for now.
    let morel = def("morel").poison.as_ref().unwrap();
    assert!(morel.raw_only && morel.when_eaten());
    assert_eq!(eating(def("morel"), 0, EATER).len(), 1);
}

#[test]
fn raw_food_effects_apply() {
    // Manna lichen's Strength +1 is raw.
    let manna = eating(def("manna_lichen"), 0, EATER);
    let effects = manna
        .iter()
        .find(|e| matches!(e.source, Source::FungusEffects(_)));
    let strength = bonus::Stat::Attribute(Stat::Strength.attribute().unwrap());
    assert_eq!(effects.expect("Strength").bonuses[0].stat, strength);
    // Lion's mane's is for cooking.
    assert!(eating(def("lions_mane"), 0, EATER).is_empty());
}

#[test]
fn a_piece_cuts_into_its_servings() {
    let puffball = def("giant_puffball");
    assert_eq!(puffball.unit, Unit::Piece);
    let whole = new_item(puffball, 1, Id::for_test(1), None);
    assert!(whole.fungus().unwrap().is_whole(puffball));
    let (eaten, rest) = cut(&whole, &mut stand_ins()).expect("a piece");
    let rest = rest.expect("servings left");
    assert_eq!(rest.count, u32::from(puffball.servings) - 1);
    assert_eq!(eaten.name(), "Giant puffball Slice");
    assert!(cut(&eaten, &mut stand_ins()).is_none(), "not cut again");
    let morel = new_item(def("morel"), 1, Id::for_test(2), None);
    assert!(cut(&morel, &mut stand_ins()).is_none());
}

#[test]
fn fungi_stack_by_fungus_whatever_their_look() {
    let chanterelle = def("chanterelle");
    let one = new_item(chanterelle, 2, Id::for_test(1), None);
    let two = new_item(chanterelle, 3, Id::for_test(2), None);
    assert_ne!(one.fungus().unwrap().look, two.fungus().unwrap().look);
    assert!(StackLimits::default().room(&one, &two) > 0);
    let morel = new_item(def("morel"), 1, Id::for_test(3), None);
    assert_eq!(StackLimits::default().room(&one, &morel), 0);
    assert_eq!(one.quality, None, "fungi have no quality");
    assert_eq!(one.kind, ItemKind::Fungus);
    assert_eq!(one.name(), "Chanterelle");
}

#[test]
fn every_fungus_has_a_dev_choice() {
    let seed = Seed::new(1);
    let morel = dev_fungus(Some("morel"), seed).map(|d| &d.key[..]);
    assert_eq!(morel, Some("morel"));
    assert!(dev_fungus(Some("apple"), seed).is_none());
    assert!(dev_fungus(None, seed).is_some());
}
