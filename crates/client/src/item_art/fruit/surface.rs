//! What is on a fruit's skin: speckles, spikes, hairs, bumps, scales, cactus
//! tufts, drupelets or stripes, spread evenly over the half that shows.

use bevy::prelude::*;

use super::super::herb::kit::Kit;
use super::body::Placed;
use super::colour::{Colours, colour};
use super::group::Surface;
use super::variety::Variety;

/// `count` directions spread evenly over a sphere (a Fibonacci sphere), less
/// those underneath that never show.
pub(super) fn spread(count: usize) -> impl Iterator<Item = Vec3> {
    (0..count).filter_map(move |n| {
        let y = 1.0 - (n as f32 + 0.5) / count as f32 * 2.0;
        let ring = (1.0 - y * y).sqrt();
        let a = n as f32 * 2.399_963;
        let d = Vec3::new(a.cos() * ring, y, a.sin() * ring);
        (y > -0.15).then_some(d)
    })
}

/// Puts `surface` on the fruit at `placed`, its `piece`th in the item.
pub(super) fn dress(
    surface: Surface,
    placed: &Placed,
    colours: &Colours,
    variety: &Variety,
    piece: usize,
    kit: &mut Kit,
) {
    let size = placed.least();
    let skin = colours.skin;
    let darker = kit.material(colour(skin * 0.6), false);
    let paler = kit.material(colour(skin.lerp(Vec3::new(0.95, 0.9, 0.75), 0.6)), false);
    let same = kit.material(colour(skin), !colours.matte);
    match surface {
        Surface::Plain => {}
        Surface::Speckled => {
            let count = variety.speckles * 3;
            for (n, d) in spread(count).enumerate() {
                if variety.wobble(piece * 50 + n, 9) < 0.0 {
                    continue;
                }
                let speck = Vec3::new(1.0, 0.25, 1.0) * size * 0.05;
                spot(kit, &paler, placed, d, speck);
            }
        }
        Surface::Spikes => {
            for d in spread(110) {
                let base = placed.on(d * 0.97);
                let tip = base + placed.normal(d) * size * 0.2;
                kit.cone(&same, base, tip, size * 0.08);
            }
        }
        Surface::Hairs => {
            let tips = kit.material(colour(skin.lerp(Vec3::new(0.7, 0.75, 0.2), 0.5)), false);
            for (n, d) in spread(70).enumerate() {
                let base = placed.on(d * 0.97);
                let bend = Vec3::new(variety.wobble(n, 10), 0.0, variety.wobble(n, 11)) * 0.3;
                let tip = base + (placed.normal(d) + bend) * size * 0.5;
                let material = if n % 2 == 0 { &tips } else { &same };
                kit.cone(material, base, tip, size * 0.05);
            }
        }
        Surface::Bumps => {
            for d in spread(120) {
                spot(kit, &same, placed, d, Vec3::new(1.0, 0.5, 1.0) * size * 0.1);
            }
        }
        Surface::Scales => {
            // A pineapple's eyes: golden scales, each with a dark point, over a
            // browner skin between them (`body` draws it darker).
            let scale = kit.material(colour(skin.lerp(Vec3::new(0.95, 0.75, 0.25), 0.4)), false);
            for (n, d) in spread(56).enumerate() {
                spot(
                    kit,
                    &scale,
                    placed,
                    d,
                    Vec3::new(1.0, 0.45, 1.0) * size * 0.17,
                );
                if n % 2 == 0 {
                    spot(kit, &darker, placed, d * 1.04, Vec3::splat(size * 0.04));
                }
            }
        }
        Surface::Areoles => {
            let spine = kit.material(Color::srgb(0.9, 0.85, 0.6), false);
            for d in spread(26) {
                spot(
                    kit,
                    &paler,
                    placed,
                    d,
                    Vec3::new(1.0, 0.4, 1.0) * size * 0.07,
                );
                let base = placed.on(d);
                kit.cone(
                    &spine,
                    base,
                    base + placed.normal(d) * size * 0.12,
                    size * 0.02,
                );
            }
        }
        Surface::Drupelets => {
            for d in spread(34) {
                kit.blob(
                    &same,
                    placed.on(d * 0.86),
                    Vec3::splat(size * 0.3),
                    (0.0, 0.0),
                );
            }
        }
        Surface::Stripes => {
            let band = kit.material(colour(skin * 0.55), !colours.matte);
            for k in 0..7 {
                let a = k as f32 / 7.0 * std::f32::consts::PI + variety.wobble(k, 12) * 0.1;
                let rotation = placed.rotation() * Quat::from_rotation_x(a);
                let scale = placed.half * Vec3::new(1.01, 1.01, 0.22);
                let transform = Transform::from_translation(placed.centre)
                    .with_rotation(rotation)
                    .with_scale(scale);
                kit.mesh(Sphere::new(1.0).mesh().uv(16, 10), &band, transform);
            }
        }
    }
}

/// A small blob sitting on the skin at direction `d`, lying along it.
fn spot(kit: &mut Kit, material: &Handle<StandardMaterial>, placed: &Placed, d: Vec3, scale: Vec3) {
    let normal = placed.normal(d.normalize());
    let rotation = Quat::from_rotation_arc(Vec3::Y, normal);
    let transform = Transform::from_translation(placed.on(d))
        .with_rotation(rotation)
        .with_scale(scale);
    kit.mesh(Sphere::new(1.0).mesh().uv(8, 6), material, transform);
}
