//! Herbs' pictures (`Look::Herb`): the part used (`shared::herb::Part`) as it is
//! picked, coloured from the herb's plant (`PlantLook`): bunched leaves, flower
//! heads, berries, seeds, a root and the rest, in a few variants by the item's
//! look seed. Pictures are kept by herb and variant, not per item. One cell,
//! lying flat, seen from above.

mod colour;
mod flowers;
mod fruit;
pub(super) mod kit;
mod leafy;
mod roots;

use bevy::prelude::*;
use shared::herb::{HerbDef, HerbForm, Herbs, Part as Used};

use super::Part;
use colour::Palette;
use kit::Kit;

/// How many ways a herb's picture varies: stacks show their bottom one's.
pub const HERB_VARIANTS: u8 = 3;

/// What a herb's picture shows: which herb, and which of its variants.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HerbLook {
    pub(super) herb: String,
    pub(super) variant: u8,
}

impl HerbLook {
    /// A herb item's look, from its form: its herb, and a variant by its seed.
    pub fn of(form: &HerbForm) -> Self {
        Self {
            herb: form.herb.clone(),
            variant: (form.look % u64::from(HERB_VARIANTS)) as u8,
        }
    }
}

/// The parts of a herb's picture, from its herb in the table: nothing for a herb
/// the table doesn't have.
pub(super) fn herb_parts(
    look: &HerbLook,
    herbs: &Herbs,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let Some(def) = herbs.get(&look.herb) else {
        return Vec::new();
    };
    let mut kit = Kit::new(meshes, materials, look.variant);
    build(def, look.variant, &mut kit);
    kit.finish()
}

/// Builds the part used, by kind.
fn build(def: &HerbDef, variant: u8, kit: &mut Kit) {
    let palette = Palette::of(def, variant);
    match def.part {
        Used::Leaves => leafy::leaves(&palette, variant, kit),
        Used::Fronds => leafy::fronds(&palette, variant, kit),
        Used::Stalks => leafy::stalks(&palette, variant, kit),
        Used::Gel => leafy::gel(&palette, kit),
        Used::Tops => flowers::tops(&palette, variant, kit),
        Used::Flowers => flowers::flowers(&palette, variant, kit),
        Used::Buds => flowers::buds(&palette, variant, kit),
        Used::Stigmas => flowers::stigmas(&palette, variant, kit),
        Used::Berries => fruit::berries(&palette, variant, kit),
        Used::Fruit => fruit::fruit(&palette, variant, kit),
        Used::Seeds => fruit::seeds(&palette, variant, kit),
        Used::Pods => fruit::pods(&palette, variant, kit),
        Used::Root => roots::root(&palette, variant, kit),
        Used::Rhizome => roots::rhizome(&palette, variant, kit),
        Used::Tuber => roots::tuber(&palette, variant, kit),
        Used::Bulb => roots::bulb(&palette, kit),
        Used::Bark => roots::bark(&palette, variant, kit),
        Used::Resin => roots::resin(&palette, variant, kit),
    }
}

#[cfg(test)]
mod tests;
