# shared: crafting_bench

Crafting benches (owner's design, 2026-11-20; crafting benches plan PR 1 of 4): furniture holding the tools of one craft, only Alchemy so far. Its crafts, the tools a bench takes and how they are put on, the bench someone is at, where generated ones stand and what they start with. The server keeps benches' tools (PR 2, `server.md`); not the client's model and panel (PR 3).

## Files

- `crates/shared/src/crafting_bench.rs`: the module list, re-exports, `BENCH_PERCENT`. Tests in `crafting_bench/tests.rs`.
- `crafting_bench/craft.rs`: `Craft` (`ALL`, `index`, `length`, `tools`, `takes`).
- `crafting_bench/tools.rs`: `BenchTools`, `NotTaken`.
- `crafting_bench/faced.rs`: `bench_faced`.
- `crafting_bench/place.rs`: `bench_percent`, `crafts_on_level`, `place_benches`.
- `crafting_bench/spot.rs`: where on a level a bench could stand (crate-private).
- `crafting_bench/starting.rs`: `starting_tools`.

## Public API

- `Craft::{Alchemy}` (serde; new ones at the end): `length()` along the wall (1 to 3; Alchemy 2; every bench 1 deep), `tools()` the kinds it takes in order (Alchemy: each `alchemy::Apparatus::item`), `takes(kind)`.
- `BenchTools { craft, .. }` (serde): one place per kind of tool, in the craft's order. `put(item) -> Result<Option<Item>, NotTaken>`: a kind already there swaps, handing back the old tool; any other kind, or more than one at once, gets the one `NotTaken`, which never says what is allowed. `take(id)`, `tool(kind)`, `has(kind)`, `tools()`.
- `bench_faced(level, at, heading) -> Option<Piece>`: the bench someone faces, any of its cells within `potion::COLLECT_REACH` and `FACING_HALF` (`facing::faced_cell`), as Collect Water's water.
- `BENCH_PERCENT` (10): work at a bench has its purity score × 110%, multiplied with each Pelican's (`alchemy::worked_quality`, `Process::at_bench`).
- `bench_percent(craft, kind)`: Alchemy in every herbalist's hut (100), 30% of stone towers, 20% of castles. `crafts_on_level(kind, seed, levels, index)`: the crafts meant for that level, one bench a craft on each of ⌈levels ÷ 5⌉ levels chosen by hash. `place_benches(site_level, kind, seed, levels, index, ruin)`: called by `site::Site::level` after `furniture::furnish`.
- `starting_tools(kind, seed, index, piece) -> Vec<(ItemKind, Quality)>`: what a generated bench starts with, for the server to make once: herbalists a Mortar and Pestle; stone towers that and a Retort; castles both, and 50% an Alembic, 25% each a Crucible, Filter and Pelican. Qualities as drops' (`Quality::roll`), never an artifact.

## Depends on

- `furniture` (`Piece`, `Furniture::CraftingBench`, `footprint`, `place::rooms`, `place::reach`), `alchemy` (`Apparatus`), `item`, `id`, `facing`, `potion` (reach and facing half), `dungeon`, `site` (`SiteLevel`), `poi`, `rng`, `door` (`Side`), `spot`.

## Used by

- `furniture` (`Furniture::CraftingBench { craft }`), `site` (`Site::level`), `alchemy` (`Inventory::check_alchemy_at`, `BENCH_PERCENT`). The server (`sites/benches`, `connections/bench.rs`, Alchemy at a bench; `protocol::BenchAsk`, `SeenBench`, `DevBench`); the client (PR 3) to come.

## Notes

- A bench is a `Piece` 1 deep and its craft's length, of wood, facing away from the wall behind it (as a seat), its shelf along the back; moves over it cost as a table's. Never stone; `is_well_made` checks its size.
- Placing: only on usable room floor (furniture's `usable_cells`: not by doors, stairs or arrivals), every cell unfurnished with a plain `Wall` behind and open, unfurnished floor in front, and never cutting anything off (furniture's `Reach`). The first such spot by hash of `seed.derive("crafting bench")`; a level with none has no bench (so "at most"). Ruins keep a bench at 100 − 15 × ruin %. Placed after furniture, so no generator version, golden test or other piece moved; cellar casks (placed after) keep off it.
- All chances are Claude's first guesses for the owner to tune.
