# client: music

Music played by the client (never game state): a SoundFont's instruments rendered by `rustysynth` (MIT) inside a custom Bevy audio source (PR 2), and tunes from MIDI files (`midly`, Unlicense) named by playlists, played by a sequencer inside the stream (PR 3; `docs/context/research-music.md`). Until music by place (PR 4) every tune plays in turn; the debug menu plays one alone, all again, or the test.

## Files

- `crates/client/src/music/mod.rs`: `MusicPlugin`: the SoundFont asset and its loader, the stream as an audio source (`add_audio_source`), the messages, and the systems in one chain in `Update`.
- `crates/client/src/music/font.rs`: `SoundFontAsset` (an `Arc<rustysynth::SoundFont>`) and its `.sf2` loader; `PATH` (`music/music.sf2`).
- `crates/client/src/music/stream.rs`: `MusicEvent` (a MIDI channel message), `Command` (Play a `Program`, Stop), `Shared` (commands and events queued for the next block, the target gain as `f32` bits), `MusicStream` (the asset Bevy plays: `Decodable`, handing its one synthesizer to the decoder).
- `crates/client/src/music/decoder.rs`: `MusicDecoder` (rodio `Source`, stereo at 44.1 kHz, endless): at each 64-sample block takes the commands (to its `Sequencer`), then queued events, and renders the block in pieces between the sequencer's events, so each lands on its sample (moving the sequencer on without rendering while the gain and target are 0); the gain eased towards the target per sample (`eased`, about 30 ms).
- `crates/client/src/music/start.rs`: `Music` (Waiting, Loading, Playing, Failed), `start_music` (loads the SoundFont only once `SoundSwitches::music_on`, `sound::MusicVolume` above 0 and `MayPlay`; then a synthesizer, 64 voices natively, 24 and no reverb or chorus in a browser, and an `AudioPlayer` entity, `MusicPlayer`), `sink_at_full`, `follow_volume` (target = `MusicVolume` × `GlobalVolume`), `send_events`.
- `crates/client/src/music/gesture.rs`: `MayPlay`: true natively; in a browser after the first click, tap or key released.
- `crates/client/src/music/tune.rs`: `Tune` (title, notes in ticks of `PPQ` = 960 a quarter, `length` once round, tempos in quarters a minute, `Signature`s, `final_class`), `Signature` (`unit`, `bar`, `beat`: three units in compound metres, `in_bar`), `nearest_of_class`.
- `crates/client/src/music/midi.rs`: `read_tune`: a standard MIDI file into a `Tune` (notes off the drum channel, any number of tracks, ticks rescaled; track name, tempo and time signature changes, "mode: ... on X" text for the final, "loop" marker for the length; notes cut at it), `final_of`.
- `crates/client/src/music/playlist.rs`: `PlaylistFile` (`players`, `tunes`), `TuneEntry` (id, file from the assets folder, title, source, read_from, licence, `Rhythm` Written or Ours, places as words for PR 4, `after`), `Players` (melody, second, drone as `Instrument` bank and program, `Percussion` strong and light drum keys, `link_seconds`; a recorder alone by default), `merge` (later files' players win, a known id replaced in place), `in_order` (an `after` tune moved just after the one it follows).
- `crates/client/src/music/loaders.rs`: `TuneAsset` (`.mid`, `.midi`) and `PlaylistAsset` (`.playlist.ron`) with their loaders.
- `crates/client/src/music/library.rs`: `Library` loading once music plays: natively every playlist in `music/playlists/` (`load_folder`; the game's first, mods' by name), in a browser only `game.playlist.ron`; merged, ordered, each tune's file loaded, unreadable ones left out with a warning; `Ready` (tunes, players); `TuneList`.
- `crates/client/src/music/play_tunes.rs`: `PlayTunes` (All, One), `Asked`, `play_tunes` (tells the stream `Command::Play` once ready, and on each ask).
- `crates/client/src/music/sequencer.rs`: `Sequencer` (run by the decoder: `play`, `stop`, `samples_to_next`, `advance`; a queue of what is due by tick, a clock turning ticks to samples at the tempo of the moment), `Program` (tunes, players), `LOOPS`; channels melody 0, second 1, drone 2, drums 9. `sequencer/due.rs`: `Due` (MIDI, Tempo, PassEnd, LinkEnd) and `Queue` (by tick; offs, then controls, then markers, then ons). `sequencer/pass.rs`: a pass of a tune queued at once: melody (2% shorter), second player on notes starting on a beat an octave down at 70%, held to the next beat with a note (at most 4), drums a unit at a time by `drum_level` (the first rhythmic mode), fading in over 2 bars after a link; `end_pass` (again, or a link after `LOOPS`). `sequencer/link.rs`: `begin_link` (1 to 3 bars by `link_seconds`, the tempo gliding a unit at a time to the next tune's, the drone moved half a bar in, a bridging note on the next final near its first note), `end_link`, `move_drone` (final and fifth in the third octave).
- `crates/client/src/music/test_tune.rs`: `PlayTestMusic`, `INSTRUMENTS`, `test_tune` (a D Dorian phrase on each instrument, bank and program, then drums), `play_test_tune` (starts once the music plays, sends events by `Time`).

## Public API

- `MusicPlugin`.
- `MusicEvent` (message): a MIDI channel message for the synthesizer (`note_on`, `note_off`, `bank`, `program`); dropped while no music plays.
- `PlayTestMusic` (message): stops the tunes and plays the test from the start (debug menu's Play Test Music).
- `PlayTunes` (message): `All` (every tune in turn, as at the start) or `One(n)` (the `n`th in `TuneList`, round and round): the debug menu's Play All Tunes and Play Tune.
- `TuneList` (resource): every tune's title in the order they play, once loaded (empty until music starts).

## Depends on

- `common`: `SoundSwitches`. `sound`: `MusicVolume`. Bevy's audio (`Decodable`, `GlobalVolume`, `AudioSink`), `rustysynth`, `midly` (without its threads, for the web).

## Used by

- `debug_menu` (Play Tune's list, Play All Tunes, Play Test Music).

## Notes

- **The SoundFont:** `crates/client/assets/music/music.sf2`, made by `tools/subset_soundfont.py` from MuseScore General 0.2 (MIT; licence beside it) and Early European Instruments (public domain; a signed-in download from polyphone.io, which the owner did), both credited in `ATTRIBUTION.md` and `docs/credits.md`; downloads kept in `D:\how-downloads\soundfonts\`, never in the repository. Presets keep their General MIDI bank and program, so tunes name instruments as General MIDI does (`PRESETS` in the script says what each stands for: recorder, fiddle, nylon guitar for the lute, harp, dulcimer for the psaltery, shanai for the shawm, trumpet, trombone for the sackbut, bagpipe, church and reed organs, voice oohs for the choir, tubular and tinkle bells, timpani for nakers, taiko). The standard kit (bank 128) keeps only the keys in `DRUM_KEYS`. The hurdy-gurdy and cornemuse are in bank 8 (General MIDI's variations) as programs 110 and 109 (the fiddle's and bagpipe's), one channel at 48 kHz (from 192 kHz stereo); a tune picks them with control change 0 = 8 (`MusicEvent::bank`) before the program. 12.9 MB; `--sizes` prints what each preset would bring (Choir Aahs alone 4.8 MB, left out). Tests check the test tune's instruments are all there, and that the two from bank 8 sound.
- **Browsers:** the SoundFont is fetched only when music first starts, so never with music muted or off. `index.html` resumes the audio context on the first click, tap or key, since Bevy opens it at startup, when browsers keep it suspended.
- **Volumes:** Bevy applies `GlobalVolume` only when a sound starts, so the music's sink is set to full once (`sink_at_full`) and General is folded into the stream's own eased gain.
- **Events** are applied at the start of the next block (64 samples, about 1.5 ms), not to the sample; the sequencer runs inside the stream, so its own are exact to the sample.
- **Tunes** (`assets/music/tunes/`, 18 from the transcription task: `~/.claude/research-tools/music/tunes/`, whose `tunes.toml` the game's playlist was made from): single-melody MIDI files, format 0, 480 ticks a quarter, melody on channel 0, no program change (the players are the playlist's), a "loop" marker where each goes round seamlessly. Credited in `docs/credits.md` (one line per manuscript or edition) and `assets/ATTRIBUTION.md` (the one CC BY edition). La Rotta follows the Lamento (`after`).
- **Mods** add a `<name>.playlist.ron` in `music/playlists/` (natively only, as a browser can't list a folder), with their tunes (paths from the assets folder) and, if they like, players; an id already known replaces that tune.
- **Players** are the playlist's for now (recorder, lute an octave down, reed organ drone, bongos); places choose their own in PR 4, and combat (PR 5) adds its parts and the tempo rise through the same queue.
- **Never silent:** one tune plays round for good; more are each played twice and linked (owner's decision: no rests). Muted music keeps time without rendering.
