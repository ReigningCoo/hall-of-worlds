# shared: fruit

Fruits and nuts (owner, 2026-11-17): what each is, the table of them (built in), fruits as counted items (a whole big fruit cut into servings when eaten), what eating one does, and fruit plants: where they grow (wild, very rarely; farms' orchards), their stages and what picking gives (PR 3 of the fruit plan). Plant models are the client's (`plants::FruitModels`, from each fruit's `FruitPlantLook`), as are the items' pictures (`item_art`, `fruit.rs`). Herbs that bear fruit stay herbs (`shared::herb`), linked by `HerbDef::fruit`. Research and the owner's decisions: `docs/context/research-fruit.md`.

## Files

- `crates/shared/src/fruit.rs`: the module list and re-exports. Tests in `fruit/tests.rs`; `fruit/climate_tests.rs` checks every biome grows some fruit item at its hottest and coldest (with `herb::climate_tests::extremes`).
- `fruit/fruits.ron`: the 101 rows (73 fruits, 22 nuts, 6 herbs' fruits with `herb: true`), generated once from the research's `final.json` (the script stayed outside the repo); edit by hand from now on. Read by `include_str!`.
- `fruit/def.rs`: `FruitDef`, `FruitSet`, `Unit`, `check`, `is_item`.
- `fruit/traits.rs`: `Abundance`, `HarvestTier` (`range`), `PlantFamily`.
- `fruit/look.rs`: `FruitLook`, `FruitShape`, `Skin` (data only, for the client).
- `fruit/plant_look.rs`: `FruitPlantLook`, `FruitPosition` (data only, for the client's plant models).
- `fruit/table.rs`: `Fruits`.
- `fruit/tag.rs`: `FruitTag`.
- `fruit/form.rs`: `FruitForm`, `new_item`, `APPLE`, `Item::fruit`.
- `fruit/eating.rs`: `eating`, `ticks`, `healing`, `RESTORE`, `EXCEPTIONAL`, `ATTRIBUTE`, `VITAL`.
- `fruit/cut.rs`: `cut`.
- `fruit/dev.rs`: `dev_fruit`.
- `fruit/growth.rs`: `PlantFamily::growth`, `has_trunk`, `FruitDef::growth`, `GIANTS`.
- `fruit/plant.rs`: `FruitPlant` (`stage`, `can_pick`), `FruitStage` (`number`), `round_minutes`, `LIFE_ROUNDS`. Tests for plants, harvests, orchards and trunks in `fruit/plant_tests.rs`.
- `fruit/harvest.rs`: `ORCHARDS`, `harvest_range`, `harvest`.
- `fruit/place.rs`: `FruitDef::grows_wild`, `suits_warmth`, `Abundance::wild_weight` / `orchard_weight`, `wild_fruits`, `orchard_fruits`, `choose`.
- `fruit/orchard.rs`: `plant_orchards`, `ROW_GAP`.

## Public API

- `FruitDef { key, name, latin, set, herb, biomes, temperature, unit, kcal, servings, grams, exceptional, harmful_raw, effects, abundance, harvest, plant, plant_look, look }`: `key` its id for good; `temperature` a `herb::Temperature` (yearly average °C); `unit` `One`, `Handful` (a unit under 25 kcal: 100 g of fruit or 20 g of kernels) or `Whole` (over 300 kcal, `servings` of about 100 kcal); `kcal` and `grams` for one item (all of a whole fruit); `effects` 0 to 2 `herb::Effect`s of ±1 or 2; `herb` for a herb's fruit, never an item of its own (`is_item`).
- `Abundance::{Common, Uncommon, Rare, OrchardOnly}`, `HarvestTier::{One, UpToTwo, UpToThree, TwoToFour, ThreeToSix}` (`range`), `PlantFamily` (13 kinds of plant), `FruitLook { shape, size (mm), ripe, unripe, skin }`: for the item pictures and the fruit on plants. `FruitPlantLook { height, spread (cm), leaf, leaf_size, flower, flower_size, fruits }` (fruit plan PR 4: measured from the research's height, spread, leaves, flowers and fruit position, picked out by a script and checked by hand; a climbing vine's height is its support's), `FruitPosition::{Clusters, Single, Tips, Trunk, Crown, Ground}`.
- `Fruits`: `built_in()` (`&'static`), `read(ron_text)` (each checked), `new(defs)`, `get(key)`, `by_tag(tag)`, `iter` (herbs' fruits too), `items` (fruit items only), `len`.
- `FruitTag(u64)`, `FruitTag::of(key)`: a fruit's key hashed through `rng::Seed`, so `bonus::Source` can name a fruit and stay `Copy`.
- `FruitForm { fruit, serving, look, grams }` (`ItemForm::Fruit`): `new(def, seed)`, `serving(def, seed)` (grams a share of the whole, rounded up), `is_whole(def)`, `name()` ("Watermelon", "Watermelon Slice", "Fruit" for a key the table lacks), `shape()` (1 × 1; 2 × 1 from 1 kg; 2 × 2 from 4 kg). `new_item(def, count, id, made_at)`; `Item::fruit()`. `APPLE`: the key old apples, shops and mob loot use.
- `eating(def, now) -> Vec<Effect>`: one serving's healing (`Source::EatenFruit(tag)`, `RESTORE` 1 Health a second, ended by moving or being hurt; none if `harmful_raw`) and its effects (`Source::FruitEffects(tag)`, attributes by `ATTRIBUTE` a step, Health or Mana restored or drained by `VITAL` a second a step, ended by nothing), both for `ticks(def)`: a quarter of a serving's kcal in whole seconds (rounded down), × `EXCEPTIONAL` (3/2) if exceptional. `healing(def)`: the Health in all (0 if harmful).
- `cut(whole, new_id) -> Option<(eaten, Option<rest>)>`: a whole fruit as the serving eaten and a stack of the rest; `None` for anything else.
- `dev_fruit(key, seed)`: Give Item's fruit, by key or rolled from the seed among fruit items, the same on both sides.
- `FruitPlant { fruit, phase, picked }` (a cell's plant; kept on `dungeon::Level`, sent in `protocol::LevelChunk::fruit`): `stage(def, now)` (game minutes) a `FruitStage::{Young, Leafy, Flowering, Fruiting { ripe }}` (`number` 1 to 4), `can_pick(def, now)` (ripe only). `round_minutes(def)`: one round of leaves, flowers and fruit, `FruitDef::growth().days()` game days. `LIFE_ROUNDS` (8).
- `PlantFamily::growth()` (owner: low herbs, mats and canes `Fast`; shrubs, vines, ground vines, rosettes, pad cacti and giant herbs `Medium`; trees, conifers, palms and column cacti `Slow`), `FruitDef::growth()` (the `GIANTS`, baobab, Brazil nut, durian, mango and the two pines, `VerySlow`); `PlantFamily::has_trunk()` (trees, conifers, palms, giant herbs, column cacti: in the way as a wild tree, `Level::has_trunk`).
- `harvest(def, (foraging, orchards), rng)`: how many items picking a ripe plant gives, in `harvest_range(def, foraging, orchards)`: its `HarvestTier`, the cube root of Foraging's percent added to the most and of Orchards' (`ORCHARDS`, `gathering/farming/orchards`) to both, as herbs. An item of a big fruit is one whole fruit.
- `FruitDef::grows_wild(habitat)` (an item fruit, not `OrchardOnly`, in its biomes and temperatures), `suits_warmth(celsius)`; `wild_fruits(fruits, habitat)`, `orchard_fruits(fruits, celsius)` (by warmth alone, any biome, else the nearest to it); `choose(fruits, weight, rng)` by `Abundance::wild_weight` (9/3/1/0) or `orchard_weight` (orchard-only 4, then 3/2/1).
- `plant_orchards(fruits, fields, celsius, seed)`: in each `FieldLook::orchard` field, rows `ROW_GAP` (3) apart from its second cell, never on its last, each one fruit from `orchard_fruits` rolled from the seed and the row's first cell; trunks `ROW_GAP` apart along the row, smaller plants side by side; phases by cell.

## Depends on

- `herb` (`Effect`, `Stat`, `Temperature`, `Rgb`, `Growth`, `MINUTES_PER_DAY`, `skill_percent`, `cube_root`), `climate` (`Biome`), `mob` (`Habitat`), `site` (`FieldLook`), `bonus` (`Effect`, `Source`), `item` (`Item`, `ItemKind::Fruit`, `Shape`), `item_form` (`ItemForm::Fruit`), `rng`, `id`, `ron`, `serde`.

## Used by

- `dungeon` (`Level`'s fruit plants, `has_trunk`), `clearance` (trunks), `wilderness` (`fruit.rs`), `protocol` (`SeenFruit`, `PickNews::PickedFruit`), server `sites/plants.rs` (orchards, harvests kept), `connections/pick_fruit.rs`, client `plants` (`FruitModels`), `dungeon_map` (`fruit.rs`, picking, hover), `notices`.
- `item` (`ItemKind::Fruit`; `saved.rs` gives old apples the apple's form; stacks compare a fruit's form without its look), `item_form` (`give_form` gives an apple; `ItemForm::Fruit`'s name, shape, grams), `bonus` (`Source::EatenFruit`, `FruitEffects`; `is_food` and `may_eat` take fruit), `herb` (`HerbDef::fruit`, `eating_herb`), server `connections/eat_fruit.rs` (`eating`, `cut`), `strength_room.rs`, `give_items.rs` (`dev_fruit`), client `item_art` (`FruitItemLook`: pictures and slices from the look), `effect_bar` (names by `by_tag`), `item_popup` (healing, effects, servings), `debug_menu` (Give Item's fruit dropdown and preview), `dev_tools` (`client.give_item`'s `fruit`).

## Notes

- Built in only, no mods yet (unlike herbs): both sides read the same table, so nothing is sent. Saves and messages name a fruit by key; effects by `FruitTag`.
- Fruit has no quality and is never magical, blessed or cursed. A stack keys on fruit and serving; each keeps its own look seed, the stack showing its bottom one's.
- Eating a whole fruit (server, `eat_fruit.rs`) cuts it: the other servings are given back to what is carried, or put on the ground when it lay there or there is no room; on the world map with no room it is refused.
- Shops and mob loot still give apples (`give_form`): fruit by place for them is left for later.
- Growing (PR 3, Claude's reading of the owner's four stages): a plant is young for one round, then bears for `LIFE_ROUNDS` rounds of leaves, flowers, unripe and ripe fruit (a quarter each), then a young one takes its place; each from its own `phase` (a hash of its cell). Picking (only when ripe) starts a round again from its leaves (owner), so only when it was picked needs keeping, in the server's `plants_picked` with herbs'. No seasons yet.
- Where they grow (PR 3, owner: wild very rare, farms get orchards): wild by `wilderness::fruit_plants`, orchards by `plant_orchards` on the server (`sites/plants.rs`). Cultivated (`OrchardOnly`) fruit grows only in orchards. A cell never has a herb and a fruit plant.
- Trunks (PR 3, Claude's decision from the brief): plants with a trunk block as wild trees do (not walkable, a `TRUNK_RADIUS` post), whatever their stage, so both sides agree whatever the clock says; orchards space them `ROW_GAP` apart and wild ones stand only with open land all round, so nothing is cut off.
- Herbs bearing fruit: eating one gives its raw dose and the fruit's healing and effects (`herb::eating_herb`, `RAW_DOSE_WITH_FRUIT`).
- The apple's ripe colour made greener by hand for its picture (owner, 2026-11-17).
- Temperatures widened by hand so every biome grows some fruit at its extremes: crowberry and bog blueberry −11 to 9, date and coconut to 40, avocado into temperate rainforest.
