//! Reading recipe scrolls into the recipe book and taking recipes out
//! (`shared::recipe`, recipes plan PR 3).

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::recipe::Entry;

/// A client asking about its recipe book, in turn with its steps. Answered
/// with `Inventory` and `Sheet` (the book is on it), whether or not it could
/// be done; a scroll read is also told as `ServerMessage::RecipeInserted`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecipeAsk {
    /// Read one scroll off the stack `item`, carried or lying in reach, into
    /// the book (`RecipeBook::read`): the scroll is used up.
    Insert { item: Id },
    /// Take this recipe out of the book (`RecipeBook::remove`): a learnt one
    /// read from a scroll gives the scroll back, carried or at the feet.
    Remove(Entry),
}
