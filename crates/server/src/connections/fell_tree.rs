//! A player felling a tree (`Ability::FellTree`, woodcutting plan PR 2): a
//! one-handed axe held and a tree faced (`woodcutting::tree_faced`), in blows
//! of `BLOW_SECONDS`. Each blow that lands adds its share of the felling time
//! (`woodcutting::fell_ticks`, by Woodcutting and the axe) to the tree's
//! damage, kept by cell (`sites::keep_tree`) and shared: anyone chopping the
//! same tree adds to it, and everyone who sees it is told (`TreeChopped`).
//! Moving, being harmed or Esc stop it (`stop_collecting`), losing only the
//! blow under way; the damage stays a day. The blow that finishes it fells it:
//! a stump is left, a pile of logs (`Tree::logs`, by its kind and stage) is
//! dropped where the feller stands (kindling for a banana, papaya or saguaro,
//! and a ripe fruit tree's fruit too), and Woodcutting is practised.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::fruit::{Fruits, harvest, new_item};
use shared::item::Item;
use shared::protocol::{Landed, ServerMessage};
use shared::rng::Seed;
use shared::skill::SkillId;
use shared::tree::TreeStage;
use shared::woodcutting::{
    FELL_PRACTICE, WOODCUTTING_SKILL, fell_ticks, new_kindling, new_log, tree_faced,
};

use super::Client;
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;
use crate::sites::{TreeChange, TreeNews, keep_tree, saved_tree, tree_here, tree_stage};
use crate::storage::TreeSave;

/// How long one blow takes: damage is added, and onlookers told, a blow at a
/// time. Claude's choice.
pub(super) const BLOW_SECONDS: u64 = 2;

impl Client {
    /// The tree the player faces within reach on their level that could be
    /// felled, and how many ticks felling it from nothing takes them: `None`
    /// off levels, facing none, or holding no one-handed axe.
    fn tree_to_fell(&self) -> Option<((i64, i64), u64)> {
        let visit = self.visit.as_ref()?;
        let at = tree_faced(visit.cells(), visit.spot(), self.heading)?;
        let axe = self.inventory.held_axe()?;
        Some((at, fell_ticks(&self.sheet.skills, axe)))
    }
}

/// Begins a blow at the tree faced, if there is one: whether the client is
/// still there, and what it cost.
pub(super) fn start_fell(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let (Some(((x, y), whole)), Some(visit)) = (client.tree_to_fell(), client.visit.as_ref())
    else {
        debug!("Not starting Fell Tree: no tree faced, or no one-handed axe held");
        return (true, 0);
    };
    let now = world.minutes().max(0.0) as u64;
    let damage = saved_tree(world, visit.plant_key(x, y)).damage;
    let left = damage.unwrap_or_default().ticks_left(now, whole);
    let lands_in = left.clamp(1, BLOW_SECONDS * ACTION_TICKS_PER_SECOND as u64);
    let tick = world.action_clock.ticks();
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::FellTree {
            at: (x, y),
            ticks: lands_in,
        },
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::FellTree,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the blow, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// A blow at the tree at `at` has landed, `ticks` long: added to its damage,
/// felling it if that finishes it, else the next blow begins. `false` if the
/// client has gone.
pub(super) fn land_fell(
    client: &mut Client,
    world: &mut ServerWorld,
    at: (i64, i64),
    ticks: u64,
) -> bool {
    let faced = client.tree_to_fell().filter(|&(faced, _)| faced == at);
    let (Some((_, whole)), Some(visit)) = (faced, client.visit.as_ref()) else {
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    let now = world.minutes().max(0.0) as u64;
    let key = visit.plant_key(at.0, at.1);
    let save = saved_tree(world, key);
    let damage = save.damage.unwrap_or_default();
    if ticks >= damage.ticks_left(now, whole) {
        return fell(client, world, at);
    }
    let damage = damage.chopped(now, ticks, whole);
    keep_tree(
        world,
        key,
        TreeSave {
            damage: Some(damage),
            ..save
        },
    );
    let news = TreeNews::Chopped(damage.per_mille.min(999));
    world.tree_changes.push(TreeChange { key, news });
    let told = client.link.send(&ServerMessage::AbilityLanded {
        who: None,
        target: None,
        landed: Landed::Chopped {
            done: damage.per_mille.min(999),
        },
    });
    if !told {
        return false;
    }
    // The next blow, at once.
    let tick = world.action_clock.ticks();
    let (stayed, cost) = start_fell(client, world);
    client.energy.spend(tick, client.speed, cost);
    stayed
}

/// Fells the tree at `at`: a stump left, its logs dropped by the feller,
/// Woodcutting practised. `false` if the client has gone.
fn fell(client: &mut Client, world: &mut ServerWorld, at: (i64, i64)) -> bool {
    let Some(visit) = client.visit.as_ref() else {
        return true;
    };
    let now = world.minutes().max(0.0) as u64;
    let tick = world.action_clock.ticks();
    let key = visit.plant_key(at.0, at.1);
    let stage = tree_stage(visit, at.0, at.1, now);
    let tree = tree_here(world, visit, at.0, at.1);
    let felled = TreeSave {
        damage: None,
        felled: Some(now),
    };
    keep_tree(world, key, felled);
    for news in [TreeNews::Chopped(1000), TreeNews::Stage(TreeStage::Stump)] {
        world.tree_changes.push(TreeChange { key, news });
    }
    let wood = tree.as_ref().and_then(|tree| tree.wood());
    let logs = tree.as_ref().map_or(0, |tree| tree.logs(stage));
    let kindling = tree.as_ref().map_or(0, |tree| tree.kindling(stage));
    info!(
        "A tree was felled at {key:?}: {tree:?}, {stage:?}, {logs} logs or {kindling} kindling of {wood:?}"
    );
    let fruit = ripe_fruit(client, world, at, now);
    if let Some(wood) = wood {
        for _ in 0..logs {
            let log = new_log(wood, new_id(), Some(world.made_now()));
            crate::ground::put_down(client, world, log);
        }
        if kindling > 0 {
            let made = Some(world.made_now());
            let pieces = new_kindling(wood, u32::from(kindling), new_id(), made);
            crate::ground::put_down(client, world, pieces);
        }
    }
    if let Some(fruit) = fruit {
        crate::ground::put_down(client, world, fruit);
    }
    client.ground_told.again = true;
    if let Some(skill) = SkillId::find(WOODCUTTING_SKILL) {
        let amount = client.learning.skills.take(skill, FELL_PRACTICE, tick);
        client.sheet.skills.practise(skill, amount);
        client.sheet.skills.used(skill, client.activity.played);
    }
    client.unsaved = true;
    client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Chopped { done: 1000 },
        })
}

/// A ripe fruit tree's fruit at `at`, falling with it: harvested as if
/// Foraging and Orchards were 0 (owner, 2026-11-19). `None` for a wild tree
/// or one not ripe.
fn ripe_fruit(client: &Client, world: &ServerWorld, at: (i64, i64), now: u64) -> Option<Item> {
    let plant = client.visit.as_ref()?.fruit_at(at.0, at.1)?;
    let def = Fruits::built_in().get(&plant.fruit)?;
    if !plant.can_pick(def, now) {
        return None;
    }
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let count = harvest(def, (0, 0), &mut rng);
    Some(new_item(def, count, new_id(), Some(world.made_now())))
}
