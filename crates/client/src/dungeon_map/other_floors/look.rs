//! The other floors' materials follow the scene's own (its theme, grey
//! copies, plain pictures), darker per storey down by the light outside (those
//! above as they are), never cut by the sight mask, and solid even as memory:
//! blended layers don't sort here, and see-through floors would show rooms
//! never seen.

use bevy::prelude::*;

use super::super::SurfaceMaterials;
use super::super::daylight::OutsideLight;
use super::super::mesh::Surface;
use super::super::sight_lines::SurfaceMaterial;
use super::floors::Floors;
use super::which::kept_per_storey;
use crate::common::{Lightning, OtherFloorLooks};

/// Keeps every other floor's materials like the scene's, those below darkened.
pub(in crate::dungeon_map) fn follow_materials(
    floors: Res<Floors>,
    surfaces: Res<SurfaceMaterials>,
    outside: Res<OutsideLight>,
    lightning: Res<Lightning>,
    looks: Res<OtherFloorLooks>,
    mut materials: ResMut<Assets<SurfaceMaterial>>,
) {
    let daylight = outside.in_flash(lightning.0).max_element();
    let kept = kept_per_storey(looks.day_dark, looks.night_dark, daylight);
    let below = floors.below.iter().enumerate();
    let below = below.map(|(depth, floor)| (floor, kept.powi(depth as i32 + 1)));
    let above = floors.above.iter().map(|floor| (floor, 1.0));
    for (floor, shade) in below.chain(above) {
        for (index, &surface) in Surface::ALL.iter().enumerate() {
            let solid = (&surfaces.solid[index], &floor.solid[index].2);
            follow(&mut materials, solid, shade, None);
            let memory = (&surfaces.memory[index], &floor.memory[index].2);
            // The wall's fade across the floor must still blend.
            let solid_memory = surface != Surface::WallBlend;
            follow(&mut materials, memory, shade, solid_memory.then_some(1.0));
        }
    }
}

/// Makes `copy` like `scene` with its colour times `shade`, and its opacity
/// `alpha` (as it is, if none), changing it only if it differs.
fn follow(
    materials: &mut Assets<SurfaceMaterial>,
    (scene, copy): (&Handle<SurfaceMaterial>, &Handle<SurfaceMaterial>),
    shade: f32,
    alpha: Option<f32>,
) {
    let Some(like) = materials.get(scene) else {
        return;
    };
    let colour = LinearRgba::from(like.base.base_color);
    let colour = LinearRgba {
        red: colour.red * shade,
        green: colour.green * shade,
        blue: colour.blue * shade,
        alpha: alpha.unwrap_or(colour.alpha),
    };
    let alpha_mode = match alpha {
        Some(_) => AlphaMode::Opaque,
        None => like.base.alpha_mode,
    };
    let texture = like.base.base_color_texture.clone();
    let same = materials.get(copy).is_some_and(|had| {
        had.base.base_color_texture == texture
            && LinearRgba::from(had.base.base_color) == colour
            && had.base.alpha_mode == alpha_mode
    });
    if same {
        return;
    }
    let mut made = like.clone();
    made.base.base_color = colour.into();
    made.base.alpha_mode = alpha_mode;
    // Nothing on another floor is cut by the edge of sight on the player's.
    made.extension.set_side(None);
    if let Some(mut had) = materials.get_mut(copy) {
        // The wedge is `aim_wedge`'s.
        made.extension.wedge_like(&had.extension);
        *had = made;
    }
}
