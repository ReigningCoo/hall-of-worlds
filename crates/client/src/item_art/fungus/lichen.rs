//! Lichens: a cushion of forking sprigs (reindeer lichen; tiny crumbs for the
//! smallest), hanging beards from a twig, and flat leafy plates overlapping.

use bevy::prelude::*;
use std::f32::consts::TAU;

use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};
use super::paint::coloured;

/// A rod of this radius and colour from `from` to `to`.
fn rod(from: Vec3, to: Vec3, radius: f32, colour: Vec3) -> Option<FungusPiece> {
    let length = from.distance(to);
    if length <= f32::EPSILON {
        return None;
    }
    let mesh = coloured(Cylinder::new(radius, length).mesh().resolution(6), colour);
    let turn = Quat::from_rotation_arc(Vec3::Y, (to - from) / length);
    Some(
        FungusPiece::new(mesh)
            .at(Transform::from_translation((from + to) / 2.0).with_rotation(turn)),
    )
}

fn blob(at: Vec3, size: Vec3, colour: Vec3) -> FungusPiece {
    let mesh = coloured(Sphere::new(1.0).mesh().uv(8, 5), colour);
    FungusPiece::new(mesh).at(Transform::from_translation(at).with_scale(size))
}

pub(super) fn tufts(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let v = build.variety;
    let crumbs = build.def.look.largest <= 20;
    let mut pieces = Vec::new();
    for k in 0..24 {
        let a = k as f32 * 2.399_963 + v.wobble(n, 1);
        let out = (k as f32 / 24.0).sqrt() * 0.42;
        let foot = Vec3::new(a.cos() * out, 0.0, a.sin() * out);
        let shade = c.body * (0.8 + 0.3 * v.share(k, 2));
        if crumbs {
            pieces.push(blob(
                foot + Vec3::Y * 0.04,
                Vec3::splat(0.05 + 0.03 * v.share(k, 3)),
                shade,
            ));
            continue;
        }
        let tall = 0.25 + 0.15 * v.share(k, 4) - out * 0.2;
        let lean = Vec3::new(a.cos(), 0.0, a.sin()) * out * 0.3;
        let top = foot + Vec3::Y * tall + lean;
        pieces.extend(rod(foot, top, 0.018, shade * 0.9));
        for b in 0..3 {
            let turn = b as f32 / 3.0 * TAU + v.wobble(k, 5);
            let tip = top + Vec3::new(turn.cos() * 0.06, 0.06, turn.sin() * 0.06);
            pieces.extend(rod(top, tip, 0.012, shade));
            pieces.push(blob(tip, Vec3::splat(0.016), shade * 1.05));
        }
    }
    pieces
}

/// Strands hanging from a twig along x, down -y.
pub(super) fn beard(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let v = build.variety;
    let bark = Vec3::new(0.35, 0.26, 0.18);
    let mut pieces: Vec<_> = rod(
        Vec3::new(-0.45, 0.0, 0.0),
        Vec3::new(0.45, 0.0, 0.0),
        0.035,
        bark,
    )
    .into_iter()
    .collect();
    for k in 0..18 {
        let x = -0.38 + 0.76 * (k as f32 + 0.5) / 18.0;
        let mut at = Vec3::new(x, 0.0, v.wobble(n * 20 + k, 1) * 0.03);
        let length = 0.5 + 0.4 * v.share(n * 20 + k, 2);
        let shade = c.body * (0.8 + 0.3 * v.share(k, 3));
        for s in 0..5 {
            let sway = Vec3::new(
                v.wobble(k, 4 + s) * 0.05,
                -length / 5.0,
                v.wobble(k, 10 + s) * 0.03,
            );
            pieces.extend(rod(at, at + sway, 0.01, shade));
            at += sway;
        }
    }
    pieces
}

/// Leafy lobes overlapping in a rosette, a little cupped.
pub(super) fn plates(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let v = build.variety;
    let profile = [
        Vec2::new(0.0, 0.0),
        Vec2::new(0.25, 0.01),
        Vec2::new(0.4, 0.04),
        Vec2::new(0.42, 0.06),
        Vec2::new(0.3, 0.05),
        Vec2::new(0.0, 0.03),
    ];
    (0..6)
        .map(|k| {
            let phase = v.wobble(n * 6 + k, 1) * 3.0;
            let shade = 0.8 + 0.3 * v.share(n * 6 + k, 2);
            let mesh = lathe(&profile, false, |row, i| {
                let vein = (i + row).is_multiple_of(5);
                Round {
                    out: 1.0 + 0.22 * (4.0 * angle(i) + phase).sin(),
                    colour: c.body * shade * if vein { 0.8 } else { 1.0 },
                }
            });
            let a = k as f32 * 2.399_963;
            let out = if k == 0 { 0.0 } else { 0.28 };
            let at = Vec3::new(a.cos() * out, k as f32 * 0.012, a.sin() * out);
            let size = if k == 0 { 1.0 } else { 0.75 };
            FungusPiece::new(mesh).at(Transform::from_translation(at).with_scale(Vec3::splat(size)))
        })
        .collect()
}
