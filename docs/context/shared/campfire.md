# shared: campfire

Campfires (cooking plan PR 7, `docs/context/cooking.md`): a real fire anyone lights on a level with Make Campfire, its fuel and burning time, where one can go, which one someone faces to feed, and the Ash it leaves. A `cooking::Heat::Fire` source for cooking later (PR 9). The server keeps fires (`sites/campfires`) and decides; levels carry only how many pieces each has left (`dungeon`'s `campfires.rs`), which the client draws (`dungeon_map`'s `campfires.rs`).

## Files

- `crates/shared/src/campfire.rs`: the module list, re-exports, `MAX_KINDLING`, `ASH_GRAMS`, `LIGHT_SECONDS` / `light_ticks`, `FEED_SECONDS` / `feed_ticks`, `LIGHT_CELLS`, `burn_seconds`. Tests in `campfire/tests.rs`.
- `campfire/fire.rs`: `Campfire`, `FeedRefused`.
- `campfire/place.rs`: `can_hold_fire`, `campfire_faced`, `place_faced`, `can_make_fire`.
- `campfire/kindling.rs`: `kindling_to_burn`.

## Public API

- `burn_seconds(wood)`: how long one piece of kindling burns, in real seconds: 300 for the lightest wood (density 2) to 600 for the densest (11), in proportion; not a tree's wood as density 6.
- `Campfire { since, burns, burnt }` (serde, saved by the server): pieces burning one after another from `since` (server seconds, real time stopped while paused), each `burns` long; `lit(now, wood)`, `ends()`, `is_lit(now)`, `pieces_left(now)` (the one burning included), `feed(now, wood) -> Result<(), FeedRefused::{Out, Full}>` (only while lit, under `MAX_KINDLING` 5 left), `settle(now)` (burnt pieces counted into `burnt`, `ends` kept), `ash()` (one Ash a piece fed).
- `can_hold_fire(level, x, y)`: bare ground, grass, a floor or a stone or dirt road (not boards, fields, gardens, covered ground, water), with no furniture or standing tree. The server also keeps fires off stairs and other fires.
- `campfire_faced(level, at, heading, has_fire)` / `place_faced(level, at, heading, free)`: the nearest faced cell as Collect Water's water (`facing::faced_cell`, `potion::COLLECT_REACH`, `FACING_HALF`), underfoot too. `can_make_fire(level, at, heading)`: either (the client's guess before asking).
- `kindling_to_burn(carried)`: the first kindling carried (own grid top to bottom, left to right, then bags) and its wood.
- `LIGHT_SECONDS` (5) and `FEED_SECONDS` (1): Make Campfire's wind-ups; `LIGHT_CELLS` (6, as a carried light): how far a lit fire's light reaches; `ASH_GRAMS` (20): one Ash item.

## Depends on

- `dungeon` (`Level`, `Cell`), `facing`, `potion` (reach), `spot`, `material` (woods' densities), `item` (`Inventory`, `ItemKind::Kindling`), `woodcutting` (`Item::kindling`), `energy`, `serde`.

## Used by

- `ability` (`Ability::MakeCampfire`, learned on first carrying kindling, needing kindling: `Learned::learn_from`, `Ability::tool`), `item` (`ItemKind::Ash`'s weight), `dungeon` (`Level::campfire_at`), the server (`sites/campfires`, `connections/campfire.rs`, `LevelSave::campfires`), the client (`abilities`' aim, `dungeon_map`'s `campfires.rs`).

## Notes

- Claude's choices, for the owner to tune: burning time by the wood's density; a piece's Ash 20 g; lighting 5 s and feeding 1 s; a fire is never moved or put out by hand, and goes when it burns out (no fire pit left), its Ash dropped on the ground at its cell; fires don't block movement, harm or spread.
- Fires burn by the server's clock in real time, not world time, so debug World Time doesn't burn them out. One on a level nobody is on burns out unseen and leaves its Ash when a player's level next includes it.
