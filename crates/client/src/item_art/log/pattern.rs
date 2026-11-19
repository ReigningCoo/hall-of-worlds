//! A log's bark as a pattern: how raised it is (`relief`, 1 on a ridge or
//! plate, 0 deep in a crack) and its colour (`bark_colour`) at any point on
//! the side, from its `BarkLook`; and its end's colour by how far out
//! (`end_colour`). Value noise wraps round the log so nothing shows a seam.
//! Pictures only, so floats are fine here.

use std::f32::consts::{PI, TAU};

use super::bark::{BarkLook, Marks, Style};

/// Value noise on a lattice `wrap` cells round (in `u`), smooth between
/// points: 0 to 1, the same for the same `seed`.
pub(super) fn noise(seed: u32, u: f32, v: f32, wrap: u32) -> f32 {
    let (cu, cv) = (u.floor(), v.floor());
    let (fu, fv) = (smooth(u - cu), smooth(v - cv));
    let at = |du: f32, dv: f32| {
        let iu = (cu + du).rem_euclid(wrap as f32) as u32;
        hashed(seed, iu, (cv + dv) as i32)
    };
    let below = lerp(at(0.0, 0.0), at(1.0, 0.0), fu);
    let above = lerp(at(0.0, 1.0), at(1.0, 1.0), fu);
    lerp(below, above, fv)
}

