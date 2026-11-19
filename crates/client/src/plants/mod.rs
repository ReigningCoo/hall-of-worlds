//! Herbs' plants as 3D models, built from each herb's `PlantLook` (its shape,
//! sizes and colours) at each of its four stages, in a few variants, with
//! black outlines; and where a patch's plants stand. Fruit plants' models too,
//! by family (`FruitModels`), vegetables' (`VegetableModels`), and trees'
//! (`TreeModels`). The maps place them (`dungeon_map`).

use bevy::prelude::*;

mod fruit;
mod layout;
mod models;
mod pieces;
mod plan;
mod shapes;
mod tree;
mod triangles;
mod vegetable;

pub use fruit::{FruitModels, fruit_plant_size};
pub use layout::patch_layout;
pub use models::PlantModels;
pub use tree::{TreeModel, TreeModels, tree_size};
pub use vegetable::VegetableModels;

/// Keeps the plant models built so far.
pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlantModels>()
            .init_resource::<FruitModels>()
            .init_resource::<VegetableModels>()
            .init_resource::<TreeModels>()
            .add_systems(Update, models::forget_on_new_table);
    }
}

#[cfg(test)]
mod tests;
