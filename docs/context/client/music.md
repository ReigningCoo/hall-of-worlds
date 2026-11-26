# client: music

Music played by the client (never game state): a SoundFont's instruments rendered by `rustysynth` (MIT) inside a custom Bevy audio source, music plan PR 2 (`docs/context/research-music.md`). No tunes yet: the sequencer and MIDI tunes come in PR 3; until then the game can send MIDI events, and the debug menu's Play Test Music plays a test.

## Files

- `crates/client/src/music/mod.rs`: `MusicPlugin`: the SoundFont asset and its loader, the stream as an audio source (`add_audio_source`), the messages, and the systems in one chain in `Update`.
- `crates/client/src/music/font.rs`: `SoundFontAsset` (an `Arc<rustysynth::SoundFont>`) and its `.sf2` loader; `PATH` (`music/music.sf2`).
- `crates/client/src/music/stream.rs`: `MusicEvent` (a MIDI channel message), `Shared` (events queued for the next block, the target gain as `f32` bits), `MusicStream` (the asset Bevy plays: `Decodable`, handing its one synthesizer to the decoder).
- `crates/client/src/music/decoder.rs`: `MusicDecoder` (rodio `Source`, stereo at 44.1 kHz, endless): applies queued events at each 64-sample block, renders it, or silence without rendering while the gain and target are 0; the gain eased towards the target per sample (`eased`, about 30 ms).
- `crates/client/src/music/start.rs`: `Music` (Waiting, Loading, Playing, Failed), `start_music` (loads the SoundFont only once `SoundSwitches::music_on`, `sound::MusicVolume` above 0 and `MayPlay`; then a synthesizer, 64 voices natively, 24 and no reverb or chorus in a browser, and an `AudioPlayer` entity, `MusicPlayer`), `sink_at_full`, `follow_volume` (target = `MusicVolume` × `GlobalVolume`), `send_events`.
- `crates/client/src/music/gesture.rs`: `MayPlay`: true natively; in a browser after the first click, tap or key released.
- `crates/client/src/music/test_tune.rs`: `PlayTestMusic`, `INSTRUMENTS`, `test_tune` (a D Dorian phrase on each instrument, bank and program, then drums), `play_test_tune` (starts once the music plays, sends events by `Time`).

## Public API

- `MusicPlugin`.
- `MusicEvent` (message): a MIDI channel message for the synthesizer (`note_on`, `note_off`, `bank`, `program`); dropped while no music plays.
- `PlayTestMusic` (message): plays the test from the start (debug menu's Play Test Music).

## Depends on

- `common`: `SoundSwitches`. `sound`: `MusicVolume`. Bevy's audio (`Decodable`, `GlobalVolume`, `AudioSink`), `rustysynth`.

## Used by

- `debug_menu` (Play Test Music). The sequencer (PR 3) will play through it.

## Notes

- **The SoundFont:** `crates/client/assets/music/music.sf2`, made by `tools/subset_soundfont.py` from MuseScore General 0.2 (MIT; licence beside it) and Early European Instruments (public domain; a signed-in download from polyphone.io, which the owner did), both credited in `ATTRIBUTION.md` and `docs/credits.md`; downloads kept in `D:\how-downloads\soundfonts\`, never in the repository. Presets keep their General MIDI bank and program, so tunes name instruments as General MIDI does (`PRESETS` in the script says what each stands for: recorder, fiddle, nylon guitar for the lute, harp, dulcimer for the psaltery, shanai for the shawm, trumpet, trombone for the sackbut, bagpipe, church and reed organs, voice oohs for the choir, tubular and tinkle bells, timpani for nakers, taiko). The standard kit (bank 128) keeps only the keys in `DRUM_KEYS`. The hurdy-gurdy and cornemuse are in bank 8 (General MIDI's variations) as programs 110 and 109 (the fiddle's and bagpipe's), one channel at 48 kHz (from 192 kHz stereo); a tune picks them with control change 0 = 8 (`MusicEvent::bank`) before the program. 12.9 MB; `--sizes` prints what each preset would bring (Choir Aahs alone 4.8 MB, left out). Tests check the test tune's instruments are all there, and that the two from bank 8 sound.
- **Browsers:** the SoundFont is fetched only when music first starts, so never with music muted or off. `index.html` resumes the audio context on the first click, tap or key, since Bevy opens it at startup, when browsers keep it suspended.
- **Volumes:** Bevy applies `GlobalVolume` only when a sound starts, so the music's sink is set to full once (`sink_at_full`) and General is folded into the stream's own eased gain.
- **Events** are applied at the start of the next block (64 samples, about 1.5 ms), not to the sample; the sequencer is planned to run inside the stream for exact timing.
