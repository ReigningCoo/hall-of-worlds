# shared: fireplace

Fireplaces (tavern furniture plan PR 4, owner's design, 2026-11-22): stone furniture with places (`furniture::Held::Fire`) against a wall, a chimney breast rising behind, 1 to 3 cells long; the 1 × 3 is an inglenook, the fire in its middle cell with a settle either side of it inside, at right angles to it (owner's). Make Campfire lights or feeds one as it does a hearth, and while lit it gives light as a campfire does (`campfire::LIGHT_CELLS`). A log basket beside the fire holds kindling in places; fire tools (poker, tongs, shovel, bellows) are part of the model. It cooks as a worse hearth (owner's). Kept by the server as racks are (`LevelSave::fireplaces`, `sites/campfires/light.rs`, `connections/cook_kitchen.rs`); drawn by client `dungeon_map` (`furniture/fireplace.rs`, `basket_logs.rs`, flames by `hearth_fires.rs`); the basket's panel is client `crafting_bench`'s.

## Files

- `crates/shared/src/fireplace.rs`: the module, re-exports. Tests in `fireplace/tests.rs`.
- `fireplace/kind.rs`: `Fireplace`, `KINDLING_PER_PLACE`.
- `fireplace/fireside.rs`: `Fireside`.
- `fireplace/faced.rs`: `fireplace_faced`.

## Public API

- `Fireplace { length }` (serde, `Copy`, `Hash`: in `furniture::Furniture::Fireplace { fireplace }`): `LENGTHS` (1 to 3), `all()`, `is_well_made()`, `name()` ("Fireplace", "Wide Fireplace", "Inglenook"), `is_inglenook()` (3 long), `fire_cell()` (the fire's cell along the wall: the first, an inglenook's middle), `basket_places()` (2, 3, 4).
- `KINDLING_PER_PLACE` (10): the most pieces of kindling one basket place holds, as one stack (`furniture::Holder::per_place`: the server puts up to that many of a carried stack at once).
- `Fireside { fireplace, always_lit, fire, .. }` (serde, new fields defaulted): what lies at a fireplace: `new`, `places()` (each place empty or a stack of kindling), `items()`, `put(item)` (kindling, at most `KINDLING_PER_PLACE`, into the first free place; anything else the one `crafting_bench::NotTaken`), `take(id)`, `is_lit(now)` (always lit, or its fire burning), `burn(now, wood)` (Make Campfire: lights its fire with a piece, or feeds it; `None` if always lit or full). The fire is a `campfire::Campfire`, burning by the server's clock; it leaves no Ash by burning.
- `fireplace_faced(level, at, heading)`: the fireplace someone is at, as `crafting_bench::bench_faced` (any of its cells within `potion::COLLECT_REACH` and `FACING_HALF`): for Make Campfire (`campfire::can_make_fire`), cooking at it and its basket.

## Depends on

- `furniture` (`Furniture`, `Piece`), `campfire` (`Campfire`), `crafting_bench` (`NotTaken`), `dungeon`, `facing`, `potion` (reach), `item`, `id`, `material`, `spot`, `serde`.

## Used by

- `furniture` (`Furniture::Fireplace`, `Holder::Fire`, `Held::Fire`, `Held::is_lit`), `campfire` (`can_make_fire`), the server (`sites/benches/held.rs`'s `fireside`, `LevelSave::fireplaces`, `sites/campfires/light.rs`'s `FireTarget::Fireplace`, `connections/cook_kitchen.rs`), the client (`cooking`'s kitchen, `dungeon_map`'s model, basket logs and fire, `debug_menu`'s Place Furniture).

## Notes

- Cooking (owner's): a lit fireplace counts as a hearth for heat (`cooking::Heat::Hearth`, as a town's: every heat step, no Rare cap), but has no Oven and no tools of its own (carried ones work), and there is no bench bonus for cooking (`crafting_bench::BENCH_PERCENT` is Alchemy's only). Cooking at one leaves Ash as at a hearth (`crafting_bench::hearth_ash`). The order names it by a cell (`CookOrder::bench`), which the cook must face.
- A fireplace is never drink storage for a bar (`Holder::is_drink_storage`).
- Moves over one cost as a table's. Castles' and towers' great halls may have one (`shared::furnishings`, PR 8), always lit in a lived-in place (`furnishings::is_lit`, lit once by the server); also the Tavern Dev Area's inglenook and Place Furniture.
- Counts, sizes and the model are Claude's first guesses for the owner to tune.
