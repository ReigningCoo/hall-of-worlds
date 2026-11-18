# shared: vegetable

Vegetables, pulses and grains (owner, 2026-11-17): what each is, the table of them (built in), vegetables as counted items (a whole big one or a piece cut into servings when eaten), and what eating one raw does. Cooking and milling come later; their figures are kept. Each has a look for its pictures (PR 2). Where they grow, how they grow and what picking gives (PR 3): wild (`wilderness::vegetables`) and in farms' fields. Each has a plant look for the client's models (PR 4); the wiki pages PR 5. Modelled on `shared::fruit`, with its own kind and table (see `decisions.md`, 2026-11-18). Research and the owner's decisions: `docs/context/research-vegetables.md`.

## Files

- `crates/shared/src/vegetable.rs`: the module list and re-exports. Tests in `vegetable/tests.rs`.
- `vegetable/vegetables.ron`: the 85 rows (51 vegetables, 17 pulses, 17 grains; 17 wild only), generated once from the research's `final.json` (the script stayed outside the repo); edit by hand from now on. Read by `include_str!`.
- `vegetable/def.rs`: `VegetableDef`, `VegetableSet`, `Unit`, `check`.
- `vegetable/look.rs`: `VegetableLook`, `MOST_COLOURS`.
- `vegetable/traits.rs`: `Raw` (`heals`), `Abundance`, `PlantShape`, `Part`.
- `vegetable/effect.rs`: `Effect` (`effect`), `When` (`raw`, `cooked`).
- `vegetable/table.rs`: `Vegetables`.
- `vegetable/tag.rs`: `VegetableTag`.
- `vegetable/form.rs`: `VegetableForm`, `new_item`, `Item::vegetable`.
- `vegetable/eating.rs`: `eating`, `raw_ticks`, `effect_ticks`, `cooked_ticks`, `healing`, `RAW_SHARE`.
- `vegetable/cut.rs`: `cut`.
- `vegetable/dev.rs`: `dev_vegetable`.
- `vegetable/plant_look.rs`: `VegetablePlantLook`.
- `vegetable/plant.rs`: `VegetablePlant` (`stage`, `can_pick`). Tests for plants, harvests, places and fields in `vegetable/plant_tests.rs`.
- `vegetable/harvest.rs`: `CROPS`, `harvest_range`, `harvest`.
- `vegetable/place.rs`: `VegetableDef::grows_wild`, `Abundance::wild_weight`, `wild_crops`, `wild_only_vegetables`, `field_crops`, `choose_wild`.
- `vegetable/fields.rs`: `plant_fields`.

## Public API

