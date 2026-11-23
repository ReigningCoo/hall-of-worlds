# shared: board_game

Board games on games tables (tavern furniture plan PR 6): chess and Hnefatafl, their boards, men and starting places. Only drawn for now (client `dungeon_map`, on `furniture::Furniture::GamesTable`); kept apart from furniture so the games can be made playable later, the table keeping a `Board` and rules here checking moves.

## Files

- `crates/shared/src/board_game.rs`: the module, re-exports. Tests in `board_game/tests.rs`.
- `board_game/game.rs`: `BoardGame` (`ALL`, `size()`, `start()`, `marked()`).
- `board_game/board.rs`: `Board`, `Man`, `Player`.

## Public API

- `BoardGame::{Chess, Hnefatafl}` (serde): `size()` (8, 11), `start()` (the board as a game starts: chess's light men on ranks 0 and 1; Hnefatafl's Copenhagen layout, the king on the throne with 12 defenders round him (light), 24 attackers in a T at each edge's middle (dark)), `marked(square)` (Hnefatafl's corners and throne).
- `Board` (serde): `empty(size)`, `size()`, `at((file, rank))`, `set(square, man)` (nothing off the board), `men()` (every man with its square). Files from Light's left, rank 0 nearest Light.
- `Player::{Light, Dark}`; `Man::{King, Queen, Rook, Bishop, Knight, Pawn, Defender, Attacker}`.

## Depends on

- Nothing of the game's.

## Used by

- `shared::furniture` (`Furniture::GamesTable`, `games_table`), client `dungeon_map` (the board's model), `debug_menu` (Place Furniture), `dev_tools` (`HOW_DEBUG_FURNITURE=games`).