fn hashed(seed: u32, u: u32, v: i32) -> f32 {
    let mut h = seed ^ u.wrapping_mul(0x9E37_79B1) ^ (v as u32).wrapping_mul(0x85EB_CA77);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    (h & 0xFFFF) as f32 / 65535.0
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn step(low: f32, high: f32, x: f32) -> f32 {
    smooth(((x - low) / (high - low)).clamp(0.0, 1.0))
}

/// How deep the relief goes, as a share of the radius.
pub(super) fn depth(style: Style) -> f32 {
    match style {
        Style::Smooth => 0.015,
        Style::Furrowed { .. } => 0.11,
        Style::Plated { .. } => 0.07,
        Style::Scaly => 0.035,
        Style::Stringy => 0.04,
        Style::Ringed => 0.05,
        Style::Ribbed => 0.16,
    }
}

/// How raised the bark is at `angle` round (radians) and `along` (metres).
pub(super) fn relief(look: &BarkLook, seed: u32, angle: f32, along: f32) -> f32 {
    let round = angle / TAU;
    let wobble = |scale: f32| noise(seed ^ 7, round * 8.0, along * scale, 8) - 0.5;
    match look.style {
        Style::Smooth => 0.85 + 0.15 * noise(seed, round * 24.0, along * 12.0, 24),
        Style::Furrowed { ridges, twist } => {
            let u = round * ridges + twist * along + 0.7 * wobble(3.0);
            let ridge = 1.0 - (2.0 * u.fract() - 1.0).abs();
            // Ridges broken across now and then.
            let n = ridges as u32;
            let gap = noise(seed ^ 3, u.floor() + 0.5, along * 5.0, n.max(1));
            let broken = 1.0 - 0.8 * step(0.78, 0.86, gap);
            let rough = 0.65 + 0.35 * noise(seed, u * 4.0, along * 14.0, n * 4);
            step(0.12, 0.55, ridge) * broken * rough
        }
        Style::Plated {
            across,
            along: per_metre,
        } => {
            let v = along * per_metre + 0.6 * wobble(2.5);
            let row = v.floor();
            let jog = noise(seed ^ 23, round * across, row * 1.7, across as u32);
            let u = round * across + 0.5 * row + 0.5 * wobble(4.0) + 0.6 * (jog - 0.5);
            let edge = (u.fract().min(1.0 - u.fract())).min(v.fract().min(1.0 - v.fract()) * 0.6);
            step(0.0, 0.1, edge) * (0.75 + 0.25 * noise(seed, u.floor() * 3.1, row, 64))
        }
        Style::Scaly => {
            let v = along * 22.0;
            let u = round * 26.0 + 0.5 * v.floor();
            let (du, dv) = (u.fract() - 0.5, v.fract() - 0.5);
            1.0 - step(0.25, 0.5, (du * du + dv * dv).sqrt())
        }
        Style::Stringy => {
            let u = round * 46.0 + 0.4 * (along * 2.0).sin() + wobble(2.0);
            0.55 + 0.45 * (PI * u).sin().abs().sqrt() * noise(seed, u, along * 4.0, 46)
        }
        Style::Ringed => {
            let v = along * 9.0 + 0.2 * wobble(1.0);
            step(0.0, 0.25, 1.0 - (2.0 * v.fract() - 1.0).abs())
        }
        Style::Ribbed => (0.5 + 0.5 * (TAU * round * 12.0).cos()).powf(0.7),
    }
}

/// Mixes two sRGB colours, `t` of the way to `b`, as linear 0 to 1.
pub(super) fn mix(a: [u8; 3], b: [u8; 3], t: f32) -> [f32; 3] {
    let linear = |c: u8| (f32::from(c) / 255.0).powf(2.2);
    [0, 1, 2].map(|i| lerp(linear(a[i]), linear(b[i]), t.clamp(0.0, 1.0)))
}

/// The bark's colour there, from how raised it is: the crack colour deep
/// down, its own on top, varied a little, with its marks.
pub(super) fn bark_colour(
    look: &BarkLook,
    seed: u32,
    angle: f32,
    along: f32,
    raised: f32,
) -> [f32; 4] {
    let round = angle / TAU;
    let shade = 0.8 + 0.2 * noise(seed ^ 11, round * 30.0, along * 20.0, 30);
    let grain = 0.85 + 0.3 * noise(seed ^ 19, round * 160.0, along * 90.0, 160);
    let shade = shade * grain;
    let mut colour = mix(look.crack, look.colour, step(0.05, 0.7, raised));
    let mark = match look.marks {
        Marks::None => None,
        Marks::Dashes(dash) => {
            let v = along * 14.0;
            let lit = noise(seed ^ 5, round * 10.0, v.floor() * 3.7, 10);
            let thin = (v.fract() - 0.5).abs() < 0.12;
            (thin && lit > 0.6).then_some(dash)
        }
        Marks::Patches(patch) => {
            let blot = noise(seed ^ 9, round * 7.0, along * 5.0, 7);
            (blot > 0.68).then_some(patch)
        }
    };
    if let Some(mark) = mark {
        colour = mix(mark, mark, 0.0);
    }
    let [r, g, b] = colour.map(|c| (c * shade).min(1.0));
    [r, g, b, 1.0]
}

/// The end's colour at `out` (0 at the pith to 1 at the bark) and `angle`:
/// heartwood paling to sapwood towards the rim, growth rings as darker lines
/// (`BarkLook::rings`), or fibre dots thicker towards the rim for a palm, two
/// drying cracks running out, and a darker pith.
pub(super) fn end_colour(
    look: &BarkLook,
    heart: [u8; 3],
    seed: u32,
    angle: f32,
    out: f32,
) -> [f32; 4] {
    let sap = [0, 1, 2].map(|i| heart[i] / 2 + 118);
    let mut colour = mix(heart, sap, step(0.6, 0.8, out));
    let round = angle / TAU;
    let wavy = out + 0.025 * (noise(seed ^ 13, round * 6.0, 0.0, 6) - 0.5);
    let darken = if look.dots {
        let dot = noise(seed ^ 17, round * 40.0, out * 18.0, 40);
        step(0.62 - 0.12 * out, 0.7 - 0.12 * out, dot) * 0.45
    } else {
        let ring = (wavy * 9.0).fract();
        look.rings * step(0.7, 0.9, ring)
    };
    let cracked = [0.13, 0.61].iter().any(|&at| {
        let apart = (round - at).abs().min(1.0 - (round - at).abs());
        out > 0.25 && apart < 0.006 * (1.2 - out)
    });
    let darken = if cracked { 0.6 } else { darken };
    let darken = if out < 0.04 { 0.35 } else { darken };
    colour = colour.map(|c| c * (1.0 - darken));
    [colour[0], colour[1], colour[2], 1.0]
}
