# shared: flour

Flour as items: a grain's 30 g sheaf milled into a sack of that grain's flour, one flour per grain (17). Not milling itself, which is trade with a town's miller, not built yet: plan in `docs/context/milling.md`.

## Files

- `crates/shared/src/flour.rs`: the module list and re-exports. Tests in `flour/tests.rs`.
- `flour/form.rs`: `FlourForm`, `new_item`, `Item::flour`.
- `flour/dev.rs`: `grains`, `dev_flour`.

## Public API

- `FlourForm { grain, grams }` (`ItemForm::Flour`): `of(def)` (the sack one sheaf of a grain mills into, `None` for anything but grain; grams the sheaf's × `VegetableDef::flour_yield` %, rounded), `name()` ("Sack of Rye Flour", "Sack of Flour" for a grain the table lacks), `shape()` (1 × 1).
- `new_item(grain, count, id, made_at) -> Option<Item>`: sacks of a grain's flour; `Item::flour()`.
- `grains()`: every grain in the vegetable table, in its order. `dev_flour(key, seed)`: Give Item's grain, by key or rolled from the seed, the same on both sides; also `item_form::give_form`'s.

## Depends on

- `vegetable` (`VegetableDef`, `VegetableSet::Grain`, `flour_yield`, `Vegetables`), `item` (`Item`, `ItemKind::Flour`, `Shape`), `item_form`, `rng`, `id`, `serde`.

## Used by

- `item` (`ItemKind::Flour`: Materials, counted, no quality; stacks compare the whole form), `item_form` (name, shape, grams; `give_form`), server `connections/give_items.rs` and client `debug_menu` (Give Item's "Any flour" and each flour, preview), `dev_tools` (`client.give_item`'s `flour`), `item_art` (`FlourLook`: the picture), `item_popup` (`about-flour`), `xtask wiki` (each grain page's Flour section and `docs/assets/flour/`).

## Notes

- No quality, never magical, blessed or cursed, never stale (owner, 2026-11-18). Not food: eating it does nothing.
- Saves and messages name the grain by key, like vegetables.
