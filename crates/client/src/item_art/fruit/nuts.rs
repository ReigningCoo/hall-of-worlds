//! Nuts, as they are picked up: in their shells (or an acorn in its cup, a
//! coconut in its fibre), a handful heaped or one alone. Their colours are the
//! shells', named here, as the table's ripe colour is often the husk's.

use bevy::prelude::*;
use shared::fruit::{FruitDef, Unit};
use shared::herb::Rgb;

use super::super::herb::kit::Kit;
use super::colour::{colour, rgb};
use super::group::NutKind;
use super::heap::places;
use super::surface::spread;
use super::variety::Variety;

/// Shells' colours by nut; others brown.
const SHELLS: [(&str, Rgb); 22] = [
    ("walnut", Rgb(165, 125, 80)),
    ("hickory_nut", Rgb(205, 180, 140)),
    ("pecan", Rgb(135, 82, 45)),
    ("almond", Rgb(190, 145, 95)),
    ("pili_nut", Rgb(110, 75, 45)),
    ("mongongo_nut", Rgb(170, 130, 80)),
    ("hazelnut", Rgb(150, 95, 50)),
    ("beaked_hazelnut", Rgb(160, 110, 65)),
    ("macadamia", Rgb(125, 85, 50)),
    ("candlenut", Rgb(95, 80, 62)),
    ("shea_nut", Rgb(95, 60, 35)),
    ("kola_nut", Rgb(185, 85, 80)),
    ("acorn", Rgb(150, 100, 45)),
    ("sweet_chestnut", Rgb(110, 55, 28)),
    ("chinquapin", Rgb(95, 58, 32)),
    ("beechnut", Rgb(130, 85, 55)),
    ("brazil_nut", Rgb(85, 55, 35)),
    ("pistachio", Rgb(225, 205, 160)),
    ("cashew", Rgb(235, 220, 180)),
    ("siberian_pine_nut", Rgb(115, 72, 45)),
    ("stone_pine_nut", Rgb(232, 215, 180)),
    ("coconut", Rgb(110, 72, 40)),
];

/// How many nuts make one item.
fn count(def: &FruitDef) -> usize {
    if def.unit != Unit::Handful {
        return 1;
    }
    // The first size it is no bigger than.
    let sizes = [(15, 10), (25, 6), (40, 5), (60, 4)];
    let size = def.look.size;
    sizes
        .iter()
        .find(|(most, _)| size <= *most)
        .map_or(3, |&(_, count)| count)
}

/// A handful of nuts, or one.
pub(super) fn nuts(def: &FruitDef, kind: NutKind, variety: &Variety, room: Vec2, kit: &mut Kit) {
    let shell = SHELLS
        .iter()
        .find(|(key, _)| *key == def.key)
        .map_or(Vec3::new(0.5, 0.33, 0.2), |(_, shell)| rgb(*shell));
    let shell = shell * (1.0 + variety.tint * 0.08);
    let places = places(count(def));
    let spread = places.iter().map(|at| at.length()).fold(0.0, f32::max);
    let reach = room.min_element() / (spread + 1.15) * variety.size;
    for (n, at) in places.iter().enumerate() {
        let r = reach * (0.92 + variety.wobble(n, 40) * 0.08);
        let yaw = variety.wobble(n, 41) * std::f32::consts::PI;
        let lift = if n * 4 < places.len() && places.len() > 3 {
            r * 0.45
        } else {
            0.0
        };
        let centre = Vec3::new(at.x * reach, lift, at.y * reach);
        nut(kind, shell, centre, r, yaw, kit);
    }
}

