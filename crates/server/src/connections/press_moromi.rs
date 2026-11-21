//! A moromi pressed with the oil press (Press Oil on a moromi, cooking plan
//! PR 12): when the wind-up lands, the moromi `Inventory::moromi_to_press`
//! names is used up whole and as many empty bottles as it gives
//! (`food::soy_sauce_bottles`) are filled with soy sauce at its quality,
//! Alchemy practised for each; then the next moromi begins, if there is one
//! and the bottles for it.

use shared::food::soy_sauce_bottles;
use shared::potion::{Liquid, pressed_into};
use shared::protocol::{Landed, ServerMessage};

use super::Client;
use super::press_oil::{practise_pressing, start_press};
use crate::ServerWorld;
use crate::ids::new_id;

/// The wind-up for a moromi is over. `false` if the client has gone.
pub(super) fn land_moromi(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let moromi = client.inventory.moromi_to_press();
    let Some(moromi) = moromi.and_then(|id| client.inventory.take_one(id, &mut new_id)) else {
        client.pressing = None;
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    let bottles = soy_sauce_bottles(&moromi);
    let quality = moromi.quality;
    for _ in 0..bottles {
        let empty = client.inventory.empty_bottle();
        let Some(mut bottle) = empty.and_then(|id| client.inventory.take_one(id, &mut new_id))
        else {
            break;
        };
        bottle = pressed_into(bottle, Liquid::SoySauce);
        bottle.quality = quality;
        if let Err(item) = client.inventory.give(bottle, world.stacks, &mut new_id) {
            crate::ground::put_down(client, world, item);
            client.ground_told.again = true;
        }
    }
    practise_pressing(client, tick, bottles);
    client.unsaved = true;
    client.update_speed(tick);
    let told = client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::SoySauce { bottles },
        });
    if !told {
        return false;
    }
    let (stayed, cost) = start_press(client, world);
    client.energy.spend(tick, client.speed, cost);
    stayed
}
