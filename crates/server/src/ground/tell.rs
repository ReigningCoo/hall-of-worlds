//! Telling players about the ground: what is in the bags in their reach, and
//! which bags they see.

use shared::ground::Load;
use shared::protocol::ServerMessage;
use shared::spot::Spot;

use super::bags::{bags_in_reach, standing_at};
use crate::ServerWorld;
use crate::connections::{Client, Phase};
use shared::ground::in_reach;

/// What was in reach: where each bag was, and each thing in them by id, with
/// how many there were of it.
pub(super) type InReach = (Vec<Spot>, Vec<(shared::id::Id, u32)>);

/// Tells each player what is in their reach, when it isn't what they were last
/// told. A client that has gone is noticed when its messages are next read.
pub(crate) fn sync(clients: &mut [Client], world: &mut ServerWorld) {
    settle_yeast(clients, world);
    for client in clients {
        if client.phase != Phase::Playing {
            continue;
        }
        let Some((id, index, at)) = standing_at(client) else {
            continue;
        };
        let size = client.size();
        // The player's own quest item lying here, as if in a bag of its own.
        let find = crate::quests::find_here(client, world);
        let level = world.site(id).levels.get(&index);
        let ground = level.map(|level| &level.ground);
        let mut bags = ground.map_or_else(Vec::new, |ground| bags_in_reach(ground, size, at));
        let mut items: Vec<_> = (bags.iter())
            .flat_map(|bag| ground.into_iter().flat_map(move |ground| &ground[bag]))
            .cloned()
            .collect();
        if let Some(find) = find.filter(|find| in_reach(size, at, find.at)) {
            if !bags.contains(&find.at) {
                bags.push(find.at);
            }
            items.push(find.item);
        }
        let counted = items.iter().map(|item| (item.id, item.count())).collect();
        let reached = (!bags.is_empty()).then(|| (bags.clone(), counted));
        if reached != client.ground_told.reached || client.ground_told.again {
            let items = client.as_seen(items);
            client
                .link
                .send(&ServerMessage::GroundItems { bags, items });
            client.ground_told.reached = reached;
            client.ground_told.again = false;
        }
        tell_bags(client, world);
    }
}

/// Yeast lying in players' reach lets the real time pass (`shared::yeast`), so
/// what they see of it is up to date: everyone on a level where some lost a
/// quality step or passed an hour on its clock is told the ground again, and
/// its site saved.
fn settle_yeast(clients: &mut [Client], world: &mut ServerWorld) {
    let now = crate::quests::unix_now().max(0) as u64;
    let mut seen = Vec::new();
    for client in clients.iter() {
        let Some((id, index, at)) = standing_at(client) else {
            continue;
        };
        // Unmarked: a save left with an older clock counts the same time
        // when next settled. Marked once something is seen to change.
        let Some(level) = world.site(id).levels.get_mut(&index) else {
            continue;
        };
        for bag in bags_in_reach(&level.ground, client.size(), at) {
            // Every item settles: no stopping at the first that changed.
            let items = level.ground.get_mut(&bag).into_iter().flatten();
            let settled: Vec<bool> =
                (items.map(|item| item.settle_lying_yeast(now).seen())).collect();
            if settled.contains(&true) {
                seen.push((id, index));
            }
        }
    }
    for &(id, _) in &seen {
        world.site_mut(id);
    }
    for client in clients.iter_mut() {
        let here = standing_at(client).map(|(id, index, _)| (id, index));
        if here.is_some_and(|here| seen.contains(&here)) {
            client.ground_told.again = true;
        }
    }
}

/// Tells a client about bags: on arriving, those its character remembers; after that
/// only what has just been seen to differ, which is all its character remembers too.
fn tell_bags(client: &mut Client, world: &mut ServerWorld) {
    let Some((id, index, _)) = standing_at(client) else {
        return;
    };
    let level = world.site(id).levels.get(&index);
    let ground = level.map(|level| &level.ground);
    let mut piles: Vec<(Spot, Load)> = ground
        .into_iter()
        .flat_map(|ground| ground.iter())
        .map(|(&at, items)| (at, Load::of(items)))
        .collect();
    if let Some(find) = crate::quests::find_here(client, world) {
        piles.push((find.at, Load::of(std::slice::from_ref(&find.item))));
    }
    let levels = client.character.bags.get(&id);
    let mut known = levels
        .and_then(|levels| levels.get(&index))
        .cloned()
        .unwrap_or_default();
    if !client.ground_told.bags_told {
        client.ground_told.bags_told = true;
        for (&at, &load) in known.iter() {
            client.link.send(&ServerMessage::GroundSeen { at, load });
        }
    }
    let Some(visit) = client.visit.as_mut() else {
        return;
    };
    let mut changed = false;
    for &(at, load) in &piles {
        let (x, y) = at.cell();
        if known.get(&at) != Some(&load) && visit.can_see(x, y) {
            known.insert(at, load);
            changed = true;
            client.link.send(&ServerMessage::GroundSeen { at, load });
        }
    }
    let gone: Vec<Spot> = (known.keys())
        .filter(|at| !piles.iter().any(|(pile, _)| pile == *at))
        .filter(|at| {
            let (x, y) = at.cell();
            visit.can_see(x, y)
        })
        .copied()
        .collect();
    for at in gone {
        known.remove(&at);
        changed = true;
        client.link.send(&ServerMessage::GroundGone { at });
    }
    if changed {
        let levels = client.character.bags.entry(id).or_default();
        levels.insert(index, known);
        client.unsaved = true;
    }
}
