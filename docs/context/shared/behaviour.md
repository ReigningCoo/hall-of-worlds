# shared: behaviour

How each race of creature behaves, as data (mob behaviour plan PR 1; the plan: `docs/context/mob-behaviour.md`): drives, who it defends, its alarm call, what it hunts, how it flees, whether fences stop it, whether it fears open fire, what it wants from the ground, its temper, and bands with their variants and relations. Only data and rules: what mobs do with them is the server's (`entities`), from the plan's PR 2 on; nothing in play reads more than `Temper` yet.

## Files

- `crates/shared/src/behaviour.rs`: the module list and re-exports. Tests in `behaviour/tests.rs`.
- `behaviour/behaviours.ron`: one entry per race (all 40 `CreatureKind`s, a test checks), first guesses; read by `include_str!`.
- `behaviour/def.rs`: `BehaviourDef`, `Drives`, `Defends`, `Alarm`, `Flee`, `Fences`, `Desire`; `check`.
- `behaviour/temper.rs`: `Temper` (moved from `creature/senses.rs`) and `CreatureKind::temper`.
- `behaviour/band.rs`: `Band` (`of_site`), `Family`, `Member`, `BandVariant` (`common`, `rolled`).
- `behaviour/relation.rs`: `Relation`, `Behaviours::relation`.
- `behaviour/table.rs`: `Behaviours`.

## Public API

- `BehaviourDef { kind, temper, drives, defends, alarm, hunts, flee, fences, fears_fire, desires, other_bands, bands }`: `kind` is the race and its key (a mod's entry replaces the game's); everything after `drives` may be left out (`Defends::Band`, no alarm, no prey, `Straight`, `Through`, no fear of fire, no desires, `Relation::Neutral`, one "common" band). `check()` an error for a mod's author (a drive over 100, a call or variant key not lower case, digits and `_`, hunting its own race, no or weightless or repeated variants, a race in two of a variant's lists).
- `Drives { aggression, courage, curiosity, wander, cohesion, guarding, desire }`, 0 to 100 each (`named()`).
- `Defends::{Nobody, Band, Allies}`, `Alarm { call, loudness (cells for normal hearing in the open) }`, `Flee::{Straight, Zigzag}`, `Fences::{Through, Seldom, Never}`, `Desire::{Food, Coins, Weapons}`; `hunts` races taken as prey (wild only).
- `Temper::{Friendly, Neutral, Timid, Territorial, Hostile}`; `CreatureKind::temper()` reads the built-in table, so client and server agree whatever a server's mods say (re-exported as `creature::Temper`).
- `Band(u64)`: a band's number, shared by its members (`mob::Spawn::band`); `Band::of_site(seed, kind)`. `Family(u64)`: reserved, unused. `Member { kind, band }`.
- `BandVariant { key, weight, allies, rivals, enemies }`: a kind of band a race has; `BandVariant::rolled(variants, band)` picks by weight from the band's number, so nothing is saved.
- `Relation::{Kin, Ally, Neutral, Rival, Enemy}`; `Behaviours::relation(one, other)`: the same band `Kin`; then what `one`'s rolled variant lists the other's race as (enemies, rivals, allies); then another band of its own race its `other_bands`; else `Neutral`. From `one`'s side only.
- `Behaviours`: `built_in()`, `built_in_ref()`, `read(ron_text)` (each checked, a race twice refused), `merge(defs)` (replacing by race; how many replaced), `of(kind)` (falls back to the game's own), `iter`, `len`, `is_empty`.

## Depends on

- `creature` (`CreatureKind`), `rng` (`Seed`), `ron`, `serde`.

## Used by

- `creature` (`Temper`, `temper()`: `feeling_about`, `reaction`), `mob` (`Spawn::band`, `family`), server `behaviours.rs` (loading mods, `BehaviourTable`), and through `temper()` the server's `entities` (territory, timid, watching) and `quests/standing.rs`.

## Notes

- Never sent to clients: a server's mods are its own. The client only ever uses `temper()`, which is the game's own.
- Bands are worked out, never saved: a site's from its seed and the race, a wild pack's from its block's roll. Renaming a race changes its site bands' numbers (they hash the race's name).
