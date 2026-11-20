//! Predicting stumbles (`shared::alcohol::stumble`): drunk or worse, a move or
//! step whose number the hash picks veers off, as the server will take it.
//! The intended move is still sent (the server veers it itself); here the
//! player is put where it ends, which stops travel, and what else was walked
//! this frame is dropped.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use shared::alcohol::{stumble, veered_move, veered_step};
use shared::id::Id;
use shared::movement::{Step, step_on_world_map_by};
use shared::protocol::ServerMessage;
use shared::spot::Spot;

use super::FromServer;
use crate::character::CharacterSheet;
use crate::common::{
    CurrentDungeon, CurrentWorld, GameClock, MapPosition, PlayerGait, PlayerSize, SmoothPosition,
    SpotPosition,
};
use crate::player::Player;

/// The character being played, as the server's welcome said: whose id the
/// stumbles are hashed from.
#[derive(Resource, Default)]
pub(super) struct OwnCharacter(pub Option<Id>);

/// Keeps `OwnCharacter` from the welcome.
pub(super) fn keep_character(
    mut from_server: MessageReader<FromServer>,
    mut own: ResMut<OwnCharacter>,
) {
    for FromServer(message) in from_server.read() {
        if let ServerMessage::Welcome { character, .. } = message {
            own.0 = character.map(|character| character.0);
        }
    }
}

/// What working a stumble out takes.
#[derive(SystemParam)]
pub(super) struct Stumbling<'w, 's> {
    own: Res<'w, OwnCharacter>,
    sheet: Res<'w, CharacterSheet>,
    dungeon: Res<'w, CurrentDungeon>,
    world: Option<Res<'w, CurrentWorld>>,
    clock: Res<'w, GameClock>,
    size: Res<'w, PlayerSize>,
    gait: Res<'w, PlayerGait>,
    player: Query<
        'w,
        's,
        (
            &'static mut MapPosition,
            &'static mut SpotPosition,
            &'static mut SmoothPosition,
        ),
        With<Player>,
    >,
}

impl Stumbling<'_, '_> {
    fn veer(&self, map: u32, number: u32) -> Option<shared::alcohol::Veer> {
        stumble(self.own.0?, map, number, self.sheet.0.stumble_one_in())
    }

    /// Where move `number` on map `map` from `from` towards `to` on a level
    /// ends if it stumbles (`from` for nowhere), the player put there.
    pub fn level_move(&mut self, map: u32, number: u32, from: Spot, to: Spot) -> Option<Spot> {
        let veer = self.veer(map, number)?;
        let (level, size) = (&self.dungeon.level, self.size.0);
        let landed = veered_move(level, size, self.gait.0, from, to, veer);
        for (mut position, mut spot, mut smooth) in &mut self.player {
            (*spot, *position, *smooth) = SpotPosition::place(landed);
        }
        Some(landed)
    }

    /// How far world map step `number` on map `map`, taken from `to` less
    /// `moved`, moves the player if it stumbles (nowhere if the turned step
    /// can't be taken), the player put there.
    pub fn world_step(
        &mut self,
        map: u32,
        number: u32,
        step: Step,
        (to, moved): ((i64, i64), (i64, i64)),
    ) -> Option<(i64, i64)> {
        let veer = self.veer(map, number)?;
        let world = self.world.as_ref()?;
        let from = (to.0 - moved.0, to.1 - moved.1);
        let turned = veered_step(step, veer);
        let conditions = world.conditions(&self.clock);
        let landed = step_on_world_map_by(&world.ground, conditions, from, turned, self.gait.0);
        let (x, y) = landed.unwrap_or(from);
        for (mut position, ..) in &mut self.player {
            // Travel notices and stops (`player`).
            *position = MapPosition { x, y };
        }
        Some(match landed {
            Some(_) => (turned.dx(), turned.dy()),
            None => (0, 0),
        })
    }
}
