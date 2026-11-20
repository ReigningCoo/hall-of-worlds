//! A bracket: a shelf, fan or hoof on wood, its top zoned in bands and its
//! underside pored or gilled; thick hooves for the big woody ones. A handful
//! (turkey tail, oysters) is a tier of shelves. Its back, where it grew from
//! the wood, is towards -z.

use bevy::prelude::*;
use shared::fungus::Group;

use super::cap::underside;
use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};

const ROWS: usize = 10;

/// One shelf, `scale` of a unit across.
pub(super) fn shelf(build: &Build, n: usize, scale: f32) -> Vec<FungusPiece> {
    let def = build.def;
    let c = build.colours;
    let hoof = def.group == Group::Bracket && def.look.largest >= 400;
    let thick = if hoof { 0.32 } else { 0.12 };
    let radius = 0.5;
    let mut profile = vec![Vec2::new(0.0, -0.02)];
    for k in 1..=ROWS {
        let t = k as f32 / ROWS as f32;
        profile.push(Vec2::new(radius * t, -0.02 + 0.02 * t));
    }
    let under_rows = profile.len();
    for k in 0..=ROWS {
        let s = k as f32 / ROWS as f32;
        let r = radius * 1.01 * (1.0 - s);
        profile.push(Vec2::new(r, 0.02 + thick * (1.0 - (1.0 - s).powi(2))));
    }
    let wave = build.variety.wobble(n, 3) * 2.0;
    let mesh = lathe(&profile, false, |row, i| {
        let a = angle(i);
        // A fan: full towards +z, narrowing to where it grew at the back.
        let front = (a.sin() * 0.5 + 0.5).powf(0.8);
        let out = (0.3 + 0.7 * front) * (1.0 + 0.05 * (6.0 * a + wave).sin());
        let colour = match row < under_rows {
            true => underside(def.look.under, c.under, row, i),
            false => {
                let r = profile[row].x / radius;
                let zone = (r * 26.0).sin() * 0.1;
                c.body * (0.82 + 0.18 * r + zone)
            }
        };
        Round { out, colour }
    });
    let shift =
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.12)).with_scale(Vec3::splat(scale));
    vec![FungusPiece::new(mesh).at(shift)]
}

/// A cluster of `count` shelves fanning out from one point on the wood, each
/// a little higher and smaller than the last.
pub(super) fn tier(build: &Build, count: usize) -> Vec<FungusPiece> {
    let middle = (count as f32 - 1.0) / 2.0;
    let mut pieces = Vec::new();
    for n in 0..count {
        let spread = (n as f32 - middle) * 0.55 + build.variety.wobble(n, 4) * 0.1;
        let scale = 1.0 - (n as f32 - middle).abs() * 0.12;
        let at = Vec3::new(spread.sin(), 0.0, spread.cos()) * 0.12 + Vec3::Y * n as f32 * 0.05;
        let place = Transform::from_translation(at).with_rotation(Quat::from_rotation_y(spread));
        pieces.extend(
            shelf(build, n, scale)
                .into_iter()
                .map(|piece| piece.moved(place)),
        );
    }
    pieces
}
