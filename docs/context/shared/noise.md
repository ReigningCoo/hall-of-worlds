# shared: noise

Noise (mob behaviour plan PR 6): a sound at a place with a loudness, muffled by walls and closed doors between, heard by a listener's hearing and distance with a hashed roll. Both sides use it: the server for mobs hearing, the client for the sneaking eye. Not what mobs do about a noise (the server's `entities`).

## Files

- `crates/shared/src/noise.rs`: `Source` (Footsteps, Blow, Spell, Door, DoorKicked, Call; `cells`: how far each carries for normal hearing in the open), `Noise { source, at, loudness }` (loudness in units, 240 a cell; `new`, `footsteps`, `call` by an `behaviour::Alarm`'s cells).
- `noise/footsteps.rs`: `Footsteps { sneaking, skill, armour_grams }` (`of(sneaking, skills, worn)`, `loudness`): `WALK_CELLS` 5, +1 cell a `ARMOUR_GRAMS_PER_CELL` (5 kg), sneaking × `SNEAK_WORST_PER_MILLE` (½) at 0% Sneaking to `SNEAK_BEST_PER_MILLE` (⅛) at 100%.
- `noise/muffle.rs`: `muffled(level, from, to, loudness)`: each wall cell crossed leaves `WALL_PER_MILLE` (⅓), each closed door `DOOR_PER_MILLE` (½); the end cells never count.
- `noise/hear.rs`: `hearing_per_mille` (None 0, Poor ½, Normal 1, Keen 1¾: a blow's 4, 8, 14 cells), `Hearer { at, hearing, seed }` (seed from the listener's `Id::seed`), `reach`, `chance` (per mille: certain within half the reach, down to nothing at its end), `roll` (0 to 999, a hash of the seed, tick, place and source), `heard`.

## Public API

- Everything above.

## Depends on

- `creature::Acuity`, `dungeon::Level`, `spot`, `rng::Seed`, `bonus::armour_grams`, `movement::MOVEMENT_SKILLS`, `skill::Skills`, `item::Item`.

## Used by

- server: `entities/noise.rs` (mobs hearing), `run_mobs` (footsteps), doors and casting (`make_noise`), `entities/curious.rs` (alarm calls, `Noise::call`).
- client: `sneak_eye` (how noticeable the player is).

## Notes

- Numbers are Claude's first guesses, to check in play.
- Footsteps are one noise a cell stepped into, so the roll comes once a step.
