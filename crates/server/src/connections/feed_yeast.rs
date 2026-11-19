//! Feeding yeast and beginning starters (`ClientMessage::Yeast`, drinks plan
//! PR 8): flour let go on a carried yeast stack feeds it, the neediest first,
//! with fresh water carried (`yeast::plan_feeding`); flour let go on a bottle
//! of fresh water turns it into a starter (`yeast::can_start`). Fermenting is
//! practised for each. Answered with `Inventory`, refusals only logged (the
//! client checks the same rules first).

use bevy::prelude::*;
use shared::ferment::FERMENTING;
use shared::id::Id;
use shared::protocol::YeastAsk;
use shared::rng::Seed;
use shared::skill::SkillId;
use shared::yeast::{FEED_PRACTICE, STARTER_PRACTICE, can_start, drawn, new_starter, plan_feeding};

use super::Client;
use super::grind::{give, practise};
use super::pour::{put_back, side};
use crate::ServerWorld;
use crate::ids::new_id;

/// Answers a `YeastAsk`: whether the client is still there.
pub(super) fn answer(client: &mut Client, world: &mut ServerWorld, ask: YeastAsk) -> bool {
    let done = match ask {
        YeastAsk::Feed { yeast, flour } => fed(client, world, yeast, flour),
        YeastAsk::Start { water, flour } => started(client, world, water, flour),
    };
    if let Err(why) = done {
        warn!("{} refused {ask:?}: {why}", client.in_log());
    }
    client.unsaved = true;
    client.ground_told.again = true;
    client.send_inventory()
}

/// Feeds the carried yeast stack `yeast` from the flour stack `flour`.
fn fed(
    client: &mut Client,
    world: &mut ServerWorld,
    yeast: Id,
    flour: Id,
) -> Result<(), &'static str> {
    if client.inventory.is_worn(yeast) {
        return Err("yeast worn");
    }
    let stack = client.inventory.find(yeast).ok_or("no such yeast")?;
    let supplies = client.inventory.feed_supplies(flour);
    let plan = plan_feeding(stack, supplies).map_err(|why| why.words())?;
    for _ in 0..plan.took.water {
        draw_water(client, world)?;
    }
    for _ in 0..plan.took.bottles {
        let bottles = client.inventory.empty_bottles().ok_or("no empty bottle")?;
        client.inventory.take_one(bottles, &mut new_id);
    }
    client.inventory.use_flour(flour, plan.took.flour);
    let fermenting =
        (SkillId::find(FERMENTING)).map_or(0, |skill| client.sheet.skills.total(skill));
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let mut roll = || rng.range_u64(0, 100) as u32;
    let (splits, _) = (client.inventory).feed_yeasts(yeast, &plan, fermenting, &mut roll);
    for mut split in splits {
        split.id = new_id();
        give(client, world, split);
    }
    let tick = world.action_clock.ticks();
    let practice = FEED_PRACTICE * plan.yeasts.len() as u32;
    practise(client, FERMENTING, practice, tick);
    Ok(())
}

/// Takes 100 ml of fresh water from what is carried.
fn draw_water(client: &mut Client, world: &mut ServerWorld) -> Result<(), &'static str> {
    let source = client.inventory.water_source().ok_or("no water")?;
    let source = side(client, world, source)?;
    let after = drawn(source.stack.top()).ok_or("no water")?;
    put_back(client, world, &source, after);
    Ok(())
}

/// Begins a starter from a bottle off the carried stack `water` and a sack of
/// the flour stack `flour`.
fn started(
    client: &mut Client,
    world: &mut ServerWorld,
    water: Id,
    flour: Id,
) -> Result<(), &'static str> {
    let bottle = client.inventory.find(water).ok_or("no such bottle")?;
    let sack = client.inventory.find(flour).ok_or("no such flour")?;
    if client.inventory.is_worn(water) || client.inventory.is_worn(flour) {
        return Err("worn");
    }
    can_start(sack.top(), bottle.top()).map_err(|why| why.words())?;
    client.inventory.take_one(water, &mut new_id);
    client.inventory.use_flour(flour, 1);
    give(client, world, new_starter(new_id(), Some(world.made_now())));
    let tick = world.action_clock.ticks();
    practise(client, FERMENTING, STARTER_PRACTICE, tick);
    Ok(())
}
