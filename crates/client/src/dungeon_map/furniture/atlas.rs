//! Where each wood's grain and each stone are in the furniture atlas: the
//! order `tools/make_furniture_atlas.py` puts them.

use shared::furniture::{Material, Stone, Wood};

/// Tiles across and down the atlas.
const COLUMNS: f32 = 4.0;
const ROWS: f32 = 3.0;
/// Pixels across a tile, for keeping clear of its neighbours.
const TILE_PIXELS: f32 = 64.0;

const STRAIGHT: usize = 0;
const COARSE: usize = 1;
const WAVY: usize = 2;
const KNOTTY: usize = 3;
const GRANITE: usize = 4;
const SANDSTONE: usize = 5;
const LIMESTONE: usize = 6;
const MARBLE: usize = 7;
const SLATE: usize = 8;
/// A wood's end, its growth rings.
pub(super) const END_GRAIN: usize = 9;

/// The picture a material's faces show.
pub(super) fn tile(material: Material) -> usize {
    match material {
        Material::Wood(wood) => match wood {
            Wood::Ash | Wood::Beech | Wood::Cherry | Wood::Ebony => STRAIGHT,
            Wood::Oak => COARSE,
            Wood::Walnut | Wood::Elm => WAVY,
            Wood::Pine => KNOTTY,
        },
        Material::Stone(stone) => match stone {
            Stone::Granite => GRANITE,
            Stone::Sandstone => SANDSTONE,
            Stone::Limestone => LIMESTONE,
            Stone::Marble => MARBLE,
            Stone::Slate => SLATE,
        },
    }
}

/// Picture coordinates in a tile, from coordinates in it (0 to 1 across,
/// clamped), kept half a pixel clear of its neighbours.
pub(super) fn uv(tile: usize, u: f32, v: f32) -> [f32; 2] {
    let (column, row) = (tile as f32 % COLUMNS, (tile as f32 / COLUMNS).floor());
    let edge = 0.5 / TILE_PIXELS;
    let inside = |t: f32| edge + t.clamp(0.0, 1.0) * (1.0 - 2.0 * edge);
    [(column + inside(u)) / COLUMNS, (row + inside(v)) / ROWS]
}
