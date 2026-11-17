# shared: herb

Herbs for potion making: what each is, the table of them (built in, plus a server's mods), herbs as counted items as good as their plant is rare, how many picking gives, the potency a user draws out, and what a character knows of each. Where herbs grow and how they grow and are harvested (PR 2; the wilderness's patches are placed by `shared::wilderness`, the server runs picking). Plant models and picking by click are the client's (`plants`, `dungeon_map`); items' pictures and popups too (`item_art`, `item_popup`). Eating one raw (PR 4) gives a weak dose of its first effect (`raw_dose`). Potions are bottles (`shared::potion`, `potion.md`); brewing them from herbs is not built yet. Research and the owner's decisions: `docs/context/research-herbs.md`.

## Files

- `crates/shared/src/herb.rs`: the module list and re-exports. Tests in `herb/tests.rs`; `herb/climate_tests.rs` checks every biome grows something at its hottest and coldest.
- `herb/herbs.ron`: the 82 built-in herbs, generated once from the research's `final.json` (the script stayed outside the repo); edit by hand from now on. Read by `include_str!`.
- `herb/def.rs`: `HerbDef`, `EFFECTS` (4), `HerbDef::check`.
- `herb/traits.rs`: `Abundance` (`quality`), `Temperature` (`suits`), `Part`.
- `herb/growth.rs`: `Growth` (`days`), `Regrow` (`back_to`), `Early`, `STAGES`, `EARLY_STAGE`.
- `herb/effect.rs`: `Stat` (the nine attributes, Health, Mana; `ALL`, `attribute`), `Effect` (`strength`, `raises`).
- `herb/plant.rs`: `PlantLook`, `PlantShape`, `PartSize`, `Rgb`.
- `herb/table.rs`: `Herbs`.
- `herb/form.rs`: `HerbForm`, `new_item`, `Item::herb`.
- `herb/skills.rs`: `FORAGING`, `HERBS`, `skill_value`, `skill_percent`, `cube_root`.
- `herb/settle.rs`: `settle_herbs` (old saves' herbs put right on loading).
- `herb/potency.rs`: `HERBALISM`, `quality_percent`, `skill_cap`, `potency`.
- `herb/knowledge.rs`: `HerbKnowledge`, `KnownHerb`.
- `herb/patch.rs`: `HerbPatch` (`stage`, `can_pick`), `stage_minutes`, `MINUTES_PER_DAY`.
- `herb/harvest.rs`: `Harvest`, `harvest`, `harvest_range`, `FULL_HARVEST`.
- `herb/place.rs`: `HerbDef::grows_in` / `suits`, `MARSH_STAND_INS`, `Abundance::wild_weight` / `bed_weight`, `wild_herbs`, `bed_herbs`, `choose`.
- `herb/beds.rs`: `plant_beds`.
- `herb/raw.rs`: `raw_dose`, `RAW_TICKS`, `RAW_ATTRIBUTE`, `RAW_VITAL`. Tests in `herb/raw_tests.rs` (with `dev_herb` and knowledge).
- `herb/dev.rs`: `dev_herb` (Give Item's herb).

## Public API

- `HerbDef { key, name, latin, biomes, marsh, abundance, temperature, part, part_name, growth, regrow, early, effects, grams, plant }`: `key` is its id for good (lower case, digits, `_`); `biomes` are `climate::Biome`s, `marsh` kept for when the climate has marshes; `temperature` the yearly average °C range (`climate`'s measure); `effects` four `Effect { stat, change }` (change ±1 to 3); `grams` one item's weight; `check()` an error for a mod's author.
- `Growth::{Fast, Medium, Slow, VerySlow}` (`days`: 2, 4, 8, 16 game days for all `STAGES`), `Regrow::{Cut, Pick, Dig, Coppice}` (`back_to`: stage 2, 3, gone, 1), `Early { taken, strength, note }` (percentages for a harvest at `EARLY_STAGE`, 3).
- `PlantLook { shape, height, spread (cm), leaf, leaf_size, flower, flower_size, form, description }` for the client's models.
- `Herbs`: `built_in()`, `read(ron_text) -> Result<Vec<HerbDef>, String>` (each checked), `merge(defs)` (by key: replaces in place, else appends; returns how many replaced), `get(key)`, `iter`, `len`. Serialized as the plain list, its index rebuilt when read.
- `Abundance::quality()`: Common, Uncommon, Rare: every picked herb's quality.
- `new_item(def, count, id, made_at)`: a counted `ItemKind::Herb` item at its plant's rarity (`Abundance::quality`) with `ItemForm::Herb(HerbForm { herb, look, grams })` (look from the id's seed). `Item::herb()`.
- `skill_value(skills, path)` (a check's total along the path, at most 10,000 = 100%), `skill_percent(value)` (whole percent), `cube_root(n)` (whole, rounded down); `FORAGING`, `HERBS` (`gathering/farming/herbs`), `HERBALISM` (paths).
- `settle_herbs(inventory, herbs, limits)`: every carried herb (any grid, bag, the bin) given its plant's rarity, then stacks alike in each grid joined as far as `limits` allow; herbs the table lacks left alone; how many changed. The server runs it on loading a character (`connections/welcome.rs`).
- `potency(quality, herbalism)`, `quality_percent(quality)` (Poor 80 to Artifact 200), `skill_cap(herbalism)` (80 at none to 200 at 100%).
- `HerbPatch { herb, phase, picked }` (a cell's herb; kept on `dungeon::Level`, sent in `protocol::LevelChunk::herbs`): `stage(def, now)` (game minutes: 1 to `STAGES`, 0 while dug up), `can_pick(def, now)` (at `EARLY_STAGE` or more, and grown a stage since last picked). `stage_minutes(def)` (a quarter of its `Growth::days`).
- `harvest(def, stage, (foraging, herbs), rng) -> Option<Harvest { count, quality }>`: full grown a number in `harvest_range(foraging, herbs)`; at `EARLY_STAGE` its `Early::taken` share of it (rounded; `None` if nothing); quality its rarity. `harvest_range`: `FULL_HARVEST` (1 to 3), the cube root of Foraging's percent added to the most and of Herbs' to both (100% Foraging 1 to 7, Herbs 5 to 7, both 5 to 11).
- `HerbDef::grows_in(biome)`, `suits(&mob::Habitat)` (biome and `temperature`); `wild_herbs(herbs, habitat)`, `bed_herbs(herbs, habitat)` (wild ones, else those nearest its warmth); `choose(herbs, weight, rng)` by `Abundance::wild_weight` (9/3/1) or `bed_weight` (3/2/1).
- `plant_beds(level, herbs, habitat, seed)`: each bed (`HerbBed` cells joined side by side: one strip) one herb from `bed_herbs`, rolled from the site's seed and the bed's first cell; each patch's phase from its cell.
- `raw_dose(def, quality, herbalism, now) -> Option<bonus::Effect>`: eating one raw: its first effect for `RAW_TICKS` (30 s), an attribute by `RAW_ATTRIBUTE` (1 point) a step of strength, Health or Mana restored or drained by `RAW_VITAL` (0.5 a second) a step, times `potency` / 100; `Source::RawHerb`, nothing ends it early.
- `dev_herb(herbs, key, seed)`: the herb with this key, or with none one rolled from the seed (`item_form::batch_seed`): what Give Item gives, the same on both sides.
- `HerbKnowledge` (on `Sheet::herbs`): `of`, `knows_name`, `knows_effect`, `learn_name`, `learn_effect` (each returns whether it was new), `learn_all(key)` (name and every effect), `forget_all` (dev tools), `known_effects(def)`, `is_empty`. `KnownHerb { name, effects }`.

## Depends on

- `climate` (`Biome`), `mob` (`Habitat`), `dungeon` (`Level`, `Cell::HerbBed`), `sheet` (`Attribute`), `item` (`Item`, `ItemKind`, `Quality`, `ODDS_OUT_OF`), `item_form` (`ItemForm::Herb`), `skill` (`Skills`, `SkillId`), `rng`, `id`, `ron`, `serde`.

## Used by

- `item` (`ItemKind::Herb`, stacks compare a herb's form without its look), `sheet` (`Sheet::herbs`), `dungeon` (`Level`'s herbs), `wilderness` (`herbs.rs`), `protocol` (`ServerMessage::Herbs`, `LevelChunk::herbs`, `HerbNews`), server `sites/herbs.rs`, `connections/pick_herb.rs`, `eat_herb.rs` (`raw_dose`), `dev_herbs.rs` and `give_items.rs` (`dev_herb`), server `herbs.rs` (loading mods) and `connections/welcome.rs` (sending), client `common::HerbTable`, `item_art` (`herb.rs`: pictures from `PlantLook`), `item_popup` (`herb.rs`: names and effects as known), `debug_menu` (Give Item's herb, the Herbs panel), `notices`, and `cargo xtask wiki` (the wiki's page per herb, never its effects). Herbs from outside go into a tied Herb Pouch first (`ItemKind::HerbPouch`, `Inventory::route_for`, in `item`).

## Notes

- Saves and messages name a herb by key, never by its place in the table, so mods can add and replace freely. A herb item keeps its own `grams`, so a mod changing a herb's weight leaves picked ones as they were (and they no longer stack with new ones).
- Herb items are counted with quality (the one counted kind with quality and a form): a stack keys on herb and quality only, each one's look seed ignored (`as_compared` zeroes it), and shows its bottom's look. Never magical, blessed or cursed (only worn kinds can be; a test checks). `give_form` gives a herb nothing, as it has no table: herbs are made with `new_item`. `Item::name` says "Herb"; the herb's name, or "Unknown herb", is for the client to show from `HerbTable` and `HerbKnowledge` (PR 4).
- Quality (owner, 2026-11-17): always the plant's rarity, never rolled (picked, beds, Give Item, which ignores the quality asked for). Saves from before had rolled ones (Poor to Artifact): `settle_herbs` puts them right when a character loads, so stacks join.
- Potency (owner): quality counts only as far as Herbalism reaches: `min(quality_percent, skill_cap)`, so with none every herb is as weak as a poor one. Eating raw uses it (`raw_dose`).
- Knowledge (owner): only changes what is shown. Picking teaches the name (buying will), eating raw the first effect; Herbalism practice, potions, books and training the rest.
- Eating raw (PR 4, first guesses): `raw_dose` above, scaled by `potency`, so an unskilled eater gets 80% whatever the quality. Anyone may eat any herb (`bonus::may_eat`). Its effect is its own source (`Source::RawHerb`): the latest herb replaces the last, alongside food.
- Growth (PR 2): a patch goes up through the four stages at its herb's speed, stays full grown as long again, then starts from stage 1 (it seeds and dies back), each from its own `phase` (a hash of its cell), so only when it was last harvested needs keeping. Harvesting starts that round again at its `Regrow::back_to` stage (dug up: nothing for a whole growth, then stage 1). A harvest needs a stage's growth since the last, so picked fruit (back to 3) can't be picked again at once. Game minutes are the world's (`WorldTime`), so moving the clock moves them. No seasons yet.
- Where they grow (PR 2): wild by each world map point's biome and yearly average temperature (`Wilderness::habitat_at`), commoner herbs more often, never in a poi's cutout or on the hatched border round it (`ChunkPlan::keeps_clear`); a herb of marshes alone grows in the wettest biomes (`MARSH_STAND_INS`) until there are marshes; those with other biomes grow only there. Herbalists grow the same, rarer ones more often, or the nearest to the place's warmth where nothing grows wild (hot deserts). Placing depends on the server's table (mods), so it isn't part of any generator's version: a mod changing herbs changes what grows where, harvests kept by cell.
- Yield (owner, 2026-11-17): 1 to 3 untrained, more with Foraging (the most) and Herbs (both), whole numbers only so both sides agree; early its `taken` share. `Early::strength` no longer changes quality. Herbs has no practice yet: planting will give it.
- Temperatures: the climate's hottest is 40 °C (`Wilderness::habitat_at` rounds 39.5), its forests' warmest 24 °C; aloe, creosote bush, henna, devil's claw and neem reach 41, plantain 25, so every biome grows something at both ends (owner asked for the desert four; the test found the rest).
