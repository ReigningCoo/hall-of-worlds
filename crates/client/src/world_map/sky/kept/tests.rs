use bevy::prelude::*;

use super::*;

fn params(blend_from: f32, written: f32, rate: f32) -> SkyParams {
    SkyParams {
        blend_from,
        written,
        still: Vec4::new(0.1, 1.0, rate, 3600.0),
        ..default()
    }
}

#[test]
fn the_fade_runs_on_from_when_it_was_written() {
    let stored = params(0.25, 10.0, 0.5);
    assert!((blend_at(&stored, 11.0) - 0.75).abs() < 1e-6);
    assert_eq!(blend_at(&stored, 20.0), 1.0);
}

#[test]
fn the_fade_runs_on_across_the_clock_wrapping() {
    let stored = params(0.0, 3599.5, 0.5);
    assert!((blend_at(&stored, 0.5) - 0.5).abs() < 1e-6);
}

#[test]
fn the_same_sky_a_moment_later_needs_no_writing() {
    let stored = params(0.25, 10.0, 0.5);
    let now = params(0.5, 10.5, 0.5);
    assert!(looks_like(&stored, &now, 10.5));
}

#[test]
fn a_fade_gone_astray_or_a_new_light_is_written() {
    let stored = params(0.25, 10.0, 0.5);
    assert!(!looks_like(&stored, &params(0.9, 10.5, 0.5), 10.5));
    let mut lit = params(0.5, 10.5, 0.5);
    lit.light = Vec4::ONE;
    assert!(!looks_like(&stored, &lit, 10.5));
}
