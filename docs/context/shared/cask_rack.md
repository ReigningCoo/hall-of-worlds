# shared: cask_rack

Cask racks (tavern furniture plan PR 1, owner's design, 2026-11-22): furniture with places (`furniture::Held`) against a wall, 1 deep, each made for one cask size. What kind of piece each size gets, how many casks it holds and where each lies, what lies on one, and racked casks ageing faster. Kept by the server as benches' tools are (`server.md`); drawn by client `dungeon_map` (`furniture/cask_rack.rs`, `rack_casks.rs`), opened into `crafting_bench`'s panel.

## Files

- `crates/shared/src/cask_rack.rs`: the module, re-exports, `RACK_AGEING_PERCENT`. Tests in `cask_rack/tests.rs`.
- `cask_rack/rack.rs`: `CaskRack`, `RackKind`, `RackPlace`.
- `cask_rack/casks.rs`: `RackCasks`.

## Public API

- `CaskRack { size, length }` (serde, `Copy`, `Hash`: in `furniture::Furniture::CaskRack { rack }`): `lengths(size)` (1 to 3 cells, a Tun's 2), `all()` (every size at every length), `kind()`, `is_well_made()`, `name()` ("Pin Rack", "Hogshead Cradle", "Tun Stand"), `places()` (each cask's `RackPlace { along, layer }`: its middle in quarters of a cell along the piece from its start, its layer from 0; the bottom row first, as filled), `capacity()`.
- `RackKind::{Rack { layers }, Cradle, Stand}`, `RackKind::of(size)` (owner's): Pin and Firkin racks 3 layers, Kilderkin 2, casks in a pyramid (2 a cell on the bottom, each row above in the grooves, one fewer): 1 × 1 holds 3, 1 × 2 holds 9 or 7, 1 × 3 15 or 11; Barrel, Hogshead, Puncheon and Butt a cradle a cell; a Tun only on its stand, 2 long, lying along the wall with its tap to the room.
- `RackCasks { rack, .. }` (serde): a place per cask, each empty or a cask. `new(rack)`, `places()`, `items()`, `takes(item)` (a cask of its size while a place is free), `put(item) -> Result<(), crafting_bench::NotTaken>` (the lowest free place; never a swap, never why), `take(id)`, `settle(now)` (casks' clocks by world time, sealed ones ageing at `RACK_AGEING_PERCENT`; whether anything seen changed).
- `RACK_AGEING_PERCENT` (110, owner's 10% faster): only a sealed drink's ageing (`ferment`'s `CaskForm::pass_ageing`); fermenting and the opened clock to vinegar go as on the ground.

## Depends on

- `cask` (`CaskSize`, `CaskForm`), `ferment` (`Item::settle_lying_casks_ageing`), `crafting_bench` (`NotTaken`), `item`, `item_form`, `id`, `serde`.

## Used by

- `furniture` (`Furniture::CaskRack`, `Holder::Casks`, `Held::Casks`), `protocol` (through `Held`), the server (`sites/benches/held.rs`, `LevelSave::racks`, `connections/bench.rs`), client `dungeon_map`, `crafting_bench`, `debug_menu` (Place Furniture's Cask Rack).

## Notes

- Casks on a rack go by the ground's world-time clock: laid down when put on (`Item::lay_casks_down`), settled whenever told or taken, the last few seconds (under a 10 s step) counted as on the ground when taken off (`Item::age_casks`).
- Settled in whole 10 s steps at 110% so ageing loses nothing to rounding however often it is settled.
- Sizes, counts and how big casks are drawn are Claude's first guesses for the owner to tune; a cell is taken as about a metre.
- Not placed in sites yet (tavern furniture plan PR 8); only by Place Furniture, which keeps a piece with places (server `made_benches`).