- `VegetableDef { key, name, latin, set, wild_only, biomes, unit, kcal, servings, grams, raw, exceptional, effects, abundance, growth, regrow, harvest, plant, plant_look, part, look }`: `key` its id for good; `set` `Vegetable`, `Pulse` or `Grain`; `unit` `One`, `Handful` (100 g fresh, 30 g for dry pulses), `Sheaf` (30 g of grain, grain only), `Whole` or `Piece` (cut into `servings` of about 100 kcal), `Cluster`; `kcal` and `grams` for one item; `raw` `Raw::{Commonly, Sometimes, Rarely, Never}`; `exceptional` for cooking only; `effects` 0 to 2 `Effect { stat, change (±1 to 3), when: When::{Raw, Cooked, Both} }`; `abundance` wild (`Common` to `VeryRare`); `growth` and `regrow` herbs' (`herb::Growth`, `herb::Regrow`); `harvest` fruit's `HarvestTier`; `plant` a `PlantShape` (18), `part` a `Part` (11) (the picture's family); `plant_look` a `VegetablePlantLook`; `look` a `VegetableLook`.
- `VegetablePlantLook { height, spread, leaf, leaf_size, flower, flower_size }` (PR 4): the crop as it stands in its first year, in cm (a climber's pole, not a flower stalk), leaf and flower (a grass's heads) colours and `herb::PartSize`s, taken once from the research's findings (`height_m`, `spread_m`, `leaves`, `flowers`); `check`: a height and spread.
- `VegetableLook { colours, accent, inside, length, width }`: from the research's shape, colours, skin and inside: `colours` the usual first, then up to six other or old varieties (purple carrots), which some items are by their look seed; `accent` a second colour where the picture puts one (a turnip's shoulder, red stalks, a tomato's calyx, a bean's mottling or eye on the usual colour, a cob's husk); `inside` the flesh for slices; `length` and `width` of one piece in mm. `check`: 1 to `MOST_COLOURS` (7) colours, a size.
- `Vegetables`: `built_in()` (`&'static`), `read(ron_text)` (each checked), `new(defs)`, `get(key)`, `by_tag(tag)`, `iter`, `len`.
- `VegetableTag(u64)`, `VegetableTag::of(key)`: a key hashed, so `bonus::Source` can name a vegetable and stay `Copy`.
- `VegetableForm { vegetable, serving, look, grams }` (`ItemForm::Vegetable`): `new(def, seed)`, `serving(def, seed)`, `is_whole(def)` (a whole one or piece of several servings), `name()` ("Pumpkin", "Pumpkin Slice", "Vegetable" for a key the table lacks), `shape()` (as fruit's: 1 × 1; 2 × 1 from 1 kg; 2 × 2 from 4 kg). `new_item(def, count, id, made_at)`; `Item::vegetable()`.
- `eating(def, now) -> Vec<Effect>`: one serving eaten raw: healing (`Source::EatenVegetable(tag)`, `fruit::RESTORE` 1 Health a second, ended by moving or being hurt) for `raw_ticks(def)`, and the raw and both effects (`Source::VegetableEffects(tag)`, by fruit's `ATTRIBUTE` and `VITAL` a step, ended by nothing) for `effect_ticks(def)`. `raw_ticks`: `RAW_SHARE` (¼) of a serving's kcal ÷ 4, in whole seconds rounded to the nearest, only for `Commonly` or `Sometimes` and never grain. `effect_ticks`: a serving's kcal ÷ 4 in whole seconds, rounded down. `cooked_ticks`: that, × `fruit::EXCEPTIONAL` if exceptional, kept for cooking. `healing(def)`: raw Health in all.
- `cut(whole, new_id) -> Option<(eaten, Option<rest>)>`: as `fruit::cut`.
- `VegetablePlant { vegetable, phase, picked }` (a cell's plant; kept on `dungeon::Level`, sent in `protocol::LevelChunk::vegetables`): `stage(def, now)` (game minutes: 1 to `herb::STAGES`, 0 while dug up; `herb::grown_stage`, as herbs grow: up the stages at its `growth`, full grown as long again, round again; picked, back to its `regrow`'s stage, dug up gone for a whole growth), `can_pick(def, now)` (full grown, and grown a stage since last picked).
- `harvest(def, (foraging, crops), rng)`: how many items picking gives, in `harvest_range(def, foraging, crops)`: its `HarvestTier` (1 to 6), the cube root of Foraging's percent added to the most and of Crops' (`CROPS`, `gathering/farming/crops`) to both.
- `VegetableDef::grows_wild(biome)` (by biome alone: the table has no temperatures); `wild_crops(table, biome)` (farmed ones), `wild_only_vegetables(table, biome)`, `field_crops(table, biome)` (cultivated ones growing there, else every cultivated one); `choose_wild(list, rng)` by `Abundance::wild_weight` (9/3/1/1).
- `plant_fields(table, fields, biome, seed)`: every cell of each field of crops (not pasture, not an orchard) one crop from `field_crops`, rolled from the site's seed and the field's corner, all its plants sharing a phase (sown together).
- `dev_vegetable(key, seed)`: Give Item's vegetable, by key or rolled from the seed, the same on both sides; also `item_form::give_form`'s.

## Depends on

- `herb` (`Stat`, `Effect`, `Growth`, `Regrow`, `grown_stage`, `skill_percent`, `cube_root`), `site` (`FieldLook`), `fruit` (`HarvestTier`, `RESTORE`, `EXCEPTIONAL`, `bonus_of`), `climate` (`Biome`), `bonus` (`Effect`, `Source`), `item` (`Item`, `ItemKind::Vegetable`, `Shape`), `item_form` (`ItemForm::Vegetable`), `rng`, `id`, `ron`, `serde`.

## Used by

- `dungeon` (`Level`'s vegetable plants), `wilderness` (`vegetables.rs`), `protocol` (`SeenVegetable`, `PickNews::PickedVegetable`), server `sites/plants.rs` (fields), `connections/pick_vegetable.rs`, client `dungeon_map` (plant models by stage, picking, hover), `plants` (`plant`, `plant_look`, `part`, `look`, `set`, for models), `notices`.
- `item` (`ItemKind::Vegetable`, Food, counted, no quality; stacks compare a vegetable's form without its look), `item_form` (`give_form` rolls one; name, shape, grams), `bonus` (`Source::EatenVegetable`, `VegetableEffects`; `is_food` and `may_eat` take vegetables), server `connections/eat_vegetable.rs` (`eating`, `cut`), `eat_fruit.rs` (`servings_fit`, both cuts), `strength_room.rs`, `give_items.rs` (`dev_vegetable`), client `item_art` (`look`, `part`, `servings` for pictures), `effect_bar` (names by `by_tag`, `delapouite/carrot` for effects), `item_popup` (raw healing, effects, "Heals nothing raw", "Heals nothing until milled", servings), `debug_menu` (Give Item's dropdown, "Any vegetable", and preview), `dev_tools` (`client.give_item`'s `vegetable`).

## Notes

- Built in only, no mods yet: both sides read the same table, so nothing is sent. Saves and messages name a vegetable by key; effects by `VegetableTag`.
- No quality, never magical, blessed or cursed. A stack keys on vegetable and serving; each keeps its own look seed.
- Grain is eaten like anything else but does nothing (no raw healing, and grains' effects are for cooking). What is rarely or never eaten raw heals nothing, though its raw harm applies (owner: harm raw is a real effect), lasting as long as cooked healing would.
- Pictures in `item_art` (`vegetable.rs`); plant models in `plants` (`vegetable.rs`).
- Growing (PR 3, owner): as herbs (`growth` and `regrow` from the research), picked only full grown; no seasons yet. Harvests are kept by cell with herbs' and fruit's (the server's `plants_picked`).
- Where they grow (PR 3, owner): each farm field of crops grows one crop chosen by the farm's biome from the cultivated ones (no wild-only); pasture stays grass; orchards keep their fruit. Wild: farmed crops 1.5 times as often as wild fruit (those only farmed very rarely), wild-only plants as often as herbs (`wilderness.md`). Placing isn't part of any generator's version: harvests are kept by cell. Water plants grow on open ground too, as there are no marshes yet.
