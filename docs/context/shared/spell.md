# shared: spell

Designed spells' rules (magic plan PR 2, `docs/context/magic-system.md`): a spell's steps as data, checking a design against the caster's skills, its Mana cost, Void's reversal, the combination table, when it is cast automatically, and its automatic name, and what each where step hits (PR 3). Pure rules both sides use, and the spellbook (PR 4) on the sheet; not casting (the server, PR 5). Today's Heal ability is still `shared::magic`'s.

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
- `spell/hit.rs`: `Side` (Caster, Friend, Enemy, Neutral: `allowed_by(filters)`, `same_as`), `Creature { body: combat::Body, side }`, `Origin { at, height, heading, creature }` (`from_creature`), `Hit { creature, at, height, heading }` (`origin()`: the next step's), `hit_anyone`.
- `spell/toward.rs`: `Toward` (Creature, Spot, Ahead): what a where is aimed at.
- `spell/line.rs`: crate-private line helpers: `ahead`, `wall_stop` (halving its way to the last clear spot), `entry_along` (where a line first meets a circle).
- `spell/book.rs`: `Spellbook` (`Sheet::spellbook`), `BookSpell { spell, name, favourite }`, `BookRefusal`, `MOST_SPELLS` (100), `LONGEST_NAME` (40), `heal_spell`, `tidy_name`.
- `spell/touch.rs`, `missile.rs`, `surround.rs`, `beam.rs`, `point.rs`: one where each, with its first-guess constants; `spell/hits.rs`: `where_hits(level, step, origin, toward, creatures)`, by the step's where and filters.

## Public API

- The design types above (serde, for messages and saves later), `Spell::new`, `first_where`, `effect_steps`; `Step::at`, `filtered`, `effects`.
- `check`: refuses, first found, `Empty`, `TooManySteps`, `NoEffect`, `WhereAtEnd`, `EmptyStep`, `ZeroCount`, `FilterNotFirst`, `Locked(where)`. `unlocked_wheres`: ⌊∛(Magic% × 6)⌋, 0 to 5, Magic being the `magic` node's own value; no skills none.
- Costs in thousandths: an effect 5000 less ∛(its skill %) in thousandths, at least 1000; an effect step its effects × counts plus 1000 per unit after the first (Void costs as any); a where step (tier × count + count − 1) × 1000, less up to `FORM_SAVING` (half) by its own Form leaf, at least 1000; filters free; the spell its steps multiplied (capped at `u32::MAX`).
- `parts`: Void flips reversal for everything after it in its step (an odd count flips, even puts back); the same part written twice is added up, in the order first written.
- `outcomes`: each `COMBINATIONS` row takes as many of its pair as both have, mixtures first, then what is left. Only Fire + Water = Steam so far.
- `auto_cast`: every outcome helpful: `WhenLow` (first step's target at or below a quarter, `magic::needs_healing`); every one harmful: `AtEnemy`; anything else `ByHand`.
- Where hits (all integers, creatures by their place in the slice the caller passes, the origin's own never hit; every hit needs a clear line from the origin, `dungeon::spots_are_clear`; nearest first):
  - `touch`: within `combat::reach_between` (no weapon) in 3D and an arc of `touch_half_arc(count)` each side (45°, 22.5° wider a count, all round from ×7); nobody: the spot an arm's reach ahead.
  - `missile`: to the spot aimed at (at most `MISSILE_RANGE`, 8 cells; ahead that far with none), stopped by the first wall and the first creature its filters allow whose circle it meets (passing through the rest): a `Flight { hit, length }`; `missile_speed` (16 cells a second, half again a count), `flight_millis`.
  - `surround`: anyone whose circle reaches within `surround_reach` (the centre's radius + a cell a count); nobody: `SURROUND_SPOTS` (8) spots on the ring facing out (Surround → Missile's eight missiles).
  - `beam`: `beam_headings` (the middle, then one either side `BEAM_SPREAD`, 22.5°, apart a count after the first), each as long as the way to the spot aimed at (at most `BEAM_RANGE`, 8 cells), stopped by walls; everyone allowed it meets, once; nobody: each beam's end.
  - `point`: the creature aimed at if allowed and in `POINT_RANGE` (8 cells, 3D), else the nearest allowed; then the nearest to it on its side (`same_as`) up to the count. Self adds the caster ("you and") and picks the count from friends: Self + Point ×2 is you and two friends, Friend + Point ×2 two friends. Unfiltered at the caster, the caster. Nobody: no hit at all.
  - A hit faces away from its origin (the origin's heading when on the same spot); a step that met nobody gives spots instead (`hit_anyone` false), which is a first step's fizzle.
- `Spellbook` (like `recipe::RecipeBook`): spells known by their design, never twice. `Default` (a new character's, and an older save's, as `Sheet::spellbook` is `serde(default)`) holds Heal (`heal_spell`, one Heal on the caster); `empty`. `store(spell, name, skills)` (checked by `check`, then the name, then `Known`, then `Full` at 100: never dropping one; to the front), `rename(spell, name)`, `set_favourite(spell, on)`, `remove(spell) -> BookSpell` (for the design panel to hold), `knows`, `get`, `iter` (favourites first, then the rest, each latest stored first), `len`, `is_empty`. Names: `tidy_name` trims, empty is `None` (the automatic name, worded by the client with `auto_name`), more than 40 characters or control characters `BadName`. The server's `connections/spellbook.rs` answers `SpellbookAsk`.
- `auto_name`: the first effect step's first outcome, shaped by the where just before it: no where `Heal`, `Healing Missile`, `Ring of Fire`, `Chilling Touch`, `Scalding Beam`; nothing `Spell`.

## Depends on

- `skill` (`SkillId`, `Skills`, `VALUE_SCALE`), `vitals` (`Points`, `PER_POINT`), `lang` (`Lang`, `Args`), `combat` (`Body`, `distance`, `reach_between`, `arm_reach`), `dungeon` (`Level`, `spots_are_clear`), `facing` (`Heading`), `spot`, `creature` (`Size`), `serde`.

## Used by

- `sheet` (`Sheet::spellbook`), `protocol` (`SpellbookAsk`), the server's `connections/spellbook.rs`; to come: the server's casting (PR 5, which also calls the where hits) and the design panel (PR 9).

## Notes

- Integers only; skills are read as values in hundredths of a percent, so costs fall smoothly between the plan's whole-percent rows.
- A spell ending on a where is refused (`WhereAtEnd`): it would hit things and do nothing. Not in the plan's wording; easy to drop.
- Aims are first guesses (Calm counts as harmful, since it is resisted and aimed at enemies; neutral ones such as Water and Light keep a spell to casting by hand).
- Words in `spell.ftl` are first guesses, for the owner to change; a test checks every part and mixture has a noun and `.adjective`.
- Where hits' ranges, arcs, spreads and speeds are first guesses, to balance by playing. Self on Point meaning "you and" friends is read from the plan's two examples (Self + Point ×2, Friend + Point ×2); on the other wheres Self only allows the caster, whom they never hit, so it fizzles alone.
- Hits work in 2D along lines and in 3D (`combat::distance`) for reach and rings; spots with nobody on them keep the origin's height, for the caller to put on the ground.
