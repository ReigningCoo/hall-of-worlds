//! The Tavern Dev Area's decor, in the building's own cells (`Shell`):
//! trophies, weapons, a tapestry and a notice board on the taproom's walls,
//! hams and onions by the back-bar, a sign outside the door, sconces and two
//! chandeliers, rugs by the fire and before the bar, and each bedroom's
//! sconce, tapestry, coat pegs and rug.

use super::tavern::Shell;
use crate::decor::{ChandelierStyle, Decor, Hanging, Rug, RugColour, RugPattern};
use crate::door::Side;
use crate::dungeon::Level;

/// Things on walls: the wall cell, the way its face looks and what hangs.
const HANGINGS: [((i64, i64), Side, Hanging); 22] = [
    ((-1, 4), Side::East, Hanging::Antlers),
    ((-1, 13), Side::East, Hanging::BoarHead),
    ((-1, 11), Side::East, Hanging::NoticeBoard),
    ((-1, 9), Side::East, Hanging::CoatPegs),
    ((-1, 2), Side::East, Hanging::Sconce),
    ((-1, 8), Side::West, Hanging::Sign),
    ((17, -1), Side::South, Hanging::Hams),
    ((18, 1), Side::West, Hanging::Onions),
    ((18, 0), Side::West, Hanging::HerbBundles),
    ((18, 4), Side::West, Hanging::Sconce),
    ((18, 10), Side::West, Hanging::Sconce),
    ((3, 15), Side::North, Hanging::CrossedWeapons),
    ((5, 15), Side::North, Hanging::Shield),
    ((7, 15), Side::North, Hanging::Sconce),
    ((11, 15), Side::North, Hanging::Tapestry),
    ((12, 15), Side::North, Hanging::Sconce),
    ((23, -1), Side::South, Hanging::Sconce),
    ((18, 0), Side::East, Hanging::CoatPegs),
    ((26, 6), Side::West, Hanging::Tapestry),
    ((21, 4), Side::South, Hanging::Sconce),
    ((22, 10), Side::South, Hanging::Sconce),
    ((26, 11), Side::West, Hanging::Tapestry),
];

/// A rug: its north-west cell, its size and how it looks.
type RugAt = ((i64, i64), (u8, u8), RugPattern, RugColour);

/// The rugs.
const RUGS: [RugAt; 4] = [
    ((3, 6), (2, 3), RugPattern::Medallion, RugColour::Madder),
    ((6, 4), (3, 2), RugPattern::Braided, RugColour::Walnut),
    ((21, 1), (2, 2), RugPattern::Bordered, RugColour::Indigo),
    ((20, 11), (2, 2), RugPattern::Striped, RugColour::Weld),
];

/// Hangs and lays it all on `level`.
pub(super) fn decorate(level: &mut Level, shell: Shell) {
    for (wall, face, hanging) in HANGINGS {
        level.place_decor(Decor::Wall {
            wall: shell.at(wall),
            face,
            hanging,
        });
    }
    for (at, style) in [
        ((5, 5), ChandelierStyle::ALL[0]),
        ((12, 5), ChandelierStyle::ALL[2]),
    ] {
        level.place_decor(Decor::Chandelier {
            at: shell.at(at),
            style,
        });
    }
    for (at, size, pattern, colour) in RUGS {
        level.place_decor(Decor::Rug {
            at: shell.at(at),
            size,
            rug: Rug { pattern, colour },
        });
    }
}
