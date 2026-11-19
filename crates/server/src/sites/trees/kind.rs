//! Which tree stands on a cell of a visit's level, and how grown it is, for
//! felling: a fruit plant with a trunk by its fruit; a wild `Cell::Tree` by
//! the kind rolled where it is kept (`Wilderness::tree_kind_at` for the
//! wilderness's own, `site_tree_kind` for a site's, through its poi).

use shared::dungeon::Cell;
use shared::fruit::Fruits;
use shared::tree::{Tree, TreeStage};

use crate::ServerWorld;
use crate::sites::Visit;
use crate::sites::wild::ensure_chunk;
use crate::storage::SiteId;

/// The tree at `(x, y)` on `visit`'s level, standing or growing back, if any.
pub(crate) fn tree_here(world: &mut ServerWorld, visit: &Visit, x: i64, y: i64) -> Option<Tree> {
    let level = visit.cells();
    if let Some(plant) = level.fruit_at(x, y) {
        let def = Fruits::built_in().get(&plant.fruit)?;
        return def
            .plant
            .has_trunk()
            .then(|| Tree::Fruit(plant.fruit.clone()));
    }
    if level.cell(x, y) != Cell::Tree {
        return None;
    }
    let (site, _, (kx, ky)) = visit.plant_key(x, y);
    let kind = match site {
        SiteId::Wilderness => {
            let chunk = world.wilderness.wild.chunk_of(kx, ky);
            ensure_chunk(world, chunk);
            let plan = &world.wilderness.chunk(chunk)?.plan;
            world.wilderness.wild.tree_kind_at(kx, ky, plan)
        }
        SiteId::Poi(chunk) if site != visit.id() => {
            let poi = world.pois.in_chunk(chunk)?;
            world.wilderness.wild.site_tree_kind(&poi, kx, ky)
        }
        _ => {
            let poi = *visit.poi()?;
            world.wilderness.wild.site_tree_kind(&poi, kx, ky)
        }
    };
    Some(Tree::Wild(kind))
}

/// How grown the tree at `(x, y)` on `visit`'s level is at game minute `now`:
/// as it has grown back if felled, a fruit tree by its own stage
/// (`Tree::fruit_stage`), else grown.
pub(crate) fn tree_stage(visit: &Visit, x: i64, y: i64, now: u64) -> TreeStage {
    let level = visit.cells();
    if let Some(stage) = level.felled_at(x, y) {
        return stage;
    }
    let fruit = level.fruit_at(x, y).and_then(|plant| {
        let def = Fruits::built_in().get(&plant.fruit)?;
        Some(Tree::fruit_stage(plant.stage(def, now)))
    });
    fruit.unwrap_or(TreeStage::Grown)
}
