# client: crafting_bench

Crafting benches and all furniture with places on the client (crafting benches plan PR 3, `shared::crafting_bench`; any `shared::furniture::Holder` since the tavern furniture plan PR 1, cask racks too, wine and spirit racks since PR 2, bar counters since PR 3, fireplaces' log baskets since PR 4, and chests and wardrobes since PR 7, their grids shown as a list as the ground's): what the server tells lies on each piece on the level, the bench the player is at now, and a piece's panel, a grid of what lies on it like a shop's stock. Clicking (or tapping) a piece walks up to it, turns to it and opens the panel. Not the models (`dungeon_map`: `furniture/crafting_bench.rs`, `bench_tools.rs`, `bench_picking.rs`), nor Alchemy at a bench (`alchemy`, `abilities`), nor sending what is let go on the grid (`inventory`).

## Files

- `crates/client/src/crafting_bench/mod.rs`: `CraftingBenchPlugin`, `AT` (where the panel opens: right of the Character panel), `COLUMNS` (6). Tests in `tests.rs`.
- `state.rs`: `BenchPanel` (the panel open, one at a time: `OpenBench` with the piece, document, grid node and `widgets::StockGrid`), `BenchHeading` (the bench walked to and the cell in front), `in_reach` (a cell's middle within `COLLECT_REACH`, as the server checks), `nearest_cell`, `front_cell` (the free cell in front nearest the player).
- `markup.rs`: `BENCH` (`bench/panel`, `bench/tools` the `object-grid`, `bench/refused`; a bar's `bench/bar`, `into`, `no_drinks`, `drinks_scroll`, `drinks` and the `drink` row template), `REFUSED` ("The bench won't take that.").
- `open.rs`: `change_of_mind`, `follow_clicked_bench` (`dungeon_map::BenchClicked`: in reach, `common::FaceTowards` its nearest cell and open; else `TravelTo` the front cell), `open_on_arrival` (`PostUpdate`), `open_panel` (any piece with places, titled by `Holder::name`: "Alchemy Bench", "Stove", "Hearth", "Range", "Pin Rack", "Tun Stand", "Chest", "Wardrobe"), `title`.
- `here.rs`: `keep_bench_here` (`common::BenchHere` from `shared::crafting_bench::bench_faced` and `common::Benches`, every frame; nothing off a level).
- `server.rs`: `read_server` (`BenchNews::Tools` into `common::Benches`, with the hearths it says are lit, catching up an open grid; `shown`: what lies on it (`Held::items`: a bench's tools then a Range's storage, a rack's casks), one grid; `Refused` puts the grid back as told and shows `REFUSED`; `Welcome`, `EnterLevel`, `EnterHall`, `OnWorldMap` forget every bench and close the panel).
- `grid.rs`: `bind_grid`, `note_gifts` (`GaveItem` / `TookItem` for the bench's id), `draw_grid`, `show_gift_aim` (green anywhere free, whatever is held).
- `close.rs`: `close_on_escape` (`PreUpdate`, before `dom`), `close_when_left` (close button, out of reach, off the map), `close_panel`.
- `pour.rs` (tavern furniture plan PR 3): `BarList`, `show_drinks` (a bar counter's panel: shows `bench/bar`, the carried vessels to fill in `bench/into` (`shared::bar::vessels_to_fill`, the first with room chosen until the player chooses), and a row a drink (`bench/drink` template: `name.N`, `pour.N`) for each drink stored in its room as told (`shared::bar::room_storage`, `drinks_in` on `common::Benches`), again whenever they change), `answer_drinks` (the choice; Pour sends `BenchAsk::Pour`).
- `debug_api.rs` (debug API only): `client.benches` (the level's pieces with places with the kinds on each, the bench here, the panel's; `click: [x, y]` clicks the piece on that cell).

## Public API

- `CraftingBenchPlugin`. What it keeps for others is in `common` (`Benches`, `BenchHere`).

## Depends on

- `common` (`Benches`, `BenchHere`, `CurrentDungeon`, `SpotPosition`, `Facing`, `FaceTowards`, `GameView`, `GaveItem`, `TookItem`, `HeldStock`, `HeldOverGifts`, `PointerSystems`), `dungeon_map` (`BenchClicked`), `player` (`Player`, `Arrived`, `TakeStep`, `Walking`, `TravelTo`, `PlayerSystems`), `connection` (`FromServer`), `dom`, `widgets` (`StockGrid`, `StockGridNode`, `StockOf`, `UiId`, `show_stock_aim`), `inventory` (`Carried`), `shared::crafting_bench`, `shared::bar`, `shared::protocol` (`BenchNews`, `BenchAsk::Pour`), `connection` (`ToServer`).

## Used by

- Nothing directly; `dungeon_map`, `inventory`, `alchemy` and `abilities` read `common::Benches` / `BenchHere`.

## Notes

- Owner's design (2026-11-20): the panel is like the trade view, unlabelled, with no pictures or tips saying which kinds a bench takes; a second tool of a kind already there swaps (the server's); anyone may use or take; using one is within reach and facing it, as Collect Water.
- The grid belongs to a stand-in id made from the bench's cell (`common::Benches::id_of`, read back by `at_of`), so the stock machinery (`StockCell`, `TakesGifts`) works unchanged and `inventory` knows to send `BenchAsk::Put` / `Take` instead of a gift or a trade. The server puts what is taken wherever it fits; `Inventory` puts the guess right.
- Clicks and taps alike: `dungeon_map`'s click is a tap too, and the grid is the shared item grid, so all of it works by touch.
