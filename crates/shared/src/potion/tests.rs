use super::*;
use crate::herb::Stat;
use crate::rng::Seed;

fn dose(stat: Stat, step: i16) -> Dose {
    Dose { stat, step }
}

#[test]
fn contents_keep_one_order() {
    let a = Contents::new([dose(Stat::Mana, 3), dose(Stat::Strength, -2)]);
    let b = Contents::new([dose(Stat::Strength, -2), dose(Stat::Mana, 3)]);
    assert_eq!(a, b);
    assert_eq!(a.doses()[0].stat, Stat::Strength);
}

#[test]
fn contents_add_up_and_drop_zeros() {
    let contents = Contents::new([
        dose(Stat::Health, 4),
        dose(Stat::Will, 2),
        dose(Stat::Health, 1),
        dose(Stat::Will, -2),
    ]);
    assert_eq!(contents.doses(), &[dose(Stat::Health, 5)]);
    assert!(Contents::new([dose(Stat::Aura, 0)]).is_empty());
}

#[test]
fn contents_read_back_in_order() {
    let unordered = vec![dose(Stat::Mana, 1), dose(Stat::Agility, 2)];
    let text = ron::to_string(&unordered).unwrap();
    let read: Contents = ron::from_str(&text).unwrap();
    assert_eq!(read, Contents::new(unordered));
}

#[test]
fn rolls_are_the_same_from_one_seed() {
    let roll = |effects| roll_contents(&mut Seed::new(7).rng(), effects);
    assert_eq!(roll(3), roll(3));
    assert_eq!(roll(3).doses().len(), 3);
    assert!(roll(0).is_empty());
    assert_eq!(roll(9).doses().len(), MOST_EFFECTS as usize);
    for seed in 0..200 {
        let contents = roll_contents(&mut Seed::new(seed).rng(), 2);
        assert!(contents.doses().iter().any(|dose| dose.step.abs() >= 4));
    }
}

/// A bottle made as Give Item makes one, from `seed`, with `effects`.
fn bottle(id: u128, seed: u64, effects: u8) -> crate::item::Item {
    use crate::id::Id;
    use crate::item::{Item, ItemKind, Quality};
    use crate::item_form::{RollRequest, give_form_to};
    let mut item = Item::new(Id::for_test(id), ItemKind::Bottle);
    item.quality = Some(Quality::Common);
    let request = RollRequest {
        effects: Some(effects),
        ..RollRequest::NONE
    };
    let mut new_id = || Id::for_test(999);
    give_form_to(&mut item, None, Seed::new(seed), &request, &mut new_id).unwrap();
    item
}

#[test]
fn bottles_are_named_and_weighed_by_contents() {
    let empty = bottle(1, 1, 0);
    let potion = bottle(2, 1, 2);
    assert!(empty.bottle().unwrap().is_empty());
    assert_eq!(empty.name(), "Empty Bottle");
    assert_eq!(potion.name(), "Potion");
    assert_eq!(empty.total_weight(), EMPTY_GRAMS);
    assert_eq!(potion.total_weight(), POTION_GRAMS);
    assert_eq!(potion.shape(), crate::item::Shape::rectangle(1, 1));
}

#[test]
fn bottles_stack_by_contents_whatever_their_look() {
    // Empty ones from different seeds look different, and still stack.
    let (a, b) = (bottle(1, 1, 0), bottle(2, 2, 0));
    assert_ne!(a.bottle().unwrap().look, b.bottle().unwrap().look);
    assert!(a.looks_like(&b));
    // Potions stack only with the same contents.
    let mut c = bottle(3, 5, 2);
    let d = bottle(4, 6, 2);
    assert_ne!(
        c.bottle().unwrap().contents(),
        d.bottle().unwrap().contents()
    );
    assert!(!c.looks_like(&d));
    let mut e = bottle(5, 7, 0);
    if let Some(crate::item_form::ItemForm::Bottle(form)) = &mut e.form {
        form.liquid = c.bottle().unwrap().liquid.clone();
    }
    assert!(c.looks_like(&e));
    // Nor across quality.
    c.quality = Some(crate::item::Quality::Rare);
    assert!(!c.looks_like(&e));
}

#[test]
fn too_many_effects_are_refused() {
    use crate::id::Id;
    use crate::item::{Item, ItemKind};
    use crate::item_form::{Field, Refused, RollRequest, give_form_to};
    let mut item = Item::new(Id::for_test(1), ItemKind::Bottle);
    let request = RollRequest {
        effects: Some(MOST_EFFECTS + 1),
        ..RollRequest::NONE
    };
    let mut new_id = || Id::for_test(2);
    let refused = give_form_to(&mut item, None, Seed::new(1), &request, &mut new_id);
    assert_eq!(refused, Err(Refused(Field::Effects)));
}
