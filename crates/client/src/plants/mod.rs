//! Herbs' plants as 3D models, built from each herb's `PlantLook` (its shape,
//! sizes and colours) at each of its four stages, in a few variants, with
//! black outlines; and where a patch's plants stand. The maps place them
//! (`dungeon_map`).

use bevy::prelude::*;

mod layout;
mod models;
mod pieces;
mod plan;
mod shapes;
mod triangles;

pub use layout::patch_layout;
pub use models::PlantModels;

/// Keeps the plant models built so far.
pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlantModels>()
            .add_systems(Update, models::forget_on_new_table);
    }
}

#[cfg(test)]
mod tests;
