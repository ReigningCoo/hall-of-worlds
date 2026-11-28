//! `Choice`: what a mob may do about someone it sees, and its scores.

use shared::id::Id;

use super::super::{ATTACK_GAP, Doing, FRIENDLY_GAP};

/// One thing a mob may do about someone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Choice {
    /// Go after them to fight.
    Attack(Id),
    /// Come over, friendly.
    Approach(Id),
    /// Stand between them and its home.
    Guard(Id),
    /// Keep away from them.
    Flee(Id),
}

impl Choice {
    /// What a mob doing this has chosen, if anything: going after, following or
    /// tracking someone as an enemy is attacking them.
    pub(crate) fn of(doing: &Doing) -> Option<Self> {
        match *doing {
            Doing::Going { after, gap, .. } if gap == FRIENDLY_GAP => Some(Self::Approach(after)),
            Doing::Going { after, gap, .. }
            | Doing::Following { after, gap, .. }
            | Doing::Tracking { after, gap, .. }
                if gap == ATTACK_GAP =>
            {
                Some(Self::Attack(after))
            }
            Doing::Guarding { against, .. } => Some(Self::Guard(against)),
            Doing::Fleeing { from, .. } => Some(Self::Flee(from)),
            _ => None,
        }
    }

    /// Who it is about.
    pub(crate) fn who(self) -> Id {
        match self {
            Self::Attack(id) | Self::Approach(id) | Self::Guard(id) | Self::Flee(id) => id,
        }
    }

    /// What it is called in Inspect.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Attack(_) => "Attack",
            Self::Approach(_) => "Approach",
            Self::Guard(_) => "Guard",
            Self::Flee(_) => "Flee",
        }
    }

    /// What a mob does for it, someone last seen at `at`.
    pub(crate) fn doing(self, at: (i64, i64)) -> Doing {
        match self {
            Self::Attack(after) => Doing::Going {
                after,
                last_seen: at,
                gap: ATTACK_GAP,
            },
            Self::Approach(after) => Doing::Going {
                after,
                last_seen: at,
                gap: FRIENDLY_GAP,
            },
            Self::Guard(against) => Doing::Guarding {
                against,
                last_seen: at,
            },
            Self::Flee(from) => Doing::Fleeing {
                from,
                struck: false,
            },
        }
    }
}

/// A choice and how strongly the mob wants it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Scored {
    pub choice: Choice,
    pub score: i32,
}

/// A mob's choices as last scored, for Inspect: every option, its fear then,
/// and what it chose (`None`: it carried on).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Scores {
    pub options: Vec<Scored>,
    pub fear: i32,
    pub chosen: Option<Choice>,
}
