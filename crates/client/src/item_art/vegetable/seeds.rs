//! A handful of seeds heaped: peas and round beans, curved kidney beans
//! (mottled or black-eyed where the usual kind is), lentils' discs, flat lima
//! and broad beans, beaked chickpeas and wedge-shaped grass peas.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Build;
use super::colours::colour;
use super::group::Seed;
use super::heap::{Place, heaped};
use super::tube::{Round, cap, rings, tube};

pub(super) fn seeds(build: &Build, seed: Seed, kit: &mut super::Kit) {
    let c = build.colours;
    let shine = !matches!(seed, Seed::Lens | Seed::Wedge);
    let skins: Vec<_> = (0..3)
        .map(|k| {
            let shade = 1.0 + (k as f32 - 1.0) * 0.07;
            kit.material(colour(c.skin * shade), shine)
        })
        .collect();
    let spots = c.spots.map(|spot| kit.material(colour(spot), shine));
    let places = heaped(build.count, 0.14, build.variety);
    for (n, place) in places.iter().enumerate() {
        let skin = &skins[n % skins.len()];
        let lie = |half: Vec3| {
            (
                place.yaw,
                0.0,
                place.at + Vec3::Y * half.y,
                half * place.size,
            )
        };
        match seed {
            Seed::Round => {
                let (yaw, pitch, at, half) = lie(Vec3::splat(0.12));
                kit.blob(skin, at, half, (yaw, pitch));
            }
            Seed::Lens => {
                let (yaw, pitch, at, half) = lie(Vec3::new(0.13, 0.05, 0.13));
                kit.blob(skin, at, half, (yaw, pitch));
            }
            Seed::Flat => {
                let (yaw, pitch, at, half) = lie(Vec3::new(0.17, 0.06, 0.12));
                kit.blob(skin, at, half, (yaw, pitch));
            }
            Seed::Wedge => {
                let (yaw, pitch, at, half) = lie(Vec3::new(0.12, 0.08, 0.09));
                kit.blob(skin, at, half, (yaw, 0.3 + pitch));
            }
            Seed::Beaked => {
                let (yaw, pitch, at, half) = lie(Vec3::splat(0.12));
                kit.blob(skin, at, half, (yaw, pitch));
                let beak = at + Quat::from_rotation_y(yaw) * Vec3::X * 0.1 * place.size;
                kit.blob(skin, beak, Vec3::splat(0.05 * place.size), (0.0, 0.0));
            }
            Seed::Kidney => kidney(build, place, n, spots.as_ref(), kit),
        }
    }
}

/// A bean curved like a kidney, its eye or mottling in the accent colour.
fn kidney(
    build: &Build,
    place: &Place,
    n: usize,
    spots: Option<&Handle<StandardMaterial>>,
    kit: &mut super::Kit,
) {
    let skin = build.colours.skin * (1.0 + build.variety.wobble(n, 1) * 0.06);
    let r = 0.075;
    let at = |t: f32| place.p(Vec3::new(t * 0.3 - 0.15, r * 0.8, -(PI * t).sin() * 0.05));
    let half = |t: f32| Vec2::new(r, r * 0.75) * cap(t, 0.25).min(cap(1.0 - t, 0.25)) * place.size;
    tube(kit, &rings(12, at, half, |_| skin), Round::default(), true);
    let Some(spot) = spots else {
        return;
    };
    let eye = build.def.key == "cowpea";
    let marks: &[f32] = if eye { &[0.5] } else { &[0.25, 0.55, 0.8] };
    for (k, &t) in marks.iter().enumerate() {
        let side = build.variety.wobble(n * 3 + k, 2) * 0.3;
        let on = at(t) + place.p(Vec3::new(0.0, r * 0.7, side * r)) - place.at;
        let size = if eye {
            0.035
        } else {
            0.03 + build.variety.share(n, 3) * 0.02
        };
        kit.blob(
            spot,
            on,
            Vec3::new(size, 0.012, size * 0.8) * place.size,
            (place.yaw, 0.0),
        );
    }
}
