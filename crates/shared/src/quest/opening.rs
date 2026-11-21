//! Standing needed for a town's quests (owner: a stranger is offered only a
//! few, and more as they become better known): a few givers' first quests
//! need nothing, the rest Known, Trusted or Honoured, and each later quest of
//! a chain one tier more than the one before.

use std::collections::{BTreeMap, BTreeSet};

use super::givers::quest_seed;
use super::world::size;
use super::{Need, Quest, Requirement, Slot, Standing};
use crate::poi::Poi;
use crate::rng::Seed;

/// First quests open to a stranger, by town size (hamlet, village, town,
/// city). *To balance.*
pub(super) const OPEN: [usize; 4] = [2, 3, 3, 4];

/// Percent of the other first quests needing Honoured standing (in towns
/// and cities), then of the rest needing only Known (else Trusted). *To
/// balance.*
const HONOURED_PERCENT: u64 = 15;
const KNOWN_PERCENT: u64 = 50;

/// Gives the givers' chains their standing needs. Run once the town's other
/// needs are set: only first quests needing no other quest, alignment or
/// moon are open.
pub(super) fn add_standing_needs(town: &Poi, quests: &mut [Quest]) {
    let seed = quest_seed(town).derive("standing needs");
    let size = size(town.kind).unwrap_or(0);
    let open = open_chains(seed, size, quests);
    let mut firsts = BTreeMap::new();
    for quest in quests.iter() {
        if let Slot::Giver {
            building,
            slot,
            chain: 0,
        } = quest.id.slot
        {
            let tier = if open.contains(&(building, slot)) {
                0
            } else {
                first_tier(seed, size, building, slot)
            };
            firsts.insert((building, slot), tier);
        }
    }
    for quest in quests.iter_mut() {
        let Slot::Giver {
            building,
            slot,
            chain,
        } = quest.id.slot
        else {
            continue;
        };
        let first = firsts.get(&(building, slot)).copied().unwrap_or(0);
        let tier = (first + usize::from(chain)).min(Standing::Honoured as usize);
        if tier > 0 {
            quest.requirements.push(Requirement::shown(Need::Standing {
                town: town.chunk,
                least: Standing::ALL[tier],
            }));
        }
    }
}

/// The chains (building, slot) whose first quest a stranger may take: by a
/// hash, from first quests needing no other quest, alignment or moon (a
/// skill is fine), one a building.
fn open_chains(seed: Seed, size: usize, quests: &[Quest]) -> BTreeSet<(u16, u8)> {
    let mut free: Vec<(u64, u16, u8)> = (quests.iter())
        .filter(|quest| (quest.requirements.iter()).all(|r| matches!(r.need, Need::Skill { .. })))
        .filter_map(|quest| match quest.id.slot {
            Slot::Giver {
                building,
                slot,
                chain: 0,
            } => {
                let order = seed
                    .derive("open")
                    .at(&[i64::from(building), i64::from(slot)]);
                Some((order, building, slot))
            }
            _ => None,
        })
        .collect();
    free.sort_unstable();
    let most = OPEN[size.min(OPEN.len() - 1)];
    let mut buildings = BTreeSet::new();
    let mut open = BTreeSet::new();
    for (_, building, slot) in free {
        if open.len() < most && buildings.insert(building) {
            open.insert((building, slot));
        }
    }
    open
}

/// The tier (index into `Standing::ALL`) a closed chain's first quest needs.
fn first_tier(seed: Seed, size: usize, building: u16, slot: u8) -> usize {
    let mut rng = Seed::new(
        seed.derive("tier")
            .at(&[i64::from(building), i64::from(slot)]),
    )
    .rng();
    let honoured = rng.range_u64(0, 100) < HONOURED_PERCENT;
    if size >= 2 && honoured {
        Standing::Honoured as usize
    } else if rng.range_u64(0, 100) < KNOWN_PERCENT {
        Standing::Known as usize
    } else {
        Standing::Trusted as usize
    }
}

#[cfg(test)]
mod tests;
