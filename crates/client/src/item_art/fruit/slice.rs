//! A serving cut from a big fruit, showing its inside: a wedge with its rind
//! (watermelon, melon, papaya, breadfruit), a ring of pineapple, half a
//! coconut, or a durian's or jackfruit's flesh. Fruits not named here are a
//! plain wedge, paler inside.

use bevy::prelude::*;
use shared::fruit::FruitDef;

use super::super::herb::kit::Kit;
use super::body::{Placed, body};
use super::colour::{Colours, colour};
use super::group::{Body, Style, Surface, Top};
use super::surface::dress;
use super::variety::Variety;

/// A wedge's colours: flesh, a pale band under the rind, seeds.
struct Wedge {
    flesh: Vec3,
    pith: Option<Vec3>,
    seeds: Option<(Vec3, Seeds)>,
}

#[derive(Clone, Copy)]
enum Seeds {
    /// Scattered through the flesh (watermelon).
    Scattered,
    /// Heaped in the middle, along the cut (papaya, melon).
    Middle,
}

const fn srgb(red: f32, green: f32, blue: f32) -> Vec3 {
    Vec3::new(red, green, blue)
}

/// A serving of `def`, filling `room`.
pub(super) fn slice(
    def: &FruitDef,
    colours: &Colours,
    variety: &Variety,
    room: Vec2,
    kit: &mut Kit,
) {
    // A wedge is twice as wide as it is long.
    let size = room.x.min(room.y * 1.9).min(room.y.max(room.x)) * variety.size;
    let wedge = match def.key.as_str() {
        "watermelon" => Wedge {
            flesh: srgb(0.87, 0.24, 0.27),
            pith: Some(srgb(0.92, 0.94, 0.8)),
            seeds: Some((srgb(0.08, 0.06, 0.05), Seeds::Scattered)),
        },
        "melon" => Wedge {
            flesh: srgb(0.95, 0.63, 0.32),
            pith: Some(srgb(0.8, 0.85, 0.6)),
            seeds: Some((srgb(0.95, 0.9, 0.75), Seeds::Middle)),
        },
        "papaya" => Wedge {
            flesh: srgb(0.94, 0.45, 0.25),
            pith: None,
            seeds: Some((srgb(0.08, 0.07, 0.06), Seeds::Middle)),
        },
        "breadfruit" => Wedge {
            flesh: srgb(0.95, 0.91, 0.72),
            pith: None,
            seeds: None,
        },
        "pineapple" => return ring(colours, size, kit),
        "coconut" => return coconut(size, kit),
        "durian" => return arils(colours, variety, size, true, kit),
        "jackfruit" => return arils(colours, variety, size, false, kit),
        _ => Wedge {
            flesh: colours.skin.lerp(srgb(0.97, 0.93, 0.8), 0.55),
            pith: None,
            seeds: None,
        },
    };
    wedge_parts(&wedge, colours, variety, size, kit);
}

/// A half disc lying cut face up, `depth` thick, its straight edge along x.
fn half_disc(kit: &mut Kit, material: &Handle<StandardMaterial>, radius: f32, depth: f32, z: f32) {
    let sector = CircularSector::new(radius, std::f32::consts::FRAC_PI_2);
    let mesh = Mesh::from(Extrusion::new(sector, depth));
    let transform = Transform::from_xyz(0.0, depth / 2.0, z)
        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
    kit.mesh(mesh, material, transform);
}

