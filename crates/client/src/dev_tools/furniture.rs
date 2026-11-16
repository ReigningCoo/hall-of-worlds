//! `HOW_DEBUG_FURNITURE`: draw every table in one style and material, or in
//! each in turn, or a table of every style beside the player, to look at
//! their models. Only what is drawn changes: the server's pieces stay as they
//! are.

use bevy::prelude::*;
use shared::furniture::{Furniture, Material, TableStyle};
use std::env;

use shared::door::Side;
use shared::furniture::Piece;

use crate::common::{CurrentDungeon, MapPosition};
use crate::player::Player;

const STYLES: [TableStyle; 6] = TableStyle::ALL;
const MATERIALS: [Material; 13] = Material::ALL;

/// The style and material every table is drawn in: `None` for each in turn,
/// table by table.
#[derive(Resource)]
struct ForcedTables {
    style: Option<usize>,
    material: Option<usize>,
}

/// `HOW_DEBUG_FURNITURE=style[,material]`: indexes into `STYLES` and
/// `MATERIALS`, `all` for each in turn, or `show` for a table of every style
/// in two rows south-east of the player.
pub(super) fn add(app: &mut App) {
    let Ok(text) = env::var("HOW_DEBUG_FURNITURE") else {
        return;
    };
    if text.trim() == "show" {
        app.add_systems(Update, show_tables);
        return;
    }
    let mut values = text
        .split(',')
        .map(|value| value.trim().parse::<usize>().ok());
    let style = values.next().flatten().map(|n| n % STYLES.len());
    let material = values.next().flatten().map(|n| n % MATERIALS.len());
    app.insert_resource(ForcedTables { style, material })
        .add_systems(Update, force_tables);
}

fn force_tables(forced: Res<ForcedTables>, mut dungeon: ResMut<CurrentDungeon>) {
    let tables: Vec<_> = dungeon
        .level
        .furniture()
        .filter(|piece| piece.furniture.is_table())
        .copied()
        .collect();
    let mut changed = Vec::new();
    for (i, piece) in tables.into_iter().enumerate() {
        let style = STYLES[forced.style.unwrap_or(i % STYLES.len())];
        let material = match forced.material {
            Some(n) => MATERIALS[n],
            // Stone for stone styles, wood for the rest, each in turn.
            None if TableStyle::STONE.contains(&style) => MATERIALS[8 + i % 5],
            None => MATERIALS[i % 8],
        };
        let wanted = Furniture::Table { style };
        if piece.furniture != wanted || piece.material != material {
            changed.push(Piece {
                furniture: wanted,
                material,
                ..piece
            });
        }
    }
    if !changed.is_empty() {
        for piece in changed {
            dungeon.level.place_furniture(piece);
        }
    }
}

/// A table of every style, each its own size and material, in two rows of
/// three south-east of the player, once they stand on a level.
fn show_tables(
    player: Query<&MapPosition, With<Player>>,
    mut dungeon: ResMut<CurrentDungeon>,
    mut anchor: Local<Option<MapPosition>>,
) {
    if dungeon.info.is_none() {
        return;
    }
    // Where the player first stood on the level, so the tables stay put.
    if anchor.is_none() {
        *anchor = player.single().ok().copied();
    }
    let Some(at) = *anchor else {
        return;
    };
    let shown = [
        (STYLES[0], (3, 2), Side::East, MATERIALS[0]),
        (STYLES[1], (2, 3), Side::South, MATERIALS[1]),
        (STYLES[2], (2, 1), Side::East, MATERIALS[3]),
        (STYLES[3], (1, 2), Side::South, MATERIALS[6]),
        (STYLES[4], (3, 1), Side::East, MATERIALS[11]),
        (STYLES[5], (2, 2), Side::East, MATERIALS[8]),
    ];
    for (i, (style, size, facing, material)) in shown.into_iter().enumerate() {
        let (column, row) = (i as i64 % 3, i as i64 / 3);
        let piece = Piece {
            at: (at.x + 1 + column * 5, at.y + 3 + row * 5),
            size,
            facing,
            material,
            furniture: Furniture::Table { style },
        };
        if dungeon.level.furniture_at(piece.at.0, piece.at.1) != Some(&piece) {
            dungeon.level.place_furniture(piece);
        }
    }
}
