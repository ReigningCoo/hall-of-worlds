//! Where the player is, for the music: worked out each frame from what the
//! client already knows (the view, the level's kind, the world map point
//! with its climate, and the suns), and taken as the place once it has held
//! for a moment, so walking along a climate border or at dusk doesn't
//! change the tunes back and forth.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::place::{Band, Daylight, Place};
use crate::common::{
    CurrentDungeon, CurrentWorld, GameView, MapPosition, ServerClock, WorldReturn,
};
use crate::player::Player;

/// Seconds a new place must hold before the music follows it.
const SETTLE_SECONDS: f32 = 1.5;

/// The place the music follows (`None` before the game shows one), and one
/// waiting to settle.
#[derive(Resource, Default)]
pub(super) struct PlaceNow {
    pub place: Option<Place>,
    waiting: Option<(Place, f32)>,
}

/// What the place is worked out from.
#[derive(SystemParam)]
pub(super) struct Surroundings<'w, 's> {
    view: Res<'w, State<GameView>>,
    dungeon: Res<'w, CurrentDungeon>,
    world: Option<Res<'w, CurrentWorld>>,
    world_return: Res<'w, WorldReturn>,
    clock: Res<'w, ServerClock>,
    player: Query<'w, 's, &'static MapPosition, With<Player>>,
}

impl Surroundings<'_, '_> {
    /// The place the player is in now, if the game shows one.
    fn place(&self) -> Option<Place> {
        let world = self.world.as_ref()?;
        let point = match self.view.get() {
            GameView::Connecting => return None,
            GameView::WorldMap => *self.player.single().ok()?,
            GameView::Dungeon => self.world_return.0,
        };
        let outside = wilderness_at(world, point, self.clock.hours());
        Some(match self.view.get() {
            GameView::WorldMap => Place::WorldMap,
            _ => Place::of_level(self.dungeon.info.as_ref()?, outside),
        })
    }
}

/// Keeps `PlaceNow` up to date.
pub(super) fn follow_place(surroundings: Surroundings, time: Res<Time>, mut now: ResMut<PlaceNow>) {
    let seen = surroundings.place();
    let Some(seen) = seen else {
        return;
    };
    if now.place.is_none() {
        info!("Music: in {seen:?}");
        now.place = Some(seen);
    }
    if now.place == Some(seen) {
        now.waiting = None;
        return;
    }
    let held = match now.waiting {
        Some((waiting, held)) if waiting == seen => held + time.delta_secs(),
        _ => 0.0,
    };
    now.waiting = Some((seen, held));
    if held >= SETTLE_SECONDS {
        info!("Music: now {seen:?}");
        now.place = Some(seen);
        now.waiting = None;
    }
}

/// The wilderness at a world map point at `hours`: its climate band, and
/// day while any sun is above the horizon.
fn wilderness_at(world: &CurrentWorld, point: MapPosition, hours: f64) -> Place {
    let height = world.surface.sample(point.x, point.y).height;
    let biome = world.climate.sample(point.x, point.y, height).biome();
    let suns = world.celestial.sun_count();
    let bodies = world.celestial.at(hours);
    let day = bodies.iter().take(suns).any(|sun| sun.direction[1] > 0.0);
    let time = match day {
        true => Daylight::Day,
        false => Daylight::Night,
    };
    Place::Wilderness(Band::of(biome), time)
}
