//! Personal map marks (owner's design, 2026-11-19): a character's own places on
//! a planet's maps, a small icon in a colour on a black disc. Placed by hand on
//! the full map, up to `MOST_HAND_MARKS` a planet; automatic ones (a pile left on
//! wilderness ground) are white and don't count. Kept by the server per
//! character per planet, never in sites or the Hall.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::movement::past_the_ice;
use crate::spot::Spot;
use crate::wilderness::SCALE;

/// The most marks placed by hand a character may have on one planet.
pub const MOST_HAND_MARKS: usize = 100;

/// The picture inside a mark. Serde by name, new ones at the end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkIcon {
    Flag,
    House,
    Cave,
    Mine,
    Farm,
    Orchard,
    Bag,
    Logs,
    Camp,
    Danger,
}

impl MarkIcon {
    /// Every icon, in the order the popup shows them.
    pub const ALL: [Self; 10] = [
        Self::Flag,
        Self::House,
        Self::Cave,
        Self::Mine,
        Self::Farm,
        Self::Orchard,
        Self::Bag,
        Self::Logs,
        Self::Camp,
        Self::Danger,
    ];

    /// Its name in kebab case, for the language files' keys and debug tools.
    pub fn key(self) -> &'static str {
        match self {
            Self::Flag => "flag",
            Self::House => "house",
            Self::Cave => "cave",
            Self::Mine => "mine",
            Self::Farm => "farm",
            Self::Orchard => "orchard",
            Self::Bag => "bag",
            Self::Logs => "logs",
            Self::Camp => "camp",
            Self::Danger => "danger",
        }
    }

    /// The icon with this `key`, if any.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|icon| icon.key() == key)
    }
}

/// The colour of a mark's icon. Serde by name, new ones at the end.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkColour {
    #[default]
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    White,
}

impl MarkColour {
    /// Every colour, in the order the popup shows them.
    pub const ALL: [Self; 8] = [
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Cyan,
        Self::Blue,
        Self::Purple,
        Self::White,
    ];

    /// Its red, green and blue, bright enough to read on black.
    pub fn rgb(self) -> [u8; 3] {
        match self {
            Self::Red => [232, 52, 44],
            Self::Orange => [245, 140, 30],
            Self::Yellow => [250, 220, 50],
            Self::Green => [70, 200, 70],
            Self::Cyan => [50, 210, 220],
            Self::Blue => [70, 120, 245],
            Self::Purple => [175, 90, 230],
            Self::White => [245, 245, 245],
        }
    }

    /// Its name in kebab case, for the language files' keys and debug tools.
    pub fn key(self) -> &'static str {
        match self {
            Self::Red => "red",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Cyan => "cyan",
            Self::Blue => "blue",
            Self::Purple => "purple",
            Self::White => "white",
        }
    }

    /// The colour with this `key`, if any.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|colour| colour.key() == key)
    }
}

/// One mark on a planet's maps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapMark {
    pub id: Id,
    /// Where, in wilderness cells (`SCALE` to a world map point), `x` within
    /// the planet's width.
    pub at: (i64, i64),
    pub icon: MarkIcon,
    pub colour: MarkColour,
    /// Placed by the game (a pile left in the wilderness), not by hand: these
    /// don't count towards `MOST_HAND_MARKS`.
    #[serde(default)]
    pub auto: bool,
    /// An automatic mark's pile: where it lies on the wilderness's level, as
    /// the server keeps it (beyond the seam, too). The mark goes when it does.
    #[serde(default)]
    pub pile: Option<Spot>,
}

impl MapMark {
    /// The automatic mark for a pile a player has just started at `pile` on
    /// the wilderness's ground, on a planet `points` world map points round:
    /// white, Logs for a log pile and Bag for anything else. `None` past the ice.
    pub fn for_pile(id: Id, pile: Spot, points: i64, logs: bool) -> Option<Self> {
        Some(Self {
            id,
            at: place(pile.cell(), points)?,
            icon: if logs { MarkIcon::Logs } else { MarkIcon::Bag },
            colour: MarkColour::White,
            auto: true,
            pile: Some(pile),
        })
    }
}

/// The middle cell of a world map point: where a mark placed on the world map goes.
pub fn middle_of_point(point: (i64, i64)) -> (i64, i64) {
    (point.0 * SCALE + SCALE / 2, point.1 * SCALE + SCALE / 2)
}

/// The world map point a cell is in.
pub fn point_of(at: (i64, i64)) -> (i64, i64) {
    (at.0.div_euclid(SCALE), at.1.div_euclid(SCALE))
}

/// Where a mark asked for at `at` (in cells) goes on a planet `points` world
/// map points round: `x` brought round into the planet, or `None` past the ice
/// caps, where nobody can go.
pub fn place(at: (i64, i64), points: i64) -> Option<(i64, i64)> {
    if points <= 0 || past_the_ice(points, at.1.div_euclid(SCALE)) {
        return None;
    }
    Some((at.0.rem_euclid(points * SCALE), at.1))
}

/// How many of `marks` were placed by hand.
pub fn hand_marks(marks: &[MapMark]) -> usize {
    marks.iter().filter(|mark| !mark.auto).count()
}

/// Whether one more mark may be placed by hand beside `marks`.
pub fn may_add_by_hand(marks: &[MapMark]) -> bool {
    hand_marks(marks) < MOST_HAND_MARKS
}

#[cfg(test)]
mod tests;
