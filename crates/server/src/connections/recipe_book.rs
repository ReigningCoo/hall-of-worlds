//! The recipe book (`ClientMessage::Recipe`, recipes plan PR 3): a scroll
//! carried or lying in reach read into it and used up (`RecipeBook::read`),
//! told back as `RecipeInserted` for the client to open; a recipe taken out
//! (`RecipeBook::remove`), a learnt one read from a scroll giving the scroll
//! back, carried or at the feet. Answered with `Inventory` and `Sheet`,
//! refusals only logged (the client checks the same rules first).

use bevy::prelude::*;
use shared::energy::handling_cost;
use shared::id::Id;
use shared::protocol::{RecipeAsk, ServerMessage};
use shared::recipe::{Entry, RECIPE_GRAMS, new_item};

use super::Client;
use super::give::give;
use crate::ServerWorld;
use crate::ids::new_id;

/// Answers a `RecipeAsk` in its turn: whether the client is still there, and
/// what it cost (handling a scroll), nothing if refused.
pub(super) fn answer(client: &mut Client, world: &mut ServerWorld, ask: RecipeAsk) -> (bool, u32) {
    let done = match &ask {
        RecipeAsk::Insert { item } => inserted(client, world, *item),
        RecipeAsk::Remove(entry) => removed(client, world, entry).map(|()| None),
    };
    let tick = world.action_clock.ticks();
    let (read, cost) = match done {
        Ok(read) => (read, handling_cost(RECIPE_GRAMS)),
        Err(why) => {
            warn!("{} refused {ask:?}: {why}", client.in_log());
            (None, 0)
        }
    };
    client.unsaved = true;
    client.ground_told.again = true;
    let mut told = client.send_inventory() && client.send_sheet(tick);
    if let Some(entry) = read {
        told = told && client.link.send(&ServerMessage::RecipeInserted(entry));
    }
    (told, cost)
}

/// Reads one scroll off the stack `item` into the book, using it up: the
/// recipe now in the book.
fn inserted(
    client: &mut Client,
    world: &mut ServerWorld,
    item: Id,
) -> Result<Option<Entry>, &'static str> {
    let carried = client.inventory.find(item).is_some();
    let scroll = match carried {
        true => client.inventory.find(item).map(|found| found.top().clone()),
        false => crate::ground::food_lying(client, world, item),
    };
    let scroll = scroll.ok_or("no such scroll in reach")?;
    let mut book = client.sheet.book.clone();
    book.read(&scroll).map_err(|why| match why {
        shared::recipe::Refused::NotAScroll => "not a recipe scroll",
        shared::recipe::Refused::Unknown => "its dish has no recipe",
        shared::recipe::Refused::Known => "already in the book",
    })?;
    match carried {
        true => take_carried(client, item)?,
        false => crate::ground::take_one_lying(client, world, item).map(|_| ())?,
    }
    client.sheet.book = book;
    Ok(client.sheet.book.iter().next().cloned())
}

/// Uses up the top one of a carried stack of scrolls.
fn take_carried(client: &mut Client, item: Id) -> Result<(), &'static str> {
    let found = client.inventory.find(item).ok_or("it isn't carried")?;
    if found.holds_quest_item() || client.inventory.is_worn(item) {
        return Err("it can't be read from there");
    }
    let taken = client.inventory.take_one(item, &mut new_id);
    taken.map(|_| ()).ok_or("it can't be taken from there")
}

/// Takes `entry` out of the book, giving back the scroll it was read from.
fn removed(
    client: &mut Client,
    world: &mut ServerWorld,
    entry: &Entry,
) -> Result<(), &'static str> {
    let removed = client.sheet.book.remove(entry).ok_or("not in the book")?;
    if let Some(form) = removed.scroll {
        give(
            client,
            world,
            new_item(form, 1, new_id(), Some(world.made_now())),
        );
    }
    Ok(())
}
