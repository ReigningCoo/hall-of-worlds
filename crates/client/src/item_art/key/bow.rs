//! The bow: what a key is held by, round the origin, and what fills it.

use bevy::prelude::*;

use super::look::{Bow, Fill, KeyLook};
use super::piece::{Piece, Shape};

/// How wide the bow's metal is.
const WIDE: f32 = 0.09;

/// The bow's pieces, and how far along x it reaches towards the shank.
pub(super) fn bow(look: &KeyLook) -> (Vec<Piece>, f32) {
    let r = 0.32 + 0.04 * look.bow_size as f32;
    let lobe = |angle: f32, away: f32, size: f32| {
        Piece::flat_loop(Vec2::from_angle(angle) * r * away, r * size, WIDE * 0.85)
    };
    let corner = |angle: f32| Vec2::from_angle(angle) * r;
    let (mut pieces, reach) = match look.bow {
        Bow::Ring => (vec![Piece::flat_loop(Vec2::ZERO, r, WIDE)], r),
        Bow::Oval => {
            let mut oval = Piece::flat_loop(Vec2::ZERO, r, WIDE);
            oval.at.scale = Vec3::new(0.7, 1.0, 1.05);
            (vec![oval], r * 0.7)
        }
        Bow::Trefoil => {
            let angles = [0.0, 2.1, -2.1].map(|turn: f32| std::f32::consts::PI + turn);
            (
                angles.map(|angle| lobe(angle, 0.48, 0.55)).to_vec(),
                r * 0.5,
            )
        }
        Bow::Quatrefoil => {
            let angles = [0.0, 1.0, 2.0, 3.0].map(|quarter| quarter * std::f32::consts::FRAC_PI_2);
            (
                angles.map(|angle| lobe(angle, 0.46, 0.5)).to_vec(),
                r * 0.96,
            )
        }
        Bow::Heart => {
            let point = Vec2::new(r, 0.0);
            let mut pieces = Vec::new();
            for side in [-1.0, 1.0] {
                pieces.push(Piece::flat_loop(
                    Vec2::new(-0.3 * r, side * 0.45 * r),
                    r * 0.5,
                    WIDE * 0.85,
                ));
                pieces.push(Piece::bar(
                    Vec2::new(-0.15 * r, side * 0.92 * r),
                    point,
                    WIDE * 0.8,
                    WIDE,
                ));
            }
            (pieces, r)
        }
        Bow::Diamond => {
            let corners =
                [0.0, 1.0, 2.0, 3.0].map(|quarter| corner(quarter * std::f32::consts::FRAC_PI_2));
            let sides =
                (0..4).map(|side| Piece::bar(corners[side], corners[(side + 1) % 4], WIDE, WIDE));
            (sides.collect(), r)
        }
        Bow::Scroll => {
            let mut pieces: Vec<Piece> = [-1.0, 1.0]
                .map(|side| {
                    Piece::flat_loop(Vec2::new(-0.2 * r, side * 0.5 * r), r * 0.48, WIDE * 0.8)
                })
                .to_vec();
            pieces.push(Piece::bar(
                Vec2::new(-0.7 * r, 0.0),
                Vec2::new(0.5 * r, 0.0),
                WIDE,
                WIDE,
            ));
            (pieces, r * 0.5)
        }
    };
    match look.fill {
        Fill::Open => {}
        Fill::Bar => pieces.push(Piece::bar(
            Vec2::new(0.0, -0.75 * r),
            Vec2::new(0.0, 0.75 * r),
            WIDE * 0.7,
            WIDE * 0.7,
        )),
        Fill::Boss => pieces.push(Piece::new(
            Shape::Bead(r * 0.2),
            Transform::from_scale(Vec3::new(1.0, 0.5, 1.0)),
        )),
    }
    (pieces, reach)
}
