//! The Retort's solids and transformations (Alchemy plan PR 8).

use super::tests::knowing;
use super::*;
use crate::bonus::{Bonus, Stat as BonusStat};
use crate::brewing::Ingredient;
use crate::brewing::drunk;
use crate::dungeon::{MOST_DARKVISION, darkvision_reach};
use crate::herb::{HerbKnowledge, Stat};
use crate::id::Id;
use crate::item::{ItemKind, Quality};
use crate::potion::{Contents, Dose, Liquid, Water};
use crate::sheet::Row;
use crate::vegetable::{VegetableTag, Vegetables, new_item};

fn dose(stat: Stat, step: i16) -> Dose {
    Dose { stat, step }
}

fn mix(doses: &[(Stat, i16)]) -> Mix {
    Mix::of(&Contents::new(doses.iter().map(|&(s, n)| dose(s, n))))
}

/// A stored Paste of these doses through a Retort of one Plain Water and
/// `solid`.
fn retorted(doses: &[(Stat, i16)], solid: Option<Solid>) -> Result<Planned, Refused> {
    let contents = Contents::new(doses.iter().map(|&(s, n)| dose(s, n)));
    let process = Process {
        source: Some(Stored {
            form: Form::Paste,
            contents,
        }),
        steps: vec![Step::Retort {
            liquids: vec![Liquid::Water(Water::Plain)],
            solid,
        }],
        at_bench: false,
    };
    plan(&process, AlchemySkills::default())
}

fn carrot() -> Solid {
    Solid::Vegetable(VegetableTag::of("carrot"))
}

#[test]
fn salt_sulphur_and_mercury_lean_their_rows() {
    let start = [(Stat::Strength, 4), (Stat::Intellect, 4), (Stat::Aura, -4)];
    let mut salted = mix(&start);
    salted.lean(Row::Body, LEAN_PERCENT);
    assert_eq!(
        salted,
        mix(&[(Stat::Strength, 5), (Stat::Intellect, 3), (Stat::Aura, -3)])
    );
    let mut sulphured = mix(&start);
    sulphured.lean(Row::Mind, LEAN_PERCENT);
    assert_eq!(
        sulphured,
        mix(&[(Stat::Strength, 3), (Stat::Intellect, 5), (Stat::Aura, -3)])
    );
    let mut mercuried = mix(&start);
    mercuried.lean(Row::Spirit, LEAN_PERCENT);
    assert_eq!(
        mercuried,
        mix(&[(Stat::Strength, 3), (Stat::Intellect, 3), (Stat::Aura, -5)])
    );
    assert_eq!(Solid::Salt.leans_to(), Some(Row::Body));
    assert_eq!(Solid::Sulphur.leans_to(), Some(Row::Mind));
    assert_eq!(Solid::Mercury.leans_to(), Some(Row::Spirit));
}

#[test]
fn a_retort_applies_its_solid_after_its_liquids() {
    let planned = retorted(&[(Stat::Strength, 8), (Stat::Will, 8)], Some(Solid::Salt)).unwrap();
    assert_eq!(
        planned.product,
        Product::Potion {
            contents: Contents::new([dose(Stat::Strength, 10), dose(Stat::Will, 6)]),
            bottles: 1,
        }
    );
}

#[test]
fn carrot_turns_perception_into_darkvision_keeping_harm() {
    let doses = [
        (Stat::Strength, 3),
        (Stat::Perception, 6),
        (Stat::Health, -2),
    ];
    let planned = retorted(&doses, Some(carrot())).unwrap();
    assert_eq!(
        planned.product,
        Product::Potion {
            contents: Contents::new([dose(Stat::Health, -2), dose(Stat::Darkvision, 6)]),
            bottles: 1,
        }
    );
    // Without helpful Perception the carrot changes nothing.
    let without = [(Stat::Strength, 5), (Stat::Perception, -1)];
    let planned = retorted(&without, Some(carrot())).unwrap();
    assert_eq!(planned.product, retorted(&without, None).unwrap().product);
}

