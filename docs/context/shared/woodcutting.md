# shared: woodcutting

Woodcutting's rules (plan: `docs/context/woodcutting.md`): the one-handed axe held, the tree faced, how long felling takes and the damage a cut-short felling leaves, logs and kindling as items, and which log Cut Kindling splits. The trees themselves (kinds, logs per tree, regrowing) are `shared::tree`; the abilities are `shared::ability`'s. The server's side is woodcutting PR 2.

## Files

- `crates/shared/src/woodcutting.rs`: the module list, re-exports, `WOODCUTTING_SKILL`, `FELL_PRACTICE`, `KINDLING_PRACTICE`. Tests in `woodcutting/tests.rs`.
- `woodcutting/axe.rs`: `HeldAxe`, `Inventory::held_axe`.
- `woodcutting/felling.rs`: `fell_ticks`, `fell_seconds`.
- `woodcutting/damage.rs`: `Damage`, `DAMAGE_KEPT_MINUTES`.
- `woodcutting/faced.rs`: `holds_tree`, `tree_faced`.
- `woodcutting/log.rs`: `LogForm`, `LOG_CM3`, `new_log`, `Item::log`.
- `woodcutting/kindling.rs`: `KindlingForm`, `KINDLING_GRAMS`, `kindling_from`, `new_kindling`, `Item::kindling`.
- `woodcutting/cut.rs`: `LogAt`, `log_to_cut`, `cut_seconds`, `cut_ticks`.
- `woodcutting/dev.rs`: `dev_wood`.

## Public API

- `HeldAxe { quality, hatchet }` (`of(item)`: an axe-family blade with one hand; a hatchet by its name, "Hatchet", from its row or its generic cell), `Inventory::held_axe()` (worn with `Location::Held`; of two, a hatchet, then the better): what Fell Tree and Cut Kindling need (`Ability::has_tool`) and what teaches them (`Learned::learn_from`).
- `fell_ticks(skills, axe)` / `fell_seconds`: 60 s untrained with a Common axe, less up to 15 s by ∛Woodcutting (as `herb::skill_value`) and up to 15 s by ∛the axe's steps above Common (Poor as Common), so 30 s at best; any axe but a hatchet × 1.5.
- `Damage { per_mille, at }` (saved later by the server): `chopped(now, ticks, whole)`, `left(now)` (whole within `DAMAGE_KEPT_MINUTES`, a day of game minutes, of the last blow, then none), `is_felled`, `ticks_left(now, whole)`. Per mille of the tree, so fellers with different times share it.
- `holds_tree(level, x, y)` (`Cell::Tree` or a fruit plant with a trunk, `Level::has_trunk`), `tree_faced(level, at, heading)`: as Collect Water's water (`facing::faced_cell`, `potion::COLLECT_REACH`, `FACING_HALF`).
- `LogForm { wood }` (`ItemForm::Log`): `name` ("Oak Log"), `grams` (`LOG_CM3`, 30 × 30 × 100 cm, × `Material::density`: oak 63 kg, pine 45 kg), `shape` (8 × 3). `new_log(wood, id, made_at)`, `Item::log()`.
- `KindlingForm { wood }` (`ItemForm::Kindling`): `name` ("Oak Kindling"), `shape` (2 × 1); `KINDLING_GRAMS` (1 kg a piece); `kindling_from(log)` (a piece a whole kilogram, at least 1); `new_kindling(wood, count, id, made_at)`, `Item::kindling()`.
- `LogAt::{Carried(Id), Ground(Id)}`, `log_to_cut(carried, ground)`: the first log carried (own grid top to bottom, left to right, then bags), else the first in `ground` (given only while the inventory is open with the ground's storage shown); `cut_seconds(skills)` / `cut_ticks` (20 − ∛Woodcutting%, 20 to 16).
- `dev_wood(asked, seed)`: Give Item's wood: the one asked if it is a tree's (`Material::wood`), refused otherwise (`Field::Material`), else rolled from the seed.
- `WOODCUTTING_SKILL` (`gathering/woodcutting`), `FELL_PRACTICE` (48 a tree), `KINDLING_PRACTICE` (8 a log).

## Depends on

- `blade` (`Family`, `Hands`, `BladeForm::name`), `creature` (`Location`), `item` (`Inventory`, `Item`, `ItemKind`, `Quality`, `Shape`), `item_form` (`ItemForm`, `Field`, `Refused`), `material` (`Material`, `WOODS`), `herb` (`cube_root`, `skill_value`, `skill_percent`, `MINUTES_PER_DAY`), `skill`, `energy`, `dungeon`, `facing`, `potion`, `spot`, `rng`, `id`, `serde`.

## Used by

- `ability` (`Inventory::held_axe` for the tool and learning), `item` (`ItemKind::Log`, `Kindling`), `item_form` (names, cells, weights; `give_form_to` through `dev_wood`; `Choices::materials`).

## Notes

- Never magical, blessed or cursed, no quality: logs and kindling are materials.
- Cut Kindling needing the axe, its time and the practice figures are Claude's defaults (plan).
