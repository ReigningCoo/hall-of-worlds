# Research: adaptive medieval music

From the research chat (2026-11-25/26), with the owner's answers recorded; brought into `docs/context/` with the first code PR (Audio settings). Review page: https://claude.ai/artifact/JFgCE4RZofh1PEXaGSh9y9 (a playable model). The raw reports are kept with it in `~/.claude/research-tools/music/`.

## Owner's decisions so far

- MIDI-style music from the game's own sequencer, played by the client only (music is not game state).
- A theme set per context (wilderness, each poi kind, the Hall, The Way Inn, ...), with more themes where players stay longer.
- Changes seamlessly when the player is attacked: tempo and parts.
- Few instruments, and more in larger towns and cities.
- Melodies as close to the medieval sources as possible, so they are free of copyright.
- An Audio settings tab with General and Music volumes, each with a speaker / crossed-out speaker mute toggle.
- A startup variable that turns sound off, off by default in debug builds.
- A debug popup listing every basic tune to play on its own.

Answers to the review page's questions (2026-11-26):

- **Combat feel:** as modelled: stinger on the next beat, tempo +20% over a bar, drum ostinato, 10 s hold, two bars easing back.
- **Attacked:** any hostile action involving the player: being hit, hitting a hostile, or a hostile choosing them as its target.
- **Between tunes:** continuous music, no silent rests, but blended better than a bar ending and the next tune starting. The model now links: over 1–3 bars the tempo glides to the next tune's, the drone glides to its final, the melody holds a bridging note on it, and the second player and percussion fade back in over the new tune's first two bars (also on place changes).
- **Sources:** download the scans and the CC0 / CC BY files. Tunes are transcribed into standard MIDI files and loaded from them, so client mods can add to the playlist.
- **Unwritten rhythm:** keep those tunes (Cantigas, troubadours, laude) with our own rhythm, recorded as ours.
- **SoundFont:** the web build fetches it only when music first plays, so a player with music muted (or `HOW_MUSIC=false`) never downloads the ~10 MB.
- **Variables:** `HOW_SOUND` (all sound, off by default in debug builds), plus `HOW_MUSIC` for music only (ambient and interface sounds come later). Music defaults to 60%.
- **Wilderness:** themes per climate band, with day and night sets.
- **Credits:** a Music section on `docs/credits.md`, kept short: one line per manuscript or file set (and the SoundFonts), not per tune.

## Playback

- **Synth:** `rustysynth` (MIT, pure Rust, 1.3.8, 2026-10) renders a SoundFont into buffers (`render(left, right)`), so it works natively and on wasm.
- **Tunes:** standard MIDI files in `assets/music/` read with `midly` (Unlicense), plus a playlist file giving each place its tunes and players; client mods can add both. Each tune's source (and whether its rhythm is ours) is recorded beside it.
- **Output:** a custom Bevy audio source (`Decodable`, registered with `add_audio_source`) with our sequencer running inside the pull. Beat-exact changes and tempo ramps are worked out to the sample.
- **Volumes:** General is `GlobalVolume`; Music volume and mute are a smoothed gain in our source.
- **Fallback:** kira 0.12 (MIT/Apache, custom `Sound`, mixer tracks, tweens). bevy_kira_audio supports Bevy 0.20, but it has no documented path for a custom sound.
- **Avoid:** OxiSynth, which is LGPL.
- **Web:**
  - Sound only after a user gesture: start after the first click in the startup menu.
  - The SoundFont is fetched the first time music plays, never while music is muted or off.
  - cpal's main-thread backend uses about 46 ms buffers and crackles on long frames.
  - The AudioWorklet backend needs nightly and atomics, so it is out.
  - Keep 16–32 voices and no reverb on web. Measure rustysynth's CPU use in the browser.

## SoundFont

- **Use:** MuseScore General or FluidR3 (both MIT), cut down to the instruments needed, plus "Early European Instruments" (John Carter, polyphone.io 516, public domain, about 7–15 MB) for the hurdy-gurdy and bagpipe.
- **Avoid:**
  - GeneralUser GS: custom licence, unclear sample origins.
  - Sonatina: CC Sampling Plus.
  - Vieuxmac: personal use only.
  - Early Music Ensemble: CC BY-NC.
