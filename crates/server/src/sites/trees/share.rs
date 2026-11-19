//! Every tick: tells everyone whose level has a tree just changed. A blow
//! goes as news (`ServerMessage::TreeChopped`) to those who can see it, for
//! the client to flash; a tree felled or grown is set on their level (and its
//! mobs'), its cell sent again if they can see it.

use shared::protocol::ServerMessage;
use shared::tree::TreeStage;

use super::{TreeChange, TreeNews};
use crate::ServerWorld;
use crate::connections::Client;
use crate::sites::doors::changed_at;
use crate::sites::update_sight;
use crate::storage::SiteId;

/// Shares `world.tree_changes` with everyone they concern.
pub(crate) fn share_trees(clients: &mut [Client], world: &mut ServerWorld) {
    let changes = std::mem::take(&mut world.tree_changes);
    if changes.is_empty() {
        return;
    }
    for change in &changes {
        let (site, index, at) = change.key;
        if let (TreeNews::Stage(stage), false) = (change.news, site == SiteId::Wilderness)
            && let Some(population) = world.populations.0.get_mut(&(site, index))
        {
            population.set_felled(at, stage);
        }
    }
    for client in clients.iter_mut() {
        if !tell(client, &changes) {
            // Gone: noticed when its messages are next read.
            continue;
        }
    }
}

/// Tells one player of the changes on their level. `false` if they have gone.
fn tell(client: &mut Client, changes: &[TreeChange]) -> bool {
    let Some(visit) = &mut client.visit else {
        return true;
    };
    let mut news = Vec::new();
    let mut changed = false;
    for change in changes {
        let Some((x, y)) = visit.cell_of(change.key) else {
            continue;
        };
        match change.news {
            TreeNews::Chopped(done) => {
                if visit.can_see(x, y) {
                    news.push(ServerMessage::TreeChopped { x, y, done });
                }
            }
            TreeNews::Stage(stage) => {
                let wanted = Some(stage).filter(|&stage| stage != TreeStage::Grown);
                if visit.cells().felled_at(x, y) != wanted {
                    visit.level.level.set_felled(x, y, Some(stage));
                    changed_at(visit, x, y);
                    changed = true;
                }
            }
        }
    }
    if !news.iter().all(|message| client.link.send(message)) {
        return false;
    }
    !changed || update_sight(client)
}
