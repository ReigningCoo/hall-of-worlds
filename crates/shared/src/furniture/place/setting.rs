//! Which kinds of site have furniture, how much, and of what.

use crate::poi::PoiKind;

/// How a kind of site is furnished.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setting {
    /// About one table per this many floor cells of a room.
    pub cells_per_table: u32,
    /// About one loose seat against a wall per this many floor cells.
    pub cells_per_loose: u32,
    /// Of rooms, how many have stone furniture rather than wood, in 100ths.
    pub stone_percent: u32,
}

/// Wooden buildings: all wood.
const WOODEN: Setting = Setting {
    cells_per_table: 30,
    cells_per_loose: 40,
    stone_percent: 0,
};

/// Stone and adobe buildings: mostly wood, some stone.
const STONE_BUILT: Setting = Setting {
    stone_percent: 15,
    ..WOODEN
};

/// Pyramids: mostly stone.
const PYRAMID: Setting = Setting {
    stone_percent: 80,
    ..WOODEN
};

/// Crypts, graves and mausoleums: sparse, mostly stone.
const CRYPT: Setting = Setting {
    cells_per_table: 120,
    cells_per_loose: 150,
    stone_percent: 85,
};

impl Setting {
    /// How a kind of site is furnished: `None` for natural caves, tents, ice
    /// shelters, towns, oases, dug ruins and barrels.
    pub fn for_kind(kind: PoiKind) -> Option<Self> {
        use PoiKind::*;
        match kind {
            ThatchedHut | StiltHut | GreenStiltHut | RedRoofCottage | GreyRoofCottage
            | SnowyCottage | WoodenTower | Farm | HerbalistHut => Some(WOODEN),
            Castle | Fort | StoneTower | StoneCottage | Warehouse | ArcheryRange | AdobeHouse
            | AdobeStore => Some(STONE_BUILT),
            StonePyramid | SandPyramid => Some(PYRAMID),
            Crypt | MossyCrypt | BloodyCrypt | Mausoleum | Grave => Some(CRYPT),
            Boulders | Ruins | GrassyCave | EarthCave | RockCave | IceCave | RedTent | BlueTent
            | GreenTent | IceShelter | Barrel | Barrels | Hamlet | Village | Town | City
            | Oasis => None,
        }
    }
}
