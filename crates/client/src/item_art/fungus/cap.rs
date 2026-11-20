//! A cap on a stem: the stem (slender, tall or stout with a swollen foot), the
//! underside (gills as fine stripes, pores, hanging spines), the cap by its
//! shape (convex, flat, bell, conical, dipped), and a ring, a volva and pale
//! warts where its look has them. The cap is a unit across.

use bevy::prelude::*;
use shared::fungus::{Cap, Stem, Under};
use std::f32::consts::{FRAC_PI_2, PI};

use super::lathe::{AROUND, Round, angle, lathe};
use super::model::{Build, FungusPiece};
use super::paint::{coloured, speckle};

/// Rows up the stem, under the cap and over it.
const STEM_ROWS: usize = 8;
const UNDER_ROWS: usize = 8;
const TOP_ROWS: usize = 14;

/// The cap's shape: its radius, the rim's height against where the stem
/// meets it, and a point on its top from the rim (`s` 0) to the middle (1).
struct Shape {
    radius: f32,
    rim: f32,
    top: fn(f32, f32) -> Vec2,
}

fn shape(cap: Cap) -> Shape {
    match cap {
        Cap::Convex => Shape {
            radius: 0.5,
            rim: -0.03,
            top: |s, r| {
                Vec2::new(
                    r * (s * FRAC_PI_2).cos(),
                    -0.01 + 0.3 * (s * FRAC_PI_2).sin(),
                )
            },
        },
        Cap::Flat => Shape {
            radius: 0.5,
            rim: 0.0,
            top: |s, r| {
                let bump = 0.05 * ((s - 0.8) / 0.2).clamp(0.0, 1.0);
                Vec2::new(
                    r * (1.0 - s),
                    0.03 + 0.07 * (1.0 - (1.0 - s).powi(2)) + bump,
                )
            },
        },
        Cap::Bell => Shape {
            radius: 0.36,
            rim: -0.2,
            top: |s, r| {
                let a = s * FRAC_PI_2;
                Vec2::new(r * a.cos().max(0.0).powf(0.6), -0.18 + 0.62 * a.sin())
            },
        },
        Cap::Conical => Shape {
            radius: 0.34,
            rim: -0.1,
            top: |s, r| Vec2::new(r * (1.0 - s), -0.08 + 0.55 * s.powf(0.8)),
        },
        Cap::Depressed => Shape {
            radius: 0.5,
            rim: 0.06,
            top: |s, r| {
                let roll = 0.04 * (1.0 - (s / 0.12).min(1.0));
                Vec2::new(r * (1.0 - s), 0.1 + 0.08 * (1.0 - s) - roll - 0.04 * s)
            },
        },
    }
}

/// The stem's height, radius and how much wider its foot is.
fn stem(stem: Stem) -> (f32, f32, f32) {
    match stem {
        Stem::Slender => (0.7, 0.085, 1.2),
        Stem::Tall => (1.25, 0.07, 1.6),
        Stem::Stout => (0.55, 0.16, 1.4),
    }
}

/// The stem's radius `t` of the way up.
fn stem_radius(radius: f32, foot: f32, t: f32) -> f32 {
    radius * (1.0 + (foot - 1.0) * (1.0 - t).powi(2))
}

pub(super) fn cap_and_stem(build: &Build, n: usize) -> Vec<FungusPiece> {
    let look = &build.def.look;
    let c = build.colours;
    let shape = shape(look.cap);
    let (height, radius, foot) = stem(look.stem);
    let height = height * (1.0 + build.variety.wobble(n, 1) * 0.12);
    let mut profile = vec![Vec2::ZERO];
    for k in 0..=STEM_ROWS {
        let t = k as f32 / STEM_ROWS as f32;
        profile.push(Vec2::new(stem_radius(radius, foot, t), t * height));
    }
    let under_from = profile.len();
    for k in 1..=UNDER_ROWS {
        let t = k as f32 / UNDER_ROWS as f32;
        let r = radius + (shape.radius - radius) * t;
        profile.push(Vec2::new(r, height + shape.rim * t + 0.03 * (PI * t).sin()));
    }
    let top_from = profile.len();
    for k in 0..=TOP_ROWS {
        let s = k as f32 / TOP_ROWS as f32;
        profile.push((shape.top)(s, shape.radius * 1.02) + Vec2::Y * height);
    }
    let under = look.under;
    let mesh = lathe(&profile, false, |row, i| {
        let colour = if row < under_from {
            let t = row as f32 / under_from as f32;
            c.stem * (0.9 + 0.1 * t)
        } else if row < top_from {
            underside(under, c.under, row, i)
        } else {
            let s = (row - top_from) as f32 / TOP_ROWS as f32;
            c.body * (1.0 - 0.18 * s) * (0.97 + 0.06 * speckle(row, i, 3))
        };
        Round { out: 1.0, colour }
    });
    let mut pieces = vec![FungusPiece::new(mesh)];
    if under == Under::Spines {
        pieces.extend(spines(&profile[under_from..top_from], c.under));
    }
    if look.ring {
        let at = height * 0.8;
        pieces.push(ring(stem_radius(radius, foot, 0.8), at, c.stem * 0.97));
    }
    if look.volva {
        let base = stem_radius(radius, foot, 0.0);
        pieces.push(volva(base, c.stem.lerp(Vec3::ONE, 0.25)));
    }
    if look.warts {
        pieces.extend(warts(build, n, &shape, height));
    }
    pieces
}

