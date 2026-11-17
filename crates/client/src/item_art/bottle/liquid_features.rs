//! What moves in a liquid, stilled for its picture: Agility's fine fizz rising
//! in threads, Strength's big bubbles, Mana's and Aura's sparkles, Intellect's
//! swirl of the second colour, and the lava-lamp blobs of a potion that both
//! helps and harms. All placed from the liquid's look alone.

use bevy::prelude::*;
use shared::rng::Rng;

use super::front::Front;
use super::liquid::LiquidLook;
use super::spheres::Ball;

/// What a set of balls is, which decides its material.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Feature {
    Fizz,
    Bubbles,
    Sparkles,
    Swirl,
    Blobs,
}

/// The liquid's features, each as balls in the bottle's space.
pub(super) fn features(look: &LiquidLook, front: &Front) -> Vec<(Feature, Vec<Ball>)> {
    let mut rng = look.seed().rng();
    let mut out = Vec::new();
    let size = front.span().max_element();
    if look.blobs.is_some() {
        out.push((Feature::Blobs, blobs(front, &mut rng, size)));
    }
    if look.swirl {
        out.push((Feature::Swirl, swirl(front, &mut rng, size)));
    }
    if look.bubbles {
        out.push((Feature::Bubbles, bubbles(front, &mut rng, size)));
    }
    if look.fizz {
        out.push((Feature::Fizz, fizz(front, &mut rng, size)));
    }
    if look.sparkles {
        out.push((Feature::Sparkles, sparkles(front, &mut rng, size)));
    }
    out
}

/// A share of the way across and up, kept off the edges.
fn somewhere(rng: &mut Rng) -> Vec2 {
    Vec2::new(
        rng.range_f64(0.15, 0.85) as f32,
        rng.range_f64(0.1, 0.9) as f32,
    )
}

/// A few big soft blobs, taller than wide.
fn blobs(front: &Front, rng: &mut Rng, size: f32) -> Vec<Ball> {
    let (_, up) = front.axes();
    let turn = Quat::from_rotation_arc(Vec3::Y, up);
    (0..4)
        .filter_map(|_| {
            let radius = size * rng.range_f64(0.08, 0.13) as f32;
            let at = front.at(somewhere(rng), radius)?;
            Some(Ball {
                at,
                turn,
                size: Vec3::new(radius, radius * 1.35, radius * 0.6),
            })
        })
        .collect()
}

/// A ribbon curling up through the liquid.
fn swirl(front: &Front, rng: &mut Rng, size: f32) -> Vec<Ball> {
    let radius = size * 0.035;
    let phase = rng.range_f64(0.0, std::f64::consts::TAU) as f32;
    let steps = 48;
    (0..steps)
        .filter_map(|k| {
            let s = k as f32 / (steps - 1) as f32;
            let across = 0.5 + 0.32 * (s * 7.5 + phase).sin();
            let up = 0.08 + 0.84 * s;
            let thick = radius * (0.5 + 0.5 * (s * std::f32::consts::PI).sin());
            front
                .at(Vec2::new(across, up), radius)
                .map(|at| Ball::round(at, thick))
        })
        .collect()
}

/// Big bubbles, more of them near the top.
fn bubbles(front: &Front, rng: &mut Rng, size: f32) -> Vec<Ball> {
    (0..7)
        .filter_map(|_| {
            let radius = size * rng.range_f64(0.05, 0.09) as f32;
            let mut share = somewhere(rng);
            share.y = share.y.sqrt().min(0.88);
            front.at(share, radius).map(|at| Ball::round(at, radius))
        })
        .collect()
}

/// Threads of tiny bubbles rising, closer together near the top.
fn fizz(front: &Front, rng: &mut Rng, size: f32) -> Vec<Ball> {
    let radius = size * 0.018;
    let mut balls = Vec::new();
    for _ in 0..5 {
        let across = rng.range_f64(0.15, 0.85) as f32;
        let mut up = rng.range_f64(0.0, 0.3) as f32;
        while up < 0.95 {
            let wobble = rng.range_f64(-0.02, 0.02) as f32;
            if let Some(at) = front.at(Vec2::new(across + wobble, up), radius) {
                balls.push(Ball::round(at, radius * (0.7 + up * 0.6)));
            }
            up += 0.12 - up * 0.08;
        }
    }
    balls
}

/// Points of light, each a dot crossed by two slivers.
fn sparkles(front: &Front, rng: &mut Rng, size: f32) -> Vec<Ball> {
    let (right, up) = front.axes();
    let across = Quat::from_rotation_arc(Vec3::X, right);
    let upright = Quat::from_rotation_arc(Vec3::X, up);
    let mut balls = Vec::new();
    for _ in 0..9 {
        let Some(at) = front.at(somewhere(rng), 0.0) else {
            continue;
        };
        let radius = size * rng.range_f64(0.012, 0.022) as f32;
        let long = Vec3::new(radius * 3.2, radius * 0.35, radius * 0.35);
        balls.push(Ball::round(at, radius));
        balls.push(Ball {
            at,
            turn: across,
            size: long,
        });
        balls.push(Ball {
            at,
            turn: upright,
            size: long,
        });
    }
    balls
}
