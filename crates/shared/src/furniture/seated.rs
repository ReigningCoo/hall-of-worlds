//! A games table or a standing barrel with its seats round it (tavern
//! furniture plan PR 6), as ordinary pieces, for code placing them (Place
//! Furniture, a tavern's layout) to make them alike: players across a board,
//! drinkers round a barrel.

use super::{Barrel, ChairStyle, Furniture, Material, Piece, Wood};
use crate::board_game::BoardGame;
use crate::door::Side;

/// A 1 × 1 piece of `wood` at `at` facing `facing`.
fn single(at: (i64, i64), facing: Side, wood: Wood, furniture: Furniture) -> Piece {
    Piece {
        at,
        size: (1, 1),
        facing,
        material: Material::Wood(wood),
        furniture,
    }
}

/// A seat of `style` beside `table` on its `side`, facing it.
fn seat_beside(table: (i64, i64), side: Side, style: ChairStyle, wood: Wood) -> Piece {
    let chair = Furniture::Chair { style, arms: false };
    single(side.beside(table), side.opposite(), wood, chair)
}

/// A games table at `at` for `game`, the light player looking `facing`
/// across the board, with a chair of `style` for each player (behind the
/// board as each sits).
pub fn games_table(
    at: (i64, i64),
    game: BoardGame,
    facing: Side,
    style: ChairStyle,
    wood: Wood,
) -> [Piece; 3] {
    [
        single(at, facing, wood, Furniture::GamesTable { game }),
        seat_beside(at, facing.opposite(), style, wood),
        seat_beside(at, facing, style, wood),
    ]
}

/// A barrel table at `at` with a stool on each of `sides`.
pub fn barrel_table(at: (i64, i64), barrel: Barrel, sides: &[Side], wood: Wood) -> Vec<Piece> {
    let table = single(at, Side::South, wood, Furniture::Barrel { barrel });
    let stools = (sides.iter()).map(|&side| seat_beside(at, side, ChairStyle::Stool, wood));
    std::iter::once(table).chain(stools).collect()
}

#[cfg(test)]
mod tests;
