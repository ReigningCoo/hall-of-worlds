//! Patterns and weaves as surface looks, with no texture: the cloth's triangles are
//! split smaller and each coloured by where it lies. Stripes run down the body and
//! checks across it; tartan is wider bands crossing, darker where they cross;
//! brocade and damask are small diamonds, gold or in the cloth's sheen.

use bevy::prelude::*;
use shared::clothing::{ClothingForm, Pattern, Weave};

use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::Triangle;

/// How far round the trunk a millimetre of angle is, about.
const ROUND: f32 = 150.0;

/// What a triangle centred here becomes, if the pattern changes it.
type Motif = dyn Fn(Vec3) -> Option<Stuff>;

/// The form's pattern or weave, if it has one; `all_over` is a robe embroidered all
/// over.
fn motif(form: &ClothingForm, all_over: bool) -> Option<Box<Motif>> {
    // Across (round the body) and down, in millimetres.
    let at = |p: Vec3| (p.z.atan2(p.x) * ROUND, p.y);
    let diamond = |size: f32, stuff: Stuff| -> Box<Motif> {
        Box::new(move |p: Vec3| {
            let (u, v) = at(p);
            let (fu, fv) = (
                (u / size).fract().abs() - 0.5,
                (v / size).fract().abs() - 0.5,
            );
            (fu.abs() + fv.abs() < 0.28).then_some(stuff)
        })
    };
    let band = |value: f32, size: f32, every: i32| (value / size).floor() as i32 % every == 0;
    Some(match (form.pattern, form.weave) {
        (Pattern::Striped, _) => Box::new(move |p| band(at(p).0, 22.0, 2).then_some(Stuff::Cloth)),
        (Pattern::Checked, _) => Box::new(move |p| {
            let (u, v) = at(p);
            let odd = ((u / 28.0).floor() + (v / 28.0).floor()) as i32 % 2 != 0;
            odd.then_some(Stuff::Cloth)
        }),
        (Pattern::Tartan, _) => Box::new(move |p| {
            let (u, v) = at(p);
            match (band(u, 16.0, 4), band(v, 16.0, 4)) {
                (true, true) => Some(Stuff::Charge),
                (true, false) | (false, true) => Some(Stuff::Cloth),
                (false, false) => None,
            }
        }),
        (Pattern::Plain, Weave::Brocade) => diamond(40.0, Stuff::Field),
        (Pattern::Plain, Weave::Damask) => diamond(40.0, Stuff::Trim),
        _ if all_over => diamond(50.0, Stuff::Field),
        _ => return None,
    })
}

/// A triangle in four, by its sides' middles.
fn split(triangle: Triangle) -> [Triangle; 4] {
    let [a, b, c] = triangle;
    let (ab, bc, ca) = ((a + b) / 2.0, (b + c) / 2.0, (c + a) / 2.0);
    [[a, ab, ca], [ab, b, bc], [ca, bc, c], [ab, bc, ca]]
}

/// Colours the cloth's main triangles by the pattern, in place.
pub(super) fn apply(model: &mut Model, form: &ClothingForm, all_over: bool) {
    let Some(motif) = motif(form, all_over) else {
        return;
    };
    let main = std::mem::take(model.of(Stuff::Main));
    for triangle in main.into_iter().flat_map(split) {
        let middle = (triangle[0] + triangle[1] + triangle[2]) / 3.0;
        let stuff = motif(middle).unwrap_or(Stuff::Main);
        model.of(stuff).push(triangle);
    }
}
