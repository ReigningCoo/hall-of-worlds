# shared: herb

Herbs for potion making: what each is, the table of them (built in, plus a server's mods), herbs as counted items, the quality picking gives, the potency a user draws out, and what a character knows of each. Not yet: where herbs grow and picking them (PR 2), plant models (PR 3), pictures, popups and eating (PR 4), potions. Research and the owner's decisions: `docs/context/research-herbs.md`.

## Files

- `crates/shared/src/herb.rs`: the module list and re-exports. Tests in `herb/tests.rs`.
- `herb/herbs.ron`: the 82 built-in herbs, generated once from the research's `final.json` (the script stayed outside the repo); edit by hand from now on. Read by `include_str!`.
- `herb/def.rs`: `HerbDef`, `EFFECTS` (4), `HerbDef::check`.
- `herb/traits.rs`: `Abundance`, `Temperature` (`suits`), `Part`.
- `herb/growth.rs`: `Growth` (`days`), `Regrow` (`back_to`), `Early`, `STAGES`, `EARLY_STAGE`.
- `herb/effect.rs`: `Stat` (the nine attributes, Health, Mana; `ALL`, `attribute`), `Effect` (`strength`, `raises`).
- `herb/plant.rs`: `PlantLook`, `PlantShape`, `PartSize`, `Rgb`.
- `herb/table.rs`: `Herbs`.
- `herb/form.rs`: `HerbForm`, `new_item`, `Item::herb`.
- `herb/quality.rs`: `FORAGING`, `HERB_ODDS`, `pick_odds`, `pick_quality`, `skill_value`.
- `herb/potency.rs`: `HERBALISM`, `quality_percent`, `skill_cap`, `potency`.
- `herb/knowledge.rs`: `HerbKnowledge`, `KnownHerb`.

## Public API

- `HerbDef { key, name, latin, biomes, marsh, abundance, temperature, part, part_name, growth, regrow, early, effects, grams, plant }`: `key` is its id for good (lower case, digits, `_`); `biomes` are `climate::Biome`s, `marsh` kept for when the climate has marshes; `temperature` the yearly average °C range (`climate`'s measure); `effects` four `Effect { stat, change }` (change ±1 to 3); `grams` one item's weight; `check()` an error for a mod's author.
- `Growth::{Fast, Medium, Slow, VerySlow}` (`days`: 2, 4, 8, 16 game days for all `STAGES`), `Regrow::{Cut, Pick, Dig, Coppice}` (`back_to`: stage 2, 3, gone, 1), `Early { taken, strength, note }` (percentages for a harvest at `EARLY_STAGE`, 3).
- `PlantLook { shape, height, spread (cm), leaf, leaf_size, flower, flower_size, form, description }` for the client's models.
- `Herbs`: `built_in()`, `read(ron_text) -> Result<Vec<HerbDef>, String>` (each checked), `merge(defs)` (by key: replaces in place, else appends; returns how many replaced), `get(key)`, `iter`, `len`. Serialized as the plain list, its index rebuilt when read.
- `new_item(def, quality, count, id, made_at)`: a counted `ItemKind::Herb` item with `ItemForm::Herb(HerbForm { herb, look, grams })` (look from the id's seed). `Item::herb()`.
- `pick_quality(rng, foraging)`, `pick_odds(foraging)`, `HERB_ODDS`; `skill_value(skills, path)` (a check's total along the path, at most 10,000 = 100%), `FORAGING`, `HERBALISM` (paths).
- `potency(quality, herbalism)`, `quality_percent(quality)` (Poor 80 to Artifact 200), `skill_cap(herbalism)` (80 at none to 200 at 100%).
- `HerbKnowledge` (on `Sheet::herbs`): `of`, `knows_name`, `knows_effect`, `learn_name`, `learn_effect` (each returns whether it was new), `known_effects(def)`, `is_empty`. `KnownHerb { name, effects }`.

## Depends on

- `climate` (`Biome`), `sheet` (`Attribute`), `item` (`Item`, `ItemKind`, `Quality`, `ODDS_OUT_OF`), `item_form` (`ItemForm::Herb`), `skill` (`Skills`, `SkillId`), `rng`, `id`, `ron`, `serde`.

## Used by

- `item` (`ItemKind::Herb`, stacks compare a herb's form without its look), `sheet` (`Sheet::herbs`), `protocol` (`ServerMessage::Herbs`), server `herbs.rs` (loading mods) and `connections/welcome.rs` (sending), client `common::HerbTable`.

## Notes

- Saves and messages name a herb by key, never by its place in the table, so mods can add and replace freely. A herb item keeps its own `grams`, so a mod changing a herb's weight leaves picked ones as they were (and they no longer stack with new ones).
- Herb items are counted with quality (the one counted kind with quality and a form): a stack keys on herb and quality only, each one's look seed ignored (`as_compared` zeroes it), and shows its bottom's look. Never magical, blessed or cursed (only worn kinds can be; a test checks). `give_form` gives a herb nothing, as it has no table: herbs are made with `new_item`. `Item::name` says "Herb"; the herb's name, or "Unknown herb", is for the client to show from `HerbTable` and `HerbKnowledge` (PR 4).
- Quality (owner): Poor to Uncommon mostly; Rare and better 2 in 100 with no Foraging (other items 8), 7 in 100 at 100% (`SKILLED_SHIFT`, linear, Poor taking what rounding leaves). First guesses.
- Potency (owner): quality counts only as far as Herbalism reaches: `min(quality_percent, skill_cap)`, so with none every herb is as weak as a poor one. Nothing uses it yet.
- Knowledge (owner): only changes what is shown. Picking or buying will teach the name, eating raw the first effect; Herbalism practice, potions, books and training the rest.
