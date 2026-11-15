# shared: furniture

Tables, chairs and benches in sites' rooms, of wood or stone, and where they go. Pieces are kept on `dungeon::Level` (as doors are) and sent with its cells; they never block, but moves over them cost more, so routes go round. Material and style are for models only and are never named to players. Drawing is client `dungeon_map` (placeholder boxes for now).

## Files

- `crates/shared/src/furniture.rs`: the module, re-exports. Tests in `furniture/tests.rs` (never cuts anything off across kinds, seeds and ruins; only on room floor away from doors, stairs and the way in; seats face a table or have a wall behind; seats match their room's table; none in unfurnished kinds; the same every time; golden test of a stone cottage's ground floor).
- `furniture/piece.rs`: `Piece`, `Furniture`, `TABLE_COST_PER_MILLE`, `SEAT_COST_PER_MILLE`.
- `furniture/material.rs`: `Material`, `Wood`, `Stone` with their weights.
- `furniture/style.rs`: `TableStyle`, `ChairStyle`, `BenchStyle`, and which seat styles go with each table style.
- `furniture/place.rs`: `furnish`, the entry point. In `place/`: `setting.rs` (`Setting::for_kind`), `palette.rs` (a site's woods and stone, a room's material), `rooms.rs` (rooms' floor cells, the cells that may be furnished), `reach.rs` (the never-cut-off check), `placer.rs` (`Placer`: what a level's placing keeps track of, a room's matching set, ruin's losses), `tables.rs`, `seats.rs`, `loose.rs` (seats against walls).

## Public API

- `Piece { at, size, facing, material, furniture }` (serde): its north-west cell, its cells east–west and north–south, which way it faces (`door::Side`: a seat the way its sitter looks; a table east or south, the way its length runs), what it is made of and what it is. `cells()`, `covers(cell)`, `moved(offset)`.
- `Furniture::{Table { style }, Chair { style, arms }, Bench { style }}`: `is_table()`, `cost_per_mille()`.
- `TABLE_COST_PER_MILLE` (3000) and `SEAT_COST_PER_MILLE` (1500): how much longer a move over one takes (`dungeon::Level::furniture_cost_per_mille`, used by `movement::LevelWay` and `pathfinding::DistanceMap`).
- `Material::{Wood(Wood), Stone(Stone)}`; `Wood` (oak, walnut, ash, pine, elm, beech, cherry, rare ebony) and `Stone` (granite, sandstone, limestone, marble, slate), each with `WEIGHTS`.
- `TableStyle` (trestle, joined, plank legs, pedestal (1 × 1 and 1 × 2 only, `fits`), stone slab, stone block; `WOOD`, `STONE`, `chairs()`, `benches()`), `ChairStyle` (stool, back-stool, ladder-back, panel-back, X-frame, stone block), `BenchStyle` (splayed-leg plank, board-ended, long stool with rail, stone slab).
- `furnish(site_level, kind, seed, index, ruin)`: called by `site::Site::level` last (not for a Dev Area, whose building brings its own).
- `Setting { cells_per_table, cells_per_loose, stone_percent }`, `Setting::for_kind(kind)`: wooden buildings all wood; stone and adobe buildings 15% stone; pyramids 80%; crypts, graves and mausoleums sparse (a table per 120 cells) and 85% stone; `None` for natural caves, boulders, dug ruins, barrels, tents, ice shelters, towns and oases.

## Depends on

- `shared::dungeon` (`Level`, `Cell`, `RoomId`), `shared::site` (`SiteLevel`), `shared::door` (`Side`), `shared::poi` (`PoiKind`), `shared::rng`.

## Used by

- `shared::dungeon` (`Level`'s furniture), `shared::site` (`Site::level`, Dev Area), `shared::wilderness` (cutouts), `shared::movement`, `shared::pathfinding`, `shared::mob` and `shared::quest` (spawns), `shared::protocol` (`LevelChunk::furniture`), `server` (`sites`), client `common` (`CurrentDungeon`), `dungeon_map`.

## Notes

- Placing (owner's design, 2026-11-15): everything from hashes of `seed.derive("furniture")`, the level and the room, never from a generator's sequence, and after the generator and any ruin, so no generator version changed and sites' golden tests didn't move. Only on `Cell::Floor` with a room; never on a cell beside a door (inside or out), within a cell of stairs, or where anyone arrives.
- About one table per `cells_per_table` floor cells of a room; the first table's size by the room's (2 × 3 from 90 cells, down to 1 × 2 or 1 × 1 in small rooms), later ones by the room shared out; nearest the room's middle, with both long sides free and nothing within two cells. Seats always along both long sides (a bench the side's length in rooms that have benches, else a chair a cell), and at the ends 40% of the time. Chairs with arms: wooden only, 60% at a table's end, 15% elsewhere. Loose seats against a plain wall, back to it, about one per `cells_per_loose`; benches 1 to 3 along the wall. A room's pieces share one material and style set; a site uses one or two woods and one stone.
- Never cutting anything off: while placing, every piece counts as wall, and a piece is kept only if a 4-way fill from each way in (entrance and stairs, every door open, no jumps) still reaches all it did without that piece.
- Ruins: each piece is kept at 100 − 15 × ruin %, decided while placing, so a lost table never leaves its seats closing off its empty cells.