/// One nut of `kind` reaching `r` from `centre` along its length.
fn nut(kind: NutKind, shell: Vec3, centre: Vec3, r: f32, yaw: f32, kit: &mut Kit) {
    let glossy = matches!(kind, NutKind::Chestnut | NutKind::Round | NutKind::Acorn);
    let skin = kit.material(colour(shell), glossy);
    let pale = kit.material(colour(shell.lerp(Vec3::new(0.92, 0.86, 0.7), 0.6)), false);
    let dark = kit.material(colour(shell * 0.55), false);
    let turn = Quat::from_rotation_y(yaw);
    let at = |x: f32, y: f32, z: f32| centre + turn * Vec3::new(x, y, z) * r;
    let half = |x: f32, y: f32, z: f32| Vec3::new(x, y, z) * r;
    match kind {
        NutKind::Round => {
            kit.blob(&skin, at(0.0, 0.8, 0.0), half(0.95, 0.8, 0.9), (yaw, 0.0));
            kit.blob(
                &pale,
                at(-0.55, 0.75, 0.0),
                half(0.4, 0.55, 0.55),
                (yaw, 0.0),
            );
        }
        NutKind::Walnut => {
            kit.blob(&skin, at(0.0, 0.85, 0.0), half(1.0, 0.85, 0.9), (yaw, 0.0));
            kit.blob(&skin, at(0.0, 1.55, 0.0), half(0.95, 0.12, 0.1), (yaw, 0.0));
            for d in spread(22) {
                let on = at(d.x * 0.95, 0.85 + d.y * 0.83, d.z * 0.88);
                kit.blob(&skin, on, Vec3::splat(r * 0.16), (0.0, 0.0));
            }
        }
        NutKind::Long | NutKind::Triangular => {
            let (y, z) = if kind == NutKind::Long {
                (0.5, 0.6)
            } else {
                (0.45, 0.5)
            };
            kit.blob(&skin, at(0.0, y, 0.0), half(0.9, y, z), (yaw, 0.0));
            kit.cone(&skin, at(0.75, y, 0.0), at(1.05, y, 0.0), r * 0.22);
            let ridge = if kind == NutKind::Long { &dark } else { &skin };
            kit.blob(
                ridge,
                at(0.0, y * 1.85, 0.0),
                half(0.75, 0.08, 0.08),
                (yaw, 0.0),
            );
        }
        NutKind::Acorn => {
            kit.blob(&skin, at(0.15, 0.6, 0.0), half(0.85, 0.6, 0.6), (yaw, 0.0));
            kit.cone(&dark, at(0.95, 0.6, 0.0), at(1.08, 0.6, 0.0), r * 0.07);
            let cup = kit.material(colour(Vec3::new(0.5, 0.42, 0.28)), false);
            kit.blob(
                &cup,
                at(-0.5, 0.62, 0.0),
                half(0.42, 0.68, 0.68),
                (yaw, 0.0),
            );
        }
        NutKind::Chestnut => {
            kit.blob(&skin, at(0.0, 0.5, 0.0), half(0.85, 0.5, 0.95), (yaw, 0.0));
            kit.blob(
                &pale,
                at(-0.5, 0.48, 0.0),
                half(0.38, 0.45, 0.85),
                (yaw, 0.0),
            );
            kit.cone(&pale, at(0.8, 0.5, 0.0), at(1.05, 0.55, 0.0), r * 0.1);
        }
        NutKind::Pistachio => {
            kit.blob(&skin, at(0.0, 0.6, 0.0), half(1.0, 0.6, 0.65), (yaw, 0.0));
            let kernel = kit.material(Color::srgb(0.55, 0.68, 0.3), false);
            kit.blob(
                &kernel,
                at(0.2, 0.75, 0.0),
                half(0.55, 0.45, 0.2),
                (yaw, 0.0),
            );
        }
        NutKind::Cashew => {
            for k in 0..5 {
                let a = (k as f32 - 2.0) * 0.45;
                let on = at(a.sin() * 0.8, 0.4, (1.0 - a.cos()) * 0.8 - 0.25);
                kit.blob(&skin, on, half(0.3, 0.38, 0.4), (yaw - a, 0.0));
            }
        }
        NutKind::PineNut => {
            kit.blob(&skin, at(0.0, 0.45, 0.0), half(1.0, 0.45, 0.5), (yaw, 0.0));
            kit.cone(&dark, at(0.85, 0.45, 0.0), at(1.05, 0.45, 0.0), r * 0.12);
        }
        NutKind::Coconut => {
            let fibre = kit.material(colour(shell * 1.25), false);
            kit.blob(&skin, at(0.0, 0.85, 0.0), half(1.0, 0.85, 0.88), (yaw, 0.0));
            for d in spread(60) {
                let base = at(d.x * 0.97, 0.85 + d.y * 0.83, d.z * 0.86);
                let out = turn * Vec3::new(d.x + d.z * 0.6, d.y, d.z - d.x * 0.6);
                kit.rod(&fibre, base, base + out * r * 0.12, r * 0.02);
            }
            for k in 0..3 {
                let a = k as f32 * std::f32::consts::TAU / 3.0;
                let eye = at(0.92, 0.85 + a.cos() * 0.18, a.sin() * 0.18);
                kit.blob(&dark, eye, Vec3::splat(r * 0.09), (0.0, 0.0));
            }
        }
    }
}
