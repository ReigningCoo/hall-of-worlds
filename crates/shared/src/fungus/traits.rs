//! A fungus's plainer properties: what sort of fungus it is, where it grows
//! (owner, 2026-11-19) and on what. Where and on what are data for placing
//! patches (fungi plan PR 3).

use serde::{Deserialize, Serialize};

/// What sort of fungus or lichen it is, for players' names of an unknown one
/// (fungi plan PR 7) and the wiki.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Group {
    /// A cap on a stem, or a puffball or earthball.
    Mushroom,
    /// A shelf or hoof on wood.
    Bracket,
    Jelly,
    /// A lump grown under the ground.
    Truffle,
    /// A hard buried lump of threads (poria, stonebread).
    Sclerotium,
    Cup,
    /// A crust or plate spreading over wood (dry rot).
    Crust,
    /// A fungus living with algae, which needs light.
    Lichen,
}

/// Where it grows (owner, 2026-11-19, decision 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Where {
    /// Anywhere underground: cave levels, basements and cellars, dark or not.
    Deep,
    /// Underground only near daylight: cave mouths' levels, skylights, under
    /// holes to the surface.
    Dim,
    /// Buried: dug from cave earth floors, and from the ground under its trees
    /// above.
    Dug,
    /// Above ground only, in its biomes.
    Surface,
}

/// What it grows on or by (owner, 2026-11-19, decision 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Niche {
    /// Old timber, roots, stumps, felled trees.
    Wood,
    /// Bat roosts, stables.
    Dung,
    /// Damp earth, compost.
    Soil,
    /// Crypts, bones.
    Remains,
    /// By pools, under drips.
    Water,
    /// Beside lava.
    Lava,
    /// Living trees above ground.
    Trees,
    /// With tree roots, so under or near trees.
    Roots,
    Rock,
    /// Buried insects (caterpillar fungus).
    Insects,
    /// Termites' nests.
    Termites,
}
