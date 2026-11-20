//! Trees (woodcutting plan PR 3): every wild tree (`Cell::Tree`) as its kind's
//! model (`plants::TreeModels`, the kind sent with the cells), and felled trees
//! as their stumps and as they grow back, fruit trees too (a standing fruit
//! tree is `fruit`'s), each standing on the ground in a variant and turned by
//! its cell.
//!
//! Unlike other plants, trees are remembered: a remembered one is drawn in sepia
//! where the sight mask says out of sight, and one in or beside sight also in
//! colour where it says in sight, so the edge of sight falls across a tree as
//! across a wall. Each tree is an entity under the scene, its copies children.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::dungeon::{Cell, Level, Sight};
use shared::fruit::Fruits;
use shared::tree::{TreeKind, TreeStage};

use super::herbs::HerbMaterials;
use super::sight_lines::SurfaceMaterial;
use super::town::cell_centre;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{CurrentDungeon, DrawnGround};
use crate::plants::{TreeModel, TreeModels};

/// Trees' sepia copies' materials, for where they are remembered, and the red
/// a tree flashes when chopped (`tree_flash`).
#[derive(Resource)]
pub(super) struct TreeMaterials {
    pub(super) memory: Handle<SurfaceMaterial>,
    pub(super) outline: Handle<SurfaceMaterial>,
    pub(super) flash: Handle<SurfaceMaterial>,
}

/// On a tree's coloured copy, which flashes red when it is chopped.
#[derive(Component)]
pub(super) struct TreeFill;

/// What a drawn tree shows, to tell when it must be drawn again.
#[derive(Clone, Debug, PartialEq)]
struct Drawn {
    model: TreeModel,
    /// Whether its coloured copy is drawn too.
    lit: bool,
    /// The ground under its middle, in tenths of a unit.
    ground: i32,
}

/// The trees drawn now, by cell, and what they were drawn for.
#[derive(Resource, Default)]
pub(super) struct DrawnTrees {
    trees: HashMap<(i64, i64), (Entity, Drawn)>,
    /// What was seen last time.
    built: Option<u64>,
}

impl DrawnTrees {
    /// The tree drawn at `cell`, if any.
    pub(super) fn at(&self, cell: (i64, i64)) -> Option<Entity> {
        self.trees.get(&cell).map(|(entity, _)| *entity)
    }
}

/// Draws the trees again when what is seen, the cells or the ground change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_trees(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, heights): (Res<CurrentDungeon>, Res<Shown>, Res<DrawnGround>),
    (lit, sepia): (Res<HerbMaterials>, Res<TreeMaterials>),
    mut models: ResMut<TreeModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnTrees>,
) {
    if drawn.built == Some(shown.looks) && !dungeon.is_changed() && !heights.is_changed() {
        return;
    }
    drawn.built = Some(shown.looks);
    let level = &dungeon.level;
    let mut keep = HashMap::with_capacity(drawn.trees.len());
    for (x, y) in remembered_cells(level, &dungeon.remembered) {
        let Some(model) = tree_model(level, (x, y)) else {
            continue;
        };
        let middle = cell_centre(x, y);
        let ground = heights.drawn_between(middle.x / CELL_SIZE, middle.z / CELL_SIZE) * CELL_SIZE;
        let now_drawn = Drawn {
            model,
            lit: shown
                .sight
                .as_ref()
                .is_some_and(|sight| near_sight(sight, x, y)),
            ground: (ground * 10.0).round() as i32,
        };
        if let Some((entity, was)) = drawn.trees.remove(&(x, y)) {
            if was == now_drawn {
                keep.insert((x, y), (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let Some(model) = models.get(&now_drawn.model, &mut meshes) else {
            continue;
        };
        let turn = super::fruit::looks(x, y).1;
        let mut tree = commands.spawn((
            Transform::from_translation(middle.with_y(ground))
                .with_rotation(Quat::from_rotation_y(turn)),
            Visibility::default(),
        ));
        tree.with_children(|tree| {
            let mut copy = |mesh: &Handle<Mesh>, material: &Handle<SurfaceMaterial>, fill: bool| {
                let mut part = tree.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    NotShadowCaster,
                ));
                if fill {
                    part.insert(TreeFill);
                }
            };
            copy(&model.memory, &sepia.memory, false);
            copy(&model.outline, &sepia.outline, false);
            if now_drawn.lit {
                copy(&model.fill, &lit.fill, true);
                copy(&model.outline, &lit.outline, false);
            }
        });
        let tree = tree.id();
        commands.entity(*scene).add_child(tree);
        keep.insert((x, y), (tree, now_drawn));
    }
    for (_, (entity, _)) in drawn.trees.drain() {
        commands.entity(entity).despawn();
    }
    drawn.trees = keep;
}

/// The remembered cells of `level` that may hold a tree: wild ones, and fruit
/// trees felled.
fn remembered_cells<'a>(
    level: &'a Level,
    remembered: &'a Sight,
) -> impl Iterator<Item = (i64, i64)> + 'a {
    let (min_x, min_y, width, height) = level.bounds();
    (min_y..min_y + height)
        .flat_map(move |y| (min_x..min_x + width).map(move |x| (x, y)))
        .filter(move |&(x, y)| {
            let tree = level.cell(x, y) == Cell::Tree || level.felled_at(x, y).is_some();
            tree && remembered.is_seen(x, y)
        })
}

/// The model of the tree at `(x, y)`, if one is drawn there: a wild one of its
/// kind (an oak, if the server hasn't said) as grown back, or a felled fruit
/// tree's.
pub(super) fn tree_model(level: &Level, (x, y): (i64, i64)) -> Option<TreeModel> {
    let variant = super::fruit::looks(x, y).0;
    let stage = level.felled_at(x, y);
    if let Some(plant) = level.fruit_at(x, y) {
        let def = Fruits::built_in().get(&plant.fruit)?;
        let stage = stage.filter(|_| def.plant.has_trunk())?;
        return Some(TreeModel::Fruit(def.key.clone(), stage, variant));
    }
    if level.cell(x, y) != Cell::Tree {
        return None;
    }
    let kind = level.tree_kind_at(x, y).unwrap_or(TreeKind::Oak);
    Some(TreeModel::Wild(
        kind,
        stage.unwrap_or(TreeStage::Grown),
        variant,
    ))
}

/// Whether `(x, y)` or a cell beside it is in sight: its tree may reach into
/// what is seen.
fn near_sight(sight: &Sight, x: i64, y: i64) -> bool {
    (-1..=1).any(|dy| (-1..=1).any(|dx| sight.is_seen(x + dx, y + dy)))
}
