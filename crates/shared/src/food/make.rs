//! A cooking output as an item: an intermediate if it is one (`Rest::of`),
//! else a Dish in servings.

use super::serving::one_serving;
use super::{Culture, FoodForm, Rest, RestClock, servings};
use crate::cooking::{Ingredients, Output};
use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Quality};
use crate::item_form::ItemForm;

/// `output` made at `quality` (after the server's luck) as one new item,
/// with `culture` put in if it ferments.
pub fn food_item(
    output: &Output,
    quality: Quality,
    culture: Option<Culture>,
    ingredients: &Ingredients,
    id: Id,
    made_at: Option<MadeAt>,
) -> Item {
    let rest = Rest::of(output, ingredients);
    let mut form = FoodForm {
        dish: output.dish.clone(),
        portions: output.portions.clone(),
        history: output.history.clone(),
        rest: None,
    };
    let mut item = match rest {
        Some(rest) => {
            let culture = culture.filter(|_| rest.culture().is_some());
            form.rest = Some(RestClock::new(rest, quality, culture));
            Item::new(id, ItemKind::Intermediate)
        }
        None => {
            let count = servings(form.grams());
            form.portions = one_serving(&form.portions, count);
            let mut item = Item::new(id, ItemKind::Dish);
            item.count = count;
            item
        }
    };
    item.quality = Some(quality);
    item.made_at = made_at;
    item.form = Some(ItemForm::Food(Box::new(form)));
    item
}
