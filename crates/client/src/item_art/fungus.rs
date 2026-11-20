//! Mushrooms', fungi's and lichens' pictures (`Look::Fungus`), built by their
//! look's form (`shared::fungus::FungusLook`) in its colours: caps on stems
//! with gills, pores or spines, rings, volvas and warts; funnels, brackets,
//! balls, coral, clubs, cups, jelly, morels, truffles and other lumps, crusts,
//! and lichens' tufts, beards and plates. One standing fungus is `fungus_one`
//! (`model.rs`), which the maps can use too; a picture tips it so its side
//! (and a little of its underside) shows, or a cup's inside, and lays a handful
//! side by side or heaped. A serving of a big one is a half-round slab
//! (`slice.rs`).

mod ball;
mod bracket;
mod cap;
mod club;
mod coral;
mod cup;
mod funnel;
mod jelly;
mod lathe;
mod lichen;
mod lump;
mod model;
mod morel;
mod paint;
mod slice;
mod variety;

use bevy::prelude::*;
use shared::fungus::{Form, Fungi, FungusDef, FungusForm, Unit};
use std::f32::consts::FRAC_PI_2;

use super::Part;
use super::herb::kit::Kit;
pub use model::fungus_one;
use model::{Build, FungusPiece, one};
use paint::Colours;
use variety::Variety;

/// How many ways one fungus's picture varies.
pub const FUNGUS_VARIANTS: u8 = 16;

/// What a fungus item's picture shows: which fungus, whether a slice of it,
/// which of its variants, and the cells it fills.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FungusItemLook {
    pub(super) fungus: String,
    pub(super) serving: bool,
    pub(super) variant: u8,
    pub(super) cells: (u8, u8),
}

impl FungusItemLook {
    /// A fungus item's look, from its form.
    pub fn of(form: &FungusForm) -> Self {
        let shape = form.shape();
        Self {
            fungus: form.fungus.clone(),
            serving: form.serving,
            variant: (form.look % u64::from(FUNGUS_VARIANTS)) as u8,
            cells: (shape.width, shape.height),
        }
    }
}

/// How far a form is tipped about x from standing: upright ones lie along the
/// picture's height, a little of their underside showing; cups and funnels
/// show their inside; low ones are seen from above.
fn tip(form: Form) -> f32 {
    match form {
        Form::CapAndStem | Form::Club | Form::Honeycomb | Form::LichenBeard => -FRAC_PI_2 - 0.1,
        Form::Funnel | Form::Cup => -0.75,
        Form::Bracket => -0.5,
        Form::LichenTufts => -0.6,
        _ => 0.0,
    }
}

/// Whether it is drawn long, along the picture's height.
fn upright(form: Form) -> bool {
    tip(form) < -1.0
}

/// How many a handful shows.
fn count(def: &FungusDef) -> usize {
    if def.unit != Unit::Handful {
        return 1;
    }
    match def.look.form {
        Form::CapAndStem if def.look.largest <= 40 => 5,
        Form::CapAndStem | Form::Funnel | Form::Cup | Form::Honeycomb | Form::Ball => 3,
        Form::Bracket => 4,
        _ => 1,
    }
}

/// The half sizes of the box a picture's model must stay in, as fruit's.
fn room(cells: (u8, u8)) -> Vec2 {
    let (wide, long) = (f32::from(cells.0), f32::from(cells.1));
    let longest = wide.max(long).max(1.0);
    Vec2::new(wide, long) / longest * 0.98
}

/// The pieces of the whole item, standing: one, a handful side by side or
/// heaped, a tier of brackets, or a slice.
fn pieces(build: &Build, serving: bool) -> Vec<FungusPiece> {
    let form = build.def.look.form;
    let count = count(build.def);
    if serving {
        return slice::slice(build, 0);
    }
    if form == Form::Bracket && count > 1 {
        return bracket::tier(build, count);
    }
    let middle = (count as f32 - 1.0) / 2.0;
    let mut pieces = Vec::new();
    for n in 0..count {
        let off = n as f32 - middle;
        let size = 1.0 - 0.15 * off.abs() + build.variety.wobble(n, 20) * 0.05;
        let place = match upright(form) {
            true => Transform::from_translation(Vec3::new(off * 0.5, 0.0, (n % 2) as f32 * 0.1))
                .with_rotation(Quat::from_rotation_z(-off * 0.22)),
            false => {
                let a = n as f32 * 2.399_963;
                let out = if n == 0 { 0.0 } else { 0.55 };
                Transform::from_translation(Vec3::new(a.cos() * out, 0.0, a.sin() * out))
                    .with_rotation(Quat::from_rotation_y(a))
            }
        };
        let place = place.with_scale(Vec3::splat(size));
        pieces.extend(one(build, n).into_iter().map(|piece| piece.moved(place)));
    }
    pieces
}

/// The parts of a fungus's picture: nothing for one the table lacks.
pub(super) fn fungus_parts(
    look: &FungusItemLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let Some(def) = Fungi::built_in().get(&look.fungus) else {
        return Vec::new();
    };
    let variety = Variety::of(look.variant);
    let build = Build {
        def,
        colours: Colours::of(&def.look, def.group, variety.shade),
        variety: &variety,
    };
    let form = def.look.form;
    let tilt = match look.serving {
        true => -0.35,
        false => tip(form),
    };
    let long = !look.serving && upright(form) || form == Form::LichenBeard;
    let turn = match (look.cells.0 > look.cells.1, long) {
        (true, true) => -FRAC_PI_2 + 0.25 + variety.turn * 0.1,
        _ => variety.turn * 0.25,
    };
    let mut kit = Kit::new(meshes, materials, 0).turned(turn);
    let matte = kit.material(Color::WHITE, false);
    let shiny = kit.material(Color::WHITE, true);
    let tipped = Transform::from_rotation(Quat::from_rotation_x(tilt));
    for piece in pieces(&build, look.serving) {
        let material = if piece.shiny { &shiny } else { &matte };
        kit.mesh(piece.mesh, material, tipped * piece.transform);
    }
    kit.centre();
    kit.fill(room(look.cells) * variety.size);
    kit.finish()
}

#[cfg(test)]
mod tests;
