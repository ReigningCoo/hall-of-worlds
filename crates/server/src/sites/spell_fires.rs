//! Spells and fires on a visit's level (magic plan PR 11): Fire lights a
//! fireplace on a cell it lands on with a piece of kindling from its basket;
//! Water and steam put out a campfire there, or a fireplace's or hearth's
//! fire (never one always lit). Players are told by the usual syncing
//! (`sync_campfires`, `share_benches`).

use shared::furniture::{Furniture, Held};
use shared::id::Id;
use shared::item::Item;
use shared::material::Material;

use super::campfires::{fire_now, keep_campfire, saved_campfire};
use super::{Visit, bench_tools, fireside, keep_bench, keep_held};
use crate::ServerWorld;

/// Lights the fireplace covering `(x, y)` with a piece of its basket's
/// kindling, if it isn't lit. Whether it was.
pub(crate) fn light_by_spell(world: &mut ServerWorld, visit: &Visit, (x, y): (i64, i64)) -> bool {
    let Some(here) = visit.held_at(x, y) else {
        return false;
    };
    let Furniture::Fireplace { fireplace } = here.piece.furniture else {
        return false;
    };
    let now = fire_now(world);
    let mut kept = fireside(world, here.key, fireplace);
    if kept.is_lit(now) {
        return false;
    }
    let Some((id, wood)) = first_kindling(kept.items()) else {
        return false;
    };
    let Some(mut stack) = kept.take(id) else {
        return false;
    };
    if kept.burn(now, wood).is_none() {
        return false;
    }
    stack.count = stack.count.saturating_sub(1);
    if stack.count > 0 {
        // Its place was just freed, so there is room.
        let _ = kept.put(stack);
    }
    keep_held(world, here.key, Held::Fire(kept));
    true
}

/// The first kindling in a basket, and its wood.
fn first_kindling<'a>(mut items: impl Iterator<Item = &'a Item>) -> Option<(Id, Material)> {
    let item = items.next()?;
    let wood = item.kindling().map_or(Material::Wood, |form| form.wood);
    Some((item.id, wood))
}

/// Puts out the fire on `(x, y)`: a campfire there (what is left of it
/// lost), or the fire of a fireplace or hearth covering it. Whether one was.
pub(crate) fn douse(world: &mut ServerWorld, visit: &Visit, (x, y): (i64, i64)) -> bool {
    let now = fire_now(world);
    let key = visit.plant_key(x, y);
    if saved_campfire(world, key).is_some_and(|fire| fire.is_lit(now)) {
        keep_campfire(world, key, None);
        return true;
    }
    if let Some(hearth) = visit.bench_at(x, y).filter(|b| b.craft.is_hearth()) {
        let mut tools = bench_tools(world, hearth.key, hearth.craft);
        if tools.always_lit || !tools.fire.as_ref().is_some_and(|f| f.is_lit(now)) {
            return false;
        }
        tools.fire = None;
        keep_bench(world, hearth.key, tools);
        return true;
    }
    let Some(here) = visit.held_at(x, y) else {
        return false;
    };
    let Furniture::Fireplace { fireplace } = here.piece.furniture else {
        return false;
    };
    let mut kept = fireside(world, here.key, fireplace);
    if kept.always_lit || !kept.is_lit(now) {
        return false;
    }
    kept.fire = None;
    keep_held(world, here.key, Held::Fire(kept));
    true
}
