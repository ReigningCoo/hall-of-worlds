//! The Hall's one hint at The Way Inn (owner: drawing only, a reminder for
//! returning players, never a mark): the marble of each edge tile in front of
//! a hidden way in (`shared::hall::inn_way_behind`) a little dulled, as if
//! worn by feet stepping off there. A faint, soft patch lying on the tile,
//! towards its outer edge, placed and bent with the Hall like the gates.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use shared::hall::{self, Side};
use shared::protocol::LevelKind;
use std::collections::BTreeMap;

use crate::common::{CurrentDungeon, HallBend};
use crate::dungeon_map::CELL_SIZE;

/// The patch's colour and how much it shows at its middle: barely there.
const WEAR: Color = Color::srgba(0.42, 0.37, 0.3, 0.16);
/// How wide it is across the tile, and how far towards the edge it sits, in
/// cells.
const ACROSS: f32 = 0.85;
const TOWARDS_EDGE: f32 = 0.12;
/// How far above the floor, in cells, so it never flickers into it.
const ABOVE: f32 = 0.004;
/// The soft edge's picture, square, in pixels.
const PIXELS: u32 = 32;

/// The worn patches lying on the stretch shown, by cell as the client counts
/// it, and the mesh and material they share.
#[derive(Resource, Default)]
pub(super) struct Worn {
    lying: BTreeMap<(i64, i64), (Entity, Side)>,
    look: Option<(Handle<Mesh>, Handle<StandardMaterial>)>,
}

/// When the stretch changes, lays a patch on each worn tile of it and takes
/// away those off it; anywhere else, takes them all away. Kept on the bent
/// Hall whenever it bends.
#[allow(clippy::too_many_arguments)]
pub(super) fn place_worn(
    dungeon: Res<CurrentDungeon>,
    bend: Res<HallBend>,
    mut worn: ResMut<Worn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut places: Query<&mut Transform>,
    mut commands: Commands,
) {
    if dungeon.is_changed() {
        let wanted = worn_tiles(&dungeon);
        worn.lying.retain(|cell, (patch, _)| {
            let stays = wanted.contains_key(cell);
            if !stays {
                commands.entity(*patch).despawn();
            }
            stays
        });
        let (mesh, material) = worn
            .look
            .get_or_insert_with(|| {
                let size = ACROSS * CELL_SIZE;
                let picture = images.add(soft_edge());
                let material = materials.add(StandardMaterial {
                    base_color: WEAR,
                    base_color_texture: Some(picture),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                });
                (meshes.add(Rectangle::new(size, size)), material)
            })
            .clone();
        for (cell, side) in wanted {
            if worn.lying.contains_key(&cell) {
                continue;
            }
            let patch = commands
                .spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    lying_at(cell, side, &bend),
                ))
                .id();
            worn.lying.insert(cell, (patch, side));
        }
    } else if !bend.is_changed() && !worn.is_changed() {
        return;
    }
    for (&cell, &(patch, side)) in &worn.lying {
        if let Ok(mut place) = places.get_mut(patch) {
            place.set_if_neq(lying_at(cell, side, &bend));
        }
    }
}

/// Where a patch lies on its tile, towards the edge on `side`, with the
/// Hall's bend and twist.
fn lying_at((x, y): (i64, i64), side: Side, bend: &HallBend) -> Transform {
    let edge = match side {
        Side::North => -TOWARDS_EDGE,
        Side::South => TOWARDS_EDGE,
    };
    let straight = Vec3::new(x as f32, ABOVE, y as f32 + edge) * CELL_SIZE;
    let flat = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
    Transform::from_translation(bend.shift(straight, CELL_SIZE))
        .with_rotation(bend.tilt(x as f32) * flat)
}

/// The worn tiles of the stretch shown, by cell as the client counts it, with
/// the side their way in is on; none away from the Hall.
fn worn_tiles(dungeon: &CurrentDungeon) -> BTreeMap<(i64, i64), Side> {
    let Some(info) = &dungeon.info else {
        return BTreeMap::new();
    };
    let LevelKind::Hall { origin } = info.kind else {
        return BTreeMap::new();
    };
    let (min_x, _, width, _) = info.bounds;
    (min_x..min_x + width)
        .flat_map(|x| [(x, 0), (x, hall::WALKWAY_ROWS - 1)])
        .filter_map(|(x, y)| hall::inn_way_behind(x + origin, y).map(|(_, side)| ((x, y), side)))
        .collect()
}

/// A white square whose alpha falls off smoothly from the middle to nothing
/// at its edge: a soft round patch once tinted.
fn soft_edge() -> Image {
    let half = PIXELS as f32 / 2.0;
    let mut bytes = Vec::with_capacity((PIXELS * PIXELS * 4) as usize);
    for row in 0..PIXELS {
        for col in 0..PIXELS {
            let dx = (col as f32 + 0.5 - half) / half;
            let dy = (row as f32 + 0.5 - half) / half;
            let near = (1.0 - (dx * dx + dy * dy).sqrt()).clamp(0.0, 1.0);
            let alpha = (near * near * (3.0 - 2.0 * near) * 255.0) as u8;
            bytes.extend_from_slice(&[255, 255, 255, alpha]);
        }
    }
    Image::new(
        Extent3d {
            width: PIXELS,
            height: PIXELS,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

/// Leaving the dungeon view: the patches go.
pub(super) fn remove_worn(mut worn: ResMut<Worn>, mut commands: Commands) {
    for (_, (patch, _)) in std::mem::take(&mut worn.lying) {
        commands.entity(patch).despawn();
    }
}
