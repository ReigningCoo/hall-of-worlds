//! Cutting a whole vegetable or piece (owner, 2026-11-17): eating one splits it
//! into its servings, one eaten and the rest a stack of servings, as fruit.

use super::{VegetableForm, Vegetables};
use crate::id::Id;
use crate::item::Item;
use crate::item_form::ItemForm;

/// One whole vegetable (`count` 1) cut: the serving eaten and the rest, a stack
/// of servings, each with an id from `new_id`. `None` if it isn't a whole one
/// the built-in table has.
pub fn cut(whole: &Item, new_id: &mut impl FnMut() -> Id) -> Option<(Item, Option<Item>)> {
    let form = whole.vegetable()?;
    let def = Vegetables::built_in().get(&form.vegetable)?;
    if !form.is_whole(def) {
        return None;
    }
    let mut serving = |count: u32| {
        let id = new_id();
        let mut item = Item::new(id, whole.kind);
        item.count = count;
        item.made_at = whole.made_at;
        item.form = Some(ItemForm::Vegetable(VegetableForm::serving(def, id.seed())));
        item
    };
    let eaten = serving(1);
    let left = u32::from(def.servings).saturating_sub(1);
    let rest = (left > 0).then(|| serving(left));
    Some((eaten, rest))
}
