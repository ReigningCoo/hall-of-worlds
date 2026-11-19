//! What is kept of each tree (`storage::TreeSave`): read, written, tidied
//! (damage gone after a day, an entry dropped once its tree has grown again,
//! so the seed's tree takes over), and put on a level as it is made.

use shared::dungeon::Level;
use shared::tree::TreeStage;

use crate::ServerWorld;
use crate::sites::plants::PlantKey;
use crate::storage::{SiteId, TreeSave};

/// What changed of a tree, for `share_trees`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TreeNews {
    /// Chopped at: this much of it is felled, per mille (1000: it fell).
    Chopped(u16),
    /// It has grown back this far (`Grown`: standing again).
    Stage(TreeStage),
}

/// A tree just changed, where it is kept.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TreeChange {
    pub(crate) key: PlantKey,
    pub(crate) news: TreeNews,
}

/// What is kept of the tree at `key`: nothing, if nothing.
pub(crate) fn saved_tree(world: &mut ServerWorld, (site, index, at): PlantKey) -> TreeSave {
    let level = world.site(site).levels.get(&index);
    level
        .and_then(|level| level.trees.get(&at))
        .copied()
        .unwrap_or_default()
}

/// Keeps `save` for the tree at `key`, dropping its entry once it holds nothing.
pub(crate) fn keep_tree(world: &mut ServerWorld, (site, index, at): PlantKey, save: TreeSave) {
    if save == saved_tree(world, (site, index, at)) {
        return;
    }
    let levels = &mut world.site_mut(site).levels;
    if save == TreeSave::default() {
        if let Some(level) = levels.get_mut(&index) {
            level.trees.remove(&at);
        }
    } else {
        levels.entry(index).or_default().trees.insert(at, save);
    }
}

/// `save` at game minute `now`, tidied (damage over a day old gone, a felled
/// tree that has grown forgotten), with how far its tree has grown back if it
/// was felled and isn't grown yet.
pub(crate) fn tidied(save: TreeSave, now: u64) -> (TreeSave, Option<TreeStage>) {
    let damage = save.damage.filter(|damage| damage.left(now) > 0);
    let stage = (save.felled)
        .map(|at| TreeStage::after_felling(now.saturating_sub(at)))
        .filter(|&stage| stage != TreeStage::Grown);
    let felled = save.felled.filter(|_| stage.is_some());
    (TreeSave { damage, felled }, stage)
}

/// Sets on `level` how far each tree felled in `places` (sites and levels) has
/// grown back at game minute `now`, by `cell_of` (where a kept cell is on
/// `level`, if it is), tidying what is kept as it goes.
pub(crate) fn put_felled(
    world: &mut ServerWorld,
    level: &mut Level,
    places: &[(SiteId, u32)],
    cell_of: impl Fn(crate::sites::plants::PlantKey) -> Option<(i64, i64)>,
) {
    let now = world.minutes().max(0.0) as u64;
    for &(site, index) in places {
        let kept: Vec<((i64, i64), TreeSave)> = (world.site(site).levels.get(&index))
            .map(|level| level.trees.iter().map(|(&at, &save)| (at, save)).collect())
            .unwrap_or_default();
        for (at, save) in kept {
            let key = (site, index, at);
            let (tidy, stage) = tidied(save, now);
            keep_tree(world, key, tidy);
            if let (Some(stage), Some((x, y))) = (stage, cell_of(key)) {
                level.set_felled(x, y, Some(stage));
            }
        }
    }
}
