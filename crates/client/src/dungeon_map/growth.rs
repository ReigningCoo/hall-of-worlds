//! Building crops again when what is seen changes.

use bevy::prelude::*;

use super::CELL_SIZE;
use super::{CropPart, Shown};
use crate::common::{CurrentDungeon, DrawnGround};

/// Builds the crops again when what is seen changes: never for the camera, as
/// fields can hold many more faces than everything else put together.
pub(super) fn update_crops(
    dungeon: Res<CurrentDungeon>,
    shown: Res<Shown>,
    heights: Res<DrawnGround>,
    part: Single<(&Mesh3d, &mut Visibility), With<CropPart>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut built: Local<u64>,
) {
    let Some(sight) = shown.sight.as_ref().filter(|_| *built != shown.looks) else {
        return;
    };
    *built = shown.looks;
    let ruined = dungeon.info.as_ref().is_some_and(|info| info.ruined);
    let ground = |at: Vec3| heights.drawn_between(at.x / CELL_SIZE, at.z / CELL_SIZE) * CELL_SIZE;
    let remembered = &dungeon.remembered;
    let crops = super::field::build_crops(&dungeon.fields, sight, remembered, ruined, &ground);
    let (handle, mut visibility) = part.into_inner();
    let any = !crops.indices.is_empty();
    visibility.set_if_neq(if any {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    });
    if any && let Some(mut stored) = meshes.get_mut(&handle.0) {
        *stored = crops.into_mesh();
    }
}
