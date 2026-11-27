//! Turning the lit water's flow on and off (`mesh::flow`), so the picture of
//! water on raised wilderness ground moves downhill. The shader keeps the time
//! itself (`sight_mask.wesl`, from Bevy's globals), so the material changes only
//! when flowing starts or stops: changing it every frame made the web build
//! make new buffers every frame. Still with Reduce Motion (owner) and off the
//! wilderness.

use bevy::prelude::*;

use super::mesh::Surface;
use super::parts::SurfaceMaterials;
use super::sight_lines::SurfaceMaterial;
use crate::common::{DrawnGround, Preferences};

/// Starts or stops the lit water flowing.
pub(super) fn run_flow(
    preferences: Res<Preferences>,
    heights: Res<DrawnGround>,
    surfaces: Res<SurfaceMaterials>,
    mut materials: ResMut<Assets<SurfaceMaterial>>,
) {
    let flowing = heights.slopes() && !preferences.graphics.reduce_motion;
    let water = &surfaces.solid[Surface::Water as usize];
    let current = materials.get(water).map(|m| m.extension.flowing());
    if current.is_some_and(|now| now != flowing)
        && let Some(mut material) = materials.get_mut(water)
    {
        material.extension.set_flowing(flowing);
    }
}
