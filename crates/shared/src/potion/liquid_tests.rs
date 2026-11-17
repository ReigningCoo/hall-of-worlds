use super::*;
use crate::herb::Stat;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::{Field, ItemForm, Refused, RollRequest, give_form_to};
use crate::rng::Seed;

/// A bottle made as Give Item makes one, asking for `liquid` and `effects`.
fn asked(id: u128, liquid: Option<LiquidKind>, effects: Option<u8>) -> Result<Item, Refused> {
    let mut item = Item::new(Id::for_test(id), ItemKind::Bottle);
    item.quality = Some(Quality::Common);
    let request = RollRequest {
        liquid,
        effects,
        ..RollRequest::NONE
    };
    let mut new_id = || Id::for_test(999);
    give_form_to(&mut item, None, Seed::new(id as u64), &request, &mut new_id)?;
    Ok(item)
}

fn water(id: u128, water: Water) -> Item {
    asked(id, Some(LiquidKind::Water(water)), None).unwrap()
}

#[test]
fn liquids_are_named() {
    assert_eq!(water(1, Water::Plain).name(), "Bottle of Water");
    assert_eq!(water(1, Water::Moon).name(), "Bottle of Moon Water");
    let oil = asked(1, Some(LiquidKind::Oil(Oil::Nut)), Some(0)).unwrap();
    assert_eq!(oil.name(), "Bottle of Nut Oil");
    assert_eq!(oil.total_weight(), POTION_GRAMS);
    let potion = asked(1, Some(LiquidKind::Potion), None).unwrap();
    assert_eq!(potion.name(), "Potion");
    assert!(!potion.bottle().unwrap().contents().is_empty());
}

#[test]
fn asking_for_what_cannot_be_is_refused() {
    let refused = Err(Refused(Field::Effects));
    assert_eq!(
        asked(1, Some(LiquidKind::Water(Water::Salt)), Some(2)),
        refused
    );
    assert_eq!(asked(1, Some(LiquidKind::Potion), Some(0)), refused);
    assert_eq!(
        asked(1, Some(LiquidKind::Potion), Some(MOST_EFFECTS + 1)),
        refused
    );
    // Without a liquid, effects alone still say potion or empty.
    assert!(
        asked(1, None, Some(0))
            .unwrap()
            .bottle()
            .unwrap()
            .is_empty()
    );
    assert!(
        asked(1, None, Some(2))
            .unwrap()
            .bottle()
            .unwrap()
            .is_potion()
    );
}

#[test]
fn waters_and_oils_stack_by_liquid_whatever_their_quality() {
    let mut a = water(1, Water::Cave);
    let b = water(2, Water::Cave);
    a.quality = Some(Quality::Rare);
    assert!(a.looks_like(&b));
    assert!(!a.looks_like(&water(3, Water::Plain)));
    let empty = asked(4, None, None).unwrap();
    assert!(!a.looks_like(&empty));
}

#[test]
fn bottles_go_through_messages_and_saves() {
    let potion = asked(1, Some(LiquidKind::Potion), Some(2)).unwrap();
    for item in [water(2, Water::Salt), potion, asked(3, None, None).unwrap()] {
        let bytes = postcard::to_allocvec(&item).unwrap();
        assert_eq!(postcard::from_bytes::<Item>(&bytes).unwrap(), item);
        let text = ron::to_string(&item).unwrap();
        assert_eq!(ron::from_str::<Item>(&text).unwrap(), item);
    }
}

#[test]
fn bottles_saved_before_liquids_load() {
    let dose = Dose {
        stat: Stat::Will,
        step: 5,
    };
    let old = format!(
        "(contents: {}, look: 9)",
        ron::to_string(&vec![dose]).unwrap()
    );
    let form: BottleForm = ron::from_str(&old).unwrap();
    assert_eq!(form.liquid, Some(Liquid::Potion(Contents::new([dose]))));
    assert_eq!(form.look, 9);
    let empty: BottleForm = ron::from_str("(contents: [], look: 9)").unwrap();
    assert!(empty.is_empty());
    let form = ItemForm::Bottle(form);
    assert!(ron::from_str::<ItemForm>(&ron::to_string(&form).unwrap()).is_ok());
}
