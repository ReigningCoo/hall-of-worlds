use std::sync::Arc;

use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

use super::decoder::{MusicDecoder, SAMPLE_RATE, eased};
use super::stream::{MusicEvent, Shared};
use super::test_tune::{INSTRUMENTS, test_tune};

const FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/music/music.sf2");

fn font() -> Arc<SoundFont> {
    let bytes = std::fs::read(FONT).expect("the SoundFont is in the assets");
    Arc::new(SoundFont::new(&mut bytes.as_slice()).expect("rustysynth reads it"))
}

#[test]
fn the_gain_eases_to_its_target_and_lands_on_it() {
    let mut gain = 0.0;
    for _ in 0..1_000 {
        gain = eased(gain, 1.0);
    }
    assert!(gain > 0.4 && gain < 1.0, "about 23 ms in: {gain}");
    for _ in 0..20_000 {
        gain = eased(gain, 1.0);
    }
    assert_eq!(gain, 1.0);
}

#[test]
fn the_test_tune_is_in_order_and_lets_every_note_go() {
    let events = test_tune();
    assert!(events.windows(2).all(|pair| pair[0].0 <= pair[1].0 + 0.5));
    let on = events.iter().filter(|(_, e)| e.command == 0x90).count();
    let off = events.iter().filter(|(_, e)| e.command == 0x80).count();
    assert_eq!(on, off);
}

#[test]
fn the_soundfont_has_every_instrument_the_test_plays() {
    let font = font();
    let has = |bank: i32, program: u8| {
        font.get_presets()
            .iter()
            .any(|p| p.get_bank_number() == bank && p.get_patch_number() == i32::from(program))
    };
    for (bank, program) in INSTRUMENTS {
        assert!(
            has(i32::from(bank), program),
            "bank {bank} program {program}"
        );
    }
    assert!(has(128, 0), "the drum kit");
}

/// How loud a note on this instrument is over its first second.
fn loudness(bank: u8, program: u8) -> f32 {
    let synth = Synthesizer::new(&font(), &SynthesizerSettings::new(SAMPLE_RATE as i32))
        .expect("a synthesizer");
    let shared = Arc::new(Shared::new());
    let decoder = MusicDecoder::new(Some(synth), shared.clone());
    shared.send(MusicEvent::bank(0, bank));
    shared.send(MusicEvent::program(0, program));
    shared.send(MusicEvent::note_on(0, 62, 110));
    shared.set_target_gain(1.0);
    decoder.take(2 * SAMPLE_RATE as usize).map(f32::abs).sum()
}

#[test]
fn the_hurdy_gurdy_and_cornemuse_sound() {
    assert!(loudness(8, 110) > 1.0);
    assert!(loudness(8, 109) > 1.0);
}

#[test]
fn the_stream_plays_notes_at_its_gain_and_silence_while_silent() {
    let synth = Synthesizer::new(&font(), &SynthesizerSettings::new(SAMPLE_RATE as i32))
        .expect("a synthesizer");
    let shared = Arc::new(Shared::new());
    let mut decoder = MusicDecoder::new(Some(synth), shared.clone());
    shared.send(MusicEvent::program(0, 74));
    shared.send(MusicEvent::note_on(0, 72, 110));
    let silent: f32 = decoder.by_ref().take(4_410).map(f32::abs).sum();
    assert_eq!(silent, 0.0, "the gain is 0 until set");
    shared.set_target_gain(1.0);
    let loud: f32 = decoder.by_ref().take(44_100).map(f32::abs).sum();
    assert!(loud > 1.0, "a recorder's note: {loud}");
}
