# shared: brewing

The brewing rules (brewing plan PR 4, owner's decisions 6 to 16 and 18 in `docs/context/brewing.md`): herbs and an oil ground into an ointment, an ointment and a water brewed into a potion, a potion's quality and how long it lasts, and an ointment used on oneself. Pure functions on whole numbers, so client and server agree. Not the abilities, items or panels that use them (Grind and Brew, PRs 6 and 7), nor drinking (PR 8), nor practice.

## Files

- `crates/shared/src/brewing.rs`: the module list and re-exports. Tests in `brewing/tests.rs` (grinding) and `brewing/brew_tests.rs` (waters, brewing, quality, durations, on the skin).
- `brewing/skill.rs`: `BrewSkills`, `square_root`, and the limits from skills: `per_herb_cap`, `stat_cap`, `most_kinds`, `most_doses`, `cut_steps`.
- `brewing/boost.rs`: `boosted` (+25%, at least 1), `stat_row`, `oil_suits`.
- `brewing/cut.rs`: `cut_unwanted` (Herbalism's cuts).
- `brewing/grind.rs`: `GroundHerb`, `Ointment`, `Refused`, `herb_steps`, `grind`.
- `brewing/brew.rs`: `watered`, `brew`, `POTION_AT`, `KEPT_FROM`.
- `brewing/quality.rs`: `quality`, `duration_seconds`, `duration_ticks`, `on_skin` and their constants.

## Public API

- `BrewSkills { alchemy, herbalism }` (hundredths of a percent, as `herb::skill_value`): `of(skills)`, `alchemy_percent`, `herbalism_percent`.
- Limits: `per_herb_cap` (√Alchemy% + 1, 1 to 11 steps an effect a herb), `stat_cap` (4 × that, 4 to 44), `most_kinds` (4 + ∛Alchemy%, 4 to 8), `most_doses` (∛Alchemy% + 1, 1 to 5), `cut_steps` (∛Herbalism%, 0 to 4). `square_root` (whole, rounded down).
- `herb_steps(effect, quality, skills)`: one herb's steps on one effect: `change × herb::potency`, rounded to the nearest, capped.
- `grind(herbs: &[GroundHerb { def, quality }], oil, doses, skills) -> Result<Ointment { contents, doses }, Refused>`: refuses no herbs, the same herb twice (by key, whatever the quality), more kinds than `most_kinds`, doses outside 1 to `most_doses`. Steps added per stat (opposites cancel), the oil's row `boosted`, each stat clamped to `stat_cap`, then `cut_unwanted`. Contents can be empty. The caller checks `doses` of each herb and the oil are carried and takes them.
- `cut_unwanted(&mut [(Stat, i32)], steps)`: the wanted way is the largest dose's sign; steps off the others one at a time, round robin from the largest.
- `boosted(step)`, `stat_row(stat)` (Health Body, Mana Mind), `oil_suits(oil)` (Vegetable Body, Nut Mind, Seed Spirit).
- `watered(ointment, water)`: Cave Water boosts harmful doses, Moon Water helpful ones, Salt Water takes the strongest helpful dose, Water nothing. `brew(ointment, water) -> Option<Contents>`: after the water, a potion if any dose reaches `POTION_AT` (4), keeping doses of `KEPT_FROM` (2) or more; `None` otherwise (ingredients still used, practice still given: the caller's).
- `quality(contents)`: (strongest − next strongest, by size) ÷ 3: ≤2 Poor, 3–4 Common, 5–6 Uncommon, 7–8 Rare, 9–10 Epic, 11+ Legendary; for ointments too.
- `duration_seconds(quality)` / `duration_ticks`: `COMMON_SECONDS` (120) ± `SECONDS_PER_QUALITY` (30) a quality from Common: Poor 90 s to Legendary 240 s.
- `on_skin(ointment)`: each dose ÷ `ON_SKIN_SHARE` (4), towards 0, zeros dropped; it lasts as `duration_*` of the ointment's own `quality`.

## Depends on

- `herb` (`HerbDef`, `Effect`, `Stat`, `potency`, `cube_root`, `skill_value`, `skill_percent`, `HERBALISM`), `potion` (`Contents`, `Dose`, `Oil`, `Water`), `ability` (`ALCHEMY`), `sheet` (`Row`), `item` (`Quality`), `energy` (`ACTION_TICKS_PER_SECOND`), `skill` (`Skills`).

## Used by

- Nothing yet: Grind (PR 6), Brew (PR 7) and drinking (PR 8) will.

## Notes

- Order (decisions 9 to 13): per herb (potency, cap), added per stat, oil's row, stat cap, Herbalism's cuts; then brewing applies the water before the threshold (decision 14). The stat cap isn't applied again after the water.
- Claude's choices where the plan left a gap: a herb's steps round to the nearest (so 1 × 80% still gives 1); a tie between a helpful and a harmful largest dose counts as helpful; Salt Water's tie takes the first in `Stat`'s order; the oil's boost and the water's work on a dose's size, keeping its sign.
- Untrained brewers: one step a herb, so a potion (a dose of 4) needs four herbs on one stat, or three with its oil's row boost.
