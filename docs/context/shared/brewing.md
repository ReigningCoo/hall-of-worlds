# shared: brewing

The brewing rules (brewing plan PR 4, owner's decisions 6 to 16 and 18 in `docs/context/brewing.md`): herbs and an oil ground into an ointment, an ointment and a water brewed into a potion, a potion's quality and how long it lasts, and an ointment used on oneself. Pure functions on whole numbers, so client and server agree. Also Grind's shared pieces (PR 6): the ointment item, the carried stacks, ointment on the skin and the recipes a character remembers. And Brew's (PR 7): the carried stacks, the bottle of water turned into the potion, what brewing teaches of herbs. And drinking's (PR 8): a potion's effects and the cooldowns. Grind and Brew themselves, their orders, times and practice, gave way to `shared::alchemy` (Alchemy plan PR 3), which uses these sums and stacks.

## Files

- `crates/shared/src/brewing.rs`: the module list and re-exports. Tests in `brewing/tests.rs` (grinding) and `brewing/brew_tests.rs` (waters, brewing, quality, durations, on the skin).
- `brewing/skill.rs`: `BrewSkills`, `square_root`, and the limits from skills: `per_herb_cap`, `stat_cap`, `most_kinds`, `most_doses`, `cut_steps`.
- `brewing/boost.rs`: `boosted` (+25%, at least 1), `stat_row`, `oil_suits`.
- `brewing/cut.rs`: `cut_unwanted` (Herbalism's cuts).
- `brewing/grind.rs`: `Ointment`, `Refused`, `herb_steps`, `grind`, `ground_doses`.
- `brewing/ingredient.rs`: `Ingredient` (Herb or Fungus), `GroundHerb` (`herb`, `fungus`, `same_kind`).
- `brewing/brew.rs`: `watered`, `brew`, `POTION_AT`, `KEPT_FROM`.
- `brewing/ointment.rs`: `OintmentForm`, `new_ointment`, `OINTMENT_GRAMS`, `Item::ointment`.
- `brewing/stacks.rs`: `HerbStack`, `OilStack`, `Inventory::herb_stacks`, `oil_stacks`. Tests in `brewing/grind_item_tests.rs` (with the other PR 6 pieces).
- `brewing/recipe.rs`: `Recipe`, `BrewedWith`, `Recipes`, `MOST_RECIPES`.
- `brewing/bottles.rs`: `OintmentStack`, `WaterStack`, `Inventory::ointment_stacks`, `water_stacks`, `brewed_into`. Tests in `brewing/brew_item_tests.rs` (with `lesson.rs` and recipes' brewing).
- `brewing/lesson.rs`: `learn_from_potion`.
- `brewing/drink.rs`: `Drunk`, `drunk`. Tests in `brewing/drink_tests.rs` (with `cooldown.rs`).
- `brewing/cooldown.rs`: `Combination`, `Cooldowns`, `DRINK_COOLDOWN_SECONDS`, `DRINK_COOLDOWN_TICKS`.
- `brewing/skin.rs`: `Applied`, `applied`, `skin_effect`, `VITAL_PER_STEP`, `ATTRIBUTE_PER_STEP`.
- `brewing/quality.rs`: `quality`, `purity`, `quality_of_purity`, `duration_seconds`, `duration_ticks`, `on_skin` and their constants.

## Public API

- `BrewSkills { alchemy, herbalism }` (hundredths of a percent, as `herb::skill_value`): `of(skills)`, `alchemy_percent`, `herbalism_percent`.
- Limits: `per_herb_cap` (√Alchemy% + 1, 1 to 11 steps an effect a herb), `stat_cap` (4 × that, 4 to 44), `most_kinds` (4 + ∛Alchemy%, 4 to 8), `most_doses` (∛Alchemy% + 1, 1 to 5), `cut_steps` (∛Herbalism%, 0 to 4). `square_root` (whole, rounded down).
- `herb_steps(effect, quality, skills)`: one herb's steps on one effect: `change × herb::potency`, rounded to the nearest, capped.
- `GroundHerb { ingredient, key, effects, quality }`: one kind in a mix, `GroundHerb::herb(def, quality)` or `GroundHerb::fungus(def)` (a fungus's four herb effects, Common: fungi have no quality, fungi plan PR 8); `same_kind` by ingredient and key, so a herb and a fungus sharing a key are two kinds.
- `grind(herbs: &[GroundHerb], oil, doses, skills) -> Result<Ointment { contents, doses }, Refused>`: refuses no herbs, the same herb twice (`same_kind`, whatever the quality), more kinds than `most_kinds`, doses outside 1 to `most_doses`. Steps added per stat (opposites cancel), the oil's row `boosted`, each stat clamped to `stat_cap`, then `cut_unwanted`. Contents can be empty. The caller checks `doses` of each herb and the oil are carried and takes them. `ground_doses(herbs, Option<Oil>, skills)`: the same sums without the checks, also for the Alchemy plan's Mortar (`shared::alchemy`; no oil, no boost).
- `cut_unwanted(&mut [(Stat, i32)], steps)`: the wanted way is the largest dose's sign; steps off the others one at a time, round robin from the largest.
- `boosted(step)`, `stat_row(stat)` (Health Body, Mana Mind), `oil_suits(oil) -> Option<Row>` (Grain Body, Nut Mind, Seed Spirit, Neutral none).
- `watered(ointment, water)`: Cave Water boosts harmful doses, Moon Water helpful ones, Salt Water takes the strongest helpful dose, Rain Water the strongest harmful one, Water nothing. `brew(ointment, water) -> Option<Contents>`: after the water, a potion if any dose reaches `POTION_AT` (4), keeping doses of `KEPT_FROM` (2) or more; `None` otherwise (ingredients still used, practice still given: the caller's).
- `quality(contents)`: (strongest − next strongest, by size) ÷ 3: ≤2 Poor, 3–4 Common, 5–6 Uncommon, 7–8 Rare, 9–10 Epic, 11+ Legendary; for ointments too. Split as `purity(contents)` (the difference) and `quality_of_purity(score)` (the bands), so Alchemy's Pelican raises the score in between.
- `duration_seconds(quality)` / `duration_ticks`: `POOR_SECONDS` (30), doubling each quality (Alchemy plan): Common 60 s, Uncommon 120, Rare 240, Epic 480, Legendary 960.
- `on_skin(ointment)`: each dose ÷ `ON_SKIN_SHARE` (4), towards 0, zeros dropped; it lasts as `duration_*` of the ointment's own `quality`.

- Ointment (PR 6): `ItemKind::Ointment` with `ItemForm::Ointment(OintmentForm { contents })`, counted, its quality `quality(contents)` (`new_ointment(contents, count, id)`), 60 g each.
- `herb_stacks()` (`HerbStack { stack, ingredient, herb, quality, count }`: herbs, then whole fungi with herb effects (not servings), by key, best quality first; never worn or quest items) and `oil_stacks()`: what Alchemy's Mortar can take (`alchemy::check_alchemy`).
- `Recipe { herbs (keys, sorted), oil, made, brewed }` (`new`, `same_mix`; `brewed: Option<BrewedWith { water, potion }>`, what its ointment last brewed into, none in older saves), `Recipes` (`remember`: to the front, the same mix replaced, at most `MOST_RECIPES` 20; `brewed(made, with)`: noted on the latest mix whose `made` is those doses; `iter`): saved on `Sheet::recipes`, sent with the sheet.
- `ointment_stacks()` (best quality first) and `water_stacks()` (by water) for Alchemy's panel. `brewed_into(bottle, potion)`: the bottle holding the potion, its look kept and its quality `quality(potion)` (Alchemy then sets its own), or empty.
- `learn_from_potion(known, recipes, herbs, ointment, potion)`: each herb of the mix remembered as making `ointment` learns its effects on the potion's stats; whether anything was new. An ointment no remembered mix made teaches nothing.
- `skin_effect(contents, now) -> Applied { effect, vitals }`: `on_skin`'s quarter, attributes `ATTRIBUTE_PER_STEP` (1 point) a step for `duration_ticks(quality(contents))` as `Source::Ointment`, Health and Mana `VITAL_PER_STEP` (5) a step at once (decision 17's rule, which drinking shares through `applied`).
- Drinking (PR 8, decisions 17 to 21; Darkvision from Alchemy PR 8: its dose a `bonus::Stat::Darkvision` of `dungeon::darkvision_reach(step, quality)` cells, as `applied` now takes the quality): `drunk(contents, quality, now) -> Drunk { effects, vitals }`: each attribute's dose its own `Effect` (`Source::Potion(stat)`, `ATTRIBUTE_PER_STEP` a step) for `duration_ticks(quality)` of the bottle's quality, so a new dose on a stat replaces only that stat's; Health and Mana `VITAL_PER_STEP` a step at once.
- `Combination::of(contents)`: a bit per stat and sign, whatever the strength. `Cooldowns` (`ready_at`, `ready`, `drank`: forgets those over), saved on `Sheet::drinks` and sent with it: `DRINK_COOLDOWN_SECONDS` (30) before the same combination can be drunk again (decision 20).

## Depends on

- `fungus` (`Fungi`, `FungusDef`, `Item::fungus`), `herb` (`HerbDef`, `Effect`, `Stat`, `potency`, `cube_root`, `skill_value`, `skill_percent`, `HERBALISM`, `Herbs`), `potion` (`Contents`, `Dose`, `Oil`, `Water`, `Liquid`, `BottleForm`), `bonus` (`Effect`, `Source`, `Bonus`), `vitals` (`Vital`, `PER_POINT`), `id`, `item_form` (`ItemForm`), `ability` (`ALCHEMY`), `sheet` (`Row`), `item` (`Quality`), `energy` (`ACTION_TICKS_PER_SECOND`), `skill` (`Skills`).

## Used by

- `alchemy` (the Mortar: `ground_doses`, `herb_steps`, `most_kinds`, `quality`, `POTION_AT`, `KEPT_FROM`; the order: `herb_stacks`, `oil_stacks`, `OintmentForm`, `new_ointment`, `brewed_into`), `item` (`ItemKind::Ointment`), `item_form` (`ItemForm::Ointment`, Give Item's), `sheet` (`Sheet::recipes`), server `connections/alchemy.rs`, `rub_ointment.rs`, `drink.rs` and `strength_room.rs`, client `abilities/item_look.rs` (`can_drink`), `item_art` (`ointment.rs`), `item_popup` (`ointment.rs`).

## Notes

- Order (decisions 9 to 13): per herb (potency, cap), added per stat, oil's row, stat cap, Herbalism's cuts; then brewing applies the water before the threshold (decision 14). The stat cap isn't applied again after the water.
- Claude's choices where the plan left a gap: a herb's steps round to the nearest (so 1 × 80% still gives 1); a tie between a helpful and a harmful largest dose counts as helpful; Salt Water's and Rain Water's ties take the first in `Stat`'s order; the oil's boost and the water's work on a dose's size, keeping its sign.
- Untrained brewers: one step a herb, so a potion (a dose of 4) needs four herbs on one stat, or three with its oil's row boost.
- Since Alchemy replaced Grind and Brew (Alchemy plan PR 3), nothing remembers new recipes or calls `learn_from_potion` (Alchemy teaches one effect a success, `alchemy::lesson`); `Sheet::recipes` stays for saves, for the Alchemy panel to use or drop.
