# client: sound

Whether this run plays sound at all, and the volumes chosen in Settings, Audio, put into play. Plays nothing itself: `music` (the synth, music plan PR 2) and the sequencer to come read what is here (`docs/context/research-music.md`). Sound is client-only, never game state.

## Files

- `crates/client/src/sound/mod.rs`: `SoundPlugin`: inserts `common::SoundSwitches` from the environment, `MusicVolume`, and runs `apply_volumes` in `Update`.
- `crates/client/src/sound/switches.rs`: `from_environment`: `HOW_SOUND` (off by default in debug builds, on in releases: `SOUND_BY_DEFAULT`) and `HOW_MUSIC` (on by default), read with `server::option_on` natively; a browser has the defaults.
- `crates/client/src/sound/volume.rs`: `MusicVolume`, `gain` (a percentage squared, so the middle sounds about half as loud), `gains` (general and music, 0 while muted or switched off), `apply_volumes` (Bevy's `GlobalVolume` from General, `MusicVolume` from Music, when `Preferences` or the switches change).

## Public API

- `SoundPlugin`.
- `MusicVolume(f32)` (resource): the linear gain music plays at on top of `GlobalVolume`; 0 while Music is muted or `HOW_MUSIC` / `HOW_SOUND` is off, so whatever plays music can stop then.
- In `common` (`common/audio.rs`): `AudioPrefs` (`Preferences::audio`: `general`, `music` as percentages, 100 and 60 at first, each with a `_muted` flag) and `SoundSwitches { sound, music }` (resource; `music_on()`), so `system_menu` reads them without depending on this module.

## Depends on

- `common`: `Preferences`, `AudioPrefs`, `SoundSwitches`. `server::option_on` (native).

## Used by

- `system_menu` (Settings, Audio) through `common`'s types; `music` reads `MusicVolume` and `SoundSwitches`.
