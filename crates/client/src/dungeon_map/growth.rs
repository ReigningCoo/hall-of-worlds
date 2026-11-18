//! Building crops again when what is seen, the hour, the plants or the debug
//! menu's shift change.

use bevy::prelude::*;

use super::CELL_SIZE;
use super::{CropPart, Shown};
use crate::common::{CropGrowthShift, CurrentDungeon, DrawnGround, ServerClock};

/// Builds the crops again when what is seen, the hour, the plants or the
/// shift change: never for the camera, as fields can hold many more faces than
/// everything else put together.
pub(super) fn update_crops(
    (dungeon, shown, heights): (Res<CurrentDungeon>, Res<Shown>, Res<DrawnGround>),
    (clock, shift): (Res<ServerClock>, Res<CropGrowthShift>),
    part: Single<(&Mesh3d, &mut Visibility), With<CropPart>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut built: Local<Option<(u64, u64, u32)>>,
) {
    let now = clock.minutes.max(0.0) as u64;
    let key = (shown.looks, now / 60, shift.0);
    let changed = *built != Some(key) || dungeon.is_changed() || heights.is_changed();
    let Some(sight) = shown.sight.as_ref().filter(|_| changed) else {
        return;
    };
    *built = Some(key);
    let ground = |at: Vec3| heights.drawn_between(at.x / CELL_SIZE, at.z / CELL_SIZE) * CELL_SIZE;
    let seen = (sight, &dungeon.remembered);
    let crops = super::field::build_crops(&dungeon.level, seen, (now, shift.0), &ground);
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
