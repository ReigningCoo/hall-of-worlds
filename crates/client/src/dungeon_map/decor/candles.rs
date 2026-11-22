//! Lights with candles: chandeliers hanging from the ceiling on chains over
//! the middle of their cell (their frame's origin on the floor below), and
//! candles standing on a table (their frame's origin on its top, `x` along
//! the way a pair stands). Sizes are first guesses.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::decor::ChandelierStyle;
use shared::furniture::Candles;

use super::super::{CELL_SIZE, WALL_HEIGHT};
use super::colours::{brass, dark_wood, iron, wax};
use super::model::Model;

const C: f32 = CELL_SIZE;
/// How high a chandelier's candles stand.
const HANG: f32 = WALL_HEIGHT * 0.74;

/// A candle standing on `at`, `height` tall, with its flame.
fn candle(model: &mut Model, at: Vec3, height: f32) {
    model.cylinder(
        at + Vec3::Y * height / 2.0,
        (0.012 * C, height),
        Quat::IDENTITY,
        wax(),
    );
    model.flame(at + Vec3::Y * height, 0.04 * C);
}

/// A cup holding a candle at `at`, in `metal`.
fn cupped(model: &mut Model, at: Vec3, metal: Vec3) {
    model.cylinder(at, (0.022 * C, 0.015 * C), Quat::IDENTITY, metal);
    candle(model, at + Vec3::Y * 0.0075 * C, 0.06 * C);
}

pub(super) fn chandelier(model: &mut Model, style: ChandelierStyle) {
    let top = Vec3::Y * WALL_HEIGHT;
    match style {
        ChandelierStyle::IronHoop => {
            let radius = 0.25 * C;
            let ring = Vec3::Y * HANG;
            model.hoop(ring, (radius, 0.02 * C), iron());
            let hook = Vec3::Y * (HANG + 0.14 * C);
            model.stick((top, hook), 0.01 * C, iron());
            for i in 0..style.candles() {
                let angle = TAU * f32::from(i) / f32::from(style.candles());
                let at = ring + Vec3::new(angle.cos(), 0.0, angle.sin()) * radius;
                if i % 2 == 0 {
                    model.stick((at, hook), 0.006 * C, iron());
                }
                cupped(model, at + Vec3::Y * 0.01 * C, iron());
            }
        }
        ChandelierStyle::WoodenCross => {
            let middle = Vec3::Y * HANG;
            let reach = 0.28 * C;
            model.cuboid(
                middle,
                Vec3::new(2.0 * reach, 0.035 * C, 0.035 * C),
                Quat::IDENTITY,
                dark_wood(),
            );
            model.cuboid(
                middle,
                Vec3::new(0.035 * C, 0.035 * C, 2.0 * reach),
                Quat::IDENTITY,
                dark_wood(),
            );
            let hook = Vec3::Y * (HANG + 0.16 * C);
            model.stick((top, hook), 0.01 * C, iron());
            for end in [Vec3::X, -Vec3::X, Vec3::Z, -Vec3::Z] {
                let at = middle + end * (reach - 0.03 * C);
                model.stick((at, hook), 0.006 * C, iron());
                cupped(model, at + Vec3::Y * 0.025 * C, iron());
            }
        }
        ChandelierStyle::Brass => {
            let foot = Vec3::Y * (HANG - 0.06 * C);
            let crown = Vec3::Y * (HANG + 0.12 * C);
            model.stick((foot, crown), 0.03 * C, brass());
            model.ball(foot, Vec3::splat(0.05 * C), brass());
            model.stick((top, crown), 0.01 * C, iron());
            for (count, radius, at) in [(6u8, 0.22 * C, HANG), (3, 0.11 * C, HANG + 0.09 * C)] {
                for i in 0..count {
                    let angle = TAU * f32::from(i) / f32::from(count) + f32::from(count) * 0.3;
                    let out = Vec3::new(angle.cos(), 0.0, angle.sin()) * radius;
                    let from = Vec3::Y * (at - 0.04 * C);
                    model.stick((from, from + out + Vec3::Y * 0.04 * C), 0.012 * C, brass());
                    cupped(model, Vec3::Y * at + out, brass());
                }
            }
        }
    }
}

/// One candlestick standing at `at` on a table.
fn candlestick(model: &mut Model, at: Vec3) {
    let metal = brass();
    model.cylinder(
        at + Vec3::Y * 0.005 * C,
        (0.04 * C, 0.01 * C),
        Quat::IDENTITY,
        metal,
    );
    model.cylinder(
        at + Vec3::Y * 0.035 * C,
        (0.011 * C, 0.05 * C),
        Quat::IDENTITY,
        metal,
    );
    let cup = at + Vec3::Y * 0.065 * C;
    model.cylinder(cup, (0.02 * C, 0.012 * C), Quat::IDENTITY, metal);
    candle(model, cup + Vec3::Y * 0.006 * C, 0.08 * C);
}

pub(super) fn candles(model: &mut Model, candles: Candles) {
    match candles {
        Candles::Candlestick => candlestick(model, Vec3::ZERO),
        Candles::Pair => {
            for side in [-1.0, 1.0] {
                candlestick(model, Vec3::X * side * 0.12 * C);
            }
        }
        Candles::Candelabrum => {
            let metal = iron();
            model.cylinder(
                Vec3::Y * 0.005 * C,
                (0.05 * C, 0.01 * C),
                Quat::IDENTITY,
                metal,
            );
            model.cylinder(
                Vec3::Y * 0.06 * C,
                (0.012 * C, 0.11 * C),
                Quat::IDENTITY,
                metal,
            );
            let arm = Vec3::Y * 0.1 * C;
            model.cuboid(
                arm,
                Vec3::new(0.24, 0.015, 0.015) * C,
                Quat::IDENTITY,
                metal,
            );
            for (x, rise) in [(-0.11, 0.0), (0.0, 0.03), (0.11, 0.0)] {
                cupped(
                    model,
                    arm + Vec3::new(x * C, rise * C + 0.01 * C, 0.0),
                    metal,
                );
            }
        }
    }
}
