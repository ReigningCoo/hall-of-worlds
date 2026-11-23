//! The spellbook (`ClientMessage::Spellbook`, magic plan PR 4): storing a
//! design checked against the character's skills, renaming, favourites and
//! removing, by `shared::spell::Spellbook`'s rules. Done at once, as it takes
//! no time; answered with `Sheet` whether or not it could be done, refusals
//! only logged (the client checks the same rules first).

use bevy::log::warn;
use shared::protocol::SpellbookAsk;
use shared::spell::BookRefusal;

use super::Client;
use crate::ServerWorld;

/// Answers one ask about the spellbook: `false` if the client has gone.
pub(super) fn answer(client: &mut Client, world: &ServerWorld, ask: SpellbookAsk) -> bool {
    let skills = client.sheet.skills.clone();
    let book = &mut client.sheet.spellbook;
    let done = match &ask {
        SpellbookAsk::Store { spell, name } => {
            book.store(spell.clone(), name.as_deref(), Some(&skills))
        }
        SpellbookAsk::Rename { spell, name } => book.rename(spell, name.as_deref()),
        SpellbookAsk::Favourite { spell, favourite } => book.set_favourite(spell, *favourite),
        SpellbookAsk::Remove { spell } => book.remove(spell).map(|_| ()),
    };
    match done {
        Ok(()) => client.unsaved = true,
        Err(why) => warn!(
            "{} had a spellbook change refused: {}",
            client.in_log(),
            refusal(why)
        ),
    }
    client.send_sheet(world.action_clock.ticks())
}

/// A refusal in words for the log (no names: they are the player's own text).
fn refusal(why: BookRefusal) -> String {
    match why {
        BookRefusal::Design(error) => format!("the design breaks the rules ({error:?})"),
        BookRefusal::Known => "that design is in the book already".to_owned(),
        BookRefusal::Full => "the book is full".to_owned(),
        BookRefusal::NotInBook => "no such spell in the book".to_owned(),
        BookRefusal::BadName => "the name isn't allowed".to_owned(),
    }
}
