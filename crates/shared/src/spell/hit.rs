//! What a where step works on and gives back: the creatures about (`Creature`,
//! each on a `Side` of the caster), where the step starts (`Origin`), and what
//! it hit (`Hit`, the next step's origin).

use crate::combat::Body;
use crate::facing::Heading;
use crate::spot::Spot;

use super::filters::Filters;

/// Whose side a creature is on, as the caster sees it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Side {
    /// The caster themself.
    Caster,
    /// A friend: another player until parties exist.
    Friend,
    /// Anything hostile to the caster.
    Enemy,
    /// Neither (timid animals): only unfiltered steps reach them.
    Neutral,
}

impl Side {
    /// Whether a step with these filters may hit someone on this side:
    /// Friend and Enemy limit it to theirs; Self limits nothing (it only aims
    /// at the caster, `aim_first`), so no filter or Self alone hits everyone.
    /// The caster is never a friend.
    pub const fn allowed_by(self, filters: Filters) -> bool {
        if !(filters.friend || filters.enemy) {
            return true;
        }
        match self {
            Self::Caster | Self::Neutral => false,
            Self::Friend => filters.friend,
            Self::Enemy => filters.enemy,
        }
    }

    /// Whether two creatures are on the same side for Point's extra targets:
    /// the caster and friends together, enemies together, the neutral alone.
    pub const fn same_as(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Caster | Self::Friend, Self::Caster | Self::Friend)
                | (Self::Enemy, Self::Enemy)
                | (Self::Neutral, Self::Neutral)
        )
    }
}

/// A creature a step might hit: where it stands and whose side it is on. The
/// callers' own creatures, by their place in the slice they pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Creature {
    pub body: Body,
    pub side: Side,
}

/// Where a where step starts: a spot, the way it faces, and the creature it
/// starts from (the caster, or the one the step before hit), which the step
/// never hits itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Origin {
    pub at: Spot,
    /// The ground's height there, in units (`combat::Body::height`).
    pub height: i32,
    pub heading: Heading,
    /// The creature standing at the origin, by its place in the slice.
    pub creature: Option<usize>,
}

impl Origin {
    /// Starting from a creature, facing `heading`.
    pub fn from_creature(creatures: &[Creature], index: usize, heading: Heading) -> Self {
        let body = creatures[index].body;
        Self {
            at: body.at,
            height: body.height,
            heading,
            creature: Some(index),
        }
    }

    /// As a body, for distances in 3D (an origin with no creature counts as
    /// Tiny, a point).
    pub(super) fn body(&self, creatures: &[Creature]) -> Body {
        match self.creature {
            Some(index) => creatures[index].body,
            None => Body {
                at: self.at,
                height: self.height,
                size: crate::creature::Size::Tiny,
            },
        }
    }
}

/// One thing a step hit, or, where it hit nobody, a spot it reached. The next
/// step starts here (`origin`), facing away from where this step came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hit {
    /// The creature hit, by its place in the slice; `None` for a spot alone.
    pub creature: Option<usize>,
    pub at: Spot,
    pub height: i32,
    /// Away from where the step came from.
    pub heading: Heading,
}

impl Hit {
    /// A hit on a creature, reached from `from` going `heading` (kept when the
    /// two stand on the same spot).
    pub(super) fn on(creatures: &[Creature], index: usize, from: Spot, heading: Heading) -> Self {
        let body = creatures[index].body;
        Self {
            creature: Some(index),
            at: body.at,
            height: body.height,
            heading: Heading::towards(from, body.at).unwrap_or(heading),
        }
    }

    /// The next step's origin.
    pub fn origin(&self) -> Origin {
        Origin {
            at: self.at,
            height: self.height,
            heading: self.heading,
            creature: self.creature,
        }
    }
}

/// Whether a step hit anyone (a first step that didn't fizzles).
pub fn hit_anyone(hits: &[Hit]) -> bool {
    hits.iter().any(|hit| hit.creature.is_some())
}
