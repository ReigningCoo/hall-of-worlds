//! Roots: globes with a thin tail (turnip, beetroot, radish), long tapers
//! (carrot, parsnip, burdock), spindles (sweet potato, cassava) and a cluster of
//! fingers (skirret). Built along +x, the top (leaf stalks or a trimmed crown)
//! at +x.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Build;
use super::colours::colour;
use super::group::Root;
use super::heap::{Place, side_by_side};
use super::tube::{Round, cap, rings, tube};

/// Roots sold with their leaf stalks; the rest are trimmed.
const LEAFY: [&str; 5] = ["carrot", "wild_carrot", "radish", "beetroot", "turnip"];

pub(super) fn roots(build: &Build, root: Root, kit: &mut super::Kit) {
    let width = build.width_share().clamp(0.07, 0.4);
    let places = match build.count {
        1 => vec![Place::ONE],
        n => side_by_side(
            n,
            if root == Root::Globe {
                0.8
            } else {
                width * 1.3
            },
            build.variety,
        ),
    };
    for (n, place) in places.iter().enumerate() {
        match root {
            Root::Globe => globe(build, place, n, kit),
            Root::Taper => taper(build, place, n, width, kit),
            Root::Spindle => spindle(build, place, n, width, kit),
            Root::Cluster => cluster(build, place, width, kit),
        }
    }
}

/// A globe about 1 across, its tail at -x and top at +x.
fn globe(build: &Build, place: &Place, n: usize, kit: &mut super::Kit) {
    let c = build.colours;
    let squat = 0.9 + build.variety.wobble(n, 1) * 0.06;
    let half = |t: f32| {
        let s = ((t - 0.35) / 0.65).clamp(0.0, 1.0);
        let body = (1.0 - (2.0 * s - 1.0).powi(2)).max(0.0).sqrt() * 0.5;
        let tail = 0.05 * cap(t, 0.1) * (1.0 - s);
        Vec2::new(body.max(tail), body.max(tail) * squat)
    };
    let shoulder = c.accent;
    let skin = c.skin;
    let at = |t: f32| place.p(Vec3::new(t * 1.55 - 0.9, 0.5 * squat, 0.0));
    let colour = move |t: f32| match shoulder {
        Some(top) => skin.lerp(top, ((t - 0.72) / 0.18).clamp(0.0, 1.0)),
        None => skin,
    };
    let rings = rings(28, at, |t| half(t) * place.size, colour);
    tube(kit, &rings, Round::default(), false);
    top(build, place, at(1.0), 0.18, n, kit);
}

/// A long taper `width` across at its shoulder (+x), coming to a point at -x.
fn taper(build: &Build, place: &Place, n: usize, width: f32, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let bend = build.variety.wobble(n, 2) * 0.06;
    let r = width / 2.0;
    let at = |t: f32| place.p(Vec3::new(t - 0.5, r, (PI * t).sin() * bend));
    let half = |t: f32| Vec2::splat(r * t.powf(0.9) * cap(1.0 - t, 0.05) * place.size);
    // Faint rings round it.
    let colour = move |t: f32| skin * (1.0 - 0.07 * f32::from((t * 13.0).fract() < 0.2));
    tube(kit, &rings(30, at, half, colour), Round::default(), false);
    rootlets(build, place, &at, r, n, kit);
    top(build, place, at(1.0), r, n, kit);
}

/// Plump in the middle, pointed at both ends, a little curved.
fn spindle(build: &Build, place: &Place, n: usize, width: f32, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let bend = 0.06 + build.variety.wobble(n, 2) * 0.05;
    let r = width / 2.0;
    let lump = build.variety.wobble(n, 3) * 0.1;
    let at = |t: f32| place.p(Vec3::new(t - 0.5, r, (PI * t).sin() * bend));
    let half = |t: f32| {
        let body = (PI * t).sin().max(0.0).powf(0.7) * (1.0 + lump * (PI * 3.0 * t).sin());
        Vec2::new(r, r * 0.9) * body * place.size
    };
    let colour = move |t: f32| skin * (1.0 - 0.08 * f32::from((t * 9.0).fract() < 0.15));
    tube(kit, &rings(30, at, half, colour), Round::default(), false);
    rootlets(build, place, &at, r, n, kit);
}

/// Slender fingers from one crown at +x.
fn cluster(build: &Build, place: &Place, width: f32, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let fingers = 6;
    let crown = place.p(Vec3::new(0.5, width, 0.0));
    for k in 0..fingers {
        let spread = (k as f32 / (fingers - 1) as f32 - 0.5) * 0.9;
        let a = PI + spread + build.variety.wobble(k, 4) * 0.08;
        let length = 0.85 + build.variety.wobble(k, 5) * 0.12;
        let out = Vec3::new(a.cos(), 0.0, a.sin());
        let lift = (k % 2) as f32 * width * 0.6;
        let r = width * 0.5;
        let at = |t: f32| crown + place.p(out * t * length + Vec3::Y * lift) - place.at;
        let half = |t: f32| Vec2::splat(r * (1.0 - t).powf(0.7) * cap(t, 0.04));
        tube(kit, &rings(16, at, half, |_| skin), Round::default(), false);
    }
    top(build, place, crown, width * 0.9, 0, kit);
}

/// Hair-fine rootlets along a root's lower half.
fn rootlets(
    build: &Build,
    place: &Place,
    at: &impl Fn(f32) -> Vec3,
    r: f32,
    n: usize,
    kit: &mut super::Kit,
) {
    let fine = kit.material(colour(build.colours.skin * 0.8), false);
    for k in 0..2 {
        let t = 0.3 + k as f32 * 0.2;
        let side = if (k + n).is_multiple_of(2) { 1.0 } else { -1.0 };
        let from = at(t);
        let out = place.p(Vec3::new(-0.03, 0.0, side * (r * 0.8 + 0.03))) - place.at;
        kit.rod(&fine, from, from + out, 0.003 * place.size);
    }
}

/// Leaf stalks cut short, or a trimmed brown crown, at `at`.
fn top(build: &Build, place: &Place, at: Vec3, r: f32, n: usize, kit: &mut super::Kit) {
    if !LEAFY.contains(&build.def.key.as_str()) {
        let crown = kit.material(colour(build.colours.skin * 0.7), false);
        kit.blob(&crown, at, Vec3::splat(r * 0.35 * place.size), (0.0, 0.0));
        return;
    }
    let stalk = match build.def.key.as_str() {
        "beetroot" => build.colours.skin.lerp(build.colours.leaf, 0.3),
        _ => build.colours.leaf,
    };
    let stalk = kit.material(colour(stalk), false);
    for k in 0..4 {
        let a = (k as f32 - 1.5) * 0.35 + build.variety.wobble(n * 4 + k, 6) * 0.1;
        let length = 0.3 + build.variety.share(n * 4 + k, 7) * 0.12;
        let tip = place.p(Vec3::new(a.cos(), 0.08, a.sin()) * length) - place.at;
        kit.rod(&stalk, at, at + tip, (0.012 + r * 0.06) * place.size);
    }
}
