//! A ball (puffballs, earthballs, lion's mane): a squat sphere pinched at its
//! foot, speckled; an earthball's skin cracked with its second colour; lion's
//! mane hung all over with spines.

use bevy::prelude::*;
use shared::fungus::Under;
use std::f32::consts::{FRAC_PI_2, PI};

use super::lathe::{AROUND, Round, angle, lathe};
use super::model::{Build, FungusPiece};
use super::paint::{coloured, speckle};

const ROWS: usize = 16;

/// The profile of a ball a unit across, `tall` against its width, pinched at
/// its foot.
pub(super) fn ball_profile(tall: f32) -> Vec<Vec2> {
    (0..=ROWS)
        .map(|k| {
            let a = -FRAC_PI_2 + k as f32 / ROWS as f32 * PI;
            let pinch = 1.0 - 0.25 * (-a).max(0.0).powi(2);
            Vec2::new(0.5 * a.cos() * pinch, (a.sin() + 1.0) * 0.5 * tall)
        })
        .collect()
}

pub(super) fn ball(build: &Build, n: usize) -> Vec<FungusPiece> {
    let look = &build.def.look;
    let c = build.colours;
    let cracked = look.colours.len() > 1;
    let profile = ball_profile(0.85 + build.variety.wobble(n, 1) * 0.06);
    let mesh = lathe(&profile, false, |row, i| {
        let spot = speckle(row, i, n as u32);
        let colour = match cracked && spot < 0.3 {
            true => c.body.lerp(c.under, 0.7),
            false => c.body * (0.94 + 0.08 * spot),
        };
        Round {
            out: 1.0 + 0.015 * spot,
            colour,
        }
    });
    let mut pieces = vec![FungusPiece::new(mesh)];
    if look.under == Under::Spines {
        pieces.extend(spines(&profile, c.body));
    }
    pieces
}

/// Spines hanging down from the upper rows.
fn spines(profile: &[Vec2], colour: Vec3) -> Vec<FungusPiece> {
    let mut pieces = Vec::new();
    for (row, point) in profile.iter().enumerate().skip(4).step_by(2) {
        for i in (row % 2..AROUND).step_by(4) {
            let a = angle(i);
            let at = Vec3::new(point.x * a.cos(), point.y - 0.05, point.x * a.sin());
            let mesh = coloured(Cone::new(0.025, 0.16), colour * 0.95);
            let turn = Quat::from_rotation_x(PI);
            pieces.push(
                FungusPiece::new(mesh).at(Transform::from_translation(at).with_rotation(turn)),
            );
        }
    }
    pieces
}
