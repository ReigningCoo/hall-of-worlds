//! Stumbling (`shared::alcohol::stumble`): drunk or worse, a move or step
//! whose number the hash picks veers off to one side, worked out from the
//! character's id so the client has already shown it. A stumble in the way
//! of a wall goes shorter, or nowhere. Not in the Hall.

use shared::alcohol::{Veer, stumble, veered_move, veered_step};
use shared::movement::{Gait, Step};
use shared::spot::Spot;

use super::Client;

impl Client {
    /// Whether its move or step `number` on this map stumbles, and which way.
    fn stumbles(&self, number: u32) -> Option<Veer> {
        let character = self.character_id?.0;
        stumble(character, self.map, number, self.sheet.alcohol_stage())
    }

    /// Where move `number` towards `to` on a level goes: `to`, or where a
    /// stumble takes it (`None`: nowhere). The Hall's moves never stumble.
    pub(super) fn stumbled_move(&self, number: u32, to: Spot) -> Option<Spot> {
        let (Some(veer), Some(visit)) = (self.stumbles(number), &self.visit) else {
            return Some(to);
        };
        let from = visit.spot();
        let gait = Gait::of(&self.sheet.skills);
        let landed = veered_move(visit.cells(), self.size(), gait, from, to, veer);
        (landed != from).then_some(landed)
    }

    /// The world map step `number` is taken as: `step`, or turned by a
    /// stumble (refused as any step if that can't be taken).
    pub(super) fn stumbled_step(&self, number: u32, step: Step) -> Step {
        match self.stumbles(number) {
            Some(veer) if self.visit.is_none() => veered_step(step, veer),
            _ => step,
        }
    }
}
