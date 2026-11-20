//! A pitted or wrinkled head on a pale stem: a morel's pointed head
//! honeycombed with dark pits between pale ridges (a conical cap), a false
//! morel's lumpy brain of folds.

use bevy::prelude::*;
use shared::fungus::Cap;
use std::f32::consts::PI;

use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};

const ROWS: usize = 18;

pub(super) fn morel(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let pointed = build.def.look.cap == Cap::Conical;
    let stem: Vec<Vec2> = [
        (0.0, 0.0),
        (0.14, 0.0),
        (0.12, 0.2),
        (0.11, 0.42),
        (0.0, 0.42),
    ]
    .into_iter()
    .map(|(r, y)| Vec2::new(r, y))
    .collect();
    let stem = lathe(&stem, false, |_, _| Round {
        out: 1.0,
        colour: c.stem,
    });
    let tall = if pointed { 0.75 } else { 0.5 };
    let head: Vec<Vec2> = (0..=ROWS)
        .map(|k| {
            let t = k as f32 / ROWS as f32;
            let width = match pointed {
                true => (t * PI).sin().max(0.0).powf(0.5) * (1.0 - t * 0.5),
                false => (t * PI).sin().max(0.0).powf(0.4),
            };
            Vec2::new(0.24 * width, 0.36 + t * tall)
        })
        .collect();
    let phase = build.variety.wobble(n, 1) * 3.0;
    let head = lathe(&head, false, |row, i| {
        let a = angle(i);
        let (out, pit) = match pointed {
            true => {
                let ridge = row.is_multiple_of(3) || (i + (row / 3) * 3).is_multiple_of(6);
                (if ridge { 1.04 } else { 0.97 }, !ridge)
            }
            false => {
                let fold = (4.0 * a + phase).sin() * (row as f32 * 0.9).cos();
                (1.0 + 0.16 * fold, fold < -0.2)
            }
        };
        let colour = match pit {
            true => c.body * 0.5,
            false => c.body * 1.1,
        };
        Round { out, colour }
    });
    vec![FungusPiece::new(stem), FungusPiece::new(head)]
}
