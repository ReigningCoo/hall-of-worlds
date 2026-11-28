//! Running every mob each tick.

use shared::dungeon::line_is_clear;

use super::choice::{choose, others_of, with_fire};
use super::doors::{Opened, open_on_the_way, way_level};
use super::escortee::mind_escort;
use super::facing::noises;
use super::fight::{MobBlow, mob_turn};
use super::foes::with_mobs;
use super::mob_blows::strike_mob;
use super::on_ground::on_ground;
use super::territory::mind_territory;
use super::tracking::track;
use super::{Doing, Maps, Population, Spot, Taken, act, cross, everyone_playing};
use crate::ServerWorld;
use crate::connections::Client;

/// How often old scent marks are thrown away, in ticks: every 10 seconds.
const SWEEP_EVERY: u64 = 200;

/// Lets each level's mobs do what they are doing, as far as they have the energy.
pub(crate) fn run_mobs(clients: &mut [Client], world: &mut ServerWorld) {
    // Developer tools' Freeze Mobs.
    if world.mobs_frozen {
        return;
    }
    let tick = world.action_clock.ticks();
    // Spells' poison harms before anyone acts.
    super::poison_mobs(world, tick);
    let everyone = everyone_playing(clients);
    let fires = super::choice::fires_by_place(clients, world);
    let behaviours = world.populations.1.clone();
    let mut crossings = Vec::new();
    let mut blows: Vec<MobBlow> = Vec::new();
    let mut on_mobs = Vec::new();
    let mut opened: Vec<Opened> = Vec::new();
    for (&place, population) in &mut world.populations.0 {
        let here = || {
            everyone
                .iter()
                .filter(|someone| someone.level == Some(place))
        };
        // Those in god mode still stand somewhere and can see, but aren't noticed.
        let standing: Vec<(i64, i64)> = here().map(|someone| someone.position).collect();
        let spots: Vec<Spot> = here()
            .filter(|someone| !someone.god_mode)
            .map(|someone| Spot {
                id: someone.id,
                character: someone.character,
                kind: someone.look.0,
                position: someone.position,
                at: someone.spot,
                scent: someone.scent,
                member: None,
            })
            .collect();
        let Population {
            mobs,
            level,
            relief,
            doors,
            due,
            scents,
            last_at,
            ..
        } = population;

        // Players leave scent on the cells they step off.
        for spot in &spots {
            if let Some((was_at, _)) = last_at.insert(spot.id, (spot.position, spot.scent)) {
                scents.walked(spot.id, was_at, spot.position, spot.scent, tick);
            }
        }
        last_at.retain(|id, _| spots.iter().any(|spot| spot.id == *id));
        if tick.is_multiple_of(SWEEP_EVERY) {
            scents.sweep(tick);
        }

        // Those at rest turn to fighting they hear.
        let fighting: Vec<_> = here()
            .filter(|someone| someone.fighting)
            .map(|someone| someone.spot)
            .collect();
        let noises = noises(&fighting, mobs);
        let fires_here = fires.get(&place).map_or(&[][..], Vec::as_slice);
        // Every mob's choice from where everyone stands now, before any moves.
        let others = others_of(mobs);
        // Mobs see, and may fight, other mobs too (mob behaviour plan PR 4).
        let mut creatures = with_mobs(&spots, mobs);
        for mob in mobs.iter_mut().filter(|mob| !mob.still) {
            mob.thaw(tick);
            // One escorted only keeps near its quester (`escortee`).
            if mob.escort.is_some() {
                if mind_escort(mob, &spots) {
                    due.insert((mob.energy.ready_at(tick, mob.speed), mob.id));
                }
                continue;
            }
            mob.listen(&noises);
            mind_territory(mob, level, &spots, tick);
            mob.ways.set(behaviours.of(mob.kind));
            mob.find_foes(&others, &behaviours);
            let (chosen, scores) = choose(
                mob,
                level,
                &creatures,
                (&others, fires_here),
                &behaviours,
                tick,
            );
            if !scores.options.is_empty() {
                mob.scores = scores;
            }
            if let Some(doing) = chosen {
                let at_rest = mob.doing == Doing::Idle;
                mob.doing = doing;
                mob.target = doing.target();
                if at_rest {
                    due.insert((mob.energy.ready_at(tick, mob.speed), mob.id));
                }
            }
        }
        // Hunters join their band's hunts or pick one up by scent.
        super::hunt::hunt(mobs, level, (scents, &behaviours), tick, due);

        let mut taken: Taken = standing.iter().copied().collect();
        taken.extend(mobs.iter().map(|mob| mob.position));
        let mut maps = Maps::default();
        let first_blow = blows.len();
        while let Some(&(when, id)) = due.first() {
            if when > tick {
                break;
            }
            due.remove(&(when, id));
            let Some(mob) = mobs.iter_mut().find(|mob| mob.id == id) else {
                continue;
            };
            // Developer tools' Still: whatever happened, it does nothing.
            if mob.still {
                mob.doing = Doing::Idle;
                mob.target = None;
                continue;
            }
            let mut crossing = None;
            let seen = |cell| {
                standing
                    .iter()
                    .any(|&spot| line_is_clear(level, spot, cell))
            };
            let was_at = mob.position;
            let was_spot = mob.spot;
            // One that fears fire steps no nearer one.
            let feared = match mob.ways.fears_fire {
                true => with_fire(&taken, mob.position, fires_here),
                false => None,
            };
            let taken_now = feared.as_ref().unwrap_or(&taken);
            let ground = (&*level, relief.as_ref());
            let fighting = match mob.escort {
                Some(_) => None,
                None => mob_turn(mob, ground, &creatures, (tick, taken_now), &mut blows),
            };
            let cost = if let Some(cost) = fighting {
                cost
            } else if matches!(mob.doing, Doing::Tracking { .. }) {
                track(
                    mob,
                    way_level(mob, level, doors.as_ref()),
                    &creatures,
                    scents,
                    taken_now,
                    tick,
                )
            } else {
                act(
                    mob,
                    way_level(mob, level, doors.as_ref()),
                    &creatures,
                    &mut maps,
                    tick,
                    seen,
                    &mut crossing,
                    taken_now,
                    place,
                )
            };
            // Slower up and down hills and through shallows.
            let cost = match crossing {
                None => on_ground(level, relief.as_ref(), (was_spot, mob.spot), cost),
                Some(_) => cost,
            };
            // One with hands that came to a closed door opens it instead.
            if let Some(doors) = doors
                && let Some((at, swing)) = open_on_the_way(mob, level, was_spot)
            {
                let poi = doors.poi;
                // A gate's other leaf opens with it.
                let other = shared::door::gate_partner(level, at.0, at.1);
                let other = other.filter(|&(x, y)| level.is_shut(x, y));
                for at in std::iter::once(at).chain(other) {
                    level.set_swing(at.0, at.1, swing);
                    opened.push(Opened {
                        place,
                        poi,
                        at,
                        swing,
                    });
                }
                doors.open = level.with_doors_open();
            }
            mob.turn_after(was_spot, &creatures);
            // Others see it where it now stands.
            if let Some(me) = creatures.iter_mut().find(|one| one.id == mob.id) {
                (me.position, me.at) = (mob.position, mob.spot);
            }
            // Mobs leave scent too.
            scents.walked(mob.id, was_at, mob.position, mob.scent, tick);
            mob.pacing.turn_ended(was_at, mob.position);
            taken.remove(&was_at);
            taken.insert(mob.position);
            mob.energy.spend(tick, mob.speed, cost);
            mob.target = mob.doing.target();
            if mob.doing.gave_up() {
                mob.harmed_by.clear();
            }
            if let Some(crossing) = crossing {
                crossings.push(crossing);
            } else if mob.doing != Doing::Idle {
                // At least the next tick, so one turn can't run for ever here.
                due.insert((mob.next_due(tick), id));
            }
        }
        // Blows on other mobs land once every level has had its turn.
        let made: Vec<MobBlow> = blows.drain(first_blow..).collect();
        for blow in made {
            match creatures
                .iter()
                .any(|one| one.id == blow.target() && one.member.is_some())
            {
                true => on_mobs.push((place, blow)),
                false => blows.push(blow),
            }
        }
    }
    for (place, blow) in on_mobs {
        strike_mob(world, place, blow, tick);
    }
    for crossing in crossings {
        cross(&mut world.populations, crossing, tick, &everyone);
    }
    for door in opened {
        crate::sites::mob_opened(world, door);
    }
    crate::connections::take_mob_blows(clients, world, blows, tick);
}

/// How far apart on the world map, the nearer way round the planet.
pub(super) fn world_gap(a: (i64, i64), b: (i64, i64), width: i64) -> i64 {
    let dx = (a.0 - b.0).rem_euclid(width);
    dx.min(width - dx).max((a.1 - b.1).abs())
}
