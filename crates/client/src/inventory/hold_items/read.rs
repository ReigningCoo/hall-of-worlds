//! Reading a recipe scroll into the recipe book by letting go of it on the
//! clothes panel (`Target::Body`), carried or from the ground, as eating is
//! (`eat`): `RecipeAsk::Insert`, one scroll used up at once as the server
//! will, the rest of the stack staying where it was. A scroll the book has
//! already is kept, and the line under the clothes says so. The server then
//! tells `RecipeInserted`, for the reading panel (recipes plan PR 4).

use bevy::prelude::*;
use shared::energy::handling_cost;
use shared::id::Id;
use shared::item::{Inventory, Item};
use shared::protocol::{ClientMessage, RecipeAsk};
use shared::recipe::{RECIPE_GRAMS, RecipeBook};

use super::super::hold::Target;
use super::super::{Carried, HeldBy, Holding};
use super::Outbox;
use crate::common::{GROUND, HeldStock, SpentEnergy, TookItem};
use crate::connection::ToServer;

/// What the line under the clothes says of a scroll the book has.
pub(in crate::inventory) const ALREADY_KNOWN: &str = "Already in your recipe book";

/// The recipe scroll held, if it is one: its stack's id (carried, or lying
/// in reach) and the stack. Never someone's stock or a worn one.
pub(super) fn held_scroll(carried: &Inventory, holding: &Holding, held: Id) -> Option<(Id, Item)> {
    if let Some((with, item)) = &holding.stock {
        let scroll = *with == GROUND && item.recipe().is_some();
        return scroll.then(|| (held, item.clone()));
    }
    let group = holding.group.as_ref().map(|(stack, _)| *stack);
    let counted = holding.counted.map(|(stack, _)| stack);
    let stack = group.or(counted).unwrap_or(held);
    let item = carried.find(stack)?;
    (item.recipe().is_some() && !carried.is_worn(stack)).then(|| (stack, item.clone()))
}

/// Whether `book` would take one of `scroll` (a real dish it hasn't).
pub(super) fn readable(book: &RecipeBook, scroll: &Item) -> bool {
    (scroll.recipe()).is_some_and(|form| form.is_known() && !book.knows(form))
}

/// Reads the scroll held, if it is let go of on the clothes: whether it was
/// one (nothing else is then done with it).
pub(super) fn read_on(
    held: HeldBy,
    aim: Target,
    holding: &mut ResMut<Holding>,
    carried: &mut ResMut<Carried>,
    held_stock: &mut ResMut<HeldStock>,
    out: &mut Outbox,
) -> bool {
    if !matches!(aim, Target::Body { .. }) {
        return false;
    }
    let Some((stack, item)) = held_scroll(&carried.0, holding, held.item.id) else {
        return false;
    };
    let from = holding.stock.take().map(|(with, _)| with);
    held_stock.set_if_neq(HeldStock(None));
    holding.group.take();
    holding.counted.take();
    if !readable(&out.sheet.0.book, &item) {
        let (note, _, time, _) = &mut out.wear;
        note.say_words(ALREADY_KNOWN.to_owned(), time.elapsed_secs_f64());
        return true;
    }
    let insert = RecipeAsk::Insert { item: stack };
    out.to_server.write(ToServer(ClientMessage::Recipe(insert)));
    out.spent.write(SpentEnergy(handling_cost(RECIPE_GRAMS)));
    match from {
        // One off the ground; all of it if it was the last.
        Some(with) if item.count() <= 1 => {
            out.took.write(TookItem { with, item: stack });
        }
        Some(_) => {}
        None => {
            carried.0.take_one(stack, &mut || holding.stand_in());
        }
    }
    true
}
