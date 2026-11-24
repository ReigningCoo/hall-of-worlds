//! A Teleport's move (magic plan PR 13): a player a Teleport lies on
//! (`Sheet::may_teleport`) asks to go straight to a cell; it is taken at once
//! if the shared rule allows it (`spell::may_teleport_to`: in range, in a
//! clear line, room to stand) and the cell is one they have been sent, using
//! the Teleport up. Otherwise refused, `Position` putting the client right
//! either way.

use bevy::prelude::*;
use shared::energy::site_move_cost;
use shared::spell::{TELEPORTING, may_teleport_to};
use shared::spot::{CELL_SPAN, Spot};

use crate::ServerWorld;
use crate::connections::Client;
use crate::sites;

/// Takes a Teleport's move to `to`, if it is allowed: whether the client is
/// still there, and what it cost (a cell's walk; nothing if refused).
pub(super) fn teleport_taken(
    client: &mut Client,
    world: &mut ServerWorld,
    to: Spot,
) -> (bool, u32) {
    let Some(visit) = &client.visit else {
        info!("Refused a teleport off a level");
        return (true, 0);
    };
    let from = visit.spot();
    let size = client.size();
    let allowed = client.sheet.may_teleport()
        && visit.was_sent(to.cell())
        && may_teleport_to(visit.cells(), size, from, to);
    if !allowed {
        info!("Refused a teleport to {to:?}");
        return (true, 0);
    }
    let tick = world.action_clock.ticks();
    client.sheet.end_spell(TELEPORTING);
    client.unsaved = true;
    let sent = client.send_sheet(tick);
    let stayed =
        sites::moved(client, to) && sites::follow(client, world) && sites::follow_inn(client);
    let cost = site_move_cost(from, from.offset(CELL_SPAN, 0));
    (sent && stayed, cost)
}
