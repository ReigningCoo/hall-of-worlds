//! Water's wet and ice's chill landing on creatures (magic plan PR 11):
//! timed effects on players (`spell::wet`, `spell::chill`), the chill a third
//! slower; mobs keep no effects yet, so they can't be wet, and a chill only
//! slows them (`entities::chill_mob`). A chill is harmful, so it takes hold
//! only by the ratio contest of the caster's Water against the target's Aura
//! and Warding (`spell::takes_hold`).

use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::spell::{CHILL_SECONDS, Effect, chill, takes_hold, wet};

use super::super::Client;
use super::effects::Landing;
use crate::ServerWorld;
use crate::combat::luck;
use crate::entities::chill_mob;

/// `count` Waters wet each player among the targets. Whether any was.
pub(super) fn wet_targets(
    clients: &mut [Client],
    at: &Landing,
    targets: &[usize],
    count: u32,
) -> bool {
    let mut changed = false;
    for &index in targets {
        let Some(player) = at.present.players[index] else {
            continue;
        };
        let client = &mut clients[player];
        client.sheet.effects.add(wet(count, at.tick));
        client.unsaved = true;
        client.send_sheet(at.tick);
        changed = true;
    }
    changed
}

/// `count` ices chill each target they take hold on. Whether any was.
pub(super) fn chill_targets(
    clients: &mut [Client],
    caster: usize,
    world: &mut ServerWorld,
    at: &Landing,
    targets: &[usize],
    count: u32,
) -> bool {
    let me = &clients[caster];
    let power = water_power(me);
    let caster_id = me.entity_id();
    let mut changed = false;
    for &index in targets {
        let id = at.present.ids[index];
        let mut rng = luck(at.tick, caster_id, id);
        if !takes_hold(power, at.present.resistances[index], &mut rng) {
            continue;
        }
        changed |= match at.present.players[index] {
            Some(player) => {
                let client = &mut clients[player];
                client.sheet.effects.add(chill(count, at.tick));
                client.update_speed(at.tick);
                client.unsaved = true;
                client.send_sheet(at.tick);
                true
            }
            None => {
                let ticks = CHILL_SECONDS * u64::from(count) * ACTION_TICKS_PER_SECOND as u64;
                chill_mob(world, at.place, id, at.tick + ticks, at.tick)
            }
        };
    }
    changed
}

/// The caster's power with the Water skill, whose reversal ice is.
fn water_power(caster: &Client) -> u64 {
    let skill = Effect::Water.skill();
    let total = caster.sheet.skills.total(skill);
    shared::combat::power(&caster.bonused.attributes, skill.leans(), total)
}
