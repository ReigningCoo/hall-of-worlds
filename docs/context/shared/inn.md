# shared: inn

The Way Inn's fixed layout (The Way Inn plan, `docs/context/the-way-inn.md`): its two floors as site levels, written in code like `shared::hall`, with no generator and no seed. Only shapes (cells, rooms, doors, stairs) and the places kept for furniture; getting in and out, instances and furnishing come in later PRs.

## Files

- `crates/shared/src/inn/mod.rs`: `FLOORS`, `GROUND`, `UPSTAIRS`, `WIDTH` (41), `DEPTH` (50), `level`, `rooms`. In `inn/`: `room.rs` (`RoomKind`, `InnRoom`, `CORNER_TIER`), `builder.rs` (`Built`: carving rooms out of solid wall, doors closed or locked, windows; `span`), `ground.rs` (the ground floor), `upstairs.rs` (the H and the strips of bedrooms, `lengths`), `spots.rs` (fixed places, public).

## Public API

- `level(index) -> Option<SiteLevel>`: `GROUND` (0) or `UPSTAIRS` (1); one staircase each on `spots::STAIRS` (Up to level 1, Down to level 0); the ground floor's `entrance` is `spots::ARRIVAL` (inside the entrance door). Doors start closed; the treasury's is locked (`Level::is_locked`). Cells `0..WIDTH` × `0..DEPTH`, the outer wall all round, no windows out.
- `rooms(index) -> Vec<InnRoom>`: `InnRoom { id, kind, rects, door }` in id order (ids from 1), `contains`, `area`. `RoomKind`: `Foyer` (with the archway's cell), `Main` (the bar, its gap and storage too), `PrivateRoom` (doored), `Booth` (open), `Kitchen`, `Brewing`, `Ageing`, `Treasury`, `Corridor`, `Bedroom { tier }` (1 beside the stairs up to `CORNER_TIER` 4, the largest).
- `spots`: `MIDDLE_X`, `STAIRS`, `ENTRANCE`, `ARRIVAL`, `FOYER`, `ARCHWAY`, `FOYER_WEST` / `FOYER_EAST` (its furniture), `ROW_FIRST_Y`, `ROW_PITCH`, `ROW_COUNT` (private rooms and booths), `STORAGE`, `BAR_GAP`, `BAR`, `BAR_FLAP`, `BACK_FIREPLACE`, `FRONT_FIREPLACES`, `TABLES` (where scattered tables go), `KITCHEN_DOOR`, `BREWING_DOOR`, `AGEING_DOOR`, `STAIR_DOWN_CORNER`, `TREASURY`, `TREASURY_DOOR`, `TREASURY_WINDOWS`. Rectangles are `site::Rect`.

## Depends on

- `shared::dungeon` (`Level`, `Cell`, `RoomId`), `shared::site` (`SiteLevel`, `Stairs`, `Exit`, `Rect`), `shared::door` (`Swing`).

## Used by

- Nothing yet (PR 2 puts players in it). `hall::inn_way` is the way in from the Hall.

## Notes

- Golden test: both floors' fingerprints (cells, rooms, doors' state, stairs); changing the layout changes them, only on purpose. Print both floors: `cargo test -p shared print_inn -- --ignored --nocapture`.
- Upstairs strips run from beside the crossbar's rows to an outer wall; bedroom lengths 3, 4, 5... while one a row longer still fits after, the last taking the rest (`lengths`). The first bedroom of each strip between the legs has its door on the crossbar.
