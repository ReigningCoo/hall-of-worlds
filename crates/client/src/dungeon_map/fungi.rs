//! Fungus patches (fungi plan PR 4): each as its fungus's model
//! (`plants::FungusModels`) at the stage it has grown to by the world's clock
//! (`FungusPatch::stage`), in a variant and turned by its cell, standing on
//! the ground. Like trees, they are remembered: drawn grey where the sight
//! mask says out of sight, and in colour too where it says in sight. Picking
//! them goes as herbs' do (`herbs::pick`).

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::fungus::Fungi;

use super::herbs::HerbMaterials;
use super::sight_lines::SurfaceMaterial;
use super::town::cell_centre;
use super::trees::TreeMaterials;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{CurrentDungeon, DrawnGround, FungusShowcase, MapPosition, ServerClock};
use crate::plants::FungusModels;
use crate::player::Player;

mod showcase;

/// What a drawn patch shows, to tell when it must be drawn again.
#[derive(Clone, Debug, PartialEq)]
struct Drawn {
    fungus: String,
    stage: u8,
    /// Whether its coloured copy is drawn too.
    lit: bool,
    /// The ground under its middle, in tenths of a unit.
    ground: i32,
}

/// The patches drawn now, by cell, and what they were drawn for.
#[derive(Resource, Default)]
pub(super) struct DrawnFungi {
    cells: HashMap<(i64, i64), (Entity, Drawn)>,
    /// What was seen, the hour and the showcase last time.
    built: Option<(u64, u64, FungusShowcase)>,
}

/// Draws the patches again when what is seen, the hour, the patches, the
/// ground or the showcase change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_fungi(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, clock): (Res<CurrentDungeon>, Res<Shown>, Res<ServerClock>),
    (heights, showcase): (Res<DrawnGround>, Res<FungusShowcase>),
    player: Single<&MapPosition, With<Player>>,
    (lit, grey): (Res<HerbMaterials>, Res<TreeMaterials>),
    mut models: ResMut<FungusModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnFungi>,
) {
    let now = clock.minutes.max(0.0) as u64;
    let key = (shown.looks, now / 60, *showcase);
    if drawn.built == Some(key) && !dungeon.is_changed() && !heights.is_changed() {
        return;
    }
    drawn.built = Some(key);
    let wanted = match showcase.0 {
        Some(asked) => showcase::cells((player.x, player.y), asked),
        None => in_memory(&dungeon, &shown, now),
    };
    let mut keep = HashMap::with_capacity(wanted.len());
    for ((x, y), fungus, stage, in_sight) in wanted {
        let middle = cell_centre(x, y);
        let ground = heights.drawn_between(middle.x / CELL_SIZE, middle.z / CELL_SIZE) * CELL_SIZE;
        let now_drawn = Drawn {
            fungus,
            stage,
            lit: in_sight,
            ground: (ground * 10.0).round() as i32,
        };
        if let Some((entity, was)) = drawn.cells.remove(&(x, y)) {
            if was == now_drawn {
                keep.insert((x, y), (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let (variant, turn) = super::fruit::looks(x, y);
        let Some(model) = models.get(&now_drawn.fungus, stage, variant, &mut meshes) else {
            continue;
        };
        let mut patch = commands.spawn((
            Transform::from_translation(middle.with_y(ground))
                .with_rotation(Quat::from_rotation_y(turn)),
            Visibility::default(),
        ));
        patch.with_children(|patch| {
            let mut copy = |mesh: &Handle<Mesh>, material: &Handle<SurfaceMaterial>| {
                patch.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    NotShadowCaster,
                ));
            };
            copy(&model.memory, &grey.memory);
            copy(&model.outline, &grey.outline);
            if now_drawn.lit {
                copy(&model.fill, &lit.fill);
                copy(&model.outline, &lit.outline);
            }
        });
        let patch = patch.id();
        commands.entity(*scene).add_child(patch);
        keep.insert((x, y), (patch, now_drawn));
    }
    for (_, (entity, _)) in drawn.cells.drain() {
        commands.entity(entity).despawn();
    }
    drawn.cells = keep;
}

/// The remembered patches grown past nothing: where, which fungus, its stage
/// and whether it is in sight now.
fn in_memory(
    dungeon: &CurrentDungeon,
    shown: &Shown,
    now: u64,
) -> Vec<((i64, i64), String, u8, bool)> {
    let fungi = Fungi::built_in();
    (dungeon.level.fungus_patches())
        .filter(|&((x, y), _)| dungeon.remembered.is_seen(x, y))
        .filter_map(|((x, y), patch)| {
            let stage = patch.stage(fungi.get(&patch.fungus)?, now);
            let seen = (shown.sight.as_ref()).is_some_and(|sight| sight.is_seen(x, y));
            (stage > 0).then(|| ((x, y), patch.fungus.clone(), stage, seen))
        })
        .collect()
}
