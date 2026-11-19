//! Every tick: felled trees on players' levels grow back by world time (so
//! they stop with the server and follow the debug actions that change the
//! time). A tree that has reached a new stage is shared (`share_trees`), and
//! what is kept of it tidied: once grown, its entry goes and the seed's tree
//! stands again.

use shared::tree::TreeStage;

use super::{TreeChange, TreeNews, keep_tree, saved_tree, tidied};
use crate::ServerWorld;
use crate::connections::Client;

/// Moves on every felled tree on the players' levels that has grown a stage.
pub(crate) fn grow_trees(clients: &[Client], world: &mut ServerWorld) {
    let now = world.minutes().max(0.0) as u64;
    for visit in clients.iter().filter_map(|client| client.visit.as_ref()) {
        let felled: Vec<_> = visit.cells().felled_trees().collect();
        for ((x, y), stage) in felled {
            let key = visit.plant_key(x, y);
            let (tidy, grown) = tidied(saved_tree(world, key), now);
            keep_tree(world, key, tidy);
            let grown = grown.unwrap_or(TreeStage::Grown);
            let told = (world.tree_changes.iter()).any(|change| change.key == key);
            if grown != stage && !told {
                let news = TreeNews::Stage(grown);
                world.tree_changes.push(TreeChange { key, news });
            }
        }
    }
}
