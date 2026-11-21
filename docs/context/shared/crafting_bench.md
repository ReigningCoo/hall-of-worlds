# shared: crafting_bench

Crafting benches (owner's design, 2026-11-20; crafting benches plan PR 1 of 4): furniture holding the tools of one craft: Alchemy, and hearths for cooking (cooking plan PR 8: Stove, Hearth, Range). Its crafts, the tools a bench takes and how they are put on, the bench someone is at, where generated ones stand and what they start with. The server keeps benches' tools (PR 2, `server.md`); the client's model and panel are `dungeon_map`'s and `crafting_bench`'s (PR 3).

## Files

- `crates/shared/src/crafting_bench.rs`: the module list, re-exports, `BENCH_PERCENT`. Tests in `crafting_bench/tests.rs`.
- `crafting_bench/craft.rs`: `Craft` (`ALL`, `index`, `name`, `length`, `is_hearth`, `has_storage`, `tools`, `takes`), `KITCHEN_TOOLS`.
- `crafting_bench/tools.rs`: `BenchTools`, `NotTaken`, `STORAGE_PLACES`.
- `crafting_bench/hearth.rs`: hearths: `hearth_percent`, `hearth_weights`, `always_lit`, `town_hearth`, `town_tools`, `hearth_faced`, `hearth_ash`, `ASH_HEAT_SECONDS`. Tests in `crafting_bench/hearth_tests.rs`.
- `crafting_bench/faced.rs`: `bench_faced`.
- `crafting_bench/place.rs`: `bench_percent`, `crafts_on_level`, `place_benches`.
- `crafting_bench/spot.rs`: where on a level a bench could stand (crate-private).
- `crafting_bench/starting.rs`: `starting_tools`.

## Public API

- `Craft::{Alchemy, Stove, Hearth, Range}` (serde; new ones at the end): `name()` ("Alchemy Bench", "Stove", ...), `length()` along the wall (1 to 3; Alchemy 2; Stove 1, Hearth 2, Range 3; every bench 1 deep), `is_hearth()` (the three hearths: stone, a hearth's heat while lit), `has_storage()` (the Range), `tools()` the kinds it takes in order (Alchemy: each `alchemy::Apparatus::item`; Hearth and Range: `KITCHEN_TOOLS`, Pan, Cauldron, Bowl, Sieve, Knife, Mortar and Pestle; a Stove none), `takes(kind)`.
- `BenchTools { craft, always_lit, fire, .. }` (serde, new fields defaulted): one place per kind of tool, in the craft's order; a Range's storage (`STORAGE_PLACES`, 8, things as put); a hearth's `always_lit` and its `fire` (`campfire::Campfire`, burning kindling by the server's clock). `put(item) -> Result<Option<Item>, NotTaken>`: a kind already there swaps, handing back the old tool; food (Dish, Intermediate) into a Range's storage while there is room; anything else, or more than one tool at once, gets the one `NotTaken`, which never says what is allowed. `take(id)` (tools or storage), `tool(kind)`, `has(kind)`, `tools()`, `storage()`, `storage_mut()`, `items()` (tools then storage), `is_lit(now)` (a hearth always lit or with its fire burning).
- Hearths: `hearth_percent(kind)` (towers, castles and forts 100, cottages and herbalists 90, huts 70) and `hearth_weights(kind)` (which of Stove, Hearth, Range: huts mostly Stoves, cottages any, towers Hearths and Ranges, castles and forts Ranges); `always_lit(kind)` (towers, castles, forts: households); `town_hearth(kind)` (hamlets to cities give a virtual Hearth, always lit, every kitchen tool, `town_tools()`, none to take, no storage; for cooking's server, PR 9); `hearth_faced` (`bench_faced` for a hearth); `hearth_ash(heat_seconds)` (one Ash a `ASH_HEAT_SECONDS`, 450, begun: Ash comes only from cooking, PR 9 calls it).
- `bench_faced(level, at, heading) -> Option<Piece>`: the bench someone faces, any of its cells within `potion::COLLECT_REACH` and `FACING_HALF` (`facing::faced_cell`), as Collect Water's water.
- `BENCH_PERCENT` (10): work at a bench has its purity score × 110%, multiplied with each Pelican's (`alchemy::worked_quality`, `Process::at_bench`).
- `bench_percent(craft, kind)`: Alchemy in every herbalist's hut (100), 30% of stone towers, 20% of castles; a hearth craft its share of `hearth_percent`. `crafts_on_level(kind, seed, levels, index)`: the crafts meant for that level, one bench a craft on each of ⌈levels ÷ 5⌉ levels chosen by hash; a site has one craft of hearth at most (one roll for having hearths, one pick by `hearth_weights`). `place_benches(site_level, kind, seed, levels, index, ruin)`: called by `site::Site::level` after `furniture::furnish`.
- `starting_tools(kind, seed, index, piece) -> Vec<(ItemKind, Quality)>`: what a generated bench starts with, for the server to make once: herbalists a Mortar and Pestle; stone towers that and a Retort; castles both, and 50% an Alembic, 25% each a Crucible, Filter and Pelican. A Hearth or Range a Pan and Cauldron, and perhaps a Bowl, Knife, Sieve and Mortar (more in castles and forts); a Stove nothing. Qualities as drops' (`Quality::roll`), never an artifact.

## Depends on

- `furniture` (`Piece`, `Furniture::CraftingBench`, `footprint`, `place::rooms`, `place::reach`), `alchemy` (`Apparatus`), `item`, `id`, `facing`, `potion` (reach and facing half), `dungeon`, `site` (`SiteLevel`), `poi`, `rng`, `door` (`Side`), `spot`.

## Used by

- `furniture` (`Furniture::CraftingBench { craft }`), `site` (`Site::level`), `alchemy` (`Inventory::check_alchemy_at`, `BENCH_PERCENT`). The server (`sites/benches`, `connections/bench.rs`, Alchemy at a bench; `protocol::BenchAsk`, `SeenBench`, `DevBench`); the client (PR 3: `Ability::can_use_at`; `dungeon_map`'s model, tools and picking, `crafting_bench`'s panel, `alchemy` at a bench).

## Notes

- A bench is a `Piece` 1 deep and its craft's length, of wood (a hearth of stone), facing away from the wall behind it (as a seat), its shelf along the back; moves over it cost as a table's. `is_well_made` checks its size and material.
- Hearths (cooking plan PR 8, Claude's choices for the owner to tune): the Oven is a hearth's own, not an item; a Stove holds no tools (carried ones still work); a hearth not always lit burns kindling as a campfire does (Make Campfire facing it lights or feeds it) and never leaves Ash by burning; food in a Range's storage rests in real time at the land's warmth, as on the ground; hearths go anywhere a bench could, not only on ground floors. Adding the hearth crafts after Alchemy keeps every Alchemy bench where it was.
- Placing: only on usable room floor (furniture's `usable_cells`: not by doors, stairs or arrivals), every cell unfurnished with a plain `Wall` behind and open, unfurnished floor in front, and never cutting anything off (furniture's `Reach`). The first such spot by hash of `seed.derive("crafting bench")`; a level with none has no bench (so "at most"). Ruins keep a bench at 100 − 15 × ruin %. Placed after furniture, so no generator version, golden test or other piece moved; cellar casks (placed after) keep off it.
- All chances are Claude's first guesses for the owner to tune.
