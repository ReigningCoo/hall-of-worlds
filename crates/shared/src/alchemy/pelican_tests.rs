//! The Pelican (Alchemy plan PR 7): a step that changes nothing but raises the
//! purity score by 20% before it becomes a quality; its order and item.

use super::*;
use crate::brewing::{BrewSkills, quality};
use crate::herb::{Herbs, Stat};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::potion::{Contents, Dose};

fn untrained() -> AlchemySkills {
    AlchemySkills {
        brew: BrewSkills {
            alchemy: 0,
            herbalism: 0,
        },
        sub: [0; 6],
    }
}

fn held(doses: &[(Stat, i16)]) -> Contents {
    Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
}

fn powder(doses: &[(Stat, i16)]) -> Option<Stored> {
    Some(Stored {
        form: Form::Powder,
        contents: held(doses),
    })
}

fn run(source: Option<Stored>, steps: Vec<Step<'_>>) -> Result<Planned, Refused> {
    plan(&Process { source, steps }, untrained())
}

#[test]
fn the_score_is_raised_a_fifth_before_it_is_banded() {
    // 8 is Poor (8 ÷ 3 is 2); 8 × 120% is 9, Common.
    assert_eq!(quality(&held(&[(Stat::Aura, 8)])), Quality::Poor);
    assert_eq!(
        circulated_quality(&held(&[(Stat::Aura, 8)]), 1),
        Quality::Common
    );
    // 13 is Common; 15 (13 × 120%, rounded down) Uncommon.
    assert_eq!(
        circulated_quality(&held(&[(Stat::Aura, 13)]), 0),
        Quality::Common
    );
    assert_eq!(
        circulated_quality(&held(&[(Stat::Aura, 13)]), 1),
        Quality::Uncommon
    );
    // Far from the next quality, it does nothing: 9 × 120% is 10, still Common.
    assert_eq!(
        circulated_quality(&held(&[(Stat::Aura, 9)]), 1),
        Quality::Common
    );
    // Each Pelican adds another 20%: 11 × 140% is 15, Uncommon.
    assert_eq!(
        circulated_quality(&held(&[(Stat::Aura, 11)]), 2),
        Quality::Uncommon
    );
    // The score is the strongest less the next: 20 − 12 = 8, × 120% is 9.
    let mixed = held(&[(Stat::Aura, 20), (Stat::Will, -12)]);
    assert_eq!(circulated_quality(&mixed, 1), Quality::Common);
}

#[test]
fn a_pelican_keeps_the_product_and_raises_its_quality() {
    let plain = run(powder(&[(Stat::Aura, 13)]), vec![Step::Crucible]).unwrap();
    let circulated = run(powder(&[(Stat::Aura, 13)]), vec![Step::Pelican]).unwrap();
    assert_eq!(
        circulated.product,
        Product::Mixture {
            form: Form::Powder,
            contents: held(&[(Stat::Aura, 13)]),
        }
    );
    assert_eq!(circulated.quality, Some(Quality::Uncommon));
    assert_eq!(circulated.practised, [Apparatus::Pelican]);
    // It counts as a step for the chance like any other.
    assert_eq!(circulated.chance, plain.chance);
}

#[test]
fn a_pelican_before_a_crucible_still_counts() {
    // 18 calcined is 13.5, so 14: Common; × 120% is 16, Uncommon.
    let both = vec![Step::Pelican, Step::Crucible];
    let planned = run(powder(&[(Stat::Aura, 18)]), both).unwrap();
    assert_eq!(planned.product.contents(), &held(&[(Stat::Aura, 14)]));
    assert_eq!(planned.quality, Some(Quality::Uncommon));
}

#[test]
fn nothing_to_circulate_is_refused() {
    assert_eq!(
        run(None, vec![Step::Pelican]),
        Err(Refused::NothingToCirculate)
    );
}

#[test]
fn a_pelican_order_needs_a_carried_pelican() {
    let herbs = Herbs::built_in();
    let mut carried = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    let made = new_mixture(
        Form::Powder,
        held(&[(Stat::Aura, 13)]),
        Quality::Common,
        1,
        Id::for_test(1),
    );
    carried
        .give(made, StackLimits::default(), stand_ins)
        .unwrap();
    let stack = carried
        .stacks_where(|item| item.kind == ItemKind::Powder)
        .next()
        .unwrap()
        .0
        .id;
    let order = AlchemyOrder {
        source: Some(stack),
        steps: vec![OrderStep::Pelican],
        count: 1,
    };
    assert_eq!(
        carried.check_alchemy(&order, &herbs, untrained()),
        Err(OrderRefused::Apparatus(Apparatus::Pelican))
    );
    let pelican = Item::new(Id::for_test(2), ItemKind::Pelican);
    carried
        .give(pelican, StackLimits::default(), stand_ins)
        .unwrap();
    let checked = carried.check_alchemy(&order, &herbs, untrained()).unwrap();
    assert_eq!(checked.planned.quality, Some(Quality::Uncommon));
}
