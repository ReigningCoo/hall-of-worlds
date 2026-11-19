# shared: alcohol

Drinking alcohol (drinks plan PR 7, `docs/context/drinks.md` decisions 21 to 25): units of alcohol on the sheet, worn off over real time by body mass; the stages they bring, kept as one effect; how much each slows; the odd stumble, from a hash both sides work out; the Tolerance skill. Not the drinks themselves (`shared::fermentation`) nor drinking's Mana (`shared::juice::drink_effect`).

## Files

- `crates/shared/src/alcohol.rs`: the module list and re-exports. Tests in `alcohol/tests.rs`.
- `alcohol/drinker.rs`: `Drinker { grams, tolerance }` (`of(sheet, kind)`, `new`, `threshold`, `worn_off`, `ticks_to_wear`), `TOLERANCE` (`survival/tolerance`), `AVERAGE_GRAMS` (70 kg), `UNIT_TICKS` (10 real minutes).
- `alcohol/state.rs`: `Alcohol { milli, at, peak, collapsed_until }` (`milli_at`, `sober_at`, `drink`, `collapsed`, `by_units`).
- `alcohol/stage.rs`: `Stage::{Tipsy, Drunk, VeryDrunk, Collapsed, Poisoned, Hangover}` (`speed_percent`, `stumble_one_in`, `bonuses`, `by_units`), the thresholds (`TIPSY_MILLI` 3, `DRUNK_MILLI` 7, `VERY_DRUNK_MILLI` 12, `POISONED_MILLI` 20 units), `COLLAPSE_TICKS` (a minute), `HANGOVER_TICKS` (half an hour); `Alcohol::stage(now, drinker)`, `next_change(now, drinker)`.
- `alcohol/effect.rs`: `stage_effect`; on `Sheet`: `alcohol_stage`, `drink_alcohol`, `keep_alcohol`.
- `alcohol/drinking.rs`: `units_in(liquid, ml)`, `tolerance_practice`, `TOLERANCE_PRACTICE_PER_UNIT`.
- `alcohol/stumble.rs`: `Veer { clockwise, eighths }` (`step`, `turned`), `stumble(character, map, number, stage)`, `veered_move(level, size, gait, from, to, veer)`, `veered_step`.

## Public API

- Everything above. Units are thousandths (`milli`); 1 unit is 10 ml of pure alcohol, so 250 ml of 5% ale is 1.25 units (`units_in`: ml × strength in tenths of a percent ÷ 10).
- `Sheet::alcohol` (saved, `serde(default)`) holds `Alcohol`; `Sheet::drink_alcohol(milli, kind, situation, now)` takes units in, `Sheet::keep_alcohol(kind, situation, now)` swaps the stage's effect when the stage changes (whether it did), `Sheet::alcohol_stage()` is the stage its effects say (what `Sheet::speed` and stumbles go by).

## Depends on

- `sheet` (`Sheet`, `Situation`, attributes), `bonus` (`Effect`, `Source::Alcohol`, `Bonus`), `creature` (`CreatureKind::def().weight`), `skill`, `movement` (`FULL`, `Step`, `Gait`, `move_on_level`), `dungeon`, `spot`, `id` (`Id::seed`), `rng`, `potion` (`Liquid::Drink`), `energy`, `vitals`, `serde`.

## Used by

- `sheet` (`Sheet::alcohol`, `Sheet::speed`), `bonus` (`Source::Alcohol(Stage)`), server `connections/alcohol.rs` (drinking, keeping the stage, collapsing), `connections/stumble.rs`, client `connection/stumble.rs` (predicting stumbles), `character/collapse.rs`, `effect_bar` (names and icons by stage).

## Notes

- Stored like `energy`: a value and the action tick it was right at, so it wears off by the server's action clock (offline on a networked server too; single player's clock stops while the game is closed or paused). 1 unit per `UNIT_TICKS` at 70 kg, faster in proportion to the body's mass (the body it is in now); the thresholds grow in proportion to the mass, and up to twice as high at 100% Tolerance.
- Stages (Claude's first guesses for the owner to tune): tipsy from 3 units (67% speed), drunk from 7 (34%, Intellect, Perception and Will −20%, one move in 10 stumbles), very drunk from 12 (34%, the Mind row −40% and Agility −20%, one move in 5 stumbles), poisoned from 20 (as very drunk, draining 1 Health a second, never below 1, as poison). Crossing into very drunk or poisoned by drinking collapses for `COLLAPSE_TICKS` (`Collapsed`: as very drunk, unable to act). Having been drunk, a hangover (Mind row −10%) runs from no longer being tipsy until `HANGOVER_TICKS` after the last unit wears off. Drinking while below tipsy forgets what was drunk before for the hangover. Speed stacks fully with load (`Sheet::speed`).
- The stage is one effect, `Source::Alcohol(stage)`, ending at `next_change` (the first tick its stage differs): the effect bar shows it with its time left, and its penalties count like any bonus. Whoever keeps the sheet calls `keep_alcohol` (the server every tick); a client goes by the effect it was last sent.
- Stumbles: `stumble` hashes the character's id, the map count and the move's number (`ClientMessage::Move` / `Step`), so the client predicts them; a level move turns an eighth or a quarter of a turn to one side, its length kept, then half as long if that is in the way (`move_on_level`, the rule moves go by), else nowhere; a world map step turns an eighth or a quarter, refused like any step if it can't be taken. Not in the Hall. The server veers whatever move it is sent, so a client can't skip them.
- No sway or other motion is drawn, so Reduce Motion has nothing to calm yet.
