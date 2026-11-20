//! What fungi grow on or under, drawn with them: a stub of wood for brackets,
//! a fallen stick for jelly and crusts on wood, a dead snag with twigs for
//! hanging beards, and the disturbed mound over a truffle (the truffle itself
//! never shows).

use bevy::prelude::*;

/// Bark, a cut's wood, earth turned over and its darker crumbs (sRGB).
const BARK: Vec3 = Vec3::new(0.30, 0.22, 0.15);
const CUT: Vec3 = Vec3::new(0.70, 0.56, 0.38);
const EARTH: Vec3 = Vec3::new(0.36, 0.26, 0.17);
const CRUMB: Vec3 = Vec3::new(0.22, 0.16, 0.10);

/// sRGB parts as a linear colour.
pub(super) fn linear(colour: Vec3) -> Vec3 {
    let c = Color::srgb(colour.x, colour.y, colour.z).to_linear();
    Vec3::new(c.red, c.green, c.blue)
}

/// `mesh` with every point coloured by `paint(point, normal)` (sRGB).
fn painted(mut mesh: Mesh, paint: impl Fn(Vec3, Vec3) -> Vec3) -> Mesh {
    let points: Vec<Vec3> = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(bevy::mesh::VertexAttributeValues::Float32x3(p)) => {
            p.iter().map(|p| Vec3::from_array(*p)).collect()
        }
        _ => Vec::new(),
    };
    let normals: Vec<Vec3> = match mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
        Some(bevy::mesh::VertexAttributeValues::Float32x3(n)) => {
            n.iter().map(|n| Vec3::from_array(*n)).collect()
        }
        _ => vec![Vec3::Y; points.len()],
    };
    let colours: Vec<[f32; 4]> = (points.iter().zip(&normals))
        .map(|(point, normal)| linear(paint(*point, *normal)).extend(1.0).to_array())
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colours);
    mesh
}

/// A round of wood standing on end, its foot at the origin: bark round it,
/// its cut top pale.
pub fn stub(radius: f32, height: f32) -> (Mesh, Transform) {
    let mesh = Cylinder::new(radius, height).mesh().resolution(10).build();
    let mesh = painted(mesh, |_, normal| if normal.y > 0.5 { CUT } else { BARK });
    (mesh, Transform::from_xyz(0.0, height / 2.0, 0.0))
}

/// A stick lying along x on the ground, its middle at the origin.
pub fn stick(radius: f32, length: f32) -> (Mesh, Transform) {
    let mesh = Cylinder::new(radius, length).mesh().resolution(8).build();
    let mesh = painted(
        mesh,
        |_, normal| {
            if normal.y.abs() > 0.5 { CUT } else { BARK }
        },
    );
    let place = Transform::from_xyz(0.0, radius * 0.8, 0.0)
        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));
    (mesh, place)
}

/// A rod of bark from `from` to `to`.
pub fn rod(from: Vec3, to: Vec3, radius: f32) -> (Mesh, Transform) {
    let length = from.distance(to).max(0.001);
    let mesh = Cylinder::new(radius, length).mesh().resolution(6).build();
    let mesh = painted(mesh, |_, _| BARK);
    let place = Transform::from_translation((from + to) / 2.0)
        .with_rotation(Quat::from_rotation_arc(Vec3::Y, (to - from) / length));
    (mesh, place)
}

/// A low mound of turned earth `radius` across, with a few crumbs round it:
/// where a truffle is.
pub fn mound(radius: f32, seed: u64) -> Vec<(Mesh, Transform)> {
    let hump = painted(Sphere::new(1.0).mesh().uv(12, 6), |point, _| {
        let speck = ((point.x * 7.1 + point.z * 3.7).sin() * 0.5 + 0.5) * 0.25;
        EARTH * (0.85 + speck)
    });
    let mut pieces = vec![(
        hump,
        Transform::from_scale(Vec3::new(radius, radius * 0.4, radius)),
    )];
    for k in 0..4u64 {
        let turn = ((seed >> (k * 8)) & 0xff) as f32 / 255.0 * std::f32::consts::TAU;
        let out = radius * (1.05 + 0.1 * k as f32);
        let size = radius * (0.14 + 0.03 * (k % 2) as f32);
        let crumb = painted(Sphere::new(1.0).mesh().uv(6, 4), |_, _| CRUMB);
        let at = Vec3::new(turn.cos() * out, size * 0.3, turn.sin() * out);
        pieces.push((
            crumb,
            Transform::from_translation(at).with_scale(Vec3::new(size, size * 0.6, size)),
        ));
    }
    pieces
}