fn wedge_parts(wedge: &Wedge, colours: &Colours, variety: &Variety, size: f32, kit: &mut Kit) {
    let r = size * 0.95;
    let z = r * 0.45;
    let depth = size * 0.22;
    let rind = kit.material(colour(colours.skin), true);
    half_disc(kit, &rind, r, depth, z);
    if let Some(pith) = wedge.pith {
        let pith = kit.material(colour(pith), false);
        half_disc(kit, &pith, r * 0.93, depth + 0.01, z);
    }
    let flesh = kit.material(colour(wedge.flesh), true);
    let flesh_r = if wedge.pith.is_some() { 0.86 } else { 0.9 };
    half_disc(kit, &flesh, r * flesh_r, depth + 0.02, z);
    let Some((seed, seeds)) = wedge.seeds else {
        return;
    };
    let seed = kit.material(colour(seed), true);
    let top = depth + 0.02;
    let count = match seeds {
        Seeds::Scattered => 9,
        Seeds::Middle => 16,
    };
    for n in 0..count {
        let (a, out) = match seeds {
            Seeds::Scattered => (0.3 + n as f32 * 0.29, 0.5 + variety.wobble(n, 50) * 0.12),
            Seeds::Middle => (0.15 + n as f32 * 0.18, 0.08 + (n % 3) as f32 * 0.07),
        };
        let at = Vec3::new(a.cos() * r * out, top, z - a.sin() * r * out);
        let scale = Vec3::new(size * 0.05, size * 0.025, size * 0.035);
        kit.blob(&seed, at, scale, (a, 0.0));
    }
}

/// A ring of pineapple: rind, flesh and its woody core.
fn ring(colours: &Colours, size: f32, kit: &mut Kit) {
    let r = size * 0.92;
    let depth = size * 0.2;
    let layers = [
        (colours.skin * 0.7, 1.0, 0.0),
        (srgb(0.97, 0.84, 0.36), 0.9, 0.01),
        (srgb(0.95, 0.9, 0.66), 0.24, 0.02),
    ];
    for (shade, share, above) in layers {
        let material = kit.material(colour(shade), true);
        let transform = Transform::from_xyz(0.0, (depth + above) / 2.0, 0.0);
        kit.mesh(
            Cylinder::new(r * share, depth + above).mesh().build(),
            &material,
            transform,
        );
    }
}

/// Half a coconut, cut face up: brown shell, white meat, the hollow.
fn coconut(size: f32, kit: &mut Kit) {
    let r = size * 0.9;
    let shell = kit.material(Color::srgb(0.43, 0.28, 0.16), false);
    kit.blob(&shell, Vec3::ZERO, Vec3::new(r, r * 0.28, r), (0.0, 0.0));
    let layers = [
        (srgb(0.43, 0.28, 0.16), 1.0, 0.0),
        (srgb(0.98, 0.97, 0.94), 0.9, 0.01),
        (srgb(0.83, 0.82, 0.78), 0.6, 0.02),
    ];
    for (shade, share, above) in layers {
        let material = kit.material(colour(shade), share < 0.95);
        let height = r * 0.3 + above;
        let transform = Transform::from_xyz(0.0, height / 2.0, 0.0);
        kit.mesh(
            Cylinder::new(r * share, height).mesh().build(),
            &material,
            transform,
        );
    }
}

/// A durian's segment of spiky rind holding two pods of flesh, or a few of a
/// jackfruit's yellow bulbs.
fn arils(colours: &Colours, variety: &Variety, size: f32, rind: bool, kit: &mut Kit) {
    let flesh = match rind {
        true => kit.material(Color::srgb(0.95, 0.85, 0.45), true),
        false => kit.material(Color::srgb(0.96, 0.76, 0.25), true),
    };
    if rind {
        let style = Style {
            body: Body::Oval,
            surface: Surface::Spikes,
            top: Top::Bare,
            count: 1,
            length: 2.0,
        };
        let placed = Placed::new(&style, Vec3::new(0.0, -size * 0.25, 0.0), size * 0.82, 0.0);
        body(&placed, colours, variety, kit);
        dress(Surface::Spikes, &placed, colours, variety, 0, kit);
    }
    let (count, y) = if rind { (2, size * 0.25) } else { (4, 0.0) };
    for n in 0..count {
        let x = (n as f32 - (count as f32 - 1.0) / 2.0) * size * if rind { 0.62 } else { 0.42 };
        let z = if rind {
            0.0
        } else {
            ((n % 2) as f32 - 0.5) * size * 0.4
        };
        let at = Vec3::new(x, y + size * 0.22, z);
        let scale = Vec3::new(size * 0.3, size * 0.2, size * 0.24);
        kit.blob(&flesh, at, scale, (variety.wobble(n, 51) * 0.6, 0.0));
    }
}
