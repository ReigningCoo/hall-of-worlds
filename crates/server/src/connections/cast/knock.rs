//! Air knocking creatures away from where its where came from, and a vacuum
//! pulling them towards it (magic plan PR 12): as far as the caster's Air
//! skill and the count say, shared out by each target's resistance as harm
//! is (`spell::resisted`), and as far as walls let (`spell::knocked_to`). A
//! player knocked is told where the server now has them, as after a refused
//! move.

use shared::facing::Heading;
use shared::spell::{Knock, effect_power, knock_units, knocked_to};
use shared::vitals::Points;

use super::super::Client;
use super::effects::Landing;
use crate::ServerWorld;
use crate::entities::knock_mob;
use crate::sites::moved;

/// `count` Airs move each target, the way `knock` says. Whether any moved.
pub(super) fn knock_targets(
    clients: &mut [Client],
    caster: usize,
    world: &mut ServerWorld,
    at: &Landing,
    targets: &[usize],
    (knock, count): (Knock, u32),
) -> bool {
    let me = &clients[caster];
    let skills = Some(&me.sheet.skills);
    let units = knock_units(count, skills);
    let power = effect_power(shared::spell::Effect::Air, &me.bonused.attributes, skills);
    let mut changed = false;
    for (n, &index) in targets.iter().enumerate() {
        let resistance = at.present.resistances[index];
        let units = shared::spell::resisted(Points(units as u32), power, resistance).0;
        let away = at.away.get(n).copied().unwrap_or_default();
        let heading = match knock {
            Knock::Away => away,
            Knock::In => Heading(away.0.wrapping_add(Heading::WEST.0)),
        };
        let units = u64::from(units);
        changed |= match at.present.players[index] {
            Some(player) => knock_player(&mut clients[player], heading, units),
            None => knock_mob(world, at.place, at.present.ids[index], heading, units),
        };
    }
    changed
}

/// Knocks a player `units` going `heading`, as far as walls let them go.
fn knock_player(client: &mut Client, heading: Heading, units: u64) -> bool {
    let size = client.inventory.creature.def().size;
    let Some(visit) = client.visit.as_ref() else {
        return false;
    };
    let from = visit.spot();
    let to = knocked_to(visit.cells(), size, from, heading, units);
    if to == from {
        return false;
    }
    moved(client, to);
    // `Position` puts the client right, after every move it has made.
    client.last_step = Some(client.steps_done);
    true
}
