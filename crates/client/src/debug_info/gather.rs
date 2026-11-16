//! Making the report from what the client knows now. Never anything of the
//! account: no email address, server address or account name.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use shared::debug_report::{Camera, DebugFlags, DebugReport, ReportClock, Where, Who};
use shared::facing::Heading;
use shared::poi::ChunkPos;
use shared::protocol::LevelKind;

use super::known::Known;
use super::platform;
use crate::character::CharacterSheet;
use crate::common::{
    CurrentDungeon, CurrentWorld, Facing, FreezeMobs, GameView, GodMode, MapPosition,
    PlayDifficulty, ServerClock, SpotPosition, Truesight, WorldReturn,
};
use crate::inventory::Carried;
use crate::player::Player;
use crate::view3d::{OrbitCamera, ViewCamera};

/// Everything a report is made from.
/// The player's cell, spot (on a level) and facing.
type PlayerPlace = (
    &'static MapPosition,
    Option<&'static SpotPosition>,
    Option<&'static Facing>,
);

#[derive(SystemParam)]
pub(super) struct Sources<'w, 's> {
    view: Res<'w, State<GameView>>,
    world: Option<Res<'w, CurrentWorld>>,
    dungeon: Res<'w, CurrentDungeon>,
    world_return: Res<'w, WorldReturn>,
    clock: Res<'w, ServerClock>,
    sheet: Res<'w, CharacterSheet>,
    carried: Res<'w, Carried>,
    difficulty: Res<'w, PlayDifficulty>,
    flags: (Res<'w, GodMode>, Res<'w, Truesight>, Res<'w, FreezeMobs>),
    known: Res<'w, Known>,
    player: Query<'w, 's, PlayerPlace, With<Player>>,
    camera: Query<'w, 's, &'static OrbitCamera, With<ViewCamera>>,
}

impl Sources<'_, '_> {
    /// The report now, with `problem` as its description. Its time is 0:
    /// the caller puts it in when the rest has changed.
    pub fn report(&self, problem: &str) -> DebugReport {
        let in_world = *self.view.get() != GameView::Connecting && self.world.is_some();
        let place = if in_world { self.place() } else { None };
        let in_site = matches!(place, Some(Where::Site { .. }));
        let (god_mode, truesight, freeze_mobs) = &self.flags;
        DebugReport {
            build: platform::build(),
            utc: 0,
            who: in_world.then(|| self.who()),
            place,
            planet: self
                .world
                .as_ref()
                .filter(|_| in_world)
                .and_then(|w| hall_planet(w)),
            clock: in_world.then(|| ReportClock {
                // Whole minutes, so the string doesn't change every frame.
                minutes: self.clock.minutes.floor(),
                rate: self.clock.time.rate,
            }),
            camera: self.camera.single().ok().map(|camera| Camera {
                turn: camera.yaw,
                tilt: camera.pitch,
                zoom: camera.distance,
            }),
            flags: DebugFlags {
                god_mode: god_mode.0,
                truesight: truesight.0,
                freeze_mobs: freeze_mobs.0,
            },
            dev: self.known.dev.filter(|_| in_site),
            problem: problem.to_owned(),
        }
    }

    fn who(&self) -> Who {
        Who {
            id: self.known.character,
            kind: self.carried.0.creature,
            gender: self.sheet.0.gender,
            difficulty: self.difficulty.current,
        }
    }

    fn place(&self) -> Option<Where> {
        let world = self.world.as_ref()?;
        let seed = world.params.seed;
        let (position, spot, facing) = self.player.single().ok()?;
        if *self.view.get() == GameView::WorldMap {
            let (x, y) = (position.x, position.y);
            return Some(Where::WorldMap { seed, x, y });
        }
        let info = self.dungeon.info.as_ref()?;
        let at = spot?.0;
        let heading = facing.map_or(Heading::default(), |facing| facing.0);
        Some(match info.kind {
            LevelKind::Wilderness => Where::Wilderness {
                seed,
                at,
                heading,
                poi: self.known.in_poi,
            },
            LevelKind::Hall { origin } => Where::Hall {
                origin,
                at,
                heading,
            },
            LevelKind::Poi(kind) => {
                let MapPosition { x, y } = self.world_return.0;
                Where::Site {
                    seed,
                    x,
                    y,
                    chunk: ChunkPos::containing(x, y, world.surface.width()),
                    kind,
                    ruined: info.ruined,
                    level: info.index,
                    levels: info.levels,
                    at,
                    heading,
                }
            }
        })
    }
}

/// The world's seed if it is one of the Hall's planets: its gates' places give
/// it (public), checked against its terrain seed.
fn hall_planet(world: &CurrentWorld) -> Option<u32> {
    let gate = world.gates.first()?;
    let seed = shared::hall::GatePlace::from_number(gate.place)
        .address()
        .seed();
    let world_seed = shared::world::WorldSeed::new(u64::from(seed));
    (world_seed.terrain() == world.params.seed).then_some(seed)
}
