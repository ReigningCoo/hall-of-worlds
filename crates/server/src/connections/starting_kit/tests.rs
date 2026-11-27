//! A new character's lantern, oil and coins.

use shared::item::{Inventory, Item, ItemKind, MadeAt, StackLimits};
use shared::item_form::ItemForm;
use shared::lantern::FOUNT_ML;
use shared::potion::{Liquid, Oil};
use shared::world::WorldSeed;

use super::{OIL_BOTTLES, STARTING_COPPER, give_starting_kit};

fn carried(inventory: &Inventory) -> Vec<&Item> {
    inventory.items.iter().map(|placed| &placed.item).collect()
}

#[test]
fn a_new_character_gets_a_full_lantern_and_two_bottles_of_oil() {
    let mut inventory = Inventory::default();
    let made_at = MadeAt {
        world: WorldSeed::new(1).id(),
        minute: 0,
    };
    give_starting_kit(&mut inventory, made_at, StackLimits::default());
    let items = carried(&inventory);
    let lanterns: Vec<_> = items.iter().filter_map(|item| item.lantern()).collect();
    assert_eq!(lanterns.len(), 1);
    assert_eq!(lanterns[0].ml, FOUNT_ML);
    // Not lit until held or tied.
    assert!(!inventory.lantern_lit());
    let oil = |item: &&&Item| {
        item.kind == ItemKind::Bottle
            && matches!(&item.form, Some(ItemForm::Bottle(form))
                if form.liquid == Some(Liquid::Oil(Oil::Neutral)))
    };
    let bottles: u32 = items.iter().filter(oil).map(|item| item.count()).sum();
    assert_eq!(bottles, OIL_BOTTLES);
}

#[test]
fn a_new_character_gets_a_gold_coin() {
    let mut inventory = Inventory::default();
    let made_at = MadeAt {
        world: WorldSeed::new(1).id(),
        minute: 0,
    };
    give_starting_kit(&mut inventory, made_at, StackLimits::default());
    let gold: Vec<_> = carried(&inventory)
        .into_iter()
        .filter(|item| item.kind == ItemKind::GoldCoin)
        .collect();
    assert_eq!(gold.len(), 1);
    assert_eq!(gold[0].count(), 1);
    assert_eq!(STARTING_COPPER, 10_000);
}
