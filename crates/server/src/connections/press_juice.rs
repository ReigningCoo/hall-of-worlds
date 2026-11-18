//! A player pressing juice (`Ability::PressJuice`, drinks plan PR 3): the
//! client names a stack of fruit carried (`UseAbility`'s `target`), and each
//! pressing is a wind-up of `juice::press_ticks`; when one lands, up to
//! `juice::PRESSING_GRAMS` of the fruit is used up, one at a time, its juice
//! (`juice::ml_from`) poured into a carried cask or vessel that takes it
//! (`Inventory::juice_target`), and Alchemy practised; then the next begins,
//! until the fruit runs out, nothing carried takes its juice, or the player
//! moves or is harmed (`stop_collecting`, as for Press Oil). `can_use` has
//! already checked the oil press is carried. The fruit is kept as the client's
//! `pressing` (`Pressing::Fruit`).

use bevy::prelude::*;
use shared::ability::{ALCHEMY, Ability};
use shared::fruit::Fruits;
use shared::id::Id;
use shared::juice::{JUICE_PRACTICE, PRESSING_GRAMS, ml_from, press_ticks, with_juice};
use shared::potion::{Liquid, Pressing};
use shared::protocol::{Landed, ServerMessage};
use shared::skill::SkillId;

use super::Client;
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

/// Press Juice asked for on `stack`: its fruit becomes what is pressed, and
/// the first pressing begins if it can.
pub(super) fn choose_juice(
    client: &mut Client,
    world: &mut ServerWorld,
    stack: Option<Id>,
) -> (bool, u32) {
    let item = stack.and_then(|stack| client.inventory.find(stack));
    let fruit = item.and_then(|item| item.top().fruit());
    client.pressing = fruit.map(|form| Pressing::Fruit(form.fruit.clone()));
    start_juice(client, world)
}

/// The fruit being pressed, if it can be pressed now.
fn fruit_to_press(client: &Client) -> Option<String> {
    match &client.pressing {
        Some(Pressing::Fruit(fruit)) if client.inventory.can_press_juice(fruit) => {
            Some(fruit.clone())
        }
        _ => None,
    }
}

/// Begins the next pressing, if it can be: whether the client is still there,
/// and what it cost.
fn start_juice(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    if fruit_to_press(client).is_none() {
        debug!("Not pressing juice: nothing chosen, none left, or nowhere for it");
        client.pressing = None;
        return (true, 0);
    }
    let tick = world.action_clock.ticks();
    let lands_in = press_ticks(&client.sheet.skills);
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::PressJuice,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::PressJuice,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the wind-up, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// A pressing's wind-up is over: the fruit is pressed into the cask or vessel
/// (if both are still carried), Alchemy practised, then the next begins.
/// `false` if the client has gone.
pub(super) fn land_juice(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let fruit = fruit_to_press(client);
    let target = (fruit.as_ref()).and_then(|fruit| client.inventory.juice_target(fruit));
    let (Some(fruit), Some(target)) = (fruit, target) else {
        client.pressing = None;
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    let room = room_in(client, target);
    let ml = press(client, &fruit, room);
    pour_in(client, target, Liquid::Juice(fruit), ml);
    if let Some(skill) = SkillId::find(ALCHEMY) {
        let amount = client.learning.skills.take(skill, JUICE_PRACTICE, tick);
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
            landed: Landed::Juiced(ml),
        });
    if !told {
        return false;
    }
    // The next pressing, at once.
    let (stayed, cost) = start_juice(client, world);
    client.energy.spend(tick, client.speed, cost);
    stayed
}

/// How many more millilitres the carried `target` takes.
fn room_in(client: &Client, target: Id) -> u32 {
    let held = client.inventory.find(target);
    held.and_then(|item| shared::pour::Held::of(item.top()))
        .map_or(0, |held| held.room)
}

/// Takes `fruit` one at a time, smallest stacks first, until
/// `PRESSING_GRAMS` have gone or their juice fills `room`: the millilitres of
/// juice they gave, no more than `room` (the rest is lost in the press).
fn press(client: &mut Client, fruit: &str, room: u32) -> u32 {
    let Some(def) = Fruits::built_in().get(fruit) else {
        return 0;
    };
    let pressing = Pressing::Fruit(fruit.to_owned());
    let (mut grams, mut ml) = (0, 0);
    while grams < PRESSING_GRAMS && ml < room {
        let Some(stack) = client.inventory.stack_to_press(&pressing) else {
            break;
        };
        let Some(one) = client.inventory.take_one(stack, &mut new_id) else {
            break;
        };
        grams += one.own_weight().max(1);
        ml += ml_from(def, one.own_weight());
    }
    ml.min(room)
}

/// Pours `ml` of `juice` into the carried cask or vessel `target`.
fn pour_in(client: &mut Client, target: Id, juice: Liquid, ml: u32) {
    let Some(item) = client.inventory.find(target) else {
        return;
    };
    let changed = with_juice(item.top(), juice, ml);
    client.inventory.replace_carried(changed);
}
