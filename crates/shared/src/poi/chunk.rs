//! World map chunks and the poi in one.

use serde::{Deserialize, Serialize};

use super::PoiKind;
use crate::rng::Seed;

/// A chunk's position: chunk `(x, y)` covers world grid points from
/// `(x × CHUNK_SIZE, y × CHUNK_SIZE)`. East–west it wraps round the planet,
/// so `x` is always `0..PoiMap::columns()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChunkPos {
    pub x: i64,
    pub y: i64,
}

impl ChunkPos {
    /// The chunk holding world grid point `(x, y)` on a planet `width`
    /// points round (`PoiMap::chunk_at`, for those without a `PoiMap`).
    pub fn containing(x: i64, y: i64, width: i64) -> Self {
        let columns = (width / super::CHUNK_SIZE).max(1);
        Self {
            x: (x.rem_euclid(width) / super::CHUNK_SIZE).min(columns - 1),
            y: y.div_euclid(super::CHUNK_SIZE),
        }
    }
}

/// One poi on the world map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poi {
    pub kind: PoiKind,
    /// The chunk it is in.
    pub chunk: ChunkPos,
    /// Its world grid position. `x` is in the frame the caller asked about,
    /// which may be a whole planet width east or west of the chunk.
    pub x: i64,
    pub y: i64,
    /// What its map (dungeon, building, town) is generated from.
    pub seed: Seed,
    /// Whether it stands in desert (`Biome::Desert`), where more of what
    /// stands is ruined (`Generator::for_site`).
    #[serde(default)]
    pub desert: bool,
}
