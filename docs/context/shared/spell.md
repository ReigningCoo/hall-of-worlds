# shared: spell

Designed spells' rules (magic plan PR 2, `docs/context/magic-system.md`): a spell's steps as data, checking a design against the caster's skills, its Mana cost, Void's reversal, the combination table, when it is cast automatically, and its automatic name. Pure rules both sides use; not where a step hits (PR 3), the spellbook (PR 4) or casting (the server, PR 5). Today's Heal ability is still `shared::magic`'s.

## Files

- `crates/shared/src/spell.rs`: `mod` lines and re-exports.
- `spell/design.rs`: `Spell` (steps), `Step` (`Where(WhereStep)` or `Effects(EffectStep)`), `WhereStep { place, count, filters }`, `EffectStep { effects: Vec<(Effect, u8)> }`, `MAX_STEPS` (10).
- `spell/place.rs`: `Where` (Touch, Missile, Surround, Beam, Point: `ALL` in unlock order, `tier` 1 to 5, `key`, `skill` under `magic/form`).
- `spell/effect.rs`: `Effect` (18, Void among them: `ALL`, `key`, `skill_path`, `skill`).
- `spell/filters.rs`: `Filters { caster, friend, enemy }` (Self is `caster`), `NONE`, `any`.
- `spell/part.rs`: `Part { effect, reversed }` (`key`: a reversed one's own, Fire reversed `cold`), `parts(step)`: Void read and left out.
- `spell/combination.rs`: `Mixture` (Steam), `Combination`, `COMBINATIONS`, `Outcome` (a part or a mixture), `outcomes(step)`.
- `spell/auto_cast.rs`: `Aim` (Helpful, Harmful, Neutral) per `Outcome::aim`, `AutoCast` (WhenLow, AtEnemy, ByHand), `auto_cast(spell)`.
- `spell/check.rs`: `DesignError`, `check(spell, skills)`, `unlocked_wheres(skills)`, `is_unlocked`.
- `spell/cost.rs`: `effect_cost`, `effect_step_cost`, `where_step_cost`, `step_cost`, `spell_cost` (a `vitals::Points`), `EFFECT_BASE`, `LEAST`, `FORM_SAVING`.
- `spell/name.rs`: `auto_name(spell, lang)`, worded by `lang/en/spell.ftl`.

## Public API

- The design types above (serde, for messages and saves later), `Spell::new`, `first_where`, `effect_steps`; `Step::at`, `filtered`, `effects`.
- `check`: refuses, first found, `Empty`, `TooManySteps`, `NoEffect`, `WhereAtEnd`, `EmptyStep`, `ZeroCount`, `FilterNotFirst`, `Locked(where)`. `unlocked_wheres`: ⌊∛(Magic% × 6)⌋, 0 to 5, Magic being the `magic` node's own value; no skills none.
- Costs in thousandths: an effect 5000 less ∛(its skill %) in thousandths, at least 1000; an effect step its effects × counts plus 1000 per unit after the first (Void costs as any); a where step (tier × count + count − 1) × 1000, less up to `FORM_SAVING` (half) by its own Form leaf, at least 1000; filters free; the spell its steps multiplied (capped at `u32::MAX`).
- `parts`: Void flips reversal for everything after it in its step (an odd count flips, even puts back); the same part written twice is added up, in the order first written.
- `outcomes`: each `COMBINATIONS` row takes as many of its pair as both have, mixtures first, then what is left. Only Fire + Water = Steam so far.
- `auto_cast`: every outcome helpful: `WhenLow` (first step's target at or below a quarter, `magic::needs_healing`); every one harmful: `AtEnemy`; anything else `ByHand`.
- `auto_name`: the first effect step's first outcome, shaped by the where just before it: no where `Heal`, `Healing Missile`, `Ring of Fire`, `Chilling Touch`, `Scalding Beam`; nothing `Spell`.

## Depends on

- `skill` (`SkillId`, `Skills`, `VALUE_SCALE`), `vitals` (`Points`, `PER_POINT`), `lang` (`Lang`, `Args`), `serde`.

## Used by

- Nothing yet: the spellbook (PR 4), the server's casting (PR 5) and the design panel (PR 9).

## Notes

- Integers only; skills are read as values in hundredths of a percent, so costs fall smoothly between the plan's whole-percent rows.
- A spell ending on a where is refused (`WhereAtEnd`): it would hit things and do nothing. Not in the plan's wording; easy to drop.
- Aims are first guesses (Calm counts as harmful, since it is resisted and aimed at enemies; neutral ones such as Water and Light keep a spell to casting by hand).
- Words in `spell.ftl` are first guesses, for the owner to change; a test checks every part and mixture has a noun and `.adjective`.
