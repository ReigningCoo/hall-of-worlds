# shared: bar

Bar counters (tavern furniture plan PR 3, owner's design, 2026-11-22): furniture with places (`furniture::Held`) for drinking vessels, 1 to 3 cells long, 1 deep, of wood, joining into longer runs (drawn as one), with a gap or hinged flap to get behind. A single counter keeps its vessels on a shelf underneath; a double one on a raised shelf above, with room underneath for the barkeep. Used like a crafting bench: at a bar, a carried vessel is filled from any drink stored in the bar's room, by the pour rule (`shared::pour`), never from anything lying on the ground. Kept by the server as racks are (`LevelSave::bars`, `connections/bar_pour.rs`); drawn by client `dungeon_map` (`furniture/bar_counter.rs`, vessels by `rack_bottles.rs`); the panel and its pour list are client `crafting_bench`'s (`pour.rs`).

## Files

- `crates/shared/src/bar.rs`: the module, re-exports. Tests in `bar/tests.rs`.
- `bar/counter.rs`: `BarCounter`, `BarKind`, `BarPlace`, `VESSELS_PER_CELL`.
- `bar/vessels.rs`: `BarVessels`.
- `bar/storage.rs`: `room_storage`, `drinks_in`, `vessels_to_fill`.

## Public API

- `BarKind::{Single, Double, Flap}` (`ALL`, `name()`: "Bar Counter", "Double Bar Counter", "Bar Flap").
- `BarCounter { kind, length }` (serde, `Copy`, `Hash`: in `furniture::Furniture::BarCounter { counter }`): `LENGTHS` (1 to 3; a flap 1), `all()` (7), `is_well_made()`, `name()`, `is_counter()` (not the flap), `places()` (each vessel's `BarPlace { along }`, its middle in twenty-fourths of a cell along the counter; none on a flap), `capacity()` (`VESSELS_PER_CELL`, 4, a cell).
- `BarVessels { counter, .. }` (serde): a place per vessel, each empty or one. `new`, `places()`, `items()`, `suits(item)` (a drinking vessel: tankard, cup, goblet, drinking horn or jug, full or empty), `takes`, `put` (the first free place, never a swap, never why: `crafting_bench::NotTaken`), `take(id)`, `replace(item)` (put back in its place after pouring; racks have the same), `settle(now)` (clocks by world time, as on the ground: nothing ages faster on a bar).
- `room_storage(level, bar)`: every piece of drink storage (`furniture::Holder::is_drink_storage`: cask racks, cradles, Tun stands, wine and spirit racks, bar counters; never a crafting bench) sharing a room with the bar, the bar among them, floor pieces then those on top. Cells in no room count as one room (Claude's choice: an open-air bar, as in a Dev Area, pours from storage also outside rooms).
- `drinks_in(held)`: what on a piece can be poured from (a cask, vessel or bottle holding something).
- `vessels_to_fill(inventory)`: carried vessels (own grid and bags, never worn nor quest items), what a bar fills.

## Depends on

- `furniture` (`Held`, `Holder`, `Piece`), `dungeon` (`Level::room`, furniture), `pour` (`Held::of`), `vessel` (`VesselKind`), `crafting_bench` (`NotTaken`), `ferment` (`Item::settle_lying_casks_ageing`), `item`, `id`, `serde`.

## Used by

- `furniture` (`Furniture::BarCounter`, `Holder::Bar`, `Held::Bar`; a counter holds a spirit rack, `holds_furniture`), `protocol` (`BenchAsk::Pour`, through `Held`), the server (`sites/benches/held.rs`, `Visit::bar_storage`, `connections/bar_pour.rs`, `LevelSave::bars`), client `crafting_bench` (`pour.rs`), `dungeon_map` (`furniture/bar_counter.rs`, `rack_bottles.rs`), `debug_menu` (Place Furniture's Bar Counter).

## Notes

- Moves over a counter cost as a table's; over a flap as open floor (lifted to walk through). A Tun counts only installed on its stand, as it is only drink storage there.
- A spirit rack may stand on a counter (not a flap), drawn as much higher as a counter's top is over a table's.
- Pouring at the bar is in reach of the counter (as putting things on it), with a tap carried to draw from a cask, as any pour.
- Counts, heights and the panel's look are Claude's first guesses for the owner to tune. Not placed in sites yet (tavern furniture plan PR 8); only by Place Furniture.
