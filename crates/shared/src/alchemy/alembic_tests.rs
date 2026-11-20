//! The Alembic (Alchemy plan PR 5): pure alcohol from drinks, potions made
//! stronger in fewer bottles, Salt from Salt Water, and the tincture.

use super::*;
use crate::brewing::{BrewSkills, GroundHerb};
use crate::fermentation::{Base, Drink, Sweetness};
use crate::herb::{Effect, HerbDef, Herbs, Stat};
use crate::item::Quality;
use crate::potion::{Liquid, Oil, Water};

fn untrained() -> AlchemySkills {
    AlchemySkills {
        brew: BrewSkills {
            alchemy: 0,
            herbalism: 0,
        },
        sub: [0; 6],
    }
}

/// A drink of `strength` tenths of a percent.
fn drink(strength: u32) -> Liquid {
    Liquid::Drink(Box::new(Drink {
        base: Base::Fruit("apple".into()),
        strength,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    }))
}

/// Four herbs raising Strength: 4 steps untrained.
fn strength_herbs() -> Vec<HerbDef> {
    let base = Herbs::built_in().iter().next().unwrap().clone();
    (0..4)
        .map(|n| HerbDef {
            key: format!("s{n}"),
            effects: vec![Effect {
                stat: Stat::Strength,
                change: 3,
            }],
            ..base.clone()
        })
        .collect()
}

fn mortar(herbs: &[HerbDef], solvent: Solvent) -> Step<'_> {
    Step::Mortar {
        herbs: herbs
            .iter()
            .map(|def| GroundHerb::herb(def, Quality::Common))
            .collect(),
        solvent,
    }
}

fn retort<'a>(liquids: Vec<Liquid>) -> Step<'a> {
    Step::Retort {
        liquids,
        solid: None,
    }
}

fn alembic<'a>(fed: Option<Liquid>) -> Step<'a> {
    Step::Alembic { fed }
}

fn planned(steps: Vec<Step<'_>>) -> Result<Planned, Refused> {
    plan(
        &Process {
            source: None,
            steps,
            at_bench: false,
        },
        untrained(),
    )
}

#[test]
fn the_fed_alcohol_sets_the_reduction() {
    assert_eq!(reduced_by(0), 1);
    assert_eq!(reduced_by(60), 1);
    assert_eq!(reduced_by(120), 2);
    assert_eq!(reduced_by(199), 2);
    assert_eq!(reduced_by(crate::potion::PURE_STRENGTH), 11);
}

#[test]
fn drinks_distilled_and_reduced_make_pure_alcohol() {
    let five = vec![drink(60); 5];
    let made = planned(vec![retort(five), alembic(Some(drink(120)))]).unwrap();
    assert_eq!(made.product, Product::Alcohol { bottles: 2 });
    assert_eq!(made.quality, None);
    // Liquids alone: no Infusing, only Distilling.
    assert_eq!(made.practised, [Apparatus::Alembic]);
    // A smaller batch gives less, down to nothing.
    let three = planned(vec![retort(vec![drink(60); 3]), alembic(Some(drink(120)))]);
    assert_eq!(three.unwrap().product, Product::Alcohol { bottles: 1 });
    let one = planned(vec![retort(vec![drink(60)]), alembic(Some(drink(120)))]);
    assert_eq!(one.unwrap().product, Product::Nothing);
}

#[test]
fn without_enough_alcohol_fed_nothing_is_reduced() {
    for fed in [None, Some(drink(50))] {
        let made = planned(vec![retort(vec![drink(60); 5]), alembic(fed)]).unwrap();
        assert_eq!(made.product, Product::Nothing);
    }
    // Water distilled holds no alcohol, however far it is reduced.
    let water = vec![Liquid::Water(Water::Plain); 5];
    let made = planned(vec![retort(water), alembic(Some(drink(120)))]).unwrap();
    assert_eq!(made.product, Product::Nothing);
    // Each bottle needs its share of the alcohol: weak drinks give fewer.
    let weak = planned(vec![retort(vec![drink(10); 5]), alembic(Some(drink(120)))]);
    assert_eq!(weak.unwrap().product, Product::Nothing);
}

#[test]
fn a_potion_distilled_comes_out_stronger_in_fewer_bottles() {
    let herb = strength_herbs();
    let waters = vec![Liquid::Water(Water::Plain); 4];
    let ointment = mortar(&herb, Solvent::Oil(Oil::Neutral));
    // Spread over 4 bottles it is too weak to be a potion.
    let diluted = planned(vec![ointment.clone(), retort(waters.clone())]).unwrap();
    assert_eq!(diluted.product, Product::Nothing);
    let steps = vec![ointment, retort(waters), alembic(Some(Liquid::Alcohol))];
    let made = planned(steps).unwrap();
    let Product::Potion { contents, bottles } = &made.product else {
        panic!("distilled: {:?}", made.product);
    };
    // Pure alcohol reduces it furthest, but a potion keeps a bottle.
    assert_eq!(*bottles, 1);
    assert!(contents.doses()[0].step >= 4);
    assert!(made.practised.contains(&Apparatus::Alembic));
}

#[test]
fn salt_water_leaves_salt() {
    let salt = vec![Liquid::Water(Water::Salt); 3];
    let made = planned(vec![retort(salt.clone()), alembic(None)]).unwrap();
    assert_eq!(made.salt, 3);
    assert_eq!(made.product, Product::Nothing);
    // Not distilled, its salt stays in the water.
    let made = planned(vec![retort(salt)]).unwrap();
    assert_eq!(made.salt, 0);
}

#[test]
fn an_alembic_needs_liquid_and_alcohol_fed() {
    let herb = strength_herbs();
    let paste = vec![mortar(&herb, Solvent::Dry), alembic(None)];
    assert_eq!(planned(paste).unwrap_err(), Refused::NothingToDistil);
    let water = Some(Liquid::Water(Water::Plain));
    let fed_water = vec![retort(vec![drink(60)]), alembic(water)];
    assert_eq!(planned(fed_water).unwrap_err(), Refused::NotAlcohol);
    let juice = vec![retort(vec![Liquid::Juice("apple".into())])];
    assert_eq!(planned(juice).unwrap_err(), Refused::Liquid);
}

#[test]
fn a_tincture_is_a_potion_five_percent_stronger() {
    let herb = strength_herbs();
    let dry = planned(vec![mortar(&herb, Solvent::Dry)]).unwrap();
    let tincture = planned(vec![mortar(&herb, Solvent::Alcohol)]).unwrap();
    let Product::Potion { contents, bottles } = &tincture.product else {
        panic!("tincture: {:?}", tincture.product);
    };
    assert_eq!(*bottles, 1);
    assert_eq!(contents, dry.product.contents());
    assert!(tincture.quality.is_some());
    // +5% in hundredths, rounded only at the end: 10 steps become 10.5, so 11.
    let mut mix = Mix::default();
    mix.0[0] = 1000;
    mix.scale(TINCTURE_PERCENT);
    assert_eq!(mix.diluted(1).doses()[0].step, 11);
}

#[test]
fn salt_is_a_counted_item() {
    let salt = new_salt(3, crate::id::Id::for_test(1));
    assert_eq!(salt.kind, crate::item::ItemKind::Salt);
    assert_eq!(salt.count, 3);
    assert_eq!(salt.quality, None);
}
