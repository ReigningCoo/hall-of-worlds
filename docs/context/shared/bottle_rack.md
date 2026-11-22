# shared: bottle_rack

Wine racks and spirit racks (tavern furniture plan PR 2, owner's design, 2026-11-22): furniture with places (`furniture::Held`) for bottles, 1 to 3 cells long, 1 deep, of wood. A wine rack stands full height against a wall, wine bottles lying on their sides in a diamond lattice; a spirit rack is three stepped shelves of upright bottles rising towards the back, standing on a table or a bar counter: the first furniture standing on furniture (`Furniture::stands_on_furniture`, `dungeon::Level::furniture_on_top`). Kept by the server as cask racks are (`server.md`, `LevelSave::bottle_racks`); drawn by client `dungeon_map` (`furniture/bottle_rack.rs`, `rack_bottles.rs`), opened into `crafting_bench`'s panel.

## Files

- `crates/shared/src/bottle_rack.rs`: the module, re-exports. Tests in `bottle_rack/tests.rs`.
- `bottle_rack/rack.rs`: `BottleRack`, `BottleRackKind`, `BottlePlace`, `WINE_ROWS`, `SPIRIT_SHELVES`.
- `bottle_rack/bottles.rs`: `RackBottles`.

## Public API

- `BottleRackKind::{Wine, Spirit}` (`ALL`, `name()`: "Wine Rack", "Spirit Rack").
- `BottleRack { kind, length }` (serde, `Copy`, `Hash`: in `furniture::Furniture::BottleRack { rack }`): `LENGTHS` (1 to 3), `all()`, `is_well_made()`, `name()`, `places()` (each bottle's `BottlePlace { along, row }`: its middle in twelfths of a cell along the rack, its lattice row or shelf from the bottom; the bottom row first, as filled), `capacity()`.
- Counts: a wine rack's `WINE_ROWS` (8) rows alternate 3 a cell and one fewer in all, each in the gap of the rows below and above (1 × 1 holds 20, 1 × 2 44, 1 × 3 68); a spirit rack's `SPIRIT_SHELVES` (3) hold 3 a cell (9, 18, 27), shelf 0 the lowest, at the front.
- `RackBottles { rack, .. }` (serde): a place per bottle, each empty or one bottle. `new(rack)`, `places()`, `items()`, `suits(kind, item)` (one alone, never a stack: a wine rack any wine bottle, full or empty; a spirit rack wine bottles, flasks, and `ItemKind::Bottle`s of a fermented drink or pure alcohol), `takes(item)`, `put(item) -> Result<(), crafting_bench::NotTaken>` (the lowest free place, never a swap, never why), `take(id)`, `replace(item)` (put back after pouring at a bar), `settle(now)` (bottles' glass clocks by world time, sealed drinks ageing at `cask_rack::RACK_AGEING_PERCENT`, owner's 10% faster).

## Depends on

- `vessel` (`VesselKind`), `potion` (`Liquid`), `cask_rack` (`RACK_AGEING_PERCENT`), `ferment` (`Item::settle_lying_casks_ageing`, which runs glass clocks too), `crafting_bench` (`NotTaken`), `item`, `item_form`, `id`, `serde`.

## Used by

- `furniture` (`Furniture::BottleRack`, `Holder::Bottles`, `Held::Bottles`), `dungeon` (spirit racks on top), `protocol` (through `Held`), the server (`sites/benches/held.rs`, `LevelSave::bottle_racks`, `connections/bench.rs`), client `dungeon_map`, `crafting_bench`, `debug_menu` (Place Furniture's Wine Rack and Spirit Rack).

## Notes

- Bottles are a counted kind: putting a stack on puts one of it (the server lifts one off).
- Spirits: there are none as drinks yet; a spirit rack takes pure alcohol's bottles and drinks' (Claude's reading of the owner's "bottles of drink").
- Counts, rows and sizes are Claude's first guesses for the owner to tune; a cell is taken as about a metre, walls a cell high.
- Not placed in sites yet (tavern furniture plan PR 8); only by Place Furniture, which keeps a piece with places (server `made_benches`), a spirit rack only while a table stands under it.
- A bar (`shared::bar`) in the same room pours from what lies on it.
