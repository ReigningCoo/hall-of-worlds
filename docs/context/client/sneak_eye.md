# client: sneak_eye

The sneaking eye (mob behaviour plan PR 6): only while Sneak is on and on a level, an eye over the player's token, closed to open by how noticeable they are, wide (and reddened) while a mob is Alert to or Aware of them. No markers over mobs (owner: they would be confused with quest markers).

## Files

- `crates/client/src/sneak_eye/mod.rs`: `SneakEyePlugin` (`take_alert` in `Update`, `show_eye` in `PostUpdate` after transforms; inits `common::MobAlert`).
- `sneak_eye/alert.rs`: `take_alert`: `common::MobAlert` from `ServerMessage::MobAlert` (mob behaviour plan PR 7), off on `Welcome`.
- `sneak_eye/noticeable.rs`: `noticeable(level, at, steps, mobs)`: the best chance (per mille, `shared::noise::chance`) any mob in sight has of hearing the player's next step.
- `sneak_eye/show.rs`: `show_eye` (the player's `Footsteps::of` from `character::CharacterSheet` and `inventory::Carried`, the mobs among `common::Other` with their `SpotPosition`, the level from `common::CurrentDungeon`; a UI almond projected over the token through `view3d::ViewCamera`, `ABOVE` its middle), `look` (height and pupil by openness, or wide for Alert).

## Public API

- `SneakEyePlugin`.

## Depends on

- `common` (`Sneaking`, `MobAlert`, `Other`, `SpotPosition`, `CurrentDungeon`, `GameView`), `character` (`CharacterSheet`), `inventory` (`Carried`), `player` (`Player`), `view3d` (`TokenView`, `ViewCamera`, `TOKEN_DIAMETER`), `shared::noise`.

## Notes

- It shows the chance as if the player stepped now, standing still too, so it doesn't flicker between steps.
- The server says `MobAlert` while a mob is Alert to (heard their steps) or Aware of (sees) the player.
