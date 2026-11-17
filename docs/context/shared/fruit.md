# shared: fruit

Fruits and nuts (owner, 2026-11-17): what each is, the table of them (built in), fruits as counted items (a whole big fruit cut into servings when eaten), and what eating one does. Not where they grow, growth stages or picking (PR 3 of the fruit plan), their pictures and slices' pictures (PR 2) or plant models (PR 4). Herbs that bear fruit stay herbs (`shared::herb`), linked by `HerbDef::fruit`. Research and the owner's decisions: `docs/context/research-fruit.md`.

## Files

- `crates/shared/src/fruit.rs`: the module list and re-exports. Tests in `fruit/tests.rs`; `fruit/climate_tests.rs` checks every biome grows some fruit item at its hottest and coldest (with `herb::climate_tests::extremes`).
- `fruit/fruits.ron`: the 101 rows (73 fruits, 22 nuts, 6 herbs' fruits with `herb: true`), generated once from the research's `final.json` (the script stayed outside the repo); edit by hand from now on. Read by `include_str!`.
- `fruit/def.rs`: `FruitDef`, `FruitSet`, `Unit`, `check`, `is_item`.
- `fruit/traits.rs`: `Abundance`, `HarvestTier` (`range`), `PlantFamily`.
- `fruit/look.rs`: `FruitLook`, `FruitShape`, `Skin` (data only, for the client).
- `fruit/table.rs`: `Fruits`.
- `fruit/tag.rs`: `FruitTag`.
- `fruit/form.rs`: `FruitForm`, `new_item`, `APPLE`, `Item::fruit`.
- `fruit/eating.rs`: `eating`, `ticks`, `healing`, `RESTORE`, `EXCEPTIONAL`, `ATTRIBUTE`, `VITAL`.
- `fruit/cut.rs`: `cut`.
- `fruit/dev.rs`: `dev_fruit`.

## Public API

- `FruitDef { key, name, latin, set, herb, biomes, temperature, unit, kcal, servings, grams, exceptional, harmful_raw, effects, abundance, harvest, plant, look }`: `key` its id for good; `temperature` a `herb::Temperature` (yearly average °C); `unit` `One`, `Handful` (a unit under 25 kcal: 100 g of fruit or 20 g of kernels) or `Whole` (over 300 kcal, `servings` of about 100 kcal); `kcal` and `grams` for one item (all of a whole fruit); `effects` 0 to 2 `herb::Effect`s of ±1 or 2; `herb` for a herb's fruit, never an item of its own (`is_item`).
- `Abundance::{Common, Uncommon, Rare, OrchardOnly}`, `HarvestTier::{One, UpToTwo, UpToThree, TwoToFour, ThreeToSix}` (`range`), `PlantFamily` (13 kinds of plant), `FruitLook { shape, size (mm), ripe, unripe, skin }`: for PRs 3 and 4.
- `Fruits`: `built_in()` (`&'static`), `read(ron_text)` (each checked), `new(defs)`, `get(key)`, `by_tag(tag)`, `iter` (herbs' fruits too), `items` (fruit items only), `len`.
- `FruitTag(u64)`, `FruitTag::of(key)`: a fruit's key hashed through `rng::Seed`, so `bonus::Source` can name a fruit and stay `Copy`.
- `FruitForm { fruit, serving, look, grams }` (`ItemForm::Fruit`): `new(def, seed)`, `serving(def, seed)` (grams a share of the whole, rounded up), `is_whole(def)`, `name()` ("Watermelon", "Watermelon Slice", "Fruit" for a key the table lacks), `shape()` (1 × 1; 2 × 1 from 1 kg; 2 × 2 from 4 kg). `new_item(def, count, id, made_at)`; `Item::fruit()`. `APPLE`: the key old apples, shops and mob loot use.
- `eating(def, now) -> Vec<Effect>`: one serving's healing (`Source::EatenFruit(tag)`, `RESTORE` 1 Health a second, ended by moving or being hurt; none if `harmful_raw`) and its effects (`Source::FruitEffects(tag)`, attributes by `ATTRIBUTE` a step, Health or Mana restored or drained by `VITAL` a second a step, ended by nothing), both for `ticks(def)`: a quarter of a serving's kcal in whole seconds (rounded down), × `EXCEPTIONAL` (3/2) if exceptional. `healing(def)`: the Health in all (0 if harmful).
- `cut(whole, new_id) -> Option<(eaten, Option<rest>)>`: a whole fruit as the serving eaten and a stack of the rest; `None` for anything else.
- `dev_fruit(key, seed)`: Give Item's fruit, by key or rolled from the seed among fruit items, the same on both sides.

## Depends on

- `herb` (`Effect`, `Stat`, `Temperature`, `Rgb`), `climate` (`Biome`), `bonus` (`Effect`, `Source`), `item` (`Item`, `ItemKind::Fruit`, `Shape`), `item_form` (`ItemForm::Fruit`), `rng`, `id`, `ron`, `serde`.

## Used by

- `item` (`ItemKind::Fruit`; `saved.rs` gives old apples the apple's form; stacks compare a fruit's form without its look), `item_form` (`give_form` gives an apple; `ItemForm::Fruit`'s name, shape, grams), `bonus` (`Source::EatenFruit`, `FruitEffects`; `is_food` and `may_eat` take fruit), `herb` (`HerbDef::fruit`, `eating_herb`), server `connections/eat_fruit.rs` (`eating`, `cut`), `strength_room.rs`, `give_items.rs` (`dev_fruit`), client `effect_bar` (names by `by_tag`), `item_popup` (healing, effects, servings), `debug_menu` (Give Item's fruit dropdown and preview), `dev_tools` (`client.give_item`'s `fruit`).

## Notes

- Built in only, no mods yet (unlike herbs): both sides read the same table, so nothing is sent. Saves and messages name a fruit by key; effects by `FruitTag`.
- Fruit has no quality and is never magical, blessed or cursed. A stack keys on fruit and serving; each keeps its own look seed, the stack showing its bottom one's.
- Eating a whole fruit (server, `eat_fruit.rs`) cuts it: the other servings are given back to what is carried, or put on the ground when it lay there or there is no room; on the world map with no room it is refused.
- Shops and mob loot give apples (`give_form`) until PR 3 decides what grows where.
- Herbs bearing fruit: eating one gives its raw dose and the fruit's healing and effects (`herb::eating_herb`, `RAW_DOSE_WITH_FRUIT`).
- Temperatures widened by hand so every biome grows some fruit at its extremes: crowberry and bog blueberry −11 to 9, date and coconut to 40, avocado into temperate rainforest.
