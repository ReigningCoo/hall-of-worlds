# shared: chest

Chests and wardrobes (tavern furniture plan PR 7): inn rooms' storage furniture with a grid inside like a bag's (`shared::item`). A chest takes anything that fits; a wardrobe only clothes (worn, Clothing or Armour category). Both are furniture with places (`furniture::Holder::Store`, `Held::Store`): the server keeps what is in one by the piece's cell and saves it (`LevelSave::stores`), and the client opens it in the one panel every such piece uses (client `crafting_bench`). Beds are `furniture::Bed`, holding nothing.

## Files

- `crates/shared/src/chest.rs`: the module, re-exports. Tests in `chest/tests.rs`.
- `chest/kind.rs`: `Chest`, `Wardrobe`, `Storage`.
- `chest/stored.rs`: `Stored`.

## Public API

- `Chest::{Small, Large}` and `Wardrobe::{Narrow, Wide}` (each `ALL`, `length()` 1 or 2 cells along its front, `name()`: "Small Chest", "Chest", "Narrow Wardrobe", "Wardrobe"), as `furniture::Furniture::Chest { chest }` and `Wardrobe { wardrobe }`.
- `Storage::{Chest(Chest), Wardrobe(Wardrobe)}`: `grid()` (first guesses: 6 × 4, 8 × 5, 4 × 8, 8 × 8), `takes(kind)`, `name()`.
- `Stored { storage, items: Vec<item::Placed> }` (serde): `new`, `items`, `items_mut`, `put(item) -> Result<(), crafting_bench::NotTaken>` (the first place it fits, rows from the top, unturned first, then each quarter turn; one refusal whether the kind is wrong or there is no room), `take(id)`.

## Depends on

- `shared::item` (`Item`, `Placed`, `Rotation`, `Shape`, `ItemKind`, `ItemCategory`), `shared::crafting_bench` (`NotTaken`).

## Used by

- `shared::furniture` (`Furniture::Chest`, `Wardrobe`, `Holder::Store`, `Held::Store`), `server` (`sites/benches/held.rs`, `LevelSave::stores`), client `dungeon_map` (`decor/inn_room.rs` models), `debug_menu` (Place Furniture), `dev_tools` (`HOW_DEBUG_FURNITURE=inn`).

## Notes

- Owner's design (2026-11-22): a chest is a grid that takes anything, like a bag's inside; a wardrobe a grid for clothes only; both shown closed. The panel lists what is inside as the ground's section does (the server lays it out); the player doesn't arrange it.
- `Stored` is whole in itself (its kind and its items), so The Way Inn's private rooms can keep one with the character, the same on every world, instead of in a level's save.
- Things nest as in bags only below the chest: the chest itself is not an item, so `item::MAX_DEPTH` counts from what is put in it.
