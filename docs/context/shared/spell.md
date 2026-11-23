# shared: spell

Designed spells' rules (magic plan PR 2, `docs/context/magic-system.md`): a spell's steps as data, checking a design against the caster's skills, its Mana cost, Void's reversal, the combination table, when it is cast automatically, and its automatic name, and what each where step hits (PR 3). Pure rules both sides use, the spellbook (PR 4) on the sheet, casting's timing, Heal's amount, spells' keys for the action bar and the rule that spells never kill the caster or friends (PR 5), missiles' flight (PR 6), and how wheres look for drawing (PR 7); the casting itself is the server's (`connections/cast.rs`). The Heal ability is gone: the Heal spell took its place.

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
- `spell/hit.rs`: `Side` (Caster, Friend, Enemy, Neutral: `allowed_by(filters)`: Friend and Enemy limit hits to theirs, Self limits nothing, the caster is never a friend; `same_as`), `Creature { body: combat::Body, side }`, `Origin { at, height, heading, creature }` (`from_creature`), `Hit { creature, at, height, heading }` (`origin()`: the next step's), `hit_anyone`.
- `spell/toward.rs`: `Toward` (Creature, Spot, Ahead): what a where is aimed at.
- `spell/line.rs`: crate-private line helpers: `ahead`, `wall_stop` (halving its way to the last clear spot), `entry_along` (where a line first meets a circle).
- `spell/book.rs`: `Spellbook` (`Sheet::spellbook`), `BookSpell { spell, name, favourite }`, `BookRefusal`, `MOST_SPELLS` (100), `LONGEST_NAME` (40), `heal_spell`, `tidy_name`.
- `spell/key.rs`: `SpellKey` (`of(spell)`: FNV-1a over the design's postcard bytes, the same everywhere), for the action bar's `BarThing::Spell` and `SpellbookAsk::Cast`.
- `spell/cast.rs`: `CAST_COST` (Heal's old 4200), `CAST_WIND_UP` (half), `STEP_MILLIS` (1000: steps a second apart), `heal_restores(count, skills)` (`magic::heal_amount` each).
- `spell/spare.rs`: `spared_harm(harm, health, side)`: harm to the caster and friends stops at 1 Health (for the harmful effects, PRs 10 to 12).
- `spell/aim.rs`: crate-private `aim_first`: what a first where with filters is aimed at (owner, 2026-11-23): Self the caster, whatever is targeted; Friend or Enemy the target if theirs, else the nearest of theirs in the where's reach with a clear line, or nothing (a fizzle); Surround stays round its centre.
- `spell/flight.rs` (PR 6): `Path` (a missile's way: from, to, the ground's height, heading, length, count and filters; serde, sent in `MissileNews`), `launch(level, step, origin, toward, creatures)` (the first where's aim by `aim_first`, then the way to the spot aimed at or ahead, stopped by walls; `None` a fizzle), `Path::speed`, `millis`, `along_after(millis)`, `at(along)`, `meets((start, end), creatures, except)` (the first allowed creature whose circle that stretch enters), `hit`, `end`: the server flies missiles a stretch a tick by these, so those stepping into or out of the way are hit or missed; `missile` is the same flight all at once.
- `spell/look.rs` (PR 7): `Look` (up to `LOOK_PARTS`, 4, distinct parts, first written first): `from_step(spell, index)` every effect after a where, `of_effects(step)` one step's; sent for clients to colour wheres by.
- `spell/shown.rs` (PR 7): `Shown` (a where as drawn: where it started after its filters' aim, the way it faced, its reach for Touch and Surround, Beam's ends (`beam.rs`'s crate-private `beam_ends`), the spots hit, its look), `shown(level, (step, look), origin, toward, creatures, hits)`, `None` on a filtered fizzle; sent in `protocol::SpellNews`.
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
  - `missile`: to the spot aimed at (at most `MISSILE_RANGE`, 8 cells; ahead that far with none), stopped by the first wall and the first creature its filters allow whose circle it meets (passing through the rest): a `Flight { hit, length }`; `missile_speed` (16 cells a second, half again a count), `flight_millis`. In play the server flies it a stretch at a time (`flight.rs`).
  - `surround`: anyone whose circle reaches within `surround_reach` (the centre's radius + a cell a count); nobody: `SURROUND_SPOTS` (8) spots on the ring facing out (Surround → Missile's eight missiles).
  - `beam`: `beam_headings` (the middle, then one either side `BEAM_SPREAD`, 22.5°, apart a count after the first), each as long as the way to the spot aimed at (at most `BEAM_RANGE`, 8 cells), stopped by walls; everyone allowed it meets, once; nobody: each beam's end.
  - `point`: the creature aimed at if allowed and in `POINT_RANGE` (8 cells, 3D), else the nearest allowed; then the nearest to it on its side (`same_as`) up to the count. Self adds the caster ("you and") and picks the count from friends: Self + Point ×2 is you and two friends, Friend + Point ×2 two friends. Unfiltered at the caster, the caster. Nobody: no hit at all.
  - A hit faces away from its origin (the origin's heading when on the same spot); a step that met nobody gives spots instead (`hit_anyone` false), which is a first step's fizzle.
- `Spellbook` (like `recipe::RecipeBook`): spells known by their design, never twice. `Default` (a new character's, and an older save's, as `Sheet::spellbook` is `serde(default)`) holds Heal (`heal_spell`, one Heal on the caster) then Healing Missile (`healing_missile`, Missile → Heal; usable once Missile is unlocked); `empty`. `store(spell, name, skills)` (checked by `check`, then the name, then `Known`, then `Full` at 100: never dropping one; to the front), `rename(spell, name)`, `set_favourite(spell, on)`, `remove(spell) -> BookSpell` (for the design panel to hold), `knows`, `get`, `by_key(SpellKey)`, `iter` (favourites first, then the rest, each latest stored first), `len`, `is_empty`. Names: `tidy_name` trims, empty is `None` (the automatic name, worded by the client with `auto_name`), more than 40 characters or control characters `BadName`. The server's `connections/spellbook.rs` answers `SpellbookAsk`.
- `auto_name`: the first effect step's first outcome, shaped by the where just before it: no where `Heal`, `Healing Missile`, `Ball of Fire`, `Chilling Touch`, `Scalding Beam`; nothing `Spell`.

## Depends on

- `skill` (`SkillId`, `Skills`, `VALUE_SCALE`), `vitals` (`Points`, `PER_POINT`), `lang` (`Lang`, `Args`), `combat` (`Body`, `distance`, `reach_between`, `arm_reach`), `dungeon` (`Level`, `spots_are_clear`), `facing` (`Heading`), `spot`, `creature` (`Size`), `serde`.

## Used by

- `sheet` (`Sheet::spellbook`), `ability` (`BarThing::Spell`, `BarThing::can_be_automatic` by `auto_cast`), `protocol` (`SpellbookAsk`, `SpellbookAsk::Cast`), the server's `connections/spellbook.rs` and `connections/cast/` (casting, through `where_hits`), the client's `abilities` (`spells.rs`: looks, `auto_name`, `auto_cast`, its own `where_hits` to tell when an automatic harmful spell would hit); the client's `spell_design` (the design panel: `check`, costs, `unlocked_wheres`, `auto_name`, `Spellbook::store` on a copy, `tidy_name`).

## Notes

- Integers only; skills are read as values in hundredths of a percent, so costs fall smoothly between the plan's whole-percent rows.
- A spell ending on a where is refused (`WhereAtEnd`): it would hit things and do nothing. Not in the plan's wording; easy to drop.
- Aims are first guesses (Calm counts as harmful, since it is resisted and aimed at enemies; neutral ones such as Water and Light keep a spell to casting by hand).
- Words in `spell.ftl` are first guesses, for the owner to change; a test checks every part and mixture has a noun and `.adjective`.
- Where hits' ranges, arcs, spreads and speeds are first guesses, to balance by playing. Self on Point meaning "you and" friends is read from the plan's two examples (Self + Point ×2, Friend + Point ×2). Filters choose the first where's aim (owner, 2026-11-23, replacing PR 3's reading that Self limited hits to the caster): Self + Surround is a ring round you whoever is targeted; Friend or Enemy with nobody of theirs in reach fizzles.
- Hits work in 2D along lines and in 3D (`combat::distance`) for reach and rings; spots with nobody on them keep the origin's height, for the caller to put on the ground.