/// An underside's colour round row `row`: gills as stripes, ridges fewer and
/// softer, pores speckled.
pub(super) fn underside(under: Under, colour: Vec3, row: usize, i: usize) -> Vec3 {
    match under {
        Under::Gills if i % 2 == 1 => colour * 0.72,
        Under::Ridges if i.is_multiple_of(3) => colour * 0.8,
        Under::Pores => colour * (0.9 + 0.1 * speckle(row, i, 1)),
        _ => colour,
    }
}

/// Teeth hanging from the underside's points.
fn spines(under: &[Vec2], colour: Vec3) -> Vec<FungusPiece> {
    let mut pieces = Vec::new();
    for point in under.iter().skip(2).step_by(2) {
        for i in (0..AROUND).step_by(3) {
            let a = angle(i);
            let at = Vec3::new(point.x * a.cos(), point.y - 0.04, point.x * a.sin());
            let mesh = coloured(Cone::new(0.018, 0.08), colour);
            let turn = Quat::from_rotation_x(PI);
            pieces.push(
                FungusPiece::new(mesh).at(Transform::from_translation(at).with_rotation(turn)),
            );
        }
    }
    pieces
}

/// A skirt hanging round the stem at `at`.
fn ring(stem: f32, at: f32, colour: Vec3) -> FungusPiece {
    let profile = [
        Vec2::new(stem * 0.98, at + 0.01),
        Vec2::new(stem + 0.06, at - 0.04),
        Vec2::new(stem + 0.075, at - 0.075),
        Vec2::new(stem + 0.055, at - 0.07),
        Vec2::new(stem * 0.98, at - 0.025),
    ];
    let mesh = lathe(&profile, true, |_, _| Round { out: 1.0, colour });
    FungusPiece::new(mesh)
}

/// A cup round the stem's foot.
fn volva(foot: f32, colour: Vec3) -> FungusPiece {
    let profile = [
        Vec2::new(foot * 0.98, 0.0),
        Vec2::new(foot * 1.35, 0.01),
        Vec2::new(foot * 1.45, 0.14),
        Vec2::new(foot * 1.3, 0.17),
        Vec2::new(foot * 1.05, 0.06),
    ];
    let mesh = lathe(&profile, true, |row, i| Round {
        out: 1.0 + 0.08 * (speckle(row, i / 6, 7) - 0.5),
        colour,
    });
    FungusPiece::new(mesh)
}

/// Pale patches on the cap's top.
fn warts(build: &Build, n: usize, shape: &Shape, height: f32) -> Vec<FungusPiece> {
    let colour = build.colours.stem.lerp(Vec3::ONE, 0.4);
    (0..14)
        .map(|k| {
            let s = 0.15 + 0.7 * build.variety.share(n * 20 + k, 11);
            let a = build.variety.wobble(n * 20 + k, 12) * PI;
            let point = (shape.top)(s, shape.radius);
            let at = Vec3::new(point.x * a.cos(), height + point.y, point.x * a.sin());
            let mesh = coloured(Sphere::new(1.0).mesh().uv(8, 5), colour);
            let size = Vec3::new(0.045, 0.02, 0.04) * (0.7 + 0.5 * build.variety.share(k, 13));
            FungusPiece::new(mesh).at(Transform::from_translation(at).with_scale(size))
        })
        .collect()
}
