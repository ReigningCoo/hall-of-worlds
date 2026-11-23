//! `HOW_DEBUG_FURNITURE=games`: the tavern furniture plan's PR 6 pieces
//! round eight cells east of where the player first stands, only drawn: a
//! games table of each game with chairs (candles on the second), a barrel
//! table of each kind with stools round it, every stack of crates and every
//! heap of sacks.

use bevy::prelude::*;
use shared::board_game::BoardGame;
use shared::door::Side;
use shared::furniture::{
    Barrel, Candles, ChairStyle, Crates, Furniture, Material, Piece, Sacks, Wood, barrel_table,
    games_table,
};

use crate::common::{CurrentDungeon, MapPosition};
use crate::player::Player;

/// Puts the pieces round where the player first stood, once on a level.
pub(super) fn show_games(
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
    let one = |at, furniture| Piece {
        at,
        size: (1, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture,
    };
    let mut pieces: Vec<Piece> = Vec::new();
    let games = [
        ((x - 4, y), Side::South, Wood::Walnut),
        ((x - 1, y), Side::East, Wood::Oak),
    ];
    for ((at, facing, wood), game) in games.into_iter().zip(BoardGame::ALL) {
        pieces.extend(games_table(at, game, facing, ChairStyle::LadderBack, wood));
    }
    pieces.push(one(
        (x - 4, y),
        Furniture::Candles {
            candles: Candles::Candlestick,
        },
    ));
    let all_round = [Side::North, Side::East, Side::South, Side::West];
    pieces.extend(barrel_table(
        (x + 3, y),
        Barrel::Bare,
        &all_round[..2],
        Wood::Oak,
    ));
    pieces.extend(barrel_table(
        (x + 6, y),
        Barrel::Topped,
        &all_round,
        Wood::Pine,
    ));
    pieces.push(one(
        (x + 6, y),
        Furniture::Candles {
            candles: Candles::Pair,
        },
    ));
    for (i, crates) in Crates::ALL.into_iter().enumerate() {
        pieces.push(one(
            (x - 4 + 2 * i as i64, y + 4),
            Furniture::Crates { crates },
        ));
    }
    for (i, sacks) in Sacks::ALL.into_iter().enumerate() {
        pieces.push(one(
            (x + 4 + 2 * i as i64, y + 4),
            Furniture::Sacks { sacks },
        ));
    }
    for piece in pieces {
        level.place_furniture(piece);
    }
}
