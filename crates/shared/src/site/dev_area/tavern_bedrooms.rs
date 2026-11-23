//! The Tavern Dev Area's bedrooms' furniture, in the building's own cells
//! (`Shell`): beds against the east wall, chests and wardrobes.

use super::tavern::Shell;
use super::tavern_furniture::{OAK, PINE, WALNUT, walled};
use crate::chest::{Chest, Wardrobe};
use crate::door::Side;
use crate::furniture::{Bed, Furniture, Piece};

/// Each bedroom: a double bed, a chest and a wardrobe; the middle one two
/// single beds with a small chest between them.
pub(super) fn bedrooms(shell: Shell) -> Vec<Piece> {
    let bed = |bed: Bed| Furniture::Bed { bed };
    let chest = |chest| Furniture::Chest { chest };
    let wardrobe = |wardrobe| Furniture::Wardrobe { wardrobe };
    vec![
        walled(shell, bed(Bed::Double), (24, 0), (2, 2), Side::East, WALNUT),
        walled(
            shell,
            chest(Chest::Large),
            (20, 0),
            (2, 1),
            Side::North,
            WALNUT,
        ),
        walled(
            shell,
            wardrobe(Wardrobe::Wide),
            (22, 3),
            (2, 1),
            Side::South,
            WALNUT,
        ),
        walled(shell, bed(Bed::Single), (24, 5), (2, 1), Side::East, PINE),
        walled(shell, bed(Bed::Single), (24, 8), (2, 1), Side::East, PINE),
        walled(
            shell,
            chest(Chest::Small),
            (25, 7),
            (1, 1),
            Side::East,
            PINE,
        ),
        walled(
            shell,
            wardrobe(Wardrobe::Narrow),
            (20, 5),
            (1, 1),
            Side::North,
            PINE,
        ),
        walled(shell, bed(Bed::Double), (24, 12), (2, 2), Side::East, OAK),
        walled(
            shell,
            chest(Chest::Small),
            (25, 14),
            (1, 1),
            Side::East,
            OAK,
        ),
        walled(
            shell,
            wardrobe(Wardrobe::Wide),
            (20, 14),
            (2, 1),
            Side::South,
            OAK,
        ),
    ]
}
