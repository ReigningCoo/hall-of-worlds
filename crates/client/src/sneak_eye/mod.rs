//! The sneaking eye (mob behaviour plan PR 6): only while Sneak is on, an eye
//! over the player, closed to open by how noticeable they are (the chance the
//! mobs they can see would hear their steps, by `shared::noise`), and wide
//! open while a mob is Alert to them (`common::MobAlert`). No markers over
//! mobs: they would be confused with quest markers (owner).

mod noticeable;
mod show;

use bevy::prelude::*;
use bevy::transform::TransformSystems;

pub struct SneakEyePlugin;

impl Plugin for SneakEyePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::common::MobAlert>().add_systems(
            PostUpdate,
            show::show_eye.after(TransformSystems::Propagate),
        );
    }
}
