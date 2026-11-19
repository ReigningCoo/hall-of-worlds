# shared: alchemy

The Alchemy rules (Alchemy plan PR 2, `docs/context/alchemy.md`): a process of apparatus steps, its chance, what it makes (doses in hundredths at full amount between steps, diluted only at the end), its least quality, the server's rolls, what the preview shows and what a success teaches. Pure functions on whole numbers, so the client's preview matches the server. Not the panel, the server's side or the Paste item (PR 3 and 4).

## Files

- `crates/shared/src/alchemy.rs`: the module list and re-exports. Tests in `alchemy/tests.rs`.
- `alchemy/apparatus.rs`: `Apparatus` (Mortar, Retort, Alembic, Crucible, Filter, Pelican; `ALL`, `name`, `sub_skill`, `index`) and the sub-skills' paths (`GRINDING` ... `CIRCULATING`).
- `alchemy/skills.rs`: `AlchemySkills { brew, sub }` (`of`, `alchemy_percent`, `sub_root`).
- `alchemy/chance.rs`: `CERTAIN`, `square_root_up`, `steps_allowed`, `chance`, `succeeds`.
- `alchemy/mix.rs`: `Mix` (each stat in hundredths: `of`, `boost_row`, `boost_way`, `remove_strongest`, `diluted`), `boosted`, `HUNDREDTHS`.
- `alchemy/process.rs`: `Process { source, steps }`, `Step`, `Solvent`, `Stored`, `Form`, `Refused`, `RETORT_UNITS`.
- `alchemy/mortar.rs`: the Mortar step, `Contribution`.
- `alchemy/retort.rs`: the Retort step.
- `alchemy/plan.rs`: `plan`, `Planned`, `Product`.
- `alchemy/quality.rs`: `skill_steps`, `least_quality`, `moved`, `luck_steps`, `LUCKY_ONCE`, `LUCKY_TWICE`.
- `alchemy/outcome.rs`: `outcome`, `Outcome`.
- `alchemy/learn.rs`: `preview`, `Seen`, `lesson`, `Lesson`.

## Public API

- `plan(process, skills) -> Result<Planned { product, chance, quality, practised, contributions }, Refused>`: every step in order. `Product::Nothing`, `Mixture { form, contents }` (a Mortar last: Paste dry, Ointment with an oil) or `Potion { contents, bottles }` (liquid in it: diluted over its 100 ml units, a bottle each, a potion if a dose reaches `brewing::POTION_AT`, keeping `KEPT_FROM` and up).
- Steps: `Mortar { herbs, solvent }` (first only, and not after a stored source; `brewing::ground_doses`, so brewing's values; `Solvent::Dry` a Paste, `Oil` an Ointment, `Alcohol` refused until PR 5); `Retort { liquids }` (1 to `RETORT_UNITS` 100 ml units, each applied in order at full amount: waters as `brewing::watered`, oils their row's boost; other liquids refused); `Alembic`, `Crucible`, `Filter`, `Pelican` refused `NotBuilt` until their PRs. `Stored { form, contents }` as a source: `Form::is_old` (Paste, Ointment; never Powder).
- `chance(alchemy%, steps, extra_kinds)`: hundredths of a percent, (100% + Alchemy%) halved per step over `steps_allowed` (⌈√Alchemy%⌉, at least 1) and per herb kind over `brewing::most_kinds`; `succeeds(chance, roll)`.
- `least_quality(purity, chance, skill_steps, old)`: `brewing::quality` of the product, plus `skill_steps` (steps × Σ∛ of the distinct sub-skills used ÷ (their count × 10), rounded down once) only when the chance is at least `CERTAIN`, less 1 when old; Poor to Legendary. `outcome(planned, chance_roll, luck_roll) -> Outcome::{Failed, Made { product, quality }}`, luck +1 at 1%, +2 at 0.1% (`luck_steps`).
- `Planned::practised`: each step's apparatus, but a Retort holding no mixture (liquids alone) gives no Infusing.
- `preview(planned, known) -> Vec<Seen>`: each product dose `Known` only when every herb that gave to its stat has that effect known. `lesson(planned, known) -> Option<Lesson { herb, effect }>`: the strongest product dose with something unknown, on the unknown herb that gave most towards it.

## Depends on

- `brewing` (`GroundHerb`, `BrewSkills`, `ground_doses`, `herb_steps`, `most_kinds`, `stat_row`, `oil_suits`, `quality`, `POTION_AT`, `KEPT_FROM`, `square_root`), `herb` (`Stat`, `HerbKnowledge`, `cube_root`, `skill_value`, `skill_percent`), `potion` (`Contents`, `Dose`, `Liquid`, `Oil`, `Water`), `item` (`Quality`), `skill` (`Skills`), `sheet` (`Row`).

## Used by

- Nothing yet: the server (PR 3) and the Alchemy panel (PR 4).

## Notes

- Dilution: each stat's total in hundredths ÷ (100 × units), rounded once to the nearest whole step, halves away from 0. Whole doses stand for one 100 ml bottle, so an ointment and one water make one bottle at full strength (as Brew did), and more liquid spreads the same total over more bottles.
- Boosts (Moon and Cave Water, oils) are an exact quarter in hundredths, at least a whole step, so a Retort can come out up to half a step stronger than Brew's whole steps did; Plain, Salt and Rain Water match Brew exactly (tested).
- Each 100 ml unit applies its liquid once: two Moon Waters boost twice, then dilute over two units.
- Claude's choices: the sub-skills' mean counts each apparatus once; an empty Mixture is `Nothing`; learning picks only among herbs whose effect is still unknown, weighing what each gave towards the dose's way (opposite effects count against).
