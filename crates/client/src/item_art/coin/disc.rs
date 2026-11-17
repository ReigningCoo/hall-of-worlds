//! The blank coin, lying flat with its face up (+y): a flat back, the outer edge
//! reeded with fine upright ridges, and a raised rim round the face, whose field
//! and emboss `face.rs` adds inside the rim.

use bevy::prelude::*;
use std::f32::consts::TAU;

use super::Coin;
use super::mesh::Builder;

/// Ridges round the edge.
pub(super) const REEDS: usize = 72;
/// Points round the coin: four to each ridge (top, top, groove, groove).
pub(super) const AROUND: usize = REEDS * 4;
/// How deep the grooves between ridges are, against the radius.
const REED_DEPTH: f32 = 0.018;
/// Where the rim's flat top ends inside, against the radius.
const RIM_INSIDE: f32 = 0.88;
/// Where the rim's inner wall meets the field, against the radius: the face's edge.
pub(super) const FIELD: f32 = 0.845;
/// How far the rim stands above the field, against the thickness.
pub(super) const RIM_HIGH: f32 = 0.35;

/// The angle of the `i`th point round.
pub(super) fn angle(i: usize) -> f32 {
    i as f32 / AROUND as f32 * TAU
}

/// How far out the edge is at the `i`th point round: ridge or groove.
fn edge_radius(coin: &Coin, i: usize) -> f32 {
    let groove = matches!(i % 4, 2 | 3);
    coin.radius * if groove { 1.0 - REED_DEPTH } else { 1.0 }
}

fn round(radius: f32, i: usize, y: f32) -> Vec3 {
    let a = angle(i);
    Vec3::new(radius * a.cos(), y, radius * a.sin())
}

/// The back, the reeded edge, the rim's top and its inner wall down to the field.
pub(super) fn blank(coin: &Coin, mesh: &mut Builder) {
    let (top, bottom) = (coin.thickness / 2.0, -coin.thickness / 2.0);
    let field = top - RIM_HIGH * coin.thickness;
    let (inside, edge) = (RIM_INSIDE * coin.radius, FIELD * coin.radius);
    for i in 0..AROUND {
        let j = (i + 1) % AROUND;
        let out = round(1.0, i, 0.0) + round(1.0, j, 0.0);
        let (ri, rj) = (edge_radius(coin, i), edge_radius(coin, j));
        mesh.quad(
            [
                round(ri, i, bottom),
                round(rj, j, bottom),
                round(rj, j, top),
                round(ri, i, top),
            ],
            out,
            0.8,
        );
        mesh.quad(
            [
                round(ri, i, top),
                round(rj, j, top),
                round(inside, j, top),
                round(inside, i, top),
            ],
            Vec3::Y,
            1.0,
        );
        mesh.quad(
            [
                round(inside, i, top),
                round(inside, j, top),
                round(edge, j, field),
                round(edge, i, field),
            ],
            Vec3::Y,
            0.85,
        );
        mesh.quad(
            [
                round(ri, i, bottom),
                round(rj, j, bottom),
                Vec3::new(0.0, bottom, 0.0),
                Vec3::new(0.0, bottom, 0.0),
            ],
            Vec3::NEG_Y,
            0.7,
        );
    }
}
