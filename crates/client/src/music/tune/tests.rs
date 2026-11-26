use super::super::midi::{final_of, read_tune};
use super::*;

const TUNES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/music/tunes");

fn tune(id: &str) -> Tune {
    let bytes = std::fs::read(format!("{TUNES}/{id}.mid")).expect("the tune is in the assets");
    read_tune(&bytes).expect("it reads")
}

#[test]
fn finals_come_from_the_mode_text() {
    assert_eq!(final_of("mode: Dorian on G"), Some(7));
    assert_eq!(final_of("mode: Lydian on F#"), Some(6));
    assert_eq!(final_of("mode: Phrygian on Bb"), Some(10));
    assert_eq!(final_of("Dorian on G"), None);
}

#[test]
fn the_nearest_key_of_a_class_is_within_half_an_octave() {
    assert_eq!(nearest_of_class(2, 62), 62);
    assert_eq!(nearest_of_class(2, 67), 62);
    assert_eq!(nearest_of_class(2, 69), 74);
    assert_eq!(nearest_of_class(0, 5), 0);
}

#[test]
fn the_trotto_reads_with_its_tempo_metre_final_and_loop() {
    let trotto = tune("trotto");
    assert_eq!(trotto.title, "Trotto");
    assert!((trotto.tempo() - 150.0).abs() < 0.01, "{}", trotto.tempo());
    let signature = trotto.signature_at(0);
    assert_eq!((signature.numerator, signature.denominator), (6, 8));
    assert_eq!(signature.beat(), PPQ * 3 / 2);
    assert_eq!(trotto.final_class, 7);
    assert_eq!(trotto.length % signature.bar(), 0, "whole bars round");
    assert!(
        trotto
            .notes
            .iter()
            .all(|n| n.at + n.length <= trotto.length)
    );
}

#[test]
fn stella_splendens_changes_metre_in_the_middle() {
    let stella = tune("stella_splendens");
    assert!(stella.signatures.len() > 2);
    assert!(stella.signatures.iter().any(|s| s.numerator == 3));
    assert!(
        stella
            .signatures
            .windows(2)
            .all(|pair| pair[0].at < pair[1].at)
    );
}

#[test]
fn every_tune_in_the_assets_reads() {
    let mut count = 0;
    for file in std::fs::read_dir(TUNES).expect("the tunes folder") {
        let path = file.expect("a file").path();
        let bytes = std::fs::read(&path).expect("readable");
        let tune = read_tune(&bytes).unwrap_or_else(|e| panic!("{path:?}: {e}"));
        assert!(!tune.notes.is_empty() && !tune.title.is_empty(), "{path:?}");
        count += 1;
    }
    assert!(count >= 18);
}
