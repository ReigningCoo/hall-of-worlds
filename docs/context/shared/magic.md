# shared: magic

Heal's numbers, kept for the Heal effect of designed spells (`shared::spell`, magic plan PR 5: the Heal ability is gone): how much Health one Heal gives by the Healing skill, and when an automatic heal is due. `HEAL_MANA`, `HEAL_RANGE` and `heal_reaches` are left from the ability, unused by casting (a spell's Mana is `spell_cost`, its reach its wheres'). Plan: `docs/context/magic-system.md`.

## Files

- `crates/shared/src/magic.rs`: re-exports.
- `crates/shared/src/magic/heal.rs`: Heal's numbers and rules.

## Public API

- `HEAL_SKILL` (`magic/divine/healing`), `heal_skill()`.
- `HEAL_MANA` (6, always spent, even on someone whole: what is beyond the most is lost; owner, 2026-11-07), `heal_ratio(skills)` (Health per point of Mana, per mille: `RATIO_AT_FIRST` 2000 to `RATIO_AT_BEST` 20000 in a straight line with Healing's `combat::skill_percent`; owner), `heal_amount(skills)`.
- `HEAL_COST` (4200: a second at normal speed), `HEAL_WIND_UP` (half), `HEAL_RANGE` (1440 units: 6 cells, between middles in 3D), `heal_reaches(level, from, to)` (`combat::Body`s: themselves, at the same spot, or `combat::within` the range with a clear line).
- `AUTO_HEAL_BELOW` (250 per mille: a quarter), `needs_healing(health, most)`: when an automatic Heal is cast.
- `PRACTICE_PER_CAST` (8, as a blow's).

## Depends on

- `combat` (`Body`, `within`, `skill_percent`), `skill` (`SkillId`, `Skills`), `dungeon` (`Level`), `vitals` (`Points`, `PER_POINT`).

## Used by

- `spell` (`heal_restores`, `CAST_COST`), server `connections/fungus_harm.rs` (`heal_skill`) and `connections/cast/`, client `abilities` (`vitals_now.rs`).

## Notes

- Integers only. The amount is worked out by the skill before the cast's own practice counts.
- First guesses (owner: balance by playing). A creature without skills (mobs) counts as none.
