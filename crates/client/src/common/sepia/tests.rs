use super::*;

#[test]
fn black_stays_black_and_white_turns_cream() {
    assert_eq!(sepia(0.0), [0.0, 0.0, 0.0]);
    assert_eq!(sepia(1.0), PAPER);
}

#[test]
fn middle_tones_are_brown_and_keep_their_lightness() {
    let [r, g, b] = sepia(0.5);
    assert!(r > g && g > b, "{r} {g} {b}");
    assert!((lightness([r, g, b]) - 0.5).abs() < 0.03);
}

#[test]
fn darker_in_is_darker_out() {
    for step in 0..10 {
        let (dark, light) = (step as f32 / 10.0, (step + 1) as f32 / 10.0);
        assert!(lightness(sepia(dark)) < lightness(sepia(light)));
        assert!(lightness(sepia_linear(dark)) < lightness(sepia_linear(light)));
    }
}

#[test]
fn linear_matches_on_screen() {
    let [r, g, b] = sepia_linear(0.2);
    assert!(r > g && g > b, "{r} {g} {b}");
}
