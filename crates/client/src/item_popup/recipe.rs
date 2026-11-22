//! A recipe scroll's line in its popup (recipes plan PR 3): "Already in your
//! recipe book" when the reader's book has it (`Sheet::book`), else how to
//! read it in ("Drop on yourself ...", `inventory`'s clothes panel).

use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::recipe::RecipeBook;

use crate::common::written;

/// The line for a recipe scroll, by its top; `None` for anything else.
pub(super) fn recipe_line(item: &Item, book: &RecipeBook, lang: &Lang) -> Option<String> {
    let form = item.top().recipe()?;
    let id = match book.knows(form) {
        true => "recipe-known",
        false => "recipe-drop-on-self",
    };
    Some(written(lang.render(id, &Args::new())))
}
