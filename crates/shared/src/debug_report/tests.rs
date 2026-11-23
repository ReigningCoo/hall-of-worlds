use super::*;
use crate::creature::CreatureKind;
use crate::difficulty::Difficulty;
use crate::facing::Heading;
use crate::gender::Gender;
use crate::id::Id;
use crate::poi::{ChunkPos, PoiKind};
use crate::protocol::CharacterId;
use crate::rng::Seed;
use crate::spot::Spot;
use crate::world::ClockRate;

fn sample() -> DebugReport {
    DebugReport {
        build: Build {
            version: "0.1.0".into(),
            protocol: 7,
            platform: Platform::Native,
            system: "windows".into(),
        },
        utc: 1_794_787_200,
        who: Some(Who {
            id: Some(CharacterId(Id(uuid::Uuid::from_u128(42)))),
            kind: CreatureKind::Human,
            gender: Gender::Female,
            difficulty: Some(Difficulty::Hard),
        }),
        place: Some(Where::Site {
            seed: Seed::new(u64::MAX - 3),
            x: -120,
            y: 77,
            chunk: ChunkPos { x: 9, y: 1 },
            kind: PoiKind::RockCave,
            ruined: true,
            level: 2,
            levels: 3,
            at: Spot::new(1234, -56),
            heading: Heading::NORTH,
        }),
        planet: Some(4321),
        clock: Some(ReportClock {
            minutes: 12_345.5,
            rate: ClockRate::NORMAL,
        }),
        camera: Some(Camera {
            turn: 0.5,
            tilt: 0.9,
            zoom: 30.0,
        }),
        flags: DebugFlags {
            god_mode: true,
            ..DebugFlags::default()
        },
        dev: Some(DevPlace::Site {
            kind: PoiKind::RockCave,
            seed: 99,
            ruin: 2,
        }),
        problem: "The stairs lead into a wall.\nTwice!".into(),
    }
}

#[test]
fn a_report_comes_back_the_same() {
    let report = sample();
    let text = encode(&report);
    assert!(text.starts_with(PREFIX));
    assert_eq!(decode(&text), Ok(report));
}

#[test]
fn the_string_is_url_safe() {
    let text = encode(&sample());
    let body = &text[PREFIX.len()..];
    let safe = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    assert!(body.chars().all(safe), "{text}");
}

#[test]
fn spaces_and_line_breaks_from_pasting_are_ignored() {
    let text = encode(&sample());
    let (start, end) = text.split_at(text.len() / 2);
    let pasted = format!("  {start}\r\n {end}\n");
    assert_eq!(decode(&pasted), Ok(sample()));
}

#[test]
fn an_empty_report_comes_back() {
    let report = DebugReport {
        who: None,
        place: None,
        planet: None,
        clock: None,
        camera: None,
        dev: None,
        problem: String::new(),
        ..sample()
    };
    assert_eq!(decode(&encode(&report)), Ok(report));
}

#[test]
fn other_text_is_not_a_report() {
    for text in [
        "", "hello", "HOW", "HOW1", "HOWx.abc", "HOW.abc", "how1.abc",
    ] {
        assert_eq!(decode(text), Err(DecodeError::NotAReport), "{text:?}");
    }
}

#[test]
fn another_version_is_named() {
    let text = encode(&sample()).replacen("HOW1.", "HOW2.", 1);
    assert_eq!(decode(&text), Err(DecodeError::UnknownVersion("2".into())));
}

#[test]
fn a_damaged_string_is_refused() {
    let text = encode(&sample());
    // Cut short.
    assert_eq!(decode(&text[..text.len() - 5]), Err(DecodeError::Damaged));
    // A letter changed in the middle: the checksum (or inflating) finds it.
    let mut letters: Vec<char> = text.chars().collect();
    let middle = PREFIX.len() + (letters.len() - PREFIX.len()) / 2;
    letters[middle] = if letters[middle] == 'A' { 'B' } else { 'A' };
    let changed: String = letters.into_iter().collect();
    assert_eq!(decode(&changed), Err(DecodeError::Damaged));
    // Not base64 at all.
    assert_eq!(decode("HOW1.!!!"), Err(DecodeError::Damaged));
}

#[test]
fn too_long_a_string_is_refused_before_reading() {
    let text = format!("{PREFIX}{}", "A".repeat(MOST_LETTERS));
    assert_eq!(decode(&text), Err(DecodeError::TooLong));
}

#[test]
fn something_inflating_too_far_is_refused() {
    let huge = vec![b' '; 1 << 20];
    let squeezed = miniz_oxide::deflate::compress_to_vec_zlib(&huge, 10);
    use base64::Engine;
    let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(squeezed);
    assert_eq!(
        decode(&format!("{PREFIX}{body}")),
        Err(DecodeError::Damaged)
    );
}

#[test]
fn a_full_problem_stays_short_enough_to_paste() {
    let report = DebugReport {
        problem: "Something odd happened here. ".repeat(40),
        ..sample()
    };
    assert!(encode(&report).len() < MOST_LETTERS / 4);
}

#[test]
fn strings_without_a_planet_still_read() {
    let mut json = serde_json::to_value(sample()).unwrap();
    json.as_object_mut().unwrap().remove("planet");
    let report: DebugReport = serde_json::from_value(json).unwrap();
    assert_eq!(report.planet, None);
}

#[test]
fn a_report_in_the_inn_comes_back_the_same() {
    let report = DebugReport {
        place: Some(Where::Inn {
            level: 1,
            at: Spot::new(4920, 7080),
            heading: Heading::NORTH,
        }),
        dev: None,
        ..sample()
    };
    assert_eq!(decode(&encode(&report)), Ok(report));
}
