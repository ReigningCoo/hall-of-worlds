//! Fruits' and nuts' pictures (`Look::Fruit`): each fruit drawn from its look in
//! the fruit table (`shared::fruit`: shape, ripe colour, skin), by shape group
//! (`group.rs`): single fruits (round, oval, pear-shaped, long, spiky, hairy,
//! bumpy, scaled, cactus fruit, drupelet clusters), handfuls of them, bunches,
//! bananas, pods and nuts. Each item looks a little different by its look seed
//! (`variety.rs`: size, blush, colour within its ripe range, stalk and leaf,
//! speckles), one of `FRUIT_VARIANTS`; a stack shows its bottom one's. A serving
//! cut from a big fruit is a slice showing its inside (`slice.rs`). Lying flat,
//! seen from above, filling the item's cells.

mod body;
mod colour;
mod group;
mod heap;
mod nuts;
mod pods;
mod slice;
mod surface;
mod tops;
mod variety;

use bevy::prelude::*;
use shared::fruit::{FruitForm, Fruits};

use super::Part;
use super::herb::kit::Kit;
use colour::Colours;
use group::Group;
use variety::Variety;

/// How many ways one fruit's picture varies.
pub const FRUIT_VARIANTS: u8 = 16;

/// What a fruit item's picture shows: which fruit, whether a slice of it, which
/// of its variants, and the cells it fills.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FruitItemLook {
    pub(super) fruit: String,
    pub(super) serving: bool,
    pub(super) variant: u8,
    pub(super) cells: (u8, u8),
}

impl FruitItemLook {
    /// A fruit item's look, from its form: its fruit, a slice or not, a variant
    /// by its look seed, and its cells.
    pub fn of(form: &FruitForm) -> Self {
        let shape = form.shape();
        Self {
            fruit: form.fruit.clone(),
            serving: form.serving,
            variant: (form.look % u64::from(FRUIT_VARIANTS)) as u8,
            cells: (shape.width, shape.height),
        }
    }
}

/// The half sizes of the box a picture's model must stay in: the longest side
/// a little under the 1.25 the picture shows, leaving room for the black edge.
fn room(cells: (u8, u8)) -> Vec2 {
    let (wide, long) = (f32::from(cells.0), f32::from(cells.1));
    let longest = wide.max(long).max(1.0);
    Vec2::new(wide, long) / longest * 0.98
}

/// The parts of a fruit's picture: nothing for a fruit the table lacks.
pub(super) fn fruit_parts(
    look: &FruitItemLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let Some(def) = Fruits::built_in().get(&look.fruit) else {
        return Vec::new();
    };
    let variety = Variety::of(look.variant);
    // Turned only in a square picture, where models keep within a circle.
    let square = look.cells.0 == look.cells.1;
    let turn = if square { variety.turn } else { 0.0 };
    let mut kit = Kit::new(meshes, materials, 0).turned(turn);
    let colours = Colours::of(def, &variety);
    let room = room(look.cells);
    if look.serving {
        slice::slice(def, &colours, &variety, room, &mut kit);
    } else {
        match Group::of(def) {
            Group::Fruit(style) => heap::fruit(&style, &colours, &variety, room, &mut kit),
            Group::Bunch => heap::bunch(&colours, &variety, room, &mut kit),
            Group::Banana => pods::banana(&colours, &variety, room, &mut kit),
            Group::Pod(pod) => pods::pod(pod, &colours, &variety, room, &mut kit),
            Group::Nut(nut) => nuts::nuts(def, nut, &variety, room, &mut kit),
        }
    }
    kit.fit(room);
    kit.finish()
}

#[cfg(test)]
mod tests;
