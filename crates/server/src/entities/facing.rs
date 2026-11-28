//! Which way mobs face (`shared::facing`): the way they go, whoever they
//! strike at and whoever strikes them (and noises they hear: `noise`).

use shared::facing::{Cone, Field, Heading};
use shared::spot::Spot as Units;

use super::{Mob, Spot};

impl Mob {
    /// What it sees round it: its kind's cone (mobs have no Observation yet).
    pub(super) fn cone(&self) -> Cone {
        Cone::new(self.spot, self.heading, Field::of(self.kind, 0))
    }

    /// Turns towards `to`, if it is somewhere else.
    pub(crate) fn face(&mut self, to: Units) {
        if let Some(heading) = Heading::towards(self.spot, to) {
            self.heading = heading;
        }
    }

    /// After its turn, having stood at `was`: faces whoever it is winding up
    /// at, else the way it went.
    pub(super) fn turn_after(&mut self, was: Units, spots: &[Spot]) {
        let striking = self.fight.pending.as_ref().map(|pending| pending.target);
        if let Some(them) = striking.and_then(|id| spots.iter().find(|spot| spot.id == id)) {
            self.face(them.at);
        } else if let Some(heading) = Heading::towards(was, self.spot) {
            self.heading = heading;
        }
    }
}
