//! The player's steps the server hasn't confirmed yet, and where they put the
//! player once it has said where it has them.

use std::collections::VecDeque;

use bevy::prelude::*;
use shared::movement::Step;
use shared::protocol::ClientMessage;
use shared::spot::Spot;

/// The steps sent and not yet confirmed, on the current map.
#[derive(Resource, Default)]
pub(super) struct Prediction {
    /// How many maps the server has put the player on (`ClientMessage::Step`
    /// and `Move`).
    map: u32,
    /// The number of the last step sent.
    last: u32,
    /// The map is a level or the Hall, where positions are in units and moves
    /// are sent instead of steps.
    on_level: bool,
    /// How far each step sent on this map that the server hasn't confirmed
    /// moved the player (in the map's measure: units on a level), by number.
    pending: VecDeque<(u32, (i64, i64))>,
    /// On a level or in the Hall, where the last move sent ends: where the
    /// player will be once the server has taken them all. The player may be
    /// shown further on, along a stretch not yet sent.
    sent: Option<(i64, i64)>,
    /// The map is the Hall.
    hall: bool,
}

impl Prediction {
    /// The message for a world map step just taken, which moved the player
    /// by `moved`, remembered until it is confirmed.
    pub fn step(&mut self, step: Step, moved: (i64, i64)) -> ClientMessage {
        self.last += 1;
        self.pending.push_back((self.last, moved));
        let (map, number) = (self.map, self.last);
        ClientMessage::Step { map, number, step }
    }

    /// The message for a move just taken on a level or in the Hall, from one
    /// spot to another, remembered until it is confirmed.
    pub fn moved(&mut self, from: Spot, to: Spot) -> ClientMessage {
        self.moved_to(from, to, to)
    }

    /// The message for a move towards `to` that `landed` somewhere else (a
    /// stumble, which the server works out from the move asked for).
    pub fn moved_to(&mut self, from: Spot, to: Spot, landed: Spot) -> ClientMessage {
        let (map, number) = self.remember(from, landed);
        ClientMessage::Move { map, number, to }
    }

    /// The message for a Teleport move from one spot to another, remembered
    /// as a move is until it is confirmed.
    pub fn teleported(&mut self, from: Spot, to: Spot) -> ClientMessage {
        let (map, number) = self.remember(from, to);
        ClientMessage::Teleport { map, number, to }
    }

    /// Remembers a move from `from` ending at `landed` until it is confirmed:
    /// its map and number.
    fn remember(&mut self, from: Spot, landed: Spot) -> (u32, u32) {
        self.last += 1;
        self.pending
            .push_back((self.last, (landed.x - from.x, landed.y - from.y)));
        self.sent = Some((landed.x, landed.y));
        (self.map, self.last)
    }

    /// The map and number the next step or move sent will have.
    pub fn next(&self) -> (u32, u32) {
        (self.map, self.last + 1)
    }

    /// Whether the map is the Hall, where nobody stumbles.
    pub fn in_hall(&self) -> bool {
        self.hall
    }

    /// Where the player is expected to be by the server's word: where the
    /// last move sent on this level ends, else where they are `shown`.
    pub fn expected(&self, shown: (i64, i64)) -> (i64, i64) {
        self.sent.filter(|_| self.on_level).unwrap_or(shown)
    }

    /// Whether the map is a level or the Hall (positions in units).
    pub fn on_level(&self) -> bool {
        self.on_level
    }

    /// The server has put the player on another map: steps on the last one
    /// will never be confirmed.
    pub fn changed_map(&mut self, on_level: bool) {
        self.changed_map_to(on_level, false);
    }

    /// The same, saying whether it is the Hall.
    pub fn changed_map_to(&mut self, on_level: bool, hall: bool) {
        self.map += 1;
        self.on_level = on_level;
        self.hall = hall;
        self.pending.clear();
        self.sent = None;
    }

    /// The server has the player at `at` on `map`, after step `number`:
    /// where the player should be now, with the steps taken since. `None` if
    /// it is about a map the player has left.
    pub fn confirmed(&mut self, map: u32, number: u32, at: (i64, i64)) -> Option<(i64, i64)> {
        if map != self.map {
            return None;
        }
        while self.pending.front().is_some_and(|&(n, _)| n <= number) {
            self.pending.pop_front();
        }
        Some(
            self.pending
                .iter()
                .fold(at, |at, (_, moved)| (at.0 + moved.0, at.1 + moved.1)),
        )
    }
}

/// Where to put the player, at `shown`, if the server's word puts them at
/// `predicted` instead: `None` if they are already there. On the world map
/// (`width`: it wraps east-west) the same place on another copy of the
/// planet counts as there; a move stays on the player's copy.
pub(super) fn correction(
    shown: (i64, i64),
    predicted: (i64, i64),
    width: Option<i64>,
) -> Option<(i64, i64)> {
    let (x, y) = predicted;
    let x = match width {
        Some(width) => x + (shown.0 - x + width / 2).div_euclid(width) * width,
        None => x,
    };
    ((x, y) != shown).then_some((x, y))
}

#[cfg(test)]
mod tests;
