# shared: livestock

The animals people keep (owner's research, 2026-11-19): a table of data with lookups, no gameplay yet. Livestock plan PR 1 of 8 (2 body parts and the sexes, 3 to 5 new kinds with models, 6 life stages and castrates, 7 wild animals from the table, 8 kept animals on farms and in towns; production later). Research, the owner's decisions (numbered there, and below) and every table: `docs/context/research-livestock.md`.

## Files

- `crates/shared/src/livestock.rs`: the module list and re-exports. Tests in `livestock/tests.rs`.
- `livestock/livestock.ron`: the 26 animals, written once from the research's `design.py`; edit by hand. Read by `include_str!`.
- `livestock/parts.ron`: the organs (26), fats (9) and other parts (10), each naming the animals it comes from.
- `livestock/def.rs`: `LivestockDef`, `Wild`, `Pet`, `check`.
- `livestock/traits.rs`: `Class`, `Rarity`, `Diet`, `Water`, `Often`.
- `livestock/wild_place.rs`: `Place`, `Warmth`, `COLD_UP_TO`, `HOT_FROM` (PR 7).
- `livestock/stages.rs`: `Stages`, `Stage`, `Sexed`, `Adult`, `GAME_HOURS_A_MONTH`.
- `livestock/age.rs`: `age_hours`, `stage_of`, `LIFE_CYCLES` (PR 6).
- `livestock/produce.rs`: `Produce`, `Product`, `Per`.
- `livestock/meat.rs`: `Meat`, `ByQuality`, `SHARES`, `YOUNG_YIELD_PERCENT`.
- `livestock/hide.rs`: `Hide`, `YoungHide`, `Cure`, `HIDE_ROTS_AFTER_SECONDS`.
- `livestock/parts.rs`: `Organ`, `Fat`, `Part`, `Animals`, `Only`.
- `livestock/parts_table.rs`: `Parts`.
- `livestock/drop.rs`: `organ_chance`, `OUT_OF`.
- `livestock/table.rs`: `Livestock`.

## Public API

- `LivestockDef { key, name, latin, class, ruminant, kind, variant_of, kept, kept_note, wild, feral, diet, favourites, water, water_note, produce, work, stages, meat, hide, pet }`: `key` its id for good; `class` `Mammal`, `Bird` or `Insect` (bees and silkworms: objects, decision 14); `ruminant` chews the cud (tripe); `kind` the `CreatureKind` that is it (Cattle, Coo, Horse, Chicken, Sheep, Goat, Pig, WaterBuffalo, Yak, Donkey, Mule, Dromedary, BactrianCamel, Llama, Alpaca, Reindeer, Rabbit, Duck, MuscovyDuck, Goose, Turkey, Guineafowl, Pigeon, Quail so far), each kind at most once; `variant_of` the animal it is a variant of (Coo of cattle, Muscovy of duck, Bactrian of dromedary, alpaca of llama; decision 13: its own row and kind); `kept` biomes; `wild` an optional `Wild { name, rarity, biomes, note, place, warmth }` (aurochs and wild dromedaries kept, decision 1; none for the mule, silkworm and Coo); `feral` a `Wild` without a name or biomes (it lives where the animal is kept); `Place` where in its biomes wild spawns put it (`Anywhere` by default, `Mountains`, `High`, `ByWater`, `NearPeople`, `Cliffs`), `Warmth` how warm (`Any`, `Cold` to 14 °C: Bactrian camels, wild yaks; `Hot` from 15 °C: dromedaries, wild asses, wild water buffalo); `Rarity` `Never` to `VeryCommon`; `Diet`; `favourites` as words; `Water` `Avoids`, `Drinks`, `Wades`, `Swims`, `Likes`, `Prefers`; `work` as words; `pet` a `Pet { often: Often, note }`.
- `Produce { product, amount, per, chance, grams, note }`: game rates per game day (decision 3). `Product` (`Milk` ml, `Wool`, `Fleece`, `AngoraWool`, `Honey` g, `Egg` count, `Cocoons` trays; `unit()`), `Per` (`Day`, `Shearing`, `FloweringDay`, `Grown`), `chance` percent (eggs), `grams` an egg's weight.
- `Stages { young, juvenile: Sexed, adult: Adult, young_until, juvenile_until }`: names by sex (`Adult` with `castrate` and `others`), ends in game hours from birth (a real month's growth a game day: `GAME_HOURS_A_MONTH` 24, decision 2); `at(age_hours) -> Stage` (`Young`, `Juvenile`, `Adult`, the default).
- `age_hours(id, world_minutes, stages) -> u32`, `stage_of(kind, id, world_minutes) -> Stage` (PR 6): a creature's age, never saved: its id picks a start in a cycle of `LIFE_CYCLES` (4) times its growing up (`juvenile_until`), and the world's clock moves it on, so about a quarter of a kind are growing at any time and they grow as time passes; nothing dies of old age (owner), the age starting again at the cycle's end. `stage_of` is `Adult` for kinds not in the table. The server sends it as `mob::EntityLook::stage`; the body for it is `body::Layout::of_stage`.
- `Meat { name, young, cuts: ByQuality }`: `ByQuality { poor .. legendary }` lists of names, empty for a quality never given (`get`, `qualities`); `cut_name(quality, hash)` one chosen by hash (decision 4); `young_lowest()` a step above the worst (decision 5); `shares(young)` each quality's percent of a carcass, `SHARES` kept for its qualities and scaled to 100 (decision 6); `YOUNG_YIELD_PERCENT` (25) a young animal's meat.
- `Hide { name, cure, cured, lowest, highest, young, note }`: `Cure::Leather` or `Pelt` (furry, decision 7), a `YoungHide` (calfskin, lambskin, kidskin), none for birds and insects; `HIDE_ROTS_AFTER_SECONDS` (7 h, decision 8; salt stops it).
- `Parts { organs, fats, others }`: `built_in()`, `read(text, animals)` (checked against the animals), `organs_of(def)`, `fats_of(def)`, `others_of(def)`. `Organ { key, name, from, except, only, food, effects, other, base, stone, bottled, quest }` (`effects` herb `Effect`s, proposed; decision 10 leaves Alchemy's use open; `bottled` blood, decision 12), `Fat { key, name, from, rendered_from, note }` (every fat an ointment base, decision 11), `Part { key, name, from, only, uses }`; `Animals` (`All` mammals and birds, `Mammals`, `Birds`, `Ruminants`, `Animal(key)`), `Only` (`Young`, `Old`, `IntactMale`, `Fattened`, `GrowingAntlers`), `comes_from(def)` ignoring `only`.
- `organ_chance(base, stone, butchery) -> u32` out of `OUT_OF` (10,000): base + (95 − base) × Butchery%, a stone base × 0.5 to 1.5 (decision 9).
- `Livestock`: `built_in()`, `read(ron_text)` (each checked; keys and kinds once; variants of a row that is no variant), `new(defs)`, `get(key)`, `by_kind(kind)`, `variants(key)`, `iter`, `len`.

## Depends on

- `climate` (`Biome`), `creature` (`CreatureKind`), `id` (`Id`), `item` (`Quality`), `herb` (`Effect`), `ron`, `serde`.

## Used by

- `body` (`Stage` for `Layout::of_stage`), `mob` (`EntityLook::stage`; `wild_spawns` from wild forms, feral herds, stages and `age_hours`, PR 7), server `entities/sync.rs` (`stage_of`), client `item_art` and `entities`. The plan's PR 8 (farms) and production will read the rest.

## Notes

- Built in only, both sides reading the same table, so nothing is sent. Saves and messages will name an animal by key.
- Words (favourites, work, notes, organs' food and uses) are English for now, as the research wrote them; they move into language files when something shows them.
- Variants have rows of their own (decision 13), so a kind finds its row with `by_kind` and its family with `variant_of`.
