use super::volume::{gain, gains};
use crate::common::{AudioPrefs, SoundSwitches};

#[test]
fn gains_follow_the_percentages_squared() {
    assert_eq!(gain(0), 0.0);
    assert_eq!(gain(100), 1.0);
    assert_eq!(gain(50), 0.25);
    assert_eq!(gain(200), 1.0);
}

#[test]
fn music_plays_at_sixty_percent_by_default() {
    let (general, music) = gains(AudioPrefs::default(), SoundSwitches::default());
    assert_eq!(general, 1.0);
    assert!((music - 0.36).abs() < 1e-6);
}

#[test]
fn mutes_and_switches_silence() {
    let on = SoundSwitches::default();
    let muted = AudioPrefs {
        music_muted: true,
        ..AudioPrefs::default()
    };
    assert_eq!(gains(muted, on), (1.0, 0.0));
    let no_sound = SoundSwitches {
        sound: false,
        music: true,
    };
    assert_eq!(gains(AudioPrefs::default(), no_sound), (0.0, 0.0));
    let no_music = SoundSwitches {
        sound: true,
        music: false,
    };
    assert_eq!(gains(AudioPrefs::default(), no_music).1, 0.0);
}
