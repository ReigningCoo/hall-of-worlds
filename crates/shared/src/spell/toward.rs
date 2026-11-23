//! `Toward`: what a where step is pointed at. Only a spell's first where uses
//! the caster's target; later ones go on the way they face.

use crate::facing::Heading;
use crate::spot::Spot;

use super::hit::{Creature, Origin};

/// What a where step is aimed at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toward {
    /// A creature (the caster's target), by its place in the slice.
    Creature(usize),
    /// A spot on the ground.
    Spot(Spot),
    /// Nothing: on the way the origin faces.
    Ahead,
}

impl Toward {
    /// The spot aimed at, if any.
    pub(super) fn spot(self, creatures: &[Creature]) -> Option<Spot> {
        match self {
            Self::Creature(index) => Some(creatures[index].body.at),
            Self::Spot(spot) => Some(spot),
            Self::Ahead => None,
        }
    }

    /// The way from the origin to what is aimed at: the origin's own heading
    /// for `Ahead`, or when it is aimed at its own spot.
    pub(super) fn heading(self, origin: &Origin, creatures: &[Creature]) -> Heading {
        self.spot(creatures)
            .and_then(|spot| Heading::towards(origin.at, spot))
            .unwrap_or(origin.heading)
    }
}
