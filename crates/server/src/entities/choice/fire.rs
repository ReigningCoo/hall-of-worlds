//! Open fires (campfires, lit fireplaces) that mobs whose race fears fire
//! keep away from (`BehaviourDef::fears_fire`): never stepping nearer one
//! within `FIRE_KEEP`, and never going up to anyone standing by one.

use std::collections::BTreeMap;

use super::super::{Taken, cells_apart};
use crate::ServerWorld;
use crate::connections::Client;
use crate::storage::SiteId;

/// How near an open fire a mob that fears fire comes, in cells.
pub(in super::super) const FIRE_KEEP: i64 = 3;

/// The open fires burning on each level in use.
pub(in super::super) type Fires = BTreeMap<(SiteId, u32), Vec<(i64, i64)>>;

/// The open fires on every level in use now: campfires as players' levels
/// show them (`sites::sync_campfires`), and a site's fireplaces lit.
pub(in super::super) fn fires_by_place(clients: &[Client], world: &mut ServerWorld) -> Fires {
    let mut fires = Fires::new();
    for client in clients {
        if let Some(visit) = &client.visit {
            let here = fires.entry(visit.population()).or_default();
            for (at, _) in visit.cells().campfires() {
                if !here.contains(&at) {
                    here.push(at);
                }
            }
        }
    }
    let now = crate::sites::campfires::fire_now(world);
    let places: Vec<(SiteId, u32)> = world.populations.0.keys().copied().collect();
    for (site, index) in places {
        if site == SiteId::Wilderness {
            continue;
        }
        let Some(level) = world.site(site).levels.get(&index) else {
            continue;
        };
        let lit = (level.fireplaces.iter())
            .filter(|(_, fireside)| fireside.is_lit(now))
            .map(|(&at, _)| at);
        let lit: Vec<(i64, i64)> = lit.collect();
        if !lit.is_empty() {
            fires.entry((site, index)).or_default().extend(lit);
        }
    }
    fires
}

/// Whether `cell` is within `FIRE_KEEP` of any of `fires`.
pub(in super::super) fn near_fire(fires: &[(i64, i64)], cell: (i64, i64)) -> bool {
    fires
        .iter()
        .any(|&fire| cells_apart(fire, cell) <= FIRE_KEEP)
}

/// `taken` with the cells a mob at `from` that fears fire won't step onto:
/// those within `FIRE_KEEP` of a fire and nearer it than `from`, so one by a
/// fire may only step away. `None` when no fire is near enough to matter.
pub(in super::super) fn with_fire(
    taken: &Taken,
    from: (i64, i64),
    fires: &[(i64, i64)],
) -> Option<Taken> {
    let near: Vec<(i64, i64)> = (fires.iter().copied())
        .filter(|&fire| cells_apart(fire, from) <= FIRE_KEEP + 2)
        .collect();
    if near.is_empty() {
        return None;
    }
    let mut feared = taken.clone();
    for fire in near {
        let mine = cells_apart(fire, from);
        for dy in -FIRE_KEEP..=FIRE_KEEP {
            for dx in -FIRE_KEEP..=FIRE_KEEP {
                let cell = (fire.0 + dx, fire.1 + dy);
                if cells_apart(fire, cell) < mine {
                    feared.insert(cell);
                }
            }
        }
    }
    Some(feared)
}
