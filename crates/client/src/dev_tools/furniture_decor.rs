//! `HOW_DEBUG_FURNITURE=decor`: every piece of decor (tavern furniture plan
//! PR 5) round eight cells east of where the player first stands, only drawn: a wall three cells
//! north with every hanging on its south face, a chandelier of each style,
//! two booths with candles of each kind on their tables, and a rug of each
//! pattern further south. The wall is set on the client's copy of the level
//! alone, so it blocks nothing.

use bevy::prelude::*;
use shared::decor::{ChandelierStyle, Decor, Hanging, Rug, RugColour, RugPattern};
use shared::door::Side;
use shared::dungeon::Cell;
use shared::furniture::{Candles, Furniture, Material, Piece, Wood, booth};

use crate::common::{CurrentDungeon, MapPosition};
use crate::player::Player;

/// Puts the decor round where the player first stood, once on a level.
pub(super) fn show_decor(
    player: Query<&MapPosition, With<Player>>,
    mut dungeon: ResMut<CurrentDungeon>,
    mut anchor: Local<Option<MapPosition>>,
    mut done: Local<bool>,
) {
    if dungeon.info.is_none() || *done {
        return;
    }
    if anchor.is_none() {
        *anchor = player.single().ok().copied();
    }
    let Some(MapPosition { x, y }) = *anchor else {
        return;
    };
    // Clear of a Dev Area's west wall.
    let x = x + 8;
    *done = true;
    let level = &mut dungeon.level;
    for dx in -7..=8 {
        level.set(x + dx, y - 3, Cell::Wall);
    }
    for (i, hanging) in Hanging::ALL.into_iter().enumerate() {
        let wall = (x - 6 + i as i64, y - 3);
        level.place_decor(Decor::Wall {
            wall,
            face: Side::South,
            hanging,
        });
    }
    for (i, style) in ChandelierStyle::ALL.into_iter().enumerate() {
        let at = (x - 4 + 4 * i as i64, y);
        level.place_decor(Decor::Chandelier { at, style });
    }
    let booths = [
        booth((x - 6, y + 2), 3, 2, true, Wood::Oak),
        booth((x - 1, y + 2), 2, 1, false, Wood::Walnut),
    ];
    for piece in booths.into_iter().flatten() {
        level.place_furniture(piece);
    }
    let tops = [(x - 6, y + 3), (x - 4, y + 4), (x, y + 2)];
    for (at, candles) in tops.into_iter().zip(Candles::ALL) {
        level.place_furniture(Piece {
            at,
            size: (1, 1),
            facing: Side::South,
            material: Material::Wood(Wood::Oak),
            furniture: Furniture::Candles { candles },
        });
    }
    for (i, pattern) in RugPattern::ALL.into_iter().enumerate() {
        let colour = RugColour::ALL[i % RugColour::ALL.len()];
        level.place_decor(Decor::Rug {
            at: (x - 7 + 3 * i as i64, y + 7),
            size: (2, 3),
            rug: Rug { pattern, colour },
        });
    }
}
