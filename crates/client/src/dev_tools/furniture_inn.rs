//! `HOW_DEBUG_FURNITURE=inn`: the tavern furniture plan's PR 7 pieces round
//! eight cells east of where the player first stands, only drawn: a single
//! and a double bed facing each way, and every chest and wardrobe in a row
//! three cells south.

use bevy::prelude::*;
use shared::chest::{Chest, Wardrobe};
use shared::door::Side;
use shared::furniture::{Bed, Furniture, Material, Piece, Wood, footprint};

use crate::common::{CurrentDungeon, MapPosition};
use crate::player::Player;

/// Puts the pieces round where the player first stood, once on a level.
pub(super) fn show_inn(
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
    let x = x + 4;
    *done = true;
    let piece = |at, (long, short), facing, furniture, wood| Piece {
        at,
        size: footprint(furniture, (long, short), facing),
        facing,
        material: Material::Wood(wood),
        furniture,
    };
    let mut pieces = Vec::new();
    let facings = [Side::South, Side::East, Side::North, Side::West];
    for (row, bed) in Bed::ALL.into_iter().enumerate() {
        for (i, facing) in facings.into_iter().enumerate() {
            let at = (x + 3 * i as i64, y - 6 + 3 * row as i64);
            let size = (Bed::LENGTH, bed.width());
            pieces.push(piece(at, size, facing, Furniture::Bed { bed }, Wood::Oak));
        }
    }
    let mut along = x;
    for chest in Chest::ALL {
        let size = (chest.length(), 1);
        pieces.push(piece(
            (along, y + 3),
            size,
            Side::South,
            Furniture::Chest { chest },
            Wood::Walnut,
        ));
        along += i64::from(chest.length()) + 1;
    }
    for wardrobe in Wardrobe::ALL {
        let size = (wardrobe.length(), 1);
        let furniture = Furniture::Wardrobe { wardrobe };
        pieces.push(piece(
            (along, y + 3),
            size,
            Side::South,
            furniture,
            Wood::Pine,
        ));
        along += i64::from(wardrobe.length()) + 1;
    }
    for piece in pieces {
        dungeon.level.place_furniture(piece);
    }
}
