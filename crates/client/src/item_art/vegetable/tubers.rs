//! Tubers, corms and rhizomes: lumpy ones with eyes (potato), knobbly ones
//! (Jerusalem artichoke), a ringed corm (taro), jointed rhizomes (cattail,
//! arrowroot) and a length cut from a yam. Built along +x.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Build;
use super::colours::colour;
use super::group::Tuber;
use super::heap::{Place, heaped, side_by_side};
use super::tube::{Round, cap, rings, tube};

pub(super) fn tubers(build: &Build, tuber: Tuber, kit: &mut super::Kit) {
    let width = build.width_share().clamp(0.1, 0.8);
    let places = match (build.count, tuber) {
        (1, _) => vec![Place::ONE],
        (n, Tuber::Knobbly) => side_by_side(n, 0.45, build.variety),
        (n, _) => heaped(n, 0.55, build.variety),
    };
    for (n, place) in places.iter().enumerate() {
        match tuber {
            Tuber::Lumpy => lumpy(build, place, n, width, kit),
            Tuber::Knobbly => knobbly(build, place, n, kit),
            Tuber::Ringed => ringed(build, place, width, kit),
            Tuber::Rhizome => rhizome(build, place, n, width.min(0.14), kit),
            Tuber::Length => length(build, place, width.min(0.3), kit),
        }
    }
}

/// An oval with a lump or two and dark eyes on top.
fn lumpy(build: &Build, place: &Place, n: usize, width: f32, kit: &mut super::Kit) {
    let v = build.variety;
    let r = width / 2.0;
    let lump = 0.06 + v.share(n, 1) * 0.06;
    let at = |t: f32| place.p(Vec3::new(t - 0.5, r * 0.85, 0.0));
    let half = |t: f32| {
        let body = (PI * t).sin().max(0.0).powf(0.55);
        let bulge = 1.0 + lump * (PI * 2.0 * t + v.wobble(n, 2) * 2.0).sin();
        Vec2::new(r, r * 0.82) * body * bulge * place.size
    };
    let skin = build.colours.skin;
    tube(kit, &rings(22, at, half, |_| skin), Round::default(), false);
    let eye = kit.material(colour(skin * 0.62), false);
    for k in 0..3 {
        let t = 0.25 + k as f32 * 0.25;
        let side = v.wobble(n * 3 + k, 3) * 0.6;
        let spot = at(t) + place.p(Vec3::new(0.0, r * 0.8, side * r)) - place.at;
        kit.blob(&eye, spot, Vec3::splat(r * 0.09 * place.size), (0.0, 0.0));
    }
}

/// Three or four knobs in a crooked line, each with a bud.
fn knobbly(build: &Build, place: &Place, n: usize, kit: &mut super::Kit) {
    let v = build.variety;
    let skin = kit.material(colour(build.colours.skin), false);
    let bud = kit.material(colour(build.colours.skin * 0.75), false);
    let mut at = Vec3::new(-0.45, 0.0, 0.0);
    for k in 0..4 {
        let size = 0.17 + v.share(n * 4 + k, 4) * 0.07;
        let spot = place.p(at + Vec3::Y * size * 0.8);
        let half = Vec3::new(size * 1.2, size * 0.8, size) * place.size;
        kit.blob(&skin, spot, half, (place.turn(v.wobble(n * 4 + k, 5)), 0.0));
        let knob = spot + place.p(Vec3::new(0.0, size * 0.6, size * 0.7)) - place.at;
        kit.blob(&bud, knob, Vec3::splat(size * 0.3 * place.size), (0.0, 0.0));
        at += Vec3::new(0.3, 0.0, v.wobble(n * 4 + k, 6) * 0.18);
    }
}

/// A round corm ringed with darker hairy bands, a pink sprout on top.
fn ringed(build: &Build, place: &Place, width: f32, kit: &mut super::Kit) {
    let r = width / 2.0;
    let skin = build.colours.skin;
    let at = |t: f32| place.p(Vec3::new(t - 0.5, r, 0.0));
    let half = |t: f32| Vec2::splat(r * (PI * t).sin().max(0.0).powf(0.6) * place.size);
    let colour_at = move |t: f32| match (t * 9.0).fract() < 0.3 {
        true => skin * 0.68,
        false => skin,
    };
    tube(
        kit,
        &rings(30, at, half, colour_at),
        Round::default(),
        false,
    );
    let sprout = kit.material(colour(Vec3::new(0.78, 0.55, 0.6)), false);
    let top = at(0.97);
    kit.cone(
        &sprout,
        top,
        top + place.p(Vec3::X * 0.12) - place.at,
        r * 0.18,
    );
}

/// A jointed rhizome, pinched at each node, rootlets there, a pointed tip.
fn rhizome(build: &Build, place: &Place, n: usize, width: f32, kit: &mut super::Kit) {
    let r = width / 2.0;
    let skin = build.colours.skin;
    let node = build.colours.accent_or(skin * 0.7);
    let joints = 4.0;
    let bend = build.variety.wobble(n, 7) * 0.05;
    let at = |t: f32| place.p(Vec3::new(t - 0.5, r, (PI * 2.0 * t).sin() * bend));
    let half = |t: f32| {
        let pinch = 1.0 - 0.18 * (1.0 - (2.0 * (t * joints).fract() - 1.0).abs()).powi(6);
        let tip = cap(1.0 - t, 0.15).min(cap(t, 0.04));
        Vec2::splat(r * pinch * tip * place.size)
    };
    let colour_at = move |t: f32| match (t * joints).fract() > 0.93 {
        true => node,
        false => skin,
    };
    tube(
        kit,
        &rings(40, at, half, colour_at),
        Round::default(),
        false,
    );
    let fine = kit.material(colour(build.colours.earth), false);
    for k in 1..4 {
        let from = at(k as f32 / joints);
        let side = if k % 2 == 0 { 1.0 } else { -1.0 };
        let to = from + place.p(Vec3::new(-0.04, -0.01, side * (r + 0.07))) - place.at;
        kit.rod(&fine, from, to, 0.005 * place.size);
    }
}

/// A length of yam: rough skin, a round end at -x and a cut face at +x.
fn length(build: &Build, place: &Place, width: f32, kit: &mut super::Kit) {
    let r = width / 2.0;
    let skin = build.colours.skin;
    let at = |t: f32| place.p(Vec3::new(t - 0.5, r, 0.0));
    let half = |t: f32| {
        let rough = 1.0 + 0.04 * (t * 37.0).sin();
        let ends = cap(t, 0.15) * if t > 0.995 { 0.0 } else { 1.0 };
        Vec2::splat(r * rough * ends * place.size)
    };
    let colour_at = move |t: f32| skin * (1.0 - 0.1 * f32::from((t * 11.0).fract() < 0.25));
    tube(
        kit,
        &rings(36, at, half, colour_at),
        Round::default(),
        false,
    );
    let flesh = kit.material(colour(build.colours.inside), false);
    let face = at(0.99);
    let half = Vec3::new(0.01, r * 0.95, r * 0.95) * place.size;
    kit.blob(&flesh, face, half, (place.yaw, 0.0));
}
