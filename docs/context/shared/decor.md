# shared: decor

Decor (tavern furniture plan PR 5, owner's design, 2026-11-22): what hangs on walls, chandeliers and rugs, kept on `dungeon::Level` beside its furniture and sent with its cells. None of it blocks or slows anyone. Owner's rule: nothing hangs from the ceiling, as it confuses the player and the camera, chandeliers the only exception; things on walls hang on a wall cell's face, never on floor. Sconces, chandeliers and candles on tables (`furniture::Candles`) are fixed lights, always lit (a first guess: only put in lived-in places). Placed by code: Place Furniture now, castles and towers (tavern plan PR 8) and The Way Inn later. Drawn by client `dungeon_map` (`decor.rs`; the light by `fixed_lights/candles.rs`).

## Files

- `crates/shared/src/decor.rs`: the module, re-exports. Tests in `decor/tests.rs` (where each fits, replacing, what is sent with a cell, the fixed lights, booths, candles).
- `decor/hanging.rs`: `Hanging`.
- `decor/chandelier.rs`: `ChandelierStyle`.
- `decor/rug.rs`: `Rug`, `RugPattern`, `RugColour`, `RUG_MOST`.
- `decor/piece.rs`: `Decor`, `DecorSlot`.
- `decor/light.rs`: `FixedLight`, `fixed_lights`, the light constants.

## Public API

- `Decor::{Wall { wall, face, hanging }, Chandelier { at, style }, Rug { at, size, rug }}` (serde, `Copy`): on the face of wall cell `wall` turned towards `face` (over the cell beside it there), over a floor cell, or on the floor from its north-west cell, `size` east–west and north–south. `slot()`, `cells()` (what it is sent and seen with: a wall's and the cell in front; a chandelier's; every cell a rug covers), `light()` (its cell and reach: a sconce's from the cell in front), `fits(level)` (a hanging on a `Cell::Wall` with a cell that `takes_furniture` in front, so a sign may hang outside; a chandelier or rug only on `Cell::Floor`, under a ceiling; a rug 1 to `RUG_MOST` (4) each way; stairs left to the caller), `moved(offset)`.
- `DecorSlot::{Wall(cell, side), Ceiling(cell), Floor(cell)}`: where a piece goes, one piece a place.
- `Hanging` (12, `ALL`, `name()`, `light_cells()`): antlers, a boar's head, a shield, crossed weapons, bundles of herbs, hams, strings of onions (on peg rails), a tapestry, a sign, coat pegs, a notice board, a sconce (a light). Their colours and sizes are the client's, from a hash of where they hang.
- `ChandelierStyle` (`ALL`: iron hoop, wooden cross, brass; `candles()`, `name()`).
- `Rug { pattern, colour }`: `RugPattern` (plain, striped, bordered, medallion, braided), `RugColour` (madder, indigo, weld, green, walnut, undyed), each with `ALL`.
- `FixedLight { cell, cells }`, `fixed_lights(level)`: every light decor and candles on tables give, decor's first. `SCONCE_LIGHT_CELLS` (4), `CHANDELIER_LIGHT_CELLS` (6), `CANDLE_LIGHT_CELLS` (3): first guesses.
- `dungeon::Level` keeps it (`dungeon/decor.rs`): `decor()`, `decor_at(x, y)` (what is sent with that cell), `place_decor` (replacing what was in its place, and any rug a rug overlaps; nothing checks it fits), `remove_decor(slot)`.

## Depends on

- `dungeon` (`Level`, `Cell`), `door` (`Side`), `furniture` (`takes_furniture`, `Furniture::Candles`), `serde`.

## Used by

- `dungeon` (`Level`'s decor), `protocol` (`LevelChunk::decor`, `DevFurniture::Decor`), the server (`sites/cells.rs` sends it, `sites/dev_furniture.rs` places it), the client (`common::CurrentDungeon` keeps it, `dungeon_map`'s `decor.rs` and fixed lights, `debug_menu`'s Place Furniture, `dev_tools`' `HOW_DEBUG_FURNITURE=decor`).

## Notes

- Not kept in saves yet: Place Furniture's is gone when the level is made again; the generators that place it (tavern plan PR 8) make it again each time.
- The light lights the place as drawn, never a player: "no light without a lantern" still holds for characters (decisions, 2026-11-22). Tokens near candles are not lit by a real point light (fixed lights only light the cells), to keep an inn full of candles cheap.
- Booths are three ordinary pieces (`furniture::booth`), settles a bench style (`BenchStyle::Settle`); both in `shared::furniture`.
