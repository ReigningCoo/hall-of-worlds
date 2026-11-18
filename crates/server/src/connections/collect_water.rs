//! A player collecting water (`Ability::CollectWater`, brewing plan decision
//! 5): facing water within reach with an empty bottle, each bottle a wind-up
//! of `potion::fill_ticks`; when one lands it is filled with the water the
//! place gives (`potion::water_from`), Water is practised, and the next empty
//! bottle starts, until there are none, the water is no longer faced, or the
//! player moves or is harmed (`stop_collecting`, as eating's effects end).
//! Once no empty bottle is left, it pours into a carried cask that takes the
//! water instead (`shared::cask`), `cask_litres` each `cask_fill_ticks`.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::cask::{CASK_PRACTICE, cask_fill_ticks, cask_litres, fill_cask_with};
use shared::potion::{
    COLLECT_PRACTICE, Liquid, Source, WATER_SKILL, Water, fill_ticks, filled, water_faced,
    water_from,
};
use shared::protocol::{Landed, ServerMessage};
use shared::skill::SkillId;

use super::Client;
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

impl Client {
    /// The water the player faces within reach on their level, and which water
    /// it gives: `None` off levels, or facing none.
    fn water_here(&self, world: &ServerWorld) -> Option<Water> {
        let visit = self.visit.as_ref()?;
        let (x, y) = water_faced(visit.cells(), visit.spot(), self.heading)?;
        let underground = visit
            .site()
            .is_some_and(|site| site.is_underground(visit.index()));
        let sea = visit.is_wilderness() && world.wilderness.wild.is_sea(x, y);
        let (own_x, own_y) = visit.position();
        let outside = visit.cells().cell(own_x, own_y).is_outside();
        let moonlit = !underground && outside && world.moonlit();
        Some(water_from(Source {
            underground,
            sea,
            moonlit,
        }))
    }

    /// Calls off a bottle being filled, with water or oil pressed
    /// (`press_oil.rs`), as moving or being harmed does: the player can act
    /// again at once. `false` if the client has gone.
    pub(crate) fn stop_collecting(&mut self, now: u64) -> bool {
        let Some(pending) = self.fight.pending else {
            return true;
        };
        if !matches!(
            pending.act,
            Act::CollectWater | Act::PressOil | Act::Grind | Act::Brew
        ) || pending.lands_at <= now
        {
            return true;
        }
        self.fight.pending = None;
        self.energy.cancel(now, self.speed, 0);
        self.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Cancelled,
        })
    }
}

/// What is filled next: an empty bottle, else a cask that takes this water.
enum Vessel {
    Bottle(shared::id::Id),
    Cask(shared::id::Id),
}

impl Client {
    fn vessel_for(&self, water: Water) -> Option<Vessel> {
        let bottle = self.inventory.empty_bottle().map(Vessel::Bottle);
        let cask = || {
            let cask = self.inventory.cask_to_fill(&Liquid::Water(water));
            cask.map(Vessel::Cask)
        };
        bottle.or_else(cask)
    }
}

/// Begins filling the next empty bottle or pouring into a cask, if there is
/// one and water is faced: whether the client is still there, and what it cost.
pub(super) fn start_collect(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let vessel = client
        .water_here(world)
        .and_then(|water| client.vessel_for(water));
    let Some(vessel) = vessel else {
        debug!("Not starting Collect Water: nothing to fill, or no water faced");
        return (true, 0);
    };
    let tick = world.action_clock.ticks();
    let lands_in = match vessel {
        Vessel::Bottle(_) => fill_ticks(&client.sheet.skills),
        Vessel::Cask(_) => cask_fill_ticks(),
    };
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::CollectWater,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::CollectWater,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the wind-up, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// A wind-up is over: a bottle is filled, or a cask poured into (if the water
/// is still faced and something to fill still carried) and Water practised,
/// then the next begins. `false` if the client has gone.
pub(super) fn land_collect(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let water = client.water_here(world);
    let vessel = water.and_then(|water| client.vessel_for(water));
    let (Some(water), Some(vessel)) = (water, vessel) else {
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    let practice = match vessel {
        Vessel::Bottle(stack) => {
            let Some(bottle) = client.inventory.take_one(stack, &mut new_id) else {
                return true;
            };
            let full = filled(bottle, water);
            if let Err(item) = client.inventory.give(full, world.stacks, &mut new_id) {
                crate::ground::put_down(client, world, item);
                client.ground_told.again = true;
            }
            COLLECT_PRACTICE
        }
        Vessel::Cask(cask) => {
            let litres = cask_litres(&client.sheet.skills);
            fill_cask_with(&mut client.inventory, cask, Liquid::Water(water), litres);
            CASK_PRACTICE
        }
    };
    if let Some(skill) = SkillId::find(WATER_SKILL) {
        let amount = client.learning.skills.take(skill, practice, tick);
        client.sheet.skills.practise(skill, amount);
        client.sheet.skills.used(skill, client.activity.played);
    }
    client.unsaved = true;
    client.update_speed(tick);
    let told = client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Filled(water),
        });
    if !told {
        return false;
    }
    // The next bottle, at once.
    let (stayed, cost) = start_collect(client, world);
    client.energy.spend(tick, client.speed, cost);
    stayed
}
