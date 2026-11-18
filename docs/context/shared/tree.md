# shared: tree

Trees that can be felled (woodcutting plan, `docs/context/woodcutting.md`): the 27 wild kinds and which one a `Cell::Tree` is, every tree's wood and logs (fruit trees' by their fruit), and a felled tree's stages as it grows back with their walking costs. Not felling itself, logs as items or the abilities: `shared::woodcutting`. Nothing saves or draws these yet (woodcutting PRs 2 and 3). Research: `docs/context/research-trees.md`.

## Files

- `crates/shared/src/tree.rs`: the module list and re-exports. Tests in `tree/tests.rs`.
- `tree/kind.rs`: `TreeKind`, `Group`, `Commonness`.
- `tree/table.rs`: `TreeDef`, `TREES` (one row per kind), `TreeKind::{all, def, name}`.
- `tree/roll.rs`: `Setting`, `TreeKind::roll`.
- `tree/stage.rs`: `TreeStage` and its times and costs.
- `tree/fruit_wood.rs`: `fruit_wood`, the fruit trees' woods and logs by key.
- `tree/standing.rs`: `Tree`.

## Public API

- `TreeKind` (27, saved by name): Scots pine to ebony. `TreeDef { kind, name, group, biomes, celsius, commonness, wet, wood, logs }` in `TREES`: where it grows (biomes, yearly °C range), `Group::{Conifer, Broadleaf}`, `Commonness::{Common, Uncommon, Rare}` (`weight` 4, 2, 1), `wet` (willow, alder), its wood (`material::Material`) and the logs a grown one gives (1 to 12, the research's formula from height and trunk).
- `Setting { biome, celsius, narrow, lush, near_water }` (per mille; `of(ClimateSample, Cover, near_water)`), `TreeKind::roll(setting, hash)`: a cell's wild tree from its place and a hash of the cell: conifer with chance `narrow`, else broadleaf (the other group if none suits); suiting kinds weighted twice their commonness within their range, once within 3 °C of it; `lush` over 500 keeps to rainforest kinds (tropical from 20 °C); willow and alder common near water, rare elsewhere; the kind nearest by temperature where nothing suits.
- `fruit_wood(key) -> Option<(Material, u8)>`: a trunked fruit plant's wood and grown logs (57 of them); banana, papaya and saguaro have none (no logs, still a stump).
- `Tree::{Wild(TreeKind), Fruit(String)}`: `wood()`, `logs(stage)`.
- `TreeStage::{Stump, Sapling, Young, HalfGrown, Grown}`: `after_felling(minutes)` (a stump for `STUMP_MINUTES`, a day; nothing until `REGROW_MINUTES`, 5 days; then a stage every `STAGE_MINUTES`, a week; `Grown` when the saved entry can go), `cost_per_mille()` (1000 a sapling, `STUMP_COST_PER_MILLE` 1500 a stump or young tree, `HALF_GROWN_COST_PER_MILLE` 3000, `None` grown: it blocks), `can_fell()` (young and up), `logs(grown)` (a quarter young, half half-grown, at least 1; none from a stump or sapling).

## Depends on

- `climate` (`Biome`, `ClimateSample`), `vegetation` (`Cover`), `material` (`Material`, `Material::wood`), `herb` (`MINUTES_PER_DAY`), `serde`; tests `fruit`.

## Used by

- Nothing outside its tests yet: the server's felling and saves (PR 2), the tree models (PR 3).

## Notes

- A wild tree's kind is never saved: it is rolled again from the cell, so changing the table or the roll changes standing trees (fine until felled trees are saved, PR 2).
- The roll's weights and the 3 °C border are Claude's, from the research's suggestion; fruit trees don't join the wild roll (plan).
- World time is game minutes, as plants' growth (`fruit`, `herb`).
