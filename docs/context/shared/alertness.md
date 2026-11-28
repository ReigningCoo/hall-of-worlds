# shared: alertness

Alertness (mob behaviour plan PR 7): whether a creature has noticed anyone. Unaware (nothing seen or heard), Alert (heard something, turning to it once its reaction is over), Aware (sees someone). The server keeps each mob's; a blow on one caught off guard is a critical (`combat::Critical`). Not what a mob does about what it notices (the server's `entities/choice`).

## Files

- `crates/shared/src/alertness.rs`: `Alertness` (Unaware, Alert, Aware), `reaction_ticks(attributes)` and its constants.
- `alertness/wariness.rs`: `Wariness`, one creature's alertness from tick to tick.

## Public API

- `reaction_ticks`: `REACTION_TICKS` (30, 1.5 s) at a Mind row (Intellect, Perception, Will) averaging 100, × 300 ÷ the row's sum, between `QUICKEST_TICKS` (10) and `SLOWEST_TICKS` (60).
- `Wariness` (`Default` Unaware): `heard(at, by, tick, reaction)` (Alert unless Aware, `about` the maker if known, a turn to the first noise due `reaction` ticks on), `sees(who, tick)` (`Some`: Aware of them, the turn dropped; `None`: Aware becomes Alert, Alert with no turn left settles to Unaware `ALERT_TICKS` (10 s) after the last noise or sight), `turn_due(tick)` (where to turn, once), `stage`, `is_unaware`, `off_guard` (Unaware, or Alert with its turn still to come: someone heard creeping up gets that moment), `alert_to` (who it is Alert to), `about` (who it is Alert to or Aware of).

## Depends on

- `sheet` (`Attributes`, `Row`), `energy::ACTION_TICKS_PER_SECOND`, `id`, `spot`.

## Used by

- server: `entities/alert.rs` (each `Mob::wariness`), `entities/noise.rs` (hearing), `fight.rs` and `mob_blows.rs` (criticals); `combat::critical` names it.

## Notes

- Numbers are Claude's first guesses, to check in play.
- One stage a creature, not one per creature it might notice: Aware of anyone means no critical from anyone.
