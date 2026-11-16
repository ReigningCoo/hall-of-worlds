//! Load in Debug Info (`ClientMessage::DevGoToPlace`): going to the place a
//! debug report was written at, in one go: the world, the site and its level,
//! the spot and the way faced, and the time if allowed. What is generated
//! there, not another server's saved state of it.

use bevy::prelude::*;
use shared::debug_report::{DevPlace, ReportClock, Where};
use shared::protocol::{DevGoToPlace, PlaceAnswer, PlaceRefusal, ServerMessage};
use shared::rng::Seed;
use shared::role::DebugTool;
use shared::spot::CELL_SPAN;
use shared::world::{ClockRate, WorldSeed};

use super::moving::change_world;
use super::{Arrival, Client};
use crate::storage::{SiteId, VisitSave};
use crate::{ServerOptions, ServerWorld, ServerWorlds, hall, sites, world_map};

/// Keeps the ask for once this tick's messages are read, as it may need another
/// world; the time only with World Time's permission.
pub(super) fn asked(client: &mut Client, mut ask: DevGoToPlace, options: &ServerOptions) -> bool {
    if !client.may_debug(DebugTool::WorldTime, options) {
        ask.clock = None;
    }
    client.going_place = Some(ask);
    true
}

/// Goes there, answering with `ServerMessage::DevPlace`. `false` if the client
/// has gone.
pub(super) fn go_to_place(
    client: &mut Client,
    worlds: &mut ServerWorlds,
    ask: DevGoToPlace,
    options: &ServerOptions,
) -> bool {
    let current = worlds.of(client.world).seed;
    let terrain = match ask.place {
        // The Hall is the same from any world: the one they are in.
        Where::Hall { .. } => current.terrain(),
        Where::WorldMap { seed, .. }
        | Where::Wilderness { seed, .. }
        | Where::Site { seed, .. } => seed,
    };
    let Some(seed) = find_world(worlds, terrain, ask.planet) else {
        return refuse(client, PlaceRefusal::UnknownWorld);
    };
    if seed != current {
        if !change_world(client, worlds, (seed, Arrival::AsBefore), options) {
            return false;
        }
        // It couldn't be opened (said in the log): left where they were.
        if worlds.of(client.world).seed != seed {
            return true;
        }
    }
    let world = worlds.of(client.world);
    if let Some(clock) = ask.clock {
        set_clock(world, clock);
    }
    let heading = match ask.place {
        Where::WorldMap { .. } => None,
        Where::Wilderness { heading, .. }
        | Where::Site { heading, .. }
        | Where::Hall { heading, .. } => Some(heading),
    };
    match go(client, world, ask) {
        Err(why) => refuse(client, why),
        Ok(false) => false,
        Ok(true) => {
            let heading = heading.unwrap_or(client.heading);
            client.face(heading)
                && client
                    .link
                    .send(&ServerMessage::DevPlace(PlaceAnswer::Arrived { heading }))
        }
    }
}

/// The world with this terrain seed: one open here (the home among them), or
/// the Hall planet `planet` if it is that one.
pub(super) fn find_world(
    worlds: &ServerWorlds,
    terrain: Seed,
    planet: Option<u32>,
) -> Option<WorldSeed> {
    let open = worlds.iter().map(|(_, world)| world.seed);
    let planet = planet.map(|planet| WorldSeed::new(u64::from(planet)));
    open.chain(planet).find(|seed| seed.terrain() == terrain)
}

/// Sets the world's time as the report had it, unless it isn't a time at all.
fn set_clock(world: &mut ServerWorld, clock: ReportClock) {
    let rate = ClockRate::new(clock.rate.game, clock.rate.real);
    if !clock.minutes.is_finite() || clock.minutes < 0.0 || rate.is_none() {
        warn!("A client asked for a world time that isn't one");
        return;
    }
    world.set_time(Some(clock.minutes), rate);
}

/// The trip within the world. `Ok(false)` if the client has gone.
fn go(
    client: &mut Client,
    world: &mut ServerWorld,
    ask: DevGoToPlace,
) -> Result<bool, PlaceRefusal> {
    Ok(match ask.place {
        Where::WorldMap { x, y, .. } => world_map::go_to(client, world, (x, y)),
        Where::Wilderness { at, .. } => sites::dev_wilderness(client, world, at),
        Where::Hall { origin, at, .. } => {
            hall::enter(client, world, at.offset(origin * CELL_SPAN, 0))
        }
        Where::Site {
            x,
            y,
            chunk,
            kind,
            level,
            at,
            ..
        } => {
            let known = world.pois.in_chunk(chunk);
            if ask.dev.is_none() && known.is_none_or(|poi| poi.kind != kind) {
                return Err(PlaceRefusal::NoSuchPoi);
            }
            // Where they stood on the world map, which a debug tool's site is
            // made at.
            world_map::go_to(client, world, (x, y))
                && match ask.dev {
                    Some(DevPlace::Site { kind, seed, ruin }) => {
                        sites::generate(client, world, (kind, seed, ruin), level)
                    }
                    Some(DevPlace::Area(area)) => sites::generate_area(client, world, area),
                    // As if coming back to a save there: a first level set in
                    // the wilderness is the wilderness's.
                    None => sites::resume(
                        client,
                        world,
                        VisitSave {
                            site: SiteId::Poi(chunk),
                            level,
                            position: at,
                            from_wilderness: false,
                        },
                    ),
                }
                && sites::land_near(client, at)
        }
    })
}

/// Tells the client it couldn't go there.
fn refuse(client: &mut Client, why: PlaceRefusal) -> bool {
    info!("Refused a debug string's place: {why:?}");
    client
        .link
        .send(&ServerMessage::DevPlace(PlaceAnswer::Refused(why)))
}
