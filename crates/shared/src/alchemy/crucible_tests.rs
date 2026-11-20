//! The Crucible and the Filter (Alchemy plan PR 6): Powder at ¾ of what was
//! calcined, never old; filtering adds the stats both hold and drops the rest;
//! the Powder item and the Filter's order.

use super::*;
use crate::brewing::{BrewSkills, GroundHerb};
use crate::herb::{Effect, HerbDef, Herbs, Stat};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::potion::{Contents, Dose, Liquid, Water};

fn untrained() -> AlchemySkills {
    AlchemySkills {
        brew: BrewSkills {
            alchemy: 0,
            herbalism: 0,
        },
        sub: [0; 6],
    }
}

/// Four herbs raising Strength.
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

fn dry(herbs: &[HerbDef]) -> Step<'_> {
    Step::Mortar {
        herbs: herbs
            .iter()
            .map(|def| GroundHerb::herb(def, Quality::Common))
            .collect(),
        solvent: Solvent::Dry,
    }
}

fn held(doses: &[(Stat, i16)]) -> Contents {
    Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
}

fn steps_of(contents: &Contents) -> Vec<(Stat, i16)> {
    contents.doses().iter().map(|d| (d.stat, d.step)).collect()
}

fn run(source: Option<Stored>, steps: Vec<Step<'_>>) -> Result<Planned, Refused> {
    plan(
        &Process {
            source,
            steps,
            at_bench: false,
        },
        untrained(),
    )
}

fn powder(doses: &[(Stat, i16)]) -> Stored {
    Stored {
        form: Form::Powder,
        contents: held(doses),
    }
}

#[test]
fn a_crucible_makes_a_powder_three_quarters_as_strong() {
    let herbs = strength_herbs();
    let paste = run(None, vec![dry(&herbs)]).unwrap();
    let strength = paste.product.contents().doses()[0].step;
    let calcined = run(None, vec![dry(&herbs), Step::Crucible]).unwrap();
    let Product::Mixture { form, contents } = &calcined.product else {
        panic!("{:?}", calcined.product);
    };
    assert_eq!(*form, Form::Powder);
    // In hundredths, ¾, rounded once, halves away from 0.
    let expected = (i32::from(strength) * POWDER_PERCENT + 50) / 100;
    assert_eq!(steps_of(contents), vec![(Stat::Strength, expected as i16)]);
    assert_eq!(calcined.practised, [Apparatus::Mortar, Apparatus::Crucible]);
    // A potion calcined loses its liquid: a Powder, no bottles.
    let water = Liquid::Water(Water::Plain);
    let potion = vec![
        dry(&herbs),
        Step::Retort {
            liquids: vec![water.clone(), water],
            solid: None,
        },
        Step::Crucible,
    ];
    let planned = run(None, potion).unwrap();
    assert!(matches!(
        planned.product,
        Product::Mixture {
            form: Form::Powder,
            ..
        }
    ));
}

#[test]
fn a_powder_infused_makes_a_potion_and_is_never_old() {
    let source = Some(powder(&[(Stat::Aura, 12)]));
    let water = vec![Liquid::Water(Water::Plain)];
    let planned = run(
        source,
        vec![Step::Retort {
            liquids: water,
            solid: None,
        }],
    )
    .unwrap();
    assert!(matches!(
        planned.product,
        Product::Potion { bottles: 1, .. }
    ));
    // 12 ÷ 3 is 4: Common, with no step lost for age.
    assert_eq!(planned.quality, Some(Quality::Common));
    assert_eq!(planned.practised, [Apparatus::Retort]);
}

#[test]
fn a_filter_adds_what_both_hold_and_drops_the_rest() {
    let source = Some(powder(&[(Stat::Strength, 4), (Stat::Will, -2)]));
    let through = held(&[(Stat::Strength, -1), (Stat::Aura, 3)]);
    let filter = Step::Filter {
        powder: Some(through),
    };
    let planned = run(source, vec![filter]).unwrap();
    let Product::Mixture { form, contents } = &planned.product else {
        panic!("{:?}", planned.product);
    };
    assert_eq!(*form, Form::Powder);
    assert_eq!(steps_of(contents), vec![(Stat::Strength, 3)]);
    assert_eq!(planned.practised, [Apparatus::Filter]);
    // Nothing in common: nothing made.
    let source = Some(powder(&[(Stat::Will, 2)]));
    let filter = Step::Filter {
        powder: Some(held(&[(Stat::Aura, 2)])),
    };
    assert_eq!(run(source, vec![filter]).unwrap().product, Product::Nothing);
}

#[test]
fn nothing_to_calcine_or_filter_is_refused() {
    let herbs = strength_herbs();
    assert_eq!(
        run(None, vec![Step::Crucible]),
        Err(Refused::NothingToCalcine)
    );
    let filter = Step::Filter {
        powder: Some(held(&[(Stat::Aura, 2)])),
    };
    assert_eq!(run(None, vec![filter]), Err(Refused::NothingToFilter));
    let empty = vec![dry(&herbs), Step::Filter { powder: None }];
    assert_eq!(run(None, empty), Err(Refused::NoPowder));
}

#[test]
fn a_powder_item_stacks_and_is_a_source() {
    let doses = held(&[(Stat::Aura, 5)]);
    let item = new_mixture(
        Form::Powder,
        doses.clone(),
        Quality::Rare,
        2,
        Id::for_test(1),
    );
    assert_eq!((item.kind, item.count), (ItemKind::Powder, 2));
    assert_eq!(item.mixture(), Some((Form::Powder, &doses)));
    assert_eq!(item.powder().map(|p| &p.contents), Some(&doses));
}

#[test]
fn a_filter_order_uses_a_carried_powder() {
    let herbs = Herbs::built_in();
    let doses = held(&[(Stat::Aura, 5)]);
    let made = |id| {
        new_mixture(
            Form::Powder,
            doses.clone(),
            Quality::Common,
            1,
            Id::for_test(id),
        )
    };
    let mut carried = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in [
        made(1),
        made(2),
        Item::new(Id::for_test(3), ItemKind::Filter),
    ] {
        carried
            .give(item, StackLimits::default(), stand_ins)
            .unwrap();
    }
    let mut powders = carried.stacks_where(|item| item.kind == ItemKind::Powder);
    let stack = powders.next().unwrap().0.id;
    let order = |powder| AlchemyOrder {
        source: Some(stack),
        steps: vec![OrderStep::Filter { powder }],
        count: 1,
    };
    let checked = carried.check_alchemy(&order(Some(stack)), &herbs, untrained());
    let planned = checked.unwrap().planned;
    assert_eq!(steps_of(planned.product.contents()), vec![(Stat::Aura, 10)]);
    let not_powder = carried.check_alchemy(&order(Some(Id::for_test(3))), &herbs, untrained());
    assert_eq!(not_powder.unwrap_err(), OrderRefused::Powder);
    let none = carried.check_alchemy(&order(None), &herbs, untrained());
    assert_eq!(none.unwrap_err(), OrderRefused::Rules(Refused::NoPowder));
    // Without a Filter carried, refused.
    let mut without = carried.clone();
    without.take_one(Id::for_test(3), &mut crate::id::stand_ins());
    let refused = without.check_alchemy(&order(Some(stack)), &herbs, untrained());
    assert_eq!(
        refused.unwrap_err(),
        OrderRefused::Apparatus(Apparatus::Filter)
    );
}