- **CC0 sample libraries:** VSCO-2 CE and VCSL, should more instruments be needed.
- **Tools:** Polyphone cuts and converts the files (a tool only, not shipped). rustysynth's SF3 support is unconfirmed, so plan on SF2.
- **Size:** aim for about 10 MB.

## Sources of melodies

- **Rule:** write the notes ourselves from public-domain manuscript scans. Use editors who died long ago only to check our readings: Aubry (d. 1910), Coussemaker, Ludwig (d. 1930), Ribera (d. 1934).
- **Ready-made files:** only CC0, dedicated public domain, or CC BY, credited in `docs/credits.md`.
- **Avoid:**
  - CC BY-SA and NC files.
  - "All rights reserved" transcriptions (Early Music Muse, Casson).
  - Anglès's Cantigas (1943), protected until about 2050.
  - Dittmer (1957), and any recent edition (Germany §70, Spain art. 129).
  - Arrangements based on recordings.
- **medieval.org's MIDI page:** a list of about 35 links to personal sites with no licences, so it is a guide to tunes only.
- **Best sources:**
  - Add MS 29987 (IMSLP scan). Kelly's Lamento di Tristano is CC0.
  - BnF fr. 844 (Gallica; Aubry 1907). Cantalapiedra's Quarte Estampie is CC BY.
  - Robertsbridge: Harding's transcription, dedicated public domain.
  - Llibre Vermell (IMSLP scan). Marieh's Stella splendens and Imperayritz, and Van Gilst's Mariam matrem, are CC0.
  - Cantigas: Toledo / Ribera 1922 on IMSLP. Marieh's CSM 100 is CC0.
  - Sumer is icumen in: Harley 978, CC0 image.
  - Kalenda maya (Gallica).
- **Rhythm:** written in the Italian dances, the Llibre Vermell and the estampies. Ours to choose in the Cantigas, the troubadour songs and the Cortona laude.
- **Avoid entirely:** Carmina Burana, whose notation is unheighted neumes.
- **25 recommended tunes:** listed in `sources-report.md`.

## Adaptive rules (proposal)

- **Parts:** a melody, a second player (heterophony: the beat skeleton held, or ornamented), a drone on the final (and fifth), percussion from the rhythmic modes (mode 1 long–short, fills at phrase ends), and combat parts.
- **Into combat:**
  - On the next beat, with a stinger (a roll, then a horn, bell or drum by place).
  - Tempo up about 20% over a bar, plus a drum ostinato and a louder drone. The key stays.
- **Out of combat:**
  - After a hold with no hostile action (proposed 8–12 s), at a bar line, two bars easing back while the combat parts fade.
  - Being attacked again while easing off, or within 30 s, skips the stinger.
- **Place changes:** at the next bar line, with the drone gliding to the new final.
- **Against repetition, without gaps:** each tune twice, then a link of 1–3 bars into the next (see the answers above). Links are short in towns and the inn, longer in the wilderness, caves and the Hall.
- **Per place:** tune counts, ensembles and modes are in the review page's table (wilderness by climate band and day or night, the world map, the Hall, the Inn, village, town, city, castle, tower, cave, farm, herbalist, oasis).

## PRs

All in order, each needing the one before.

1. **Audio settings, `HOW_SOUND` and `HOW_MUSIC`.** The Audio tab (General and Music volumes, Music 60% by default, speaker mute toggles), saved with the settings; both variables in `docs/environment-variables.md`.
2. **Synth and SoundFont.** rustysynth behind a custom Bevy source, the cut-down SoundFont in `assets/` (fetched by the web build only when music first plays), starting after the first click on the web; the Music section in `docs/credits.md`.
3. **Sequencer, MIDI tunes and the tune list.** MIDI loading and the playlist file (moddable), the sequencer with layers and links between tunes, the first transcribed tunes, the debug tune list (`docs/debug-menu.md`). Needs the transcription's first batch.
4. **Music by place.** Tune sets per place (wilderness by climate band, day and night, every poi kind, the Hall, the Inn, town sizes), changes at bar lines. Needs enough transcribed tunes for every place.
5. **Combat music.** The client's "attacked" signal, stinger, tempo ramp, combat parts, hold and ease-off. (All five built.)

Alongside (non-code): transcribing the tunes from the scans into MIDI files, in batches, each with its source and whether its rhythm is ours.
