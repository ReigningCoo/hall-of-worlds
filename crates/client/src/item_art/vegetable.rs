//! Vegetables', pulses' and grains' pictures (`Look::Vegetable`): each drawn by
//! the part picked (`group.rs`: roots, tubers, bulbs, leaves, heads, stalks,
//! shoots, fruits, pods, seeds, and a sheaf for grains), in its look's colours
//! (`shared::vegetable::VegetableLook`). Each item looks a little different by
//! its look seed (`variety.rs`: which of its colours, most the usual one, a few
//! other varieties such as purple carrots; size, shade, the pieces' wobbles),
//! one of `VEGETABLE_VARIANTS`; a stack shows its bottom one's. A serving cut
//! from a whole one is a slice showing its inside (`slice.rs`). Lying flat,
//! seen from above, long ones across the picture's diagonal, filling its cells.

mod bulbs;
mod colours;
mod ears;
mod fruits;
mod group;
mod heads;
mod heap;
mod leaf;
mod leaves;
mod pods;
mod roots;
mod seeds;
mod sheaf;
mod shoots;
mod slice;
mod stalks;
mod tube;
mod tubers;
mod variety;

use bevy::prelude::*;
use shared::vegetable::{VegetableDef, VegetableForm, Vegetables};
use std::f32::consts::FRAC_PI_4;

use super::Part;
use super::herb::kit::Kit;
use colours::Colours;
use group::{Drawing, Fruit, Head, Shoot, Stalk, Tuber};
use variety::Variety;

/// How many ways one vegetable's picture varies.
pub const VEGETABLE_VARIANTS: u8 = 16;

/// What a vegetable item's picture shows: which vegetable, whether a slice of
/// it, which of its variants, and the cells it fills.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VegetableItemLook {
    pub(super) vegetable: String,
    pub(super) serving: bool,
    pub(super) variant: u8,
    pub(super) cells: (u8, u8),
}

impl VegetableItemLook {
    /// A vegetable item's look, from its form: its vegetable, a slice or not, a
    /// variant by its look seed, and its cells.
    pub fn of(form: &VegetableForm) -> Self {
        let shape = form.shape();
        Self {
            vegetable: form.vegetable.clone(),
            serving: form.serving,
            variant: (form.look % u64::from(VEGETABLE_VARIANTS)) as u8,
            cells: (shape.width, shape.height),
        }
    }
}

/// What every family's drawing is handed.
struct Build<'a> {
    def: &'a VegetableDef,
    colours: &'a Colours,
    variety: &'a Variety,
    /// How many pieces it shows.
    count: usize,
}

impl Build<'_> {
    /// One piece's width against its length.
    fn width_share(&self) -> f32 {
        f32::from(self.def.look.width) / f32::from(self.def.look.length.max(1))
    }
}

/// The half sizes of the box a picture's model must stay in, as fruit's.
fn room(cells: (u8, u8)) -> Vec2 {
    let (wide, long) = (f32::from(cells.0), f32::from(cells.1));
    let longest = wide.max(long).max(1.0);
    Vec2::new(wide, long) / longest * 0.98
}

/// Whether it is drawn long, along +x, so across a square picture's diagonal.
fn long(drawing: Drawing, count: usize) -> bool {
    match drawing {
        Drawing::Tuber(Tuber::Lumpy | Tuber::Ringed) => count == 1,
        Drawing::Tuber(_) | Drawing::Root(_) | Drawing::Leaves(_) | Drawing::Sheaf(_) => true,
        Drawing::Stalk(stalk) => stalk != Stalk::Heart,
        Drawing::Shoot(shoot) => shoot != Shoot::Fiddlehead,
        Drawing::Fruit(fruit) => matches!(fruit, Fruit::Cucumber | Fruit::Aubergine | Fruit::Gourd),
        Drawing::Pod(pod) => pod != group::Pod::Shell,
        Drawing::Head(head) => head == Head::Artichoke,
        Drawing::Bulb | Drawing::Seed(_) => false,
    }
}

/// The parts of a vegetable's picture: nothing for one the table lacks.
pub(super) fn vegetable_parts(
    look: &VegetableItemLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let Some(def) = Vegetables::built_in().get(&look.vegetable) else {
        return Vec::new();
    };
    let variety = Variety::of(look.variant, def.look.colours.len());
    let colours = Colours::of(def, &variety);
    let drawing = Drawing::of(def);
    let build = Build {
        def,
        colours: &colours,
        variety: &variety,
        count: heap::count(def, drawing),
    };
    let square = look.cells.0 == look.cells.1;
    let turn = match (square, !look.serving && long(drawing, build.count)) {
        (true, true) => FRAC_PI_4 + variety.turn * 0.08,
        (true, false) => variety.turn * 0.4,
        (false, _) => variety.turn * 0.03,
    };
    let mut kit = Kit::new(meshes, materials, 0).turned(turn);
    if look.serving {
        slice::slice(&build, &mut kit);
    } else {
        draw(&build, drawing, &mut kit);
    }
    kit.centre();
    kit.fill(room(look.cells) * variety.size);
    kit.finish()
}

fn draw(build: &Build, drawing: Drawing, kit: &mut Kit) {
    match drawing {
        Drawing::Root(root) => roots::roots(build, root, kit),
        Drawing::Tuber(tuber) => tubers::tubers(build, tuber, kit),
        Drawing::Bulb => bulbs::bulbs(build, kit),
        Drawing::Leaves(leaves) => leaves::leaves(build, leaves, kit),
        Drawing::Head(head) => heads::head(build, head, kit),
        Drawing::Stalk(stalk) => stalks::stalks(build, stalk, kit),
        Drawing::Shoot(shoot) => shoots::shoots(build, shoot, kit),
        Drawing::Fruit(fruit) => fruits::fruit(build, fruit, kit),
        Drawing::Pod(pod) => pods::pods(build, pod, kit),
        Drawing::Seed(seed) => seeds::seeds(build, seed, kit),
        Drawing::Sheaf(ear) => sheaf::sheaf(build, ear, kit),
    }
}

#[cfg(test)]
mod tests;
