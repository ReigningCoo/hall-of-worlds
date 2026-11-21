//! Campfires burning on a level (`shared::campfire`), by cell: how many pieces
//! of kindling each has left, the one burning included (0: embers, about to
//! go out). The server keeps the fires themselves and sets these; the client
//! is sent them with the cells, and draws a fire's flames and light by them.

use super::Level;

impl Level {
    /// The pieces of kindling left in the campfire at `(x, y)`, if one burns
    /// there.
    pub fn campfire_at(&self, x: i64, y: i64) -> Option<u8> {
        self.campfires.get(&(x, y)).copied()
    }

    /// Sets the campfire at `(x, y)`: `None` for none.
    pub fn set_campfire(&mut self, x: i64, y: i64, pieces: Option<u8>) {
        match pieces {
            Some(pieces) => self.campfires.insert((x, y), pieces),
            None => self.campfires.remove(&(x, y)),
        };
    }

    /// Every campfire burning, by cell, with its pieces left.
    pub fn campfires(&self) -> impl Iterator<Item = ((i64, i64), u8)> + '_ {
        self.campfires.iter().map(|(&at, &pieces)| (at, pieces))
    }
}