#[test]
fn a_solid_alone_needs_something_before_it() {
    let paste = Process {
        source: Some(Stored {
            form: Form::Paste,
            contents: Contents::new([dose(Stat::Perception, 5)]),
        }),
        steps: vec![Step::Retort {
            liquids: Vec::new(),
            solid: Some(carrot()),
        }],
        at_bench: false,
    };
    let planned = plan(&paste, AlchemySkills::default()).unwrap();
    assert_eq!(
        planned.product,
        Product::Mixture {
            form: Form::Paste,
            contents: Contents::new([dose(Stat::Darkvision, 5)]),
        }
    );
    let alone = Process {
        source: None,
        steps: vec![Step::Retort {
            liquids: Vec::new(),
            solid: Some(Solid::Salt),
        }],
        at_bench: false,
    };
    assert_eq!(
        plan(&alone, AlchemySkills::default()),
        Err(Refused::NoLiquid)
    );
    let turnip = Solid::Vegetable(VegetableTag::of("turnip"));
    assert_eq!(
        retorted(&[(Stat::Will, 4)], Some(turnip)),
        Err(Refused::Solid)
    );
}

#[test]
fn which_items_are_solids() {
    let id = Id::for_test(1);
    assert_eq!(
        Solid::of_item(&new_solid(ItemKind::Salt, 3, id)),
        Some(Solid::Salt)
    );
    assert_eq!(
        Solid::of_item(&new_solid(ItemKind::Sulphur, 1, id)),
        Some(Solid::Sulphur)
    );
    assert_eq!(
        Solid::of_item(&new_solid(ItemKind::Mercury, 1, id)),
        Some(Solid::Mercury)
    );
    let vegetables = Vegetables::built_in();
    let carrot_item = new_item(vegetables.get("carrot").unwrap(), 2, id, None);
    assert_eq!(Solid::of_item(&carrot_item), Some(carrot()));
    let turnip = new_item(vegetables.get("turnip").unwrap(), 2, id, None);
    assert_eq!(Solid::of_item(&turnip), None);
    for change in TRANSFORMATIONS {
        assert!(vegetables.get(change.solid).is_some(), "{}", change.solid);
    }
}

#[test]
fn darkvision_is_learned_through_perception() {
    assert_eq!(made_from(Stat::Darkvision), Stat::Perception);
    assert_eq!(made_from(Stat::Will), Stat::Will);
    let mut planned = retorted(&[(Stat::Perception, 6)], Some(carrot())).unwrap();
    planned.contributions = vec![Contribution {
        ingredient: Ingredient::Herb,
        herb: "eyebright".into(),
        effect: 0,
        stat: Stat::Perception,
        steps: 6,
    }];
    let mut known = HerbKnowledge::default();
    assert_eq!(preview(&planned, knowing(&known)), vec![Seen::Unknown]);
    let lesson = lesson(&planned, knowing(&known)).unwrap();
    assert_eq!((lesson.herb.as_str(), lesson.effect), ("eyebright", 0));
    known.learn_effect("eyebright", 0);
    let seen = preview(&planned, knowing(&known));
    assert_eq!(seen, vec![Seen::Known(dose(Stat::Darkvision, 6))]);
}

#[test]
fn darkvision_reaches_by_strength_and_purity() {
    assert_eq!(darkvision_reach(0, Quality::Legendary), 0);
    assert_eq!(darkvision_reach(-3, Quality::Common), 0);
    assert_eq!(darkvision_reach(1, Quality::Poor), 2);
    assert_eq!(darkvision_reach(4, Quality::Common), 9);
    assert_eq!(darkvision_reach(4, Quality::Rare), 11);
    assert_eq!(darkvision_reach(40, Quality::Legendary), MOST_DARKVISION);
    let potion = Contents::new([dose(Stat::Darkvision, 4), dose(Stat::Health, -1)]);
    let effects = drunk(&potion, Quality::Common, 0).effects;
    let bonuses: Vec<Bonus> = effects.iter().flat_map(|e| e.bonuses.clone()).collect();
    assert_eq!(bonuses, vec![Bonus::flat(BonusStat::Darkvision, 9_000)]);
    let totals = crate::bonus::Totals::of(&bonuses);
    assert_eq!(totals.darkvision(), 9);
}
