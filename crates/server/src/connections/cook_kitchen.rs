//! Where a cook is working (cooking plan PR 9), checked before each run: the
//! hearth asked for (`CookOrder::bench`), which they must face, its tools,
//! heat, Oven and storage (rested to now), or a fireplace asked for (a
//! hearth's heat while lit, no Oven, no tools: tavern furniture plan PR 4);
//! else a town's virtual hearth;
//! else a lit campfire faced; else no heat. Water faced counts anywhere.

use shared::campfire::campfire_faced;
use shared::cooking::{CookOrder, Heat, Kitchen};
use shared::crafting_bench::{Craft, town_hearth, town_tools};
use shared::fireplace::fireplace_faced;
use shared::furniture::Furniture;
use shared::item::{Item, ItemKind};
use shared::potion::water_faced;

use super::Client;
use crate::ServerWorld;
use crate::sites::campfires::{fire_now, saved_campfire};
use crate::sites::{BenchKey, fireside, settle_storage};

/// The kitchen found for an order, owning what `Kitchen` borrows.
pub(super) struct KitchenHere {
    pub(super) heat: Heat,
    pub(super) oven: bool,
    pub(super) tools: Vec<ItemKind>,
    pub(super) storage: Vec<Item>,
    pub(super) water_faced: bool,
    /// The hearth worked at, if one was asked for: where its tools are kept.
    pub(super) bench: Option<(BenchKey, Craft)>,
    /// Whether it is a fireplace asked for (which leaves Ash as a hearth).
    pub(super) fireplace: bool,
}

impl KitchenHere {
    /// As the rules take it.
    pub(super) fn kitchen(&self) -> Kitchen<'_> {
        Kitchen {
            heat: self.heat,
            oven: self.oven,
            tools: &self.tools,
            storage: &self.storage,
            water_faced: self.water_faced,
        }
    }

    /// Whether it is a real hearth or fireplace lit (one that leaves Ash
    /// from cooking).
    pub(super) fn lit_hearth(&self) -> bool {
        (self.bench.is_some() || self.fireplace) && self.heat == Heat::Hearth
    }
}

/// The kitchen the cook is at for `order`; `Err` if it asks for a hearth
/// they aren't at, facing it.
pub(super) fn kitchen_here(
    client: &Client,
    world: &mut ServerWorld,
    order: &CookOrder,
) -> Result<KitchenHere, ()> {
    let visit = client.visit.as_ref();
    let water = visit
        .is_some_and(|visit| water_faced(visit.cells(), visit.spot(), client.heading).is_some());
    let mut here = KitchenHere {
        heat: Heat::Cold,
        oven: false,
        tools: Vec::new(),
        storage: Vec::new(),
        water_faced: water,
        bench: None,
        fireplace: false,
    };
    let now = fire_now(world);
    if let Some(asked) = order.bench {
        let visit = visit.ok_or(())?;
        if let Some(piece) = fireplace_faced(visit.cells(), visit.spot(), client.heading)
            && piece.cells().any(|cell| cell == asked)
        {
            let here_key = visit.held_at(piece.at.0, piece.at.1).ok_or(())?.key;
            let Furniture::Fireplace { fireplace } = piece.furniture else {
                return Err(());
            };
            // A steady heat, but no Oven nor tools of its own (owner's).
            if fireside(world, here_key, fireplace).is_lit(now) {
                here.heat = Heat::Hearth;
            }
            here.fireplace = true;
            return Ok(here);
        }
        let bench = visit.bench_faced(client.heading).ok_or(())?;
        if !bench.craft.is_hearth() || !bench.piece.cells().any(|cell| cell == asked) {
            return Err(());
        }
        let celsius = world.habitat_at(client.world_position()).celsius;
        let tools = settle_storage(world, bench.key, bench.craft, celsius);
        let lit = tools.is_lit(now);
        here.heat = if lit { Heat::Hearth } else { Heat::Cold };
        here.oven = lit;
        here.tools = tools.tools().iter().map(|tool| tool.kind).collect();
        here.storage = tools.storage().to_vec();
        here.bench = Some((bench.key, bench.craft));
        return Ok(here);
    }
    let Some(visit) = visit else {
        return Ok(here);
    };
    if visit.site().is_some_and(|site| town_hearth(site.kind())) {
        here.heat = Heat::Hearth;
        here.oven = true;
        here.tools = town_tools().to_vec();
        return Ok(here);
    }
    let level = visit.cells();
    let burning = |x: i64, y: i64| level.campfire_at(x, y).is_some();
    if let Some((x, y)) = campfire_faced(level, visit.spot(), client.heading, burning) {
        let key = visit.plant_key(x, y);
        if saved_campfire(world, key).is_some_and(|fire| fire.is_lit(now)) {
            here.heat = Heat::Fire;
        }
    }
    Ok(here)
}
